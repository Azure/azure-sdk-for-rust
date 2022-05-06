// cargo run --example gen_svc --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/batch/data-plane
use autorust_codegen::{
    self, autorust_toml, cargo_toml, get_svc_readmes, io, lib_rs,
    readme_md::{self, ReadmeMd},
    CrateConfig, Error, Result, RunConfig, SpecReadme,
};
use std::{collections::HashMap, fs};

const OUTPUT_FOLDER: &str = "../svc";

const ONLY_SERVICES: &[&str] = &[];

const SKIP_SERVICES: &[&str] = &[
    "datalake-store",          // query param "sources" not used
    "machinelearningservices", // need to box inner errors
    "hdinsight",               // job_id appears multiple times https://github.com/Azure/azure-sdk-for-rust/issues/503
    "videoanalyzer",           // no operations
    "mediaservices",           // no operations
];

const SKIP_SERVICE_TAGS: &[(&str, &str)] = &[
    ("agrifood", "package-2021-03-31-preview"), // duplicate params https://github.com/Azure/azure-sdk-for-rust/issues/501
    ("maps", "package-preview-2.0"),            // global responses https://github.com/Azure/azure-sdk-for-rust/issues/502
    ("maps", "package-1.0-preview"),            // global responses https://github.com/Azure/azure-sdk-for-rust/issues/502
    ("servicefabric", "6.2"),                   // invalid model TimeBasedBackupScheduleDescription
    ("servicefabric", "6.3"),                   // invalid model TimeBasedBackupScheduleDescription
    ("servicefabric", "6.4"),                   // invalid model TimeBasedBackupScheduleDescription
    ("storagedatalake", "package-2018-11"),     // "invalid value: string \"ErrorResponse\", expected length 3"
    ("storagedatalake", "package-2018-06-preview"),
    ("storagedatalake", "package-2019-10"),
];

const INVALID_TYPE_WORKAROUND: &[(&str, &str, &str)] = &[
    (
        "../../../azure-rest-api-specs/specification/applicationinsights/data-plane/Microsoft.Insights/preview/v1/AppInsights.json",
        "table",
        "rows",
    ),
    (
        "../../../azure-rest-api-specs/specification/operationalinsights/data-plane/Microsoft.OperationalInsights/stable/v1/OperationalInsights.json",
        "table",
        "rows",
    ),
];

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
    ("../../../azure-rest-api-specs/specification/keyvault/data-plane/Microsoft.KeyVault/stable/7.3/common.json" , "Error" , "innererror"),
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
    (
        "../../../azure-rest-api-specs/specification/mixedreality/data-plane/Microsoft.MixedReality/preview/0.3-preview.0/mr-aoa.json",
        "InnerError",
        "innererror",
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
    (
        "../../../azure-rest-api-specs/specification/deviceupdate/data-plane/Microsoft.DeviceUpdate/preview/2021-06-01-preview/deviceupdate.json",
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
    (
        "../../../azure-rest-api-specs/specification/digitaltwins/data-plane/Microsoft.DigitalTwins/preview/2021-06-30-preview/digitaltwins.json",
        "InnerError",
        "innererror"
    ),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "NumberFormat", "currencyInstance"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "NumberFormat", "instance"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "NumberFormat", "integerInstance"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "NumberFormat", "numberInstance"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "NumberFormat", "percentInstance"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "TimeZone", "default"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "DateFormat", "dateInstance"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "DateFormat", "instance"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "DateFormat", "dateTimeInstance"),
    ("../../../azure-rest-api-specs/specification/purview/data-plane/Azure.Analytics.Purview.Catalog/preview/2022-03-01-preview/purviewcatalog.json", "DateFormat", "timeInstance"),
];

fn main() -> Result<()> {
    let run_config = &mut RunConfig::new("azure_svc_");
    run_config.set_skip_service_tags(SKIP_SERVICE_TAGS);
    run_config.set_fix_case_properties(FIX_CASE_PROPERTIES);
    run_config.set_box_properties(BOX_PROPERTIES);
    run_config.set_invalid_types(INVALID_TYPE_WORKAROUND);

    for (i, spec) in get_svc_readmes()?.iter().enumerate() {
        if !ONLY_SERVICES.is_empty() {
            if ONLY_SERVICES.contains(&spec.spec()) {
                println!("{} {}", i + 1, spec.spec());
                gen_crate(spec, run_config)?;
            }
        } else if !SKIP_SERVICES.contains(&spec.spec()) {
            println!("{} {}", i + 1, spec.spec());
            gen_crate(spec, run_config)?;
        }
    }
    Ok(())
}

fn gen_crate(spec: &SpecReadme, run_config: &RunConfig) -> Result<()> {
    let spec_config = spec.config()?;
    let service_name = &spec.service_name();
    let crate_name = &format!("{}{}", &run_config.crate_name_prefix, service_name);
    let output_folder = &io::join(OUTPUT_FOLDER, service_name)?;
    let package_config = autorust_toml::read(&io::join(&output_folder, "autorust.toml")?)?;
    let tags = spec_config.tags_filtered(spec.spec(), run_config.skip_service_tags());
    let tags = &package_config.tags(tags);
    if tags.is_empty() {
        println!("not generating {} - no tags", spec.spec());
        return Ok(());
    }

    let src_folder = io::join(output_folder, "src")?;
    if src_folder.exists() {
        fs::remove_dir_all(&src_folder)?;
    }

    let mut operation_totals = HashMap::new();
    let mut api_version_totals = HashMap::new();
    let mut api_versions = HashMap::new();
    for tag in tags {
        println!("  {}", tag.name());
        let output_folder = io::join(&src_folder, &tag.rust_mod_name())?;
        let input_files: Result<Vec<_>> = tag
            .input_files()
            .iter()
            .map(|input_file| io::join(spec.readme(), input_file).map_err(Error::from))
            .collect();
        let input_files = input_files?;
        let crate_config = &CrateConfig {
            run_config,
            output_folder,
            input_files,
        };
        let cg = autorust_codegen::run(crate_config)?;
        operation_totals.insert(tag.name(), cg.spec.operations()?.len());
        let mut versions = cg.spec.api_versions();
        versions.sort_unstable();
        api_version_totals.insert(tag.name(), versions.len());
        api_versions.insert(
            tag.name(),
            versions.iter().map(|v| format!("`{}`", v)).collect::<Vec<_>>().join(", "),
        );
    }

    let default_tag = cargo_toml::get_default_tag(tags, spec_config.tag());
    cargo_toml::create(crate_name, tags, default_tag, &io::join(output_folder, "Cargo.toml")?)?;
    lib_rs::create(tags, &io::join(src_folder, "lib.rs")?, false)?;
    let readme = ReadmeMd {
        crate_name,
        readme_url: readme_md::url(spec.readme().as_str()),
        tags,
        default_tag,
        operation_totals,
        api_version_totals,
        api_versions,
    };
    readme.create(&io::join(output_folder, "README.md")?)?;

    Ok(())
}
