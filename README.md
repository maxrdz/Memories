# Gallery

A free and open source photo/video gallery app for Linux mobile,
built with GTK4 and libadwaita, designed to be well integrated
with GNOME technologies and mobile devices.

## Software Requirements

- glib version 2+
- gio version 2+
- desktop-file-utils
- appstream
- appstream-glib
- gtk4
- libadwaita 1.4
- Rustup (provides Cargo & rustc)
- Ninja build system
- Meson build system

## Building from Source

To build for release:

```sh
meson setup build
meson compile -C build
meson install -C build
```

To build for debug:

```sh
meson setup builddevel -Dprofile=dev
meson compile -C builddevel
meson install -C builddevel
```

## License

Gallery is licensed under the GNU GPL version 3.0.
You can read the full license in the `COPYING` file.
