#!/usr/bin/env -S cargo +nightly-2026-04-14 -Zscript
---
[package]
edition = "2021"

[dependencies]
cargo-util-schemas = "0.1.0"
serde = { version = "1.0.197", features = ["derive"] }
toml = "0.8.10"
---

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use cargo_util_schemas::manifest::{InheritableDependency, TomlManifest};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashSet},
    ffi::OsStr,
    fs::{self, DirEntry},
    io::{self, Write as _},
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Dependencies {
    dependencies: Option<BTreeMap<String, InheritableDependency>>,
    dev_dependencies: Option<BTreeMap<String, InheritableDependency>>,
    build_dependencies: Option<BTreeMap<String, InheritableDependency>>,
}

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
        let manifest: Dependencies = toml::from_str(&manifest)?;

        add_dependencies(&mut crate_names, manifest);

        Ok(())
    })
    .expect("expected Cargo.toml manifest");

    for entry in fs::read_dir(workspace_root.join("eng/scripts")).expect("read scripts directory") {
        let entry = entry.expect("read script entry");
        let path = entry.path();
        if !entry.file_type().expect("script file type").is_file()
            || path.extension() != Some(OsStr::new("rs"))
        {
            continue;
        }

        eprintln!("Reading {path:?}");
        let script = fs::read_to_string(&path).expect("script content");
        let script_name = path
            .file_stem()
            .and_then(OsStr::to_str)
            .expect("expected script name");
        let manifest = script_manifest(&script).expect("deserialize script manifest");
        crate_names.insert(script_name.to_string());
        add_dependencies(&mut crate_names, manifest);
    }

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

fn add_dependencies(crate_names: &mut HashSet<String>, manifest: Dependencies) {
    for dependencies in [
        manifest.dependencies,
        manifest.dev_dependencies,
        manifest.build_dependencies,
    ]
    .into_iter()
    .flatten()
    {
        crate_names.extend(dependencies.into_iter().filter_map(|(name, dependency)| {
            (!matches!(dependency, InheritableDependency::Inherit(_))).then_some(name.to_string())
        }));
    }
}

fn script_manifest(script: &str) -> Result<Dependencies, Box<dyn std::error::Error + Send + Sync>> {
    let mut lines = script.lines();
    if lines.find(|line| *line == "---").is_some() {
        let mut manifest = String::new();
        for line in lines {
            if line == "---" {
                return Ok(toml::from_str(&manifest)?);
            }
            manifest.push_str(line);
            manifest.push('\n');
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "missing script manifest delimiters",
    )
    .into())
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

#[cfg(test)]
mod tests {
    use super::{add_dependencies, script_manifest};
    use std::collections::HashSet;

    #[test]
    fn script_dependencies() {
        let script = "#!/usr/bin/env -S cargo -Zscript\n---\n\
            [package]\nedition = \"2021\"\n\
            [dependencies]\ncargo-util-schemas = \"0.1\"\n\
            serde = { workspace = true }\n\
            [dev-dependencies]\ntoml_edit = { version = \"0.22\" }\n\
            [build-dependencies]\nsyn = \"2\"\n---\nfn main() {}\n";
        for script in [script.to_string(), script.replace('\n', "\r\n")] {
            let mut names = HashSet::new();
            let manifest = script_manifest(&script).unwrap();
            add_dependencies(&mut names, manifest);
            assert_eq!(
                names,
                HashSet::from([
                    "cargo-util-schemas".to_string(),
                    "toml_edit".to_string(),
                    "syn".to_string()
                ])
            );
        }
    }

    #[test]
    fn script_manifest_without_package() {
        let manifest =
            script_manifest("---\n[dev-dependencies]\ntoml = \"0.8\"\n---\nfn main() {}\n")
                .unwrap();
        let mut names = HashSet::new();
        add_dependencies(&mut names, manifest);
        assert_eq!(names, HashSet::from(["toml".to_string()]));
    }

    #[test]
    fn script_manifest_ignores_unrelated_tables() {
        let manifest =
            script_manifest("---\n[package]\nname = 42\n[unrelated]\nvalue = true\n---\n").unwrap();
        let mut names = HashSet::new();
        add_dependencies(&mut names, manifest);
        assert!(names.is_empty());
    }

    #[test]
    fn script_manifest_requires_delimiters() {
        assert!(script_manifest("fn main() {}\n").is_err());
        assert!(script_manifest("---\n[dependencies]\n").is_err());
        assert!(script_manifest("---\nnot valid toml\n---\n").is_err());
        assert!(script_manifest("---\n[dependencies]\ntoml = 42\n---\n").is_err());
    }
}
