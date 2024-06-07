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

use crate::application::AlbumsApplication;
use crate::globals::DIRECTORY_MODEL_PRIORITY;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{clone, g_debug, g_error};
use gtk::{gio, glib};
use std::cell::{Cell, RefCell, RefMut};
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
pub(super) struct RootListModel {
    pub(super) model: gtk::DirectoryList,
    subdir_models: RefCell<Vec<SubdirectoryListModel>>,
    public_items: Cell<u32>,
    items_changed_callback: RefCell<Option<glib::SignalHandlerId>>,
}

#[derive(Debug)]
struct SubdirectoryListModel {
    model: gtk::DirectoryList,
    public_items: u32,
    _items_changed_callback: glib::SignalHandlerId,
    _loading_callback: glib::SignalHandlerId,
}

/// Custom implementation of GListModel that uses
/// `GtkDirectoryList` models under the hood to recursively
/// enumerate files under certain root directory paths.
#[derive(glib::Properties, Debug)]
#[properties(wrapper_type = super::AlbumsLibraryListModel)]
pub struct AlbumsLibraryListModel {
    #[property(get, set)]
    subdirectories: RefCell<glib::StrV>,
    #[property(get, set)]
    models_loaded: Cell<bool>,
    #[property(get)]
    refresh_widget_rows: Cell<bool>,

    pub(super) root_models: RefCell<Vec<Rc<RootListModel>>>,
    loading_notifies: Cell<u32>,
    public_items: RefCell<Vec<glib::Object>>,
}

impl Default for AlbumsLibraryListModel {
    fn default() -> Self {
        Self {
            subdirectories: RefCell::new({
                let gsettings: gio::Settings = AlbumsApplication::default().gsettings();
                gsettings.strv("library-collection-paths")
            }),
            models_loaded: Cell::new(false),
            refresh_widget_rows: Cell::new(false),
            root_models: RefCell::new(vec![]),
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
        let gsettings: gio::Settings = AlbumsApplication::default().gsettings();

        // Bind our `subdirectories` property with the gschema key.
        gsettings
            .bind("library-collection-paths", &obj.clone(), "subdirectories")
            .build();

        obj.connect_subdirectories_notify(
            clone!(@weak self as s, @weak obj => move |model: &super::AlbumsLibraryListModel| {
                g_debug!("LibraryListModel", "notify::subdirectories");

                // Signal to refresh the 'library collection' preference group, which
                // displays the current subdirectories configured for the library list model.
                obj.notify_refresh_widget_rows();

                let subdirs: glib::StrV = model.subdirectories();

                for folder in &subdirs {
                    let folder_path: String = folder.to_string();
                    g_debug!("LibraryListModel", "Creating root list model for {}", folder_path);

                    let gfile: gio::File = gio::File::for_path(folder_path);

                    let new_model: RootListModel = RootListModel {
                        model: gtk::DirectoryList::new(None, Some(&gfile)),
                        subdir_models: RefCell::new(vec![]),
                        public_items: Cell::new(0_u32),
                        items_changed_callback: RefCell::new(None),
                    };

                    // Connect the root model's `items_changed` signal with our model
                    // so that the view that owns the `LibraryListModel` can be notified.
                    let signal_handler_id: glib::SignalHandlerId =
                        new_model
                            .model
                            .connect_items_changed(clone!(@weak obj as o => move |
                            model: &gtk::DirectoryList, pos: u32, removed: u32, added: u32| {
                                if added != 0 {
                                    let base_index: u32 = pos - removed;
                                    o.imp().new_root_model_item_enumerated(model, base_index, Some(added));
                                }
                            }));
                    new_model
                        .model
                        .connect_loading_notify(clone!(@weak s => move |
                            dl: &gtk::DirectoryList| {
                            s.register_model_loading_notify(dl);
                        }));

                    new_model.items_changed_callback.replace(Some(signal_handler_id));
                    new_model.model.set_io_priority(DIRECTORY_MODEL_PRIORITY);

                    s.root_models.borrow_mut().push(Rc::new(new_model));
                }
            }),
        );
    }
}

impl ListModelImpl for AlbumsLibraryListModel {
    fn item(&self, position: u32) -> Option<glib::Object> {
        self.public_items
            .borrow()
            .get(TryInto::<usize>::try_into(position).unwrap())
            .cloned()
    }

