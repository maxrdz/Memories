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

use crate::config::{APP_NAME, VERSION};
use adw::gtk::License;
use cfg_if::cfg_if;
use libadwaita as adw;

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

/// The number of permits given to the async semaphore
/// used to control the amount of ffmpeg processes spawned.
pub static FFMPEG_CONCURRENT_PROCESSES: usize = 5;

/// The following statics are related to Albums' cache.
pub static CACHE_THUMBNAILS_SUBDIR: &str = "thumbnails";

/// Following paths relative to $HOME env var path.
pub static DEFAULT_LIBRARY_DIRECTORY: &str = "Pictures";
pub static DEFAULT_TRASH_DIRECTORY: &str = ".local/share/Trash";

/// Following paths relative to DEFAULT_LIBRARY_DIRECTORY.
pub static DEFAULT_SCREENSHOTS_REL_DIR: &str = "Screenshots";
pub static DEFAULT_CAMERA_REL_DIR: &str = "Camera";

/// Default `height-request` used in list item widgets
/// displayed on the library grid view on mobile.
pub static DEFAULT_GRID_WIDGET_HEIGHT: i32 = 70;

pub static GRID_MOBILE_ZOOM_LEVELS: &[(u32, i32)] = &[(5, 70), (3, 119), (2, 178)];
pub static GRID_DESKTOP_ZOOM_LEVELS: &[(u32, i32)] = &[(15, 78), (10, 118)];

/// Purge delay for recently deleted library items.
#[rustfmt::skip]
#[repr(u8)]
pub enum RecentlyDeletedPurgeAfter {
    OneHour, OneDay, TwoDays, ThreeDays, FourDays,
    FiveDays, SixDays, OneWeek, TwoWeeks, OneMonth,
}

/// Default time delay for purging recently deleted library items.
/// Following the same default delay as org.gnome.Settings.
pub static DEFAULT_RECENTLY_DELETED_PURGE_DELAY: RecentlyDeletedPurgeAfter =
    RecentlyDeletedPurgeAfter::OneMonth;

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
    app_repo: "https://gitlab.gnome.org/maxrdz/Albums",
    app_author: "Max Rodriguez",
    authors: &["Max Rodriguez <me@maxrdz.com>"],
    artists: &["Max Rodriguez <me@maxrdz.com>"],
    documenters: &[""],
    copyright: "© 2024 Max Rodriguez",
    license: "GNU General Public License v3.0",
    license_type: License::Gpl30,
};
