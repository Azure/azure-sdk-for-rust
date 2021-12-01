// cargo run --example gen_mgmt --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/compute/resource-manager
use autorust_codegen::{self, cargo_toml, config_parser::to_mod_name, get_mgmt_readmes, lib_rs, path, Config, PropertyName, SpecReadme};
use std::{collections::HashSet, fs, path::PathBuf};

const OUTPUT_FOLDER: &str = "../mgmt";

const ONLY_SERVICES: &[&str] = &["vmware"];

const SKIP_SERVICES: &[&str] = &[
    "datamigration",
    "deviceprovisioningservices", // TODO #82 certificate_name used as parameter more than once
    "dnc",                        // https://github.com/Azure/azure-rest-api-specs/pull/11578 two ControllerDetails types
    "iotspaces",                  // no operations
    "m365securityandcompliance",  // can't find privateLinkServicesForO365ManagementActivityAPI.json
    "marketplace",
    "mixedreality",  // TODO #83 AccountKeyRegenerateRequest not generated
    "service-map",   // Ident "Ref:machine"
    "servicefabric", // https://github.com/Azure/azure-rest-api-specs/pull/11581 allOf mistakes and duplicate Operations_List
    "servicefabricmanagedclusters",
];

const SKIP_SERVICE_TAGS: &[(&str, &str)] = &[
    ("applicationinsights", "package-preview-2020-06"), // defines operation `list` multiple times
    ("analysisservices", "package-2017-08"),
    ("authorization", "package-2020-10-01-preview"),
    ("authorization", "package-2018-05-01-preview"),
    ("authorization", "package-2021-03-01-preview-only"),
    ("authorization", "package-2021-07-01-preview-only"),
    ("azureactivedirectory", "package-preview-2020-07"),
    ("consumption", "package-2018-03"), // defines get_balances_by_billing_account twice
    ("consumption", "package-2019-11"), // ReservationRecommendationDetails_Get has a path and query param both named "scope"
    ("consumption", "package-2021-05"),
    ("databricks", "package-2021-04-01-preview"), // duplicate tag https://github.com/Azure/azure-rest-api-specs/issues/14995
    // datamigration, same error for all
    // SchemaNotFound MigrateSqlServerSqlDbTask.json ValidationStatus, but may be buried
    ("datamigration", "package-2018-07-15-preview"),
    ("datamigration", "package-2018-04-19"),
    ("datamigration", "package-2018-03-31-preview"),
    ("datamigration", "package-2018-03-15-preview"),
    ("datamigration", "package-2017-11-15-preview"),
    ("datamigration", "package-2021-06"),
    ("deploymentmanager", "package-2018-09-01-preview"), //  identifiers are bound more than once in param list.   https://github.com/Azure/azure-sdk-for-rust/issues/415
    ("iothub", "package-preview-2021-07"),               // duplicate tag https://github.com/Azure/azure-rest-api-specs/issues/16692
    ("iothub", "package-2021-07"),                       // duplicate tag https://github.com/Azure/azure-rest-api-specs/issues/16692
    ("mediaservices", "package-2019-05-preview"), // invalid unicode character of a dash instead of a hyphen https://github.com/Azure/azure-rest-api-specs/pull/11576
    ("marketplace", "package-2020-01-01"),
    ("marketplace", "package-2020-12-01"),
    ("marketplace", "package-composite-v1"),             // mixing versions
    ("marketplace", "package-composite-v2"),             // mixing versions
    ("recoveryservicesbackup", "package-2020-07"),       // duplicate fn get_operation_status
    ("recoveryservicesbackup", "package-2020-10"),       // duplicate fn get_operation_status
    ("recoveryservicessiterecovery", "package-2016-08"), // duplicate package-2016-08 https://github.com/Azure/azure-rest-api-specs/pull/11287
    ("resources", "package-policy-2020-03"),
    ("resources", "package-policy-2020-09"), // SchemaNotFound { ref_key: RefKey { file_path: "../../../azure-rest-api-specs/specification/resources/resource-manager/Microsoft.Authorization/stable/2020-09-01/dataPolicyManifests.json", name: "CloudError"
    ("security", "package-2020-01-preview-only"), // duplicate tag https://github.com/Azure/azure-rest-api-specs/pull/13828
    ("security", "package-2019-08-only"),    // defines `start_time_utc` param twice.
];

// because of a bug in compute specs, some properties need to be forced to be optional
// https://github.com/Azure/azure-rest-api-specs/issues/14459
// https://github.com/Azure/azure-sdk-for-rust/issues/54
const OPTIONAL_PROPERTIES: &[(&str, &str, &str)] = &[
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/preview/2016-04-30-preview/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2015-06-15/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2016-03-30/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2017-03-30/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2017-12-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2018-04-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2018-06-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2018-10-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2019-03-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2019-07-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2019-12-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2020-06-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2020-12-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2021-03-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2021-04-01/compute.json",
        "Resource",
        "location",
    ),
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2021-07-01/compute.json",
        "Resource",
        "location",
    ),
];

