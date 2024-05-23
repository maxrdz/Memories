#!/usr/bin/env python3
# -*- coding: utf-8 -*-
#
# Copyright © 2018, 2019 Endless Mobile, Inc.
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

GNOME_GITLAB_URI = 'https://gitlab.gnome.org'

f"""
NEWS/ChangeLog generation script for GitLab.

Usage:
    gitlab-changelog.py -H {GNOME_GITLAB_URI} -t <TOKEN> GNOME/glib 2.58.2..
then put the output into NEWS/ChangeLog and make the release.

The -H and -t arguments can be omitted by putting the following in
`~/.config/gitlab-changelog.ini`:
    [gitlab-changelog]
    default-hostname = {GNOME_GITLAB_URI}
    [{GNOME_GITLAB_URI}]
    token = <TOKEN>

To generate an authentication token for the script, go to
    {GNOME_GITLAB_URI}/-/profile/personal_access_tokens
and generate a token with the `api` scope.

To install the dependencies for this script:
    pip3 install python-gitlab GitPython
"""

import argparse
import configparser
import os
import re
import sys
import urllib.parse
from git import Repo
import gitlab
import textwrap

os.environ['LANG'] = 'C'

import gi
gi.require_version('GnomeDesktop', '3.0')
from gi.repository import GLib, GnomeDesktop  # noqa: E402


def get_commit_translations(commit):
    """Return a potentially empty set of locale codes changed by this
       commit."""
    diff = commit.diff(commit.hexsha + '~1')
    locales = []

    for item in diff:
        if item.a_path.startswith('po/') and item.a_path.endswith('.po'):
            locales.append(item.a_path[3:-3])

    return set(locales)


def extract_int_line_ending(line, prefix):
    """If @line starts with @prefix, return a set() of the integer following
    the @prefix. If @line doesn’t start with @prefix, or if @prefix isn’t
    followed by an integer, return an empty set()."""
    if line.startswith(prefix):
        try:
            return set([int(line[len(prefix):])])
        except ValueError:
            pass
    return set()

def extract_commit_line_issues(line, prefix, gl_project_name, project_ns, project_name):
    issues = set()
    external_issues = set()

    # https://docs.gitlab.com/ee/user/project/issues/managing_issues.html#default-closing-pattern
    # Closes #542, #456
    # Fixes #542
    # Closes: #542
    # Fixes: #542, #123
    matches = re.match(r'^(?:Closes|Fixes|Resolves|Implements)(:?\s+|:)', line)
    if not matches:
        return (issues, external_issues)

    commit_issues = re.split(r',|\s', line[matches.end(1):])

    for ref in commit_issues:
        matches = re.search(r'([^\s]+/issues/|((\w+)#)|#)(\d+)$', ref)
        if matches:
            issue_id = int(matches.group(4))
            if matches.group(1) == '#':
                # #1234
                issues.add((gl_project_name, issue_id))
            elif matches.group(1).startswith('http'):
                uri = matches.group(1)
                if uri.startswith(prefix):
                    # http(s)://gitlab.gnome.org/NAMESPACE/PROJECT/issues/1234
                    issues.add((gl_project_name, issue_id))
                else:
                    uri_path = urllib.parse.urlparse(uri).path
                    if uri_path.startswith(f'/{project_ns}'):
                        # http(s)://gitlab.gnome.org/NAMESPACE/OTHER_PROJECT/issues/1234
                        # is converted to NAMESPACE/OTHER_PROJECT#1234
                        m = re.match(r'/?(.*?)(/-)?/issues/.*', uri_path)
                        if m:
                            short_uri = m.group(1)
                            issues.add((short_uri, issue_id))
                    else:
                        external_issues.add(uri)
            elif matches.group(3):
                short_uri = matches.group(3)
                if (short_uri == gl_project_name or
                    short_uri == project_name):
                    # PROJECT#1234 or NAMESPACE/PROJECT#1234
                    issues.add((gl_project_name, issue_id))
                else:
                    if not short_uri.startswith(project_ns):
                        if '/' not in short_uri:
                            #OTHER_PROJECT#1234
                            short_uri = f'{project_ns}/{short_uri}'
                    # NAMESPACE/OTHER_PROJECT#1234 or OTHER_NAMESPACE/OTHER_PROJECT#1234
                    issues.add((short_uri, issue_id))

    return (issues, external_issues)

