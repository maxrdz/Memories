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

use adw::gtk;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

mod imp {
    use super::*;
    use crate::globals::DEFAULT_LIBRARY_DIR;

    #[derive(Debug)]
    pub struct LibraryListModel {
        pub library_directory: gtk::DirectoryList,
    }

    impl Default for LibraryListModel {
        fn default() -> Self {
            Self {
                library_directory: gtk::DirectoryList::new(
                    None,
                    Some(&gio::File::for_path(DEFAULT_LIBRARY_DIR)),
                ),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LibraryListModel {
        const NAME: &'static str = "LibraryListModel";
        type Type = super::LibraryListModel;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for LibraryListModel {}
}

glib::wrapper! {
    pub struct LibraryListModel(ObjectSubclass<imp::LibraryListModel>)
        @implements gtk::SelectionModel, gio::ListModel;
}

impl LibraryListModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
