#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"
description = "In all Cargo.toml files in the repo, for all dependencies that have both path and version properties, update the version property to the version in the package."

[dependencies]
regex = "1.5"
toml_edit = "0.22"
---

use regex::Regex;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, error::Error, fs, path::PathBuf};
use toml_edit::{value, DocumentMut, Item, Table};

static VERBOSE: AtomicBool = AtomicBool::new(false);

macro_rules! verbose {
    ($($arg:tt)*) => {
        if VERBOSE.load(Ordering::Relaxed) {
            eprintln!("[VERBOSE] {}", format!($($arg)*));
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Check for --verbose flag
    let verbose_mode = args.iter().any(|arg| arg == "--verbose" || arg == "-v");
    VERBOSE.store(verbose_mode, Ordering::Relaxed);
    
    // Filter out verbose flag to get mode argument
    let mode_args: Vec<&String> = args.iter()
        .skip(1)
        .filter(|arg| *arg != "--verbose" && *arg != "-v")
        .collect();
    
    let add_mode = mode_args.first()
        .map(|arg| match arg.as_str() {
            "add" => true,
            "update" => false,
            _ => panic!("Invalid mode. Use 'add' or 'update'. Optional: --verbose or -v")
        })
        .expect("requires 'add' or 'update' mode argument. Optional: --verbose or -v");

    verbose!("Mode: {}", if add_mode { "add" } else { "update" });

    let script_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = script_root.join("../..").canonicalize()?;
    verbose!("Repository root: {}", repo_root.display());

    // find all Cargo.toml files in the repo_root directory
    let exclude_dirs = vec![
        repo_root.join("eng"),
        repo_root.join("target")
    ];
    verbose!("Excluded directories: {:?}", exclude_dirs);

    let toml_files = load_cargo_toml_files(&repo_root, &exclude_dirs)?;
    verbose!("Found {} Cargo.toml files", toml_files.len());

    let package_versions = get_package_versions(&toml_files);
    verbose!("Found {} packages with versions", package_versions.len());
    for (name, version, is_publish_disabled) in &package_versions {
        verbose!("  Package: {} @ {} (publish_disabled: {})", name, version, is_publish_disabled);
    }

    verbose!("Starting to process toml files...");
    for mut toml_file in toml_files {
        verbose!("Processing: {}", toml_file.path.display());
        let should_add = add_mode && !toml_file.is_publish_disabled;
        verbose!("  should_add: {} (add_mode: {}, is_publish_disabled: {})", 
            should_add, add_mode, toml_file.is_publish_disabled);

        update_package_versions(toml_file.document.as_table_mut(), &package_versions, should_add, &toml_file.path);

        // if the toml file has a workspace table, update the workspace table
        if let Some(workspace) = toml_file.document.get_mut("workspace") {
            if let Some(table) = workspace.as_table_mut() {
                verbose!("  Processing workspace table");
                update_package_versions(table, &package_versions, should_add, &toml_file.path);
            }
        }

        // write the updated document back to the file
        verbose!("  Writing changes to file");
        let mut file = fs::File::create(&toml_file.path)?;
        fs::File::write_all(&mut file, toml_file.document.to_string().as_bytes())?;
    }

    verbose!("Done!");
    Ok(())
}

fn load_cargo_toml_files(repo_root: &PathBuf, exclude_dirs: &Vec<PathBuf>) -> Result<Vec<TomlInfo>, Box<dyn Error>> {
    let mut toml_paths = Vec::new();
    verbose!("Searching for Cargo.toml files in: {}", repo_root.display());
    find_cargo_toml_files(repo_root, exclude_dirs, &mut toml_paths)?;
    verbose!("Found {} Cargo.toml paths", toml_paths.len());

    let mut toml_files = Vec::new();
    for path in toml_paths {
        verbose!("  Loading: {}", path.display());
        let content = fs::read_to_string(&path)?;
        let doc = content.parse::<DocumentMut>()?;
        let package_table = doc.get("package").and_then(Item::as_table);
        let publish_property = package_table.and_then(|table| table.get("publish")).and_then(Item::as_bool);
        let package_name = package_table.and_then(|table| table.get("name")).and_then(Item::as_str);
        let package_version = package_table.and_then(|table| table.get("version")).and_then(Item::as_str);

        verbose!("    name: {:?}, version: {:?}, publish: {:?}", 
            package_name, package_version, publish_property);

        toml_files.push(TomlInfo {
            path,
            package_name: package_name.map(|s| s.to_string()),
            package_version: package_version.map(|s| s.to_string()),
            is_publish_disabled: publish_property == Some(false),
            document: doc
        });
    }

    Ok(toml_files)
}

fn find_cargo_toml_files(dir: &PathBuf, exclude_dirs: &Vec<PathBuf>, toml_paths: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    verbose!("  Scanning directory: {}", dir.display());
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if exclude_dirs.contains(&path) {
                verbose!("    Skipping excluded dir: {}", path.display());
            } else {
                find_cargo_toml_files(&path, exclude_dirs, toml_paths)?;
            }
        } else if path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
            verbose!("    Found Cargo.toml: {}", path.display());
            toml_paths.push(path);
        }
    }

    Ok(())
}

fn get_package_versions(toml_files: &Vec<TomlInfo>) -> Vec<(String, String, bool)> {
    let mut package_versions = Vec::new();

    for toml_file in toml_files {
        if toml_file.package_name.is_none() || toml_file.package_version.is_none() {
            continue;
        }

        package_versions.push((toml_file.package_name.clone().unwrap(), toml_file.package_version.clone().unwrap(), toml_file.is_publish_disabled));
    }

    package_versions
}

fn update_package_versions(toml: &mut Table, package_versions: &Vec<(String, String, bool)>, add: bool, file_path: &PathBuf) {
    // for each dependency table, for each package in package_versions
    // if the package is in the dependency table
    //   if the dependency has both path and version properties, update the version property
    //   if the dependency has has path, but not version, add the version property only if
    //     1. the table name is not "dev-dependencies"
    //     2. the package is not publish disabled
    //     3. the add flag is true

    let dependency_tables = get_dependency_tables(toml);
    verbose!("  Found {} dependency tables", dependency_tables.len());

    for (table_name, table) in dependency_tables {
        verbose!("    Processing table: {}", table_name);
        for (package, version, is_publish_disabled) in package_versions {
            if let Some(dependency) = table.get_mut(package) {
                // azure_identity will only be a transitive dev-dependency
                let should_add = add && table_name != "dev-dependencies" && !is_publish_disabled && package != "azure_identity";

                let has_path_property = dependency.get("path").is_some();
                let has_version_property = dependency.get("version").is_some();

                verbose!("      Checking package '{}': has_path={}, has_version={}, should_add={}", 
                    package, has_path_property, has_version_property, should_add);

                if has_path_property && (has_version_property || should_add) {
                    verbose!("        Updating '{}' version to '{}' in {}", package, version, file_path.display());
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
                verbose!("      Found dependency table: {}", key);
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
