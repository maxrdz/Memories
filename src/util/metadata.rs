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

use async_fs::{File, Metadata};
use gtk::glib;
use md5::{Digest, Md5};
use serde::Serialize;
use std::io;
use std::time::SystemTime;

/// A data structure that contains the file metadata information
/// that the thumbnailer needs to serialize and fingerprint hash.
#[derive(Debug, Clone, Serialize)]
pub struct MetadataInfo {
    pub file_type: String,
    pub size: u64,
    pub modified: SystemTime,
    pub accessed: SystemTime,
    pub created: SystemTime,
}

impl MetadataInfo {
    pub fn pretty_print_bytes(&self) -> String {
        glib::format_size(self.size).to_string()
    }
}

/// Takes in `std::file::Metadata` and packs necessary
/// information into the `MetadataInfo` structure.
pub fn pack_metadata_as_struct(metadata: &Metadata) -> io::Result<MetadataInfo> {
    Ok(MetadataInfo {
        file_type: format!("{:?}", metadata.file_type()),
        size: metadata.len(),
        modified: metadata.modified()?,
        accessed: metadata.accessed()?,
        created: metadata.created()?,
    })
}

/// Returns `MetadataInfo` struct and a `String` that
/// contains the metadata MD5 digest in hexadecimal format.
pub async fn get_metadata_with_hash(file: File) -> io::Result<(MetadataInfo, String)> {
    let in_metadata: Metadata = file.metadata().await?;

    let mut md5_hasher: Md5 = Md5::new();
    let metadata = pack_metadata_as_struct(&in_metadata)?;

    md5_hasher.update(serde_json::to_vec(&metadata)?);

    Ok((metadata, format!("{:x}", md5_hasher.finalize())))
}
