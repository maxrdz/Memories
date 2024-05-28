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

use crate::library::media_grid::media_cell::AlbumsMediaCell;
use adw::glib;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::g_warning;
use libadwaita as adw;
use std::ffi::OsStr;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PictureDetails(pub glycin::ImageInfo);

impl PictureDetails {
    pub fn pretty_print_dimensions(&self) -> String {
        let height: u32 = self.0.height;
        let width: u32 = self.0.width;

        format!("{} x {}", height, width)
    }
}

#[derive(Debug, Clone)]
pub struct VideoDetails;

/// Represents the detail information of a library
/// item, which can be a picture or a video.
#[derive(Debug, Default, Clone)]
pub enum ContentDetails {
    Picture(PictureDetails),
    Video(VideoDetails),
    #[default]
    Missing,
}

glib::wrapper! {
    pub struct AlbumsDetails(ObjectSubclass<imp::AlbumsDetails>)
        @extends gtk::Widget, adw::Bin;
}

impl AlbumsDetails {
    pub fn new() -> Self {
        glib::Object::new()
    }

    /// Updates the preference rows in the details widget with
    /// the content metadata from the `AlbumsMediaCell` object passed.
    pub fn update_details(&self, cell_data: &AlbumsMediaCell) {
        self.clear_rows();

        let file_info: &gio::FileInfo = cell_data.imp().file_info.get().unwrap();

        match cell_data.imp().content_details.borrow().deref() {
            ContentDetails::Picture(img_data) => {
                self.update_fileinfo(file_info);

                let metadata = cell_data.imp().file_metadata.get().unwrap();
                let size: String = metadata.pretty_print_bytes();
                let dimensions: String = img_data.pretty_print_dimensions();

                self.imp().dimensions.set_subtitle(&dimensions);
                self.imp().size.set_subtitle(&size);
            }
            ContentDetails::Video(_) => {
                self.update_fileinfo(file_info);
            }
            ContentDetails::Missing => {
                self.update_fileinfo(file_info);
            }
        }
    }

    /// Updates details that we get from the `GFileInfo` object.
    fn update_fileinfo(&self, file_info: &gio::FileInfo) {
        let filename: PathBuf = file_info.name();
        let file_ext: Option<&OsStr> = filename.extension();
        if file_ext.is_none() {
            g_warning!(
                "Details",
                "Got a missing extension while trying to update details."
            );
        }
        let ext_str = file_ext.unwrap().to_str().unwrap().to_uppercase();

        self.imp().format.set_subtitle(&ext_str);
    }

    /// Sets all `AdwActionRow` widget subtitles to their placeholder text.
    fn clear_rows(&self) {
        let imp = self.imp();
        Self::update_row(&imp.format, None::<String>);
        Self::update_row(&imp.dimensions, None::<String>);
        Self::update_row(&imp.folder, None::<String>);
        Self::update_row(&imp.size, None::<String>);
        Self::update_row(&imp.created, None::<String>);
        Self::update_row(&imp.modified, None::<String>);
        Self::update_row(&imp.timestamp, None::<String>);
        Self::update_row(&imp.aperture, None::<String>);
        Self::update_row(&imp.exposure, None::<String>);
        Self::update_row(&imp.iso, None::<String>);
        Self::update_row(&imp.focal_length, None::<String>);
        Self::update_row(&imp.make_model, None::<String>);
    }

    /// Modified snippet from GNOME Image Viewer (Loupe).
    /// Updates the given `AdwActionRow` with `value`.
    /// If `value` is `None`, a placeholder string is set.
    fn update_row(row: &adw::ActionRow, value: Option<impl AsRef<str>>) -> bool {
        if let Some(v) = value {
            row.set_subtitle(v.as_ref());
            true
        } else {
            // Translators: 'N/A' is short for 'Not Available'.
            row.set_subtitle(&gettext("N/A"));
            false
        }
    }
}

impl Default for AlbumsDetails {
    fn default() -> Self {
        Self::new()
    }
}
