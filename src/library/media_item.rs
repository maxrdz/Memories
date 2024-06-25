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

use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};
    use std::cell::{Cell, OnceCell};

    #[derive(glib::Properties, Default, Debug)]
    #[properties(wrapper_type = super::MemoriesMediaItem)]
    pub struct MemoriesMediaItem {
        #[property(get, set)]
        basename: OnceCell<glib::GString>,
        #[property(get, set)]
        timestamp: OnceCell<glib::DateTime>,
        #[property(get, set)]
        favorite: Cell<bool>,
        #[property(get, set)]
        hidden: Cell<bool>,
        #[property(get, set)]
        file: OnceCell<gio::File>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoriesMediaItem {
        const NAME: &'static str = "MemoriesMediaItem";
        type Type = super::MemoriesMediaItem;
    }

    #[glib::derived_properties]
    impl ObjectImpl for MemoriesMediaItem {}
}

glib::wrapper! {
    pub struct MemoriesMediaItem(ObjectSubclass<imp::MemoriesMediaItem>);
}

impl MemoriesMediaItem {
    pub fn new(file_info_obj: &glib::Object) -> Self {
        let file_info: gio::FileInfo = file_info_obj.clone().downcast().unwrap();
        let file_obj: glib::Object = file_info.attribute_object("standard::file").unwrap();
        let gfile: gio::File = file_obj.downcast().unwrap();

        let obj: Self = glib::Object::new();

        obj.set_file(gfile.clone());
        obj.set_basename(gfile.basename().unwrap().to_string_lossy());
        obj
    }

    pub fn new_and_upcast(file_info_obj: &glib::Object) -> glib::Object {
        MemoriesMediaItem::new(file_info_obj).upcast()
    }
}
