project('gnome-gallery', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

# ---------- Imports ---------- #

i18n = import('i18n')
gnome = import('gnome')

# -------- Dependencies -------- #

dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')
dependency('gtk4', version: '>= 4.10.0')
dependency('libadwaita-1', version: '>= 1.4.alpha')

# ------ Dev Dependencies ------ #

glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)
appstream_cli = find_program('appstreamcli', required: false)
cargo = find_program('cargo', required: true)

# ------- Build Profile ------- #

base_id = 'com.maxrdz.GnomeGallery'

if get_option('profile') == 'dev'
  profile = 'dev'
  application_id = '@0@.Dev'.format(base_id)
else
  profile = ''
  application_id = base_id
endif

# ---- Install Directories ---- #

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'po'
gettext_package = meson.project_name()

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)
