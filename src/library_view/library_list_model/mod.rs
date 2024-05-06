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

use adw::gtk;
use adw::subclass::prelude::*;
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
    pub fn set_file(&self, file: Option<&impl glib::prelude::IsA<gio::File>>) {
        self.imp().root_model.set_file(file)
    }

    /// Bridge LibraryListModel interface to underlying GtkDirectoryList.
    /// TODO: Don't connect to only root model, but all models, similar to loading.
    pub fn connect_error_notify<F>(&self, callback: F) -> glib::signal::SignalHandlerId
    where
        F: Fn(&gtk::DirectoryList) + 'static,
    {
        self.imp().root_model.connect_error_notify(callback)
    }
}

impl Default for LibraryListModel {
    fn default() -> Self {
        Self::new()
    }
}
