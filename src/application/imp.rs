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

use super::library_list_model::AlbumsLibraryListModel;
use crate::config::{APP_ID, GRESOURCE_DOMAIN};
use crate::globals::CACHE_THUMBNAILS_SUBDIR;
use crate::utils::get_app_cache_directory;
use crate::window::AlbumsApplicationWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::g_debug;
use gtk::{gdk, gio, glib};
use std::cell::OnceCell;
use std::fs::{DirBuilder, File};
use std::path::Path;

#[derive(Debug, glib::Properties)]
#[properties(wrapper_type = super::AlbumsApplication)]
pub struct AlbumsApplication {
    pub(super) gsettings: gio::Settings,
    /// Core GListModel for enumerating photo and video album files.
    /// Initialized after the application window is presented.
    #[property(get, set)]
    pub library_list_model: OnceCell<AlbumsLibraryListModel>,
}

impl Default for AlbumsApplication {
    fn default() -> Self {
        Self {
            gsettings: gio::Settings::new(APP_ID),
            library_list_model: OnceCell::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsApplication {
    const NAME: &'static str = "AlbumsApplication";
    type Type = super::AlbumsApplication;
    type ParentType = adw::Application;
}

#[glib::derived_properties]
impl ObjectImpl for AlbumsApplication {
    fn constructed(&self) {
        g_debug!("AlbumsApplication", "Reached constructed()");
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

impl ApplicationImpl for AlbumsApplication {
    fn activate(&self) {
        let application = self.obj();

        let albums_cache_dir: String = get_app_cache_directory();
        let cache_subdirs: &[&str] = &[CACHE_THUMBNAILS_SUBDIR];

        // Before initializing the window, let's check our cache directory.
        // If the cache is missing, set the 'fresh-cache' gschema flag to true.
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

                        let _ = self.gsettings.set_boolean("fresh-cache", true);
                    }
                    _ => todo!(), // TODO: Extend error handling for cache check
                },
            }
        }

        // The activate() callback also notifies us when the user tries
        // to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        let window = if let Some(window) = application.active_window() {
            g_debug!("AlbumsApplication", "Application has an active window present!");
            window
        } else {
            g_debug!(
                "AlbumsApplication",
                "No active window found; Creating a new window."
            );
            let window = AlbumsApplicationWindow::new(&application);
            window.upcast()
        };

        window.set_title(Some(&gettext("Albums")));
        window.present();

        // Setup our own CSS provider from gresource
        let gdk_screen: gdk::Display = gdk::Display::default().unwrap();
        let new_css_provider: gtk::CssProvider = gtk::CssProvider::new();

        new_css_provider.load_from_resource(&format!("{}/style.css", GRESOURCE_DOMAIN));

        gtk::style_context_add_provider_for_display(
            &gdk_screen,
            &new_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

impl GtkApplicationImpl for AlbumsApplication {}
impl AdwApplicationImpl for AlbumsApplication {}
