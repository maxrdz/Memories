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

use adw::subclass::prelude::*;
use gtk::glib;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Memories/window/theme_selector/theme-selector.ui")]
pub struct MrsThemeSelector {
    #[template_child]
    pub selector_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub system_selector: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub light_selector: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub dark_selector: TemplateChild<gtk::CheckButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for MrsThemeSelector {
    const NAME: &'static str = "MrsThemeSelector";
    type Type = super::MrsThemeSelector;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MrsThemeSelector {}
impl WidgetImpl for MrsThemeSelector {}
impl BinImpl for MrsThemeSelector {}
