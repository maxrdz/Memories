// globals.rs
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

/// Following paths relative to $HOME env var path.
pub static DEFAULT_LIBRARY_DIRECTORY: &str = "Pictures";
pub static DEFAULT_TRASH_DIRECTORY: &str = ".local/share/Trash";

/// Following paths relative to DEFAULT_LIBRARY_DIRECTORY.
pub static DEFAULT_SCREENSHOTS_REL_DIR: &str = "Screenshots";
pub static DEFAULT_CAMERA_REL_DIR: &str = "Camera";

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
    pub app_title: &'static str,
    pub app_version: &'static str,
    pub app_id: &'static str,
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
    app_title: {
        match DEVELOPMENT_BUILD {
            false => "Album",
            true => "Album (Devel)",
        }
    },
    app_version: VERSION,
    app_repo: "https://gitlab.gnome.org/maxrdz/Album",
    app_author: "Max Rodriguez",
    app_id: {
        match DEVELOPMENT_BUILD {
            false => "com.maxrdz.Album",
            true => "com.maxrdz.Album.Devel",
        }
    },
    authors: &["Max Rodriguez <me@maxrdz.com>"],
    artists: &["Max Rodriguez <me@maxrdz.com>"],
    documenters: &[""],
    copyright: "© 2024 Max Rodriguez",
    license: "GNU General Public License v3.0",
    license_type: License::Gpl30,
};
