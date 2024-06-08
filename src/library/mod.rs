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
mod media_grid;
pub mod viewer;

use crate::application::library_list_model::AlbumsLibraryListModel;
use crate::application::AlbumsApplication;
use crate::globals::{APP_INFO, FFMPEG_BINARY};
use crate::i18n::gettext_f;
use crate::window::AlbumsApplicationWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{clone, g_debug, g_error};
use gtk::{gio, glib};
use std::io;
use std::process::Command;

glib::wrapper! {
    pub struct AlbumsLibraryView(ObjectSubclass<imp::AlbumsLibraryView>)
        @extends gtk::Widget, adw::Bin;
}

impl AlbumsLibraryView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn window(&self) -> AlbumsApplicationWindow {
        self.root()
            .expect("Must be in a GtkApplicationWindow.")
            .downcast()
            .expect("Failed to downcast to AlbumsApplicationWindow.")
    }

    fn update_library_item_count(&self) {
        let albums: AlbumsApplication = self.window().app().unwrap();
        let library_model: AlbumsLibraryListModel = albums.library_list_model();

        let item_count: u32 = library_model.n_items();
        g_debug!("Library", "Updated list model item count: {}", item_count);

        self.imp().media_grid.imp().total_items_label.set_label(&format!(
            "{} {}",
            item_count,
            &gettext("Items")
        ));
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
                _ => g_error!("Library", "Unexpected error received at ffmpeg binary check."),
            }
        }
        self.imp().spinner.start();

        // We need to refresh the item count when the gallery page is visible.
        // If we only do this at the list model's `items_changed` notify signal,
        // it will not update the item count if the user started the app on the
        // albums view instead of the default library view.
        self.imp().library_view_stack.connect_visible_child_name_notify(
            clone!(@weak self as s => move |stack: &adw::ViewStack| {
                if stack.visible_child_name().unwrap() == "gallery_page" {
                    s.update_library_item_count();
                }
            }),
        );

        let albums: AlbumsApplication = self.window().app().unwrap();
        let library_model: AlbumsLibraryListModel = albums.library_list_model();

        let msm: gtk::MultiSelection = gtk::MultiSelection::new(Some(library_model.clone()));

        if !library_model.models_loaded() {
            library_model.connect_models_loaded_notify(
                clone!(@weak self as s => move |model: &AlbumsLibraryListModel| {
                    g_debug!("Library", "notify::models_loaded");

                    let item_count: u32 = model.n_items();
                    if item_count == 0 {
                        s.imp().library_view_stack.set_visible_child_name("placeholder_page");
                        return;
                    }
                    s.imp().library_view_stack.set_visible_child_name("gallery_page");
                    s.imp().spinner.stop();

                    let gsettings: gio::Settings = AlbumsApplication::default().gsettings();

                    // If our cache is not populated, warn the user that this may take a while.
                    if gsettings.boolean("fresh-cache") {
                        let new_toast: adw::Toast = adw::Toast::builder()
                            .title(gettext(
                                "Making thumbnails for the first time. This may take a while.",
                            ))
                            .build();
                        s.imp().media_grid.imp().toast_overlay.add_toast(new_toast);

                        let _ = gsettings.set_boolean("fresh-cache", false);
                    }
                }),
            );
        } else {
            self.imp()
                .library_view_stack
                .set_visible_child_name("gallery_page");
            self.imp().spinner.stop();
        }

        library_model.connect_items_changed(clone!(@weak self as s => move |_, _, _, _| {
            s.update_library_item_count();
        }));

        library_model.connect_error_notify(move |dl: &gtk::DirectoryList| {
            g_error!(
                "Library",
                "AlbumsLibraryListModel returned an error!\n\n{}",
                dl.error().unwrap()
            );
        });

        self.imp().media_grid.set_custom_title(&gettext("Photo Library"));
        self.imp().media_grid.imp().photo_grid_view.set_model(Some(&msm));

        if let Err(err_str) = library_model.start_enumerating_items() {
            self.imp().library_view_stack.set_visible_child_name("error_page");
            self.imp().error_status_widget.set_description(Some(&err_str));
        }
    }
}

impl Default for AlbumsLibraryView {
    fn default() -> Self {
        Self::new()
    }
}
