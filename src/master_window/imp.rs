// master_window/imp.rs
//
// Copyright (c) 2024 Max Rodriguez
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::library_view::LibraryView;
use crate::options_view::OptionsView;
use adw::gtk;
use adw::subclass::prelude::*;
use gtk::glib;
use libadwaita as adw;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Gallery/master_window/master-window.ui")]
pub struct MasterWindow {
    #[template_child]
    pub master_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    pub library_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub album_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub search_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub options_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub library_view: TemplateChild<LibraryView>,
    #[template_child]
    pub options_view: TemplateChild<OptionsView>,
}

#[glib::object_subclass]
impl ObjectSubclass for MasterWindow {
    const NAME: &'static str = "MasterWindow";
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
        // This callback wont be triggered on start up by itself, so we
        // want to check the very first visible child in the master stack.
        obj.master_stack_child_visible();
    }
}
impl WidgetImpl for MasterWindow {}
impl WindowImpl for MasterWindow {}
impl ApplicationWindowImpl for MasterWindow {}
impl AdwApplicationWindowImpl for MasterWindow {}
