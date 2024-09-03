// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use cargo_util_schemas::manifest::TomlManifest;
use std::{fs, path::PathBuf};

pub fn run() {
    let workspace_root = get_workspace_root();
    let workspace_manifest_path = workspace_root.join("Cargo.toml");
    eprintln!("Reading {workspace_manifest_path:?}");

    let workspace_manifest =
        fs::read_to_string(workspace_root.join("Cargo.toml")).expect("workspace manifest content");
    let workspace_manifest: TomlManifest =
        toml::from_str(&workspace_manifest).expect("deserialize workspace manifest");

    let dependencies = workspace_manifest
        .workspace
        .expect("expected workspace")
        .dependencies
        .expect("expected workspace dependencies");
    let crate_names: Vec<&str> = dependencies.keys().map(|name| name.as_str()).collect();

    let crate_names_path = workspace_root
        .join("eng/dict/crates.txt")
        .canonicalize()
        .expect("canonical crate names path");

    eprintln!(
        "Writing {} crate names to {crate_names_path:?}",
        crate_names.len()
    );
    fs::write(crate_names_path, crate_names.join("\n")).expect("serialize crate names")
}

fn get_workspace_root() -> PathBuf {
    for dir in std::env::current_dir()
        .expect("current directory")
        .ancestors()
    {
        let path = dir.join("Cargo.toml");
        if path.exists() {
            return dir.into();
        }
    }
    panic!("Cargo.toml not found in parent directories");
}
