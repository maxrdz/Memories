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

use crate::application::AlbumsApplication;
use crate::globals::{GRID_DESKTOP_ZOOM_LEVELS, GRID_MOBILE_ZOOM_LEVELS};
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

glib::wrapper! {
    pub struct AlbumsApplicationWindow(ObjectSubclass<imp::AlbumsApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

#[gtk::template_callbacks]
impl AlbumsApplicationWindow {
    pub fn new(application: &AlbumsApplication) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn app(&self) -> Option<AlbumsApplication> {
        self.application().and_downcast()
    }

    fn setup_gactions(&self) {
        let settings_action = gio::ActionEntry::builder("settings")
            .activate(move |win: &Self, _, _| {
                win.imp().master_stack.set_visible_child_name("preferences");
            })
            .build();
        let grid_zoom_in_action = gio::ActionEntry::builder("grid-zoom-in")
            .activate(move |win: &Self, _, _| {
                win.gallery_grid_zoom(true);
            })
            .build();
        let grid_zoom_out_action = gio::ActionEntry::builder("grid-zoom-out")
            .activate(move |win: &Self, _, _| {
                win.gallery_grid_zoom(false);
            })
            .build();
        self.add_action_entries([settings_action, grid_zoom_in_action, grid_zoom_out_action]);
    }

    fn gallery_grid_zoom(&self, zoom_in: bool) {
        let library_view_imp = self.imp().library_view.imp();
        let current_columns: u32 = library_view_imp.photo_grid_view.max_columns();
        let mut current_zoom_level: usize = 0;

        let zoom_levels: &'static [(u32, i32)] = self.get_zoom_levels();
        for (i, set) in zoom_levels.iter().enumerate() {
            if set.0 == current_columns {
                current_zoom_level = i;
            }
        }
        if zoom_in {
            if current_zoom_level == zoom_levels.len() - 1 {
                return;
            }
            self.set_grid_zoom_level(current_zoom_level + 1);
        } else {
            if current_zoom_level == 0 {
                return;
            }
            self.set_grid_zoom_level(current_zoom_level - 1);
        }
    }

    /// Returns the zoom levels array for the appropriate window size.
    fn get_zoom_levels(&self) -> &'static [(u32, i32)] {
        if self.imp().library_view.grid_desktop_zoom() {
            GRID_DESKTOP_ZOOM_LEVELS
        } else {
            GRID_MOBILE_ZOOM_LEVELS
        }
    }

    /// Sets the grid view columns and list item widget height requests
    /// using the given zoom level index, and updates the grid control
    /// popover menu if the user has reached the min/max zoom setting.
    fn set_grid_zoom_level(&self, zoom_level: usize) {
        let zoom_levels: &'static [(u32, i32)] = self.get_zoom_levels();
        let new_zoom_level: (u32, i32) = zoom_levels[zoom_level];
        let library_view_imp = self.imp().library_view.imp();

        self.imp().library_view.set_grid_widget_height(new_zoom_level.1);
        library_view_imp.photo_grid_view.set_min_columns(new_zoom_level.0);
        library_view_imp.photo_grid_view.set_max_columns(new_zoom_level.0);

        if zoom_level == 0 {
            // Reached minimum zoom level
            library_view_imp
                .photo_grid_controls
                .set_menu_model(Some(&library_view_imp.grid_controls_menu_min_zoom.clone()));
        } else if zoom_level == zoom_levels.len() - 1 {
            // Reached maximum zoom level
            library_view_imp
                .photo_grid_controls
                .set_menu_model(Some(&library_view_imp.grid_controls_menu_max_zoom.clone()));
        } else {
            library_view_imp
                .photo_grid_controls
                .set_menu_model(Some(&library_view_imp.grid_controls_menu.clone()));
        }
    }

    #[template_callback]
    fn master_stack_child_visible(&self) {
        if let Some(child_name) = self.imp().master_stack.visible_child_name() {
            if child_name == self.imp().library_page.name().unwrap() {
                // If the photo grid has no model, load the photo library now.
                if self.imp().library_view.imp().photo_grid_view.model().is_none() {
                    self.imp().library_view.load_library();
                }
            }
        }
    }
}
