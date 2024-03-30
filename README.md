# Gallery

A free and open source photo and video gallery application designed
for mobile devices, built with GTK4 and libadwaita, aiming to be
well integrated with GNOME technologies and the Phosh shell.

## Building from Source

We use Meson/Ninja as the build system for Gallery.
The quickest way to build for release is to do the following:

### Getting the Source

```sh
git clone https://gitlab.gnome.org/maxrdz/gallery
cd gallery
```

### Build Gallery

Before building Gallery, make sure you have the required
dependencies installed. Meson will let you know what binaries or
libraries it is missing or cannot find, but you can also read the
root [Meson](./meson.build) build file to see the list of
dependencies it searches for on your system.

```sh
meson setup build
meson compile -C build
meson install -C build
```

You can append the `-Dprofile=dev` argument to build for debug:

```sh
meson setup builddevel -Dprofile=dev
meson compile -C builddevel
meson install -C builddevel
```

### Running from the source tree

If you would like to run Gallery without installing it on your
system, you can use `cargo` directly to build and run the source.

### Uninstalling

To uninstall the app build from your local system:
```sh
sudo ninja -C build uninstall
```
Replace `build` with the Meson build directory of the
application build version that you want to uninstall.

## Cross Compiling

For cross compiling, we use
[cross-rs](https://github.com/cross-rs/cross), which is a
drop-in replacement for Cargo. This tool allows the developer
to cross compile using **Docker** (or a Docker drop-in
replacement, such as [Podman](https://podman.io/))
instead of installing dependencies and additional pkg-conf
configuration on the build machine. On setup, Meson will check
that cross and docker, or an alternative, are installed.

To install the cross binary on your system user's cargo:
```sh
cargo install cross --git https://github.com/cross-rs/cross
```
NOTE: This will install the `cross` program under `~/.cargo/bin`.
Be sure to add `~/.cargo/bin` to your PATH so Meson can find it.

To setup a build that targets ARM64 GNU/Linux:

```sh
meson setup buildaarch64 -Dtarget=aarch64-unknown-linux-gnu
```

## Guidelines for Maintainers

We follow [Phosh's Guidelines for Maintainers](https://gitlab.gnome.org/World/Phosh/phosh/-/wikis/Guidelines-for-maintainers).

## Copyright and License

Copyright &copy; 2024 Max Rodriguez

Gallery is released under the terms of the GNU General Public
License, either version 3.0 or, at your option, any later
version. You can read the full license in the `COPYING` file.
