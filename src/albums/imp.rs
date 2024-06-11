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

use adw::glib;
use adw::subclass::prelude::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Memories/albums/albums.ui")]
pub struct MrsAlbumsView {
    #[template_child]
    albums_box: TemplateChild<gtk::Box>,
    #[template_child]
    new_album_button: TemplateChild<gtk::Button>,
    #[template_child]
    album_carousel_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    album_carousel: TemplateChild<adw::Carousel>,
    #[template_child]
    create_album: TemplateChild<gtk::Button>,
    #[template_child]
    videos_button: TemplateChild<gtk::Button>,
    #[template_child]
    screenshots_button: TemplateChild<gtk::Button>,
    #[template_child]
    gif_button: TemplateChild<gtk::Button>,
    #[template_child]
    svg_button: TemplateChild<gtk::Button>,
    #[template_child]
    trash_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for MrsAlbumsView {
    const NAME: &'static str = "MrsAlbumsView";
    type Type = super::MrsAlbumsView;
    type ParentType = adw::BreakpointBin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MrsAlbumsView {}
impl WidgetImpl for MrsAlbumsView {}
impl BinImpl for MrsAlbumsView {}
impl BreakpointBinImpl for MrsAlbumsView {}
