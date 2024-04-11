i18n.gettext(
    meson.project_name(),
    preset: 'glib',
    args: [
        '--add-comments',
        '--from-code=utf-8',
        # Handle our gettext wrapper funcs from src/i18n.rs
        '--keyword=gettext_f',
        '--keyword=ngettext_f:1,2',
    ]
)
