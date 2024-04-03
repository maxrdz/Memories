// library_list_model.rs
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

//! Defines the LibraryListModel class, which inherits from GObject
//! and implements the GListModel and GtkSelectionModel interfaces.
//! Used by the GtkGridView in the Library view as its list model.

use adw::gtk;
use gio::glib::subclass::types::ObjectSubclassIsExt;
use gtk::{gio, glib};
use libadwaita as adw;

mod imp {
    use super::*;
    use gio::prelude::*;
    use gtk::subclass::prelude::*;

    /// Wraps GtkDirectoryList with a GObject that implements
    /// GListModel, GtkSelectionModel, and GtkSectionModel.
    #[derive(Debug)]
    pub struct LibraryListModel {
        pub(super) library_directory: gtk::DirectoryList,
    }

    impl Default for LibraryListModel {
        fn default() -> Self {
            Self {
                library_directory: gtk::DirectoryList::new(
                    None,
                    None::<&gio::File>,
                ),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LibraryListModel {
        const NAME: &'static str = "LibraryListModel";
        type Type = super::LibraryListModel;
        type ParentType = glib::Object;
        type Interfaces = (gio::ListModel, gtk::SelectionModel);
    }

    impl ObjectImpl for LibraryListModel {}

    /// Basically just redirect all GListModel interface calls
    /// to our underlying GtkDirectoryList object.
    impl ListModelImpl for LibraryListModel {
        fn item(&self, position: u32) -> Option<glib::Object> {
            self.library_directory.item(position)
        }

        fn item_type(&self) -> glib::Type {
            glib::Object::static_type()
        }

        fn n_items(&self) -> u32 {
            self.library_directory.n_items()
        }
    }

    impl SectionModelImpl for LibraryListModel {}
    impl SelectionModelImpl for LibraryListModel {}
}

glib::wrapper! {
    pub struct LibraryListModel(ObjectSubclass<imp::LibraryListModel>)
        @implements gtk::SelectionModel, gtk::SectionModel, gio::ListModel;
}

impl LibraryListModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Bridge LibraryListModel interface to underlying GtkDirectoryList.
    pub fn is_loading(&self) -> bool {
        self.imp().library_directory.is_loading()
    }

    /// Bridge LibraryListModel interface to underlying GtkDirectoryList.
    pub fn connect_loading_notify<F>(&self, callback: F) -> glib::signal::SignalHandlerId
    where
        F: Fn(&gtk::DirectoryList) + 'static,
    {
        self.imp().library_directory.connect_loading_notify(callback)
    }

    /// Bridge LibraryListModel interface to underlying GtkDirectoryList.
    pub fn set_file(&self, file: Option<&impl glib::prelude::IsA<gio::File>>) {
        self.imp().library_directory.set_file(file)
    }
}
