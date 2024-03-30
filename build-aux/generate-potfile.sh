#!/bin/bash
# Original Source: https://gitlab.gnome.org/World/warp/-/blob/main/build-aux/generate-potfile.sh

PROJECT_NAME=gallery

src="$(find src/ -path '*.rs')"
ui="$(find src/ -path '*.ui')"

git ls-files \
	$src $ui "data/*.desktop.in.in" "data/*.xml.in.in" \
	> po/POTFILES.in

cd po || exit 1
intltool-update --maintain 2> /dev/null

cd ..
xgettext --add-comments --from-code=utf-8 --files-from=po/POTFILES.in -o po/$PROJECT_NAME.pot 2>/dev/null || (echo "Error running xgettext"; exit 1)
