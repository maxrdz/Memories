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

use crate::config::{APP_NAME, APP_REPO, VERSION};
use cfg_if::cfg_if;
use gtk::glib::source::Priority;
use gtk::License;

#[cfg(debug_assertions)]
pub static DEVELOPMENT_BUILD: bool = true;
#[cfg(not(debug_assertions))]
pub static DEVELOPMENT_BUILD: bool = false;

cfg_if! {
    if #[cfg(debug_assertions)] {
        pub static RUST_LOG_ENVVAR_DEFAULT: &str = "debug";
        pub static G_MESSAGES_DEBUG_DEFAULT: &str = "all";
    } else {
        pub static RUST_LOG_ENVVAR_DEFAULT: &str = "info";
        pub static G_MESSAGES_DEBUG_DEFAULT: &str = "";
    }
}

/// Can be read by other modules to display to the user
/// what binary needs to be installed to use Memories.
pub static FFMPEG_BINARY: &str = "ffmpeg";

/// The number of permits given to the async semaphore
/// used to control the amount of ffmpeg processes spawned.
pub static FFMPEG_CONCURRENT_PROCESSES: usize = 5;

/// IO priority for new `GtkDirectoryList` models. We override
/// the default since it is usually higher than GTK redraw priority.
pub static DIRECTORY_MODEL_PRIORITY: Priority = Priority::LOW;

/// The following statics are related to the application cache.
pub static CACHE_THUMBNAILS_SUBDIR: &str = "thumbnails";

/// Following paths relative to $HOME env var path.
pub static DEFAULT_LIBRARY_COLLECTION: &[&str] = &["Pictures", "Videos"];
pub static DEFAULT_TRASH_DIRECTORY: &str = ".local/share/Trash";

/// Following paths relative to DEFAULT_LIBRARY_DIRECTORY.
pub static DEFAULT_SCREENSHOTS_REL_DIR: &str = "Screenshots";
pub static DEFAULT_CAMERA_REL_DIR: &str = "Camera";

/// Default `height-request` used in list item widgets
/// displayed on the library grid view on mobile.
pub static DEFAULT_GRID_WIDGET_HEIGHT: i32 = 66;

/// Grid zoom levels are tuples where the first integer is the amount
/// of min/max columns in the `GtkGridView`, and the second integer
/// is the value to set on the `grid-widget-height` property of the media grid view.
pub static GRID_MOBILE_ZOOM_LEVELS: &[(u32, i32)] = &[(5, 66), (3, 114), (2, 173)];
pub static GRID_DESKTOP_ZOOM_LEVELS: &[(u32, i32)] = &[(10, 112), (5, 234)];

/// Representation of the preferred theme options offered
/// in the application main popover menu widget.
pub enum PreferredAdwaitaTheme {
    System = 0,
    Light = 1,
    Dark = 2,
}

// Implement enum variants to translate to an i32.
// Must be in range of the 'adwaita-theme' GSchema key.
impl PreferredAdwaitaTheme {
    pub fn value(&self) -> i32 {
        match *self {
            PreferredAdwaitaTheme::System => 0,
            PreferredAdwaitaTheme::Light => 1,
            PreferredAdwaitaTheme::Dark => 2,
        }
    }
}

pub struct AboutInformation {
    pub app_name: &'static str,
    pub app_version: &'static str,
    pub app_repo: &'static str,
    pub app_author: &'static str,
    pub authors: &'static [&'static str],
    pub artists: &'static [&'static str],
    pub documenters: &'static [&'static str],
    pub copyright: &'static str,
    pub license: &'static str,
    pub license_type: License,
}

pub static APP_INFO: AboutInformation = AboutInformation {
    app_name: APP_NAME,
    app_version: VERSION,
    app_repo: APP_REPO,
    app_author: "Max Rodriguez",
    authors: &["Max Rodriguez <me@maxrdz.com>"],
    artists: &["Max Rodriguez <me@maxrdz.com>"],
    documenters: &[""],
    copyright: "© 2024 Max Rodriguez",
    license: "GNU General Public License v3.0",
    license_type: License::Gpl30,
};
