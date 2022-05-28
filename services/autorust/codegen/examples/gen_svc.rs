// cargo run --example gen_svc --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/batch/data-plane
use autorust_codegen::{self, gen::gen_crate, get_svc_readmes, Result, RunConfig};

const OUTPUT_FOLDER: &str = "../svc";

const ONLY_SERVICES: &[&str] = &["applicationinsights"];

const INVALID_TYPE_WORKAROUND: &[(&str, &str, &str)] = &[
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
    for (i, spec) in get_svc_readmes()?.iter().enumerate() {
        if !ONLY_SERVICES.is_empty() {
            if ONLY_SERVICES.contains(&spec.spec()) {
                println!("{} {}", i + 1, spec.spec());
                gen_crate(spec, run_config, OUTPUT_FOLDER)?;
            }
        } else {
            println!("{} {}", i + 1, spec.spec());
            gen_crate(spec, run_config, OUTPUT_FOLDER)?;
        }
    }
    Ok(())
}
