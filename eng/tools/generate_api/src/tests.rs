// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{cli, resolve_repository_root, source_map};
use std::{env, fs, path::PathBuf};

fn request(root: Option<PathBuf>) -> cli::Request {
    cli::Request {
        manifest_path: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
        root,
        format: cli::OutputFormat::Markdown,
        review: true,
        check: false,
        output_dir: None,
    }
}

fn assert_default_output_source_root(request: &cli::Request) {
    let package_dir = fs::canonicalize(env!("CARGO_MANIFEST_DIR")).unwrap();
    let repository_root = resolve_repository_root(request).unwrap();
    assert_eq!(repository_root, package_dir);

    let output_dir = request.output_dir(&package_dir);
    let rendered = source_map::render("api.md", &[], output_dir, &repository_root).unwrap();
    let map: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    assert_eq!(map["sourceRoot"], "");
}

#[test]
fn root_with_parent_components_preserves_default_output_source_root() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/..");
    assert_default_output_source_root(&request(Some(root)));
}

#[cfg(unix)]
#[test]
fn symlinked_root_preserves_default_output_source_root() {
    let link = env::temp_dir().join(format!("generate-api-root-{}", std::process::id()));
    std::os::unix::fs::symlink(env!("CARGO_MANIFEST_DIR"), &link).unwrap();
    let root = resolve_repository_root(&request(Some(link.clone())));
    fs::remove_file(link).unwrap();

    let root = root.unwrap();
    assert_eq!(root, fs::canonicalize(env!("CARGO_MANIFEST_DIR")).unwrap());
    assert_default_output_source_root(&request(Some(root)));
}

#[test]
fn default_root_is_canonical() {
    let root = resolve_repository_root(&request(None)).unwrap();
    assert_eq!(root, fs::canonicalize(env::current_dir().unwrap()).unwrap());
}

#[test]
fn root_without_manifest_is_rejected() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let error = resolve_repository_root(&request(Some(root))).unwrap_err();
    assert!(error.contains("does not contain Cargo.toml"));
}
