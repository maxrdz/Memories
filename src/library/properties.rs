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

use crate::library::media_cell::MemoriesMediaCell;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::g_warning;
use gtk::{gio, glib};
use std::ffi::OsStr;
use std::ops::Deref;
use std::path::PathBuf;

mod imp {
    use adw::subclass::prelude::*;
    use gtk::glib;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/media-properties.ui")]
    pub struct MemoriesProperties {
        #[template_child]
        pub(super) folder: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) format: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) dimensions: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) size: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) created: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) modified: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) timestamp: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) aperture: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) exposure: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) iso: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) focal_length: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(super) make_model: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoriesProperties {
        const NAME: &'static str = "MemoriesProperties";
        type Type = super::MemoriesProperties;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MemoriesProperties {}
    impl WidgetImpl for MemoriesProperties {}
    impl BinImpl for MemoriesProperties {}
}

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
    pub struct MemoriesProperties(ObjectSubclass<imp::MemoriesProperties>)
        @extends gtk::Widget, adw::Bin;
}

impl MemoriesProperties {
    pub fn new() -> Self {
        glib::Object::new()
    }

    /// Updates the preference rows in the details widget with
    /// the content metadata from the `MemoriesMediaCell` object passed.
    pub fn update_details(&self, cell_data: &MemoriesMediaCell) {
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

impl Default for MemoriesProperties {
    fn default() -> Self {
        Self::new()
    }
}
