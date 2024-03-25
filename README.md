# GNOME Gallery

A free and open source Photo/Video gallery app for Linux mobile,
built with Adwaita/GTK for the best experience on GNOME-based
phone shells, such as Phosh.

## Software Requirements

- glib version 2+
- gio version 2+
- desktop-file-utils
- appstream
- appstream-glib
- gtk4
- libadwaita
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

