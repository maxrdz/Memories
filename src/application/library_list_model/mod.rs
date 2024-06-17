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

use crate::globals::DEFAULT_LIBRARY_COLLECTION;
use crate::i18n::gettext_f;
use crate::window::MemoriesApplicationWindow;
use adw::subclass::prelude::*;
use glib::{g_critical, g_debug};
use gtk::{gio, glib};
use std::env;

glib::wrapper! {
    pub struct MemoriesLibraryListModel(ObjectSubclass<imp::MemoriesLibraryListModel>)
        @implements gio::ListModel;
}

impl MemoriesLibraryListModel {
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

    /// Setup code for initialize the library list model at start up of Memories.
    /// Passes newly constructed list model to the Memories application object.
    pub fn initialize_new_model(window: &MemoriesApplicationWindow) {
        let new_library_model = MemoriesLibraryListModel::default();

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
                    so Memories cannot open your photo library.",
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

impl Default for MemoriesLibraryListModel {
    fn default() -> Self {
        Self::new()
    }
}
