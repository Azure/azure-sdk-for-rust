// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    cli::Request,
    diagnostics, extract,
    model::{ApiModel, PackageMetadata as ApiPackageMetadata},
};
use rustdoc_types::{Crate, FORMAT_VERSION};
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
        .args(package.rustdoc_selector_args())
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
    description: Option<String>,
    edition: Option<String>,
    rust_version: Option<String>,
    #[serde(default)]
    features: BTreeMap<String, Vec<String>>,
    metadata: Option<CargoPackageMetadata>,
    targets: Vec<CargoTarget>,
}

#[derive(Default, Deserialize)]
struct CargoPackageMetadata {
    #[serde(default)]
    docs: CargoDocsMetadata,
}

#[derive(Default, Deserialize)]
struct CargoDocsMetadata {
    #[serde(default)]
    rs: CargoDocsRsMetadata,
}

#[derive(Default, Deserialize)]
struct CargoDocsRsMetadata {
    features: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct CargoTarget {
    name: String,
    kind: Vec<String>,
}

fn is_library_target_kind(kind: &str) -> bool {
    matches!(
        kind,
        "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro"
    )
}

fn crate_target_name(package_name: &str, targets: &[CargoTarget]) -> String {
    targets
        .iter()
        .find(|target| target.kind.iter().any(|kind| is_library_target_kind(kind)))
        .map(|target| target.name.clone())
        .unwrap_or_else(|| package_name.replace('-', "_"))
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
        let name = crate_target_name(&package.name, &package.targets);

        if manifest_path == requested_manifest {
            current_package = Some(name.clone());
        }

        let has_library_target = package
            .targets
            .iter()
            .any(|target| target.kind.iter().any(|kind| is_library_target_kind(kind)));

        let features = select_features(
            package.features,
            package
                .metadata
                .as_ref()
                .and_then(|metadata| metadata.docs.rs.features.as_deref()),
        );
        workspace_packages.insert(
            name.clone(),
            PackageMetadata {
                name,
                version: package.version,
                manifest_path,
                has_library_target,
                api: ApiPackageMetadata {
                    description: package.description,
                    edition: package.edition,
                    rust_version: package.rust_version,
                    features,
                },
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

fn select_features(
    features: BTreeMap<String, Vec<String>>,
    docs_rs_features: Option<&[String]>,
) -> BTreeMap<String, Vec<String>> {
    if features.is_empty() {
        return BTreeMap::from([("default".to_string(), Vec::new())]);
    }

    match docs_rs_features {
        Some(visible_features) => {
            let mut selected = BTreeMap::new();
            if let Some(enabled) = features.get("default") {
                selected.insert("default".to_string(), enabled.clone());
            }
            selected.extend(visible_features.iter().filter_map(|feature| {
                features
                    .get(feature)
                    .map(|enabled| (feature.clone(), enabled.clone()))
            }));
            selected
        }
        None => features,
    }
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
        let format: RustdocFormat = serde_json::from_str(&contents).map_err(|error| {
            format!(
                "Failed to read rustdoc JSON format version from '{}': {error}",
                rustdoc_json_path.display()
            )
        })?;
        if format.format_version != FORMAT_VERSION {
            return Err(format!(
                "Unsupported rustdoc JSON format {} in '{}'; expected {}",
                format.format_version,
                rustdoc_json_path.display(),
                FORMAT_VERSION
            ));
        }

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

#[derive(Deserialize)]
struct RustdocFormat {
    format_version: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct PackageMetadata {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) manifest_path: PathBuf,
    pub(crate) has_library_target: bool,
    pub(crate) api: ApiPackageMetadata,
}

impl PackageMetadata {
    fn rustdoc_selector_args(&self) -> &[&str] {
        if self.has_library_target {
            &["--lib"]
        } else {
            &[]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{crate_target_name, select_features, CargoPackage, CargoTarget};
    use std::{collections::BTreeMap, path::PathBuf};

    #[test]
    fn prefers_library_like_target_names() {
        assert_eq!(
            crate_target_name(
                "azure_data_cosmos_driver_native",
                &[CargoTarget {
                    name: "azurecosmosdriver".to_string(),
                    kind: vec!["cdylib".to_string(), "staticlib".to_string()],
                }],
            ),
            "azurecosmosdriver"
        );
        assert_eq!(
            crate_target_name(
                "typespec_macros",
                &[CargoTarget {
                    name: "typespec_macros".to_string(),
                    kind: vec!["proc-macro".to_string()],
                }],
            ),
            "typespec_macros"
        );
    }

    #[test]
    fn falls_back_to_package_name_when_no_library_like_target_exists() {
        assert_eq!(
            crate_target_name(
                "azure-data-cosmos-benchmarks",
                &[CargoTarget {
                    name: "smoke".to_string(),
                    kind: vec!["bin".to_string()],
                }],
            ),
            "azure_data_cosmos_benchmarks"
        );
    }

    #[test]
    fn adds_lib_selector_for_library_like_packages() {
        let package = super::PackageMetadata {
            name: "azurecosmosdriver".to_string(),
            version: "0.1.0".to_string(),
            manifest_path: PathBuf::from("sdk/cosmos/azure_data_cosmos_driver_native/Cargo.toml"),
            has_library_target: true,
            api: Default::default(),
        };

        assert_eq!(package.rustdoc_selector_args(), ["--lib"]);
    }

    #[test]
    fn omits_lib_selector_for_non_library_packages() {
        let package = super::PackageMetadata {
            name: "azure_data_cosmos_benchmarks".to_string(),
            version: "0.1.0".to_string(),
            manifest_path: PathBuf::from("sdk/cosmos/azure_data_cosmos_benchmarks/Cargo.toml"),
            has_library_target: false,
            api: Default::default(),
        };

        assert!(package.rustdoc_selector_args().is_empty());
    }

    #[test]
    fn reads_api_metadata_from_cargo_metadata() {
        let package: CargoPackage = serde_json::from_value(serde_json::json!({
            "id": "demo 1.0.0 (path+file:///demo)",
            "manifest_path": "/demo/Cargo.toml",
            "version": "1.0.0",
            "name": "demo",
            "description": "First line\nSecond line",
            "edition": "2021",
            "rust_version": "1.88",
            "features": {
                "default": ["dep:foo", "foo/std"],
                "test": ["default"]
            },
            "metadata": {
                "docs": {
                    "rs": {
                        "features": ["test"]
                    }
                }
            },
            "targets": []
        }))
        .expect("package metadata should deserialize");

        assert_eq!(
            package.description.as_deref(),
            Some("First line\nSecond line")
        );
        assert_eq!(package.edition.as_deref(), Some("2021"));
        assert_eq!(package.rust_version.as_deref(), Some("1.88"));
        assert_eq!(
            package.features["default"],
            ["dep:foo".to_string(), "foo/std".to_string()]
        );
        assert_eq!(
            package
                .metadata
                .and_then(|metadata| metadata.docs.rs.features),
            Some(vec!["test".to_string()])
        );
    }

    #[test]
    fn allows_missing_optional_api_metadata() {
        let package: CargoPackage = serde_json::from_value(serde_json::json!({
            "id": "demo 1.0.0 (path+file:///demo)",
            "manifest_path": "/demo/Cargo.toml",
            "version": "1.0.0",
            "name": "demo",
            "targets": []
        }))
        .expect("package metadata should deserialize");

        assert!(package.edition.is_none());
        assert!(package.rust_version.is_none());
        assert!(package.features.is_empty());
        assert!(package.metadata.is_none());
    }

    #[test]
    fn selects_default_and_docs_rs_features() {
        let features = BTreeMap::from([
            ("alpha".to_string(), vec!["dep:alpha".to_string()]),
            ("default".to_string(), vec!["alpha".to_string()]),
            ("test".to_string(), vec!["default".to_string()]),
        ]);

        assert_eq!(
            select_features(features, Some(&["alpha".to_string()])),
            BTreeMap::from([
                ("alpha".to_string(), vec!["dep:alpha".to_string()]),
                ("default".to_string(), vec!["alpha".to_string()]),
            ])
        );
    }

    #[test]
    fn does_not_duplicate_default_from_docs_rs_features() {
        let features = BTreeMap::from([
            ("alpha".to_string(), Vec::new()),
            ("default".to_string(), vec!["alpha".to_string()]),
        ]);

        assert_eq!(
            select_features(
                features,
                Some(&["default".to_string(), "alpha".to_string()])
            ),
            BTreeMap::from([
                ("alpha".to_string(), Vec::new()),
                ("default".to_string(), vec!["alpha".to_string()]),
            ])
        );
    }

    #[test]
    fn selects_all_features_without_docs_rs_features() {
        let features = BTreeMap::from([
            ("alpha".to_string(), Vec::new()),
            ("default".to_string(), vec!["alpha".to_string()]),
        ]);

        assert_eq!(select_features(features.clone(), None), features);
    }

    #[test]
    fn declares_default_when_no_features_are_defined() {
        assert_eq!(
            select_features(BTreeMap::new(), Some(&["test".to_string()])),
            BTreeMap::from([("default".to_string(), Vec::new())])
        );
    }
}
