// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

#[test]
fn defaults_to_markdown_format() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
    ]);

    assert_eq!(args.format, OutputFormat::Markdown);
    assert!(!args.review);
    assert!(!args.check);
    assert_eq!(args.output, None);
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
fn accepts_review_for_markdown() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--review",
    ]);

    let request = Request::try_from(args).unwrap();
    assert!(request.review);
}

#[test]
fn accepts_check_switch() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--check",
        "--output",
        "/tmp/generate_api",
    ]);

    assert!(args.check);
}

#[test]
fn rejects_review_for_apiview() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--format",
        "apiview",
        "--review",
    ]);

    assert_eq!(
        Request::try_from(args).unwrap_err(),
        "--review can only be used with --format markdown"
    );
}

#[test]
fn accepts_explicit_output_directory() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--output",
        "/tmp/generate_api",
    ]);

    assert_eq!(args.output, Some(PathBuf::from("/tmp/generate_api")));
}
