#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1CredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen1CredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen1CredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
}
impl AdlsGen1CredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1DataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen1DataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1MsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen1MsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen1MsiScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
}
impl AdlsGen1MsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen1Properties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AdlsGen1Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1ScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen1ScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen1ScanRulesetProperties {
    #[serde(flatten)]
    pub scanning_rule_scan_ruleset_properties: ScanningRuleScanRulesetProperties,
}
impl AdlsGen1ScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1SystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen1SystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2CredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen2CredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen2CredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
}
impl AdlsGen2CredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2DataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen2DataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2MsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen2MsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen2MsiScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
}
impl AdlsGen2MsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen2Properties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AdlsGen2Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2ScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen2ScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen2ScanRulesetProperties {
    #[serde(flatten)]
    pub scanning_rule_scan_ruleset_properties: ScanningRuleScanRulesetProperties,
}
impl AdlsGen2ScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2SystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AdlsGen2SystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonAccountCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonAccountCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonAccountCredentialScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AmazonAccountCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonAccountDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonAccountDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonAccountProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "awsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}
impl AmazonAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonAccountScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonAccountScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonAccountScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AmazonAccountScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonAccountSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonAccountSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonPostgreSqlCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonPostgreSqlCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonPostgreSqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonPostgreSqlDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonPostgreSqlProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonPostgreSqlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlRoleArnScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonPostgreSqlRoleArnScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonPostgreSqlRoleArnScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonPostgreSqlRoleArnScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonPostgreSqlScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonPostgreSqlScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AmazonPostgreSqlScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonPostgreSqlSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3CredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonS3CredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonS3CredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "isMauiScan", default, skip_serializing_if = "Option::is_none")]
    pub is_maui_scan: Option<bool>,
}
impl AmazonS3CredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3DataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonS3DataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonS3Properties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}
impl AmazonS3Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3RoleArnScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonS3RoleArnScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonS3RoleArnScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "isMauiScan", default, skip_serializing_if = "Option::is_none")]
    pub is_maui_scan: Option<bool>,
}
impl AmazonS3RoleArnScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3ScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonS3ScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonS3ScanRulesetProperties {
    #[serde(flatten)]
    pub scanning_rule_scan_ruleset_properties: ScanningRuleScanRulesetProperties,
}
impl AmazonS3ScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3SystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonS3SystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonSqlCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonSqlCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonSqlCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonSqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonSqlDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonSqlDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonSqlProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonSqlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonSqlScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonSqlScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonSqlScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AmazonSqlScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonSqlSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AmazonSqlSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCosmosDbCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureCosmosDbCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCosmosDbCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
impl AzureCosmosDbCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCosmosDbDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureCosmosDbDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCosmosDbProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "accountUri", default, skip_serializing_if = "Option::is_none")]
    pub account_uri: Option<String>,
}
impl AzureCosmosDbProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCosmosDbScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureCosmosDbScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCosmosDbScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureCosmosDbScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCosmosDbSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureCosmosDbSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureDataExplorerCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataExplorerCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}
impl AzureDataExplorerCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureDataExplorerDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureDataExplorerMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataExplorerMsiScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}
impl AzureDataExplorerMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataExplorerProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AzureDataExplorerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureDataExplorerScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataExplorerScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureDataExplorerScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureDataExplorerSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataSourceProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
impl AzureDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileServiceCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureFileServiceCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileServiceCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
}
impl AzureFileServiceCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileServiceDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureFileServiceDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileServiceProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AzureFileServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileServiceScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureFileServiceScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileServiceScanRulesetProperties {
    #[serde(flatten)]
    pub scanning_rule_scan_ruleset_properties: ScanningRuleScanRulesetProperties,
}
impl AzureFileServiceScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileServiceSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureFileServiceSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVault {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureKeyVault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVaultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureKeyVault>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for AzureKeyVaultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AzureKeyVaultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVaultProperties {
    #[serde(rename = "baseUrl", default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AzureKeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMySqlCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureMySqlCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMySqlCredentialScanProperties {
    #[serde(flatten)]
    pub azure_my_sql_scan_properties: AzureMySqlScanProperties,
}
impl AzureMySqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMySqlDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureMySqlDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMySqlProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl AzureMySqlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMySqlScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
}
impl AzureMySqlScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMySqlScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureMySqlScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMySqlScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureMySqlScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMySqlSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureMySqlSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePostgreSqlCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzurePostgreSqlCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePostgreSqlCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "sslMode", default, skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<i32>,
}
impl AzurePostgreSqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePostgreSqlDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzurePostgreSqlDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePostgreSqlProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl AzurePostgreSqlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePostgreSqlScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzurePostgreSqlScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePostgreSqlScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzurePostgreSqlScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePostgreSqlSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzurePostgreSqlSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureResourceGroupCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceGroupCredentialScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AzureResourceGroupCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureResourceGroupDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureResourceGroupMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceGroupMsiScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AzureResourceGroupMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceGroupProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl AzureResourceGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureResourceGroupScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceGroupScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureResourceGroupScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureResourceGroupSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlCredentialScanProperties {
    #[serde(flatten)]
    pub azure_sql_scan_properties: AzureSqlScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
}
impl AzureSqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDataWarehouseCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDataWarehouseCredentialScanProperties {
    #[serde(flatten)]
    pub azure_sql_credential_scan_properties: AzureSqlCredentialScanProperties,
}
impl AzureSqlDataWarehouseCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDataWarehouseDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDataWarehouseMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDataWarehouseMsiScanProperties {
    #[serde(flatten)]
    pub azure_sql_scan_properties: AzureSqlScanProperties,
}
impl AzureSqlDataWarehouseMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDataWarehouseProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
}
impl AzureSqlDataWarehouseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDataWarehouseScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDataWarehouseScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureSqlDataWarehouseScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDataWarehouseSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseCredentialScanProperties {
    #[serde(flatten)]
    pub azure_sql_credential_scan_properties: AzureSqlCredentialScanProperties,
}
impl AzureSqlDatabaseCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseManagedInstanceCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseManagedInstanceCredentialScanProperties {
    #[serde(flatten)]
    pub azure_sql_credential_scan_properties: AzureSqlCredentialScanProperties,
}
impl AzureSqlDatabaseManagedInstanceCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseManagedInstanceDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseManagedInstanceMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseManagedInstanceMsiScanProperties {
    #[serde(flatten)]
    pub azure_sql_scan_properties: AzureSqlScanProperties,
}
impl AzureSqlDatabaseManagedInstanceMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseManagedInstanceProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
}
impl AzureSqlDatabaseManagedInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseManagedInstanceScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseManagedInstanceScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureSqlDatabaseManagedInstanceScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseManagedInstanceSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseMsiScanProperties {
    #[serde(flatten)]
    pub azure_sql_scan_properties: AzureSqlScanProperties,
}
impl AzureSqlDatabaseMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
}
impl AzureSqlDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureSqlDatabaseScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSqlDatabaseSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
impl AzureSqlScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureStorageCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
}
impl AzureStorageCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureStorageDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureStorageMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageMsiScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
}
impl AzureStorageMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AzureStorageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureStorageScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageScanRulesetProperties {
    #[serde(flatten)]
    pub scanning_rule_scan_ruleset_properties: ScanningRuleScanRulesetProperties,
}
impl AzureStorageScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureStorageSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSubscriptionCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSubscriptionCredentialScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AzureSubscriptionCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSubscriptionDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSubscriptionMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSubscriptionMsiScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AzureSubscriptionMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSubscriptionProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl AzureSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSubscriptionScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSubscriptionScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureSubscriptionScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSubscriptionSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseCredentialScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AzureSynapseCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseMsiScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AzureSynapseMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "sqlEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub sql_endpoint: Option<String>,
    #[serde(rename = "sqlOnDemandEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub sql_on_demand_endpoint: Option<String>,
}
impl AzureSynapseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureSynapseScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseWorkspaceCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseWorkspaceCredentialScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AzureSynapseWorkspaceCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseWorkspaceDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseWorkspaceMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseWorkspaceMsiScanProperties {
    #[serde(flatten)]
    pub expanding_resource_scan_properties: ExpandingResourceScanProperties,
}
impl AzureSynapseWorkspaceMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseWorkspaceProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "dedicatedSqlEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_sql_endpoint: Option<String>,
    #[serde(rename = "serverlessSqlEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub serverless_sql_endpoint: Option<String>,
}
impl AzureSynapseWorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseWorkspaceScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseWorkspaceScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl AzureSynapseWorkspaceScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureSynapseWorkspaceSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassificationRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub kind: classification_rule::Kind,
}
impl ClassificationRule {
    pub fn new(kind: classification_rule::Kind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            kind,
        }
    }
}
pub mod classification_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        System,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::System => serializer.serialize_unit_variant("Kind", 0u32, "System"),
                Self::Custom => serializer.serialize_unit_variant("Kind", 1u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassificationRuleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClassificationRule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ClassificationRuleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClassificationRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassificationRulePattern {
    pub kind: classification_rule_pattern::Kind,
}
impl ClassificationRulePattern {
    pub fn new(kind: classification_rule_pattern::Kind) -> Self {
        Self { kind }
    }
}
pub mod classification_rule_pattern {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Regex,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Regex => serializer.serialize_unit_variant("Kind", 0u32, "Regex"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionReference {
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CollectionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedVia {
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
}
impl ConnectedVia {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialReference {
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[serde(rename = "credentialType", default, skip_serializing_if = "Option::is_none")]
    pub credential_type: Option<credential_reference::CredentialType>,
}
impl CredentialReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod credential_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CredentialType")]
    pub enum CredentialType {
        AccountKey,
        ServicePrincipal,
        BasicAuth,
        SqlAuth,
        #[serde(rename = "AmazonARN")]
        AmazonArn,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CredentialType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CredentialType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CredentialType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AccountKey => serializer.serialize_unit_variant("CredentialType", 0u32, "AccountKey"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("CredentialType", 1u32, "ServicePrincipal"),
                Self::BasicAuth => serializer.serialize_unit_variant("CredentialType", 2u32, "BasicAuth"),
                Self::SqlAuth => serializer.serialize_unit_variant("CredentialType", 3u32, "SqlAuth"),
                Self::AmazonArn => serializer.serialize_unit_variant("CredentialType", 4u32, "AmazonARN"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomClassificationRule {
    #[serde(flatten)]
    pub classification_rule: ClassificationRule,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl CustomClassificationRule {
    pub fn new(classification_rule: ClassificationRule) -> Self {
        Self {
            classification_rule,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomClassificationRuleProperties {
    #[serde(rename = "minimumPercentageMatch", default, skip_serializing_if = "Option::is_none")]
    pub minimum_percentage_match: Option<f64>,
    #[serde(rename = "classificationAction", default, skip_serializing_if = "Option::is_none")]
    pub classification_action: Option<custom_classification_rule_properties::ClassificationAction>,
    #[serde(rename = "dataPatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub data_patterns: Vec<ClassificationRulePattern>,
    #[serde(rename = "columnPatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub column_patterns: Vec<ClassificationRulePattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "classificationName", default, skip_serializing_if = "Option::is_none")]
    pub classification_name: Option<String>,
    #[serde(rename = "ruleStatus", default, skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<custom_classification_rule_properties::RuleStatus>,
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl CustomClassificationRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_classification_rule_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClassificationAction")]
    pub enum ClassificationAction {
        Keep,
        Delete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClassificationAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClassificationAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClassificationAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Keep => serializer.serialize_unit_variant("ClassificationAction", 0u32, "Keep"),
                Self::Delete => serializer.serialize_unit_variant("ClassificationAction", 1u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RuleStatus")]
    pub enum RuleStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RuleStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RuleStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RuleStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RuleStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RuleStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomFileExtension {
    #[serde(rename = "customFileType", default, skip_serializing_if = "Option::is_none")]
    pub custom_file_type: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "fileExtension", default, skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
}
impl CustomFileExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomFileType {
    #[serde(rename = "builtInType", default, skip_serializing_if = "Option::is_none")]
    pub built_in_type: Option<custom_file_type::BuiltInType>,
    #[serde(rename = "customDelimiter", default, skip_serializing_if = "Option::is_none")]
    pub custom_delimiter: Option<String>,
}
impl CustomFileType {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_file_type {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BuiltInType")]
    pub enum BuiltInType {
        #[serde(rename = "AVRO")]
        Avro,
        #[serde(rename = "ORC")]
        Orc,
        #[serde(rename = "PARQUET")]
        Parquet,
        #[serde(rename = "JSON")]
        Json,
        #[serde(rename = "TXT")]
        Txt,
        #[serde(rename = "XML")]
        Xml,
        Documents,
        #[serde(rename = "CSV")]
        Csv,
        #[serde(rename = "PSV")]
        Psv,
        #[serde(rename = "SSV")]
        Ssv,
        #[serde(rename = "TSV")]
        Tsv,
        #[serde(rename = "GZ")]
        Gz,
        #[serde(rename = "DOC")]
        Doc,
        #[serde(rename = "DOCM")]
        Docm,
        #[serde(rename = "DOCX")]
        Docx,
        #[serde(rename = "DOT")]
        Dot,
        #[serde(rename = "ODP")]
        Odp,
        #[serde(rename = "ODS")]
        Ods,
        #[serde(rename = "ODT")]
        Odt,
        #[serde(rename = "PDF")]
        Pdf,
        #[serde(rename = "POT")]
        Pot,
        #[serde(rename = "PPS")]
        Pps,
        #[serde(rename = "PPSX")]
        Ppsx,
        #[serde(rename = "PPT")]
        Ppt,
        #[serde(rename = "PPTM")]
        Pptm,
        #[serde(rename = "PPTX")]
        Pptx,
        #[serde(rename = "XLC")]
        Xlc,
        #[serde(rename = "XLS")]
        Xls,
        #[serde(rename = "XLSB")]
        Xlsb,
        #[serde(rename = "XLSM")]
        Xlsm,
        #[serde(rename = "XLSX")]
        Xlsx,
        #[serde(rename = "XLT")]
        Xlt,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BuiltInType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BuiltInType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BuiltInType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Avro => serializer.serialize_unit_variant("BuiltInType", 0u32, "AVRO"),
                Self::Orc => serializer.serialize_unit_variant("BuiltInType", 1u32, "ORC"),
                Self::Parquet => serializer.serialize_unit_variant("BuiltInType", 2u32, "PARQUET"),
                Self::Json => serializer.serialize_unit_variant("BuiltInType", 3u32, "JSON"),
                Self::Txt => serializer.serialize_unit_variant("BuiltInType", 4u32, "TXT"),
                Self::Xml => serializer.serialize_unit_variant("BuiltInType", 5u32, "XML"),
                Self::Documents => serializer.serialize_unit_variant("BuiltInType", 6u32, "Documents"),
                Self::Csv => serializer.serialize_unit_variant("BuiltInType", 7u32, "CSV"),
                Self::Psv => serializer.serialize_unit_variant("BuiltInType", 8u32, "PSV"),
                Self::Ssv => serializer.serialize_unit_variant("BuiltInType", 9u32, "SSV"),
                Self::Tsv => serializer.serialize_unit_variant("BuiltInType", 10u32, "TSV"),
                Self::Gz => serializer.serialize_unit_variant("BuiltInType", 11u32, "GZ"),
                Self::Doc => serializer.serialize_unit_variant("BuiltInType", 12u32, "DOC"),
                Self::Docm => serializer.serialize_unit_variant("BuiltInType", 13u32, "DOCM"),
                Self::Docx => serializer.serialize_unit_variant("BuiltInType", 14u32, "DOCX"),
                Self::Dot => serializer.serialize_unit_variant("BuiltInType", 15u32, "DOT"),
                Self::Odp => serializer.serialize_unit_variant("BuiltInType", 16u32, "ODP"),
                Self::Ods => serializer.serialize_unit_variant("BuiltInType", 17u32, "ODS"),
                Self::Odt => serializer.serialize_unit_variant("BuiltInType", 18u32, "ODT"),
                Self::Pdf => serializer.serialize_unit_variant("BuiltInType", 19u32, "PDF"),
                Self::Pot => serializer.serialize_unit_variant("BuiltInType", 20u32, "POT"),
                Self::Pps => serializer.serialize_unit_variant("BuiltInType", 21u32, "PPS"),
                Self::Ppsx => serializer.serialize_unit_variant("BuiltInType", 22u32, "PPSX"),
                Self::Ppt => serializer.serialize_unit_variant("BuiltInType", 23u32, "PPT"),
                Self::Pptm => serializer.serialize_unit_variant("BuiltInType", 24u32, "PPTM"),
                Self::Pptx => serializer.serialize_unit_variant("BuiltInType", 25u32, "PPTX"),
                Self::Xlc => serializer.serialize_unit_variant("BuiltInType", 26u32, "XLC"),
                Self::Xls => serializer.serialize_unit_variant("BuiltInType", 27u32, "XLS"),
                Self::Xlsb => serializer.serialize_unit_variant("BuiltInType", 28u32, "XLSB"),
                Self::Xlsm => serializer.serialize_unit_variant("BuiltInType", 29u32, "XLSM"),
                Self::Xlsx => serializer.serialize_unit_variant("BuiltInType", 30u32, "XLSX"),
                Self::Xlt => serializer.serialize_unit_variant("BuiltInType", 31u32, "XLT"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub kind: data_source::Kind,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scans: Vec<Scan>,
}
impl DataSource {
    pub fn new(kind: data_source::Kind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            kind,
            scans: Vec::new(),
        }
    }
}
pub mod data_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        None,
        AzureSubscription,
        AzureResourceGroup,
        AzureSynapseWorkspace,
        AzureSynapse,
        AdlsGen1,
        AdlsGen2,
        AmazonAccount,
        AmazonS3,
        AmazonSql,
        AzureCosmosDb,
        AzureDataExplorer,
        AzureFileService,
        AzureSqlDatabase,
        AmazonPostgreSql,
        AzurePostgreSql,
        SqlServerDatabase,
        AzureSqlDatabaseManagedInstance,
        AzureSqlDataWarehouse,
        AzureMySql,
        AzureStorage,
        Teradata,
        Oracle,
        SapS4Hana,
        SapEcc,
        #[serde(rename = "PowerBI")]
        PowerBi,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Kind", 0u32, "None"),
                Self::AzureSubscription => serializer.serialize_unit_variant("Kind", 1u32, "AzureSubscription"),
                Self::AzureResourceGroup => serializer.serialize_unit_variant("Kind", 2u32, "AzureResourceGroup"),
                Self::AzureSynapseWorkspace => serializer.serialize_unit_variant("Kind", 3u32, "AzureSynapseWorkspace"),
                Self::AzureSynapse => serializer.serialize_unit_variant("Kind", 4u32, "AzureSynapse"),
                Self::AdlsGen1 => serializer.serialize_unit_variant("Kind", 5u32, "AdlsGen1"),
                Self::AdlsGen2 => serializer.serialize_unit_variant("Kind", 6u32, "AdlsGen2"),
                Self::AmazonAccount => serializer.serialize_unit_variant("Kind", 7u32, "AmazonAccount"),
                Self::AmazonS3 => serializer.serialize_unit_variant("Kind", 8u32, "AmazonS3"),
                Self::AmazonSql => serializer.serialize_unit_variant("Kind", 9u32, "AmazonSql"),
                Self::AzureCosmosDb => serializer.serialize_unit_variant("Kind", 10u32, "AzureCosmosDb"),
                Self::AzureDataExplorer => serializer.serialize_unit_variant("Kind", 11u32, "AzureDataExplorer"),
                Self::AzureFileService => serializer.serialize_unit_variant("Kind", 12u32, "AzureFileService"),
                Self::AzureSqlDatabase => serializer.serialize_unit_variant("Kind", 13u32, "AzureSqlDatabase"),
                Self::AmazonPostgreSql => serializer.serialize_unit_variant("Kind", 14u32, "AmazonPostgreSql"),
                Self::AzurePostgreSql => serializer.serialize_unit_variant("Kind", 15u32, "AzurePostgreSql"),
                Self::SqlServerDatabase => serializer.serialize_unit_variant("Kind", 16u32, "SqlServerDatabase"),
                Self::AzureSqlDatabaseManagedInstance => {
                    serializer.serialize_unit_variant("Kind", 17u32, "AzureSqlDatabaseManagedInstance")
                }
                Self::AzureSqlDataWarehouse => serializer.serialize_unit_variant("Kind", 18u32, "AzureSqlDataWarehouse"),
                Self::AzureMySql => serializer.serialize_unit_variant("Kind", 19u32, "AzureMySql"),
                Self::AzureStorage => serializer.serialize_unit_variant("Kind", 20u32, "AzureStorage"),
                Self::Teradata => serializer.serialize_unit_variant("Kind", 21u32, "Teradata"),
                Self::Oracle => serializer.serialize_unit_variant("Kind", 22u32, "Oracle"),
                Self::SapS4Hana => serializer.serialize_unit_variant("Kind", 23u32, "SapS4Hana"),
                Self::SapEcc => serializer.serialize_unit_variant("Kind", 24u32, "SapEcc"),
                Self::PowerBi => serializer.serialize_unit_variant("Kind", 25u32, "PowerBI"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataSource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for DataSourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataSourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceProperties {
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<serde_json::Value>,
}
impl DataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorInfo>,
}
impl ErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorModel>,
}
impl ErrorModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}
impl azure_core::Continuable for ErrorResponseModel {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpandingResourceScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<expanding_resource_scan_properties::ResourceTypes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
}
impl ExpandingResourceScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod expanding_resource_scan_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ResourceTypes {
        #[serde(rename = "None", default, skip_serializing_if = "Option::is_none")]
        pub none: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureSubscription", default, skip_serializing_if = "Option::is_none")]
        pub azure_subscription: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureResourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub azure_resource_group: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureSynapseWorkspace", default, skip_serializing_if = "Option::is_none")]
        pub azure_synapse_workspace: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureSynapse", default, skip_serializing_if = "Option::is_none")]
        pub azure_synapse: Option<ResourceTypeFilter>,
        #[serde(rename = "AdlsGen1", default, skip_serializing_if = "Option::is_none")]
        pub adls_gen1: Option<ResourceTypeFilter>,
        #[serde(rename = "AdlsGen2", default, skip_serializing_if = "Option::is_none")]
        pub adls_gen2: Option<ResourceTypeFilter>,
        #[serde(rename = "AmazonAccount", default, skip_serializing_if = "Option::is_none")]
        pub amazon_account: Option<ResourceTypeFilter>,
        #[serde(rename = "AmazonS3", default, skip_serializing_if = "Option::is_none")]
        pub amazon_s3: Option<ResourceTypeFilter>,
        #[serde(rename = "AmazonSql", default, skip_serializing_if = "Option::is_none")]
        pub amazon_sql: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureCosmosDb", default, skip_serializing_if = "Option::is_none")]
        pub azure_cosmos_db: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureDataExplorer", default, skip_serializing_if = "Option::is_none")]
        pub azure_data_explorer: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureFileService", default, skip_serializing_if = "Option::is_none")]
        pub azure_file_service: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureSqlDatabase", default, skip_serializing_if = "Option::is_none")]
        pub azure_sql_database: Option<ResourceTypeFilter>,
        #[serde(rename = "AmazonPostgreSql", default, skip_serializing_if = "Option::is_none")]
        pub amazon_postgre_sql: Option<ResourceTypeFilter>,
        #[serde(rename = "AzurePostgreSql", default, skip_serializing_if = "Option::is_none")]
        pub azure_postgre_sql: Option<ResourceTypeFilter>,
        #[serde(rename = "SqlServerDatabase", default, skip_serializing_if = "Option::is_none")]
        pub sql_server_database: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureSqlDatabaseManagedInstance", default, skip_serializing_if = "Option::is_none")]
        pub azure_sql_database_managed_instance: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureSqlDataWarehouse", default, skip_serializing_if = "Option::is_none")]
        pub azure_sql_data_warehouse: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureMySql", default, skip_serializing_if = "Option::is_none")]
        pub azure_my_sql: Option<ResourceTypeFilter>,
        #[serde(rename = "AzureStorage", default, skip_serializing_if = "Option::is_none")]
        pub azure_storage: Option<ResourceTypeFilter>,
        #[serde(rename = "Teradata", default, skip_serializing_if = "Option::is_none")]
        pub teradata: Option<ResourceTypeFilter>,
        #[serde(rename = "Oracle", default, skip_serializing_if = "Option::is_none")]
        pub oracle: Option<ResourceTypeFilter>,
        #[serde(rename = "SapS4Hana", default, skip_serializing_if = "Option::is_none")]
        pub sap_s4_hana: Option<ResourceTypeFilter>,
        #[serde(rename = "SapEcc", default, skip_serializing_if = "Option::is_none")]
        pub sap_ecc: Option<ResourceTypeFilter>,
        #[serde(rename = "PowerBI", default, skip_serializing_if = "Option::is_none")]
        pub power_bi: Option<ResourceTypeFilter>,
    }
    impl ResourceTypes {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filter {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Filter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilterProperties {
    #[serde(rename = "excludeUriPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_uri_prefixes: Vec<String>,
    #[serde(rename = "includeUriPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub include_uri_prefixes: Vec<String>,
}
impl FilterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MitiScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(rename = "maximumMemoryAllowedInGb", default, skip_serializing_if = "Option::is_none")]
    pub maximum_memory_allowed_in_gb: Option<String>,
    #[serde(rename = "mitiCache", default, skip_serializing_if = "Option::is_none")]
    pub miti_cache: Option<String>,
}
impl MitiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Notification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}
impl Notification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[serde(rename = "scanResultId", default, skip_serializing_if = "Option::is_none")]
    pub scan_result_id: Option<String>,
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_response::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        InProgress,
        TransientFailure,
        Succeeded,
        Failed,
        Canceled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::TransientFailure => serializer.serialize_unit_variant("Status", 2u32, "TransientFailure"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OracleDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleOracleCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OracleOracleCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleOracleCredentialScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "driverLocation", default, skip_serializing_if = "Option::is_none")]
    pub driver_location: Option<String>,
}
impl OracleOracleCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleOracleUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OracleOracleUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleOracleUserPassScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "driverLocation", default, skip_serializing_if = "Option::is_none")]
    pub driver_location: Option<String>,
}
impl OracleOracleUserPassScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}
impl OracleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OracleScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl OracleScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OracleSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PowerBiDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiDelegatedScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PowerBiDelegatedScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PowerBiDelegatedScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "includePersonalWorkspaces", default, skip_serializing_if = "Option::is_none")]
    pub include_personal_workspaces: Option<bool>,
}
impl PowerBiDelegatedScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PowerBiMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PowerBiMsiScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(rename = "includePersonalWorkspaces", default, skip_serializing_if = "Option::is_none")]
    pub include_personal_workspaces: Option<bool>,
}
impl PowerBiMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PowerBiProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}
impl PowerBiProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PowerBiScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PowerBiScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl PowerBiScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PowerBiSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecurrenceSchedule {
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub minutes: Vec<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hours: Vec<i32>,
    #[serde(rename = "weekDays", default, skip_serializing_if = "Vec::is_empty")]
    pub week_days: Vec<String>,
    #[serde(rename = "monthDays", default, skip_serializing_if = "Vec::is_empty")]
    pub month_days: Vec<i32>,
    #[serde(rename = "monthlyOccurrences", default, skip_serializing_if = "Vec::is_empty")]
    pub monthly_occurrences: Vec<RecurrenceScheduleOccurrence>,
}
impl RecurrenceSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecurrenceScheduleOccurrence {
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<recurrence_schedule_occurrence::Day>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<i32>,
}
impl RecurrenceScheduleOccurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recurrence_schedule_occurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Day")]
    pub enum Day {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Day {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Day {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Day {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sunday => serializer.serialize_unit_variant("Day", 0u32, "Sunday"),
                Self::Monday => serializer.serialize_unit_variant("Day", 1u32, "Monday"),
                Self::Tuesday => serializer.serialize_unit_variant("Day", 2u32, "Tuesday"),
                Self::Wednesday => serializer.serialize_unit_variant("Day", 3u32, "Wednesday"),
                Self::Thursday => serializer.serialize_unit_variant("Day", 4u32, "Thursday"),
                Self::Friday => serializer.serialize_unit_variant("Day", 5u32, "Friday"),
                Self::Saturday => serializer.serialize_unit_variant("Day", 6u32, "Saturday"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegexClassificationRulePattern {
    #[serde(flatten)]
    pub classification_rule_pattern: ClassificationRulePattern,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}
impl RegexClassificationRulePattern {
    pub fn new(classification_rule_pattern: ClassificationRulePattern) -> Self {
        Self {
            classification_rule_pattern,
            pattern: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNameFilter {
    #[serde(rename = "excludePrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_prefixes: Vec<String>,
    #[serde(rename = "includePrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub include_prefixes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
}
impl ResourceNameFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeFilter {
    #[serde(rename = "scanRulesetName", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_name: Option<String>,
    #[serde(rename = "scanRulesetType", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_type: Option<resource_type_filter::ScanRulesetType>,
    #[serde(rename = "resourceNameFilter", default, skip_serializing_if = "Option::is_none")]
    pub resource_name_filter: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
}
impl ResourceTypeFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_type_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScanRulesetType")]
    pub enum ScanRulesetType {
        Custom,
        System,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScanRulesetType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScanRulesetType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScanRulesetType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Custom => serializer.serialize_unit_variant("ScanRulesetType", 0u32, "Custom"),
                Self::System => serializer.serialize_unit_variant("ScanRulesetType", 1u32, "System"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapEccDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapEccProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "applicationServer", default, skip_serializing_if = "Option::is_none")]
    pub application_server: Option<String>,
    #[serde(rename = "systemNumber", default, skip_serializing_if = "Option::is_none")]
    pub system_number: Option<String>,
}
impl SapEccProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccSapEccCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapEccSapEccCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapEccSapEccCredentialScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "jCoLibraryPath", default, skip_serializing_if = "Option::is_none")]
    pub j_co_library_path: Option<String>,
}
impl SapEccSapEccCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccSapEccUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapEccSapEccUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapEccSapEccUserPassScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "jCoLibraryPath", default, skip_serializing_if = "Option::is_none")]
    pub j_co_library_path: Option<String>,
}
impl SapEccSapEccUserPassScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapEccScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapEccScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl SapEccScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapEccSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapS4HanaDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapS4HanaProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(rename = "applicationServer", default, skip_serializing_if = "Option::is_none")]
    pub application_server: Option<String>,
    #[serde(rename = "systemNumber", default, skip_serializing_if = "Option::is_none")]
    pub system_number: Option<String>,
}
impl SapS4HanaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaSapS4HanaCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapS4HanaSapS4HanaCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapS4HanaSapS4HanaCredentialScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(rename = "jCoLibraryPath", default, skip_serializing_if = "Option::is_none")]
    pub j_co_library_path: Option<String>,
}
impl SapS4HanaSapS4HanaCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaSapS4HanaUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapS4HanaSapS4HanaUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapS4HanaSapS4HanaUserPassScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "jCoLibraryPath", default, skip_serializing_if = "Option::is_none")]
    pub j_co_library_path: Option<String>,
}
impl SapS4HanaSapS4HanaUserPassScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapS4HanaScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapS4HanaScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl SapS4HanaScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SapS4HanaSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scan {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub kind: scan::Kind,
    #[serde(rename = "scanResults", default, skip_serializing_if = "Vec::is_empty")]
    pub scan_results: Vec<ScanResult>,
}
impl Scan {
    pub fn new(kind: scan::Kind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            kind,
            scan_results: Vec::new(),
        }
    }
}
pub mod scan {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        AzureSubscriptionCredential,
        AzureSubscriptionMsi,
        AzureResourceGroupCredential,
        AzureResourceGroupMsi,
        AzureSynapseWorkspaceCredential,
        AzureSynapseWorkspaceMsi,
        AzureSynapseCredential,
        AzureSynapseMsi,
        AdlsGen1Credential,
        AdlsGen1Msi,
        AdlsGen2Credential,
        AdlsGen2Msi,
        AmazonAccountCredential,
        AmazonS3Credential,
        #[serde(rename = "AmazonS3RoleARN")]
        AmazonS3RoleArn,
        AmazonSqlCredential,
        AzureCosmosDbCredential,
        AzureDataExplorerCredential,
        AzureDataExplorerMsi,
        AzureFileServiceCredential,
        AzureSqlDatabaseCredential,
        AzureSqlDatabaseMsi,
        AmazonPostgreSqlCredential,
        #[serde(rename = "AmazonPostgreSqlRoleARN")]
        AmazonPostgreSqlRoleArn,
        AzurePostgreSqlCredential,
        SqlServerDatabaseCredential,
        AzureSqlDatabaseManagedInstanceCredential,
        AzureSqlDatabaseManagedInstanceMsi,
        AzureSqlDataWarehouseCredential,
        AzureSqlDataWarehouseMsi,
        AzureMySqlCredential,
        AzureStorageCredential,
        AzureStorageMsi,
        TeradataTeradataCredential,
        TeradataTeradataUserPass,
        TeradataUserPass,
        OracleOracleCredential,
        OracleOracleUserPass,
        SapS4HanaSapS4HanaCredential,
        SapS4HanaSapS4HanaUserPass,
        SapEccSapEccCredential,
        SapEccSapEccUserPass,
        #[serde(rename = "PowerBIDelegated")]
        PowerBiDelegated,
        #[serde(rename = "PowerBIMsi")]
        PowerBiMsi,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureSubscriptionCredential => serializer.serialize_unit_variant("Kind", 0u32, "AzureSubscriptionCredential"),
                Self::AzureSubscriptionMsi => serializer.serialize_unit_variant("Kind", 1u32, "AzureSubscriptionMsi"),
                Self::AzureResourceGroupCredential => serializer.serialize_unit_variant("Kind", 2u32, "AzureResourceGroupCredential"),
                Self::AzureResourceGroupMsi => serializer.serialize_unit_variant("Kind", 3u32, "AzureResourceGroupMsi"),
                Self::AzureSynapseWorkspaceCredential => serializer.serialize_unit_variant("Kind", 4u32, "AzureSynapseWorkspaceCredential"),
                Self::AzureSynapseWorkspaceMsi => serializer.serialize_unit_variant("Kind", 5u32, "AzureSynapseWorkspaceMsi"),
                Self::AzureSynapseCredential => serializer.serialize_unit_variant("Kind", 6u32, "AzureSynapseCredential"),
                Self::AzureSynapseMsi => serializer.serialize_unit_variant("Kind", 7u32, "AzureSynapseMsi"),
                Self::AdlsGen1Credential => serializer.serialize_unit_variant("Kind", 8u32, "AdlsGen1Credential"),
                Self::AdlsGen1Msi => serializer.serialize_unit_variant("Kind", 9u32, "AdlsGen1Msi"),
                Self::AdlsGen2Credential => serializer.serialize_unit_variant("Kind", 10u32, "AdlsGen2Credential"),
                Self::AdlsGen2Msi => serializer.serialize_unit_variant("Kind", 11u32, "AdlsGen2Msi"),
                Self::AmazonAccountCredential => serializer.serialize_unit_variant("Kind", 12u32, "AmazonAccountCredential"),
                Self::AmazonS3Credential => serializer.serialize_unit_variant("Kind", 13u32, "AmazonS3Credential"),
                Self::AmazonS3RoleArn => serializer.serialize_unit_variant("Kind", 14u32, "AmazonS3RoleARN"),
                Self::AmazonSqlCredential => serializer.serialize_unit_variant("Kind", 15u32, "AmazonSqlCredential"),
                Self::AzureCosmosDbCredential => serializer.serialize_unit_variant("Kind", 16u32, "AzureCosmosDbCredential"),
                Self::AzureDataExplorerCredential => serializer.serialize_unit_variant("Kind", 17u32, "AzureDataExplorerCredential"),
                Self::AzureDataExplorerMsi => serializer.serialize_unit_variant("Kind", 18u32, "AzureDataExplorerMsi"),
                Self::AzureFileServiceCredential => serializer.serialize_unit_variant("Kind", 19u32, "AzureFileServiceCredential"),
                Self::AzureSqlDatabaseCredential => serializer.serialize_unit_variant("Kind", 20u32, "AzureSqlDatabaseCredential"),
                Self::AzureSqlDatabaseMsi => serializer.serialize_unit_variant("Kind", 21u32, "AzureSqlDatabaseMsi"),
                Self::AmazonPostgreSqlCredential => serializer.serialize_unit_variant("Kind", 22u32, "AmazonPostgreSqlCredential"),
                Self::AmazonPostgreSqlRoleArn => serializer.serialize_unit_variant("Kind", 23u32, "AmazonPostgreSqlRoleARN"),
                Self::AzurePostgreSqlCredential => serializer.serialize_unit_variant("Kind", 24u32, "AzurePostgreSqlCredential"),
                Self::SqlServerDatabaseCredential => serializer.serialize_unit_variant("Kind", 25u32, "SqlServerDatabaseCredential"),
                Self::AzureSqlDatabaseManagedInstanceCredential => {
                    serializer.serialize_unit_variant("Kind", 26u32, "AzureSqlDatabaseManagedInstanceCredential")
                }
                Self::AzureSqlDatabaseManagedInstanceMsi => {
                    serializer.serialize_unit_variant("Kind", 27u32, "AzureSqlDatabaseManagedInstanceMsi")
                }
                Self::AzureSqlDataWarehouseCredential => {
                    serializer.serialize_unit_variant("Kind", 28u32, "AzureSqlDataWarehouseCredential")
                }
                Self::AzureSqlDataWarehouseMsi => serializer.serialize_unit_variant("Kind", 29u32, "AzureSqlDataWarehouseMsi"),
                Self::AzureMySqlCredential => serializer.serialize_unit_variant("Kind", 30u32, "AzureMySqlCredential"),
                Self::AzureStorageCredential => serializer.serialize_unit_variant("Kind", 31u32, "AzureStorageCredential"),
                Self::AzureStorageMsi => serializer.serialize_unit_variant("Kind", 32u32, "AzureStorageMsi"),
                Self::TeradataTeradataCredential => serializer.serialize_unit_variant("Kind", 33u32, "TeradataTeradataCredential"),
                Self::TeradataTeradataUserPass => serializer.serialize_unit_variant("Kind", 34u32, "TeradataTeradataUserPass"),
                Self::TeradataUserPass => serializer.serialize_unit_variant("Kind", 35u32, "TeradataUserPass"),
                Self::OracleOracleCredential => serializer.serialize_unit_variant("Kind", 36u32, "OracleOracleCredential"),
                Self::OracleOracleUserPass => serializer.serialize_unit_variant("Kind", 37u32, "OracleOracleUserPass"),
                Self::SapS4HanaSapS4HanaCredential => serializer.serialize_unit_variant("Kind", 38u32, "SapS4HanaSapS4HanaCredential"),
                Self::SapS4HanaSapS4HanaUserPass => serializer.serialize_unit_variant("Kind", 39u32, "SapS4HanaSapS4HanaUserPass"),
                Self::SapEccSapEccCredential => serializer.serialize_unit_variant("Kind", 40u32, "SapEccSapEccCredential"),
                Self::SapEccSapEccUserPass => serializer.serialize_unit_variant("Kind", 41u32, "SapEccSapEccUserPass"),
                Self::PowerBiDelegated => serializer.serialize_unit_variant("Kind", 42u32, "PowerBIDelegated"),
                Self::PowerBiMsi => serializer.serialize_unit_variant("Kind", 43u32, "PowerBIMsi"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanDiagnostics {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notifications: Vec<Notification>,
    #[serde(rename = "exceptionCountMap", default, skip_serializing_if = "Option::is_none")]
    pub exception_count_map: Option<serde_json::Value>,
}
impl ScanDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanHistoryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScanResult>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ScanHistoryList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScanHistoryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Scan>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ScanList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScanList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanProperties {
    #[serde(rename = "scanRulesetName", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_name: Option<String>,
    #[serde(rename = "scanRulesetType", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_type: Option<scan_properties::ScanRulesetType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<i32>,
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[serde(rename = "connectedVia", default, skip_serializing_if = "Option::is_none")]
    pub connected_via: Option<serde_json::Value>,
}
impl ScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scan_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScanRulesetType")]
    pub enum ScanRulesetType {
        Custom,
        System,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScanRulesetType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScanRulesetType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScanRulesetType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Custom => serializer.serialize_unit_variant("ScanRulesetType", 0u32, "Custom"),
                Self::System => serializer.serialize_unit_variant("ScanRulesetType", 1u32, "System"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanResult {
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "assetsDiscovered", default, skip_serializing_if = "Option::is_none")]
    pub assets_discovered: Option<i64>,
    #[serde(rename = "assetsClassified", default, skip_serializing_if = "Option::is_none")]
    pub assets_classified: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<serde_json::Value>,
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[serde(rename = "queuedTime", with = "azure_core::date::rfc3339::option")]
    pub queued_time: Option<time::OffsetDateTime>,
    #[serde(rename = "pipelineStartTime", with = "azure_core::date::rfc3339::option")]
    pub pipeline_start_time: Option<time::OffsetDateTime>,
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[serde(rename = "scanRulesetVersion", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_version: Option<i32>,
    #[serde(rename = "scanRulesetType", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_type: Option<scan_result::ScanRulesetType>,
    #[serde(rename = "scanLevelType", default, skip_serializing_if = "Option::is_none")]
    pub scan_level_type: Option<scan_result::ScanLevelType>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    #[serde(rename = "dataSourceType", default, skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<scan_result::DataSourceType>,
}
impl ScanResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scan_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScanRulesetType")]
    pub enum ScanRulesetType {
        Custom,
        System,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScanRulesetType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScanRulesetType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScanRulesetType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Custom => serializer.serialize_unit_variant("ScanRulesetType", 0u32, "Custom"),
                Self::System => serializer.serialize_unit_variant("ScanRulesetType", 1u32, "System"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScanLevelType")]
    pub enum ScanLevelType {
        Full,
        Incremental,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScanLevelType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScanLevelType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScanLevelType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Full => serializer.serialize_unit_variant("ScanLevelType", 0u32, "Full"),
                Self::Incremental => serializer.serialize_unit_variant("ScanLevelType", 1u32, "Incremental"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSourceType")]
    pub enum DataSourceType {
        None,
        AzureSubscription,
        AzureResourceGroup,
        AzureSynapseWorkspace,
        AzureSynapse,
        AdlsGen1,
        AdlsGen2,
        AmazonAccount,
        AmazonS3,
        AmazonSql,
        AzureCosmosDb,
        AzureDataExplorer,
        AzureFileService,
        AzureSqlDatabase,
        AmazonPostgreSql,
        AzurePostgreSql,
        SqlServerDatabase,
        AzureSqlDatabaseManagedInstance,
        AzureSqlDataWarehouse,
        AzureMySql,
        AzureStorage,
        Teradata,
        Oracle,
        SapS4Hana,
        SapEcc,
        #[serde(rename = "PowerBI")]
        PowerBi,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("DataSourceType", 0u32, "None"),
                Self::AzureSubscription => serializer.serialize_unit_variant("DataSourceType", 1u32, "AzureSubscription"),
                Self::AzureResourceGroup => serializer.serialize_unit_variant("DataSourceType", 2u32, "AzureResourceGroup"),
                Self::AzureSynapseWorkspace => serializer.serialize_unit_variant("DataSourceType", 3u32, "AzureSynapseWorkspace"),
                Self::AzureSynapse => serializer.serialize_unit_variant("DataSourceType", 4u32, "AzureSynapse"),
                Self::AdlsGen1 => serializer.serialize_unit_variant("DataSourceType", 5u32, "AdlsGen1"),
                Self::AdlsGen2 => serializer.serialize_unit_variant("DataSourceType", 6u32, "AdlsGen2"),
                Self::AmazonAccount => serializer.serialize_unit_variant("DataSourceType", 7u32, "AmazonAccount"),
                Self::AmazonS3 => serializer.serialize_unit_variant("DataSourceType", 8u32, "AmazonS3"),
                Self::AmazonSql => serializer.serialize_unit_variant("DataSourceType", 9u32, "AmazonSql"),
                Self::AzureCosmosDb => serializer.serialize_unit_variant("DataSourceType", 10u32, "AzureCosmosDb"),
                Self::AzureDataExplorer => serializer.serialize_unit_variant("DataSourceType", 11u32, "AzureDataExplorer"),
                Self::AzureFileService => serializer.serialize_unit_variant("DataSourceType", 12u32, "AzureFileService"),
                Self::AzureSqlDatabase => serializer.serialize_unit_variant("DataSourceType", 13u32, "AzureSqlDatabase"),
                Self::AmazonPostgreSql => serializer.serialize_unit_variant("DataSourceType", 14u32, "AmazonPostgreSql"),
                Self::AzurePostgreSql => serializer.serialize_unit_variant("DataSourceType", 15u32, "AzurePostgreSql"),
                Self::SqlServerDatabase => serializer.serialize_unit_variant("DataSourceType", 16u32, "SqlServerDatabase"),
                Self::AzureSqlDatabaseManagedInstance => {
                    serializer.serialize_unit_variant("DataSourceType", 17u32, "AzureSqlDatabaseManagedInstance")
                }
                Self::AzureSqlDataWarehouse => serializer.serialize_unit_variant("DataSourceType", 18u32, "AzureSqlDataWarehouse"),
                Self::AzureMySql => serializer.serialize_unit_variant("DataSourceType", 19u32, "AzureMySql"),
                Self::AzureStorage => serializer.serialize_unit_variant("DataSourceType", 20u32, "AzureStorage"),
                Self::Teradata => serializer.serialize_unit_variant("DataSourceType", 21u32, "Teradata"),
                Self::Oracle => serializer.serialize_unit_variant("DataSourceType", 22u32, "Oracle"),
                Self::SapS4Hana => serializer.serialize_unit_variant("DataSourceType", 23u32, "SapS4Hana"),
                Self::SapEcc => serializer.serialize_unit_variant("DataSourceType", 24u32, "SapEcc"),
                Self::PowerBi => serializer.serialize_unit_variant("DataSourceType", 25u32, "PowerBI"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScanRuleset {
    #[serde(flatten)]
    pub versioned_scan_ruleset: VersionedScanRuleset,
    pub kind: scan_ruleset::Kind,
}
impl ScanRuleset {
    pub fn new(kind: scan_ruleset::Kind) -> Self {
        Self {
            versioned_scan_ruleset: VersionedScanRuleset::default(),
            kind,
        }
    }
}
pub mod scan_ruleset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        None,
        AzureSubscription,
        AzureResourceGroup,
        AzureSynapseWorkspace,
        AzureSynapse,
        AdlsGen1,
        AdlsGen2,
        AmazonAccount,
        AmazonS3,
        AmazonSql,
        AzureCosmosDb,
        AzureDataExplorer,
        AzureFileService,
        AzureSqlDatabase,
        AmazonPostgreSql,
        AzurePostgreSql,
        SqlServerDatabase,
        AzureSqlDatabaseManagedInstance,
        AzureSqlDataWarehouse,
        AzureMySql,
        AzureStorage,
        Teradata,
        Oracle,
        SapS4Hana,
        SapEcc,
        #[serde(rename = "PowerBI")]
        PowerBi,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Kind", 0u32, "None"),
                Self::AzureSubscription => serializer.serialize_unit_variant("Kind", 1u32, "AzureSubscription"),
                Self::AzureResourceGroup => serializer.serialize_unit_variant("Kind", 2u32, "AzureResourceGroup"),
                Self::AzureSynapseWorkspace => serializer.serialize_unit_variant("Kind", 3u32, "AzureSynapseWorkspace"),
                Self::AzureSynapse => serializer.serialize_unit_variant("Kind", 4u32, "AzureSynapse"),
                Self::AdlsGen1 => serializer.serialize_unit_variant("Kind", 5u32, "AdlsGen1"),
                Self::AdlsGen2 => serializer.serialize_unit_variant("Kind", 6u32, "AdlsGen2"),
                Self::AmazonAccount => serializer.serialize_unit_variant("Kind", 7u32, "AmazonAccount"),
                Self::AmazonS3 => serializer.serialize_unit_variant("Kind", 8u32, "AmazonS3"),
                Self::AmazonSql => serializer.serialize_unit_variant("Kind", 9u32, "AmazonSql"),
                Self::AzureCosmosDb => serializer.serialize_unit_variant("Kind", 10u32, "AzureCosmosDb"),
                Self::AzureDataExplorer => serializer.serialize_unit_variant("Kind", 11u32, "AzureDataExplorer"),
                Self::AzureFileService => serializer.serialize_unit_variant("Kind", 12u32, "AzureFileService"),
                Self::AzureSqlDatabase => serializer.serialize_unit_variant("Kind", 13u32, "AzureSqlDatabase"),
                Self::AmazonPostgreSql => serializer.serialize_unit_variant("Kind", 14u32, "AmazonPostgreSql"),
                Self::AzurePostgreSql => serializer.serialize_unit_variant("Kind", 15u32, "AzurePostgreSql"),
                Self::SqlServerDatabase => serializer.serialize_unit_variant("Kind", 16u32, "SqlServerDatabase"),
                Self::AzureSqlDatabaseManagedInstance => {
                    serializer.serialize_unit_variant("Kind", 17u32, "AzureSqlDatabaseManagedInstance")
                }
                Self::AzureSqlDataWarehouse => serializer.serialize_unit_variant("Kind", 18u32, "AzureSqlDataWarehouse"),
                Self::AzureMySql => serializer.serialize_unit_variant("Kind", 19u32, "AzureMySql"),
                Self::AzureStorage => serializer.serialize_unit_variant("Kind", 20u32, "AzureStorage"),
                Self::Teradata => serializer.serialize_unit_variant("Kind", 21u32, "Teradata"),
                Self::Oracle => serializer.serialize_unit_variant("Kind", 22u32, "Oracle"),
                Self::SapS4Hana => serializer.serialize_unit_variant("Kind", 23u32, "SapS4Hana"),
                Self::SapEcc => serializer.serialize_unit_variant("Kind", 24u32, "SapEcc"),
                Self::PowerBi => serializer.serialize_unit_variant("Kind", 25u32, "PowerBI"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanRulesetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScanRuleset>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ScanRulesetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScanRulesetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanRulesetProperties {
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "excludedSystemClassifications", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_system_classifications: Vec<String>,
    #[serde(rename = "includedCustomClassificationRuleNames", default, skip_serializing_if = "Vec::is_empty")]
    pub included_custom_classification_rule_names: Vec<String>,
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl ScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanningRule {
    #[serde(rename = "fileExtensions", default, skip_serializing_if = "Vec::is_empty")]
    pub file_extensions: Vec<String>,
    #[serde(rename = "customFileExtensions", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_file_extensions: Vec<CustomFileExtension>,
}
impl ScanningRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanningRuleScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
    #[serde(rename = "scanningRule", default, skip_serializing_if = "Option::is_none")]
    pub scanning_rule: Option<serde_json::Value>,
}
impl ScanningRuleScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SqlServerDatabaseCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerDatabaseCredentialScanProperties {
    #[serde(flatten)]
    pub azure_sql_credential_scan_properties: AzureSqlCredentialScanProperties,
}
impl SqlServerDatabaseCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SqlServerDatabaseDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerDatabaseProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
}
impl SqlServerDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SqlServerDatabaseScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerDatabaseScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl SqlServerDatabaseScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SqlServerDatabaseSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemClassificationRule {
    #[serde(flatten)]
    pub classification_rule: ClassificationRule,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SystemClassificationRule {
    pub fn new(classification_rule: ClassificationRule) -> Self {
        Self {
            classification_rule,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemClassificationRuleProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "classificationName", default, skip_serializing_if = "Option::is_none")]
    pub classification_name: Option<String>,
    #[serde(rename = "ruleStatus", default, skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<system_classification_rule_properties::RuleStatus>,
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemClassificationRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_classification_rule_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RuleStatus")]
    pub enum RuleStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RuleStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RuleStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RuleStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RuleStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RuleStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemScanRuleset {
    #[serde(flatten)]
    pub versioned_scan_ruleset: VersionedScanRuleset,
    pub kind: system_scan_ruleset::Kind,
}
impl SystemScanRuleset {
    pub fn new(kind: system_scan_ruleset::Kind) -> Self {
        Self {
            versioned_scan_ruleset: VersionedScanRuleset::default(),
            kind,
        }
    }
}
pub mod system_scan_ruleset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        None,
        AzureSubscription,
        AzureResourceGroup,
        AzureSynapseWorkspace,
        AzureSynapse,
        AdlsGen1,
        AdlsGen2,
        AmazonAccount,
        AmazonS3,
        AmazonSql,
        AzureCosmosDb,
        AzureDataExplorer,
        AzureFileService,
        AzureSqlDatabase,
        AmazonPostgreSql,
        AzurePostgreSql,
        SqlServerDatabase,
        AzureSqlDatabaseManagedInstance,
        AzureSqlDataWarehouse,
        AzureMySql,
        AzureStorage,
        Teradata,
        Oracle,
        SapS4Hana,
        SapEcc,
        #[serde(rename = "PowerBI")]
        PowerBi,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Kind", 0u32, "None"),
                Self::AzureSubscription => serializer.serialize_unit_variant("Kind", 1u32, "AzureSubscription"),
                Self::AzureResourceGroup => serializer.serialize_unit_variant("Kind", 2u32, "AzureResourceGroup"),
                Self::AzureSynapseWorkspace => serializer.serialize_unit_variant("Kind", 3u32, "AzureSynapseWorkspace"),
                Self::AzureSynapse => serializer.serialize_unit_variant("Kind", 4u32, "AzureSynapse"),
                Self::AdlsGen1 => serializer.serialize_unit_variant("Kind", 5u32, "AdlsGen1"),
                Self::AdlsGen2 => serializer.serialize_unit_variant("Kind", 6u32, "AdlsGen2"),
                Self::AmazonAccount => serializer.serialize_unit_variant("Kind", 7u32, "AmazonAccount"),
                Self::AmazonS3 => serializer.serialize_unit_variant("Kind", 8u32, "AmazonS3"),
                Self::AmazonSql => serializer.serialize_unit_variant("Kind", 9u32, "AmazonSql"),
                Self::AzureCosmosDb => serializer.serialize_unit_variant("Kind", 10u32, "AzureCosmosDb"),
                Self::AzureDataExplorer => serializer.serialize_unit_variant("Kind", 11u32, "AzureDataExplorer"),
                Self::AzureFileService => serializer.serialize_unit_variant("Kind", 12u32, "AzureFileService"),
                Self::AzureSqlDatabase => serializer.serialize_unit_variant("Kind", 13u32, "AzureSqlDatabase"),
                Self::AmazonPostgreSql => serializer.serialize_unit_variant("Kind", 14u32, "AmazonPostgreSql"),
                Self::AzurePostgreSql => serializer.serialize_unit_variant("Kind", 15u32, "AzurePostgreSql"),
                Self::SqlServerDatabase => serializer.serialize_unit_variant("Kind", 16u32, "SqlServerDatabase"),
                Self::AzureSqlDatabaseManagedInstance => {
                    serializer.serialize_unit_variant("Kind", 17u32, "AzureSqlDatabaseManagedInstance")
                }
                Self::AzureSqlDataWarehouse => serializer.serialize_unit_variant("Kind", 18u32, "AzureSqlDataWarehouse"),
                Self::AzureMySql => serializer.serialize_unit_variant("Kind", 19u32, "AzureMySql"),
                Self::AzureStorage => serializer.serialize_unit_variant("Kind", 20u32, "AzureStorage"),
                Self::Teradata => serializer.serialize_unit_variant("Kind", 21u32, "Teradata"),
                Self::Oracle => serializer.serialize_unit_variant("Kind", 22u32, "Oracle"),
                Self::SapS4Hana => serializer.serialize_unit_variant("Kind", 23u32, "SapS4Hana"),
                Self::SapEcc => serializer.serialize_unit_variant("Kind", 24u32, "SapEcc"),
                Self::PowerBi => serializer.serialize_unit_variant("Kind", 25u32, "PowerBI"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemScanRulesetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SystemScanRuleset>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for SystemScanRulesetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SystemScanRulesetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TeradataDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}
impl TeradataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TeradataScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
}
impl TeradataScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TeradataSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataTeradataCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TeradataTeradataCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataTeradataCredentialScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "driverLocation", default, skip_serializing_if = "Option::is_none")]
    pub driver_location: Option<String>,
}
impl TeradataTeradataCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataTeradataUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TeradataTeradataUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataTeradataUserPassScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "driverLocation", default, skip_serializing_if = "Option::is_none")]
    pub driver_location: Option<String>,
}
impl TeradataTeradataUserPassScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TeradataUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataUserPassScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl TeradataUserPassScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Trigger {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Trigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<serde_json::Value>,
    #[serde(rename = "recurrenceInterval", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_interval: Option<String>,
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[serde(rename = "lastScheduled", with = "azure_core::date::rfc3339::option")]
    pub last_scheduled: Option<time::OffsetDateTime>,
    #[serde(rename = "scanLevel", default, skip_serializing_if = "Option::is_none")]
    pub scan_level: Option<trigger_properties::ScanLevel>,
    #[serde(rename = "incrementalScanStartTime", with = "azure_core::date::rfc3339::option")]
    pub incremental_scan_start_time: Option<time::OffsetDateTime>,
}
impl TriggerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod trigger_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScanLevel")]
    pub enum ScanLevel {
        Full,
        Incremental,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScanLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScanLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScanLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Full => serializer.serialize_unit_variant("ScanLevel", 0u32, "Full"),
                Self::Incremental => serializer.serialize_unit_variant("ScanLevel", 1u32, "Incremental"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerRecurrence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<trigger_recurrence::Frequency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<serde_json::Value>,
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
impl TriggerRecurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod trigger_recurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Frequency")]
    pub enum Frequency {
        Week,
        Month,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Frequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Frequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Frequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Week => serializer.serialize_unit_variant("Frequency", 0u32, "Week"),
                Self::Month => serializer.serialize_unit_variant("Frequency", 1u32, "Month"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionedScanRuleset {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "scanRulesetType", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_type: Option<versioned_scan_ruleset::ScanRulesetType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<versioned_scan_ruleset::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl VersionedScanRuleset {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod versioned_scan_ruleset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScanRulesetType")]
    pub enum ScanRulesetType {
        Custom,
        System,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScanRulesetType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScanRulesetType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScanRulesetType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Custom => serializer.serialize_unit_variant("ScanRulesetType", 0u32, "Custom"),
                Self::System => serializer.serialize_unit_variant("ScanRulesetType", 1u32, "System"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
