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

pub mod list_model;
pub mod media_cell;
pub mod media_grid;
pub mod media_viewer;
pub mod properties;

use crate::application::MemoriesApplication;
use crate::globals::{APP_INFO, FFMPEG_BINARY};
use crate::i18n::gettext_f;
use crate::window::MemoriesApplicationWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{clone, g_debug, g_error};
use gtk::{gio, glib};
use list_model::MemoriesLibraryListModel;
use std::io;
use std::process::Command;

mod imp {
    use super::media_grid::MemoriesMediaGridView;
    use adw::subclass::prelude::*;
    use gtk::glib;
    use std::cell::Cell;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/library.ui")]
    pub struct MemoriesLibraryView {
        pub(super) view_mode: Cell<super::LibraryViewMode>,
        #[template_child]
        pub(super) library_view_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub(super) spinner_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub(super) spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub(super) error_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub(super) error_status_widget: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub(super) gallery_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub media_grid: TemplateChild<MemoriesMediaGridView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoriesLibraryView {
        const NAME: &'static str = "MemoriesLibraryView";
        type Type = super::MemoriesLibraryView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MemoriesLibraryView {}
    impl WidgetImpl for MemoriesLibraryView {}
    impl BinImpl for MemoriesLibraryView {}
}

/// Enum that represents the type of library view
/// instantiated. Depending on the variant, the
/// UI layout and placeholder page are altered
/// for the type of library view set.
#[derive(Debug, Default, Clone, Copy)]
pub enum LibraryViewMode {
    #[default]
    Library,
    Album,
    Favorites,
}

glib::wrapper! {
    pub struct MemoriesLibraryView(ObjectSubclass<imp::MemoriesLibraryView>)
        @extends gtk::Widget, adw::Bin;
}

impl MemoriesLibraryView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn window(&self) -> MemoriesApplicationWindow {
        self.root()
            .expect("Must be in a GtkApplicationWindow.")
            .downcast()
            .expect("Failed to downcast to MemoriesApplicationWindow.")
    }

    /// Sets the library view mode. Depending on the variant, the UI layout
    /// and placeholder page are altered for the type of library view set.
    pub fn set_view_mode(&self, mode_variant: LibraryViewMode) {
        self.imp().view_mode.set(mode_variant);
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

        let memories: MemoriesApplication = self.window().app().unwrap();
        let library_model: MemoriesLibraryListModel = memories.library_list_model();

        let msm: gtk::MultiSelection = gtk::MultiSelection::new(Some(library_model.clone()));

        if !library_model.models_loaded() {
            library_model.connect_models_loaded_notify(
                clone!(@weak self as s => move |model: &MemoriesLibraryListModel| {
                    g_debug!("LibraryView", "notify::models_loaded");

                    let item_count: u32 = model.n_items();

                    if item_count == 0 {
                        let mut placeholder_page: &str = "placeholder_page";

                        match s.imp().view_mode.get() {
                            LibraryViewMode::Album => placeholder_page = "album_placeholder_page",
                            LibraryViewMode::Favorites => placeholder_page = "favorites_placeholder_page",
                            _ => (),
                        }
                        s.imp().library_view_stack.set_visible_child_name(placeholder_page);
                        return;
                    }
                    s.imp().library_view_stack.set_visible_child_name("gallery_page");
                    s.imp().spinner.stop();

                    let gsettings: gio::Settings = MemoriesApplication::default().gsettings();

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
                "LibraryView",
                "MemoriesLibraryListModel returned an error!\n\n{}",
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

impl Default for MemoriesLibraryView {
    fn default() -> Self {
        Self::new()
    }
}
