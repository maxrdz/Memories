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

use adw::subclass::prelude::*;
use gtk::glib;

#[derive(Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/library/details/details.ui")]
pub struct AlbumsDetails {
    #[template_child]
    pub(super) folder: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) format: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) dimensions: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) size: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) created: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) modified: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) timestamp: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) aperture: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) exposure: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) iso: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) focal_length: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub(super) make_model: TemplateChild<adw::ActionRow>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsDetails {
    const NAME: &'static str = "AlbumsDetails";
    type Type = super::AlbumsDetails;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AlbumsDetails {}
impl WidgetImpl for AlbumsDetails {}
impl BinImpl for AlbumsDetails {}
