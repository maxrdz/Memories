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

use super::theme_selector::ThemeSelector;
use adw::gtk;
use adw::subclass::prelude::*;
use gtk::glib;
use libadwaita as adw;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/preferences/preferences-view.ui")]
pub struct PreferencesView {
    #[template_child]
    pub theme_selector: TemplateChild<ThemeSelector>,
    #[template_child]
    pub mobile_actions_flowbox: TemplateChild<gtk::FlowBox>,
}

#[glib::object_subclass]
impl ObjectSubclass for PreferencesView {
    const NAME: &'static str = "AlbumsPreferencesView";
    type Type = super::PreferencesView;
    type ParentType = adw::BreakpointBin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PreferencesView {}
impl WidgetImpl for PreferencesView {}
impl BreakpointBinImpl for PreferencesView {}
