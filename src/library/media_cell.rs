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

use crate::library::media_grid::MemoriesMediaGridView;
use crate::library::media_viewer::{MemoriesMediaViewer, ViewerContentType};
use crate::library::properties::{ContentDetails, PictureDetails};
use crate::util::metadata::get_metadata_with_hash;
use crate::util::thumbnails::generate_thumbnail_image;
use adw::prelude::*;
use adw::subclass::prelude::*;
use async_fs::File;
use async_semaphore::{Semaphore, SemaphoreGuard};
use glib::{clone, g_critical, g_error, g_warning};
use glycin::Loader;
#[cfg(feature = "disable-glycin-sandbox")]
use glycin::SandboxMechanism;
use gtk::{gio, glib};
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;

mod imp {
    use crate::library::media_viewer::ViewerContentType;
    use crate::library::properties::ContentDetails;
    use crate::util::metadata::MetadataInfo;
    use adw::subclass::prelude::*;
    use gtk::{gio, glib};
    use std::cell::{Cell, OnceCell, RefCell};

    /// `AdwBin` subclass to store arbitrary data for grid cells
    /// of the library photo grid view. Stores signal
    /// handler IDs, glib async join handles, metadata, etc.
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/media-cell.ui")]
    pub struct MemoriesMediaCell {
        #[template_child]
        pub(super) revealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub(super) aspect_frame: TemplateChild<gtk::AspectFrame>,
        #[template_child]
        pub thumbnail_image: TemplateChild<gtk::Image>,
        #[template_child]
        favorited: TemplateChild<gtk::Image>,
        #[template_child]
        media_type_icon: TemplateChild<gtk::Image>,
        #[template_child]
        video_length: TemplateChild<gtk::Label>,

        pub img_file_notify: RefCell<OnceCell<glib::SignalHandlerId>>,
        pub tx_join_handle: Cell<Option<glib::JoinHandle<()>>>,
        pub rx_join_handle: Cell<Option<glib::JoinHandle<()>>>,
        pub file_info: OnceCell<gio::FileInfo>,
        pub file_metadata: OnceCell<MetadataInfo>,
        pub viewer_content_type: OnceCell<ViewerContentType>,
        pub content_details: RefCell<ContentDetails>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoriesMediaCell {
        const NAME: &'static str = "MemoriesMediaCell";
        type ParentType = adw::Bin;
        type Type = super::MemoriesMediaCell;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.set_css_name("mediacell");
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MemoriesMediaCell {}
    impl WidgetImpl for MemoriesMediaCell {}
    impl BinImpl for MemoriesMediaCell {}
}

glib::wrapper! {
    pub struct MemoriesMediaCell(ObjectSubclass<imp::MemoriesMediaCell>)
        @extends gtk::Widget, adw::Bin;
}

impl MemoriesMediaCell {
    pub fn new() -> Self {
        glib::Object::new()
    }

