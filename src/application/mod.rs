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

mod imp;
pub mod library_list_model;

use crate::config::{APP_ID, APP_NAME, VCS_TAG};
use crate::globals::*;
use crate::i18n::gettext_f;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{clone, g_critical, g_debug, g_error};
use gtk::{gio, glib};

glib::wrapper! {
    pub struct MemoriesApplication(ObjectSubclass<imp::MemoriesApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MemoriesApplication {
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
                    app.set_adwaita_theme(PreferredAdwaitaTheme::System.value());
                    app.update_theme_action_states(&action.name());
                },
            )
            .build();
        let light_theme_action = gio::ActionEntry::builder("light-theme")
            .state(false.to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    app.set_adwaita_theme(PreferredAdwaitaTheme::Light.value());
                    app.update_theme_action_states(&action.name());
                },
            )
            .build();
        let dark_theme_action = gio::ActionEntry::builder("dark-theme")
            .state(false.to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    app.set_adwaita_theme(PreferredAdwaitaTheme::Dark.value());
                    app.update_theme_action_states(&action.name());
                },
            )
            .build();

        let toggle_autoplay_action = gio::ActionEntry::builder("toggle-autoplay")
            .state(self.gsettings().boolean("autoplay-videos").to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    let previous_state: glib::Variant = action.state().unwrap();

                    let previous_toggle: bool = bool::from_variant(&previous_state).unwrap();
                    let new_toggle: bool = !previous_toggle;

                    action.set_state(&new_toggle.to_variant());
                    app.toggle_autoplay(new_toggle);
                },
            )
            .build();

        // Application GAction for toggling FFmpeg hardware acceleration
        let toggle_hwaccel_action = gio::ActionEntry::builder("toggle-hardware-acceleration")
            .state(
                self.gsettings()
                    .boolean("ffmpeg-hardware-acceleration")
                    .to_variant(),
            )
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    let previous_state: glib::Variant = action.state().unwrap();

                    let previous_toggle: bool = bool::from_variant(&previous_state).unwrap();
                    let new_toggle: bool = !previous_toggle;

                    action.set_state(&new_toggle.to_variant());
                    app.toggle_ffmpeg_hardware_acceleration(new_toggle);
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
            toggle_autoplay_action,
            toggle_hwaccel_action,
            clear_cache_action,
            about_action,
            quit_action,
        ]);
    }

    fn update_theme_action_states(&self, action_name: &str) {
        match action_name {
            "system-theme" => {
                self.change_action_state("system-theme", &true.to_variant());
                self.change_action_state("dark-theme", &false.to_variant());
                self.change_action_state("light-theme", &false.to_variant());
            }
            "light-theme" => {
                self.change_action_state("system-theme", &false.to_variant());
                self.change_action_state("dark-theme", &false.to_variant());
                self.change_action_state("light-theme", &true.to_variant());
            }
            "dark-theme" => {
                self.change_action_state("system-theme", &false.to_variant());
                self.change_action_state("dark-theme", &true.to_variant());
                self.change_action_state("light-theme", &false.to_variant());
            }
            _ => g_error!(
                "Application",
                "update_theme_action_states() received an invalid action name."
            ),
        }
    }

    fn set_adwaita_color_scheme(&self, color_scheme: adw::ColorScheme) {
        let adw_style_manager: adw::StyleManager = adw::StyleManager::default();
        adw_style_manager.set_color_scheme(color_scheme);
    }

    fn toggle_autoplay(&self, toggle: bool) {
        self.toggle_gschema_key("autoplay-videos", toggle);
    }

    fn toggle_ffmpeg_hardware_acceleration(&self, toggle: bool) {
        self.toggle_gschema_key("ffmpeg-hardware-acceleration", toggle);
    }

    fn show_clear_app_cache_prompt(&self) {
        let window: gtk::Window = self.active_window().unwrap();

        let alert_dialog: adw::AlertDialog = adw::AlertDialog::builder()
            .heading(gettext("Clear App Cache?"))
            .body(gettext("Are you sure you want to clear the cache? This may result in a slower start up on the next launch."))
            .build();

        alert_dialog.add_responses(&[("cancel", &gettext("Cancel")), ("clear", &gettext("Clear Cache"))]);
        alert_dialog.set_response_appearance("clear", adw::ResponseAppearance::Destructive);

        alert_dialog.connect_response(
            None,
            clone!(@weak self as s => move |_: &adw::AlertDialog, response: &str| {
                if response == "clear" {
                    glib::spawn_future_local(async move {
                        let app_cache_dir: String = MemoriesApplication::get_app_cache_directory();

                        if let Err(io_error) = async_fs::remove_dir_all(&app_cache_dir).await {
                            match io_error.kind() {
                                std::io::ErrorKind::NotFound => (),
                                std::io::ErrorKind::PermissionDenied => g_critical!(
                                    "Application",
                                    "Insufficient permissions to clear cache directory."
                                ),
                                _ => g_error!(
                                    "Application",
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
            .application_name(gettext("Memories"))
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
          Initial release of Memories. Following the GNOME release schedule \
          as of GNOME version 46.3.\
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

    /// Returns a `String` that represents the absolute path of
    /// the user's cache directory, which is either the equivalent
    /// of the `$XDG_CACHE_HOME` env var, or `$HOME/.cache`.
    ///
    /// If the `$XDG_CACHE_HOME` environment variable is not present,
    /// and Memories is running as a sandboxed Flatpak application,
    /// `$HOME/.var/app/$FLATPAK_ID/cache` is returned.
    pub fn get_cache_directory() -> String {
        match std::env::var("XDG_CACHE_HOME") {
            Ok(value) => value,
            Err(e) => {
                g_debug!("Application", "$XDG_CACHE_HOME not found; Using fallback.");
                match e {
                    std::env::VarError::NotPresent => {
                        let user_home: String = std::env::var("HOME").expect("$HOME not present.");

                        match MemoriesApplication::is_flatpak() {
                            Some(flatpak_id) => {
                                format!("{}/.var/app/{}/cache", user_home, flatpak_id)
                            }
                            None => {
                                // If $XDG_CACHE_HOME is either not set or empty,
                                // a default equal to $HOME/.cache should be used.
                                // https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html#variables
                                format!("{}/.cache", user_home)
                            }
                        }
                    }
                    _ => {
                        g_error!("Application", "Unexpected std::env::VarError variant received.");
                        panic!(); // g_error! terminates for us; this just silences the compiler.
                    }
                }
            }
        }
    }

    /// Returns a `String` that represents the absolute
    /// path of the application's cache directory location.
    pub fn get_app_cache_directory() -> String {
        if MemoriesApplication::is_flatpak().is_some() {
            format!("{}/{}", MemoriesApplication::get_cache_directory(), APP_NAME)
        } else {
            // We can simply use `$XDG_CACHE_HOME` instead of `$XDG_CACHE_HOME/APP_NAME`
            // if we are running inside a Flatpak; See:
            // https://developer.gnome.org/documentation/tutorials/save-state.html
            MemoriesApplication::get_cache_directory()
        }
    }

    fn toggle_gschema_key(&self, key: &str, toggle: bool) {
        if let Err(err_msg) = self.gsettings().set_boolean(key, toggle) {
            g_critical!("Application", "GSettings returned error: {}", err_msg);
        }
    }

    /// Returns Some($FLATPAK_ID) if in a Flatpak sandbox environment.
    pub fn is_flatpak() -> Option<String> {
        if let Ok(var) = std::env::var("FLATPAK_ID") {
            assert!(var == APP_ID, "$FLATPAK_ID doesn't match APP_ID!");
            Some(var)
        } else {
            None
        }
    }
}

impl Default for MemoriesApplication {
    fn default() -> Self {
        gio::Application::default()
            .and_downcast::<MemoriesApplication>()
            .unwrap()
    }
}
