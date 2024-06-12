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

mod details;
mod imp;
mod media_grid;
pub mod viewer;

use crate::application::library_list_model::MrsLibraryListModel;
use crate::application::MrsApplication;
use crate::globals::{APP_INFO, FFMPEG_BINARY};
use crate::i18n::gettext_f;
use crate::window::MrsApplicationWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{clone, g_debug, g_error};
use gtk::{gio, glib};
use std::io;
use std::process::Command;

glib::wrapper! {
    pub struct MrsLibraryView(ObjectSubclass<imp::MrsLibraryView>)
        @extends gtk::Widget, adw::Bin;
}

impl MrsLibraryView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn window(&self) -> MrsApplicationWindow {
        self.root()
            .expect("Must be in a GtkApplicationWindow.")
            .downcast()
            .expect("Failed to downcast to MrsApplicationWindow.")
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

        let memories: MrsApplication = self.window().app().unwrap();
        let library_model: MrsLibraryListModel = memories.library_list_model();

        let msm: gtk::MultiSelection = gtk::MultiSelection::new(Some(library_model.clone()));

        if !library_model.models_loaded() {
            library_model.connect_models_loaded_notify(
                clone!(@weak self as s => move |model: &MrsLibraryListModel| {
                    g_debug!("Library", "notify::models_loaded");

                    let item_count: u32 = model.n_items();
                    if item_count == 0 {
                        s.imp().library_view_stack.set_visible_child_name("placeholder_page");
                        return;
                    }
                    s.imp().library_view_stack.set_visible_child_name("gallery_page");
                    s.imp().spinner.stop();

                    let gsettings: gio::Settings = MrsApplication::default().gsettings();

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

        library_model.connect_error_notify(move |dl: &gtk::DirectoryList| {
            g_error!(
                "Library",
                "MrsLibraryListModel returned an error!\n\n{}",
                dl.error().unwrap()
            );
        });

        self.imp().media_grid.imp().photo_grid_view.set_model(Some(&msm));

        if let Err(err_str) = library_model.start_enumerating_items() {
            self.imp().library_view_stack.set_visible_child_name("error_page");
            self.imp().error_status_widget.set_description(Some(&err_str));
        }
    }
}

impl Default for MrsLibraryView {
    fn default() -> Self {
        Self::new()
    }
}