    fn item_type(&self) -> glib::Type {
        // Does not matter which root model we grab, they're all `GtkDirectoryList`
        self.root_models.borrow().first().unwrap().model.item_type()
    }

    fn n_items(&self) -> u32 {
        self.public_items.borrow().len().try_into().unwrap()
    }
}

impl AlbumsLibraryListModel {
    /// Returns a root model by comparing all root
    /// models with the given `GtkDirectoryList` instance.
    fn lookup_root_model(&self, directory_list: &gtk::DirectoryList) -> Option<Rc<RootListModel>> {
        for root_model in self.root_models.borrow().iter() {
            if root_model.model == *directory_list {
                return Some(root_model.clone());
            }
        }
        None
    }

    /// Returns the total number of `GtkDirectoryList` models
    /// used within this `GListModel` implementation.
    fn directory_list_count(&self) -> u32 {
        let mut count: u32 = 0;
        for root_model in self.root_models.borrow().iter() {
            count += TryInto::<u32>::try_into(root_model.subdir_models.borrow().len()).unwrap();
            count += 1; // also take this root model into account
        }
        count
    }

    /// Called by a `GtkDirectoryList` model upon its 'loading_notify' signal.
    pub(super) fn register_model_loading_notify(&self, model: &gtk::DirectoryList) {
        let notifies: u32 = self.loading_notifies.get();

        if !model.is_loading() {
            let updated_notifies: u32 = notifies + 1;
            self.loading_notifies.set(updated_notifies);

            if updated_notifies == self.directory_list_count() {
                self.obj().set_models_loaded(true);
            }
        }
    }

    /// Called by the handler for a root model's `items_changed` signal event.
    pub(super) fn new_root_model_item_enumerated(
        &self,
        list_model: &gtk::DirectoryList,
        base_index: u32,
        items_added: Option<u32>,
    ) {
        let root_model: Rc<RootListModel> = self.lookup_root_model(list_model).unwrap();

        if let Some(added) = items_added {
            // 'recursively' call our function per item added.
            for i in 0..added {
                let adjusted_index: u32 = base_index + i;
                self.new_root_model_item_enumerated(list_model, adjusted_index, None);
            }
            return;
        }
        let item_query: Option<glib::Object> = list_model.item(base_index);

        assert!(
            item_query.is_some(),
            "New item found in a root list model, but item query returned None.",
        );
        let file_info: gio::FileInfo = item_query.unwrap().downcast().unwrap();

        match file_info.file_type() {
            gio::FileType::Directory => self.create_new_subdirectory_model(root_model, file_info),
            gio::FileType::Regular => {
                self.update_public_items(root_model, list_model, base_index, 0, 1);
            }
            _ => g_debug!(
                "LibraryListModel",
                "Enumerated a file that is not a directory or file. Ignoring."
            ),
        }
    }

