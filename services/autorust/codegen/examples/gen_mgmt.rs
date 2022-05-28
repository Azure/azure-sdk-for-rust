// cargo run --example gen_mgmt --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/compute/resource-manager
use autorust_codegen::{
    self, autorust_toml, cargo_toml, get_mgmt_readmes, io, lib_rs,
    readme_md::{self, ReadmeMd},
    CrateConfig, Error, Result, RunConfig, SpecReadme,
};
use std::{collections::HashMap, fs};

const OUTPUT_FOLDER: &str = "../mgmt";

const ONLY_SERVICES: &[&str] = &[];

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
    (
        "../../../azure-rest-api-specs/specification/compute/resource-manager/Microsoft.Compute/stable/2021-11-01/compute.json",
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
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2021-12-01/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2021-12-01/databox.json", "transferAllDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2022-03-01/databox.json", "transferAllDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2022-02-01/databox.json", "transferFilterDetails", "include"),
    ("../../../azure-rest-api-specs/specification/databox/resource-manager/Microsoft.DataBox/stable/2022-02-01/databox.json", "transferAllDetails", "include"),
    // dataprotection
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/stable/2021-01-01/dataprotection.json", "InnerError", "embeddedInnerError"),
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/stable/2021-07-01/dataprotection.json", "InnerError", "embeddedInnerError"),
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/stable/2022-01-01/dataprotection.json", "InnerError", "embeddedInnerError"),
    ("../../../azure-rest-api-specs/specification/dataprotection/resource-manager/Microsoft.DataProtection/stable/2022-03-01/dataprotection.json", "InnerError", "embeddedInnerError"),
    // hardwaresecuritymodels
    ("../../../azure-rest-api-specs/specification/hardwaresecuritymodules/resource-manager/Microsoft.HardwareSecurityModules/preview/2018-10-31-preview/dedicatedhsm.json", "Error", "innererror"),
    ("../../../azure-rest-api-specs/specification/hardwaresecuritymodules/resource-manager/Microsoft.HardwareSecurityModules/stable/2021-11-30/dedicatedhsm.json", "Error", "innererror"),
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
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-08-01/networkInterface.json", "IPConfiguration", "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-08-01/networkInterface.json", "IPConfigurationPropertiesFormat", "publicIPAddress"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-08-01/publicIpAddress.json", "PublicIPAddress" , "properties"),
    ("../../../azure-rest-api-specs/specification/network/resource-manager/Microsoft.Network/stable/2021-08-01/publicIpAddress.json", "PublicIPAddressPropertiesFormat", "ipConfiguration"),
    // operationalinsights
    ("../../../azure-rest-api-specs/specification/operationalinsights/resource-manager/Microsoft.OperationalInsights/preview/2019-09-01-preview/QueryPackQueries_API.json", "ErrorInfo", "innererror"),
    ("../../../azure-rest-api-specs/specification/operationalinsights/resource-manager/Microsoft.OperationalInsights/preview/2019-09-01-preview/QueryPacks_API.json", "ErrorInfo", "innererror"),
    // keyvault
    ("../../../azure-rest-api-specs/specification/keyvault/resource-manager/Microsoft.KeyVault/preview/2020-04-01-preview/managedHsm.json", "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/resource-manager/Microsoft.KeyVault/preview/2021-04-01-preview/managedHsm.json", "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/resource-manager/Microsoft.KeyVault/preview/2021-06-01-preview/managedHsm.json", "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/resource-manager/Microsoft.KeyVault/preview/2021-11-01-preview/managedHsm.json", "Error" , "innererror"),
    ("../../../azure-rest-api-specs/specification/keyvault/resource-manager/Microsoft.KeyVault/stable/2021-10-01/managedHsm.json", "Error", "innererror"),
];

fn main() -> Result<()> {
    let run_config = &mut RunConfig::new("azure_mgmt_");
    run_config.set_box_properties(BOX_PROPERTIES);
    run_config.set_optional_properties(OPTIONAL_PROPERTIES);

    for (i, spec) in get_mgmt_readmes()?.iter().enumerate() {
        if !ONLY_SERVICES.is_empty() {
            if ONLY_SERVICES.contains(&spec.spec()) {
                println!("{} {}", i + 1, spec.spec());
                gen_crate(spec, run_config)?;
            }
        } else {
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
    let mut package_config = autorust_toml::read(&io::join(&output_folder, "autorust.toml")?)?;
    if package_config.tags.sort.is_none() {
        package_config.tags.sort = Some(true);
    }
    if package_config.tags.deny_contains_only.is_none() {
        package_config.tags.deny_contains_only = Some(true);
    }
    if package_config.tags.limit.is_none() {
        package_config.tags.limit = Some(5);
    }
    // TODO remove skip_service_tags and use the autorust.toml files
    let tags = &package_config.filter_tags(spec_config.tags());
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

    let default_tag_name = if let Some(name) = package_config.default_tag() {
        Some(name)
    } else if let Some(name) = spec_config.tag() {
        Some(name)
    } else {
        None
    };
    let default_tag = cargo_toml::get_default_tag(tags, default_tag_name);
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
