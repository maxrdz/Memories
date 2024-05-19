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

use crate::library::details::ContentDetails;
use crate::library::viewer::ViewerContentType;
use crate::utils::MetadataInfo;
use adw::glib;
use adw::subclass::prelude::*;
use libadwaita as adw;
use std::cell::{Cell, OnceCell, RefCell};

/// `AdwBin` subclass to store arbitrary data for grid cells
/// of the library photo grid view. Stores signal
/// handler IDs, glib async join handles, metadata, etc.
#[derive(Default)]
pub struct AlbumsItemData {
    pub img_file_notify: RefCell<OnceCell<glib::SignalHandlerId>>,
    pub tx_join_handle: Cell<Option<glib::JoinHandle<()>>>,
    pub rx_join_handle: Cell<Option<glib::JoinHandle<()>>>,
    pub file_info: OnceCell<gio::FileInfo>,
    pub file_metadata: OnceCell<MetadataInfo>,
    pub viewer_content_type: OnceCell<ViewerContentType>,
    pub content_details: RefCell<ContentDetails>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsItemData {
    const NAME: &'static str = "AlbumsItemData";
    type ParentType = adw::Bin;
    type Type = super::AlbumsItemData;
}

impl ObjectImpl for AlbumsItemData {}
impl WidgetImpl for AlbumsItemData {}
impl BinImpl for AlbumsItemData {}
