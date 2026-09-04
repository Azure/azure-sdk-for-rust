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
        Path::new("/repo"),
        Path::new("/repo"),
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
    assert_eq!(json["sourceRoot"], "");
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
        Path::new("/repo"),
        Path::new("/repo"),
    )
    .unwrap();
    let json: Value = serde_json::from_str(&rendered).unwrap();

    assert_eq!(json["sources"], serde_json::json!(["src/a.rs", "src/z.rs"]));
    assert_eq!(json["mappings"], ";AAAA;ACAA");
}

fn source_map_from_rendered(rendered: &str) -> Value {
    serde_json::from_str(rendered).unwrap()
}

fn resolved_source_from_rendered(rendered: &str) -> PathBuf {
    let json: Value = serde_json::from_str(rendered).unwrap();
    Path::new(json["sourceRoot"].as_str().unwrap()).join(json["sources"][0].as_str().unwrap())
}

#[test]
fn source_root_points_to_repository_from_nested_output() {
    let rendered = render(
        "API.md",
        &[mapping(0, 0, "sdk/core/azure_core/src/lib.rs", 0, 0)],
        Path::new("/repo/target/generate_api/azure_core"),
        Path::new("/repo"),
    )
    .unwrap();
    let json = source_map_from_rendered(&rendered);

    assert_eq!(json["sourceRoot"], "../../..");
    assert_eq!(json["sources"][0], "sdk/core/azure_core/src/lib.rs");
}

#[test]
fn source_root_accounts_for_deeper_output_directories() {
    let rendered = render(
        "API.md",
        &[mapping(0, 0, "sdk/core/azure_core/src/lib.rs", 0, 0)],
        Path::new("/repo/sdk/core/azure_core/api"),
        Path::new("/repo"),
    )
    .unwrap();
    let json = source_map_from_rendered(&rendered);

    assert_eq!(json["sourceRoot"], "../../../..");
    assert_eq!(json["sources"][0], "sdk/core/azure_core/src/lib.rs");
}

#[test]
fn source_root_is_omitted_for_output_outside_the_repository() {
    let rendered = render(
        "API.md",
        &[mapping(0, 0, "sdk/core/azure_core/src/lib.rs", 0, 0)],
        Path::new("/artifacts/azure_core"),
        Path::new("/repo"),
    )
    .unwrap();
    let json = source_map_from_rendered(&rendered);

    assert!(json.get("sourceRoot").is_none());
    assert_eq!(json["sources"][0], "sdk/core/azure_core/src/lib.rs");
}

#[test]
fn rendered_source_resolves_from_the_output_directory() {
    let root = std::env::temp_dir().join(format!(
        "generate_api_source_map_test_{}",
        std::process::id()
    ));
    let repository_root = root.join("repo");
    let source = repository_root.join("sdk/core/azure_core/src/lib.rs");
    let output = repository_root.join("sdk/core/azure_core/api");
    std::fs::create_dir_all(source.parent().unwrap()).unwrap();
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(&source, "").unwrap();

    let rendered = render(
        "API.md",
        &[mapping(0, 0, "sdk/core/azure_core/src/lib.rs", 0, 0)],
        &output,
        &repository_root,
    )
    .unwrap();
    let resolved = output.join(resolved_source_from_rendered(&rendered));
    let resolved = resolved.canonicalize().unwrap();

    assert_eq!(resolved, source.canonicalize().unwrap());
    std::fs::remove_dir_all(root).unwrap();
}
