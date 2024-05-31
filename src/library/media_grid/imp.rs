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

use crate::application::AlbumsApplication;
use crate::globals::{DEFAULT_GRID_WIDGET_HEIGHT, FFMPEG_CONCURRENT_PROCESSES};
use adw::glib;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use async_semaphore::Semaphore;
use libadwaita as adw;
use std::cell::Cell;
use std::sync::Arc;

#[derive(Debug, glib::Properties, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/library/media_grid/media_grid.ui")]
#[properties(wrapper_type = super::AlbumsMediaGridView)]
pub struct AlbumsMediaGridView {
    pub(super) subprocess_semaphore: Arc<Semaphore>,
    #[property(get, set)]
    hardware_accel: Cell<bool>,
    #[property(get, set)]
    grid_widget_height: Cell<i32>,
    #[property(get, set)]
    grid_desktop_zoom: Cell<bool>,

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
    pub zoom_in: TemplateChild<gtk::Button>,
    #[template_child]
    pub zoom_out: TemplateChild<gtk::Button>,
}

impl Default for AlbumsMediaGridView {
    fn default() -> Self {
        Self {
            subprocess_semaphore: Arc::new(Semaphore::new(FFMPEG_CONCURRENT_PROCESSES)),
            hardware_accel: Cell::new({
                let gsettings: gio::Settings = AlbumsApplication::default().gsettings();
                gsettings.boolean("hardware-acceleration")
            }),
            grid_widget_height: Cell::new(DEFAULT_GRID_WIDGET_HEIGHT),
            grid_desktop_zoom: Cell::new(false),
            toast_overlay: TemplateChild::default(),
            overlay_labels_box: TemplateChild::default(),
            time_period_label: TemplateChild::default(),
            total_items_label: TemplateChild::default(),
            photo_grid_controls: TemplateChild::default(),
            photo_grid_view: TemplateChild::default(),
            zoom_in: TemplateChild::default(),
            zoom_out: TemplateChild::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsMediaGridView {
    const NAME: &'static str = "AlbumsMediaGridView";
    type ParentType = adw::BreakpointBin;
    type Type = super::AlbumsMediaGridView;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for AlbumsMediaGridView {
    fn constructed(&self) {
        let obj = self.obj();

        obj.connect_grid_desktop_zoom_notify(move |media_grid: &super::AlbumsMediaGridView| {
            // `grid_desktop_zoom` is modified only when the `AdwBreakpoint` is triggered.
            // The default zoom settings for the grid view are always at the minimum zoom
            // by default in the UI files, so we reset the grid controls to min zoom below.
            media_grid.imp().zoom_in.set_sensitive(true);
            media_grid.imp().zoom_out.set_sensitive(false);
        });

        // Bind any application preferences to our application's GSettings.
        let gsettings: gio::Settings = AlbumsApplication::default().gsettings();

        gsettings
            .bind("hardware-acceleration", &self.obj().clone(), "hardware-accel")
            .build();
    }
}

impl WidgetImpl for AlbumsMediaGridView {}
impl BinImpl for AlbumsMediaGridView {}
impl BreakpointBinImpl for AlbumsMediaGridView {}
