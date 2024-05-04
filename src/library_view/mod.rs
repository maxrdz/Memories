// library_view/mod.rs
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
mod library_list_model;

use crate::globals::APP_INFO;
use crate::globals::DEFAULT_LIBRARY_DIRECTORY;
use crate::i18n::gettext_f;
use crate::thumbnails::{generate_thumbnail_image, FFMPEG_BINARY};
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use async_semaphore::{Semaphore, SemaphoreGuard};
use gettextrs::gettext;
use glib::{g_critical, g_debug, g_error, g_warning};
use glib_macros::clone;
use gtk::{gio, glib};
use libadwaita as adw;
use library_list_model::LibraryListModel;
use std::env;
use std::io;
use std::process::Command;
use std::sync::Arc;

glib::wrapper! {
    pub struct LibraryView(ObjectSubclass<imp::LibraryView>)
        @extends gtk::Widget, adw::Bin,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl LibraryView {
    pub fn new<P: IsA<adw::gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
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

        let llm: LibraryListModel = LibraryListModel::default();
        let msm: gtk::MultiSelection = gtk::MultiSelection::new(
            // We can clone our LibraryListModel model because gobjects are reference-counted.
            Some(llm.clone()),
        );

        llm.connect_models_loaded_notify(clone!(@weak self as s => move |model: &LibraryListModel| {
            let item_count: u32 = model.n_items();
            if item_count == 0 {
                s.imp().library_view_stack.set_visible_child_name("placeholder_page");
                return;
            }
            s.imp().library_view_stack.set_visible_child_name("gallery_page");
            s.imp().spinner.stop();
        }));

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

        let lif: gtk::SignalListItemFactory = gtk::SignalListItemFactory::new();

        lif.connect_setup(
            clone!(@weak self as s => move |_: &gtk::SignalListItemFactory, obj: &glib::Object| {
                let list_item: gtk::ListItem = obj.clone().downcast().unwrap();

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

                // Once the image file has been set, we know it has been loaded, so
                // we can hide the content (placeholder icon) immediately, then reveal
                // the actual image content with a proper delay + transition type.
                image.connect_file_notify(clone!(@weak revealer as r => move |_: &gtk::Image| {
                    r.set_reveal_child(false);
                    r.set_transition_duration(1000); // milliseconds
                    r.set_transition_type(gtk::RevealerTransitionType::Crossfade);
                    r.set_reveal_child(true);
                }));

                list_item.set_property("child", &revealer);
            }
        ));

        lif.connect_bind(clone!(@weak self as s => move |_: &gtk::SignalListItemFactory, obj: &glib::Object| {
            let list_item: gtk::ListItem = obj.clone().downcast().unwrap();
            // There **has** to be a better way to get the GtkImage object.
            let revealer: gtk::Revealer = list_item.child().and_downcast().unwrap();
            let frame: gtk::AspectFrame = revealer.child().and_downcast().unwrap();
            let image: gtk::Image = frame.child().and_downcast().unwrap();

            let model_list_item: gio::FileInfo = list_item.item().and_downcast().unwrap();

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

                        glib::spawn_future_local(clone!(@strong tx, @strong semaphore as sp => async move {
                            let semaphore_guard: SemaphoreGuard<'_> = sp.acquire().await;

                            if let Ok(path) = generate_thumbnail_image(&absolute_path).await {
                                drop(semaphore_guard);
                                tx.send(path).await.expect("Async channel needs to be open.");
                            } else {
                                g_critical!("LibraryView", "FFmpeg failed to generate a thumbnail image.");
                            }
                        }));
                        glib::spawn_future_local(clone!(@weak image => async move {
                            while let Ok(path) = rx.recv().await {
                                image.clear();
                                image.set_file(Some(&path));
                            }
                        }));
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

        self.imp().photo_grid_view.set_model(Some(&msm));
        self.imp().photo_grid_view.set_factory(Some(&lif));

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
                        so Album cannot open your photo library.",
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
    }
}
