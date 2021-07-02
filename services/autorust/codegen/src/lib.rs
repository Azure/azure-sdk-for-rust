pub mod cargo_toml;
mod codegen;
mod codegen_models;
mod codegen_operations;
mod codegen_routes;
pub mod config_parser;
pub mod identifier;
pub mod lib_rs;
pub mod path;
pub mod spec;
mod status_codes;

use std::{
    collections::HashSet,
    fs::{
        self,
        File,
    },
    io::prelude::*,
    path::{
        Path,
        PathBuf,
    },
};

use config_parser::Configuration;
use proc_macro2::TokenStream;

pub use self::{
    codegen::{
        create_mod,
        CodeGen,
    },
    spec::{
        OperationVerb,
        ResolvedSchema,
        Spec,
    },
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not create output directory {}: {}", directory.display(), source)]
    CreateOutputDirectoryError { directory: PathBuf, source: std::io::Error },
    #[error("Could not create file {}: {}", file.display(), source)]
    CreateFileError { file: PathBuf, source: std::io::Error },
    #[error("Could not write file {}: {}", file.display(), source)]
    WriteFileError { file: PathBuf, source: std::io::Error },
    #[error("CodeGenNewError")]
    CodeGenNewError { source: codegen::Error },
    #[error("CreateModelsError {} {}", config.output_folder.display(), source)]
    CreateModelsError { source: codegen::Error, config: Config },
    #[error("CreateOperationsError")]
    CreateOperationsError { source: codegen::Error },
    #[error("PathError")]
    PathError { source: path::Error },
    #[error("IoError")]
    IoError { source: std::io::Error },
    #[error("file name was not utf-8")]
    FileNameNotUtf8Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropertyName {
    pub file_path: PathBuf,
    pub schema_name: String,
    pub property_name: String,
}

/// Different types of code generators to run
#[derive(Clone, Debug, PartialEq)]
pub enum Runs {
    Models,
    Operations,
    Routes,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub input_files: Vec<PathBuf>,
    pub output_folder: PathBuf,
    pub api_version: Option<String>,
    pub box_properties: HashSet<PropertyName>,
    pub runs: Vec<Runs>,
    pub print_writing_file: bool,
}

impl Config {
    pub fn should_run(&self, runs: &Runs) -> bool {
        self.runs.contains(runs)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_files: Vec::new(),
            output_folder: ".".into(),
            api_version: None,
            box_properties: HashSet::new(),
            runs: vec![Runs::Models, Runs::Operations],
            print_writing_file: true,
        }
    }
}

pub fn run(config: Config) -> Result<()> {
    let directory = &config.output_folder;
    fs::create_dir_all(directory).map_err(|source| Error::CreateOutputDirectoryError {
        source,
        directory: directory.into(),
    })?;
    let cg = &CodeGen::new(config.clone()).map_err(|source| Error::CodeGenNewError { source })?;

    // create models from schemas
    if config.should_run(&Runs::Models) {
        let models = codegen_models::create_models(cg).map_err(|source| Error::CreateModelsError {
            source,
            config: config.clone(),
        })?;
        let models_path = path::join(&config.output_folder, "models.rs").map_err(|source| Error::PathError { source })?;
        write_file(&models_path, &models, config.print_writing_file)?;
    }

    // create api client from operations
    if config.should_run(&Runs::Operations) {
        let operations = codegen_operations::create_operations(cg).map_err(|source| Error::CreateOperationsError { source })?;
        let operations_path = path::join(&config.output_folder, "operations.rs").map_err(|source| Error::PathError { source })?;
        write_file(&operations_path, &operations, config.print_writing_file)?;

        if let Some(api_version) = &config.api_version {
            let operations = create_mod(api_version);
            let operations_path = path::join(&config.output_folder, "mod.rs").map_err(|source| Error::PathError { source })?;
            write_file(&operations_path, &operations, config.print_writing_file)?;
        }
    }

    // create server-side routes
    if config.should_run(&Runs::Routes) {
        let routes = codegen_routes::create_routes(cg).map_err(|source| Error::CreateOperationsError { source })?;
        let routes_path = path::join(&config.output_folder, "routes.rs").map_err(|source| Error::PathError { source })?;
        write_file(&routes_path, &routes, config.print_writing_file)?;
    }

    Ok(())
}

fn write_file<P: AsRef<Path>>(file: P, tokens: &TokenStream, print_writing_file: bool) -> Result<()> {
    let file = file.as_ref();
    if print_writing_file {
        println!("writing file {}", &file.display());
    }
    let code = tokens.to_string();
    let mut buffer = File::create(&file).map_err(|source| Error::CreateFileError { source, file: file.into() })?;
    buffer
        .write_all(&code.as_bytes())
        .map_err(|source| Error::WriteFileError { source, file: file.into() })?;
    Ok(())
}

const SPEC_FOLDER: &str = "../../../azure-rest-api-specs/specification";

// gets a sorted list of folders in ../azure-rest-api-specs/specification
fn get_spec_folders(spec_folder: &str) -> Result<Vec<String>, Error> {
    let paths = fs::read_dir(spec_folder).map_err(|source| Error::IoError { source })?;
    let mut spec_folders = Vec::new();
    for path in paths {
        let path = path.map_err(|source| Error::IoError { source })?;
        if path.file_type().map_err(|source| Error::IoError { source })?.is_dir() {
            let file_name = path.file_name();
            let spec_folder = file_name.to_str().map_or(Err(Error::FileNameNotUtf8Error), Ok)?;
            spec_folders.push(spec_folder.to_owned());
        }
    }
    spec_folders.sort();
    Ok(spec_folders)
}

const RESOURCE_MANAGER_README: &str = "resource-manager/readme.md";
const DATA_PLANE_README: &str = "data-plane/readme.md";

pub fn get_mgmt_configs() -> Result<Vec<SpecConfigs>> {
    get_spec_configs(SPEC_FOLDER, &RESOURCE_MANAGER_README)
}

pub fn get_svc_configs() -> Result<Vec<SpecConfigs>> {
    get_spec_configs(SPEC_FOLDER, &DATA_PLANE_README)
}

fn get_readme(spec_folder_full: &dyn AsRef<Path>, readme_kind: &dyn AsRef<Path>) -> Option<PathBuf> {
    match path::join(spec_folder_full, readme_kind) {
        Ok(readme) => {
            if readme.exists() {
                Some(readme)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub struct SpecConfigs {
    spec: String,
    readme: PathBuf,
    configs: Vec<Configuration>,
}

impl SpecConfigs {
    pub fn spec(&self) -> &str {
        self.spec.as_str()
    }
    pub fn readme(&self) -> &Path {
        self.readme.as_path()
    }
    pub fn configs(&self) -> &Vec<Configuration> {
        self.configs.as_ref()
    }
}

fn get_spec_configs(spec_folder: &str, readme_kind: &dyn AsRef<Path>) -> Result<Vec<SpecConfigs>> {
    let specs = get_spec_folders(spec_folder)?;
    Ok(specs
        .into_iter()
        .filter_map(|spec| match path::join(SPEC_FOLDER, &spec) {
            Ok(spec_folder_full) => match get_readme(&spec_folder_full, readme_kind) {
                Some(readme) => {
                    let configs = config_parser::parse_configurations_from_autorest_config_file(&readme);
                    Some(SpecConfigs { spec, readme, configs })
                }
                None => None,
            },
            Err(_) => None,
        })
        .collect())
}
