// library_view/library_list_model/imp.rs
//
// Copyright (c) 2024 Max Rodriguez
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Data model implementation of the LibraryListModel class.

use adw::gtk;
use adw::{glib, gio};
use adw::prelude::*;
use adw::subclass::prelude::*;
use libadwaita as adw;

/// Wraps GtkDirectoryList with a GObject that implements
/// GListModel, GtkSelectionModel, and GtkSectionModel.
#[derive(Debug)]
pub struct LibraryListModel(pub(super) gtk::DirectoryList);

impl Default for LibraryListModel {
    fn default() -> Self {
        Self(gtk::DirectoryList::new(None, None::<&gio::File>))
    }
}

#[glib::object_subclass]
impl ObjectSubclass for LibraryListModel {
    const NAME: &'static str = "LibraryListModel";
    type Type = super::LibraryListModel;
    type ParentType = glib::Object;
    type Interfaces = (gio::ListModel,);

    fn class_init(_: &mut Self::Class) {}

    fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
}

impl ObjectImpl for LibraryListModel {}

/// Basically just redirect all GListModel interface calls
/// to our underlying GtkDirectoryList object.
impl ListModelImpl for LibraryListModel {
    fn item(&self, position: u32) -> Option<glib::Object> {
        self.0.item(position)
    }

    fn item_type(&self) -> glib::Type {
        self.0.item_type()
    }

    fn n_items(&self) -> u32 {
        self.0.n_items()
    }
}

