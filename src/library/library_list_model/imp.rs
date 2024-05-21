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

//! Data model implementation of the LibraryListModel class.

use crate::globals::DEFAULT_LIBRARY_DIRECTORY;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};
use glib::g_debug;
use glib::source::Priority;
use glib_macros::clone;
use libadwaita as adw;
use std::cell::{Cell, RefCell, RefMut};
use std::env;

/// IO priority for new `GtkDirectoryList` models. We override
/// the default since it is usually higher than GTK redraw priority.
static DIRECTORY_MODEL_PRIORITY: Priority = Priority::LOW;

#[derive(Debug)]
struct SubdirectoryListModel {
    model: gtk::DirectoryList,
    _items_changed_callback: glib::SignalHandlerId,
    _loading_callback: glib::SignalHandlerId,
}

/// Custom implementation of GListModel that uses GTK's
/// `GtkDirectoryList` models under the hood to recursively
/// enumerate files under a certain directory path.
#[derive(glib::Properties, Debug)]
#[properties(wrapper_type = super::AlbumsLibraryListModel)]
pub struct AlbumsLibraryListModel {
    pub(super) root_model: gtk::DirectoryList,
    root_items_changed_signal: RefCell<Option<glib::SignalHandlerId>>,
    subdir_models: RefCell<Vec<SubdirectoryListModel>>,
    #[property(get, set)]
    models_loaded: Cell<bool>,
    loading_notifies: Cell<u32>,
    public_items: RefCell<Vec<glib::Object>>,
}

