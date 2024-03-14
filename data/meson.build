subdir('icons')

desktop_conf = configuration_data()
desktop_conf.set('icon', application_id)

# ---------- Desktop File --------- #

desktop_file = i18n.merge_file(
  input: configure_file(
    input: '@0@.desktop.in.in'.format(application_id),
    output: '@BASENAME@',
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
appdata_conf.set('app-id', application_id)

appdata_file = i18n.merge_file(
  input: configure_file(
    input: '@0@.metainfo.xml.in.in'.format(base_id),
    output: '@BASENAME@',
    configuration: appdata_conf
  ),
  output: '@0@.metainfo.xml'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'metainfo'
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
