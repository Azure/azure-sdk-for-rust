// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

#[test]
fn defaults_to_markdown_format() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--output",
        "/tmp/generate_api",
    ]);

    assert_eq!(args.format, OutputFormat::Markdown);
    assert!(!args.no_docs);
}

#[test]
fn accepts_explicit_apiview_format() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--format",
        "apiview",
        "--output",
        "/tmp/generate_api",
    ]);

    assert_eq!(args.format, OutputFormat::Apiview);
}

#[test]
fn accepts_no_docs_switch() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--format",
        "apiview",
        "--no-docs",
        "--output",
        "/tmp/generate_api",
    ]);

    assert!(args.no_docs);
}
