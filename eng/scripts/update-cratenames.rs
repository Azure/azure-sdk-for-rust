#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
cargo-util-schemas = "0.1.0"
toml = "0.8.10"
---

use cargo_util_schemas::manifest::TomlManifest;
use std::{collections::HashSet, ffi::OsStr, fs, io::Write as _, path::PathBuf};

fn main() {
    let workspace_root = get_workspace_root();
    let workspace_manifest_path = workspace_root.join("Cargo.toml");
    eprintln!("Reading {workspace_manifest_path:?}");

    let workspace_manifest =
        fs::read_to_string(workspace_root.join("Cargo.toml")).expect("workspace manifest content");
    let workspace_manifest: TomlManifest =
        toml::from_str(&workspace_manifest).expect("deserialize workspace manifest");

    // Extract dependencies.
    let dependencies = workspace_manifest
        .workspace
        .as_ref()
        .expect("expected workspace")
        .dependencies
        .as_ref()
        .expect("expected workspace dependencies");
    let mut crate_names: HashSet<String> = dependencies
        .iter()
        .map(|(name, _)| name.to_string())
        .collect();

    // Extract workspace members.
    let members = workspace_manifest
        .workspace
        .as_ref()
        .expect("expected workspace")
        .members
        .as_ref()
        .expect("expected workspace members");

    for relative_path in members.into_iter() {
        let member_dir_name = PathBuf::from(relative_path)
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .expect("member directory name");
        let member_manifest_path = workspace_root.join(relative_path).join("Cargo.toml");
        eprintln!("Reading member manifest {member_manifest_path:?}");
        let member_manifest = fs::read_to_string(&member_manifest_path)
            .unwrap_or_else(|_| panic!("member manifest content: {member_manifest_path:?}"));
        let member_manifest: TomlManifest = toml::from_str(&member_manifest)
            .unwrap_or_else(|_| panic!("deserialize member manifest: {member_manifest_path:?}"));

        let sections = [
            member_manifest.dependencies,
            member_manifest.dev_dependencies,
            member_manifest.build_dependencies,
        ];
        for section in sections.iter().flatten() {
            for (name, _) in section.iter() {
                crate_names.insert(name.to_string());
            }
        }

        // Add the crate name for this member.
        if let Some(name) = member_manifest.package.as_ref().map(|p| p.name.as_ref()) {
            crate_names.insert(name.to_string());
        } else {
            crate_names.insert(member_dir_name.to_string());
        }
    }

    let mut crate_names: Vec<String> = crate_names.into_iter().collect();
    crate_names.sort();
    let crate_names_path = workspace_root
        .join("eng/dict/crates.txt")
        .canonicalize()
        .expect("canonical crate names path");

    eprintln!(
        "Writing {} crate names to {crate_names_path:?}",
        crate_names.len()
    );

    let mut f = fs::File::create(crate_names_path).expect("create eng/dict/crates.txt");
    writeln!(f, "{}", crate_names.join("\n")).expect("serialize crate names");
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
