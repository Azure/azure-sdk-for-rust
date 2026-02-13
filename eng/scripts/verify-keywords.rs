#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.114"
---

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const MAX_KEYWORDS: usize = 5;

fn main() {
    let manifest_path = std::env::args()
        .nth(1)
        .or_else(|| {
            find_file(
                std::env::current_dir().expect("current directory"),
                "Cargo.toml",
            )
        })
        .expect("manifest path");

    let package_manifest_path = package_manifest_path(&manifest_path);
    let workspace_manifest_path = workspace_manifest_path();

    let metadata = workspace_metadata(&workspace_manifest_path);

    // Determine which packages to check.
    let packages: Vec<&Package> = if package_manifest_path == workspace_manifest_path {
        metadata.packages.iter().collect()
    } else {
        metadata
            .packages
            .iter()
            .filter(|p| PathBuf::from(&p.manifest_path) == package_manifest_path)
            .collect()
    };

    let mut found = false;
    for package in packages {
        if package.keywords.is_empty() {
            continue;
        }

        eprintln!("Checking keywords for {}", package.name);

        // Check keyword count does not exceed crates.io limit.
        if package.keywords.len() > MAX_KEYWORDS {
            println!(
                "Package `{}` has {} keywords but the maximum allowed is {}:",
                package.name,
                package.keywords.len(),
                MAX_KEYWORDS,
            );
            println!("  keywords: {:?}", package.keywords);
            println!(
                "  Remove keywords to meet the crates.io limit of {} keywords.\n",
                MAX_KEYWORDS
            );
            found = true;
        }
    }

    if found {
        std::process::exit(1);
    }
}

fn package_manifest_path(manifest_path: &str) -> PathBuf {
    let output = Command::new("cargo")
        .args([
            "locate-project",
            "--message-format",
            "plain",
            "--manifest-path",
            manifest_path,
        ])
        .output()
        .expect("executing cargo locate-project");

    let path: PathBuf = String::from_utf8(output.stdout)
        .expect("valid path")
        .trim_end()
        .into();

    if !path.exists() {
        panic!("package manifest not found");
    }

    path
}

fn workspace_manifest_path() -> PathBuf {
    let output = Command::new("cargo")
        .args(["locate-project", "--message-format", "plain", "--workspace"])
        .output()
        .expect("executing cargo locate-project");

    let path: PathBuf = String::from_utf8(output.stdout)
        .expect("valid path")
        .trim_end()
        .into();

    if !path.exists() {
        panic!("workspace manifest not found");
    }

    path
}

fn workspace_metadata(manifest_path: &Path) -> Metadata {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--manifest-path",
            &manifest_path.to_string_lossy(),
        ])
        .output()
        .expect("executing cargo metadata");

    serde_json::from_slice(&output.stdout).expect("bad workspace metadata")
}

fn find_file(dir: impl AsRef<Path>, name: &str) -> Option<String> {
    for dir in dir.as_ref().ancestors() {
        let path = dir.join(name);
        if path.exists() {
            return Some(path.to_str().unwrap().into());
        }
    }
    None
}

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    manifest_path: String,
    #[serde(default)]
    keywords: Vec<String>,
}
