// application/imp.rs
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

use crate::master_window::MasterWindow;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::glib;
use libadwaita as adw;

#[derive(Debug, Default)]
pub struct Album {}

#[glib::object_subclass]
impl ObjectSubclass for Album {
    const NAME: &'static str = "Album";
    type Type = super::Album;
    type ParentType = adw::Application;
}

impl ObjectImpl for Album {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_gactions();
        obj.set_accels_for_action("win.settings", &["<primary>comma"]);

        obj.set_accels_for_action("app.system-theme", &["<primary><shift>s"]);
        obj.set_accels_for_action("app.light-theme", &["<primary><shift>l"]);
        obj.set_accels_for_action("app.dark-theme", &["<primary><shift>d"]);

        obj.set_accels_for_action("app.about", &["<primary>a"]);
        obj.set_accels_for_action("app.quit", &["<primary>q"]);
    }
}

impl ApplicationImpl for Album {
    fn activate(&self) {
        let application = self.obj();

        // The activate() callback also notifies us when the user tries
        // to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        let window = if let Some(window) = application.active_window() {
            window
        } else {
            let window = MasterWindow::new(&*application);
            window.upcast()
        };

        window.set_title(Some(&gettext("Album")));
        window.present();

        // Setup our own CSS provider from gresource
        let gdk_screen: gdk::Display = gdk::Display::default().unwrap();
        let new_css_provider: gtk::CssProvider = gtk::CssProvider::new();
        new_css_provider.load_from_resource("/com/maxrdz/Album/style.css");

        gtk::style_context_add_provider_for_display(
            &gdk_screen,
            &new_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

impl GtkApplicationImpl for Album {}
impl AdwApplicationImpl for Album {}
