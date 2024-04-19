// library_view/library_list_model/mod.rs
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

mod imp;

use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib_macros::clone;
use gtk::{gio, glib};
use libadwaita as adw;

glib::wrapper! {
    pub struct LibraryListModel(ObjectSubclass<imp::LibraryListModel>)
        @implements gio::ListModel;
}

impl LibraryListModel {
    pub fn new() -> Self {
        glib::Object::new()
    }

    /// Bridge LibraryListModel interface to underlying GtkDirectoryList.
    pub fn is_loading(&self) -> bool {
        self.imp().0.is_loading()
    }

    /// Bridge LibraryListModel interface to underlying GtkDirectoryList.
    pub fn connect_loading_notify<F>(&self, callback: F) -> glib::signal::SignalHandlerId
    where
        F: Fn(&gtk::DirectoryList) + 'static,
    {
        self.imp().0.connect_loading_notify(callback)
    }

    /// Bridge LibraryListModel interface to underlying GtkDirectoryList.
    pub fn connect_error_notify<F>(&self, callback: F) -> glib::signal::SignalHandlerId
    where
        F: Fn(&gtk::DirectoryList) + 'static,
    {
        self.imp().0.connect_error_notify(callback)
    }

    /// Bridge LibraryListModel interface to underlying GtkDirectoryList.
    pub fn set_file(&self, file: Option<&impl glib::prelude::IsA<gio::File>>) {
        // This is a good point to connect the `items_changed` signal from
        // GtkDirectoryList with our signal. Without this, the `GtkGridView`
        // will never know when to tell the factory to make list item widgets.
        self.imp()
            .0
            .connect_items_changed(clone!(@weak self as s => move |
            _: &gtk::DirectoryList, a: u32, b: u32, c: u32| {
                s.items_changed(a, b, c);
            }));
        self.imp().0.set_file(file)
    }
}

impl Default for LibraryListModel {
    fn default() -> Self {
        Self::new()
    }
}