def get_issues_and_merge_requests(commit, gl_default_hostname, gl_project_name):
    """Return a potentially empty set of issue numbers closed by this
       commit. This parses a number of common ways of indicating issue closure
       in the commit message. See the comments inline below for the styles
       supported.
    """

    # FIXME: Would be good if gitlab put metadata in merge commits, then we
    # could parse those instead, rather than the human-readable commit message.
    # See https://gitlab.com/gitlab-org/gitlab/-/issues/26266
    lines = commit.message.split('\n')

    issues = set()
    external_issues = set()
    merge_requests = set()
    prefix = urllib.parse.urljoin(gl_default_hostname, gl_project_name)
    project_name = gl_project_name.split('/')[-1]
    project_ns = gl_project_name.split('/')[0]

    for line in lines:
        (line_issues, line_external_issues) = extract_commit_line_issues (
            line, prefix, gl_project_name, project_ns, project_name)
        issues |= line_issues
        external_issues |= line_external_issues

        # Part-of: <https://gitlab.gnome.org/GNOME/gnome-initial-setup/-/merge_requests/119>
        matches = re.search(r'^Part-of: <{}(/-)?/merge_requests/(\d+)>$'.format(
            re.escape(prefix)), line)
        if matches:
            merge_requests.add(int(matches.group(2)))

        # https://gitlab.gnome.org/GNOME/glib/issues/1620
        # https://gitlab.gnome.org/GNOME/glib/merge_requests/554
        issue_prefix = urllib.parse.urljoin(prefix, 'issues/')
        merge_request_prefix = urllib.parse.urljoin(prefix, 'merge_requests/')

        issues |= extract_int_line_ending(line, issue_prefix)
        merge_requests |= extract_int_line_ending(line, merge_request_prefix)

        # See merge request GNOME/glib!554
        # See issue GNOME/glib#1601
        issue_prefix = 'See issue {}#'.format(gl_project_name)
        merge_request_prefix = 'See merge request {}!'.format(gl_project_name)

        issues |= extract_int_line_ending(line, issue_prefix)
        merge_requests |= extract_int_line_ending(line, merge_request_prefix)

    return (issues, external_issues, merge_requests)


class ListWrapper(textwrap.TextWrapper):

    def __init__(self, width=80, initial_ident='  - ', subsequent_indent='    '):
        super().__init__(width=width,
            initial_indent=initial_ident,
            subsequent_indent=subsequent_indent)

    def wrap(self, text):
        split_lines = text.splitlines()
        return [line for para in split_lines for line in
            textwrap.TextWrapper.wrap(self, para)]

    def wrap_lines(self, lines):
        return '\n'.join(self.wrap('\n'.join(lines)))


def print_wrapped(wrapper, text):
    print('\n'.join(wrapper.wrap(text)))


def get_formatted_issues(gl_project_name, issues, issues_authors):
    lines = []
    for issue, (project, gl_issue) in sorted(issues.items()):
        issues_project = '' if project == gl_project_name else project
        lines.append(f'{issues_project}#{issue} {gl_issue.title}')
        authors = issues_authors.get((project, issue), [])
        if authors:
            lines[-1] += f' ({", ".join(authors)})'
    return lines


def get_formatted_mrs(merge_requests, mr_authors):
    lines = []
    for merge_request, gl_merge_request in sorted(merge_requests.items()):
        lines.append(f'!{merge_request} {gl_merge_request.title}')
        authors = mr_authors.get(merge_request, [])
        if authors:
            lines[-1] += f' ({", ".join(authors)})'
    return lines


def get_formatted_locales(locales, locale_authors):
    lines = []
    for l in sorted(locales):
        lines.append(f'{l}')
        authors = locale_authors.get(l, [])
        if authors:
            lines[-1] += f' ({", ".join(authors)})'
    return lines


