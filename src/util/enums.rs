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

use crate::globals::{FALLBACK_XDG_PICTURES_DIR, FALLBACK_XDG_VIDEOS_DIR};
use std::process::{Command, Output};
use std::str::from_utf8;

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

fn trim_newline(string: &mut String) {
    if string.ends_with('\n') {
        string.pop();
        if string.ends_with('\r') {
            string.pop();
        }
    }
}

/// Represents all XDG user directories
/// per the freedesktop specification.
#[allow(dead_code)]
pub enum XDGUserDir {
    Desktop,
    Download,
    Templates,
    Public,
    Documents,
    Music,
    Pictures,
    Videos,
}

impl XDGUserDir {
    // Translates an enum variant to a tuple with
    // 2 string slices. The tuple's first string represents
    // the XDG user dir variable name as configured in
    // $XDG_CONFIG_HOME/user-dirs.dirs. The tuple's second
    // string slice represents the user directory name
    // according to the `xdg-user-dir` utility binary.
    pub fn value(&self) -> (&str, &str) {
        match *self {
            XDGUserDir::Desktop => ("XDG_DESKTOP_DIR", "DESKTOP"),
            XDGUserDir::Download => ("XDG_DOWNLOAD_DIR", "DOWNLOAD"),
            XDGUserDir::Templates => ("XDG_TEMPLATES_DIR", "TEMPLATES"),
            XDGUserDir::Public => ("XDG_PUBLICSHARE_DIR", "PUBLICSHARE"),
            XDGUserDir::Documents => ("XDG_DOCUMENTS_DIR", "DOCUMENTS"),
            XDGUserDir::Music => ("XDG_MUSIC_DIR", "MUSIC"),
            XDGUserDir::Pictures => ("XDG_PICTURES_DIR", "PICTURES"),
            XDGUserDir::Videos => ("XDG_VIDEOS_DIR", "VIDEOS"),
        }
    }

    /// Returns the absolute path for the XDG user directory.
    /// Looks up the configured XDG user directory through the
    /// set environment variable for it. (e.g. `XDG_PICTURES_DIR`)
    /// If the environment variable is not present, the `xdg-usr-dir`
    /// binary utility is used to search for the configured path.
    pub fn get_path(&self) -> String {
        let variant_value: (&str, &str) = self.value();
        let home: String = std::env::var("HOME").expect("$HOME env var not set.");

        if let Ok(dir) = std::env::var(variant_value.0) {
            dir
        } else {
            let xdg_user_dir_out: Output = Command::new("xdg-user-dir")
                .arg(variant_value.1)
                .output()
                .expect("A problem occurred when running xdg-user-dir.");

            let mut xdg_user_dir: String = from_utf8(&xdg_user_dir_out.stdout).unwrap().to_string();

            trim_newline(&mut xdg_user_dir);

            if xdg_user_dir == home {
                match *self {
                    XDGUserDir::Pictures => format!("{}/{}", home, FALLBACK_XDG_PICTURES_DIR),
                    XDGUserDir::Videos => format!("{}/{}", home, FALLBACK_XDG_VIDEOS_DIR),
                    _ => panic!("Received unexpected XDGUserDir enum variant."),
                }
            } else {
                xdg_user_dir
            }
        }
    }
}
