project('memories', 'rust',
          version: '46.3',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

i18n = import('i18n')
gnome = import('gnome')

base_id = 'com.maxrdz.Memories'
app_name = meson.project_name().substring(0, 1).to_upper()
app_name += meson.project_name().substring(1, 99)
binary = meson.project_name()
version = meson.project_version()
gettext_domain = meson.project_name()
base_gresource_domain = '/' + base_id.replace('.', '/')
gresource_domain = base_gresource_domain
project_git = 'https://gitlab.gnome.org/maxrdz/Memories'
vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD', check: false).stdout().strip()

# Keep dependencies below matching with the current
# gtk-rs 'gnome_xx' feature's dependencies, found at
# https://github.com/gtk-rs/gtk4-rs/blob/master/gtk4/Cargo.toml
dependency('gtk4', version: '>= 4.14')
dependency('cairo', version: '>= 1.16')
dependency('pango', version: '>= 1.52')
dependency('gdk-pixbuf-2.0', version: '>= 2.42')
dependency('gio-2.0', version: '>= 2.80')
dependency('glib-2.0', version: '>= 2.80')

dependency('libadwaita-1', version: '>= 1.5')
dependency('mount', version: '>= 2.39')
dependency('gstreamer-1.0', version: '>= 1.22')
dependency('lcms2', version: '>= 2.12.0') # Required by Glycin
dependency('libseccomp', version: '>= 2.5.0') # Required by Glycin

if get_option('use-feedbackd')
  dependency('libfeedback-0.0', version: '>= 0')
endif

# Development Dependencies
glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)
appstream_cli = find_program('appstreamcli', required: false)
cargo = find_program('cargo', required: true)

# Build Profile
if get_option('profile') == 'devel'
  profile = 'devel'
  application_id = '@0@.Devel'.format(base_id)
  gresource_domain = '/' + application_id.replace('.', '/')
  binary = '@0@-devel'.format(binary)
else
  profile = ''
  application_id = base_id
endif

# Install Directories
prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'po'

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)

meson.add_dist_script(
  'build-aux/meson/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / binary + '-' + version,
  meson.project_source_root()
)
