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

pub(crate) fn render_markdown_metadata(
    api_md_contents: &str,
    package_version: &str,
    parser_version: &str,
    rust_version: &str,
) -> Result<String, String> {
    let api_md_sha256 = sha256_hex(api_md_contents.as_bytes())
        .map_err(|error| format!("Failed to hash API.md content: {error}"))?;

    Ok(format!(
        "apiMdSha256: {api_md_sha256}\npackageVersion: {}\nparserVersion: {}\nrustVersion: {}\n",
        yaml_scalar(package_version),
        yaml_scalar(parser_version),
        yaml_scalar(rust_version),
    ))
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

fn sha256_hex(reader: impl Read) -> io::Result<String> {
    let hash = sha256(reader)?;
    Ok(hash.iter().map(|byte| format!("{byte:02x}")).collect())
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

fn yaml_scalar(value: &str) -> String {
    if requires_yaml_quotes(value) {
        format!("'{}'", value.replace('\'', "''"))
    } else {
        value.to_string()
    }
}

fn requires_yaml_quotes(value: &str) -> bool {
    if value.is_empty()
        || value.chars().next().is_some_and(char::is_whitespace)
        || value.chars().last().is_some_and(char::is_whitespace)
    {
        return true;
    }

    if value.contains('\n')
        || value.contains('\r')
        || value.contains('\t')
        || value.contains(": ")
        || value.contains(" #")
    {
        return true;
    }

    matches!(
        value.chars().next(),
        Some(
            '-' | '?'
                | ':'
                | '!'
                | '&'
                | '*'
                | '{'
                | '}'
                | '['
                | ']'
                | ','
                | '#'
                | '|'
                | '>'
                | '\''
                | '"'
                | '%'
                | '@'
                | '`'
        )
    ) || matches!(
        value.to_ascii_lowercase().as_str(),
        "null" | "~" | "true" | "false" | "yes" | "no" | "on" | "off"
    )
}

#[cfg(test)]
mod tests;
