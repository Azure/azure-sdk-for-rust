#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"
description = "In all Cargo.toml files in the repo, for all dependencies that have both path and version properties, update the version property to the version in the package."

[dependencies]
regex = "1.5"
semver = "1.0"
toml_edit = "0.22"
---

use regex::Regex;
use semver::Version;
use std::collections::HashMap;
use std::io::Write;
use std::{env, error::Error, fs, path::PathBuf};
use toml_edit::{value, DocumentMut, Item, Table};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add_mode = env::args()
        .nth(1)
        .map(|arg| match arg.as_str() {
            "add" => true,
            "update" => false,
            _ => panic!("Invalid mode. Use 'add' or 'update'."),
        })
        .expect("requires 'add' or 'update' mode argument");

    let script_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = script_root.join("../../..").canonicalize()?;

    // find all Cargo.toml files in the repo_root directory
    let exclude_dirs = vec![repo_root.join("eng"), repo_root.join("target")];

    let toml_files = load_cargo_toml_files(&repo_root, &exclude_dirs)?;

    let package_versions = get_package_versions(&toml_files);

    for mut toml_file in toml_files {
        let should_add = add_mode && !toml_file.is_publish_disabled;

        update_package_versions(
            toml_file.document.as_table_mut(),
            &package_versions,
            should_add,
        );

        // if the toml file has a workspace table, update the workspace table
        if let Some(workspace) = toml_file.document.get_mut("workspace") {
            if let Some(table) = workspace.as_table_mut() {
                update_package_versions(table, &package_versions, should_add);
            }
        }

        // write the updated document back to the file
        let mut file = fs::File::create(toml_file.path)?;
        fs::File::write_all(&mut file, toml_file.document.to_string().as_bytes())?;
    }

    Ok(())
}

fn load_cargo_toml_files(
    repo_root: &PathBuf,
    exclude_dirs: &Vec<PathBuf>,
) -> Result<Vec<TomlInfo>, Box<dyn Error>> {
    let mut toml_paths = Vec::new();
    find_cargo_toml_files(repo_root, exclude_dirs, &mut toml_paths)?;

    let mut toml_files = Vec::new();
    for path in toml_paths {
        let content = fs::read_to_string(&path)?;
        let doc = content.parse::<DocumentMut>()?;
        let package_table = doc.get("package").and_then(Item::as_table);
        let publish_property = package_table
            .and_then(|table| table.get("publish"))
            .and_then(Item::as_bool);
        let package_name = package_table
            .and_then(|table| table.get("name"))
            .and_then(Item::as_str);
        let package_version = package_table
            .and_then(|table| table.get("version"))
            .and_then(Item::as_str);

        toml_files.push(TomlInfo {
            path,
            package_name: package_name.map(|s| s.to_string()),
            package_version: package_version.map(|s| s.to_string()),
            is_publish_disabled: publish_property == Some(false),
            document: doc,
        });
    }

    Ok(toml_files)
}

fn find_cargo_toml_files(
    dir: &PathBuf,
    exclude_dirs: &Vec<PathBuf>,
    toml_paths: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !exclude_dirs.contains(&path) {
            find_cargo_toml_files(&path, exclude_dirs, toml_paths)?;
        } else if path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
            toml_paths.push(path);
        }
    }

    Ok(())
}

fn get_package_versions(toml_files: &Vec<TomlInfo>) -> Vec<(String, String, bool)> {
    // Use a HashMap to deduplicate package versions.
    // When there are multiple Cargo.toml files with the same package name but different versions
    // (e.g., test fixtures, examples, or stale files), we need to pick the correct version.
    let mut package_map: HashMap<String, (String, bool)> = HashMap::new();

    for toml_file in toml_files {
        if toml_file.package_name.is_none() || toml_file.package_version.is_none() {
            continue;
        }

        let name = toml_file.package_name.clone().unwrap();
        let version = toml_file.package_version.clone().unwrap();
        let is_publish_disabled = toml_file.is_publish_disabled;

        // Determine if we should use this version
        let should_use = match package_map.get(&name) {
            None => true, // First occurrence, always use it
            Some((existing_version, existing_publish_disabled)) => {
                // Prefer publishable packages over non-publishable ones
                if *existing_publish_disabled && !is_publish_disabled {
                    true
                } else if !*existing_publish_disabled && is_publish_disabled {
                    false
                } else {
                    // Both have the same publish status, prefer the higher version
                    match (Version::parse(&version), Version::parse(existing_version)) {
                        (Ok(new_ver), Ok(existing_ver)) => new_ver > existing_ver,
                        _ => false, // If parsing fails, keep existing
                    }
                }
            }
        };

        if should_use {
            package_map.insert(name, (version, is_publish_disabled));
        }
    }

    // Convert HashMap back to Vec for compatibility with existing code
    package_map
        .into_iter()
        .map(|(name, (version, is_publish_disabled))| (name, version, is_publish_disabled))
        .collect()
}

fn update_package_versions(
    toml: &mut Table,
    package_versions: &Vec<(String, String, bool)>,
    add: bool,
) {
    // for each dependency table, for each package in package_versions
    // if the package is in the dependency table
    //   if the dependency has both path and version properties, update the version property
    //   if the dependency has has path, but not version, add the version property only if
    //     1. the table name is not "dev-dependencies"
    //     2. the package is not publish disabled
    //     3. the add flag is true

    let dependency_tables = get_dependency_tables(toml);

    for (table_name, table) in dependency_tables {
        for (package, version, is_publish_disabled) in package_versions {
            if let Some(dependency) = table.get_mut(package) {
                // azure_idenentity will only be a transitive dev-dependency
                let should_add = add
                    && table_name != "dev-dependencies"
                    && !is_publish_disabled
                    && package != "azure_identity";

                let has_path_property = dependency.get("path").is_some();
                let has_version_property = dependency.get("version").is_some();

                if has_path_property && (has_version_property || should_add) {
                    dependency["version"] = value(version);
                }
            }
        }
    }
}

fn get_dependency_tables(toml: &mut Table) -> Vec<(String, &mut Table)> {
    let re = Regex::new(r"[.-]?dependencies$").unwrap();
    let mut tables = Vec::new();

    for (key, value) in toml.iter_mut() {
        if let Some(table) = value.as_table_mut() {
            if re.is_match(&key) {
                tables.push((key.to_string(), table));
            }
        }
    }

    tables
}

struct TomlInfo {
    path: PathBuf,
    package_name: Option<String>,
    package_version: Option<String>,
    is_publish_disabled: bool,
    document: DocumentMut,
}
