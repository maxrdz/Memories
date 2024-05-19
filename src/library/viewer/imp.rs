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

use crate::library::details::AlbumsDetails;
use crate::preferences::theme_selector::AlbumsThemeSelector;
use adw::glib;
use adw::gtk;
use adw::subclass::prelude::*;
use libadwaita as adw;

#[derive(Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/library/viewer/viewer-page.ui")]
pub struct AlbumsViewer {
    #[template_child]
    pub header_bar: TemplateChild<adw::HeaderBar>,
    #[template_child]
    pub main_menu_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub primary_menu: TemplateChild<gtk::PopoverMenu>,
    #[template_child]
    pub details_button: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub split_view: TemplateChild<adw::OverlaySplitView>,
    #[template_child]
    pub details_widget: TemplateChild<AlbumsDetails>,
    #[template_child]
    pub viewer_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    pub image_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub video_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub viewer_picture: TemplateChild<gtk::Picture>,
    #[template_child]
    pub viewer_video: TemplateChild<gtk::Video>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsViewer {
    const NAME: &'static str = "AlbumsViewer";
    type Type = super::AlbumsViewer;
    type ParentType = adw::BreakpointBin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AlbumsViewer {
    fn constructed(&self) {
        self.parent_constructed();

        // We have to add the theme selector widget as a child of our
        // GtkPopoverMenu widget manually here, because the UI XML method
        // does not work (for some reason..) GTK and its docs are a pain.
        let new_theme_selector = AlbumsThemeSelector::new();
        self.primary_menu.add_child(&new_theme_selector, "theme-selector");
    }
}
impl WidgetImpl for AlbumsViewer {}
impl BinImpl for AlbumsViewer {}
impl BreakpointBinImpl for AlbumsViewer {}
