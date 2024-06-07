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

use crate::globals::DEFAULT_LIBRARY_COLLECTION;
use crate::i18n::gettext_f;
use crate::window::AlbumsApplicationWindow;
use adw::subclass::prelude::*;
use glib::{clone, g_critical, g_debug};
use gtk::{gio, glib};
use std::env;

glib::wrapper! {
    pub struct AlbumsLibraryListModel(ObjectSubclass<imp::AlbumsLibraryListModel>)
        @implements gio::ListModel;
}

impl AlbumsLibraryListModel {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn connect_error_notify<F>(&self, callback: F) -> glib::signal::SignalHandlerId
    where
        F: Fn(&gtk::DirectoryList) + 'static,
    {
        // FIXME: temp compiler silence fix
        self.imp()
            .root_models
            .borrow()
            .first()
            .unwrap()
            .model
            .connect_error_notify(callback)
    }

    /// Setup code for initialize the library list model at start up of Albums.
    /// Passes newly constructed list model to the Albums application object.
    pub fn initialize_new_model(window: &AlbumsApplicationWindow) {
        let new_library_model = AlbumsLibraryListModel::default();

        // When the directory path(s) of the library list model are updated,
        // append the folder paths on the preferences page for user configuration.
        new_library_model.connect_refresh_widget_rows_notify(
            clone!(@weak window => move |list_model: &AlbumsLibraryListModel| {
                g_debug!("LibraryListModel", "notify::refresh_widget_rows");
                window.imp().preferences_view.clear_folder_entries();

                for subdir in &list_model.subdirectories() {
                    window.imp().preferences_view.append_folder_entry(
                        gio::File::for_path(&subdir.to_string())
                    );
                }
            }),
        );
        // `refresh-widget-rows` is notified on the `notify::subdirectories` signal,
        // but that signal is first emitted when constructed, and we assign a
        // callback to `notify::refresh-widget-rows` until after the constructor.
        // So, we manually emit it here. Will be emitted automatically going forward.
        new_library_model.notify_refresh_widget_rows();

        window.app().unwrap().set_library_list_model(new_library_model);
    }

    pub fn start_enumerating_items(&self) -> Result<(), String> {
        // We need to get the user's home directory first, via env var.
        let home_path: String = {
            if let Ok(home_path) = env::var("HOME") {
                home_path
            } else {
                g_critical!(
                    "LibraryListModel",
                    "No $HOME env var found! Cannot open library collection."
                );
                return Err(gettext_f(
                    // TRANSLATORS: You can remove odd spacing. This is due to code linting.
                    "The {ENV_VAR} environment variable was found, \
                    so Albums cannot open your photo library.",
                    &[("ENV_VAR", "$HOME")],
                ));
            }
        };

        if self.subdirectories().is_empty() {
            // Probably the first launch, set the default library folders.
            let mut default_subdirs: glib::StrV = glib::StrV::default();

            for folder in DEFAULT_LIBRARY_COLLECTION {
                default_subdirs.push(format!("{}/{}", home_path, folder).into());
            }
            // This property will synchronize with the corresponding gschema key.
            self.set_subdirectories(default_subdirs.clone());

            g_debug!(
                "LibraryListModel",
                "Enumerating library files from: {:?}",
                default_subdirs
            );
        }
        Ok(())
    }
}

impl Default for AlbumsLibraryListModel {
    fn default() -> Self {
        Self::new()
    }
}
