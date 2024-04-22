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
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};
use glib_macros::clone;
use libadwaita as adw;
use std::cell::{RefCell, RefMut};

/// Custom implementation of GListModel that uses GTK's
/// `GtkDirectoryList` models under the hood to recursively
/// enumerate files under a certain directory path.
#[derive(Debug)]
pub struct LibraryListModel {
    items_changed_signal_id: RefCell<Option<glib::SignalHandlerId>>,
    pub(super) root_model: gtk::DirectoryList,
    subdir_models: RefCell<Vec<(gtk::DirectoryList, glib::SignalHandlerId)>>,
}

impl Default for LibraryListModel {
    fn default() -> Self {
        Self {
            items_changed_signal_id: RefCell::new(None),
            root_model: gtk::DirectoryList::new(None, None::<&gio::File>),
            subdir_models: RefCell::new(vec![]),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for LibraryListModel {
    const NAME: &'static str = "AlbumLibraryListModel";
    type Type = super::LibraryListModel;
    type Interfaces = (gio::ListModel,);
}

impl ObjectImpl for LibraryListModel {
    fn constructed(&self) {
        let obj = self.obj();
        // This is a good point to connect the `items_changed` signal from
        // GtkDirectoryList with our signal. Without this, the `GtkGridView`
        // will never know when to tell the factory to make list item widgets.
        let signal_handler_id: glib::SignalHandlerId =
            self.root_model
                .connect_items_changed(clone!(@weak obj as o => move |
                _: &gtk::DirectoryList, pos: u32, removed: u32, added: u32| {
                    o.items_changed(pos, removed, added);
                }));
        self.items_changed_signal_id.replace(Some(signal_handler_id));
    }
}

/// Basically just redirect all GListModel interface calls
/// to our underlying GtkDirectoryList objects. Specifically,
/// if the root directory list model is empty, reroute data
/// from our subdirectory models to make this object look
/// like it has a continuous list of items from all subdirs.
impl ListModelImpl for LibraryListModel {
    fn item(&self, position: u32) -> Option<glib::Object> {
        if let Some(res) = self.root_model.item(position) {
            Some(res)
        } else {
            let sdm_mut: RefMut<'_, Vec<(gtk::DirectoryList, glib::SignalHandlerId)>> =
                self.subdir_models.borrow_mut();
            if !sdm_mut.is_empty() {
                assert!(
                    position >= self.root_model.n_items(),
                    "Given position u32 value is less than the size of the root GtkDirectoryList!",
                );
                let mut adjusted_position: u32 = position - self.root_model.n_items();
                for subdir_model in sdm_mut.iter() {
                    if let Some(res) = subdir_model.0.item(adjusted_position) {
                        return Some(res);
                    }
                    if adjusted_position >= subdir_model.0.n_items() {
                        adjusted_position -= subdir_model.0.n_items();
                        continue;
                    }
                    return None;
                }
                return None;
            }
            None
        }
    }

    fn item_type(&self) -> glib::Type {
        self.root_model.item_type()
    }

    fn n_items(&self) -> u32 {
        let mut subdir_item_count: u32 = 0;
        for subdir_model in self.subdir_models.borrow_mut().iter() {
            subdir_item_count += subdir_model.0.n_items();
        }
        self.root_model.n_items() + subdir_item_count
    }
}
