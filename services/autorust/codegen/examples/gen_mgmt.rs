// cargo run --example gen_mgmt --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/compute/resource-manager
use autorust_codegen::{
    self, cargo_toml,
    config_parser::{to_api_version, to_mod_name},
    get_mgmt_configs, lib_rs, path, Config, PropertyName, SpecConfigs,
};
use heck::SnakeCase;

use std::{collections::HashSet, fs, path::PathBuf};

const OUTPUT_FOLDER: &str = "../mgmt";

const ONLY_SERVICES: &[&str] = &[
    // "vmware",
    // "network",
];

const SKIP_SERVICES: &[&str] = &[
    "automation",                 // TODO #81 DataType::File
    "deploymentmanager",          // TODO #80 path parameters
    "deviceprovisioningservices", // TODO #82 certificate_name used as parameter more than once
    "dnc",                        // https://github.com/Azure/azure-rest-api-specs/pull/11578 two ControllerDetails types
    "m365securityandcompliance",  // can't find privateLinkServicesForO365ManagementActivityAPI.json
    "mixedreality",               // TODO #83 AccountKeyRegenerateRequest not generated
    "netapp",                     // Ident "10minutely"
    "powerplatform",              // https://github.com/Azure/azure-rest-api-specs/pull/11580 incorrect ref & duplicate Operations_List
    "service-map",                // Ident "Ref:machine"
    "servicefabric",              // https://github.com/Azure/azure-rest-api-specs/pull/11581 allOf mistakes and duplicate Operations_List
    "servicefabricmanagedclusters",
    "web", // TODO #81 DataType::File
];

const SKIP_SERVICE_TAGS: &[(&str, &str)] = &[
    ("analysisservices", "package-2017-08"),
    ("authorization", "package-2018-05-01-preview"),
    ("authorization", "package-2021-03-01-preview-only"),
    ("azureactivedirectory", "package-preview-2020-07"),
    ("compute", "package-2020-10-01-preview"),      // TODO #81 DataType::File
    ("compute", "package-2020-10-01-preview-only"), // TODO #81 DataType::File
    ("compute", "package-2021-03-01"),              // TODO #81 DataType::File
    ("compute", "package-2021-03-01-only"),         // TODO #81 DataType::File
    ("consumption", "package-2019-11"),             // ReservationRecommendationDetails_Get has a path and query param both named "scope"
    // datamigration, same error for all
    // SchemaNotFound MigrateSqlServerSqlDbTask.json ValidationStatus, but may be buried
    ("datamigration", "package-2018-07-15-preview"),
    ("datamigration", "package-2018-04-19"),
    ("datamigration", "package-2018-03-31-preview"),
    ("datamigration", "package-2018-03-15-preview"),
    ("datamigration", "package-2017-11-15-preview"),
    ("mediaservices", "package-2019-05-preview"), // invalid unicode character of a dash instead of a hyphen https://github.com/Azure/azure-rest-api-specs/pull/11576
    ("marketplace", "package-composite-v1"),
    ("network", "package-2017-03-30-only"), // SchemaNotFound 2017-09-01/network.json SubResource
    ("recoveryservicesbackup", "package-2020-07"), // duplicate fn get_operation_status
    ("recoveryservicesbackup", "package-2020-10"), // duplicate fn get_operation_status
    ("recoveryservicessiterecovery", "package-2016-08"), // duplicate package-2016-08 https://github.com/Azure/azure-rest-api-specs/pull/11287
    ("resources", "package-policy-2020-03"),
    ("resources", "package-policy-2020-09"), // SchemaNotFound { ref_key: RefKey { file_path: "../../../azure-rest-api-specs/specification/resources/resource-manager/Microsoft.Authorization/stable/2020-09-01/dataPolicyManifests.json", name: "CloudError"
    ("security", "package-2020-01-preview-only"), // duplicate tag https://github.com/Azure/azure-rest-api-specs/pull/13828
    ("synapse", "package-2019-06-01-preview"), // TODO #80 path parameters
    ("synapse", "package-2020-12-01"),
    ("synapse", "package-2021-03"),
];

// becuse of recursive types, some properties have to be boxed
// https://github.com/ctaggart/autorust/issues/73
const BOX_PROPERTIES: &[(&str, &str, &str)] = &[
    // cost-management
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2020-06-01/costmanagement.json", "ReportConfigFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2020-06-01/costmanagement.json", "QueryFilter", "not"),
    // databox
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2020-11-01/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2020-11-01/databox.json", "transferAllDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2021-03-01/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2021-03-01/databox.json", "transferAllDetails", "include"),
    // dataprotection
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/stable/2021-01-01/dataprotection.json", "InnerError", "embeddedInnerError"),
    // hardwaresecuritymodels
    ("../../../azure-rest-api-specs/specification/hardwaresecuritymodules/resource-manager/Microsoft.HardwareSecurityModules/preview/2018-10-31-preview/dedicatedhsm.json", "Error", "innererror"),
    // logic
    ("../../../azure-rest-api-specs/specification/logic/resource-manager/Microsoft.Logic/stable/2019-05-01/logic.json", "SwaggerSchema", "items"),
    // migrateprojects
    ("../../../azure-rest-api-specs/specification/migrateprojects/resource-manager/Microsoft.Migrate/preview/2018-09-01-preview/migrate.json", "IEdmNavigationProperty", "partner"),
    ("../../../azure-rest-api-specs/specification/migrateprojects/resource-manager/Microsoft.Migrate/preview/2018-09-01-preview/migrate.json", "IEdmStructuredType", "baseType"),
    // network
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-07-01/publicIpAddress.json", "PublicIPAddressPropertiesFormat", "ipConfiguration"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-08-01/publicIpAddress.json", "PublicIPAddressPropertiesFormat", "ipConfiguration"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-11-01/publicIpAddress.json", "PublicIPAddressPropertiesFormat", "ipConfiguration"),
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
    for (i, spec) in get_mgmt_configs()
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
    let crate_name = &format!("azure_mgmt_{}", service_name);
    let output_folder = &path::join(OUTPUT_FOLDER, service_name).map_err(|source| Error::PathError { source })?;

    let src_folder = path::join(output_folder, "src").map_err(|source| Error::PathError { source })?;
    if src_folder.exists() {
        fs::remove_dir_all(&src_folder).map_err(|source| Error::IoError { source })?;
    }

    let mut feature_mod_names = Vec::new();
    let skip_service_tags: HashSet<&(&str, &str)> = SKIP_SERVICE_TAGS.iter().collect();

    let mut box_properties = HashSet::new();
    for (file_path, schema_name, property_name) in BOX_PROPERTIES {
        box_properties.insert(PropertyName {
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
                box_properties: box_properties.clone(),
            })
            .map_err(|source| Error::CodegenError { source })?;
        }
    }
    if feature_mod_names.len() == 0 {
        return Ok(());
    }
    cargo_toml::create(
        crate_name,
        &feature_mod_names,
        &path::join(output_folder, "Cargo.toml").map_err(|source| Error::PathError { source })?,
    )
    .map_err(|source| Error::CargoTomlError { source })?;
    lib_rs::create(
        &feature_mod_names,
        &path::join(src_folder, "lib.rs").map_err(|source| Error::PathError { source })?,
    )
    .map_err(|source| Error::LibRsError { source })?;

    Ok(())
}

fn get_service_name(spec_folder: &str) -> String {
    spec_folder.to_snake_case().replace("-", "_")
}
