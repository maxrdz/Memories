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
mod globals;
mod i18n;
mod library_list_model;
mod library_view;
mod master_window;
mod vcs;

use adw::gtk;
use application::Gallery;
use config::{APP_NAME, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

fn main() -> glib::ExitCode {
    // Set up gettext translations.
    bindtextdomain(APP_NAME, LOCALEDIR).expect("Unable to bind the text domain!");
    bind_textdomain_codeset(APP_NAME, "UTF-8").expect("Unable to set the text domain encoding!");
    textdomain(APP_NAME).expect("Unable to switch to the text domain!");

    // Load the gresource bundle.
    let resources = gio::Resource::load(format!("{}/{}.gresource", PKGDATADIR.to_owned(), APP_NAME))
        .expect("Failed to load the gresource bundle!");
    gio::resources_register(&resources);

    let app = Gallery::new(globals::APP_INFO.app_id, &gio::ApplicationFlags::empty());
    app.run()
}
