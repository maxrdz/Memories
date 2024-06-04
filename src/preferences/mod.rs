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
pub mod theme_selector;

use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};
use libadwaita as adw;
use std::path::PathBuf;

glib::wrapper! {
    pub struct AlbumsPreferencesView(ObjectSubclass<imp::AlbumsPreferencesView>)
        @extends gtk::Widget, adw::BreakpointBin,
        @implements gtk::Accessible, gtk::Buildable;
}

impl AlbumsPreferencesView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn append_folder_entry(&self, folder: gio::File) {
        self.imp()
            .library_collection
            .add(&AlbumsPreferencesView::build_folder_row(&folder));
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

impl Default for AlbumsPreferencesView {
    fn default() -> Self {
        Self::new()
    }
}
