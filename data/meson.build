subdir('icons')

config = configuration_data()
config.set('APP_NAME', app_name)
config.set('APP_ID', application_id)
config.set('ICON', application_id)
config.set('BINARY', binary)
config.set('PROJECT_GIT', project_git)
config.set('GRESOURCE_DOMAIN', gresource_domain)
config.set('GETTEXT_DOMAIN', gettext_domain)

# ---------- Desktop File --------- #
desktop_file = i18n.merge_file(
  input: configure_file(
    input: '@0@.desktop.in.in'.format(base_id),
    output: '@0@.desktop.in'.format(application_id),
    configuration: config
  ),
  output: '@0@.desktop'.format(application_id),
  type: 'desktop',
  po_dir: podir,
  install: true,
  install_dir: datadir / 'applications'
)

if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file.full_path()
    ],
    depends: desktop_file,
  )
endif

# ----------- App Data ----------- #
appdata_file = i18n.merge_file(
  input: configure_file(
    input: '@0@.appdata.xml.in.in'.format(base_id),
    output: '@0@.appdata.xml.in'.format(base_id),
    configuration: config
  ),
  output: '@0@.appdata.xml'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'appdata'
)

if appstream_util.found()
  test(
    'validate-appdata', appstream_util,
    args: [
      'validate', '--nonet', appdata_file.full_path()
    ],
    depends: appdata_file,
  )
endif

if appstream_cli.found()
  test(
    'validate-appstreamcli', appstream_cli,
    args: [
      'validate', '--no-color', '--no-net', appdata_file.full_path()
    ],
    depends: appdata_file,
  )
endif

# ----------- GSchema ----------- #
configure_file(
  input: '@0@.gschema.xml.in'.format(base_id),
  output: '@0@.gschema.xml'.format(application_id),
  configuration: config,
  install: true,
  install_dir: datadir / 'glib-2.0' / 'schemas'
)

test('Validate schema file',
     glib_compile_schemas,
     args: ['--strict', '--dry-run', meson.current_source_dir()])
