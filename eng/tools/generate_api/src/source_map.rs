// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::model::SourceLocation;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    path::{Component, Path, PathBuf},
};

const BASE64_DIGITS: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct GeneratedMapping {
    pub(crate) generated_line: usize,
    pub(crate) generated_column: usize,
    pub(crate) original: SourceLocation,
}

#[derive(Serialize)]
struct SourceMap {
    version: u8,
    file: String,
    #[serde(rename = "sourceRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    source_root: Option<String>,
    sources: Vec<String>,
    mappings: String,
}

pub(crate) fn render(
    file: impl Into<String>,
    mappings: &[GeneratedMapping],
    source_map_dir: &Path,
    repository_root: &Path,
) -> Result<String, String> {
    let mut mappings = mappings.to_vec();
    mappings.sort_by_key(|mapping| (mapping.generated_line, mapping.generated_column));

    let source_root = relative_source_root(source_map_dir, repository_root)?;
    let mut source_indices = BTreeMap::new();
    let mut sources = Vec::new();
    for mapping in &mappings {
        if !source_indices.contains_key(&mapping.original.path) {
            let index = sources.len();
            source_indices.insert(mapping.original.path.clone(), index);
            sources.push(mapping.original.path.clone());
        }
    }

    let mappings = encode_mappings(&mappings, &source_indices)?;
    let map = SourceMap {
        version: 3,
        file: file.into(),
        source_root,
        sources,
        mappings,
    };
    let mut json = serde_json::to_string(&map)
        .map_err(|error| format!("Failed to serialize source map: {error}"))?;
    json.push('\n');
    Ok(json)
}

fn relative_source_root(
    source_map_dir: &Path,
    repository_root: &Path,
) -> Result<Option<String>, String> {
    let source_map_dir = std::path::absolute(source_map_dir).map_err(|error| {
        format!(
            "Failed to resolve source map directory '{}': {error}",
            source_map_dir.display()
        )
    })?;
    let repository_root = std::path::absolute(repository_root).map_err(|error| {
        format!(
            "Failed to resolve repository root '{}': {error}",
            repository_root.display()
        )
    })?;
    if !source_map_dir.starts_with(&repository_root) {
        return Ok(None);
    }
    Ok(relative_path(&source_map_dir, &repository_root)
        .map(|path| path.to_string_lossy().replace('\\', "/")))
}

fn relative_path(from: &Path, to: &Path) -> Option<PathBuf> {
    let from = from.components().collect::<Vec<_>>();
    let to = to.components().collect::<Vec<_>>();
    let common = from
        .iter()
        .zip(&to)
        .take_while(|(from, to)| from == to)
        .count();

    if common == 0
        || from[common..]
            .iter()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return None;
    }

    let mut relative = PathBuf::new();
    for _ in common..from.len() {
        relative.push("..");
    }
    for component in &to[common..] {
        relative.push(component.as_os_str());
    }
    Some(relative)
}

fn encode_mappings(
    mappings: &[GeneratedMapping],
    source_indices: &BTreeMap<String, usize>,
) -> Result<String, String> {
    let mut output = String::new();
    let mut generated_line = 0usize;
    let mut previous_source = 0i64;
    let mut previous_original_line = 0i64;
    let mut previous_original_column = 0i64;
    let mut previous_generated_column = 0i64;
    let mut first_segment_on_line = true;

    for mapping in mappings {
        if mapping.generated_line < generated_line {
            return Err("Source map mappings are not ordered by generated line".to_string());
        }
        while generated_line < mapping.generated_line {
            output.push(';');
            generated_line += 1;
            first_segment_on_line = true;
            previous_generated_column = 0;
        }
        if !first_segment_on_line {
            output.push(',');
        }

        let source = *source_indices
            .get(&mapping.original.path)
            .ok_or_else(|| format!("Source '{}' was not indexed", mapping.original.path))?
            as i64;
        encode_vlq(
            mapping.generated_column as i64 - previous_generated_column,
            &mut output,
        );
        encode_vlq(source - previous_source, &mut output);
        encode_vlq(
            mapping.original.line as i64 - previous_original_line,
            &mut output,
        );
        encode_vlq(
            mapping.original.column as i64 - previous_original_column,
            &mut output,
        );

        previous_source = source;
        previous_original_line = mapping.original.line as i64;
        previous_original_column = mapping.original.column as i64;
        previous_generated_column = mapping.generated_column as i64;
        first_segment_on_line = false;
    }

    Ok(output)
}

fn encode_vlq(value: i64, output: &mut String) {
    let mut value = if value < 0 {
        ((-value) as u64) << 1 | 1
    } else {
        (value as u64) << 1
    };

    loop {
        let mut digit = (value & 0b1_1111) as usize;
        value >>= 5;
        if value != 0 {
            digit |= 0b10_0000;
        }
        output.push(BASE64_DIGITS[digit] as char);
        if value == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests;
