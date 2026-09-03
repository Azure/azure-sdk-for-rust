// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use serde_json::Value;

fn mapping(
    generated_line: usize,
    generated_column: usize,
    path: &str,
    line: usize,
    column: usize,
) -> GeneratedMapping {
    GeneratedMapping {
        generated_line,
        generated_column,
        original: SourceLocation {
            path: path.to_string(),
            line,
            column,
        },
    }
}

#[test]
fn encodes_base64_vlq_values() {
    let mut encoded = String::new();
    encode_vlq(17, &mut encoded);
    encode_vlq(-10, &mut encoded);
    assert_eq!(encoded, "iBV");
}

#[test]
fn renders_v3_source_map() {
    let rendered = render(
        "API.md",
        &[
            mapping(7, 0, "src/lib.rs", 4, 0),
            mapping(9, 4, "src/client.rs", 11, 8),
        ],
    )
    .unwrap();
    let json: Value = serde_json::from_str(&rendered).unwrap();

    assert_eq!(json["version"], 3);
    assert_eq!(json["file"], "API.md");
    assert_eq!(
        json["sources"],
        serde_json::json!(["src/lib.rs", "src/client.rs"])
    );
    assert_eq!(json["mappings"], ";;;;;;;AAIA;;ICOQ");
    assert!(json.get("sourceRoot").is_none());
    assert!(json.get("names").is_none());
}

#[test]
fn sources_follow_first_mapping_occurrence() {
    let rendered = render(
        "API.md",
        &[
            mapping(2, 0, "src/z.rs", 0, 0),
            mapping(1, 0, "src/a.rs", 0, 0),
        ],
    )
    .unwrap();
    let json: Value = serde_json::from_str(&rendered).unwrap();

    assert_eq!(json["sources"], serde_json::json!(["src/a.rs", "src/z.rs"]));
    assert_eq!(json["mappings"], ";AAAA;ACAA");
}
