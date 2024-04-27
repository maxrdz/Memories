// library_view/library_list_model/imp.rs
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

//! Data model implementation of the LibraryListModel class.

use crate::globals::DEFAULT_LIBRARY_DIRECTORY;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};
use glib_macros::clone;
use libadwaita as adw;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::env;

#[derive(Debug)]
struct SubdirectoryListModel {
    model: gtk::DirectoryList,
    items_changed_callback: glib::SignalHandlerId,
    loading_callback: glib::SignalHandlerId,
}

/// Custom implementation of GListModel that uses GTK's
/// `GtkDirectoryList` models under the hood to recursively
/// enumerate files under a certain directory path.
#[derive(glib::Properties, Debug)]
#[properties(wrapper_type = super::LibraryListModel)]
pub struct LibraryListModel {
    root_items_changed_signal: RefCell<Option<glib::SignalHandlerId>>,
    hidden_items: RefCell<Vec<u32>>,
    pub(super) root_model: gtk::DirectoryList,
    subdir_models: RefCell<Vec<SubdirectoryListModel>>,
    #[property(get, set)]
    models_loaded: Cell<bool>,
    loading_notifies: Cell<u32>,
}

impl Default for LibraryListModel {
    fn default() -> Self {
        Self {
            root_items_changed_signal: RefCell::new(None),
            hidden_items: RefCell::new(vec![]),
            root_model: gtk::DirectoryList::new(None, None::<&gio::File>),
            subdir_models: RefCell::new(vec![]),
            models_loaded: Cell::new(true),
            loading_notifies: Cell::new(0_u32),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for LibraryListModel {
    const NAME: &'static str = "AlbumLibraryListModel";
    type Type = super::LibraryListModel;
    type Interfaces = (gio::ListModel,);
}

#[glib::derived_properties]
impl ObjectImpl for LibraryListModel {
    fn constructed(&self) {
        let obj = self.obj();

        // Clone the `hidden_items` vec for the callback to use it.
        // It doesn't modify `hidden_items`, so ideally we would use
        // an immutable borrow, but the borrow checker doesn't like it.
        let a = self.hidden_items.borrow();
        let hidden: Vec<u32> = a.clone();
        drop(a);
        // Connect the root model's `items_changed` signal with our model
        // so that the view that owns the `LibraryListModel` can be notified.
        let signal_handler_id: glib::SignalHandlerId =
            self.root_model
                .connect_items_changed(clone!(@weak obj as o, @strong hidden as h => move |
                model: &gtk::DirectoryList, pos: u32, removed: u32, added: u32| {
                    if added != 0 {
                        let base_index: u32 = pos - removed;
                        o.imp().new_model_item_enumerated(model, base_index, Some(added));
                    }
                    // Artificially pull backward index per hidden item below given index.
                    // This keeps the illusion of our 'hidden' items.
                    let mut adjusted_pos: u32 = pos;
                    for hidden in h.iter() {
                        if *hidden <= pos {
                            adjusted_pos += 1;
                        }
                    }
                    o.items_changed(adjusted_pos, removed, added);
                }));
        self.root_model
            .connect_loading_notify(clone!(@weak self as s => move |
                dl: &gtk::DirectoryList| {
                s.register_subdir_loading_notify(dl);
            }));

        self.root_items_changed_signal.replace(Some(signal_handler_id));
    }
}

/// Basically just redirect all GListModel interface calls
/// to our underlying GtkDirectoryList objects. Specifically,
/// if the root directory list model is empty, reroute data
/// from our subdirectory models to make this object look
/// like it has a continuous list of items from all subdirs.
impl ListModelImpl for LibraryListModel {
    fn item(&self, position: u32) -> Option<glib::Object> {
        let mut pos: u32 = position;
        {
            let b_hidden_items: Ref<'_, Vec<u32>> = self.hidden_items.borrow();
            // Artificially push forward index per hidden item below given index.
            // This keeps the illusion of our 'hidden' items.
            for hidden in b_hidden_items.iter() {
                if *hidden <= position {
                    pos += 1;
                }
            }
        }
        if let Some(res) = self.root_model.item(pos) {
            Some(res)
        } else {
            let sdm_mut: RefMut<'_, Vec<SubdirectoryListModel>> = self.subdir_models.borrow_mut();
            if !sdm_mut.is_empty() {
                assert!(
                    pos >= self.root_model.n_items(),
                    "Given position u32 value is less than the size of the root GtkDirectoryList!",
                );
                let mut adjusted_position: u32 = pos - self.root_model.n_items();
                for subdir_model in sdm_mut.iter() {
                    if let Some(res) = subdir_model.model.item(adjusted_position) {
                        return Some(res);
                    }
                    if adjusted_position >= subdir_model.model.n_items() {
                        adjusted_position -= subdir_model.model.n_items();
                        continue;
                    }
                    return None;
                }
                return None;
            }
            None
        }
    }

    fn item_type(&self) -> glib::Type {
        self.root_model.item_type()
    }

    fn n_items(&self) -> u32 {
        let hidden_count: u32 = self.hidden_items.borrow().len() as u32;
        let mut subdir_item_count: u32 = 0;
        for subdir_model in self.subdir_models.borrow().iter() {
            subdir_item_count += subdir_model.model.n_items();
        }
        self.root_model.n_items() + subdir_item_count - hidden_count
    }
}

impl LibraryListModel {
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
            gio::FileType::Directory | gio::FileType::SymbolicLink => {
                self.create_new_subdirectory_model(file_info, base_index)
            }
            _ => (), // no other file types currently handled
        }
    }

    /// Called by `new_model_item_enumerated()` if the GFile is a directory.
    ///
    /// Creates a new `GtkDirectoryList` model for the given directory,
    /// and adds the original list item from the root model to the list
    /// of 'hidden' items, since the directory itself is not a picture/video.
    fn create_new_subdirectory_model(&self, file_info: gio::FileInfo, index: u32) {
        let obj = self.obj();
        let subdirectory_absolute_path: String = format!(
            "{}/{}/{}",
            env::var("HOME").unwrap(), // err handling at library_view/mod.rs
            DEFAULT_LIBRARY_DIRECTORY,
            file_info.name().to_str().unwrap(),
        );
        debug!("Enumerated new subdirectory: {}", subdirectory_absolute_path);

        let new_directory_list = gtk::DirectoryList::new(None, None::<&gio::File>);

        let items_changed_signal_id: glib::SignalHandlerId =
            new_directory_list.connect_items_changed(clone!(@weak obj as o => move |
            _: &gtk::DirectoryList, pos: u32, removed: u32, added: u32| {
                o.items_changed(pos, removed, added);
            }));

        let loading_signal_id: glib::SignalHandlerId =
            new_directory_list.connect_loading_notify(clone!(@weak obj as o => move |
            list_model: &gtk::DirectoryList| {
                o.imp().register_subdir_loading_notify(list_model);
            }));

        new_directory_list.set_file(Some(&gio::File::for_path(subdirectory_absolute_path)));

        let mut sdm_mut: RefMut<'_, Vec<SubdirectoryListModel>> = self.subdir_models.borrow_mut();

        sdm_mut.push(SubdirectoryListModel {
            model: new_directory_list,
            items_changed_callback: items_changed_signal_id,
            loading_callback: loading_signal_id,
        });

        drop(sdm_mut); // drop to avoid double mutable borrow error at `self.n_items`

        // Since this item represents a directory, we will hide it.
        self.hidden_items.borrow_mut().push(index);
        //self.obj().items_changed(index, 1, 0); FIXME
    }
}
