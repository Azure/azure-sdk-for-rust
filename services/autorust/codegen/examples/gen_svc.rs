// cargo run --example gen_svc --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/batch/data-plane
use autorust_codegen::{self, cargo_toml, config_parser::to_mod_name, get_svc_readmes, lib_rs, path, Config, PropertyName, SpecReadme};
use std::{collections::HashSet, fs, path::PathBuf};

const OUTPUT_FOLDER: &str = "../svc";

const ONLY_SERVICES: &[&str] = &[];

const SKIP_SERVICES: &[&str] = &[
    "machinelearningservices", // need to box inner errors
    "hdinsight",               // job_id appears multiple times https://github.com/Azure/azure-sdk-for-rust/issues/503
    "videoanalyzer",           // no operations
    "mediaservices",           // no operations
];

const SKIP_SERVICE_TAGS: &[(&str, &str)] = &[
    ("agrifood", "package-2021-03-31-preview"), // duplicate params https://github.com/Azure/azure-sdk-for-rust/issues/501
    ("purview", "package-2021-05-01-preview"),  // need to box types
    ("maps", "package-preview-2.0"),            // global responses https://github.com/Azure/azure-sdk-for-rust/issues/502
    ("maps", "package-1.0-preview"),            // global responses https://github.com/Azure/azure-sdk-for-rust/issues/502
    ("servicefabric", "6.2"),                   // invalid model TimeBasedBackupScheduleDescription
    ("servicefabric", "6.3"),                   // invalid model TimeBasedBackupScheduleDescription
    ("servicefabric", "6.4"),                   // invalid model TimeBasedBackupScheduleDescription
    ("storagedatalake", "package-2018-11"),     // "invalid value: string \"ErrorResponse\", expected length 3"
    ("storagedatalake", "package-2018-06-preview"),
    ("storagedatalake", "package-2019-10"),
];

const INVALID_TYPE_WORKAROUND: &[(&str, &str, &str)] = &[(
    "../../../azure-rest-api-specs/specification/applicationinsights/data-plane/Microsoft.Insights/preview/v1/AppInsights.json",
    "table",
    "rows",
)];

const FIX_CASE_PROPERTIES: &[&str] = &["BatchServiceClient", "BatchService"];

