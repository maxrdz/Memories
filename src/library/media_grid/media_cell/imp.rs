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

use crate::library::details::ContentDetails;
use crate::library::viewer::ViewerContentType;
use crate::utils::MetadataInfo;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use std::cell::{Cell, OnceCell, RefCell};

/// `AdwBin` subclass to store arbitrary data for grid cells
/// of the library photo grid view. Stores signal
/// handler IDs, glib async join handles, metadata, etc.
#[derive(Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Memories/library/media_grid/media_cell/media-cell.ui")]
pub struct MrsMediaCell {
    #[template_child]
    pub(super) revealer: TemplateChild<gtk::Revealer>,
    #[template_child]
    pub(super) aspect_frame: TemplateChild<gtk::AspectFrame>,
    #[template_child]
    pub thumbnail_image: TemplateChild<gtk::Image>,
    #[template_child]
    favorited: TemplateChild<gtk::Image>,
    #[template_child]
    media_type_icon: TemplateChild<gtk::Image>,
    #[template_child]
    video_length: TemplateChild<gtk::Label>,

    pub img_file_notify: RefCell<OnceCell<glib::SignalHandlerId>>,
    pub tx_join_handle: Cell<Option<glib::JoinHandle<()>>>,
    pub rx_join_handle: Cell<Option<glib::JoinHandle<()>>>,
    pub file_info: OnceCell<gio::FileInfo>,
    pub file_metadata: OnceCell<MetadataInfo>,
    pub viewer_content_type: OnceCell<ViewerContentType>,
    pub content_details: RefCell<ContentDetails>,
}

#[glib::object_subclass]
impl ObjectSubclass for MrsMediaCell {
    const NAME: &'static str = "MrsMediaCell";
    type ParentType = adw::Bin;
    type Type = super::MrsMediaCell;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MrsMediaCell {}
impl WidgetImpl for MrsMediaCell {}
impl BinImpl for MrsMediaCell {}
