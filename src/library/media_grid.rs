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

use crate::globals::{GRID_DESKTOP_ZOOM_LEVELS, GRID_MOBILE_ZOOM_LEVELS};
use crate::window::MemoriesApplicationWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::{gio, glib};

pub mod imp {
    use crate::application::MemoriesApplication;
    use crate::globals::{DEFAULT_GRID_WIDGET_HEIGHT, FFMPEG_CONCURRENT_PROCESSES};
    use crate::library::media_cell::MemoriesMediaCell;
    use crate::library::media_viewer::ViewerContentType;
    use crate::window::MemoriesApplicationWindow;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use async_semaphore::Semaphore;
    use glib::{clone, g_warning};
    use gtk::{gio, glib};
    use std::cell::Cell;
    use std::sync::Arc;

    #[derive(Debug, glib::Properties, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/media-grid.ui")]
    #[properties(wrapper_type = super::MemoriesMediaGridView)]
    pub struct MemoriesMediaGridView {
        pub subprocess_semaphore: Arc<Semaphore>,
        pub list_item_factory: gtk::SignalListItemFactory,

        #[property(get, set)]
        hardware_accel: Cell<bool>,
        #[property(get, set)]
        grid_widget_height: Cell<i32>,
        #[property(get, set)]
        grid_desktop_zoom: Cell<bool>,

        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub overlay_header_buttons: TemplateChild<gtk::Box>,
        #[template_child]
        pub photo_grid_controls: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub photo_grid_view: TemplateChild<gtk::GridView>,
    }

    impl Default for MemoriesMediaGridView {
        fn default() -> Self {
            Self {
                subprocess_semaphore: Arc::new(Semaphore::new(FFMPEG_CONCURRENT_PROCESSES)),
                list_item_factory: gtk::SignalListItemFactory::default(),
                hardware_accel: Cell::new({
                    let gsettings: gio::Settings = MemoriesApplication::default().gsettings();
                    gsettings.boolean("ffmpeg-hardware-acceleration")
                }),
                grid_widget_height: Cell::new(DEFAULT_GRID_WIDGET_HEIGHT),
                grid_desktop_zoom: Cell::new(false),
                toast_overlay: TemplateChild::default(),
                overlay_header_buttons: TemplateChild::default(),
                photo_grid_controls: TemplateChild::default(),
                photo_grid_view: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoriesMediaGridView {
        const NAME: &'static str = "MemoriesMediaGridView";
        type ParentType = adw::BreakpointBin;
        type Type = super::MemoriesMediaGridView;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for MemoriesMediaGridView {
        fn constructed(&self) {
            let obj = self.obj();

            obj.connect_grid_desktop_zoom_notify(move |media_grid: &super::MemoriesMediaGridView| {
                let win: MemoriesApplicationWindow = media_grid.window();

                // `grid_desktop_zoom` is modified only when the `AdwBreakpoint` is triggered.
                // The default zoom settings for the grid view are always at the minimum zoom
                // by default in the UI files, so we reset the grid controls to min zoom below.
                win.action_set_enabled("mediagrid.zoom_in", true);
                win.action_set_enabled("mediagrid.zoom_out", false);
            });

            // Bind any application preferences to our application's GSettings.
            let gsettings: gio::Settings = MemoriesApplication::default().gsettings();

            gsettings
                .bind(
                    "ffmpeg-hardware-acceleration",
                    &self.obj().clone(),
                    "hardware-accel",
                )
                .build();

            self.list_item_factory.connect_setup(clone!(
                #[weak]
                obj,
                move |_: &gtk::SignalListItemFactory, widget: &glib::Object| {
                    let list_item_widget: gtk::ListItem = widget.clone().downcast().unwrap();

                    let cell: MemoriesMediaCell = MemoriesMediaCell::default();
                    cell.setup_cell(&obj, &list_item_widget);
                }
            ));

            self.list_item_factory.connect_bind(clone!(
                #[weak(rename_to = this)]
                self,
                move |_: &gtk::SignalListItemFactory, obj: &glib::Object| {
                    let list_item: gtk::ListItem = obj.clone().downcast().unwrap();
                    let cell: MemoriesMediaCell = list_item.child().and_downcast().unwrap();

                    let model_list_item: gio::FileInfo = list_item.item().and_downcast().unwrap();

                    let file_obj: glib::Object = model_list_item.attribute_object("standard::file").unwrap();
                    let file: gio::File = file_obj.downcast().unwrap();
                    let file_path_buf: std::path::PathBuf = file.path().unwrap();

                    // Convert file_path_buf to a String (not a string slice) since file_path_buf
                    // does not live long enough to be borrowed in the futures spawned below.
                    let absolute_path: String = file_path_buf.to_string_lossy().to_string();

                    if let Some(ext) = model_list_item.name().extension() {
                        cell.bind_cell(&this, ViewerContentType::from_ext(ext), &list_item);
                    } else {
                        g_warning!(
                            "MediaGridView",
                            "Found a file with no file extension, with file path '{}'.",
                            absolute_path
                        );
                    }
                }
            ));

            self.photo_grid_view.set_factory(Some(&self.list_item_factory));
        }
    }

    impl WidgetImpl for MemoriesMediaGridView {}
    impl BinImpl for MemoriesMediaGridView {}
    impl BreakpointBinImpl for MemoriesMediaGridView {}
}

glib::wrapper! {
    pub struct MemoriesMediaGridView(ObjectSubclass<imp::MemoriesMediaGridView>)
        @extends gtk::Widget, adw::Bin, adw::BreakpointBin;
}

impl MemoriesMediaGridView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn window(&self) -> MemoriesApplicationWindow {
        self.root()
            .expect("Must be in a GtkApplicationWindow.")
            .downcast()
            .expect("Failed to downcast to MemoriesApplicationWindow.")
    }

    /// This function is public so that it can be called once we
    /// are placed in the widget tree and can access the window.
    pub fn setup_gactions(&self) {
        let win: MemoriesApplicationWindow = self.window();
        let action_group = gio::SimpleActionGroup::new();

        let zoom_in_action = gio::ActionEntry::builder("zoom_in")
            .activate(clone!(
                #[weak(rename_to = this)]
                self,
                move |_: &gio::SimpleActionGroup, _, _| {
                    this.gallery_grid_zoom(true);
                }
            ))
            .build();

        let zoom_out_action = gio::ActionEntry::builder("zoom_out")
            .activate(clone!(
                #[weak(rename_to = this)]
                self,
                move |_: &gio::SimpleActionGroup, _, _| {
                    this.gallery_grid_zoom(false);
                }
            ))
            .build();

        action_group.add_action_entries([zoom_in_action, zoom_out_action]);
        win.insert_action_group("mediagrid", Some(&action_group));

        win.action_set_enabled("mediagrid.zoom_in", true);
        win.action_set_enabled("mediagrid.zoom_out", false);
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

        let win: MemoriesApplicationWindow = self.window();

        if zoom_level == 0 {
            // Reached minimum zoom level
            win.action_set_enabled("mediagrid.zoom_in", true);
            win.action_set_enabled("mediagrid.zoom_out", false);
        } else if zoom_level == zoom_levels.len() - 1 {
            // Reached maximum zoom level
            win.action_set_enabled("mediagrid.zoom_in", false);
            win.action_set_enabled("mediagrid.zoom_out", true);
        } else {
            win.action_set_enabled("mediagrid.zoom_in", true);
            win.action_set_enabled("mediagrid.zoom_out", true);
        }
    }
}

impl Default for MemoriesMediaGridView {
    fn default() -> Self {
        Self::new()
    }
}
