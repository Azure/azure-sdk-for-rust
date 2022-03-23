pub mod cargo_toml;
mod codegen;
mod codegen_models;
mod codegen_operations;
pub mod config_parser;
pub mod identifier;
pub mod io;
pub mod lib_rs;
pub mod readme_md;
pub mod spec;
mod status_codes;
use camino::{Utf8Path, Utf8PathBuf};
use config_parser::Configuration;
use proc_macro2::TokenStream;
use std::io::Write;
use std::{
    collections::HashSet,
    fs::{self, File},
};

pub use self::{
    codegen::{create_mod, CodeGen},
    spec::{ResolvedSchema, Spec, WebOperation},
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(io::Error),
    #[error(transparent)]
    CodeGen(#[from] codegen::Error),
    #[error(transparent)]
    ConfigParser(#[from] config_parser::Error),
    #[error(transparent)]
    CargoToml(#[from] cargo_toml::Error),
    #[error(transparent)]
    ReadmeMd(#[from] readme_md::Error),
    #[error(transparent)]
    LibRs(#[from] lib_rs::Error),
}
impl<T: Into<io::Error>> From<T> for Error {
    fn from(error: T) -> Self {
        Self::Io(error.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropertyName {
    pub file_path: Utf8PathBuf,
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
    pub input_files: Vec<Utf8PathBuf>,
    pub output_folder: Utf8PathBuf,
    pub box_properties: HashSet<PropertyName>,
    pub optional_properties: HashSet<PropertyName>,
    pub fix_case_properties: HashSet<String>,
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

pub fn run(config: Config) -> Result<(), Error> {
    let directory = &config.output_folder;
    fs::create_dir_all(directory).map_err(|source| io::Error::CreateOutputDirectory {
        source,
        directory: directory.into(),
    })?;
    let cg = &CodeGen::new(config.clone())?;

    // create models from schemas
    if config.should_run(&Runs::Models) {
        let models = codegen_models::create_models(cg)?;
        let models_path = io::join(&config.output_folder, "models.rs")?;
        write_file(&models_path, &models, config.print_writing_file)?;
    }

    // create api client from operations
    if config.should_run(&Runs::Operations) {
        let operations = codegen_operations::create_operations(cg)?;
        let operations_path = io::join(&config.output_folder, "operations.rs")?;
        write_file(&operations_path, &operations, config.print_writing_file)?;

        let operations = create_mod();
        let operations_path = io::join(&config.output_folder, "mod.rs")?;
        write_file(&operations_path, &operations, config.print_writing_file)?;
    }

    Ok(())
}

fn write_file<P: AsRef<Utf8Path>>(file: P, tokens: &TokenStream, print_writing_file: bool) -> Result<(), io::Error> {
    let file = file.as_ref();
    if print_writing_file {
        println!("writing file {}", &file);
    }
    let code = tokens.to_string();
    let mut buffer = File::create(&file).map_err(|source| io::Error::CreateFile { source, file: file.into() })?;
    buffer
        .write_all(code.as_bytes())
        .map_err(|source| io::Error::WriteFile { source, file: file.into() })?;
    Ok(())
}

const SPEC_FOLDER: &str = "../../../azure-rest-api-specs/specification";

// gets a sorted list of folders in azure-rest-api-specs/specification
fn get_spec_folders(spec_folder: &str) -> Result<Vec<String>, io::Error> {
    let paths = fs::read_dir(spec_folder)?;
    let mut spec_folders = Vec::new();
    for path in paths {
        let path = path?;
        if path.file_type()?.is_dir() {
            let file_name = path.file_name();
            let spec_folder = file_name.to_str().ok_or(io::Error::FileNameNotUtf8)?;
            spec_folders.push(spec_folder.to_owned());
        }
    }
    spec_folders.sort();
    Ok(spec_folders)
}

fn get_readme(spec_folder_full: &dyn AsRef<Utf8Path>, readme_kind: &dyn AsRef<Utf8Path>) -> Option<Utf8PathBuf> {
    match io::join(spec_folder_full, readme_kind) {
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
    readme: Utf8PathBuf,
}

impl SpecReadme {
    pub fn spec(&self) -> &str {
        self.spec.as_str()
    }
    pub fn service_name(&self) -> String {
        get_service_name(&self.spec)
    }
    pub fn readme(&self) -> &Utf8Path {
        self.readme.as_path()
    }
    pub fn config(&self) -> Result<Configuration, Error> {
        Ok(config_parser::parse_configurations_from_autorest_config_file(&self.readme)?)
    }
}

fn get_spec_readmes(spec_folders: Vec<String>, readme: impl AsRef<Utf8Path>) -> Result<Vec<SpecReadme>, io::Error> {
    Ok(spec_folders
        .into_iter()
        .filter_map(|spec| match io::join(SPEC_FOLDER, &spec) {
            Ok(spec_folder_full) => get_readme(&spec_folder_full, &readme).map(|readme| SpecReadme { spec, readme }),
            Err(_) => None,
        })
        .collect())
}

pub fn get_mgmt_readmes() -> Result<Vec<SpecReadme>, io::Error> {
    get_spec_readmes(get_spec_folders(SPEC_FOLDER)?, "resource-manager/readme.md")
}

pub fn get_svc_readmes() -> Result<Vec<SpecReadme>, io::Error> {
    let mut readmes = get_spec_readmes(get_spec_folders(SPEC_FOLDER)?, "data-plane/readme.md")?;
    // the storage data-plane specs do not follow the pattern
    readmes.push(SpecReadme {
        spec: "blobstorage".to_owned(),
        readme: io::join(SPEC_FOLDER, "storage/data-plane/Microsoft.BlobStorage/readme.md")?,
    });
    readmes.push(SpecReadme {
        spec: "filestorage".to_owned(),
        readme: io::join(SPEC_FOLDER, "storage/data-plane/Microsoft.FileStorage/readme.md")?,
    });
    readmes.push(SpecReadme {
        spec: "queuestorage".to_owned(),
        readme: io::join(SPEC_FOLDER, "storage/data-plane/Microsoft.QueueStorage/readme.md")?,
    });
    readmes.push(SpecReadme {
        spec: "storagedatalake".to_owned(),
        readme: io::join(SPEC_FOLDER, "storage/data-plane/Microsoft.StorageDataLake/readme.md")?,
    });
    Ok(readmes)
}

fn get_service_name(spec_name: &str) -> String {
    spec_name.replace("azure", "").replace('_', "").replace('-', "").to_lowercase()
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
