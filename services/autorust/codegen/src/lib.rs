pub mod autorust_toml;
pub mod cargo_toml;
mod codegen;
mod codegen_models;
mod codegen_operations;
pub mod config_parser;
pub mod content_type;
pub mod crates;
mod error;
pub mod gen;
pub mod identifier;
pub mod io;
pub mod jinja;
pub mod lib_rs;
pub mod readme_md;
pub mod spec;
mod status_codes;
use autorust_toml::PackageConfig;
use camino::{Utf8Path, Utf8PathBuf};
use config_parser::Configuration;
pub use error::{Error, ErrorKind, Result, ResultExt};
use proc_macro2::TokenStream;
use std::io::Write;
use std::{
    collections::HashSet,
    fs::{self, File},
};

pub use self::{
    codegen::CodeGen,
    spec::{ResolvedSchema, Spec, WebOperation},
};

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

/// Settings for the entire run, generating multiple crates
#[derive(Clone, Debug, PartialEq)]
pub struct RunConfig {
    pub crate_name_prefix: &'static str,
    pub runs: Vec<Runs>,
    pub print_writing_file: bool,
}

impl RunConfig {
    pub fn new(crate_name_prefix: &'static str) -> Self {
        Self {
            crate_name_prefix,
            runs: vec![Runs::Models, Runs::Operations],
            print_writing_file: false,
        }
    }
}

/// Settings for generating of a single crate
#[derive(Clone, Debug, PartialEq)]
pub struct CrateConfig<'a> {
    pub run_config: &'a RunConfig,
    pub input_files: Vec<Utf8PathBuf>,
    pub output_folder: Utf8PathBuf,
}

impl<'a> CrateConfig<'a> {
    pub fn should_run(&self, runs: &Runs) -> bool {
        self.run_config.runs.contains(runs)
    }
    pub fn print_writing_file(&self) -> bool {
        self.run_config.print_writing_file
    }
}

#[allow(clippy::ptr_arg)]
fn to_property_name(triple: &Vec<String>) -> PropertyName {
    PropertyName {
        file_path: Utf8PathBuf::from(triple[0].clone()),
        schema_name: triple[1].clone(),
        property_name: triple[2].clone(),
    }
}

pub fn run<'a>(crate_config: &'a CrateConfig, package_config: &'a PackageConfig) -> Result<CodeGen<'a>> {
    let directory = &crate_config.output_folder;
    fs::create_dir_all(directory).with_context(ErrorKind::Io, || format!("create directory {directory}"))?;

    let box_properties: HashSet<PropertyName> = package_config.properties.boxed.iter().map(to_property_name).collect();
    let optional_properties: HashSet<PropertyName> = package_config.properties.optional.iter().map(to_property_name).collect();
    let fix_case_properties: HashSet<&'a str> = package_config.properties.fix_case.iter().map(AsRef::as_ref).collect();
    let invalid_types: HashSet<PropertyName> = package_config.properties.invalid_type.iter().map(to_property_name).collect();

    let cg = CodeGen::new(
        crate_config,
        box_properties,
        optional_properties,
        fix_case_properties,
        invalid_types,
    )?;

    // create models from schemas
    if crate_config.should_run(&Runs::Models) {
        let models = codegen_models::create_models(&cg)?;
        let models_path = io::join(&crate_config.output_folder, "models.rs")?;
        write_file(&models_path, &models, crate_config.print_writing_file())?;
    }

    // create api client from operations
    if crate_config.should_run(&Runs::Operations) {
        let operations = codegen_operations::create_operations(&cg)?;
        let operations_path = io::join(&crate_config.output_folder, "mod.rs")?;
        write_file(&operations_path, &operations, crate_config.print_writing_file())?;
    }

    Ok(cg)
}

fn write_file<P: AsRef<Utf8Path>>(file: P, tokens: &TokenStream, print_writing_file: bool) -> Result<()> {
    let file = file.as_ref();
    if print_writing_file {
        println!("writing file {}", &file);
    }
    let code = tokens.to_string();
    let mut buffer = File::create(&file).with_context(ErrorKind::Io, || format!("create file {file}"))?;
    buffer
        .write_all(code.as_bytes())
        .with_context(ErrorKind::Io, || format!("write file {file}"))?;
    Ok(())
}

const SPEC_FOLDER: &str = "../../../azure-rest-api-specs/specification";

// gets a sorted list of folders in azure-rest-api-specs/specification
fn get_spec_folders(spec_folder: &str) -> Result<Vec<String>> {
    let paths = fs::read_dir(spec_folder)?;
    let mut spec_folders = Vec::new();
    for path in paths {
        let path = path?;
        if path.file_type()?.is_dir() {
            let file_name = path.file_name();
            let spec_folder = file_name
                .to_str()
                .ok_or_else(|| Error::with_message(ErrorKind::Io, || format!("file name not UTF-8 {file_name:?}")))?;
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
    pub fn config(&self) -> Result<Configuration> {
        Ok(config_parser::parse_configurations_from_autorest_config_file(&self.readme).unwrap_or_default())
    }
}

fn get_spec_readmes(spec_folders: Vec<String>, readme: impl AsRef<Utf8Path>) -> Result<Vec<SpecReadme>> {
    Ok(spec_folders
        .into_iter()
        .filter_map(|spec| match io::join(SPEC_FOLDER, &spec) {
            Ok(spec_folder_full) => get_readme(&spec_folder_full, &readme).map(|readme| SpecReadme { spec, readme }),
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