// because of recursive types, some properties have to be boxed
// https://github.com/ctaggart/autorust/issues/73
const BOX_PROPERTIES: &[(&str, &str, &str)] = &[
    // cost-management
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/preview/2019-03-01-preview/costmanagement.json", "QueryFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/preview/2019-03-01-preview/costmanagement.json", "ReportConfigFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/preview/2019-03-01-preview/costmanagement.json", "Scope", "childScope"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/preview/2019-04-01-preview/costmanagement.json", "QueryFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/preview/2019-04-01-preview/costmanagement.json", "ReportConfigFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/preview/2019-04-01-preview/costmanagement.json", "Scope", "childScope"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2018-05-31/costmanagement.json", "QueryFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2018-05-31/costmanagement.json", "ReportConfigFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2019-01-01/costmanagement.json", "QueryFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2019-01-01/costmanagement.json", "ReportConfigFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2019-09-01/costmanagement.json", "QueryFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2019-09-01/costmanagement.json", "ReportConfigFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2019-10-01/costmanagement.json", "QueryFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2019-10-01/costmanagement.json", "ReportConfigFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2020-06-01/costmanagement.json", "QueryFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2020-06-01/costmanagement.json", "ReportConfigFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2021-10-01/costmanagement.json", "QueryFilter", "not"),
    ("../../../azure-rest-api-specs/specification/cost-management/resource-manager/Microsoft.CostManagement/stable/2021-10-01/costmanagement.json", "ReportConfigFilter", "not"),
    // databox
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/preview/2021-08-01-preview/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/preview/2021-08-01-preview/databox.json", "transferAllDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2020-04-01/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2020-04-01/databox.json", "transferAllDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2020-11-01/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2020-11-01/databox.json", "transferAllDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2021-03-01/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2021-03-01/databox.json", "transferAllDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2021-05-01/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2021-05-01/databox.json", "transferAllDetails", "include"),
    // dataprotection
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/stable/2021-01-01/dataprotection.json", "InnerError", "embeddedInnerError"),
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/stable/2021-07-01/dataprotection.json", "InnerError", "embeddedInnerError"),
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/preview/2021-02-01-preview/dataprotection.json", "InnerError", "embeddedInnerError"),
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/preview/2021-06-01-preview/dataprotection.json", "InnerError", "embeddedInnerError"),
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/preview/2021-10-01-preview/dataprotection.json", "InnerError", "embeddedInnerError"),
    // hardwaresecuritymodels
    ("../../../azure-rest-api-specs/specification/hardwaresecuritymodules/resource-manager/Microsoft.HardwareSecurityModules/preview/2018-10-31-preview/dedicatedhsm.json", "Error", "innererror"),
    // logic
    ("../../../azure-rest-api-specs/specification/logic/resource-manager/Microsoft.Logic/stable/2019-05-01/logic.json", "SwaggerSchema", "items"),
    // migrateprojects
    ("../../../azure-rest-api-specs/specification/migrateprojects/resource-manager/Microsoft.Migrate/preview/2018-09-01-preview/migrate.json", "IEdmNavigationProperty", "partner"),
    ("../../../azure-rest-api-specs/specification/migrateprojects/resource-manager/Microsoft.Migrate/preview/2018-09-01-preview/migrate.json", "IEdmStructuredType", "baseType"),
    // network
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2015-06-15/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2015-06-15/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-03-30/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-03-30/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-06-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-06-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-09-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-09-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-12-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-12-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-03-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-03-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-06-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-06-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-08-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-08-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-09-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-09-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-10-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-10-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-11-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2017-11-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-01-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-01-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-02-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-02-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-04-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-04-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-06-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-06-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-07-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-07-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-08-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-08-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-10-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-10-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-11-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-11-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-12-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2018-12-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-02-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-02-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-04-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-04-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-06-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-06-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-07-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-07-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-08-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-08-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-09-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-09-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-11-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-11-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-12-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2019-12-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-03-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-03-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-04-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-04-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-05-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-05-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-06-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-06-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-07-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-07-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-08-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-08-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-11-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2020-11-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-02-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-02-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-03-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-03-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-05-01/customIpPrefix.json", "CustomIpPrefix", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-05-01/publicIpAddress.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-06-01/network.json", "PublicIPAddressPropertiesFormat", "ipConfiguration"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-06-01/network.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-06-01/network.json", "IPConfigurationPropertiesFormat", "publicIPAddress"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-06-01/network.json", "IPConfiguration", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-03-30/network.json", "PublicIPAddressPropertiesFormat", "ipConfiguration"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-03-30/network.json", "PublicIPAddress", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-03-30/network.json", "IPConfigurationPropertiesFormat", "publicIPAddress"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2016-03-30/network.json", "IPConfiguration", "properties"),
    // operationalinsights
    ("../../../azure-rest-api-specs/specification/operationalinsights/resource-manager/Microsoft.OperationalInsights/preview/2019-09-01-preview/QueryPackQueries_API.json", "ErrorInfo", "innererror"),
    ("../../../azure-rest-api-specs/specification/operationalinsights/resource-manager/Microsoft.OperationalInsights/preview/2019-09-01-preview/QueryPacks_API.json", "ErrorInfo", "innererror"),
    // keyvault
    ("../../../azure-rest-api-specs/specification/keyvault/resource-manager/Microsoft.KeyVault/preview/2020-04-01-preview/managedHsm.json", "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/resource-manager/Microsoft.KeyVault/preview/2021-04-01-preview/managedHsm.json", "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/resource-manager/Microsoft.KeyVault/preview/2021-06-01-preview/managedHsm.json", "Error" , "innererror"),
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
    for (i, spec) in get_mgmt_readmes()
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
    let service_name = &spec.service_name();
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
            doc_file: PathBuf::from(file_path),
            schema_name: schema_name.to_string(),
            property_name: property_name.to_string(),
        });
    }

    let mut optional_properties = HashSet::new();
    for (file_path, schema_name, property_name) in OPTIONAL_PROPERTIES {
        optional_properties.insert(PropertyName {
            doc_file: PathBuf::from(file_path),
            schema_name: schema_name.to_string(),
            property_name: property_name.to_string(),
        });
    }

    for config in spec.configs() {
        let tag = config.tag.as_str();
        if skip_service_tags.contains(&(spec.spec(), tag)) {
            println!("  skipping {}", tag);
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
            optional_properties: optional_properties.clone(),
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
