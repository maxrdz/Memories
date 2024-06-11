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

use super::media_cell::MrsMediaCell;
use crate::application::MrsApplication;
use crate::globals::{DEFAULT_GRID_WIDGET_HEIGHT, FFMPEG_CONCURRENT_PROCESSES};
use crate::library::details::{ContentDetails, PictureDetails};
use crate::thumbnails::generate_thumbnail_image;
use crate::utils::{get_content_type_from_ext, get_metadata_with_hash};
use crate::window::theme_selector::MrsThemeSelector;
use adw::prelude::*;
use adw::subclass::prelude::*;
use async_fs::File;
use async_semaphore::{Semaphore, SemaphoreGuard};
use glib::{clone, g_critical, g_warning};
use gtk::{gio, glib};
use std::cell::{Cell, RefCell};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, glib::Properties, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Memories/library/media_grid/media-grid.ui")]
#[properties(wrapper_type = super::MrsMediaGridView)]
pub struct MrsMediaGridView {
    pub(super) subprocess_semaphore: Arc<Semaphore>,
    pub list_item_factory: gtk::SignalListItemFactory,

    /// Disabled by default. Overlay title displays timestamp ranges of
    /// visible items in the media grid view. (currently not implemented)
    /// When enabled, a custom title has been set to the media grid view.
    /// For example, when viewing albums, the custom title is set to
    /// the name of the album being viewed.
    pub(super) custom_title: Cell<bool>,

    #[property(get, set)]
    hardware_accel: Cell<bool>,
    #[property(get, set)]
    grid_widget_height: Cell<i32>,
    #[property(get, set)]
    grid_desktop_zoom: Cell<bool>,

    #[template_child]
    pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    #[template_child]
    pub overlay_labels_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub time_period_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub total_items_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub overlay_header_buttons: TemplateChild<gtk::Box>,
    #[template_child]
    pub photo_grid_controls: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub main_menu: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub primary_menu: TemplateChild<gtk::PopoverMenu>,
    #[template_child]
    pub photo_grid_view: TemplateChild<gtk::GridView>,
    #[template_child]
    pub zoom_in: TemplateChild<gtk::Button>,
    #[template_child]
    pub zoom_out: TemplateChild<gtk::Button>,
}

