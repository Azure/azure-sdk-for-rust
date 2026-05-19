// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use rustdoc_types::Crate;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Verify we're running from the repository root
    if !Path::new("eng/tools/generate_api_report").exists() {
        return Err(
            "This tool must be run from the root of the azure-sdk-for-rust repository. Use: cargo run --manifest-path eng/tools/generate_api_report/Cargo.toml -- --package {package_name}".into(),
        );
    }

    // Parse command-line arguments: --package <name> [--manifest-path <path>]
    let args: Vec<String> = env::args().collect();
    let mut package_name: Option<String> = None;
    let mut manifest_path: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--package" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("--package requires a value");
                    std::process::exit(1);
                }
                if package_name.is_some() {
                    eprintln!("--package specified more than once");
                    std::process::exit(1);
                }
                package_name = Some(args[i].clone());
            }
            "--manifest-path" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("--manifest-path requires a value");
                    std::process::exit(1);
                }
                if manifest_path.is_some() {
                    eprintln!("--manifest-path specified more than once");
                    std::process::exit(1);
                }
                manifest_path = Some(args[i].clone());
            }
            arg => {
                eprintln!("Unknown argument: {}", arg);
                eprintln!(
                    "Usage: {} --package <package_name> [--manifest-path <path>]",
                    args[0]
                );
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let package_name = match package_name {
        Some(name) => name,
        None => {
            eprintln!(
                "Usage: {} --package <package_name> [--manifest-path <path>]",
                args[0]
            );
            std::process::exit(1);
        }
    };

    // Resolve the manifest path, querying cargo metadata if not provided.
    let (manifest_path, crate_name) = if let Some(path) = manifest_path {
        let crate_name = package_name.replace('-', "_");
        (path, crate_name)
    } else {
        let mut command = Command::new("cargo");
        command
            .arg("metadata")
            .arg("--format-version")
            .arg("1")
            .arg("--no-deps")
            .arg("--manifest-path")
            .arg("Cargo.toml");

        println!(
            "Getting manifest path for '{package_name}': {} {}",
            command.get_program().to_string_lossy(),
            command
                .get_args()
                .collect::<Vec<&OsStr>>()
                .join(OsStr::new(" "))
                .to_string_lossy(),
        );

        let output = command.output()?;
        if !output.status.success() {
            eprintln!(
                "Failed to run cargo metadata: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            std::process::exit(1);
        }

        let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        let pkg = metadata["packages"]
            .as_array()
            .and_then(|packages| {
                packages
                    .iter()
                    .find(|pkg| pkg["name"].as_str() == Some(package_name.as_str()))
            })
            .ok_or_else(|| {
                format!(
                    "Package '{}' not found in cargo metadata output",
                    package_name
                )
            })?;

        let path = pkg["manifest_path"]
            .as_str()
            .ok_or("manifest_path not found in cargo metadata output")?
            .to_string();

        // Derive the lib crate name from the lib target, falling back to the package name.
        let crate_name = pkg["targets"]
            .as_array()
            .and_then(|targets| {
                targets.iter().find_map(|t| {
                    let is_lib = t["kind"]
                        .as_array()
                        .map(|k| k.iter().any(|v| v.as_str() == Some("lib")))
                        .unwrap_or(false);
                    if is_lib {
                        t["name"].as_str().map(|s| s.replace('-', "_"))
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_else(|| package_name.replace('-', "_"));

        (path, crate_name)
    };

    let path_str = format!("./target/doc/{}.json", crate_name);
    let path = Path::new(&path_str);

    // Call cargo +nightly rustdoc to generate the JSON file
    let channel = env!("TOOLCHAIN_CHANNEL");
    let mut command = Command::new("cargo");
    command
        .arg(format!("+{channel}"))
        .arg("rustdoc")
        .arg("-Z")
        .arg("unstable-options")
        .arg("--output-format")
        .arg("json")
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--all-features");
    println!(
        "Running: {} {}",
        command.get_program().to_string_lossy(),
        command
            .get_args()
            .collect::<Vec<&OsStr>>()
            .join(OsStr::new(" "))
            .to_string_lossy(),
    );
    let output = command.output()?;

    if !output.status.success() {
        eprintln!(
            "Failed to generate JSON file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Processing rustdoc output for package: {}", package_name);

    let mut root: Crate = serde_json::from_str(&contents)?;

    // Remove items
    // 1. with item.inner.impl.is_synthetic set to true - [auto traits]
    // 2. with item.inner.impl.blanket_impl is not null - [blanket impls]
    root.index.retain(|_id, item| {
        if let rustdoc_types::ItemEnum::Impl(impl_item) = &item.inner {
            // Filter out auto traits
            if impl_item.is_synthetic {
                return false;
            }
            // Filter out blanket implementations
            if impl_item.blanket_impl.is_some() {
                return false;
            }
        }
        true
    });

    // Clear unnecessary fields in the Crate structure
    // 1. external_crates
    // 2. span in all items
    root.external_crates.clear();
    for (_id, item) in root.index.iter_mut() {
        // Reset span to default empty value
        item.span = Default::default();
    }

    // Derive the package directory from the manifest path.
    let package_dir = Path::new(&manifest_path)
        .parent()
        .ok_or("Cannot determine package directory from manifest path")?;

    // Create the review/ folder under the obtained path if it doesn't exist
    let review_folder_path = package_dir.join("review");
    if !review_folder_path.exists() {
        std::fs::create_dir_all(&review_folder_path)?;
    }

    // Create the package_name.rust.json in the review/ folder
    let output_path_str = review_folder_path.join(format!("{}.rust.json", package_name));
    let output_path = Path::new(&output_path_str);
    let mut output_file = File::create(output_path)?;
    serde_json::to_writer(&mut output_file, &root)?;

    println!("File has been generated at: {}", output_path_str.display());

    Ok(())
}
