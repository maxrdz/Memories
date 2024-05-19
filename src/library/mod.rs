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

mod details;
mod imp;
pub mod library_list_model;
pub mod viewer;

use crate::application::AlbumsApplication;
use crate::globals::APP_INFO;
use crate::globals::DEFAULT_LIBRARY_DIRECTORY;
use crate::i18n::gettext_f;
use crate::library::details::{ContentDetails, PictureDetails};
use crate::thumbnails::{generate_thumbnail_image, FFMPEG_BINARY};
use crate::utils::{get_content_type_from_ext, get_metadata_with_hash, MetadataInfo};
use crate::window::AlbumsApplicationWindow;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use async_fs::File;
use async_semaphore::{Semaphore, SemaphoreGuard};
use gettextrs::gettext;
use glib::{g_critical, g_debug, g_error, g_warning};
use glib_macros::clone;
use gtk::{gio, glib};
use libadwaita as adw;
use library_list_model::LibraryListModel;
use std::cell::RefCell;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

glib::wrapper! {
    pub struct LibraryView(ObjectSubclass<imp::LibraryView>)
        @extends gtk::Widget, adw::Bin;
}

impl LibraryView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn window(&self) -> AlbumsApplicationWindow {
        self.root()
            .expect("Must be in a GtkApplicationWindow.")
            .downcast()
            .expect("Failed to downcast to AlbumsApplicationWindow.")
    }

    /// Called by MasterWindow once the Library view stack page is visible on screen.
    pub fn load_library(&self) {
        // First things first, check that the ffmpeg binary is installed.
        if let Err(e) = Command::new(FFMPEG_BINARY).output() {
            self.imp().library_view_stack.set_visible_child_name("error_page");

            match e.kind() {
                io::ErrorKind::NotFound => {
                    self.imp().error_status_widget.set_description(Some(&gettext_f(
                        "{BIN} was not found on your system. {APP} requires {BIN} to run.",
                        &[("BIN", FFMPEG_BINARY), ("APP", APP_INFO.app_name)],
                    )));
                    return;
                }
                io::ErrorKind::PermissionDenied => {
                    self.imp().error_status_widget.set_description(Some(&gettext_f(
                        "{APP} does not have the sufficient permissions to run {BIN}.",
                        &[("BIN", FFMPEG_BINARY), ("APP", APP_INFO.app_name)],
                    )));
                    return;
                }
                _ => g_error!("LibraryView", "Unexpected error received at ffmpeg binary check."),
            }
        }
        self.imp().spinner.start();

        let albums: AlbumsApplication = self.window().app().unwrap();
        let llm: LibraryListModel = albums.library_list_model();

        let msm: gtk::MultiSelection = gtk::MultiSelection::new(Some(llm.clone()));

        if !llm.models_loaded() {
            llm.connect_models_loaded_notify(clone!(@weak self as s => move |model: &LibraryListModel| {
                let item_count: u32 = model.n_items();
                if item_count == 0 {
                    s.imp().library_view_stack.set_visible_child_name("placeholder_page");
                    return;
                }
                s.imp().library_view_stack.set_visible_child_name("gallery_page");
                s.imp().spinner.stop();
            }));
        } else {
            self.imp()
                .library_view_stack
                .set_visible_child_name("gallery_page");
            self.imp().spinner.stop();
        }

        llm.connect_items_changed(
            clone!(@weak self as s => move |model: &LibraryListModel, _: u32, _: u32, _:u32| {
                let item_count: u32 = model.n_items();
                s.imp().total_items_label.set_label(&format!("{} {}", item_count, &gettext("Items")));
            }),
        );

        llm.connect_error_notify(move |dl: &gtk::DirectoryList| {
            g_error!(
                "LibraryView",
                "GtkDirectoryList returned an error!\n\n{}",
                dl.error().unwrap()
            );
        });

        let factory: gtk::SignalListItemFactory = self.create_list_item_factory();

        self.imp().photo_grid_view.set_model(Some(&msm));
        self.imp().photo_grid_view.set_factory(Some(&factory));

        let absolute_library_dir: String = format!(
            "{}/{}",
            {
                if let Ok(home_path) = env::var("HOME") {
                    home_path
                } else {
                    g_critical!("LibraryView", "No $HOME env var found! Cannot open photo albums.");

                    self.imp().library_view_stack.set_visible_child_name("error_page");
                    self.imp().error_status_widget.set_description(Some(&gettext_f(
                        // TRANSLATORS: You can remove odd spacing. This is due to code linting.
                        "The {ENV_VAR} environment variable was found, \
                        so Albums cannot open your photo library.",
                        &[("ENV_VAR", "$HOME")],
                    )));
                    // place NULL byte at start of string to signal error
                    String::from('\0')
                }
            },
            DEFAULT_LIBRARY_DIRECTORY
        );

        if !absolute_library_dir.starts_with('\0') {
            g_debug!(
                "LibraryView",
                "Enumerating library files from directory: {}",
                absolute_library_dir
            );
            llm.set_file(Some(&gio::File::for_path(absolute_library_dir)));
        }

        let gsettings: gio::Settings = AlbumsApplication::default().gsettings();

        if gsettings.boolean("first-boot") {
            let new_toast: adw::Toast = adw::Toast::builder()
                .title(gettext(
                    "Creating photo thumbnails for the first time. This may take a while.",
                ))
                .build();
            self.imp().gallery_toast_overlay.add_toast(new_toast);

            let _ = gsettings.set_boolean("first-boot", false);
        }
    }

    /// Returns a new `GtkSignalListItemFactory` with signal handlers allocated
    /// to create, bind, and clean up list item widgets in the library grid view.
    fn create_list_item_factory(&self) -> gtk::SignalListItemFactory {
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(
            clone!(@weak self as s => move |_: &gtk::SignalListItemFactory, obj: &glib::Object| {
                let list_item_widget: gtk::ListItem = obj.clone().downcast().unwrap();

                let image: gtk::Image = gtk::Image::builder()
                    .use_fallback(true)
                    .icon_size(gtk::IconSize::Large)
                    .icon_name("emblem-photos-symbolic")
                    .build();
                let aspect_frame: gtk::AspectFrame = gtk::AspectFrame::builder()
                    .child(&image)
                    .height_request(s.grid_widget_height())
                    .build();

                s.bind_property("grid-widget-height", &aspect_frame, "height-request").sync_create().build();

                let revealer: gtk::Revealer = gtk::Revealer::builder()
                    .child(&aspect_frame)
                    .transition_type(gtk::RevealerTransitionType::None)
                    .reveal_child(true)
                    .build();

                let cell_data: GridCellData = GridCellData::builder()
                    .child(&revealer)
                    .build();

                // Once the image file has been set, we know it has been loaded, so
                // we can hide the content (placeholder icon) immediately, then reveal
                // the actual image content with a proper delay + transition type.
                let handler_id: glib::SignalHandlerId = image.connect_file_notify(clone!(@weak revealer as r => move |_: &gtk::Image| {
                    r.set_reveal_child(false);
                    r.set_transition_duration(1000); // milliseconds
                    r.set_transition_type(gtk::RevealerTransitionType::Crossfade);
                    r.set_reveal_child(true);
                }));

                cell_data.imp()
                    .img_file_notify
                    .borrow()
                    .set(handler_id)
                    .expect("Cell data `img_file_notify` already initialized!");

                list_item_widget.set_property("child", &cell_data);

                list_item_widget.connect_selected_notify(
                    clone!(@weak s as library_view, @weak list_item_widget as li => move |_| {
                        if li.is_selected() {
                            let current_nav_page: adw::NavigationPage = library_view.window()
                                .imp()
                                .window_navigation
                                .visible_page()
                                .unwrap();

                            // Do not proceed to push a new nav page if one is already open.
                            if current_nav_page.tag().unwrap() != "window" {
                                return;
                            }
                            let grid_cell_data: GridCellData = li.child().and_downcast().unwrap();

                            let model_item: gio::FileInfo = li.item().and_downcast().unwrap();
                            let file_obj: glib::Object = model_item.attribute_object("standard::file").unwrap();
                            let file: gio::File = file_obj.downcast().unwrap();

                            let nav_view = library_view.window().imp().window_navigation.clone();

                            let viewer_content: viewer::Viewer = viewer::Viewer::default();
                            viewer_content.set_content_type(grid_cell_data.imp().viewer_content_type.get().unwrap());
                            viewer_content.set_content_file(&file);

                            viewer_content.imp()
                                .details_widget
                                .update_details(&grid_cell_data);

                            let nav_page: adw::NavigationPage = viewer_content.wrap_in_navigation_page();
                            nav_page.set_title(&file.basename().unwrap().to_string_lossy());

                            nav_view.push(&nav_page);
                        }
                    }),
                );
            }
        ));

        factory.connect_bind(clone!(@weak self as s => move |_: &gtk::SignalListItemFactory, obj: &glib::Object| {
            let list_item: gtk::ListItem = obj.clone().downcast().unwrap();
            // There **has** to be a better way to get the GtkImage object.
            let cell_data: GridCellData = list_item.child().and_downcast().unwrap();
            let revealer: gtk::Revealer = cell_data.child().and_downcast().unwrap();
            let frame: gtk::AspectFrame = revealer.child().and_downcast().unwrap();
            let image: gtk::Image = frame.child().and_downcast().unwrap();

            let model_list_item: gio::FileInfo = list_item.item().and_downcast().unwrap();

            // Store `GFileInfo` object reference in `GridCellData` object.
            let _ = cell_data.imp().file_info.set(model_list_item.clone());

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
                    "svg" => image.set_file(Some(&absolute_path)),
                    _ => {
                        let (tx, rx) = async_channel::bounded(1);
                        let semaphore: Arc<Semaphore> = s.imp().subprocess_semaphore.clone();

                        let tx_handle = glib::spawn_future_local(clone!(@weak s as lv, @weak cell_data as cd => async move {
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
                            let _ = cd.imp().file_metadata.set(metadata);

                            if let Ok(path) = generate_thumbnail_image(in_path, &hash, lv.hardware_accel()).await {
                                drop(semaphore_guard);

                                if let Err(err_string) = tx.send(path).await {
                                    g_critical!(
                                        "LibraryView",
                                        "Tried to transmit thumbnail path, async channel is not open.\n{}",
                                        err_string
                                    );
                                }
                            } else {
                                g_warning!("LibraryView", "FFmpeg failed to generate a thumbnail image.");
                            }
                        }));
                        let rx_handle = glib::spawn_future_local(clone!(@weak image => async move {
                            while let Ok(path) = rx.recv().await {
                                image.clear();
                                image.set_file(Some(&path));
                            }
                        }));

                        cell_data.imp().tx_join_handle.set(Some(tx_handle));
                        cell_data.imp().rx_join_handle.set(Some(rx_handle));
                    }
                }
                // We can safely ignore the result of this since the bind callback that
                // we are in is going to be called multiple times during the app's lifetime.
                let _ = cell_data.imp().viewer_content_type.set(get_content_type_from_ext(ext_str));

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

                                    cell_data.imp().content_details.swap(&RefCell::new(details));
                                }
                                Err(glycin_err) => g_warning!(
                                    "LibraryView",
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
                    "LibraryView",
                    "Found a file with no file extension, with file path '{}'.",
                    absolute_path
                );
            }
        }));

        factory
    }
}