    /// Called once by the list item widget factory when it creates a new cell.
    pub fn setup_cell(&self, media_grid: &MemoriesMediaGridView, list_item: &gtk::ListItem) {
        // First things first, set the list item widget as our parent.
        list_item.set_property("child", self);
        self.imp()
            .aspect_frame
            .set_height_request(media_grid.grid_widget_height());

        // Bind the `GtkAspectFrame`s height-request to the `grid-widget-height`
        // property of the `MemoriesMediaGridView` object.
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
                .thumbnail_image
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
                    let media_cell: MemoriesMediaCell = list_item.child().and_downcast().unwrap();

                    let model_item: gio::FileInfo = list_item.item().and_downcast().unwrap();
                    let file_obj: glib::Object = model_item.attribute_object("standard::file").unwrap();
                    let file: gio::File = file_obj.downcast().unwrap();

                    let nav_view = media_grid.window().imp().window_navigation.clone();

                    let viewer_content: MemoriesMediaViewer = MemoriesMediaViewer::default();
                    viewer_content.set_content_type(media_cell.imp().viewer_content_type.get().unwrap());
                    viewer_content.set_content_file(&file);

                    viewer_content.imp()
                        .details_widget
                        .update_details(&media_cell);

                    let nav_page: adw::NavigationPage = viewer_content.wrap_in_navigation_page();
                    nav_page.set_title(&file.basename().unwrap().to_string_lossy());

                    nav_view.push(&nav_page);

                    // See docstring of setup_gactions() for why we're calling it here.
                    viewer_content.setup_gactions();
                }
            }
        ));
    }

    /// Called every time the list item widget factory fires the 'bind'
    /// event on the list item widget, which loads it with new data.
    pub fn bind_cell(
        &self,
        media_grid_imp: &super::media_grid::imp::MemoriesMediaGridView,
        content_type: ViewerContentType,
        list_item: &gtk::ListItem,
    ) {
        // First, let's unwrap the media's `GFile` from our list item widget.
        let model_list_item: gio::FileInfo = list_item.item().and_downcast().unwrap();
        let file_obj: glib::Object = model_list_item.attribute_object("standard::file").unwrap();
        let file: gio::File = file_obj.downcast().unwrap();

        let file_path_buf: std::path::PathBuf = file.path().unwrap();

        // Convert file_path_buf to a String (not a string slice) since file_path_buf
        // does not live long enough to be borrowed in the futures spawned below.
        let absolute_path: String = file_path_buf.to_string_lossy().to_string();

        // Store content type variant and `GFileInfo` object reference in our object.
        let _ = self.imp().viewer_content_type.set(content_type.clone());
        let _ = self.imp().file_info.set(model_list_item.clone());

        // Match statement for choosing how to load the thumbnail image.
        match content_type {
            // SVGs can be rendered by GNOME's librsvg, so we don't need ffmpeg.
            ViewerContentType::VectorGraphics => self.imp().thumbnail_image.set_file(Some(&absolute_path)),
            _ => {
                let (tx, rx) = async_channel::bounded(1);
                let semaphore: Arc<Semaphore> = media_grid_imp.subprocess_semaphore.clone();

                let tx_handle = glib::spawn_future_local(
                    clone!(@weak self as cell, @weak media_grid_imp => async move {
                        let semaphore_guard: SemaphoreGuard<'_> = semaphore.acquire().await;

                        // We need to get 3 things done in this closure:
                        // - file metadata
                        // - metadata md5 digest
                        // - thumbnail image
                        // So, first, we need to open the image/video file asynchronously.
                        let in_path: &Path = Path::new(&absolute_path);
                        let in_file: File = File::open(in_path).await.unwrap();

                        let (metadata, hash) = get_metadata_with_hash(in_file).await.unwrap();

                        // Store the `MetadataInfo` struct in our `MemoriesMediaCell` object.
                        let _ = cell.imp().file_metadata.set(metadata);

                        if let Ok(path) = generate_thumbnail_image(in_path, &hash, media_grid_imp.obj().hardware_accel()).await {
                            drop(semaphore_guard);

                            if let Err(err_string) = tx.send(path).await {
                                g_critical!(
                                    "MediaCell",
                                    "Tried to transmit thumbnail path, async channel is not open.\n{}",
                                    err_string
                                );
                            }
                        } else {
                            g_warning!("MediaCell", "FFmpeg failed to generate a thumbnail image.");
                        }
                    }),
                );

                let rx_handle = glib::spawn_future_local(clone!(@weak self as cell => async move {
                    while let Ok(path) = rx.recv().await {
                        cell.imp().thumbnail_image.clear();
                        cell.imp().thumbnail_image.set_file(Some(&path));
                    }
                }));

                self.imp().tx_join_handle.set(Some(tx_handle));
                self.imp().rx_join_handle.set(Some(rx_handle));
            }
        }

        // Match statement for choosing how to get the media metadata.
        match content_type {
            // TODO: Currently video format metadata is not yet implemented.
            ViewerContentType::Video => (),
            // If the media is a picture, load its texture and metadata with glycin.
            ViewerContentType::Image | ViewerContentType::VectorGraphics => {
                // FIXME: This adds quite a performance hit. Maybe do all
                // glycin metadata processing on a new separate thread?
                glib::spawn_future_local(clone!(@weak self as cell => async move {
                    #[allow(unused_mut)]
                    let mut glycin_loader: Loader = Loader::new(file.clone());

                    #[cfg(feature = "disable-glycin-sandbox")]
                    glycin_loader.sandbox_mechanism(Some(SandboxMechanism::NotSandboxed));

                    match glycin_loader.load().await {
                        Ok(image) => {
                            let pic_details = PictureDetails(image.info().clone());
                            let details = ContentDetails::Picture(pic_details);

                            cell.imp().content_details.swap(&RefCell::new(details));
                        }
                        Err(glycin_err) => g_warning!(
                            "MediaCell",
                            "{}: Glycin error: {}",
                            file.basename().unwrap().to_string_lossy(),
                            glycin_err
                        ),
                    }
                }));
            }
            ViewerContentType::Invalid => {
                g_error!(
                    "MediaCell",
                    "Received `ViewerContentType::Invalid`. Should not happen!"
                );
            }
        }
    }
}

impl Default for MemoriesMediaCell {
    fn default() -> Self {
        Self::new()
    }
}
