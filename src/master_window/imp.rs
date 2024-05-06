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

use crate::library_view::LibraryView;
use crate::preferences_view::theme_selector::ThemeSelector;
use crate::preferences_view::PreferencesView;
use adw::gtk;
use adw::subclass::prelude::*;
use gtk::glib;
use libadwaita as adw;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/master_window/master-window.ui")]
pub struct MasterWindow {
    #[template_child]
    pub header_bar: TemplateChild<adw::HeaderBar>,
    #[template_child]
    pub desktop_view_switcher: TemplateChild<adw::ViewSwitcher>,
    #[template_child]
    pub mobile_switcher_bar: TemplateChild<adw::ViewSwitcherBar>,
    #[template_child]
    pub primary_menu: TemplateChild<gtk::PopoverMenu>,
    #[template_child]
    pub master_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    pub library_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub albums_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub search_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub preferences_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub library_view: TemplateChild<LibraryView>,
    #[template_child]
    pub preferences_view: TemplateChild<PreferencesView>,
}

#[glib::object_subclass]
impl ObjectSubclass for MasterWindow {
    const NAME: &'static str = "AlbumsMasterWindow";
    type Type = super::MasterWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MasterWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // We have to add the theme selector widget as a child of our
        // GtkPopoverMenu widget manually here, because the UI XML method
        // does not work (for some reason..) GTK and its docs are a pain.
        let new_theme_selector = ThemeSelector::new();
        self.primary_menu.add_child(&new_theme_selector, "theme-selector");

        obj.setup_gactions();
        // This callback wont be triggered on start up by itself, so we
        // want to check the very first visible child in the master stack.
        obj.master_stack_child_visible();
    }
}
impl WidgetImpl for MasterWindow {}
impl WindowImpl for MasterWindow {}
impl ApplicationWindowImpl for MasterWindow {}
impl AdwApplicationWindowImpl for MasterWindow {}
