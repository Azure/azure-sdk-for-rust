pub mod cargo_toml;
mod codegen;
mod codegen_models;
mod codegen_operations;
pub mod config_parser;
pub mod identifier;
pub mod lib_rs;
pub mod path;
pub mod spec;
mod status_codes;
use config_parser::Configuration;
use proc_macro2::TokenStream;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
};

pub use self::{
    codegen::{create_mod, CodeGen},
    spec::{ResolvedSchema, Spec, WebOperation},
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not create output directory {}: {}", directory.display(), source)]
    CreateOutputDirectory { directory: PathBuf, source: std::io::Error },
    #[error("Could not create file {}: {}", file.display(), source)]
    CreateFile { file: PathBuf, source: std::io::Error },
    #[error("Could not write file {}: {}", file.display(), source)]
    WriteFile { file: PathBuf, source: std::io::Error },
    #[error("CodeGenNewError")]
    CodeGenNew(#[source] codegen::Error),
    #[error("CreateModelsError {0}")]
    CreateModels(#[source] codegen::Error),
    #[error("CreateOperationsError")]
    CreateOperations(#[source] codegen::Error),
    #[error("path: {0}")]
    Path(#[from] path::Error),
    #[error("io: {0}")]
    Io(#[source] std::io::Error),
    #[error("file name was not utf-8")]
    FileNameNotUtf8,
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub input_files: Vec<PathBuf>,
    pub output_folder: PathBuf,
    pub box_properties: HashSet<PropertyName>,
    pub optional_properties: HashSet<PropertyName>,
    pub fix_case_properties: HashSet<PropertyName>,
    pub invalid_types: HashSet<PropertyName>,
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
            box_properties: HashSet::new(),
            optional_properties: HashSet::new(),
            fix_case_properties: HashSet::new(),
            invalid_types: HashSet::new(),
            runs: vec![Runs::Models, Runs::Operations],
            print_writing_file: true,
        }
    }
}

pub fn run(config: Config) -> Result<()> {
    let directory = &config.output_folder;
    fs::create_dir_all(directory).map_err(|source| Error::CreateOutputDirectory {
        source,
        directory: directory.into(),
    })?;
    let cg = &CodeGen::new(config.clone()).map_err(Error::CodeGenNew)?;

    // create models from schemas
    if config.should_run(&Runs::Models) {
        let models = codegen_models::create_models(cg).map_err(Error::CreateModels)?;
        let models_path = path::join(&config.output_folder, "models.rs").map_err(Error::Path)?;
        write_file(&models_path, &models, config.print_writing_file)?;
    }

    // create api client from operations
    if config.should_run(&Runs::Operations) {
        let operations = codegen_operations::create_operations(cg).map_err(Error::CreateOperations)?;
        let operations_path = path::join(&config.output_folder, "operations.rs").map_err(Error::Path)?;
        write_file(&operations_path, &operations, config.print_writing_file)?;

        if let Some(api_version) = cg.spec.api_version() {
            let operations = create_mod(&api_version);
            let operations_path = path::join(&config.output_folder, "mod.rs").map_err(Error::Path)?;
            write_file(&operations_path, &operations, config.print_writing_file)?;
        } else {
            println!("    no api-version");
        }
    }

    Ok(())
}

fn write_file<P: AsRef<Path>>(file: P, tokens: &TokenStream, print_writing_file: bool) -> Result<()> {
    let file = file.as_ref();
    if print_writing_file {
        println!("writing file {}", &file.display());
    }
    let code = tokens.to_string();
    let mut buffer = File::create(&file).map_err(|source| Error::CreateFile { source, file: file.into() })?;
    buffer
        .write_all(&code.as_bytes())
        .map_err(|source| Error::WriteFile { source, file: file.into() })?;
    Ok(())
}

const SPEC_FOLDER: &str = "../../../azure-rest-api-specs/specification";

// gets a sorted list of folders in azure-rest-api-specs/specification
fn get_spec_folders(spec_folder: &str) -> Result<Vec<String>, Error> {
    let paths = fs::read_dir(spec_folder).map_err(Error::Io)?;
    let mut spec_folders = Vec::new();
    for path in paths {
        let path = path.map_err(Error::Io)?;
        if path.file_type().map_err(Error::Io)?.is_dir() {
            let file_name = path.file_name();
            let spec_folder = file_name.to_str().ok_or_else(|| Error::FileNameNotUtf8)?;
            spec_folders.push(spec_folder.to_owned());
        }
    }
    spec_folders.sort();
    Ok(spec_folders)
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

pub struct SpecReadme {
    /// service name
    spec: String,
    readme: PathBuf,
}

impl SpecReadme {
    pub fn spec(&self) -> &str {
        self.spec.as_str()
    }
    pub fn service_name(&self) -> String {
        get_service_name(&self.spec)
    }
    pub fn readme(&self) -> &Path {
        self.readme.as_path()
    }
    pub fn configs(&self) -> Vec<Configuration> {
        config_parser::parse_configurations_from_autorest_config_file(&self.readme)
    }
}

fn get_spec_readmes(spec_folders: Vec<String>, readme: impl AsRef<Path>) -> Result<Vec<SpecReadme>> {
    Ok(spec_folders
        .into_iter()
        .filter_map(|spec| match path::join(SPEC_FOLDER, &spec) {
            Ok(spec_folder_full) => match get_readme(&spec_folder_full, &readme) {
                Some(readme) => Some(SpecReadme { spec, readme }),
                None => None,
            },
            Err(_) => None,
        })
        .collect())
}

pub fn get_mgmt_readmes() -> Result<Vec<SpecReadme>> {
    get_spec_readmes(get_spec_folders(SPEC_FOLDER)?, "resource-manager/readme.md")
}

pub fn get_svc_readmes() -> Result<Vec<SpecReadme>> {
    let mut readmes = get_spec_readmes(get_spec_folders(SPEC_FOLDER)?, "data-plane/readme.md")?;
    // the storage data-plane specs do not follow the pattern
    readmes.push(SpecReadme {
        spec: "blobstorage".to_owned(),
        readme: path::join(SPEC_FOLDER, "storage/data-plane/Microsoft.BlobStorage/readme.md")?,
    });
    readmes.push(SpecReadme {
        spec: "filestorage".to_owned(),
        readme: path::join(SPEC_FOLDER, "storage/data-plane/Microsoft.FileStorage/readme.md")?,
    });
    readmes.push(SpecReadme {
        spec: "queuestorage".to_owned(),
        readme: path::join(SPEC_FOLDER, "storage/data-plane/Microsoft.QueueStorage/readme.md")?,
    });
    readmes.push(SpecReadme {
        spec: "storagedatalake".to_owned(),
        readme: path::join(SPEC_FOLDER, "storage/data-plane/Microsoft.StorageDataLake/readme.md")?,
    });
    Ok(readmes)
}

fn get_service_name(spec_name: &str) -> String {
    spec_name.replace("azure", "").replace("_", "").replace("-", "").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_name() {
        assert_eq!("activedirectory", get_service_name("azureactivedirectory"));
        assert_eq!("cosmosdb", get_service_name("cosmos_db"));
        assert_eq!("datalakestore", get_service_name("datalake_store"));
        assert_eq!("kusto", get_service_name("azure-kusto"));
        assert_eq!("enterpriseknowledgegraph", get_service_name("EnterpriseKnowledgeGraph"));
    }
}
