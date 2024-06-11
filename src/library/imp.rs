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

use super::media_grid::MrsMediaGridView;
use adw::subclass::prelude::*;
use gtk::glib;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Memories/library/library.ui")]
pub struct MrsLibraryView {
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
    pub media_grid: TemplateChild<MrsMediaGridView>,
}

#[glib::object_subclass]
impl ObjectSubclass for MrsLibraryView {
    const NAME: &'static str = "MrsLibraryView";
    type Type = super::MrsLibraryView;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MrsLibraryView {}
impl WidgetImpl for MrsLibraryView {}
impl BinImpl for MrsLibraryView {}
