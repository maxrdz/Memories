subdir('icons')

desktop_conf = configuration_data()
desktop_conf.set('APP_NAME', app_name)
desktop_conf.set('BINARY', binary)
desktop_conf.set('ICON', application_id)

# ---------- Desktop File --------- #
desktop_file = i18n.merge_file(
  input: configure_file(
    input: '@0@.desktop.in.in'.format(base_id),
    output: '@0@.desktop.in'.format(application_id),
    configuration: desktop_conf
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
appdata_conf = configuration_data()
appdata_conf.set('APP_ID', application_id)
appdata_conf.set('BINARY', binary)
appdata_conf.set('PROJECT_GIT', project_git)
appdata_conf.set('GETTEXT_DOMAIN', gettext_domain)

appdata_file = i18n.merge_file(
  input: configure_file(
    input: '@0@.appdata.xml.in.in'.format(base_id),
    output: '@0@.appdata.xml.in'.format(base_id),
    configuration: appdata_conf
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

# ----------- Schemas ----------- #
install_data(
  '@0@.gschema.xml'.format(base_id),
  install_dir: datadir / 'glib-2.0' / 'schemas',
  rename: '@0@.gschema.xml'.format(application_id)
)

test('Validate schema file',
     glib_compile_schemas,
     args: ['--strict', '--dry-run', meson.current_source_dir()])
