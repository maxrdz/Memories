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
#[template(resource = "/com/maxrdz/Albums/library/media_grid/media_grid.ui")]
pub struct AlbumsMediaGridView {
    #[template_child]
    pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    #[template_child]
    pub overlay_labels_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub time_period_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub total_items_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub photo_grid_controls: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub photo_grid_view: TemplateChild<gtk::GridView>,
    #[template_child]
    pub grid_controls_menu: TemplateChild<gio::Menu>,
    #[template_child]
    pub grid_controls_menu_max_zoom: TemplateChild<gio::Menu>,
    #[template_child]
    pub grid_controls_menu_min_zoom: TemplateChild<gio::Menu>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsMediaGridView {
    const NAME: &'static str = "AlbumsMediaGridView";
    type ParentType = adw::BreakpointBin;
    type Type = super::AlbumsMediaGridView;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AlbumsMediaGridView {}
impl WidgetImpl for AlbumsMediaGridView {}
impl BinImpl for AlbumsMediaGridView {}
impl BreakpointBinImpl for AlbumsMediaGridView {}
