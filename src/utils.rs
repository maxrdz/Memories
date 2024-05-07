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

//! Utility functions used at seldom in Albums source.

use crate::config::APP_NAME;

/// Returns a `String` that represents the absolute path of
/// the user's cache directory, which is either the equivalent
/// of the `$XDG_CACHE_HOME` env var, or `$HOME/.cache`.
pub fn get_cache_directory() -> String {
    match std::env::var("XDG_CACHE_HOME") {
        Ok(value) => value,
        Err(e) => {
            match e {
                std::env::VarError::NotPresent => {
                    // If $XDG_CACHE_HOME is either not set or empty,
                    // a default equal to $HOME/.cache should be used.
                    // https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html#variables
                    format!("{}/.cache", std::env::var("HOME").unwrap())
                }
                _ => panic!("Unexpected std::env::VarError variant received."),
            }
        }
    }
}

/// Returns a `String` that represents the absolute
/// path of the application's cache directory location.
pub fn get_app_cache_directory() -> String {
    // TODO: If running within a Flapak sandboxed environment,
    // we can use just $XDG_CACHE_HOME as the app cache directory.
    format!("{}/{}", get_cache_directory(), APP_NAME)
}
