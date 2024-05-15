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
use crate::library::viewer::ViewerContentType;
use adw::glib::g_debug;
use async_fs::Metadata;
use libadwaita as adw;
use serde::Serialize;
use std::io;
use std::time::SystemTime;

/// A data structure that contains the file metadata information
/// that the thumbnailer needs to serialize and fingerprint hash.
#[derive(Serialize)]
struct MetadataInfo {
    file_type: String,
    permissions: String,
    size: u64,
    modified: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
}

/// Serializes an `std::fs::Metadata` structure to a serialized JSON byte array.
pub fn serialize_file_metadata(metadata: &Metadata) -> io::Result<Vec<u8>> {
    let structure: MetadataInfo = MetadataInfo {
        file_type: format!("{:?}", metadata.file_type()),
        permissions: format!("{:?}", metadata.permissions()),
        size: metadata.len(),
        modified: metadata.modified()?,
        accessed: metadata.accessed()?,
        created: metadata.created()?,
    };
    Ok(serde_json::to_vec(&structure)?)
}

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
    format!("{}/{}", get_cache_directory(), APP_NAME)
}

/// Returns a `ViewerContentType` enum that matches the file extension given.
pub fn get_content_type_from_ext(file_ext: &str) -> ViewerContentType {
    match file_ext {
        "svg" => ViewerContentType::Renderable,
        "png" | "jpg" | "jpeg" | "webp" | "heic" | "heif" => ViewerContentType::Image,
        "mp4" | "webm" | "mkv" | "mov" | "avi" | "gif" => ViewerContentType::Video,
        _ => {
            g_debug!(
                "Utils",
                "get_content_type_from_ext() received invalid file extension."
            );
            ViewerContentType::Invalid
        }
    }
}
