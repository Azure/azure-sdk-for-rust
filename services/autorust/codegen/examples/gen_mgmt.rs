// cargo run --example gen_mgmt --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/compute/resource-manager
use autorust_codegen::{self, gen::gen_crate, get_mgmt_readmes, Result, RunConfig};

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
    for (i, spec) in get_mgmt_readmes()?.iter().enumerate() {
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
