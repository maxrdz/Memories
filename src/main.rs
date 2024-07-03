// This file is part of Memories.
//
// Copyright (c) 2024 Max Rodriguez
// All rights reserved.
//
// Memories is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Memories is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Memories.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

/*!
# Memories
<img src="https://gitlab.gnome.org/maxrdz/Memories/-/raw/main/data/icons/com.maxrdz.Memories.Devel.svg?ref_type=heads" height=10%>

Source Code Documentation

## Prelude
Memories uses GObject-based C libraries, which we interface with via
Rust bindings. Memories source follows the file structure and file
naming convention demonstrated by the
[gtk-rs examples](https://github.com/gtk-rs/gtk4-rs/tree/master/examples).

All GObject subclasses should be named in
[UpperCamelCase](https://en.wikipedia.org/wiki/Camel_case),
and prefixed with project codename. e.g. "MemoriesLibraryView"

All use of logging via GLib should to set the log domain name to
the name of the GObject that the logic is from, without the project
codename prefix. e.g. "MediaCell" for the subclass, "MemoriesMediaCell"

Most of the application's widget hierarchy is created by the program via
[GtkBuilder](https://docs.gtk.org/gtk3/class.Builder.html), which parses
`.ui` files, which are in XML format, and stored in the application's
[gresource bundle](https://docs.gtk.org/gio/struct.Resource.html).

## Application Widget Structure
Simplified widget tree including the most important widgets.

- [`MemoriesApplicationWindow`]
    - [`AdwNavigationView`]
        - [`AdwNavigationPage`]
            - [`AdwViewStack`]
                - [`MemoriesLibraryView`]
                - [`MemoriesAlbumsView`]
                - [`MemoriesFavoritesView`]

[`AdwNavigationView`]: adw::NavigationView
[`AdwNavigationPage`]: adw::NavigationPage
[`AdwViewStack`]: adw::ViewStack

[`MemoriesApplicationWindow`]: window::MemoriesApplicationWindow
[`MemoriesLibraryView`]: library::MemoriesLibraryView
[`MemoriesAlbumsView`]: albums::MemoriesAlbumsView
[`MemoriesFavoritesView`]: favorites::MemoriesFavoritesView
*/

#![doc(
    html_logo_url = "https://gitlab.gnome.org/maxrdz/Memories/-/raw/main/data/icons/com.maxrdz.Memories.Devel.svg?ref_type=heads"
)]

mod albums;
mod application;
mod config;
#[allow(dead_code)]
mod globals;
#[allow(dead_code)]
mod i18n;
mod library;
mod util;
mod window;

use application::MemoriesApplication;
use config::{APP_ID, APP_NAME, GETTEXT_DOMAIN, LOCALEDIR, PKGDATADIR, VERSION};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
#[cfg(feature = "use-feedbackd")]
use gtk::glib::g_error;
use gtk::glib::g_info;
use gtk::prelude::*;
use gtk::{gio, glib};
use std::env;

fn main() -> glib::ExitCode {
    if let Ok(v) = env::var("RUST_LOG") {
        if v.as_str() == "debug" {
            env::set_var("G_MESSAGES_DEBUG", "all");
        };
    } else {
        env::set_var("RUST_LOG", globals::RUST_LOG_ENVVAR_DEFAULT);
        env::set_var("G_MESSAGES_DEBUG", globals::G_MESSAGES_DEBUG_DEFAULT);
        g_info!(
            "Memories",
            "No RUST_LOG env var found. Setting to default: '{}'",
            globals::RUST_LOG_ENVVAR_DEFAULT
        );
    }

    let flatpak_id: Option<String> = MemoriesApplication::is_flatpak();

    g_info!(
        "Memories",
        "{} v{}{}; Build revision (Git SHA1): {}",
        APP_NAME,
        VERSION,
        {
            if flatpak_id.is_some() {
                format!(" [FLATPAK_ID: {}]", flatpak_id.clone().unwrap())
            } else {
                "".to_owned()
            }
        },
        config::VCS_TAG
    );

    // Search for the XDG_xxx_DIR environment variables that our
    // library collection will enumerate. If the env vars are not present,
    // create them by looking up the XDG user directory paths.
    for xdg_user_dir in globals::DEFAULT_LIBRARY_COLLECTION {
        let env_var: &str = xdg_user_dir.value().0;

        if env::var(env_var).is_err() {
            env::set_var(env_var, xdg_user_dir.get_path());
        }
    }

    // Set up gettext translations.
    bindtextdomain(GETTEXT_DOMAIN, LOCALEDIR).expect("Unable to bind the text domain!");
    bind_textdomain_codeset(GETTEXT_DOMAIN, "UTF-8").expect("Unable to set the text domain encoding!");
    textdomain(GETTEXT_DOMAIN).expect("Unable to switch to the text domain!");

    // Load the gresource bundle.
    let resources = gio::Resource::load(format!("{}/{}.gresource", PKGDATADIR.to_owned(), APP_NAME))
        .expect("Failed to load the gresource bundle!");
    gio::resources_register(&resources);

    // Initialize Lfb for haptic feedback.
    #[cfg(feature = "use-feedbackd")]
    if let Err(lfb_error) = libfeedback::init(APP_ID) {
        g_error!(
            "Memories",
            "Failed to initialize Lfb for haptic feedback: {}",
            lfb_error
        );
    }

    let app = MemoriesApplication::new(APP_ID, &gio::ApplicationFlags::empty());
    app.run()
}
