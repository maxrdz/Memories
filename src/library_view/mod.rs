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

use crate::globals::DEFAULT_LIBRARY_DIRECTORY;
use crate::i18n::gettext_f;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib_macros::clone;
use gtk::{gio, glib};
use libadwaita as adw;
use library_list_model::LibraryListModel;
use log::{debug, error};
use std::env;

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
        self.imp().spinner.start();

        let llm: LibraryListModel = LibraryListModel::default();
        let msm: gtk::MultiSelection = gtk::MultiSelection::new(
            // We can clone our LibraryListModel model because gobjects are reference-counted.
            Some(llm.clone()),
        );

        llm.connect_loading_notify(clone!(@weak self as s => move |dl: &gtk::DirectoryList| {
            if !dl.is_loading() {
                let item_count: u32 = dl.n_items();
                debug!("Enumerated {} items in library path.", item_count);

                for item in dl.iter::<gio::FileInfo>() {
                    debug!("{:?}", item);
                    match item.unwrap().file_type() {
                        gio::FileType::Directory => debug!("Directory!"),
                        _ => debug!("Not directory!"),
                    }
                }

                if item_count == 0 {
                    s.imp().library_view_stack.set_visible_child_name("placeholder_page");
                    return;
                }
                s.imp().library_view_stack.set_visible_child_name("gallery_page");
                s.imp().spinner.stop();
            }
        }));
        llm.connect_error_notify(move |dl: &gtk::DirectoryList| {
            error!("GtkDirectoryList returned an error!\n\n{}", dl.error().unwrap());
            panic!("Received an error signal from a critical function.");
        });

        let blif: gtk::BuilderListItemFactory = gtk::BuilderListItemFactory::from_resource(
            None::<&gtk::BuilderRustScope>,
            "/com/maxrdz/Album/library_view/library_list_model/library-list-item.ui",
        );

        self.imp().photo_grid_view.set_model(Some(&msm));
        self.imp().photo_grid_view.set_factory(Some(&blif));

        let absolute_library_dir: String = format!(
            "{}/{}",
            {
                if let Ok(home_path) = env::var("HOME") {
                    home_path
                } else {
                    error!("No $HOME env var found! Cannot open photo albums.");

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
            debug!(
                "Enumerating library files from directory: {}",
                absolute_library_dir
            );
            llm.set_file(Some(&gio::File::for_path(absolute_library_dir)));
        }
    }
}
