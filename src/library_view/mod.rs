// library_view/mod.rs
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

use crate::globals::DEFAULT_LIBRARY_ABS_DIR;
use crate::library_list_model::LibraryListModel;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib_macros::clone;
use gtk::{gio, glib};
use libadwaita as adw;

glib::wrapper! {
    pub struct LibraryView(ObjectSubclass<imp::LibraryView>)
        @extends gtk::Widget, adw::Bin,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl LibraryView {
    pub fn new<P: IsA<adw::gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    /// Called by MasterWindow once the Library view stack page is visible on screen.
    pub fn load_library(&self) {
        let llm: LibraryListModel = LibraryListModel::new();

        llm.connect_loading_notify(clone!(@weak self as s => move |dl: &gtk::DirectoryList| {
            if dl.is_loading() == false {
                s.imp().library_view_stack.set_visible_child_name("gallery_page");
                println!("{}", dl.n_items());
            }
        }));
        llm.connect_error_notify(move |_| {
            panic!("GtkDirectoryList returned an error!\n");
        });

        let slif: gtk::SignalListItemFactory = gtk::SignalListItemFactory::new();
        slif.connect_setup(move |factory: &gtk::SignalListItemFactory, new_gobject: &glib::Object| {
            ()
        });

        self.imp().photo_grid_view.set_model(Some(&llm));
        self.imp().photo_grid_view.set_factory(Some(&slif));
        llm.set_file(Some(&gio::File::for_path(DEFAULT_LIBRARY_ABS_DIR)));
    }
}
