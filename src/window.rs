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

use crate::application::MemoriesApplication;
// We manually include only the traits we need to use
// to avoid ambiguity errors when multiple traits share
// the same methods, such as WidgetExt & ActionGroupExt.
use adw::prelude::{
    ActionMapExtManual, AdwDialogExt, ApplicationWindowExt, CastNone, GtkWindowExt, SettingsExt, ToVariant,
    ToggleButtonExt, WidgetExt,
};
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::g_error;
use gtk::{gio, glib};

mod imp {
    use crate::albums::MemoriesAlbumsView;
    use crate::application::MemoriesApplication;
    use crate::config::GRESOURCE_DOMAIN;
    use crate::globals::DEVELOPMENT_BUILD;
    use crate::library::list_model::MemoriesLibraryListModel;
    use crate::library::MemoriesLibraryView;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use glib::clone;
    use gtk::{gio, glib};

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/window.ui")]
    pub struct MemoriesApplicationWindow {
        #[template_child]
        pub window_navigation: TemplateChild<adw::NavigationView>,
        #[template_child]
        header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        desktop_view_switcher: TemplateChild<adw::ViewSwitcher>,
        #[template_child]
        mobile_switcher_bar: TemplateChild<adw::ViewSwitcherBar>,
        #[template_child]
        primary_menu: TemplateChild<gtk::PopoverMenu>,
        #[template_child]
        pub(super) search_bar: TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub(super) search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub(super) master_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub(super) library_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        albums_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        albums_view: TemplateChild<MemoriesAlbumsView>,
        #[template_child]
        pub(super) library_view: TemplateChild<MemoriesLibraryView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoriesApplicationWindow {
        const NAME: &'static str = "MemoriesApplicationWindow";
        type Type = super::MemoriesApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MemoriesApplicationWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            // Add the 'devel' CSS class to our application
            // window if this is a development build of Memories.
            if DEVELOPMENT_BUILD {
                obj.add_css_class("devel");
            }

            // Setup Keyboard Shortcuts window for application window
            let mut builder =
                gtk::Builder::from_resource(&format!("{}/gtk/help-overlay.ui", GRESOURCE_DOMAIN));
            let shortcuts = builder.object("shortcuts").unwrap();
            obj.set_help_overlay(Some(&shortcuts));

            // We have to add the theme selector widget as a child of our
            // GtkPopoverMenu widget manually here, because the UI XML method
            // does not work (for some reason..) GTK and its docs are a pain.
            builder = gtk::Builder::from_resource("/com/maxrdz/Memories/ui/theme-selector.ui");
            let new_theme_selector: adw::Bin = builder.object("theme_selector").unwrap();
            self.primary_menu.add_child(&new_theme_selector, "theme-selector");

            obj.setup_gactions();

            obj.connect_show(move |window: &super::MemoriesApplicationWindow| {
                // MemoriesLibraryListModel instance MUST be initialized after
                // the application window, but before the library view.
                MemoriesLibraryListModel::initialize_new_model(window);

                // This callback wont be triggered on start up by itself, so we
                // want to check the very first visible child in the master view stack.
                window.master_stack_child_visible();
            });

            // Persist application window state (width, height, maximized, etc) with GSettings
            let gsettings: gio::Settings = MemoriesApplication::default().gsettings();

            gsettings
                .bind(
                    "active-view",
                    &obj.imp().master_stack.clone(),
                    "visible-child-name",
                )
                .build();

            obj.set_maximized(gsettings.boolean("maximized"));
            obj.set_default_width(gsettings.int("window-width"));
            obj.set_default_height(gsettings.int("window-height"));
            obj.set_fullscreened(gsettings.boolean("fullscreened"));

            obj.connect_maximized_notify(clone!(
                #[weak(rename_to = settings)]
                gsettings,
                move |win: &super::MemoriesApplicationWindow| {
                    settings.set_boolean("maximized", win.is_maximized()).unwrap();
                }
            ));

            obj.connect_close_request(move |win: &super::MemoriesApplicationWindow| {
                if !win.is_maximized() {
                    gsettings.set_int("window-width", win.width()).unwrap();
                    gsettings.set_int("window-height", win.height()).unwrap();
                }
                glib::Propagation::Proceed
            });
        }
    }

    impl WidgetImpl for MemoriesApplicationWindow {}
    impl WindowImpl for MemoriesApplicationWindow {}
    impl ApplicationWindowImpl for MemoriesApplicationWindow {}
    impl AdwApplicationWindowImpl for MemoriesApplicationWindow {}
}

glib::wrapper! {
    pub struct MemoriesApplicationWindow(ObjectSubclass<imp::MemoriesApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

#[gtk::template_callbacks]
impl MemoriesApplicationWindow {
    pub fn new(application: &MemoriesApplication) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn app(&self) -> Option<MemoriesApplication> {
        self.application().and_downcast()
    }

    fn setup_gactions(&self) {
        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(move |win: &Self, _, _| {
                let builder = gtk::Builder::from_resource("/com/maxrdz/Memories/ui/preferences.ui");
                let dialog: adw::PreferencesDialog = builder.object("preferences_dialog").unwrap();
                dialog.present(Some(win));
            })
            .build();

        let shortcuts_window_action = gio::ActionEntry::builder("show-help-overlay")
            .activate(move |win: &Self, _, _| {
                // GActions are setup after constructor, which guarantees that
                // the help overlay is setup for the window, so we can unwrap().
                win.help_overlay().unwrap().present();
            })
            .build();

        let toggle_fullscreen_action = gio::ActionEntry::builder("fullscreen")
            .state({
                let gsettings: gio::Settings = MemoriesApplication::default().gsettings();
                gsettings.boolean("fullscreened").to_variant()
            })
            .activate(move |win: &Self, action: &gio::SimpleAction, _| {
                let new_state: bool = !win.is_fullscreen();

                win.set_fullscreened(new_state);
                action.set_state(&new_state.to_variant());

                win.app().unwrap().toggle_gschema_key("fullscreened", new_state);
            })
            .build();

        let leave_fullscreen_action = gio::ActionEntry::builder("leave-fullscreen")
            .activate(move |win: &Self, _, _| {
                if win.is_fullscreen() {
                    win.activate_action("win.fullscreen", None)
                        .expect("Action not found.");
                }
            })
            .build();

        self.add_action_entries([
            preferences_action,
            shortcuts_window_action,
            toggle_fullscreen_action,
            leave_fullscreen_action,
        ]);
    }

    #[template_callback]
    fn master_stack_child_visible(&self) {
        let media_grid_imp = self.imp().library_view.imp().media_grid.imp();

        if let Some(child_name) = self.imp().master_stack.visible_child_name() {
            match child_name.as_str() {
                "library" => {
                    self.imp()
                        .search_entry
                        .set_placeholder_text(Some(&gettext("Search Photos")));

                    // If the photo grid has no model, load the photo library now.
                    if media_grid_imp.photo_grid_view.model().is_none() {
                        self.imp().library_view.load_library();
                    }
                }
                "albums" => self
                    .imp()
                    .search_entry
                    .set_placeholder_text(Some(&gettext("Search Albums"))),
                "favorites" => self
                    .imp()
                    .search_entry
                    .set_placeholder_text(Some(&gettext("Search Favorites"))),
                _ => g_error!("ApplicationWindow", "Unexpected master stack child found."),
            }
        }
    }

    #[template_callback]
    fn toggle_search_bar(&self, toggle_button: &gtk::ToggleButton) {
        self.imp().search_bar.set_search_mode(toggle_button.is_active());
    }
}
