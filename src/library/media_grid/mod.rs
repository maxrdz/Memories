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
pub mod media_cell;

use crate::globals::{GRID_DESKTOP_ZOOM_LEVELS, GRID_MOBILE_ZOOM_LEVELS};
use crate::window::AlbumsApplicationWindow;
use adw::glib;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use libadwaita as adw;

glib::wrapper! {
    pub struct AlbumsMediaGridView(ObjectSubclass<imp::AlbumsMediaGridView>)
        @extends gtk::Widget, adw::Bin, adw::BreakpointBin;
}

#[gtk::template_callbacks]
impl AlbumsMediaGridView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn window(&self) -> AlbumsApplicationWindow {
        self.root()
            .expect("Must be in a GtkApplicationWindow.")
            .downcast()
            .expect("Failed to downcast to AlbumsApplicationWindow.")
    }

    fn gallery_grid_zoom(&self, zoom_in: bool) {
        let current_columns: u32 = self.imp().photo_grid_view.max_columns();
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
        if self.grid_desktop_zoom() {
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

        self.set_grid_widget_height(new_zoom_level.1);

        self.imp().photo_grid_view.set_min_columns(new_zoom_level.0);
        self.imp().photo_grid_view.set_max_columns(new_zoom_level.0);

        if zoom_level == 0 {
            // Reached minimum zoom level
            self.imp().zoom_in.set_sensitive(true);
            self.imp().zoom_out.set_sensitive(false);
        } else if zoom_level == zoom_levels.len() - 1 {
            // Reached maximum zoom level
            self.imp().zoom_in.set_sensitive(false);
            self.imp().zoom_out.set_sensitive(true);
        } else {
            self.imp().zoom_in.set_sensitive(true);
            self.imp().zoom_out.set_sensitive(true);
        }
    }

    #[template_callback]
    fn zoom_in_callback(&self, _: &gtk::Button) {
        self.gallery_grid_zoom(true);
    }

    #[template_callback]
    fn zoom_out_callback(&self, _: &gtk::Button) {
        self.gallery_grid_zoom(false);
    }
}

impl Default for AlbumsMediaGridView {
    fn default() -> Self {
        Self::new()
    }
}
