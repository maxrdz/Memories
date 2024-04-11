// library_view/imp.rs
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

use adw::gtk;
use adw::subclass::prelude::*;
use gtk::glib;
use libadwaita as adw;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Album/library_view/library-view.ui")]
pub struct LibraryView {
    #[template_child]
    pub library_view_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    pub spinner_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub spinner: TemplateChild<gtk::Spinner>,
    #[template_child]
    pub error_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub error_status_widget: TemplateChild<adw::StatusPage>,
    #[template_child]
    pub gallery_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub photo_grid_view: TemplateChild<gtk::GridView>,
}

#[glib::object_subclass]
impl ObjectSubclass for LibraryView {
    const NAME: &'static str = "LibraryView";
    type Type = super::LibraryView;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for LibraryView {}
impl WidgetImpl for LibraryView {}
impl BinImpl for LibraryView {}
