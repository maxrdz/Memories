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
use std::time::Duration;

mod imp {
    use crate::application::MemoriesApplication;
    use crate::library::properties::MemoriesProperties;
    use adw::prelude::{ObjectExt, SettingsExtManual, WidgetExt};
    use adw::subclass::prelude::*;
    use glib::clone;
    use gtk::{gio, glib};
    use std::cell::Cell;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/Memories/ui/media-viewer.ui")]
    pub struct MemoriesMediaViewer {
        pub(super) motion_last_x: Cell<f64>,
        pub(super) motion_last_y: Cell<f64>,
        pub(super) overlay_timeout_source: Cell<Option<glib::SourceId>>,

        #[template_child]
        header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        more_button: TemplateChild<gtk::MenuButton>,
        // TODO: Update to `adw::MultiLayoutView` once bindings for 1.6 are merged.
        #[template_child]
        multi_layout: TemplateChild<gtk::Widget>,
        #[template_child]
        controls_overlay: TemplateChild<gtk::Overlay>,
        #[template_child]
        pub(super) split_view: TemplateChild<adw::OverlaySplitView>,
        // TODO: Update to `adw::BottomSheet` once bindings for 1.6 are merged.
        #[template_child]
        pub(super) bottom_sheet: TemplateChild<gtk::Widget>,
        #[template_child]
        pub properties_widget: TemplateChild<MemoriesProperties>,
        #[template_child]
        pub(super) nav_overlay_revealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        nav_overlay_controls: TemplateChild<gtk::Box>,
        #[template_child]
        pub(super) zoom_overlay_revealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        zoom_overlay_controls: TemplateChild<gtk::Box>,
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
            let obj = self.obj();
            self.parent_constructed();

            let gsettings: gio::Settings = MemoriesApplication::default().gsettings();

            gsettings
                .bind("autoplay-videos", &self.viewer_video.clone(), "autoplay")
                .build();

            self.bottom_sheet.connect_notify_local(
                Some("open"),
                clone!(
                    #[weak]
                    obj,
                    move |bottom_sheet: &gtk::Widget, _: &glib::ParamSpec| {
                        if !bottom_sheet.property::<bool>("open") {
                            obj.window()
                                .activate_action("viewer.properties", None)
                                .expect("Action not found!");
                        }
                    }
                ),
            );
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
            .activate(clone!(
                #[weak(rename_to = this)]
                self,
                move |_: &gio::SimpleActionGroup, _, _| {
                    this.activate_action("navigation.pop", None)
                        .expect("Action not found.");
                }
            ))
            .build();

        let properties_action = gio::ActionEntry::builder("properties")
            .state(false.to_variant())
            .activate(clone!(
                #[weak(rename_to = this)]
                self,
                move |_: &gio::SimpleActionGroup, action: &gio::SimpleAction, _| {
                    let new_state: bool = !this.imp().split_view.shows_sidebar();

                    this.imp().split_view.set_show_sidebar(new_state);

                    if !this.imp().bottom_sheet.property::<bool>("open") {
                        this.imp().bottom_sheet.set_property("open", new_state);
                    }
                    action.set_state(&new_state.to_variant());
                }
            ))
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
                glib::spawn_future_local(clone!(
                    #[weak(rename_to = this)]
                    self,
                    #[strong]
                    file,
                    async move {
                        #[allow(unused_mut)]
                        let mut glycin_loader: glycin::Loader = glycin::Loader::new(file);

                        #[cfg(feature = "disable-glycin-sandbox")]
                        glycin_loader.sandbox_mechanism(Some(SandboxMechanism::NotSandboxed));

                        let image: glycin::Image = glycin_loader.load().await.expect("FIXME");
                        let texture: gdk::Texture = image.next_frame().await.expect("FIXME").texture();

                        this.imp().viewer_picture.set_paintable(Some(&texture));
                    }
                ));
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

    fn reveal_overlay_controls(&self) {
        self.imp().nav_overlay_revealer.set_reveal_child(true);
        self.imp().zoom_overlay_revealer.set_reveal_child(true);

        if let Some(src_id) = self.imp().overlay_timeout_source.replace(None) {
            src_id.remove();
        }

        let timeout: glib::SourceId = glib::timeout_add_local_once(
            Duration::from_secs(3),
            clone!(
                #[weak(rename_to = this)]
                self,
                move || {
                    this.imp().nav_overlay_revealer.set_reveal_child(false);
                    this.imp().zoom_overlay_revealer.set_reveal_child(false);

                    // If this closure is executed, `overlay_timeout_source` is
                    // guaranteed to contain a `Some` option value, so we can unwrap().
                    this.imp().overlay_timeout_source.replace(None).unwrap().remove();
                }
            ),
        );
        self.imp().overlay_timeout_source.set(Some(timeout));
    }

    #[template_callback]
    fn overlay_motion_handler(&self, x: f64, y: f64) {
        // After the overlay is hidden again, the motion controller emits a motion
        // event again, but with rounded values. Because of this, we have to get the
        // delta of the mouse movement to the last movement detected and only reveal
        // the overlay if the delta for **both** x and y are above 1.0 units.

        let last_x: f64 = self.imp().motion_last_x.replace(x);
        let last_y: f64 = self.imp().motion_last_y.replace(y);

        if last_x != 0.0 || last_y != 0.0 {
            let motion_delta_x: f64 = (last_x - x).abs();
            let motion_delta_y: f64 = (last_y - y).abs();

            if (motion_delta_x <= 1_f64) && (motion_delta_y <= 1_f64) {
                return;
            }
        }
        self.reveal_overlay_controls();
    }

    #[template_callback]
    fn touch_gesture_handler(&self, _: i32, _: f64, _: f64) {
        self.reveal_overlay_controls();
    }
}

impl Default for MemoriesMediaViewer {
    fn default() -> Self {
        Self::new()
    }
}
