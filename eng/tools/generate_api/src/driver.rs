// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{cli::Request, diagnostics, extract, model::ApiModel};
use rustdoc_types::Crate;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::OsStr,
    fs,
    path::PathBuf,
    process::Command,
    sync::Arc,
};

pub(crate) fn load_model(request: &Request) -> Result<ApiModel, String> {
    let metadata = load_workspace_metadata(request)?;
    let mut loader = ModelLoader::new(metadata.packages);
    Ok((*loader.load_model_for_workspace(&metadata.current_package)?).clone())
}

fn run_command(mut command: Command, error_prefix: &str) -> Result<std::process::Output, String> {
    diagnostics::info(format!(
        "Running command: {} {}",
        command.get_program().to_string_lossy(),
        command
            .get_args()
            .collect::<Vec<&OsStr>>()
            .join(OsStr::new(" "))
            .to_string_lossy(),
    ));

    let output = command
        .output()
        .map_err(|error| format!("{error_prefix}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{error_prefix}: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(output)
}

fn generate_rustdoc_json(package: &PackageMetadata) -> Result<PathBuf, String> {
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
        .arg(&package.manifest_path)
        .arg("--all-features");

    run_command(command, "Failed to generate rustdoc JSON")?;

    Ok(PathBuf::from("target")
        .join("doc")
        .join(format!("{}.json", package.name)))
}

#[derive(Deserialize)]
struct CargoMetadata {
    workspace_members: Vec<String>,
    packages: Vec<CargoPackage>,
}

#[derive(Deserialize)]
struct CargoPackage {
    id: String,
    manifest_path: String,
    version: String,
    name: String,
    targets: Vec<CargoTarget>,
}

#[derive(Deserialize)]
struct CargoTarget {
    name: String,
    kind: Vec<String>,
}

fn load_workspace_metadata(request: &Request) -> Result<WorkspaceMetadata, String> {
    let mut command = Command::new("cargo");
    command
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps")
        .arg("--manifest-path")
        .arg(&request.manifest_path);

    let output = run_command(command, "Failed to run cargo metadata")?;

    let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("Failed to parse cargo metadata JSON: {error}"))?;
    let requested_manifest = std::fs::canonicalize(&request.manifest_path).map_err(|error| {
        format!(
            "Failed to canonicalize manifest path '{}': {error}",
            request.manifest_path.display()
        )
    })?;
    let workspace_members: HashSet<String> = metadata.workspace_members.into_iter().collect();

    let mut workspace_packages = BTreeMap::new();
    let mut current_package = None;
    for package in metadata.packages {
        if !workspace_members.contains(&package.id) {
            continue;
        }

        let manifest_path = std::fs::canonicalize(&package.manifest_path).map_err(|error| {
            format!(
                "Failed to canonicalize manifest path '{}': {error}",
                package.manifest_path
            )
        })?;
        let name = package
            .targets
            .iter()
            .find(|target| target.kind.iter().any(|k| k == "lib"))
            .map(|target| target.name.clone())
            .unwrap_or_else(|| package.name.replace('-', "_"));

        if manifest_path == requested_manifest {
            current_package = Some(name.clone());
        }

        workspace_packages.insert(
            name.clone(),
            PackageMetadata {
                name,
                version: package.version,
                manifest_path,
            },
        );
    }

    let current_package = current_package.ok_or_else(|| {
        format!(
            "cargo metadata did not return a package for manifest '{}'",
            request.manifest_path.display()
        )
    })?;

    Ok(WorkspaceMetadata {
        current_package,
        packages: workspace_packages,
    })
}

struct WorkspaceMetadata {
    current_package: String,
    packages: BTreeMap<String, PackageMetadata>,
}

struct ModelLoader {
    packages: BTreeMap<String, PackageMetadata>,
    crates: HashMap<String, Arc<Crate>>,
    models: HashMap<String, Arc<ApiModel>>,
}

impl ModelLoader {
    fn new(packages: BTreeMap<String, PackageMetadata>) -> Self {
        Self {
            packages,
            crates: HashMap::new(),
            models: HashMap::new(),
        }
    }

    fn load_crate_for_workspace(&mut self, crate_name: &str) -> Result<Arc<Crate>, String> {
        if let Some(krate) = self.crates.get(crate_name) {
            return Ok(Arc::clone(krate));
        }

        let package = self
            .packages
            .get(crate_name)
            .cloned()
            .ok_or_else(|| format!("Unknown workspace crate '{crate_name}'"))?;
        let rustdoc_json_path = generate_rustdoc_json(&package)?;
        diagnostics::info(format!("Reading file: {}", rustdoc_json_path.display()));

        let contents = fs::read_to_string(&rustdoc_json_path).map_err(|error| {
            format!(
                "Failed to read rustdoc JSON '{}': {error}",
                rustdoc_json_path.display()
            )
        })?;
        let krate: Crate = serde_json::from_str(&contents).map_err(|error| {
            format!(
                "Failed to parse rustdoc JSON '{}': {error}",
                rustdoc_json_path.display()
            )
        })?;
        let krate = Arc::new(krate);
        self.crates
            .insert(crate_name.to_string(), Arc::clone(&krate));
        Ok(krate)
    }

    fn load_model_for_workspace(&mut self, crate_name: &str) -> Result<Arc<ApiModel>, String> {
        if let Some(model) = self.models.get(crate_name) {
            return Ok(Arc::clone(model));
        }

        let package = self
            .packages
            .get(crate_name)
            .cloned()
            .ok_or_else(|| format!("Unknown workspace crate '{crate_name}'"))?;
        let krate = self.load_crate_for_workspace(crate_name)?;

        let model = extract::extract_model(&package, &krate, self)?;
        let model = Arc::new(model);
        self.models
            .insert(crate_name.to_string(), Arc::clone(&model));
        Ok(model)
    }
}

impl extract::WorkspaceResolver for ModelLoader {
    fn is_workspace_crate(&self, crate_name: &str) -> bool {
        self.packages.contains_key(crate_name)
    }

    fn load_workspace_model(&mut self, crate_name: &str) -> Result<Option<Arc<ApiModel>>, String> {
        if !self.is_workspace_crate(crate_name) {
            return Ok(None);
        }
        self.load_model_for_workspace(crate_name).map(Some)
    }

    fn load_workspace_crate(&mut self, crate_name: &str) -> Result<Option<Arc<Crate>>, String> {
        if !self.is_workspace_crate(crate_name) {
            return Ok(None);
        }
        self.load_crate_for_workspace(crate_name).map(Some)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PackageMetadata {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) manifest_path: PathBuf,
}
