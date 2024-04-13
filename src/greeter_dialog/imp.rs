// greeter_dialog/imp.rs
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
#[template(resource = "/com/maxrdz/Album/greeter_dialog/greeter-dialog.ui")]
pub struct GreeterDialog {}

#[glib::object_subclass]
impl ObjectSubclass for GreeterDialog {
    const NAME: &'static str = "GreeterDialog";
    type Type = super::GreeterDialog;
    type ParentType = adw::Dialog;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(_obj: &glib::subclass::InitializingObject<Self>) {
        //obj.init_template();
    }
}

impl ObjectImpl for GreeterDialog {}
impl WidgetImpl for GreeterDialog {}
impl AdwDialogImpl for GreeterDialog {}
