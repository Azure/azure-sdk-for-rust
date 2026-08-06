// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::cli::Request;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn output_path(request: &Request) -> Result<PathBuf, String> {
    fs::create_dir_all(&request.output_dir).map_err(|error| {
        format!(
            "Failed to create output directory '{}': {error}",
            request.output_dir.display()
        )
    })?;

    Ok(request.output_dir.join(request.format.default_file_name()))
}

pub(crate) fn write_file(path: &Path, contents: &str) -> Result<(), String> {
    fs::write(path, contents)
        .map_err(|error| format!("Failed to write output file '{}': {error}", path.display()))
}