def main():
    # Load our GitLab authentication details.
    config = configparser.ConfigParser()
    config_path = os.path.join(GLib.get_user_config_dir(),
                               'gitlab-changelog.ini')
    config.read(config_path)

    gl_default_hostname = None
    gl_default_token = None

    try:
        gl_default_hostname = config['gitlab-changelog']['default-hostname']
        gl_default_token = config[gl_default_hostname]['token']
        write_config = False
    except KeyError:
        write_config = True

    # Parse command line arguments.
    parser = argparse.ArgumentParser(
        description='Generate a NEWS/ChangeLog entry for a project which uses '
                    'GitLab. The entry will be outputted on stdout.')
    parser.add_argument('gl_project', metavar='<gitlab project>',
                        help='GitLab project to query '
                             '(for example: ‘GNOME/glib’)')
    parser.add_argument('revs', metavar='<revision range>',
                        nargs='?',
                        help='range of commits to summarise '
                             '(for example: ‘2.58.2..’; default: commits since last annotated tag)')
    parser.add_argument('-C', dest='path', default='.',
                        help='repository to use (default: current directory)')

    parser.add_argument('-H', '--hostname', default=gl_default_hostname,
                        required=(gl_default_hostname is None),
                        help='GitLab hostname (for example: '
                             '‘https://gitlab.gnome.org/’, default: '
                             'load from {})'.format(config_path))
    parser.add_argument('-t', '--token', default=None, required=False,
                        help='GitLab authentication token (default: '
                             'load from {})'.format(config_path))
    parser.add_argument('-w', '--wrap-width', default=80, required=False,
                        help='wrap width of listed lines')

    args = parser.parse_args()

    # Try and look up the token for the given hostname, if not given on the
    # command line.
    try:
        if not args.token:
            args.token = config[args.hostname]['token']
    except KeyError:
        parser.error('Configuration key {}.{} not found in {}'.format(
                     args.hostname, 'token', config_path))
        sys.exit(1)

    # Authenticate with GitLab to check the config.
    try:
        gl = gitlab.Gitlab(args.hostname, private_token=args.token)
        gl_project_name = args.gl_project
        gl_project = gl.projects.get(gl_project_name)
        # Normalize project name (eg gnome/foo → GNOME/foo)
        gl_project_name = gl_project.path_with_namespace
    except gitlab.exceptions.GitlabAuthenticationError as e:
        print('Authentication error: ' + e.error_message)
        print('Your token may have expired. Check and refresh it at:')
        print(f'{args.hostname}-/profile/personal_access_tokens')
        print('It will need the `read_api` scope.')
        sys.exit(1)

    if write_config:
        if 'gitlab-changelog' not in config.sections() or \
           'default-hostname' not in config['gitlab-changelog']:
            config['gitlab-changelog'] = {
                'default-hostname': args.hostname,
            }

        config[args.hostname] = {
            'token': args.token,
        }

        with open(os.open(config_path, os.O_CREAT | os.O_WRONLY, 0o600), 'w') as fh:
            config.write(fh)

    # Get the commits we’re interested in.
    repo = Repo(args.path, search_parent_directories=(args.path == "."))
    if args.revs is None:
        args.revs = repo.git.describe(abbrev=0) + ".."

    commits = repo.iter_commits(args.revs)

    # Parse the git log to find translation edits, issues and merge requests.
    locales = set()
    issues = set()
    external_issues = set ()
    merge_requests = set()
    issues_authors = dict()
    mr_authors = dict()
    locales_authors = dict()
    i = 0

    for commit in commits:
        i += 1

        commit_locales = get_commit_translations(commit)
        locales |= commit_locales
        (commit_issues, commit_external_issues, commit_merge_requests) = \
            get_issues_and_merge_requests(commit, gl_default_hostname, gl_project_name)
        issues |= commit_issues
        external_issues |= commit_external_issues
        if not commit_issues:
            merge_requests |= commit_merge_requests

        # Ignore merge commits
        if len(commit.parents) != 2:
            for issue in commit_issues:
                if issue not in issues_authors:
                    issues_authors[issue] = set()
                issues_authors[issue].add(commit.author.name)

            for mr in commit_merge_requests:
                if mr not in mr_authors:
                    mr_authors[mr] = set()
                mr_authors[mr].add(commit.author.name)

            for l in commit_locales:
                if l not in locales_authors:
                    locales_authors[l] = set()
                locales_authors[l].add(commit.author.name)

    # Query GitLab to get more information on issues and merge requests.
    closed_gl_issues = {}
    unclosed_gl_issues = {}

    gl_projects = { gl_project_name: gl_project }
    for (project, issue) in issues:
        if project not in gl_projects.keys():
            try:
                gl_projects[project] = gl.projects.get(project)
            except gitlab.exceptions.GitlabGetError as e:
                continue

        try:
            gl_issue = gl_projects[project].issues.get(issue)
        except gitlab.exceptions.GitlabGetError:
            continue

        if not gl_issue.closed_at:
            unclosed_gl_issues[issue] = (project, gl_issue)
        else:
            closed_gl_issues[issue] = (project, gl_issue)

    merged_gl_merge_requests = {}
    unmerged_gl_merge_requests = {}

    for merge_request in merge_requests:
        gl_merge_request = gl_project.mergerequests.get(merge_request)
        if not gl_merge_request.merged_at:
            unmerged_gl_merge_requests[merge_request] = gl_merge_request
        else:
            merged_gl_merge_requests[merge_request] = gl_merge_request

    # Map the locale names.
    locale_names = set()
    unmapped_locale_names = set()
    for l in locales:
        # Get the name of 'l' in US English. This data actually comes from the
        # iso-codes package, in the iso_639 and iso_3166 gettext domains, with
        # extra logic to suppress the country name if the language is "unique".
        #
        # Without appending .UTF-8, these come out as (eg):
        #   Portuguese (Brazil) [ISO-8859-1]
        locale_name = GnomeDesktop.get_language_from_locale(f"{l}.UTF-8")
        if locale_name:
            locale_names.add(locale_name)
            locales_authors[locale_name] = locales_authors.get(l, '')
        else:
            unmapped_locale_names.add(l)

    lines = []
    list_wrapper = ListWrapper(width=args.wrap_width)
    text_wrapper = textwrap.TextWrapper(width=args.wrap_width)

    # Print it all
    print_wrapped(text_wrapper,
        '{} commits in range {}, referencing {} issues and {} '
          'MRs'.format(i, args.revs,
                       len(unclosed_gl_issues) + len(closed_gl_issues),
                       len(merge_requests)))
    print_wrapped(text_wrapper,
        '{} translation updates.'.format(len(locale_names)))

    if unclosed_gl_issues:
        print('')
        print_wrapped(text_wrapper,
            f'The following {len(unclosed_gl_issues)} ' +
            'issues were unclosed, and have not been '
            'included in the output:')
        lines = get_formatted_issues (gl_project_name,
            unclosed_gl_issues, issues_authors)
        print(list_wrapper.wrap_lines(sorted(lines)))

    if unmerged_gl_merge_requests:
        print('')
        print_wrapped(text_wrapper,
            'The following ' +
            f'{len(unmerged_gl_merge_requests)} merge requests were not merged, ' +
            'and have not been included in the output:')
        lines = get_formatted_mrs(unmerged_gl_merge_requests,  mr_authors)
        print(list_wrapper.wrap_lines(sorted(lines)))

    if unmapped_locale_names:
        print('')
        print_wrapped(text_wrapper,
            f'The following {len(unmapped_locale_names)} locales ' +
            'could not be named:')
        lines = get_formatted_locales(unmapped_locale_names, locales_authors)
        print(list_wrapper.wrap_lines(lines))

    print('---')
    print('')

    print('* TODO Major news items')
    print('')
    print('* TODO One bullet point each')

    if closed_gl_issues or merged_gl_merge_requests or external_issues:
        print('')
        print('* Bugs fixed:')
        lines = get_formatted_issues(gl_project_name,
            closed_gl_issues, issues_authors)

        lines.extend(external_issues)
        lines.extend(get_formatted_mrs(merged_gl_merge_requests,  mr_authors))

    print(list_wrapper.wrap_lines(lines))

    if locale_names:
        print('')
        print('* Translation updates:')
        lines = get_formatted_locales(locale_names, locales_authors)
        print(list_wrapper.wrap_lines(lines))


if __name__ == '__main__':
    main()

