#!/bin/bash
# Intended to be called from the project root directory.
# Synchronizes the Gitlab Git repository to the Github mirror repo.

GITLAB_USER=maxrdz
GITLAB_NAME=Albums
MIRROR_USER=maxrdz
MIRROR_NAME=Albums

git clone --bare https://gitlab.gnome.org/$GITLAB_USER/$GITLAB_NAME.git/
cd $GITLAB_NAME.git
git push --mirror git@github.com:$MIRROR_USER/$MIRROR_NAME
rm -rf $GITLAB_NAME.git