impl Default for MrsMediaGridView {
    fn default() -> Self {
        Self {
            subprocess_semaphore: Arc::new(Semaphore::new(FFMPEG_CONCURRENT_PROCESSES)),
            list_item_factory: gtk::SignalListItemFactory::default(),
            custom_title: Cell::new(false),
            hardware_accel: Cell::new({
                let gsettings: gio::Settings = MrsApplication::default().gsettings();
                gsettings.boolean("hardware-acceleration")
            }),
            grid_widget_height: Cell::new(DEFAULT_GRID_WIDGET_HEIGHT),
            grid_desktop_zoom: Cell::new(false),
            toast_overlay: TemplateChild::default(),
            overlay_labels_box: TemplateChild::default(),
            time_period_label: TemplateChild::default(),
            total_items_label: TemplateChild::default(),
            overlay_header_buttons: TemplateChild::default(),
            photo_grid_controls: TemplateChild::default(),
            main_menu: TemplateChild::default(),
            primary_menu: TemplateChild::default(),
            photo_grid_view: TemplateChild::default(),
            zoom_in: TemplateChild::default(),
            zoom_out: TemplateChild::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MrsMediaGridView {
    const NAME: &'static str = "MrsMediaGridView";
    type ParentType = adw::BreakpointBin;
    type Type = super::MrsMediaGridView;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for MrsMediaGridView {
    fn constructed(&self) {
        let obj = self.obj();

        obj.connect_grid_desktop_zoom_notify(move |media_grid: &super::MrsMediaGridView| {
            // `grid_desktop_zoom` is modified only when the `AdwBreakpoint` is triggered.
            // The default zoom settings for the grid view are always at the minimum zoom
            // by default in the UI files, so we reset the grid controls to min zoom below.
            media_grid.imp().zoom_in.set_sensitive(true);
            media_grid.imp().zoom_out.set_sensitive(false);
        });

        // Bind any application preferences to our application's GSettings.
        let gsettings: gio::Settings = MrsApplication::default().gsettings();

        gsettings
            .bind("hardware-acceleration", &self.obj().clone(), "hardware-accel")
            .build();

        self.list_item_factory.connect_setup(
            clone!(@weak obj => move |_: &gtk::SignalListItemFactory, widget: &glib::Object| {
                    let list_item_widget: gtk::ListItem = widget.clone().downcast().unwrap();

                    let cell: MrsMediaCell = MrsMediaCell::default();
                    cell.setup_cell(&obj, &list_item_widget);
                }
            ),
        );

        self.list_item_factory.connect_bind(clone!(@weak self as s => move |_: &gtk::SignalListItemFactory, obj: &glib::Object| {
            let list_item: gtk::ListItem = obj.clone().downcast().unwrap();
            let cell: MrsMediaCell = list_item.child().and_downcast().unwrap();

            let model_list_item: gio::FileInfo = list_item.item().and_downcast().unwrap();

            // Store `GFileInfo` object reference in `MrsMediaCell` object.
            let _ = cell.imp().file_info.set(model_list_item.clone());

            let file_obj: glib::Object = model_list_item.attribute_object("standard::file").unwrap();
            let file: gio::File = file_obj.downcast().unwrap();
            let file_path_buf: std::path::PathBuf = file.path().unwrap();

            // Convert file_path_buf to a String (not a string slice) since file_path_buf
            // does not live long enough to be borrowed in the futures spawned below.
            let absolute_path: String = file_path_buf.to_string_lossy().to_string();

            if let Some(ext) = model_list_item.name().extension() {
                let ext_str: &str = &ext.to_str().unwrap().to_lowercase();

                match ext_str {
                    // SVGs are rendered by GNOME's librsvg, which is cheap and optimal
                    // and making a thumbnail for it would be more expensive than rendering it.
                    "svg" => cell.imp().image.set_file(Some(&absolute_path)),
                    _ => {
                        let (tx, rx) = async_channel::bounded(1);
                        let semaphore: Arc<Semaphore> = s.subprocess_semaphore.clone();

                        let tx_handle = glib::spawn_future_local(clone!(@weak cell => async move {
                            let semaphore_guard: SemaphoreGuard<'_> = semaphore.acquire().await;

                            // We need to get 3 things done in this closure:
                            // - file metadata
                            // - metadata md5 digest
                            // - thumbnail image
                            // So, first, we need to open the image/video file asynchronously.
                            let in_path: &Path = Path::new(&absolute_path);
                            let in_file: File = File::open(in_path).await.unwrap();

                            let (metadata, hash) = get_metadata_with_hash(in_file).await.unwrap();

                            // Store the `MetadataInfo` struct in our `GridCellData` object.
                            let _ = cell.imp().file_metadata.set(metadata);

                            if let Ok(path) = generate_thumbnail_image(in_path, &hash, s.obj().hardware_accel()).await {
                                drop(semaphore_guard);

                                if let Err(err_string) = tx.send(path).await {
                                    g_critical!(
                                        "MediaGridView",
                                        "Tried to transmit thumbnail path, async channel is not open.\n{}",
                                        err_string
                                    );
                                }
                            } else {
                                g_warning!("MediaGridView", "FFmpeg failed to generate a thumbnail image.");
                            }
                        }));
                        let rx_handle = glib::spawn_future_local(clone!(@weak cell => async move {
                            while let Ok(path) = rx.recv().await {
                                cell.imp().image.clear();
                                cell.imp().image.set_file(Some(&path));
                            }
                        }));

                        cell.imp().tx_join_handle.set(Some(tx_handle));
                        cell.imp().rx_join_handle.set(Some(rx_handle));
                    }
                }
                // We can safely ignore the result of this since the bind callback that
                // we are in is going to be called multiple times during the app's lifetime.
                let _ = cell.imp().viewer_content_type.set(get_content_type_from_ext(ext_str));

                // Load image metadata using glycin. Currently video formats are not supported.
                match ext_str {
                    "mov" => (),
                    "mp4" => (),
                    _ => {
                        // NOTE: This adds quite a performance hit on launch
                        glib::spawn_future_local(async move {
                            let loader: glycin::Loader = glycin::Loader::new(file.clone());

                            #[cfg(feature = "disable-glycin-sandbox")]
                            loader.sandbox_mechanism(Some(glycin::SandboxMechanism::NotSandboxed));

                            match loader.load().await {
                                Ok(image) => {
                                    let pic_details = PictureDetails(image.info().clone());
                                    let details = ContentDetails::Picture(pic_details);

                                    cell.imp().content_details.swap(&RefCell::new(details));
                                }
                                Err(glycin_err) => g_warning!(
                                    "MediaGridView",
                                    "{}: Glycin error: {}",
                                    file.basename().unwrap().to_string_lossy(),
                                    glycin_err
                                ),
                            }
                        });
                    }
                }
            } else {
                g_warning!(
                    "MediaGridView",
                    "Found a file with no file extension, with file path '{}'.",
                    absolute_path
                );
            }
        }));

        self.photo_grid_view.set_factory(Some(&self.list_item_factory));

        // We have to add the theme selector widget as a child of our
        // GtkPopoverMenu widget manually here, because the UI XML method
        // does not work (for some reason..) GTK and its docs are a pain.
        let new_theme_selector = MrsThemeSelector::new();
        self.primary_menu.add_child(&new_theme_selector, "theme-selector");
    }
}

impl WidgetImpl for MrsMediaGridView {}
impl BinImpl for MrsMediaGridView {}
impl BreakpointBinImpl for MrsMediaGridView {}
