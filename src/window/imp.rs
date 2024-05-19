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

use crate::albums::AlbumsView;
use crate::application::AlbumsApplication;
use crate::library::library_list_model::AlbumsLibraryListModel;
use crate::library::AlbumsLibraryView;
use crate::preferences::theme_selector::AlbumsThemeSelector;
use crate::preferences::AlbumsPreferencesView;
use adw::gtk;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib_macros::clone;
use gtk::glib;
use libadwaita as adw;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/window/window.ui")]
pub struct AlbumsApplicationWindow {
    #[template_child]
    pub window_navigation: TemplateChild<adw::NavigationView>,
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
    pub preferences_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub albums_view: TemplateChild<AlbumsView>,
    #[template_child]
    pub library_view: TemplateChild<AlbumsLibraryView>,
    #[template_child]
    pub preferences_view: TemplateChild<AlbumsPreferencesView>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsApplicationWindow {
    const NAME: &'static str = "AlbumsApplicationWindow";
    type Type = super::AlbumsApplicationWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AlbumsApplicationWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // We have to add the theme selector widget as a child of our
        // GtkPopoverMenu widget manually here, because the UI XML method
        // does not work (for some reason..) GTK and its docs are a pain.
        let new_theme_selector = AlbumsThemeSelector::new();
        self.primary_menu.add_child(&new_theme_selector, "theme-selector");

        obj.setup_gactions();

        obj.connect_show(move |window: &super::AlbumsApplicationWindow| {
            // AlbumsLibraryListModel instance MUST be initialized after
            // the application window, but before the library view.
            window
                .app()
                .unwrap()
                .set_library_list_model(AlbumsLibraryListModel::default());

            // This callback wont be triggered on start up by itself, so we
            // want to check the very first visible child in the master view stack.
            window.master_stack_child_visible();
        });

        // Persist application window state (width, height, maximized) with GSettings
        let gsettings: gio::Settings = AlbumsApplication::default().gsettings();

        obj.set_maximized(gsettings.boolean("maximized"));
        obj.set_default_width(gsettings.int("window-width"));
        obj.set_default_height(gsettings.int("window-height"));

        obj.connect_maximized_notify(
            clone!(@weak gsettings as gs => move |win: &super::AlbumsApplicationWindow| {
                gs.set_boolean("maximized", win.is_maximized()).unwrap();
            }),
        );

        obj.connect_close_request(move |win: &super::AlbumsApplicationWindow| {
            if !win.is_maximized() {
                gsettings.set_int("window-width", win.width()).unwrap();
                gsettings.set_int("window-height", win.height()).unwrap();
            }
            glib::Propagation::Proceed
        });
    }
}

impl WidgetImpl for AlbumsApplicationWindow {}
impl WindowImpl for AlbumsApplicationWindow {}
impl ApplicationWindowImpl for AlbumsApplicationWindow {}
impl AdwApplicationWindowImpl for AlbumsApplicationWindow {}
