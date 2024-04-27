// main.rs
//
// Copyright (c) 2024 Max Rodriguez
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod application;
mod config;
#[allow(dead_code)]
mod globals;
#[allow(dead_code)]
mod i18n;
mod library_view;
mod master_window;
mod preferences_view;
mod vcs;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use adw::gtk;
use application::Album;
use config::{APP_NAME, LOCALEDIR, PKGDATADIR, VERSION};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use glib_macros::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;
use std::env;

fn main() -> glib::ExitCode {
    let mut xdg_startup_data: Option<String> = None;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", globals::RUST_LOG_ENVVAR_DEFAULT);
        pretty_env_logger::init();
        info!(
            "No RUST_LOG env var found. Setting to default: '{}'",
            globals::RUST_LOG_ENVVAR_DEFAULT
        );
    } else {
        pretty_env_logger::init();
    }

    if let Ok(va) = env::var("DESKTOP_LAUNCH_ID") {
        env::remove_var("DESKTOP_LAUNCH_ID");
        xdg_startup_data = Some(va.clone());
        debug!("DESKTOP_LAUNCH_ID env var read and unset: {}", va);
    } else {
        // Expected per XDG startup notification specification:
        // https://specifications.freedesktop.org/startup-notification-spec/startup-notification-0.1.txt
        debug!("No DESKTOP_LAUNCH_ID environment variable received. Ignoring.");
    }

    info!(
        "{} v{}; Build revision (Git SHA1): {}",
        APP_NAME,
        VERSION,
        vcs::VCS_TAG
    );
    // Set up gettext translations.
    bindtextdomain(APP_NAME, LOCALEDIR).expect("Unable to bind the text domain!");
    bind_textdomain_codeset(APP_NAME, "UTF-8").expect("Unable to set the text domain encoding!");
    textdomain(APP_NAME).expect("Unable to switch to the text domain!");

    // Load the gresource bundle.
    let resources = gio::Resource::load(format!("{}/{}.gresource", PKGDATADIR.to_owned(), APP_NAME))
        .expect("Failed to load the gresource bundle!");
    gio::resources_register(&resources);

    let app = Album::new(globals::APP_INFO.app_id, &gio::ApplicationFlags::empty());

    if xdg_startup_data.is_some() {
        app.connect_activate(clone!(@strong xdg_startup_data as x => move |_: &Album| {
            () // TODO
        }));
    }
    app.run()
}
