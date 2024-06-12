# Contributing Guidelines

Thank you for considering contributing to Memories!
All code contributions are made using merge requests.

If you are new to the GNOME community and software ecosystem,
the Memories project would like to give you a warm welcome to
the GNOME community. We highly encourage you to visit
[welcome.gnome.org](https://welcome.gnome.org/), a website
made by the GNOME foundation to introduce new users to GNOME.

Users interested in making contributions to the software
behind Memories are encouraged to check out
[developer.gnome.org](https://developer.gnome.org/).

Before starting to write your own contribution, please make
sure to read the project [README](./README) beforehand!

## Getting Started

The most important first step is to read the
[GNOME Code of Conduct](https://conduct.gnome.org/).

If you are planning on contributing to the translation work,
please see [GNOME Damned Lies](https://l10n.gnome.org/).

- If you don't yet have an account with GNOME's GitLab instance,
please [register on GitLab](https://gitlab.gnome.org/users/sign_up),
this will be important for submitting your changes!
- Set up your [SSH](https://en.wikipedia.org/wiki/Secure_Shell)
cryptographic key in your GNOME GitLab account.
Alternatively, you can also register a
[PGP](https://en.wikipedia.org/wiki/Pretty_Good_Privacy)
cryptographic key for signing your commits within Git.
[welcome.gnome.org](https://welcome.gnome.org/en/app/Loupe/#setting-up-gitlab)
has a good tutorial on setting up your GNOME GitLab account.
- Once you have your account setup and signed in,
[fork the project Git repository](https://gitlab.gnome.org/maxrdz/Memories).
This will create your own copy of the source under your user's namespace.
- Next, **clone** your new fork on your local machine. If you have your
SSH key configured on your GNOME GitLab account, run the following:
```sh
git clone git@ssh.gitlab.gnome.org:maxrdz/Memories.git
```
If you do not have SSH set up, you can also clone via HTTPS.
```sh
git clone https://gitlab.gnome.org/maxrdz/Memories.git
```
- Before starting to write your contribution, create a **new branch**
for your patch/contribution within Git:
```sh
git checkout -b new-patch
```
- After writing your changes, **commit** your changes:
```sh
git commit -am 'src: Fixed bug and resolved #1234'
```
and **push** your new branch to your remote fork.
```sh
git push origin new-patch
```
- The remote Git server at GNOME GitLab may recognize that
you pushed to your fork and will send you a message containing
a link to create a **new merge request** on the GitLab website.
If you do not receive this message by any chance, there is also
a guide by GNOME on creating a new merge request at
[welcome.gnome.org](https://welcome.gnome.org/en/app/Loupe/#creating-a-merge-request).

## Git Commit Naming Convention

Git commit messages should follow the guidelines for commit
messages provided in the
[GNOME Handbook](https://handbook.gnome.org/development/commit-messages.html).

The following is the structure of a commit message, along
with an example of a standard commit message.

`<directory>: <summary>` e.g. "po: Updated Mexican Spanish translation"

We also encourage to provide a more elaborate description of
your changes inside your commit description.

```
po: Updated Mexican Spanish translation

Update the translation for "Library", which was previously
"Biblioteca" to "Fototeca". This word is unique to the
language and is a more suitable translation for a library
of photos, not books!

Closes #1234
```

"As a general rule, it is always better to write too much in the
commit message body than too little."
(GNOME/[gnome-shell](https://gitlab.gnome.org/GNOME/gnome-shell/-/blob/9f5a323e06d6b5b3818d934ba5b31c437c4c07b3/docs/commit-messages.md))

### Draft Merge Requests

Merge requests marked as draft will not be reviewed by Memories'
maintainers or merged. When the change is ready for review please
mark the merge request as ready.

### Inactive Merge Requests

If a merge request has comments from maintainers that have not been
responded to within 4 weeks this merge request is considered to be
inactive and will be closed. The reporter may re-open it at a later
date if they respond to the comments.

### Making a Release

This section of the document is more of a personal note for the
maintainer, and future maintainers of Memories. See the
[GNOME Handbook](https://handbook.gnome.org/maintainers/making-a-release.html)
for more details.

Release versioning for Memories should follow the GNOME release
schedule.
See the [GNOME Handbook](https://handbook.gnome.org/maintainers.html).

#### Setup Checklist

- Ensure [git-evtag](https://github.com/cgwalters/git-evtag) is
installed on your machine. This software will be used to provide
strong signing guarantees when creating a new git release tag.
- Verify your user `.gitconfig` file has your **SSH** key and PGP
(**GPG**) key configured for authentication to GNOME GitLab and
signing Git commits and tags.
- Verify that your GNOME GitLab account has your **public** SSH/PGP
keys up to date. These should always match the keys used when
releasing. To verify, go to your
[GitLab key settings](https://gitlab.gnome.org/-/user_settings/gpg_keys#index).
- `git status` and `git pull` to ensure local repository is up to date.

#### Release Commit Checklist

- Update project version in root `meson.build` file.
- Update crate version in `Cargo.toml` manifest file.
- Update the `CHANGELOG` text document.
- Add a `<release>` entry in the Appstream app metadata file. You
should read through the Git commit log for this release and come up
with a bullet point for each significant change and credit the
people who did the work. Refer to the
[Appstream specification](https://www.freedesktop.org/software/appstream/docs/).
- Review the `README.md` document and update if necessary.
- Commit changes via `git commit`, or `git commit -S<key-id>` if your
PGP (GPG) key is not globally configured in your user `.gitconfig`.
- Run `meson dist` to create the tarball for the release. If
successful, Meson should output similar to the following:
```
Distribution package /opt/gnome/build/glib/meson-dist/glib-2.57.3.tar.xz tested.
```
- Run `git evtag sign 47.0`. `git-evtag` is required, see setup. The
message included in the Git tag should be in the following format:
```
Memories 47.0

* The contents of the CHANGELOG file for this release.
* Dependency updates
* A bug fix
* Small maintenance tasks
* Translation updates with credits
```
- Upload the tarball (this applies once Memories uses GNOME master):
```
$ scp memories-47.0.tar.xz USER@master.gnome.org:~
$ ssh USER@master.gnome.org
$ ftpadmin install memories-47.0.tar.xz
```

### Updating Memories' Application Screenshots

Clone the following repository locally and set it as the only
root directory in the library collection setting:
[https://gitlab.gnome.org/maxrdz/memories-stock-photos](https://gitlab.gnome.org/maxrdz/memories-stock-photos)
