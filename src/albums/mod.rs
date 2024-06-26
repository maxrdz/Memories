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

mod viewer;

use gtk::glib;

mod imp {
    use adw::glib;
    use adw::subclass::prelude::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/albums.ui")]
    pub struct MemoriesAlbumsView {
        #[template_child]
        albums_grid_view: TemplateChild<gtk::GridView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoriesAlbumsView {
        const NAME: &'static str = "MemoriesAlbumsView";
        type Type = super::MemoriesAlbumsView;
        type ParentType = adw::BreakpointBin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MemoriesAlbumsView {}
    impl WidgetImpl for MemoriesAlbumsView {}
    impl BinImpl for MemoriesAlbumsView {}
    impl BreakpointBinImpl for MemoriesAlbumsView {}
}

glib::wrapper! {
    pub struct MemoriesAlbumsView(ObjectSubclass<imp::MemoriesAlbumsView>)
        @extends gtk::Widget, adw::Bin;
}

impl MemoriesAlbumsView {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for MemoriesAlbumsView {
    fn default() -> Self {
        Self::new()
    }
}
