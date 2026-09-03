// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::cli::Request;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
};

pub(crate) fn output_path(request: &Request) -> PathBuf {
    request.output_dir.join(request.format.default_file_name())
}

pub(crate) fn output_file_path(request: &Request, file_name: &str) -> PathBuf {
    request.output_dir.join(file_name)
}

pub(crate) fn write_file(path: &Path, contents: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("Output file '{}' has no parent directory", path.display()))?;
    fs::create_dir_all(parent).map_err(|error| {
        format!(
            "Failed to create output directory '{}': {error}",
            parent.display()
        )
    })?;
    fs::write(path, contents)
        .map_err(|error| format!("Failed to write output file '{}': {error}", path.display()))
}

pub(crate) fn check_file(path: &Path, contents: &str) -> Result<bool, String> {
    let existing = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(format!(
                "Failed to read output file '{}': {error}",
                path.display()
            ));
        }
    };

    let existing_hash = sha256(existing)
        .map_err(|error| format!("Failed to read output file '{}': {error}", path.display()))?;
    let generated_hash = sha256(contents.as_bytes())
        .map_err(|error| format!("Failed to hash generated content: {error}"))?;

    if existing_hash == generated_hash {
        Ok(true)
    } else {
        Err(format!(
            "Generated content does not match existing file '{}'",
            path.display()
        ))
    }
}

fn sha256(mut reader: impl Read) -> io::Result<[u8; 32]> {
    let mut contents = Vec::new();
    reader.read_to_end(&mut contents)?;

    let mut normalized = Vec::with_capacity(contents.len());
    let mut index = 0;
    while index < contents.len() {
        if contents[index] == b'\r' {
            normalized.push(b'\n');
            index += usize::from(contents.get(index + 1) == Some(&b'\n'));
        } else {
            normalized.push(contents[index]);
        }
        index += 1;
    }

    Ok(Sha256::digest(normalized).into())
}

#[cfg(test)]
mod tests;
