// application/mod.rs
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

mod imp;

use adw::gtk;
use adw::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};
use libadwaita as adw;

use crate::globals::*;
use crate::vcs::VCS_TAG;

glib::wrapper! {
    pub struct Album(ObjectSubclass<imp::Album>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Album {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        // The reason we have a separate action per theme is for allowing the
        // user to be able to set the application theme via keyboard shortcuts.
        let system_theme_action = gio::ActionEntry::builder("system-theme")
            .state(true.to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    app.set_adwaita_color_scheme(adw::ColorScheme::Default);
                    app.change_action_state("dark-theme", &false.to_variant());
                    app.change_action_state("light-theme", &false.to_variant());
                    action.set_state(&true.to_variant());
                },
            )
            .build();
        let light_theme_action = gio::ActionEntry::builder("light-theme")
            .state(false.to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    app.set_adwaita_color_scheme(adw::ColorScheme::ForceLight);
                    app.change_action_state("system-theme", &false.to_variant());
                    app.change_action_state("dark-theme", &false.to_variant());
                    action.set_state(&true.to_variant());
                },
            )
            .build();
        let dark_theme_action = gio::ActionEntry::builder("dark-theme")
            .state(false.to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    app.set_adwaita_color_scheme(adw::ColorScheme::ForceDark);
                    app.change_action_state("system-theme", &false.to_variant());
                    app.change_action_state("light-theme", &false.to_variant());
                    action.set_state(&true.to_variant());
                },
            )
            .build();

        let choose_album_dir_action = gio::ActionEntry::builder("choose-album-directory")
            .activate(move |_: &Self, _, _| ())
            .build();
        let configure_action = gio::ActionEntry::builder("configure")
            .parameter_type(Some(glib::VariantTy::INT32))
            .activate(move |_: &Self, _, _| ())
            .build();

        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        self.add_action_entries([
            system_theme_action,
            light_theme_action,
            dark_theme_action,
            choose_album_dir_action,
            configure_action,
            about_action,
            quit_action,
        ]);
    }

    fn set_adwaita_color_scheme(&self, color_scheme: adw::ColorScheme) {
        let adw_style_manager: adw::StyleManager = adw::StyleManager::default();
        adw_style_manager.set_color_scheme(color_scheme);
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
                    "A free and open source photo/video album app for Linux mobile, \
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
