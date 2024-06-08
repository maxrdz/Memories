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

use crate::application::library_list_model::AlbumsLibraryListModel;
use crate::application::AlbumsApplication;
use crate::window::AlbumsApplicationWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{clone, g_debug};
use gtk::{gio, glib};
use std::path::PathBuf;

glib::wrapper! {
    pub struct AlbumsPreferencesDialog(ObjectSubclass<imp::AlbumsPreferencesDialog>)
        @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl AlbumsPreferencesDialog {
    pub fn new(win: &AlbumsApplicationWindow) -> Self {
        let instance: AlbumsPreferencesDialog = glib::Object::new();

        let app: AlbumsApplication = win.app().unwrap();
        let library_model: AlbumsLibraryListModel = app.library_list_model();

        // When the directory path(s) of the library list model are updated,
        // append the folder paths on the preferences page for user configuration.
        library_model.connect_refresh_widget_rows_notify(
            clone!(@weak instance => move |list_model: &AlbumsLibraryListModel| {
                g_debug!("LibraryListModel", "notify::refresh_widget_rows");
                instance.clear_folder_entries();

                for subdir in &list_model.subdirectories() {
                    instance.append_folder_entry(
                        gio::File::for_path(&subdir.to_string())
                    );
                }
            }),
        );
        // `refresh-widget-rows` is notified on the `notify::subdirectories` signal,
        // but that signal is first emitted when constructed, and we assign a
        // callback to `notify::refresh-widget-rows` after the model is constructed.
        // So, we manually emit it here. Will be emitted automatically going forward.
        library_model.notify_refresh_widget_rows();

        instance
    }

    /// Appends a new `AdwActionRow` to the "Library Collection"
    /// `AdwPreferencesGroup` widget of the preferences page.
    pub fn append_folder_entry(&self, folder: gio::File) {
        let new_widget: adw::ActionRow = AlbumsPreferencesDialog::build_folder_row(&folder);

        self.imp().library_collection.add(&new_widget);
        self.imp().library_collection_rows.borrow_mut().push(new_widget);
    }

    /// Clears all children of `library_collection` preferences group.
    pub fn clear_folder_entries(&self) {
        for row_widget in self.imp().library_collection_rows.borrow().iter() {
            self.imp().library_collection.remove(row_widget);
        }
        self.imp().library_collection_rows.borrow_mut().clear();
    }

    /// Builds a new `AdwActionRow` widget object based on the `GFile` given.
    /// Represents a subdirectory configured to be part of the library collection.
    pub fn build_folder_row(folder: &gio::File) -> adw::ActionRow {
        let file_path_buf: PathBuf = folder.path().unwrap();

        let basename: String = folder.basename().unwrap().to_string_lossy().to_string();
        let absolute_path: String = file_path_buf.to_string_lossy().to_string();

        let new_action_row: adw::ActionRow = adw::ActionRow::builder()
            .title(basename)
            .subtitle(absolute_path)
            .build();

        let remove_entry_button: gtk::Button =
            gtk::Button::builder().margin_top(10).margin_bottom(10).build();

        let button_context: adw::ButtonContent = adw::ButtonContent::builder()
            .icon_name("list-remove")
            .tooltip_text(gettext("Remove Folder"))
            .build();

        remove_entry_button.set_property("child", button_context);
        new_action_row.add_suffix(&remove_entry_button);

        new_action_row
    }
}