impl Default for AlbumsLibraryListModel {
    fn default() -> Self {
        Self {
            root_model: gtk::DirectoryList::new(None, None::<&gio::File>),
            root_items_changed_signal: RefCell::new(None),
            subdir_models: RefCell::new(vec![]),
            models_loaded: Cell::new(false),
            loading_notifies: Cell::new(0_u32),
            public_items: RefCell::new(vec![]),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsLibraryListModel {
    const NAME: &'static str = "AlbumsLibraryListModel";
    type Type = super::AlbumsLibraryListModel;
    type Interfaces = (gio::ListModel,);
}

#[glib::derived_properties]
impl ObjectImpl for AlbumsLibraryListModel {
    fn constructed(&self) {
        let obj = self.obj();

        // Connect the root model's `items_changed` signal with our model
        // so that the view that owns the `LibraryListModel` can be notified.
        let signal_handler_id: glib::SignalHandlerId =
            self.root_model
                .connect_items_changed(clone!(@weak obj as o => move |
                model: &gtk::DirectoryList, pos: u32, removed: u32, added: u32| {
                    if added != 0 {
                        let base_index: u32 = pos - removed;
                        o.imp().new_model_item_enumerated(model, base_index, Some(added));
                    }
                }));
        self.root_model
            .connect_loading_notify(clone!(@weak self as s => move |
                dl: &gtk::DirectoryList| {
                s.register_subdir_loading_notify(dl);
            }));

        self.root_items_changed_signal.replace(Some(signal_handler_id));
        self.root_model.set_io_priority(DIRECTORY_MODEL_PRIORITY);
    }

    fn dispose(&self) {
        self.cleanup_model();
    }
}

/// Basically just redirect all GListModel interface calls
/// to our underlying GtkDirectoryList objects. Specifically,
/// if the root directory list model is empty, reroute data
/// from our subdirectory models to make this object look
/// like it has a continuous list of items from all subdirs.
impl ListModelImpl for AlbumsLibraryListModel {
    fn item(&self, position: u32) -> Option<glib::Object> {
        self.public_items
            .borrow()
            .get(TryInto::<usize>::try_into(position).unwrap())
            .cloned()
    }

    fn item_type(&self) -> glib::Type {
        self.root_model.item_type()
    }

    fn n_items(&self) -> u32 {
        self.public_items.borrow().len().try_into().unwrap()
    }
}

impl AlbumsLibraryListModel {
    /// Called by a subdirectory GtkDirectoryList model's
    /// 'loading_notify' signal event callback.
    pub(super) fn register_subdir_loading_notify(&self, model: &gtk::DirectoryList) {
        let notifies: u32 = self.loading_notifies.get();

        if !model.is_loading() {
            self.loading_notifies.set(notifies + 1);
            // We don't take into account the latest notify, but we also
            // don't take into account the root model as the models count.
            // So, no need to +1 the RHS & LHS of the expression below lol.
            if notifies == self.subdir_models.borrow().len() as u32 {
                self.obj().set_models_loaded(true);
            }
        }
    }

    /// Called by the handler for the root GtkDirectoryList
    /// model's `items_changed` signal event.
    pub(super) fn new_model_item_enumerated(
        &self,
        list_model: &gtk::DirectoryList,
        base_index: u32,
        items_added: Option<u32>,
    ) {
        if let Some(added) = items_added {
            // 'recursively' call our function per item added.
            for i in 0..added {
                let adjusted_index: u32 = base_index + i;
                self.new_model_item_enumerated(list_model, adjusted_index, None);
            }
            return;
        }
        let q_res: Option<glib::Object> = list_model.item(base_index);
        assert!(
            q_res.is_some(),
            "New item found in root GtkDirectoryList model, but item query returned None type.",
        );
        let file_info: gio::FileInfo = q_res.unwrap().downcast().unwrap();

        match file_info.file_type() {
            gio::FileType::Directory => self.create_new_subdirectory_model(file_info),
            gio::FileType::Regular => {
                self.update_public_items(list_model, base_index, 0, 1);
            }
            _ => g_debug!(
                "LibraryListModel",
                "Enumerated a file that is not a directory or file. Ignoring."
            ),
        }
    }

    /// Called by `new_model_item_enumerated()` if the GFile is a directory.
    ///
    /// Creates a new `GtkDirectoryList` model for the given directory.
    fn create_new_subdirectory_model(&self, file_info: gio::FileInfo) {
        let obj = self.obj();

        let subdirectory_absolute_path: String = format!(
            "{}/{}/{}",
            env::var("HOME").unwrap(), // err handling at library/mod.rs
            DEFAULT_LIBRARY_DIRECTORY,
            file_info.name().to_str().unwrap(),
        );
        g_debug!(
            "LibraryListModel",
            "Enumerated new subdirectory: {}",
            subdirectory_absolute_path
        );

        let new_model = gtk::DirectoryList::new(None, None::<&gio::File>);

        let items_changed_signal_id: glib::SignalHandlerId =
            new_model.connect_items_changed(clone!(@weak self as s, @weak obj as o => move |
            list_model: &gtk::DirectoryList, pos: u32, removed: u32, added: u32| {
                // FIXME: do not append directory items to visible items indices.
                s.update_public_items(list_model, pos, removed, added);
            }));

        let loading_signal_id: glib::SignalHandlerId =
            new_model.connect_loading_notify(clone!(@weak obj as o => move |
            list_model: &gtk::DirectoryList| {
                o.imp().register_subdir_loading_notify(list_model);
            }));

        new_model.set_io_priority(DIRECTORY_MODEL_PRIORITY);
        new_model.set_file(Some(&gio::File::for_path(subdirectory_absolute_path)));

        let mut subdirs: RefMut<'_, Vec<SubdirectoryListModel>> = self.subdir_models.borrow_mut();

        subdirs.push(SubdirectoryListModel {
            model: new_model,
            _items_changed_callback: items_changed_signal_id,
            _loading_callback: loading_signal_id,
        });

        drop(subdirs); // drop to avoid double mutable borrow error at `self.n_items`
    }

    /// Updates the `public_items` vector and emits the `items_changed`
    /// signal for our GListModel gobject subclass instance.
    fn update_public_items(&self, model: &gtk::DirectoryList, pos: u32, removed: u32, added: u32) {
        let obj = self.obj();
        let mut private_index_offset: u32 = 0;
        let mut added_items: Vec<glib::Object> = vec![];

        for _ in 1..removed {
            self.public_items.borrow_mut().remove(pos as usize);
        }

        for _ in 1..added {
            if let Some(object) = model.item(pos) {
                added_items.push(object);
            } else {
                // FIXME
                g_debug!("LibraryListModel", "critical error FIXME");
            }
        }

        let mut public_vec: RefMut<'_, Vec<glib::Object>> = self.public_items.borrow_mut();

        if let Some(_) = self.root_model.item(pos) {
            for added_item in added_items.iter() {
                public_vec.insert(pos.try_into().unwrap(), added_item.clone());
            }
            drop(public_vec);
            obj.items_changed(pos, removed, added);
        } else {
            private_index_offset += self.root_model.n_items();

            let subdirs: RefMut<'_, Vec<SubdirectoryListModel>> = self.subdir_models.borrow_mut();

            for subdir in subdirs.iter() {
                if subdir.model.file().unwrap() == model.file().unwrap() {
                    for added_item in added_items.iter() {
                        public_vec.insert(
                            TryInto::<usize>::try_into(private_index_offset + pos).unwrap(),
                            added_item.clone(),
                        );
                    }
                    drop(public_vec);
                    obj.items_changed(private_index_offset + pos, removed, added);
                    return;
                }
                private_index_offset += subdir.model.n_items();
            }
            // FIXME
            g_debug!("LibraryListModel", "critical error FIXME");
        }
    }

    /// Cleans up all `GtkDirectoryList` instances and their signal handlers.
    /// This is usually called by `GObject::dispose()` or `Self::set_file`.
    pub(super) fn cleanup_model(&self) {
        if self.root_model.file().is_some() {
            todo!()
        }
    }
}
