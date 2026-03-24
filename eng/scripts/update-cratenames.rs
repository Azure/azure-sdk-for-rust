#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
cargo-util-schemas = "0.1.0"
toml = "0.8.10"
---

use cargo_util_schemas::manifest::{InheritableDependency, TomlManifest};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{self, DirEntry},
    io::{self, Write as _},
    path::{Path, PathBuf},
};

fn main() {
    let mut crate_names: HashSet<String> = HashSet::new();
    let workspace_root = get_workspace_root();

    let workspace_manifest_path = workspace_root.join("Cargo.toml");
    eprintln!("Reading {workspace_manifest_path:?}");

    let workspace_manifest =
        fs::read_to_string(workspace_root.join("Cargo.toml")).expect("workspace manifest content");
    let workspace_manifest: TomlManifest =
        toml::from_str(&workspace_manifest).expect("deserialize workspace manifest");

    // Extract workspace members.
    for relative_path in workspace_manifest
        .workspace
        .as_ref()
        .expect("expected workspace")
        .members
        .as_ref()
        .expect("expected workspace members")
        .iter()
    {
        let crate_name = PathBuf::from(relative_path)
            .file_stem()
            .and_then(OsStr::to_str)
            .expect("expected crate name")
            .to_string();
        crate_names.insert(crate_name);
    }

    // Extract all workspace dependencies.
    for dependency_name in workspace_manifest
        .workspace
        .as_ref()
        .expect("expected workspace")
        .dependencies
        .as_ref()
        .expect("expected workspace dependencies")
        .iter()
        .map(|(name, _)| name.to_string())
    {
        crate_names.insert(dependency_name);
    }

    // Extract dependencies from all SDKs' Cargo.toml manifests.
    find(&workspace_root, &mut |file: &DirEntry| {
        if file.file_name() != "Cargo.toml" {
            return Ok(());
        }

        let manifest_path = file.path();
        eprintln!("Reading {manifest_path:?}");

        let manifest = fs::read_to_string(manifest_path)?;
        let manifest: TomlManifest = toml::from_str(&manifest)?;

        let Some(mut dependencies) = manifest.dependencies else {
            return Ok(());
        };
        if let Some(dev_dependencies) = manifest.dev_dependencies {
            dependencies.extend(dev_dependencies);
        }
        if let Some(build_dependencies) = manifest.build_dependencies {
            dependencies.extend(build_dependencies);
        }

        let dependencies: Vec<String> = dependencies
            .iter()
            .filter_map(|(name, dependency)| {
                if matches!(dependency, InheritableDependency::Inherit(_)) {
                    return None;
                }
                Some(name.to_string())
            })
            .collect();
        crate_names.extend(dependencies);

        Ok(())
    })
    .expect("expected Cargo.toml manifest");

    // Sort and write the crate names along with import names for crates with "-" in the name.
    let mut crate_names: Vec<String> = crate_names.into_iter().collect();
    let import_names: Vec<String> = crate_names
        .iter()
        .filter_map(|name| {
            if name.contains("-") {
                return Some(name.replace("-", "_"));
            }

            None
        })
        .collect();
    crate_names.extend(import_names);
    // Sort using the same order as VSCode "Sort lines ascending" (and cSpell),
    // where '_' sorts before '-' (opposite of standard ASCII order).
    let sort_key = |c: char| match c {
        '-' => '_',
        '_' => '-',
        c => c,
    };
    crate_names.sort_by(|a, b| a.chars().map(sort_key).cmp(b.chars().map(sort_key)));

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

fn find<F>(dir: &Path, f: &mut F) -> io::Result<()>
where
    F: FnMut(&DirEntry) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if name == "target" || name.as_encoded_bytes().starts_with(&[b'.']) {
                        continue;
                    }
                }
                find(&path, f)?;
            } else {
                f(&entry).map_err(io::Error::other)?;
            }
        }
    }

    Ok(())
}
