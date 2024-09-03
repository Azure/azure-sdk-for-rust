// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::find_file;
use cargo_util_schemas::manifest::{InheritableDependency, TomlDependency, TomlManifest};
use serde::Deserialize;
use std::{
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn run(args: impl Iterator<Item = String>) {
    let manifest_path = args
        .into_iter()
        .next()
        .or_else(|| {
            find_file(
                std::env::current_dir().expect("current directory"),
                "Cargo.toml",
            )
        })
        .expect("manifest path");

    let package_manifest_path = package_manifest_path(&manifest_path);
    let workspace_manifest_path = workspace_manifest_path(&manifest_path);

    eprintln!("Checking {}", package_manifest_path.display());
    let package_manifest_content =
        std::fs::read_to_string(package_manifest_path).expect("read package manifest");
    let package_manifest: TomlManifest =
        toml::from_str(&package_manifest_content).expect("deserialize package manifest");

    // Collect all package dependencies including in platform targets.
    let mut all_dependencies = vec![
        (
            "dependencies".to_string(),
            package_manifest.dependencies.as_ref(),
        ),
        (
            "dev-dependencies".to_string(),
            package_manifest.dev_dependencies(),
        ),
        (
            "build-dependencies".to_string(),
            package_manifest.build_dependencies(),
        ),
    ];
    if let Some(targets) = package_manifest.target.as_ref() {
        for (target, platform) in targets {
            all_dependencies.push((
                format!("target.'{}'.dependencies", target),
                platform.dependencies.as_ref(),
            ));
            all_dependencies.push((
                format!("target.'{}'.dev-dependencies", target),
                platform.dev_dependencies(),
            ));
            all_dependencies.push((
                format!("target.'{}'.build-dependencies", target),
                platform.build_dependencies(),
            ));
        }
    }

    let mut dependencies: Vec<Package> = all_dependencies
        .into_iter()
        .filter_map(|v| {
            if let Some(dependencies) = v.1 {
                return Some((v.0, dependencies));
            }
            None
        })
        .flat_map(|v| std::iter::repeat(v.0).zip(v.1.iter()))
        .filter_map(|v| match v.1 .1 {
            InheritableDependency::Value(dep) => match dep {
                TomlDependency::Simple(_) => Some(Package {
                    section: v.0,
                    name: v.1 .0.to_string(),
                }),
                TomlDependency::Detailed(details) if details.path.is_none() => Some(Package {
                    section: v.0,
                    name: v.1 .0.to_string(),
                }),
                _ => None,
            },
            InheritableDependency::Inherit(_) => None,
        })
        .collect();

    if !dependencies.is_empty() {
        dependencies.sort();
        println!(
            "The following dependencies do not inherit from workspace `{}`:\n",
            workspace_manifest_path.display()
        );
        println!(
            "* {}\n",
            dependencies
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("\n* ")
        );
        println!("Add dependencies to workspace and change the package dependency to `{{ workspace = true }}`.");
        println!("See <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace> for more information.");

        std::process::exit(1);
    }
}

fn package_manifest_path(manifest_path: &str) -> PathBuf {
    let mut cmd = Command::new("cargo")
        .args(["read-manifest", "--manifest-path", manifest_path])
        .stdout(Stdio::piped())
        .spawn()
        .expect("executing cargo read-manifest");

    let path: PathBuf = read_manifest(&mut cmd)
        .manifest_path
        .expect("manifest_path")
        .into();

    if !path.exists() {
        panic!("package manifest not found");
    }

    path
}

fn workspace_manifest_path(manifest_path: &str) -> PathBuf {
    let mut cmd = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--all-features",
            "--manifest-path",
            manifest_path,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("executing cargo metadata");

    let path: PathBuf = read_manifest(&mut cmd)
        .workspace_root
        .expect("workspace_root")
        .into();
    let path = path.join("Cargo.toml");

    if !path.exists() {
        panic!("workspace manifest not found");
    }

    path
}

fn read_manifest(cmd: &mut std::process::Child) -> CargoManifest {
    let mut reader = std::io::BufReader::new(cmd.stdout.take().expect("buffering stdout"));

    let mut content: String = Default::default();
    reader.read_to_string(&mut content).expect("reading stdout");

    serde_json::from_str::<CargoManifest>(&content).expect("deserializing manifest")
}

#[derive(Deserialize)]
struct CargoManifest {
    pub manifest_path: Option<String>,
    pub workspace_root: Option<String>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Package {
    name: String,
    section: String,
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.section)
    }
}
