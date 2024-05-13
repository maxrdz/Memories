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

use adw::glib;
use adw::gtk;
use adw::subclass::prelude::*;
use libadwaita as adw;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/albums/albums-view.ui")]
pub struct AlbumsView {
    #[template_child]
    new_album_button: TemplateChild<gtk::Button>,
    #[template_child]
    album_carousel_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    album_carousel: TemplateChild<adw::Carousel>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsView {
    const NAME: &'static str = "AlbumsView";
    type Type = super::AlbumsView;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AlbumsView {}
impl WidgetImpl for AlbumsView {}
impl BinImpl for AlbumsView {}
