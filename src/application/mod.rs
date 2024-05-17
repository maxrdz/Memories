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

mod imp;

use crate::config::APP_ID;
use crate::i18n::gettext_f;
use crate::utils::get_app_cache_directory;
use crate::window::AlbumsApplicationWindow;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};
use gettextrs::gettext;
use glib::{g_critical, g_error};
use glib_macros::clone;
use libadwaita as adw;

use crate::globals::*;
use crate::vcs::VCS_TAG;

glib::wrapper! {
    pub struct AlbumsApplication(ObjectSubclass<imp::AlbumsApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AlbumsApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    /// Clones and returns a reference to the app's GSettings instance.
    pub fn gsettings(&self) -> gio::Settings {
        self.imp().gsettings.clone()
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

        let choose_album_dir_action = gio::ActionEntry::builder("choose-library-directory")
            .activate(move |_: &Self, _, _| ())
            .build();
        let configure_action = gio::ActionEntry::builder("configure")
            .parameter_type(Some(glib::VariantTy::INT32))
            .activate(move |_: &Self, _, _| ())
            .build();

        // Application GAction for toggling FFmpeg hardware acceleration
        let toggle_hwaccel_action = gio::ActionEntry::builder("toggle-hardware-acceleration")
            .state(self.gsettings().boolean("hardware-acceleration").to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    let previous_state: glib::Variant = action.state().unwrap();

                    let previous_toggle: bool = bool::from_variant(&previous_state).unwrap();
                    let new_toggle: bool = !previous_toggle;

                    action.set_state(&new_toggle.to_variant());
                    app.toggle_hardware_acceleration(new_toggle);
                },
            )
            .build();

        let clear_cache_action = gio::ActionEntry::builder("clear-app-cache")
            .activate(move |app: &Self, _, _| app.show_clear_app_cache_prompt())
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
            toggle_hwaccel_action,
            clear_cache_action,
            about_action,
            quit_action,
        ]);
    }

    fn set_adwaita_color_scheme(&self, color_scheme: adw::ColorScheme) {
        let adw_style_manager: adw::StyleManager = adw::StyleManager::default();
        adw_style_manager.set_color_scheme(color_scheme);
    }

    fn toggle_hardware_acceleration(&self, toggle: bool) {
        let window: gtk::Window = self.active_window().unwrap();
        let master_window: AlbumsApplicationWindow = window.downcast().unwrap();

        master_window.imp().library_view.set_hardware_accel(toggle);
    }

    fn show_clear_app_cache_prompt(&self) {
        let window: gtk::Window = self.active_window().unwrap();

        let alert_dialog: adw::AlertDialog = adw::AlertDialog::builder()
            .heading(gettext("Clear App Cache?"))
            .body(gettext("Are you sure you want to clear Albums' cache? This may result in a slower start up on the next launch."))
            .build();

        alert_dialog.add_responses(&[("cancel", &gettext("Cancel")), ("clear", &gettext("Clear Cache"))]);
        alert_dialog.set_response_appearance("clear", adw::ResponseAppearance::Destructive);

        alert_dialog.connect_response(
            None,
            clone!(@weak self as s => move |_: &adw::AlertDialog, response: &str| {
                if response == "clear" {
                    glib::spawn_future_local(async move {
                        let app_cache_dir: String = get_app_cache_directory();

                        if let Err(io_error) = async_fs::remove_dir_all(&app_cache_dir).await {
                            match io_error.kind() {
                                std::io::ErrorKind::NotFound => (),
                                std::io::ErrorKind::PermissionDenied => g_critical!(
                                    "AlbumsApplication",
                                    "Insufficient permissions to clear cache directory."
                                ),
                                _ => g_error!(
                                    "AlbumsApplication",
                                    "Received an unexpected error kind after trying to clear the cache."
                                ),
                            }
                        }
                    });
                }
            }),
        );
        alert_dialog.present(&window);
    }

    fn show_about(&self) {
        let window: gtk::Window = self.active_window().unwrap();

        let about: adw::AboutDialog = adw::AboutDialog::builder()
            .application_icon(APP_ID)
            .application_name(gettext("Albums"))
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

        about.set_release_notes(
            "<p>\
          Initial release of Albums. Following the GNOME release schedule \
          as of GNOME version 46.2.\
        </p>",
        );

        about.add_credit_section(
            Some(&gettext("Powered by the following technologies")),
            &[
                &gettext_f(
                    "The GNOME Project {WEBSITE}",
                    &[("WEBSITE", "https://www.gnome.org")],
                ),
                "GTK https://gtk.org/",
                "Libadwaita https://gnome.pages.gitlab.gnome.org/libadwaita/",
                "FFmpeg https://ffmpeg.org/",
                "glycin https://gitlab.gnome.org/sophie-h/glycin",
                "smol-rs https://github.com/smol-rs",
            ],
        );

        about.add_legal_section(
            "gtk-rs",
            Some("Copyright (C) 2020-2024 The gtk-rs Project Developers"),
            gtk::License::MitX11,
            None,
        );
        about.add_legal_section(
            "libadwaita-rs",
            Some("Copyright (C) 2021-2024 Bilal Elmoussaoui (bil.elmoussaoui@gmail.com)"),
            gtk::License::MitX11,
            None,
        );
        about.add_legal_section(
            "gettext-rs",
            Some(
                "Copyright (C) 2016 Konstantin Salikhov (koka58@yandex.ru)\n\
                Copyright (C) Alexander Batischev (eual.jp@gmail.com)\n",
            ),
            gtk::License::MitX11,
            None,
        );
        about.add_legal_section(
            "glycin",
            Some("Copyright (C) 2023-2024 Sophie Herold (sophieherold@gnome.org)"),
            gtk::License::Mpl20,
            None,
        );
        about.add_legal_section(
            "ffmpeg",
            Some("Copyright (C) 2000-2024 The FFmpeg Developers"),
            gtk::License::Lgpl21,
            None,
        );
        about.add_legal_section(
            "smol-rs",
            Some("Copyright (C) 2020-2024 Stjepan Glavina (stjepang@gmail.com)"),
            gtk::License::MitX11,
            None,
        );
        about.add_legal_section(
            "libadwaita",
            Some(
                "Copyright (C) 2018 Adrien Plazas (adrien.plazas@puri.sm)\
                Copyright (C) 2018 Bob Ham (bob.ham@puri.sm)\
                Copyright (C) 2018 Dorota Czaplejewicz (dorota.czaplejewicz@puri.sm)\
                Copyright (C) 2018 Guido Günther (agx@sigxcpu.org)\
                Copyright (C) 2018 Heather Ellsworth (heather.ellsworth@puri.sm)\
                Copyright (C) 2018 Julian Richen (julian@richen.io)\
                Copyright (C) 2018 Julian Sparber (julian@sparber.net)\
                Copyright (C) 2018 Sebastien Lafargue (slafargue@gnome.org)\
                Copyright (C) 2019 Zander Brown (zbrown@gnome.org)",
            ),
            gtk::License::MitX11,
            None,
        );
        about.add_legal_section(
            "gtk",
            Some(
                "Copyright (C) 2000-2020 Alexander Larsson (alexl@redhat.com)\n\
                Copyright (C) 2008-2024 Benjamin Otte (otte@gnome.org)\n\
                Copyright (C) 2004-2024 Carlos Garnacho (mrgarnacho@gmail.com)\n\
                Copyright (C) Carsten Haitzler (raster@gtk.org)\n\
                Copyright (C) 2009-2024 Christian Hergert (chergert@gnome.org)\n\
                Copyright (C) 2013-2024 Chun-wei Fan (fanchunwei@src.gnome.org)\n\
                Copyright (C) Damon Chaplin (damon@gtk.org)\n\
                Copyright (C) Elliot Lee (sopwith@gtk.org)\n\
                Copyright (C) 2006-2024 Emmanuele Bassi (ebassi@gnome.org)\n\
                Copyright (C) 2011-2022 Federico Mena (quartic@gtk.org)\n\
                Copyright (C) Ian Main (imain@gtk.org)\n\
                Copyright (C) Jay Painter (jpaint@gtk.org)\n\
                Copyright (C) Jeff Garzik (jgarzik@gtk.org)\n\
                Copyright (C) Jerome Bolliet (bolliet@gtk.org)\n\
                Copyright (C) 2015-2023 Jonas Ådahl (jadahl@gmail.com)\n\
                Copyright (C) 1995-1997 Josh MacDonald (jmacd@xcf.berkeley.edu)\n\
                Copyright (C) Lars Hamann (lars@gtk.org)\n\
                Copyright (C) 2001-2007 Manish Singh (manish@gtk.org)\n\
                Copyright (C) 2013-2024 Matthias Clasen (mclasen@redhat.com)\n\
                Copyright (C) 1998-2016 Owen Taylor (otaylor@gtk.org)\n\
                Copyright (C) Paolo Molaro (lupus@gtk.org)\n\
                Copyright (C) 1995-1997 Peter Mattis (petm@xcf.berkeley.edu)\n\
                Copyright (C) Raja R Harinath (harinath@gtk.org)\n\
                Copyright (C) Raph Levien (raph@gtk.org)\n\
                Copyright (C) Shawn T. Amundson (amundson@gtk.org)\n\
                Copyright (C) 1995-1997 Spencer Kimball (spencer@xcf.berkeley.edu)\n\
                Copyright (C) Stefan Jeske (stefan@gtk.org)\n\
                Copyright (C) 2013-2022 Tim Bäder (mail@baedert.org)\n\
                Copyright (C) 1998-2007 Tim Janik (timj@gtk.org)\n\
                Copyright (C) Tony Gale (gale@gtk.org)\n",
            ),
            gtk::License::Gpl20,
            None,
        );
        about.add_legal_section(
            "GNU gettext",
            Some("Copyright (C) 1995-2024 Free Software Foundation, Inc."),
            gtk::License::Gpl30,
            None,
        );
        about.present(&window)
    }
}

impl Default for AlbumsApplication {
    fn default() -> Self {
        gio::Application::default()
            .and_downcast::<AlbumsApplication>()
            .unwrap()
    }
}
