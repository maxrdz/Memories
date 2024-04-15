// master_window/mod.rs
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
use gtk::{gio, glib};
use libadwaita as adw;

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

    fn setup_gactions(&self) {
        let settings_action = gio::ActionEntry::builder("settings")
            .activate(move |win: &Self, _, _| {
                win.imp().master_stack.set_visible_child_name("preferences");
            })
            .build();
        self.add_action_entries([settings_action]);
    }

    #[template_callback]
    fn master_stack_child_visible(&self) {
        let class_imp: &imp::MasterWindow = self.imp();

        if let Some(child_name) = class_imp.master_stack.visible_child_name() {
            if child_name == class_imp.library_page.name().unwrap() {
                // if the photo grid has no model, it has not been loaded before
                if class_imp.library_view.imp().photo_grid_view.model().is_none() {
                    class_imp.library_view.load_library();
                }
            }
        }
    }
}
