// library_view.rs
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

use crate::globals::DEFAULT_LIBRARY_DIR;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Gallery/ui/library-view.ui")]
    pub struct LibraryView {
        #[template_child]
        pub photo_grid_view: TemplateChild<gtk::GridView>,
    }

    unsafe impl IsSubclassable<LibraryView> for adw::Bin {}

    #[glib::object_subclass]
    impl ObjectSubclass for LibraryView {
        const NAME: &'static str = "LibraryView";
        type Type = super::LibraryView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LibraryView {}
    impl WidgetImpl for LibraryView {}
    impl WindowImpl for LibraryView {}
    impl ApplicationWindowImpl for LibraryView {}
    impl AdwApplicationWindowImpl for LibraryView {}
}

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
        use crate::library_list_model::LibraryListModel;
        self.imp()
            .photo_grid_view
            .set_model(Some(&LibraryListModel::new()));
    }
}
