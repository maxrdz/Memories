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

use crate::library::media_grid::MrsMediaGridView;
use crate::library::viewer::MrsViewer;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct MrsMediaCell(ObjectSubclass<imp::MrsMediaCell>)
        @extends gtk::Widget, adw::Bin;
}

impl MrsMediaCell {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn setup_cell(&self, media_grid: &MrsMediaGridView, list_item: &gtk::ListItem) {
        list_item.set_property("child", self);
        self.imp()
            .aspect_frame
            .set_height_request(media_grid.grid_widget_height());

        // Bind the `GtkAspectFrame`s height-request to the `grid-widget-height`
        // property of the `MrsMediaGridView` object.
        media_grid
            .bind_property(
                "grid-widget-height",
                &self.imp().aspect_frame.clone(),
                "height-request",
            )
            .sync_create()
            .build();

        // Once the image file has been set, we know it has been loaded, so
        // we can hide the content (placeholder icon) immediately, then reveal
        // the actual image content with a proper delay + transition type.
        let handler_id: glib::SignalHandlerId =
            self.imp()
                .image
                .connect_file_notify(clone!(@weak self as s => move |_: &gtk::Image| {
                    s.imp().revealer.set_reveal_child(false);
                    s.imp().revealer.set_transition_duration(1000); // milliseconds
                    s.imp().revealer.set_transition_type(gtk::RevealerTransitionType::Crossfade);
                    s.imp().revealer.set_reveal_child(true);
                }));

        self.imp()
            .img_file_notify
            .borrow()
            .set(handler_id)
            .expect("Media cell's `img_file_notify` already initialized!");

        let click_gesture: gtk::GestureClick = gtk::GestureClick::default();

        self.imp().revealer.add_controller(click_gesture.clone());

        click_gesture.connect_pressed(clone!(@weak media_grid, @weak list_item => move |_, _, _, _| {
                if list_item.is_selected() {
                    let current_nav_page: adw::NavigationPage = media_grid.window()
                        .imp()
                        .window_navigation
                        .visible_page()
                        .unwrap();

                    // Do not proceed to push a new nav page if one is already open.
                    if current_nav_page.tag().unwrap() != "window" {
                        return;
                    }
                    let grid_cell_data: MrsMediaCell = list_item.child().and_downcast().unwrap();

                    let model_item: gio::FileInfo = list_item.item().and_downcast().unwrap();
                    let file_obj: glib::Object = model_item.attribute_object("standard::file").unwrap();
                    let file: gio::File = file_obj.downcast().unwrap();

                    let nav_view = media_grid.window().imp().window_navigation.clone();

                    let viewer_content: MrsViewer = MrsViewer::default();
                    viewer_content.set_content_type(grid_cell_data.imp().viewer_content_type.get().unwrap());
                    viewer_content.set_content_file(&file);

                    viewer_content.imp()
                        .details_widget
                        .update_details(&grid_cell_data);

                    let nav_page: adw::NavigationPage = viewer_content.wrap_in_navigation_page();
                    nav_page.set_title(&file.basename().unwrap().to_string_lossy());

                    nav_view.push(&nav_page);
                }
            }
        ));
    }
}

impl Default for MrsMediaCell {
    fn default() -> Self {
        Self::new()
    }
}
