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

mod imp;

use crate::application::MemoriesApplication;
use crate::preferences::MemoriesPreferencesDialog;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::g_error;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct MemoriesApplicationWindow(ObjectSubclass<imp::MemoriesApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

#[gtk::template_callbacks]
impl MemoriesApplicationWindow {
    pub fn new(application: &MemoriesApplication) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn app(&self) -> Option<MemoriesApplication> {
        self.application().and_downcast()
    }

    fn setup_gactions(&self) {
        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(move |win: &Self, _, _| {
                MemoriesPreferencesDialog::default().present(win);
            })
            .build();

        let shortcuts_window_action = gio::ActionEntry::builder("show-help-overlay")
            .activate(move |win: &Self, _, _| {
                // GActions are setup after constructor, which guarantees that
                // the help overlay is setup for the window, so we can unwrap().
                win.help_overlay().unwrap().present();
            })
            .build();

        self.add_action_entries([preferences_action, shortcuts_window_action]);
    }

    #[template_callback]
    fn master_stack_child_visible(&self) {
        let media_grid_imp = self.imp().library_view.imp().media_grid.imp();

        if let Some(child_name) = self.imp().master_stack.visible_child_name() {
            match child_name.as_str() {
                "library" => {
                    self.imp()
                        .search_entry
                        .set_placeholder_text(Some(&gettext("Search Photos")));

                    // If the photo grid has no model, load the photo library now.
                    if media_grid_imp.photo_grid_view.model().is_none() {
                        self.imp().library_view.load_library();
                    }
                }
                "albums" => self
                    .imp()
                    .search_entry
                    .set_placeholder_text(Some(&gettext("Search Albums"))),
                "favorites" => self
                    .imp()
                    .search_entry
                    .set_placeholder_text(Some(&gettext("Search Favorites"))),
                _ => g_error!("ApplicationWindow", "Unexpected master stack child found."),
            }
        }
    }

    #[template_callback]
    fn toggle_search_bar(&self, toggle_button: &gtk::ToggleButton) {
        self.imp().search_bar.set_search_mode(toggle_button.is_active());
    }
}
