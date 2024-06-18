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

use crate::window::MemoriesApplicationWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{clone, g_debug, g_error};
#[cfg(feature = "disable-glycin-sandbox")]
use glycin::SandboxMechanism;
use gtk::{gdk, gio, glib};
use std::ffi::OsStr;

mod imp {
    use crate::application::MemoriesApplication;
    use crate::library::properties::MemoriesDetails;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use gtk::{gio, glib};

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/media-viewer.ui")]
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
}

/// Enum that represents the types of content that
/// can be displayed by the `MemoriesMediaViewer` object.
#[derive(Debug, Clone)]
pub enum ViewerContentType {
    VectorGraphics,
    Image,
    Video,
    Invalid,
}

impl ViewerContentType {
    /// Returns a `ViewerContentType` enum that matches the file extension given.
    pub fn from_ext(extension: &OsStr) -> ViewerContentType {
        let ext_str: &str = &extension.to_str().unwrap().to_lowercase();

        match ext_str {
            "svg" => ViewerContentType::VectorGraphics,
            "png" | "jpg" | "jpeg" | "webp" | "heic" | "heif" => ViewerContentType::Image,
            "mp4" | "webm" | "mkv" | "mov" | "avi" | "gif" => ViewerContentType::Video,
            _ => {
                g_debug!("ViewerContentType", "from_ext() received invalid file extension.");
                ViewerContentType::Invalid
            }
        }
    }
}

glib::wrapper! {
    pub struct MemoriesMediaViewer(ObjectSubclass<imp::MemoriesMediaViewer>)
        @extends gtk::Widget, adw::Bin;
}

#[gtk::template_callbacks]
impl MemoriesMediaViewer {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn window(&self) -> MemoriesApplicationWindow {
        self.root()
            .expect("Must be in a GtkApplicationWindow.")
            .downcast()
            .expect("Failed to downcast to MemoriesApplicationWindow.")
    }

    /// This function is public so that it can be called once we
    /// are placed in the widget tree and can access the window.
    pub fn setup_gactions(&self) {
        let win: MemoriesApplicationWindow = self.window();
        let action_group = gio::SimpleActionGroup::new();

        let exit_viewer_action = gio::ActionEntry::builder("exit")
            .activate(
                clone!(@weak self as viewer => move |_: &gio::SimpleActionGroup, _, _| {
                    viewer.activate_action("navigation.pop", None).expect("Action not found.");
                }),
            )
            .build();

        let properties_action = gio::ActionEntry::builder("properties")
            .state(false.to_variant())
            .activate(
                clone!(@weak self as viewer => move |_: &gio::SimpleActionGroup, action: &gio::SimpleAction, _| {
                    let new_state: bool = !viewer.imp().split_view.shows_sidebar();

                    viewer.imp().split_view.set_show_sidebar(new_state);
                    action.set_state(&new_state.to_variant());
                }),
            )
            .build();

        action_group.add_action_entries([exit_viewer_action, properties_action]);
        win.insert_action_group("viewer", Some(&action_group));
    }

    /// Sets the content type setting for the viewer page.
    /// The `ViewerContentType` enum given directly correlates
    /// to a stack page that has the proper widget for the content.
    pub fn set_content_type(&self, content_type: &ViewerContentType) {
        match content_type {
            ViewerContentType::VectorGraphics => self.imp().viewer_stack.set_visible_child_name("image"),
            ViewerContentType::Image => self.imp().viewer_stack.set_visible_child_name("image"),
            ViewerContentType::Video => self.imp().viewer_stack.set_visible_child_name("video"),
            _ => g_debug!("Viewer", "Received invalid ViewerContentType enum!"),
        }
    }

    pub fn set_content_file(&self, file: &gio::File) {
        match self.imp().viewer_stack.visible_child_name().unwrap().as_str() {
            "render" => self.imp().viewer_picture.set_file(Some(file)),
            "image" => {
                glib::spawn_future_local(clone!(@weak self as viewer, @strong file => async move {
                    #[allow(unused_mut)]
                    let mut glycin_loader: glycin::Loader = glycin::Loader::new(file);

                    #[cfg(feature = "disable-glycin-sandbox")]
                    glycin_loader.sandbox_mechanism(Some(SandboxMechanism::NotSandboxed));

                    let image: glycin::Image = glycin_loader.load().await.expect("FIXME");
                    let texture: gdk::Texture = image.next_frame().await.expect("FIXME").texture;

                    viewer.imp().viewer_picture.set_paintable(Some(&texture));
                }));
            }
            "video" => self.imp().viewer_video.set_file(Some(file)),
            _ => g_error!("Viewer", "Found unexpected visible child name in viewer stack."),
        }
    }

    /// Returns a new `AdwNavigationPage` object that
    /// has its child set to the `&self` GObject.
    pub fn wrap_in_navigation_page(&self) -> adw::NavigationPage {
        let new_navigation_page: adw::NavigationPage = adw::NavigationPage::builder()
            .title(gettext("Loading Content"))
            .child(self)
            .build();
        new_navigation_page
    }
}

impl Default for MemoriesMediaViewer {
    fn default() -> Self {
        Self::new()
    }
}
