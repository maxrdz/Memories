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

use super::theme_selector::AlbumsThemeSelector;
use adw::gtk;
use adw::subclass::prelude::*;
use gtk::glib;
use libadwaita as adw;
use std::cell::RefCell;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Albums/preferences/preferences.ui")]
pub struct AlbumsPreferencesView {
    // We have to store references to our `AdwActionRow` widgets
    // to be able to clear them out of the preference group as children.
    pub(super) library_collection_rows: RefCell<Vec<adw::ActionRow>>,

    #[template_child]
    mobile_header_label: TemplateChild<gtk::Label>,
    #[template_child]
    theme_selector: TemplateChild<AlbumsThemeSelector>,
    #[template_child]
    mobile_actions_flowbox: TemplateChild<gtk::FlowBox>,
    #[template_child]
    pub(super) library_collection: TemplateChild<adw::PreferencesGroup>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumsPreferencesView {
    const NAME: &'static str = "AlbumsPreferencesView";
    type Type = super::AlbumsPreferencesView;
    type ParentType = adw::BreakpointBin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AlbumsPreferencesView {}
impl WidgetImpl for AlbumsPreferencesView {}
impl BreakpointBinImpl for AlbumsPreferencesView {}