impl Default for LibraryView {
    fn default() -> Self {
        Self::new()
    }
}

mod grid_cell_data_imp {
    use super::adw;
    use super::details::ContentDetails;
    use super::glib;
    use super::viewer::ViewerContentType;
    use super::MetadataInfo;
    use adw::subclass::prelude::*;
    use std::cell::{Cell, OnceCell, RefCell};

    /// `AdwBin` subclass to store arbitrary data for grid cells
    /// of the library photo grid view. Stores signal
    /// handler IDs, glib async join handles, metadata, etc.
    #[derive(Default)]
    pub struct GridCellData {
        pub img_file_notify: RefCell<OnceCell<glib::SignalHandlerId>>,
        pub tx_join_handle: Cell<Option<glib::JoinHandle<()>>>,
        pub rx_join_handle: Cell<Option<glib::JoinHandle<()>>>,
        pub file_info: OnceCell<gio::FileInfo>,
        pub file_metadata: OnceCell<MetadataInfo>,
        pub viewer_content_type: OnceCell<ViewerContentType>,
        pub content_details: RefCell<ContentDetails>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GridCellData {
        const NAME: &'static str = "GridCellData";
        type ParentType = adw::Bin;
        type Type = super::GridCellData;
    }

    impl ObjectImpl for GridCellData {}
    impl WidgetImpl for GridCellData {}
    impl BinImpl for GridCellData {}
}

glib::wrapper! {
    pub struct GridCellData(ObjectSubclass<grid_cell_data_imp::GridCellData>)
        @extends gtk::Widget, adw::Bin;
}

impl GridCellData {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> GridCellDataBuilder {
        GridCellDataBuilder::new()
    }
}

impl Default for GridCellData {
    fn default() -> Self {
        Self::new()
    }
}

/// A [builder-pattern] type to construct `LibraryGridCellData` objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used."]
pub struct GridCellDataBuilder {
    builder: glib::object::ObjectBuilder<'static, GridCellData>,
}

impl GridCellDataBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    /// Build the `GridCellData` object.
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects."]
    pub fn build(self) -> GridCellData {
        self.builder.build()
    }
}
