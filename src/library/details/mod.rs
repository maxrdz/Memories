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

use crate::utils::MetadataInfo;
use adw::glib;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::g_warning;
use libadwaita as adw;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PictureDetails {
    pub file_info: gio::FileInfo,
    pub file_metadata: Option<MetadataInfo>,
    pub glycin: glycin::ImageInfo,
}

impl PictureDetails {
    pub fn pretty_print_bytes(&self) -> Result<String, ()> {
        if let Some(metadata) = &self.file_metadata {
            return Ok(metadata.pretty_print_bytes());
        }
        Err(())
    }

    pub fn pretty_print_dimensions(&self) -> String {
        let height: u32 = self.glycin.height;
        let width: u32 = self.glycin.width;
        // Glycin image info should always be in pixels.
        let unit: String = gettext("pixels");

        format!("{} x {} {}", height, width, unit)
    }
}

#[derive(Debug)]
pub struct VideoDetails {
    pub file_info: gio::FileInfo,
    pub file_metadata: Option<MetadataInfo>,
}

impl VideoDetails {
    pub fn pretty_print_bytes(&self) -> Result<String, ()> {
        if let Some(metadata) = &self.file_metadata {
            return Ok(metadata.pretty_print_bytes());
        }
        Err(())
    }
}

/// Represents the detail information
/// of a library item, which can be
/// a picture or a video.
#[derive(Debug)]
pub enum ContentDetails {
    Picture(PictureDetails),
    Video(VideoDetails),
}

glib::wrapper! {
    pub struct Details(ObjectSubclass<imp::Details>)
        @extends gtk::Widget, adw::Bin;
}

impl Details {
    pub fn new() -> Self {
        glib::Object::new()
    }

    /// Updates the preference rows in the details widget with
    /// the file metadata from the `ContentDetails` struct given.
    pub fn update_details(&self, data_enum: ContentDetails) {
        match data_enum {
            ContentDetails::Picture(data) => {
                let filename: PathBuf = data.file_info.name();
                let size: Result<String, ()> = data.pretty_print_bytes();
                let dimensions: String = data.pretty_print_dimensions();
                let file_ext: Option<&OsStr> = filename.extension();

                if file_ext.is_none() {
                    g_warning!(
                        "Details",
                        "Got a missing extension while trying to update details."
                    );
                }
                let ext_str = file_ext.unwrap().to_str().unwrap().to_uppercase();

                self.imp().format.set_subtitle(&ext_str);
                self.imp().dimensions.set_subtitle(&dimensions);
                self.imp().size.set_subtitle(&size.unwrap());
            }
            ContentDetails::Video(data) => panic!(),
        }
    }
}

impl Default for Details {
    fn default() -> Self {
        Self::new()
    }
}
