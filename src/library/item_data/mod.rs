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

use adw::glib;
use adw::prelude::*;
use libadwaita as adw;

glib::wrapper! {
    pub struct AlbumsItemData(ObjectSubclass<imp::AlbumsItemData>)
        @extends gtk::Widget, adw::Bin;
}

impl AlbumsItemData {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> AlbumsItemDataBuilder {
        AlbumsItemDataBuilder::new()
    }
}

impl Default for AlbumsItemData {
    fn default() -> Self {
        Self::new()
    }
}

/// A [builder-pattern] type to construct `LibraryGridCellData` objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used."]
pub struct AlbumsItemDataBuilder {
    builder: glib::object::ObjectBuilder<'static, AlbumsItemData>,
}

impl AlbumsItemDataBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    /// Build the `AlbumsItemData` object.
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects."]
    pub fn build(self) -> AlbumsItemData {
        self.builder.build()
    }
}
