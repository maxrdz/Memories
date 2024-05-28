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

use super::AlbumsLibraryView;
use crate::library::details::{ContentDetails, PictureDetails};
use crate::thumbnails::generate_thumbnail_image;
use crate::utils::{get_content_type_from_ext, get_metadata_with_hash};
use adw::glib;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use async_fs::File;
use async_semaphore::{Semaphore, SemaphoreGuard};
use glib::{g_critical, g_warning};
use glib_macros::clone;
use libadwaita as adw;
use media_cell::AlbumsMediaCell;
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;

glib::wrapper! {
    pub struct AlbumsMediaGridView(ObjectSubclass<imp::AlbumsMediaGridView>)
        @extends gtk::Widget, adw::Bin, adw::BreakpointBin;
}

impl AlbumsMediaGridView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    /// Returns a new `GtkSignalListItemFactory` with signal handlers allocated
    /// to create, bind, and clean up list item widgets in the library grid view.
    pub fn create_list_item_factory(library: &AlbumsLibraryView) -> gtk::SignalListItemFactory {
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(
            clone!(@weak library => move |_: &gtk::SignalListItemFactory, obj: &glib::Object| {
                    let list_item_widget: gtk::ListItem = obj.clone().downcast().unwrap();

                    let cell: AlbumsMediaCell = AlbumsMediaCell::default();
                    cell.setup_cell(&library, &list_item_widget);
                }
            ),
        );

        factory.connect_bind(clone!(@weak library => move |_: &gtk::SignalListItemFactory, obj: &glib::Object| {
            let list_item: gtk::ListItem = obj.clone().downcast().unwrap();
            let cell: AlbumsMediaCell = list_item.child().and_downcast().unwrap();

            let model_list_item: gio::FileInfo = list_item.item().and_downcast().unwrap();

            // Store `GFileInfo` object reference in `AlbumsMediaCell` object.
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
                        let semaphore: Arc<Semaphore> = library.imp().subprocess_semaphore.clone();

                        let tx_handle = glib::spawn_future_local(clone!(@weak library, @weak cell => async move {
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

                            if let Ok(path) = generate_thumbnail_image(in_path, &hash, library.hardware_accel()).await {
                                drop(semaphore_guard);

                                if let Err(err_string) = tx.send(path).await {
                                    g_critical!(
                                        "Library",
                                        "Tried to transmit thumbnail path, async channel is not open.\n{}",
                                        err_string
                                    );
                                }
                            } else {
                                g_warning!("Library", "FFmpeg failed to generate a thumbnail image.");
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
                                    "Library",
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
                    "Library",
                    "Found a file with no file extension, with file path '{}'.",
                    absolute_path
                );
            }
        }));

        factory
    }
}

impl Default for AlbumsMediaGridView {
    fn default() -> Self {
        Self::new()
    }
}
