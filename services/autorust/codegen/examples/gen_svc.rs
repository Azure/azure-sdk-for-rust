// cargo run --example gen_svc --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/batch/data-plane
use autorust_codegen::{
    self, cargo_toml,
    config_parser::{to_api_version, to_mod_name},
    get_svc_configs, lib_rs, path, Config, PropertyName, SpecConfigs,
};
use heck::SnakeCase;
use std::{collections::HashSet, fs, path::PathBuf};

const OUTPUT_FOLDER: &str = "../svc";

const ONLY_SERVICES: &[&str] = &["batch"];

const SKIP_SERVICES: &[&str] = &[ ];

const SKIP_SERVICE_TAGS: &[(&str, &str)] = &[
    ("batch", "package-2018-03.6.1"), // TODO #81 DataType::File
    ("batch", "package-2017-09.6.0"), // TODO #81 DataType::File
    ("batch", "package-2017-06.5.1"), // TODO #81 DataType::File
];

const FIX_CASE_PROPERTIES: &[(&str, &str, &str)] = &[
    (
        "../../../azure-rest-api-specs/specification/batch/data-plane/Microsoft.Batch/stable/2021-06-01.14.0/BatchService.json",
        "TaskSchedulingPolicy",
        "nodeFillType",
    ),
    (
        "../../../azure-rest-api-specs/specification/batch/data-plane/Microsoft.Batch/stable/2021-06-01.14.0/BatchService.json",
        "NodePlacementConfiguration",
        "policy",
    ),
    (
        "../../../azure-rest-api-specs/specification/batch/data-plane/Microsoft.Batch/stable/2021-06-01.14.0/BatchService.json",
        "PublicIPAddressConfiguration",
        "provision",
    ),
];

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("file name was not utf-8")]
    FileNameNotUtf8Error {},
    #[error("IoError")]
    IoError { source: std::io::Error },
    #[error("PathError")]
    PathError { source: path::Error },
    #[error("CodegenError")]
    CodegenError { source: autorust_codegen::Error },
    #[error("CargoTomlError")]
    CargoTomlError { source: cargo_toml::Error },
    #[error("LibRsError")]
    LibRsError { source: lib_rs::Error },
    #[error("GetSpecFoldersError")]
    GetSpecFoldersError { source: autorust_codegen::Error },
}

fn main() -> Result<()> {
    for (i, spec) in get_svc_configs()
        .map_err(|source| Error::GetSpecFoldersError { source })?
        .iter()
        .enumerate()
    {
        if ONLY_SERVICES.len() > 0 {
            if ONLY_SERVICES.contains(&spec.spec()) {
                println!("{} {}", i + 1, spec.spec());
                gen_crate(spec)?;
            }
        } else {
            if !SKIP_SERVICES.contains(&spec.spec()) {
                println!("{} {}", i + 1, spec.spec());
                gen_crate(spec)?;
            }
        }
    }
    Ok(())
}

fn gen_crate(spec: &SpecConfigs) -> Result<()> {
    let service_name = &get_service_name(spec.spec());
    let crate_name = &format!("azure_svc_{}", service_name);
    let output_folder = &path::join(OUTPUT_FOLDER, service_name).map_err(|source| Error::PathError { source })?;

    let src_folder = path::join(output_folder, "src").map_err(|source| Error::PathError { source })?;
    if src_folder.exists() {
        fs::remove_dir_all(&src_folder).map_err(|source| Error::IoError { source })?;
    }

    let mut feature_mod_names = Vec::new();
    let skip_service_tags: HashSet<&(&str, &str)> = SKIP_SERVICE_TAGS.iter().collect();

    let mut fix_case_properties = HashSet::new();
    for (file_path, schema_name, property_name) in FIX_CASE_PROPERTIES {
        fix_case_properties.insert(PropertyName {
            file_path: PathBuf::from(file_path),
            schema_name: schema_name.to_string(),
            property_name: property_name.to_string(),
        });
    }

    for config in spec.configs() {
        let tag = config.tag.as_str();
        if let Some(api_version) = to_api_version(&config) {
            if skip_service_tags.contains(&(spec.spec(), tag)) {
                // println!("  skipping {}", tag);
                continue;
            }
            println!("  {}", tag);
            // println!("  {}", api_version);
            let mod_name = &to_mod_name(tag);
            feature_mod_names.push((tag.to_string(), mod_name.clone()));
            // println!("  {}", mod_name);
            let mod_output_folder = path::join(&src_folder, mod_name).map_err(|source| Error::PathError { source })?;
            // println!("  {:?}", mod_output_folder);
            // for input_file in &config.input_files {
            //     println!("  {}", input_file);
            // }
            let input_files: Result<Vec<_>> = config
                .input_files
                .iter()
                .map(|input_file| Ok(path::join(spec.readme(), input_file).map_err(|source| Error::PathError { source })?))
                .collect();
            let input_files = input_files?;
            // for input_file in &input_files {
            //     println!("  {:?}", input_file);
            // }
            autorust_codegen::run(Config {
                api_version: Some(api_version),
                output_folder: mod_output_folder.into(),
                input_files,
                fix_case_properties: fix_case_properties.clone(),
                print_writing_file: false,
                ..Config::default()
            })
            .map_err(|source| Error::CodegenError { source })?;
        }
    }
    if feature_mod_names.len() == 0 {
        return Ok(());
    }
    println!("creating");
    cargo_toml::create(
        crate_name,
        &feature_mod_names,
        &path::join(output_folder, "Cargo.toml").map_err(|source| Error::PathError { source })?,
    )
    .map_err(|source| Error::CargoTomlError { source })?;
    lib_rs::create(
        &feature_mod_names,
        &path::join(src_folder, "lib.rs").map_err(|source| Error::PathError { source })?,
        false,
    )
    .map_err(|source| Error::LibRsError { source })?;

    Ok(())
}

fn get_service_name(spec_folder: &str) -> String {
    spec_folder.to_snake_case().replace("-", "_").replace(".", "_")
}
