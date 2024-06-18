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
use crate::library::details::MemoriesDetails;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

#[derive(Default, gtk::CompositeTemplate)]
#[template(resource = "/com/maxrdz/Memories/library/viewer/viewer.ui")]
pub struct MemoriesMediaViewer {
    #[template_child]
    header_bar: TemplateChild<adw::HeaderBar>,
    #[template_child]
    more_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub(super) split_view: TemplateChild<adw::OverlaySplitView>,
    #[template_child]
    pub details_widget: TemplateChild<MemoriesDetails>,
    #[template_child]
    overlay_controls: TemplateChild<gtk::Box>,
    #[template_child]
    pub(super) viewer_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    image_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    video_page: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    scrolled_window: TemplateChild<gtk::ScrolledWindow>,
    #[template_child]
    pub(super) viewer_picture: TemplateChild<gtk::Picture>,
    #[template_child]
    pub(super) viewer_video: TemplateChild<gtk::Video>,
}

#[glib::object_subclass]
impl ObjectSubclass for MemoriesMediaViewer {
    const NAME: &'static str = "MemoriesMediaViewer";
    type Type = super::MemoriesMediaViewer;
    type ParentType = adw::BreakpointBin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MemoriesMediaViewer {
    fn constructed(&self) {
        self.parent_constructed();
        let gsettings: gio::Settings = MemoriesApplication::default().gsettings();

        gsettings
            .bind("autoplay-videos", &self.viewer_video.clone(), "autoplay")
            .build();
    }
}

impl WidgetImpl for MemoriesMediaViewer {}
impl BinImpl for MemoriesMediaViewer {}
impl BreakpointBinImpl for MemoriesMediaViewer {}
