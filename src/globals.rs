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

use crate::util::enums::XDGUserDir;
use cfg_if::cfg_if;
use gtk::glib::source::Priority;

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

cfg_if! {
    // We're gonna assume that if we're targeting ARM,
    // we are targeting mobile devices.
    if #[cfg(target_arch = "aarch64")] {
        /// The number of permits given to the async semaphore
        /// used to control the amount of ffmpeg processes spawned.
        pub static FFMPEG_CONCURRENT_PROCESSES: usize = 2;
    } else {
        pub static FFMPEG_CONCURRENT_PROCESSES: usize = 5;
    }
}

/// IO priority for new `GtkDirectoryList` models. We override
/// the default since it is usually higher than GTK redraw priority.
pub static DIRECTORY_MODEL_PRIORITY: Priority = Priority::LOW;

/// The following statics are related to the application cache.
pub static CACHE_THUMBNAILS_SUBDIR: &str = "thumbnails";

/// The following statics are related to XDG user directories.
/// These strings are paths relative to $HOME.
pub static FALLBACK_XDG_PICTURES_DIR: &str = "Pictures";
pub static FALLBACK_XDG_VIDEOS_DIR: &str = "Videos";

/// Default library list model root directories. Must be XDG user directories.
pub static DEFAULT_LIBRARY_COLLECTION: &[XDGUserDir] = &[XDGUserDir::Pictures, XDGUserDir::Videos];

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
