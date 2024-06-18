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

use gtk::glib;

mod imp {
    use adw::glib;
    use adw::subclass::prelude::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/album-viewer.ui")]
    pub struct MemoriesAlbumViewer {
        #[template_child]
        header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        more_button: TemplateChild<gtk::MenuButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoriesAlbumViewer {
        const NAME: &'static str = "MemoriesAlbumViewer";
        type Type = super::MemoriesAlbumViewer;
        type ParentType = adw::BreakpointBin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MemoriesAlbumViewer {}
    impl WidgetImpl for MemoriesAlbumViewer {}
    impl BinImpl for MemoriesAlbumViewer {}
    impl BreakpointBinImpl for MemoriesAlbumViewer {}
}

glib::wrapper! {
    pub struct MemoriesAlbumViewer(ObjectSubclass<imp::MemoriesAlbumViewer>)
        @extends gtk::Widget, adw::Bin;
}

impl MemoriesAlbumViewer {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for MemoriesAlbumViewer {
    fn default() -> Self {
        Self::new()
    }
}
