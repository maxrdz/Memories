project_name = meson.project_name()
gnome = import('gnome')

# ----- Generated Source Files ----- #
pkgdatadir = get_option('prefix') / get_option('datadir') / project_name

conf = configuration_data()
conf.set_quoted('APP_ID', application_id)
conf.set_quoted('APP_NAME', project_name)
conf.set_quoted('APP_REPO', project_git)
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('LOCALEDIR', get_option('prefix') / get_option('localedir'))
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('GETTEXT_DOMAIN', gettext_domain)
conf.set('BASE_GRESOURCE_DOMAIN', base_gresource_domain)
conf.set('GRESOURCE_DOMAIN', gresource_domain)
conf.set_quoted('VCS_TAG', vcs_tag)

configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: conf
)

# Copy the config.rs source file to the source directory.
run_command(
  'cp',
  meson.project_build_root() / 'src' / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true
)

# ------ Compile GResource Bundle ------ #
gnome.compile_resources(project_name,
  configure_file(
    input: '@0@.gresource.xml.in'.format(project_name),
    output: '@0@.gresource.xml'.format(project_name),
    configuration: conf
  ),
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

# ------------ Cargo Setup ------------- #
out_binary_file = project_name

cargo_bin = find_program('cargo')
cargo_opts = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opts += [ '--target-dir', meson.project_build_root()  / 'src' ]
cargo_env = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

cargo_doc_opts = ['--document-private-items']
cargo_doc_opts += ['--config', 'build.rustdocflags="--default-theme=ayu"']

if get_option('profile') == 'default' or get_option('profile') == 'release'
  cargo_opts += [ '--release' ]
  rust_target = 'release'
  message('Building Memories for Release.')
else
  rust_target = 'debug'
  out_binary_file += '-devel'
  message('Building Memories for Debug.')
endif

if get_option('disable-glycin-sandbox')
  cargo_opts += [ '--features', 'disable-glycin-sandbox' ]
endif

if get_option('use-feedbackd')
  cargo_opts += [ '--features', 'use-feedbackd' ]
endif

# ---------- Cross Compilation ---------- #
if get_option('target') != ''
  cargo_opts += [ '--target', get_option('target') ]

  # For cross compiling, we use the cross-rs 
  # project for building our application via Docker.
  # This makes the cross compilation process simpler,
  # and requires less setup on the developer's machine.

  find_program(['docker', 'podman']) # podman also works
  cargo_bin  = find_program('cross')
  cargo_env += [ 'CROSS_BUILD_OPTS=--network=host' ]

  if get_option('cross-verbose')
    cargo_opts += [ '-v' ]
  endif
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: out_binary_file,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'env', cargo_env,
    cargo_bin, 'build',
    cargo_opts,
    # Replace below with `--out-dir` once stable.
    # Issue: https://github.com/rust-lang/cargo/issues/6790#issuecomment-655728327=
    '&&', 'cp', 'src' / rust_target / project_name, '@OUTPUT@',
  ]
)

cargo_doc = custom_target(
  'cargo-doc',
  build_by_default: false,
  build_always_stale: true,
  output: 'doc',
  console: true,
  command: [
    cargo_bin, 'doc',
    cargo_opts + cargo_doc_opts,
  ],
)

test(
  'cargo-test',
  cargo_bin,
  args: [
    'test',
    cargo_opts,
    '--',
    '--test-threads=1',
  ],
  is_parallel: false,
  timeout: 1800,
)
