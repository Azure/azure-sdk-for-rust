// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::model::SourceLocation;
use serde::Serialize;
use std::collections::BTreeMap;

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
    sources: Vec<String>,
    mappings: String,
}

pub(crate) fn render(
    file: impl Into<String>,
    mappings: &[GeneratedMapping],
) -> Result<String, String> {
    let mut mappings = mappings.to_vec();
    mappings.sort_by_key(|mapping| (mapping.generated_line, mapping.generated_column));

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
        sources,
        mappings,
    };
    let mut json = serde_json::to_string(&map)
        .map_err(|error| format!("Failed to serialize source map: {error}"))?;
    json.push('\n');
    Ok(json)
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
