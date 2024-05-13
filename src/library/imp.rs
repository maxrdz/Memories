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
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use async_semaphore::Semaphore;
use gtk::glib;
use libadwaita as adw;
use std::cell::Cell;
use std::sync::Arc;

#[derive(Debug, glib::Properties, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/library/library-view.ui")]
#[properties(wrapper_type = super::LibraryView)]
pub struct LibraryView {
    pub(super) subprocess_semaphore: Arc<Semaphore>,
    #[property(get, set)]
    hardware_accel: Cell<bool>,
    #[property(get, set)]
    grid_widget_height: Cell<i32>,
    #[property(get, set)]
    grid_desktop_zoom: Cell<bool>,
    #[template_child]
    pub(super) library_view_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    pub(super) spinner_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub(super) spinner: TemplateChild<gtk::Spinner>,
    #[template_child]
    pub(super) error_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub(super) error_status_widget: TemplateChild<adw::StatusPage>,
    #[template_child]
    pub(super) gallery_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub(super) gallery_toast_overlay: TemplateChild<adw::ToastOverlay>,
    #[template_child]
    pub(super) overlay_labels_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub(super) time_period_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub(super) total_items_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub photo_grid_view: TemplateChild<gtk::GridView>,
    #[template_child]
    pub photo_grid_controls: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub grid_controls_menu: TemplateChild<gio::MenuModel>,
    #[template_child]
    pub grid_controls_menu_max_zoom: TemplateChild<gio::MenuModel>,
    #[template_child]
    pub grid_controls_menu_min_zoom: TemplateChild<gio::MenuModel>,
}

impl Default for LibraryView {
    fn default() -> Self {
        Self {
            subprocess_semaphore: Arc::new(Semaphore::new(FFMPEG_CONCURRENT_PROCESSES)),
            hardware_accel: Cell::new({
                let gsettings: gio::Settings = AlbumsApplication::default().gsettings();
                gsettings.boolean("hardware-acceleration")
            }),
            grid_widget_height: Cell::new(DEFAULT_GRID_WIDGET_HEIGHT),
            grid_desktop_zoom: Cell::new(false),
            library_view_stack: TemplateChild::default(),
            spinner_page: TemplateChild::default(),
            spinner: TemplateChild::default(),
            error_page: TemplateChild::default(),
            error_status_widget: TemplateChild::default(),
            gallery_page: TemplateChild::default(),
            gallery_toast_overlay: TemplateChild::default(),
            overlay_labels_box: TemplateChild::default(),
            time_period_label: TemplateChild::default(),
            total_items_label: TemplateChild::default(),
            photo_grid_view: TemplateChild::default(),
            photo_grid_controls: TemplateChild::default(),
            grid_controls_menu: TemplateChild::default(),
            grid_controls_menu_max_zoom: TemplateChild::default(),
            grid_controls_menu_min_zoom: TemplateChild::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for LibraryView {
    const NAME: &'static str = "AlbumsLibraryView";
    type Type = super::LibraryView;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for LibraryView {
    fn constructed(&self) {
        self.obj()
            .connect_grid_desktop_zoom_notify(move |view: &super::LibraryView| {
                let library_view_imp = view.imp();
                // `grid_desktop_zoom` is modified only when the `AdwBreakpoint` is triggered.
                // The default zoom settings for the grid view are always at the minimum zoom
                // by default in the UI files, so we reset the grid controls to min zoom below.
                library_view_imp
                    .photo_grid_controls
                    .set_menu_model(Some(&library_view_imp.grid_controls_menu_min_zoom.clone()));
            });
        // Bind any application preferences to our application's GSettings.
        let gsettings: gio::Settings = AlbumsApplication::default().gsettings();
        gsettings
            .bind("hardware-acceleration", &self.obj().clone(), "hardware-accel")
            .build();
    }
}

impl WidgetImpl for LibraryView {}
impl BinImpl for LibraryView {}
