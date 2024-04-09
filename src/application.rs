// application.rs
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

use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};
use libadwaita as adw;

use crate::globals::*;
use crate::master_window::MasterWindow;
use crate::vcs::VCS_TAG;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Gallery {}

    #[glib::object_subclass]
    impl ObjectSubclass for Gallery {
        const NAME: &'static str = "Gallery";
        type Type = super::Gallery;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Gallery {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for Gallery {
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

            window.set_title(Some(APP_INFO.app_title));

            window.present();
        }
    }

    impl GtkApplicationImpl for Gallery {}
    impl AdwApplicationImpl for Gallery {}
}

glib::wrapper! {
    pub struct Gallery(ObjectSubclass<imp::Gallery>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Gallery {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window: gtk::Window = self.active_window().unwrap();

        let about: adw::AboutDialog = adw::AboutDialog::builder()
            .application_icon(APP_INFO.app_id)
            .application_name(APP_INFO.app_title)
            .developer_name(APP_INFO.app_author)
            .version({
                if DEVELOPMENT_BUILD {
                    VCS_TAG
                } else {
                    APP_INFO.app_version
                }
            })
            .issue_url(format!("{}/issues", APP_INFO.app_repo).as_str())
            .developers(APP_INFO.authors)
            .artists(APP_INFO.artists.to_vec())
            //.documenters(APP_INFO.documenters.to_vec())
            .copyright(APP_INFO.copyright)
            .license(APP_INFO.license)
            .license_type(APP_INFO.license_type)
            .comments(format!(
                "{}\n\n{} (Git SHA1): {}",
                &gettext(
                    // TRANSLATORS: Generated POT file will have lots of whitespace.
                    // This is due to code linting. You can remove the whitespace in your PO file.
                    "A free and open source photo/video gallery app for Linux mobile, \
                        built with GTK4 and libadwaita, designed to be well integrated \
                        with GNOME technologies and mobile devices running Phosh.\
                        \n\nReleased under the GNU General Public License version 3.0."
                ),
                &gettext("Build Revision"),
                VCS_TAG
            ))
            .build();

        about.present(&window)
    }
}
