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
use crate::globals::APP_INFO;
use crate::globals::DEFAULT_LIBRARY_DIRECTORY;
use crate::i18n::gettext_f;
use crate::thumbnails::FFMPEG_BINARY;
use crate::window::AlbumsApplicationWindow;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{g_critical, g_debug, g_error};
use glib_macros::clone;
use gtk::{gio, glib};
use libadwaita as adw;
use media_grid::AlbumsMediaGridView;
use std::env;
use std::io;
use std::process::Command;

glib::wrapper! {
    pub struct AlbumsLibraryView(ObjectSubclass<imp::AlbumsLibraryView>)
        @extends gtk::Widget, adw::Bin, adw::BreakpointBin;
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

        let albums: AlbumsApplication = self.window().app().unwrap();
        let llm: AlbumsLibraryListModel = albums.library_list_model();

        let msm: gtk::MultiSelection = gtk::MultiSelection::new(Some(llm.clone()));

        if !llm.models_loaded() {
            llm.connect_models_loaded_notify(
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

        llm.connect_items_changed(
            clone!(@weak self as s => move |model: &AlbumsLibraryListModel, _: u32, _: u32, _:u32| {
                let item_count: u32 = model.n_items();
                g_debug!("Library", "Updated list model item count: {}", item_count);
                s.imp().media_grid.imp().total_items_label.set_label(&format!("{} {}", item_count, &gettext("Items")));
            }),
        );

        llm.connect_error_notify(move |dl: &gtk::DirectoryList| {
            g_error!(
                "Library",
                "AlbumsLibraryListModel returned an error!\n\n{}",
                dl.error().unwrap()
            );
        });

        let factory: gtk::SignalListItemFactory = AlbumsMediaGridView::create_list_item_factory(self);

        self.imp().media_grid.imp().photo_grid_view.set_model(Some(&msm));
        self.imp()
            .media_grid
            .imp()
            .photo_grid_view
            .set_factory(Some(&factory));

        let absolute_library_dir: String = format!(
            "{}/{}",
            {
                if let Ok(home_path) = env::var("HOME") {
                    home_path
                } else {
                    g_critical!("Library", "No $HOME env var found! Cannot open photo albums.");

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
                "Library",
                "Enumerating library files from directory: {}",
                absolute_library_dir
            );
            llm.set_file(Some(&gio::File::for_path(absolute_library_dir)));
        }
    }
}

impl Default for AlbumsLibraryView {
    fn default() -> Self {
        Self::new()
    }
}
