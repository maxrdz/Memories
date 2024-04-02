// master_window.rs
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

use crate::library_view::LibraryView;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Gallery/ui/master-window.ui")]
    pub struct MasterWindow {
        #[template_child]
        pub master_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub library_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub album_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub search_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub options_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub library_view: TemplateChild<LibraryView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MasterWindow {
        const NAME: &'static str = "MasterWindow";
        type Type = super::MasterWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MasterWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let _obj = self.obj();
        }
    }
    impl WidgetImpl for MasterWindow {}
    impl WindowImpl for MasterWindow {}
    impl ApplicationWindowImpl for MasterWindow {}
    impl AdwApplicationWindowImpl for MasterWindow {}
}

glib::wrapper! {
    pub struct MasterWindow(ObjectSubclass<imp::MasterWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl MasterWindow {
    pub fn new<P: IsA<adw::gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    #[template_callback]
    fn master_stack_child_visible(&self) {
        let class_imp: &imp::MasterWindow = self.imp();

        if let Some(child_name) = class_imp.master_stack.visible_child_name() {
            if child_name == "library" {
                // if the photo grid has no model, it has not been loaded before
                if let None = class_imp.library_view.imp().photo_grid_view.model() {
                    class_imp.library_view.load_library();
                }
            }
        }
    }
}
