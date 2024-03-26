project_name = meson.project_name()
gnome = import('gnome')

# ----- Generated Source Files ----- #

pkgdatadir = get_option('prefix') / get_option('datadir') / project_name

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', project_name)
conf.set_quoted('LOCALEDIR', get_option('prefix') / get_option('localedir'))
conf.set_quoted('PKGDATADIR', pkgdatadir)

configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: conf
)

# This file is Generated until actual compile time.
# The Cargo custom target handles copying the out file to the src dir.
vcs_tag(input: 'vcs.rs.in', output: 'vcs.rs')

# Copy the config.rs source file to the source directory.
run_command(
  'cp',
  meson.project_build_root() / 'src' / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true
)

# ------ Compile GResource Bundle ------ #

gnome.compile_resources(project_name,
  '@0@.gresource.xml'.format(project_name),
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

# ------------ Cargo Setup ------------- #

out_binary_file = project_name

cargo_bin  = find_program('cargo')
cargo_opts  = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opts += [ '--target-dir', meson.project_build_root()  / 'src' ]
cargo_env  = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

if get_option('profile') == 'default' or get_option('profile') == 'release'
  cargo_opts += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  out_binary_file += '-dev'
  message('Building in debug mode')
endif

# ---------- Cross Compilation ---------- #

file_extension = '' # Unix-like default

if get_option('target') != ''
  cargo_opts += [ '--target', get_option('target') ]
  if get_option('target').contains('windows')
    file_extension = '.exe'
    bindir = ''
  endif
endif

# ---------- Cargo Build Target ---------- #

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: out_binary_file + file_extension,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'env', cargo_env,
    cargo_bin, 'build',
    cargo_opts,
    # Since vcs_tag() runs at compile time, we copy it over to source until here.
    '&&', 'cp', 'src' / 'vcs.rs', meson.project_source_root() / 'src',
    # Replace below with `--out-dir` once stable.
    # Issue: https://github.com/rust-lang/cargo/issues/6790#issuecomment-655728327=
    '&&', 'cp', 'src' / rust_target / project_name + file_extension, '@OUTPUT@',
  ]
)
