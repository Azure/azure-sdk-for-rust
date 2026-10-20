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
    assert_eq!(args.root, None);
    assert!(!args.review);
    assert!(!args.check);
    assert_eq!(args.output_dir, None);
}

#[test]
fn accepts_explicit_root() {
    let args = Args::parse_from([
        "generate_api",
        "--root",
        "/repo",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--output",
        "/tmp/generate_api",
    ]);

    assert_eq!(args.root, Some(std::path::PathBuf::from("/repo")));
}

#[test]
fn accepts_explicit_apiview_format() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--format",
        "apiview",
        "--output-dir",
        "/tmp/generate_api",
    ]);

    assert_eq!(args.format, OutputFormat::Apiview);
    assert_eq!(args.output_dir, Some(PathBuf::from("/tmp/generate_api")));
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
        "--output-dir",
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
        "--output-dir",
        "/tmp/generate_api",
    ]);

    assert_eq!(args.output_dir, Some(PathBuf::from("/tmp/generate_api")));
}

#[test]
fn accepts_legacy_output_alias() {
    let args = Args::parse_from([
        "generate_api",
        "--manifest-path",
        "sdk/core/azure_core/Cargo.toml",
        "--output",
        "/tmp/generate_api",
    ]);

    assert_eq!(args.output_dir, Some(PathBuf::from("/tmp/generate_api")));
}

#[test]
fn defaults_output_directory_to_crate_directory() {
    let request = Request::try_from(Args {
        manifest_path: PathBuf::from("sdk/core/azure_core/Cargo.toml"),
        root: None,
        format: OutputFormat::Markdown,
        review: true,
        check: false,
        output_dir: None,
    })
    .unwrap();

    let crate_dir = Path::new("/tmp/crate");
    assert_eq!(request.output_dir(crate_dir), crate_dir);
}
