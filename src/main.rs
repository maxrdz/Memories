// This file is part of Albums.
//
// Copyright (c) 2024 Max Rodriguez
// All rights reserved.
//
// Albums is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Albums is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Albums.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod albums;
mod application;
mod config;
#[allow(dead_code)]
mod globals;
#[allow(dead_code)]
mod i18n;
mod library;
mod preferences;
mod thumbnails;
mod utils;
mod vcs;
mod window;

use adw::gtk;
use application::AlbumsApplication;
use config::{APP_ID, APP_NAME, GETTEXT_DOMAIN, LOCALEDIR, PKGDATADIR, VERSION};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::glib::{g_debug, g_info};
use gtk::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;
use std::env;
use std::fs::{DirBuilder, File};
use std::path::Path;

fn main() -> glib::ExitCode {
    if let Ok(v) = env::var("RUST_LOG") {
        if v.as_str() == "debug" {
            env::set_var("G_MESSAGES_DEBUG", "all");
        };
    } else {
        env::set_var("RUST_LOG", globals::RUST_LOG_ENVVAR_DEFAULT);
        env::set_var("G_MESSAGES_DEBUG", globals::G_MESSAGES_DEBUG_DEFAULT);
        g_info!(
            "Albums",
            "No RUST_LOG env var found. Setting to default: '{}'",
            globals::RUST_LOG_ENVVAR_DEFAULT
        );
    }
    g_info!(
        "Albums",
        "{} v{}; Build revision (Git SHA1): {}",
        APP_NAME,
        VERSION,
        vcs::VCS_TAG
    );

    let albums_cache_dir: String = utils::get_app_cache_directory();
    let cache_subdirs: &[&str] = &[globals::CACHE_THUMBNAILS_SUBDIR];

    for subdirectory in cache_subdirs {
        let absolute_path: String = format!("{}/{}", albums_cache_dir, subdirectory);

        match File::open(Path::new(&absolute_path)) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    g_debug!(
                        "Albums",
                        "Cache subdirectory '{}' does not exist. A new one will be made.",
                        absolute_path,
                    );
                    DirBuilder::new()
                        .recursive(true)
                        .create(absolute_path)
                        .expect("Failed to create new cache subdirectory.");
                }
                _ => todo!(), // TODO: Extend error handling for cache check
            },
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

    let app = AlbumsApplication::new(APP_ID, &gio::ApplicationFlags::empty());
    app.run()
}