// because of recursive types, some properties have to be boxed
// https://github.com/ctaggart/autorust/issues/73
const BOX_PROPERTIES: &[(&str, &str, &str)] = &[
    // applicationinsights
    ("../../../azure-rest-api-specs/specification/applicationinsights/data-plane/Microsoft.Insights/preview/v1/AppInsights.json", "errorInfo", "innererror"),
    // keyvault
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/preview/7.0/keyvault.json" , "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/preview/7.1/common.json" , "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/preview/7.2-preview/common.json" , "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/preview/7.3-preview/common.json" , "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/stable/2016-10-01/keyvault.json" , "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/stable/7.0/keyvault.json" , "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/stable/7.1/common.json" , "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/stable/7.2/common.json" , "Error" , "innererror"),
    // webpubsub
    (
        "../../../azure-rest-api-specs/specification/webpubsub/data-plane/WebPubSub/stable/2021-10-01/webpubsub.json",
        "InnerError",
        "inner",
    ),
    // mixedreality
    (
        "../../../azure-rest-api-specs/specification/mixedreality/data-plane/Microsoft.MixedReality/stable/2021-01-01/mr-arr.json",
        "error",
        "innerError",
    ),
    (
         "../../../azure-rest-api-specs/specification/mixedreality/data-plane/Microsoft.MixedReality/preview/2021-01-01-preview/mr-arr.json",
        "error",
        "innerError",
    ),
    // confidentialledger
    (
        "../../../azure-rest-api-specs/specification/confidentialledger/data-plane/Microsoft.ConfidentialLedger/preview/0.1-preview/common.json",
        "ConfidentialLedgerErrorBody",
        "innererror",
    ),
    // operationalinsights
    (
        "../../../azure-rest-api-specs/specification/operationalinsights/data-plane/Microsoft.OperationalInsights/stable/v1/OperationalInsights.json",
        "errorInfo",
        "innererror",
    ),
    (
        "../../../azure-rest-api-specs/specification/operationalinsights/data-plane/Microsoft.OperationalInsights/preview/2017-10-01/swagger.json",
        "errorInfo",
        "innererror",
    ),
    (
        "../../../azure-rest-api-specs/specification/operationalinsights/data-plane/Microsoft.OperationalInsights/preview/2021-05-19_Preview/OperationalInsights.json",
        "errorInfo",
        "innererror",
    ),
    // timeseriesinsights
    (
        "../../../azure-rest-api-specs/specification/timeseriesinsights/data-plane/Microsoft.TimeSeriesInsights/stable/2020-07-31/timeseriesinsights.json",
        "TsiErrorBody",
        "innerError",
    ),
    // datalake-analytics
    (
        "../../../azure-rest-api-specs/specification/datalake-analytics/data-plane/Microsoft.DataLakeAnalytics/stable/2016-11-01/job.json",
        "JobInnerError",
        "innerError"
    ),
    (
        "../../../azure-rest-api-specs/specification/datalake-analytics/data-plane/Microsoft.DataLakeAnalytics/preview/2017-09-01-preview/job.json",
        "JobInnerError",
        "innerError"
    ),
    // deviceupdate
    (
        "../../../azure-rest-api-specs/specification/deviceupdate/data-plane/Microsoft.DeviceUpdate/preview/2020-09-01/deviceupdate.json",
        "Error",
        "innerError"
    ),
    (
        "../../../azure-rest-api-specs/specification/deviceupdate/data-plane/Microsoft.DeviceUpdate/preview/2020-09-01/deviceupdate.json",
        "InnerError",
        "innerError"
    ),
    // digitaltwins
    (
        "../../../azure-rest-api-specs/specification/digitaltwins/data-plane/Microsoft.DigitalTwins/preview/2020-05-31-preview/digitaltwins.json",
        "Error",
        "innererror"
    ),
    (
        "../../../azure-rest-api-specs/specification/digitaltwins/data-plane/Microsoft.DigitalTwins/preview/2020-05-31-preview/digitaltwins.json",
        "InnerError",
        "innererror"
    ),
    (
        "../../../azure-rest-api-specs/specification/digitaltwins/data-plane/Microsoft.DigitalTwins/stable/2020-10-31/digitaltwins.json",
        "Error",
        "innererror"
    ),
    (
        "../../../azure-rest-api-specs/specification/digitaltwins/data-plane/Microsoft.DigitalTwins/stable/2020-10-31/digitaltwins.json",
        "InnerError",
        "innererror"
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
    for (i, spec) in get_svc_readmes()
        .map_err(|source| Error::GetSpecFoldersError { source })?
        .iter()
        .enumerate()
    {
        if !ONLY_SERVICES.is_empty() {
            if ONLY_SERVICES.contains(&spec.spec()) {
                println!("{} {}", i + 1, spec.spec());
                gen_crate(spec)?;
            }
        } else if !SKIP_SERVICES.contains(&spec.spec()) {
            println!("{} {}", i + 1, spec.spec());
            gen_crate(spec)?;
        }
    }
    Ok(())
}

fn gen_crate(spec: &SpecReadme) -> Result<()> {
    let skip_service_tags: HashSet<&(&str, &str)> = SKIP_SERVICE_TAGS.iter().collect();
    let has_no_configs = spec
        .configs()
        .iter()
        .all(|x| skip_service_tags.contains(&(spec.spec(), x.tag.as_str())));
    if has_no_configs {
        println!("not generating {}", spec.spec());
        return Ok(());
    }

    let service_name = &spec.service_name();
    let crate_name = &format!("azure_svc_{}", service_name);
    let output_folder = &path::join(OUTPUT_FOLDER, service_name).map_err(|source| Error::PathError { source })?;

    let src_folder = path::join(output_folder, "src").map_err(|source| Error::PathError { source })?;
    if src_folder.exists() {
        fs::remove_dir_all(&src_folder).map_err(|source| Error::IoError { source })?;
    }

    let mut feature_mod_names = Vec::new();

    let mut fix_case_properties = HashSet::new();
    for spec_title in FIX_CASE_PROPERTIES {
        fix_case_properties.insert(spec_title.to_string());
    }

    let mut box_properties = HashSet::new();
    for (file_path, schema_name, property_name) in BOX_PROPERTIES {
        box_properties.insert(PropertyName {
            file_path: PathBuf::from(file_path),
            schema_name: schema_name.to_string(),
            property_name: property_name.to_string(),
        });
    }

    let mut invalid_types = HashSet::new();
    for (file_path, schema_name, property_name) in INVALID_TYPE_WORKAROUND {
        invalid_types.insert(PropertyName {
            file_path: PathBuf::from(file_path),
            schema_name: schema_name.to_string(),
            property_name: property_name.to_string(),
        });
    }

    for config in spec.configs() {
        let tag = config.tag.as_str();
        if skip_service_tags.contains(&(spec.spec(), tag)) {
            // println!("  skipping {}", tag);
            continue;
        }
        println!("  {}", tag);
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
            .map(|input_file| path::join(spec.readme(), input_file).map_err(|source| Error::PathError { source }))
            .collect();
        let input_files = input_files?;
        // for input_file in &input_files {
        //     println!("  {:?}", input_file);
        // }
        autorust_codegen::run(Config {
            output_folder: mod_output_folder,
            input_files,
            box_properties: box_properties.clone(),
            fix_case_properties: fix_case_properties.clone(),
            invalid_types: invalid_types.clone(),
            print_writing_file: false,
            ..Config::default()
        })
        .map_err(|source| Error::CodegenError { source })?;
    }
    if feature_mod_names.is_empty() {
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
        false,
    )
    .map_err(|source| Error::LibRsError { source })?;

    Ok(())
}