    /// Called by `new_root_model_item_enumerated()` if the GFile is a directory.
    ///
    /// Creates a new `GtkDirectoryList` model for the given parent directory model.
    fn create_new_subdirectory_model(
        &self,
        parent_list_model: Rc<RootListModel>,
        item_file_info: gio::FileInfo,
    ) {
        let obj = self.obj();

        // Extract the parent directory absolute path from its `GFile` object.
        let parent_file: gio::File = parent_list_model.model.file().unwrap();
        let file_path: PathBuf = parent_file.path().unwrap();
        let parent_dir_path: String = file_path.to_string_lossy().to_string();

        let subdirectory_absolute_path: String =
            format!("{}/{}", parent_dir_path, item_file_info.name().to_str().unwrap());

        g_debug!(
            "LibraryListModel",
            "Enumerated new subdirectory: {}",
            subdirectory_absolute_path
        );

        let new_model = gtk::DirectoryList::new(None, None::<&gio::File>);

        let items_changed_signal_id: glib::SignalHandlerId =
            new_model.connect_items_changed(clone!(@weak self as s, @weak parent_list_model => move |
            list_model: &gtk::DirectoryList, pos: u32, removed: u32, added: u32| {
                // FIXME: do not append directory items to public items.
                s.update_public_items(parent_list_model, list_model, pos, removed, added);
            }));

        let loading_signal_id: glib::SignalHandlerId =
            new_model.connect_loading_notify(clone!(@weak obj as o => move |
            list_model: &gtk::DirectoryList| {
                o.imp().register_model_loading_notify(list_model);
            }));

        new_model.set_io_priority(DIRECTORY_MODEL_PRIORITY);
        new_model.set_file(Some(&gio::File::for_path(subdirectory_absolute_path)));

        let mut subdirs: RefMut<'_, Vec<SubdirectoryListModel>> =
            parent_list_model.subdir_models.borrow_mut();

        subdirs.push(SubdirectoryListModel {
            model: new_model,
            public_items: 0_u32,
            _items_changed_callback: items_changed_signal_id,
            _loading_callback: loading_signal_id,
        });

        drop(subdirs); // drop to avoid double mutable borrow error at `self.n_items`
    }

    /// Updates the `public_items` vector and emits the `items_changed`
    /// signal for our GListModel gobject subclass instance.
    fn update_public_items(
        &self,
        parent_model: Rc<RootListModel>,
        model: &gtk::DirectoryList,
        pos: u32,
        removed: u32,
        added: u32,
    ) {
        let obj = self.obj();

        let model_file: gio::File = model.file().unwrap();
        let mut private_index_offset: u32 = 0;
        let mut added_items: Vec<glib::Object> = vec![];

        g_debug!(
            "LibraryListModel",
            "update_public_items(): {:?} {} {} {}",
            &model_file,
            pos,
            removed,
            added
        );

        for i in 0..removed {
            self.public_items
                .borrow_mut()
                .remove((pos + i).try_into().unwrap());
        }

        for i in 0..added {
            if let Some(object) = model.item(pos + i) {
                added_items.push(object);
            } else {
                g_error!(
                    "LibraryListModel",
                    "update_public_items(): model.item(pos) returned None."
                );
            }
        }

        let mut public_vec: RefMut<'_, Vec<glib::Object>> = self.public_items.borrow_mut();

        // First, check if the `model` given is the root `GtkDirectoryList` model.
        if parent_model.model.file().unwrap() == model_file {
            for added_item in added_items.iter() {
                public_vec.insert(pos.try_into().unwrap(), added_item.clone());
            }
            drop(public_vec);

            // Update the `RootListModel`s `public_items` count.
            let previous_public_count: u32 = parent_model.public_items.get();
            parent_model
                .public_items
                .swap(&Cell::new(previous_public_count + added - removed));

            obj.items_changed(pos, removed, added);
        } else {
            private_index_offset += parent_model.public_items.get();

            let mut subdirs: RefMut<'_, Vec<SubdirectoryListModel>> = parent_model.subdir_models.borrow_mut();

            for subdir in subdirs.iter_mut() {
                if subdir.model.file().unwrap() == model_file {
                    for added_item in added_items.iter() {
                        public_vec.insert(
                            TryInto::<usize>::try_into(private_index_offset + pos).unwrap(),
                            added_item.clone(),
                        );
                    }
                    drop(public_vec);

                    subdir.public_items += added - removed;

                    obj.items_changed(private_index_offset + pos, removed, added);
                    return;
                }
                private_index_offset += subdir.public_items;
            }
            g_error!(
                "LibraryListModel",
                "Model given doesn't exist. Should not be possible."
            );
        }
    }
}
