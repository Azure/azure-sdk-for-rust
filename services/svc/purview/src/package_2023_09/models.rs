#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Credential type that uses Account Key for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountKeyAuthAzureKeyVaultCredential {
    #[serde(flatten)]
    pub credential: Credential,
    #[doc = "Properties of account key credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountKeyCredentialProperties>,
}
impl AccountKeyAuthAzureKeyVaultCredential {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            properties: None,
        }
    }
}
#[doc = "Properties of account key credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountKeyCredentialProperties {
    #[doc = "Properties of key vault secret account key credential type."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<KeyVaultSecretAccountKeyCredentialTypeProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AccountKeyCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ADLS Gen1 credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1CredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "ADLS Gen1 credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen1CredentialScanProperties>,
}
impl AdlsGen1CredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "ADLS Gen1 credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen1CredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
}
impl AdlsGen1CredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ADLS Gen1 data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1DataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of ADLS Gen1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen1Properties>,
}
impl AdlsGen1DataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "ADLS Gen1 MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1MsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "ADLS Gen1 MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen1MsiScanProperties>,
}
impl AdlsGen1MsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "ADLS Gen1 MSI scan properties."]
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
#[doc = "The properties of ADLS Gen1."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen1Properties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The endpoint of ADLS Gen1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AdlsGen1Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ADLS Gen1 scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1ScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "ADLS Gen1 scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen1ScanRulesetProperties>,
}
impl AdlsGen1ScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "ADLS Gen1 scan ruleset properties."]
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
#[doc = "ADLS Gen1 System scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1SystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "ADLS Gen1 scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen1ScanRulesetProperties>,
}
impl AdlsGen1SystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "ADLS Gen2 credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2CredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "ADLS Gen2 credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen2CredentialScanProperties>,
}
impl AdlsGen2CredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "ADLS Gen2 credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen2CredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
}
impl AdlsGen2CredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ADLS Gen2 data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2DataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of ADLS Gen2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen2Properties>,
}
impl AdlsGen2DataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "ADLS Gen2 MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2MsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "ADLS Gen2 MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen2MsiScanProperties>,
}
impl AdlsGen2MsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "ADLS Gen2 MSI scan properties."]
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
#[doc = "The properties of ADLS Gen2."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen2Properties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The endpoint of ADLS Gen2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AdlsGen2Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Adls gen 2 scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2ScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Adls gen 2 scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen2ScanRulesetProperties>,
}
impl AdlsGen2ScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Adls gen 2 scan ruleset properties."]
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
#[doc = "ADLS Gen2 system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2SystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Adls gen 2 scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdlsGen2ScanRulesetProperties>,
}
impl AdlsGen2SystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Amazon account credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonAccountCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Amazon account credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonAccountCredentialScanProperties>,
}
impl AmazonAccountCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Amazon account credential scan properties."]
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
#[doc = "The Amazon account data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonAccountDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Amazon account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonAccountProperties>,
}
impl AmazonAccountDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of Amazon account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonAccountProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "AWS account id."]
    #[serde(rename = "awsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[doc = "Role arn."]
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}
impl AmazonAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Amazon account scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonAccountScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Amazon account scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonAccountScanRulesetProperties>,
}
impl AmazonAccountScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Amazon account scan ruleset properties."]
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
#[doc = "Amazon account system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonAccountSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Amazon account scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonAccountScanRulesetProperties>,
}
impl AmazonAccountSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Amazon Postgre SQL credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Amazon Postgre SQL credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonPostgreSqlCredentialScanProperties>,
}
impl AmazonPostgreSqlCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Amazon Postgre SQL credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonPostgreSqlCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[doc = "The endpoint of Amazon Postgre SQL server."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[doc = "The database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The port of Amazon Postgre SQL server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The vpc endpoint service name of Amazon Postgre SQL server."]
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonPostgreSqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Amazon Postgre SQL data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Amazon Postgre SQL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonPostgreSqlProperties>,
}
impl AmazonPostgreSqlDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of Amazon Postgre SQL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonPostgreSqlProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "The server endpoint of Amazon Postgre SQL."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[doc = "The port of Amazon Postgre SQL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The vpc endpoint service name of Amazon Postgre SQL."]
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonPostgreSqlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Postgre SQL scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure Postgre SQL scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonPostgreSqlScanRulesetProperties>,
}
impl AmazonPostgreSqlScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure Postgre SQL scan ruleset properties."]
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
#[doc = "Amazon Postgre SQL system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonPostgreSqlSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure Postgre SQL scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonPostgreSqlScanRulesetProperties>,
}
impl AmazonPostgreSqlSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Amazon S3 credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3CredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Amazon S3 credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonS3CredentialScanProperties>,
}
impl AmazonS3CredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Amazon S3 credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonS3CredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
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
#[doc = "The Amazon S3 data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3DataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Amazon S3."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonS3Properties>,
}
impl AmazonS3DataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of Amazon S3."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonS3Properties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "Service URL."]
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[doc = "Role ARN."]
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}
impl AmazonS3Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Amazon S3 role ARN scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3RoleArnScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Amazon S3 role ARN scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonS3RoleArnScanProperties>,
}
impl AmazonS3RoleArnScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Amazon S3 role ARN scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonS3RoleArnScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[doc = "The role ARN of the scan."]
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[doc = "The flag to indicate whether the scan is a Maui scan or not."]
    #[serde(rename = "isMauiScan", default, skip_serializing_if = "Option::is_none")]
    pub is_maui_scan: Option<bool>,
}
impl AmazonS3RoleArnScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Amazon S3 scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3ScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Amazon S3 scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonS3ScanRulesetProperties>,
}
impl AmazonS3ScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Amazon S3 scan ruleset properties."]
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
#[doc = "Amazon S3 system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonS3SystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Amazon S3 scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonS3ScanRulesetProperties>,
}
impl AmazonS3SystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Amazon SQL credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonSqlCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Amazon SQL credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonSqlCredentialScanProperties>,
}
impl AmazonSqlCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Amazon SQL credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonSqlCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[doc = "The endpoint of Amazon SQL server."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[doc = "The database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The port of Amazon SQL server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The VPC endpoint service name of Amazon SQL server."]
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonSqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Amazon SQL data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonSqlDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Amazon SQL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonSqlProperties>,
}
impl AmazonSqlDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of Amazon SQL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmazonSqlProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "Server Endpoint."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[doc = "Port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Vpc Endpoint Service Name."]
    #[serde(rename = "vpcEndpointServiceName", default, skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}
impl AmazonSqlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Amazon SQL scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonSqlScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Amazon SQL scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonSqlScanRulesetProperties>,
}
impl AmazonSqlScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Amazon SQL scan ruleset properties."]
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
#[doc = "Amazon SQL system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmazonSqlSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Amazon SQL scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmazonSqlScanRulesetProperties>,
}
impl AmazonSqlSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure Cosmos DB credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCosmosDbCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure Cosmos DB credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureCosmosDbCredentialScanProperties>,
}
impl AzureCosmosDbCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure Cosmos DB credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCosmosDbCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
impl AzureCosmosDbCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure Cosmos DB data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCosmosDbDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Azure Cosmos DB."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureCosmosDbProperties>,
}
impl AzureCosmosDbDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of Azure Cosmos DB."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCosmosDbProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The account URI of Azure Cosmos DB."]
    #[serde(rename = "accountUri", default, skip_serializing_if = "Option::is_none")]
    pub account_uri: Option<String>,
}
impl AzureCosmosDbProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Cosmos DB scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCosmosDbScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure Cosmos DB scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureCosmosDbScanRulesetProperties>,
}
impl AzureCosmosDbScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure Cosmos DB scan ruleset properties."]
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
#[doc = "Azure Cosmos DB system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCosmosDbSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure Cosmos DB scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureCosmosDbScanRulesetProperties>,
}
impl AzureCosmosDbSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure data Explorer credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure data Explorer credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDataExplorerCredentialScanProperties>,
}
impl AzureDataExplorerCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure data Explorer credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataExplorerCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}
impl AzureDataExplorerCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The azure data explorer data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of azure data explorer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDataExplorerProperties>,
}
impl AzureDataExplorerDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure data Explorer MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure data Explorer MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDataExplorerMsiScanProperties>,
}
impl AzureDataExplorerMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure data Explorer MSI scan properties."]
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
#[doc = "The properties of azure data explorer."]
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
#[doc = "Azure data explorer scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure data explorer scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDataExplorerScanRulesetProperties>,
}
impl AzureDataExplorerScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure data explorer scan ruleset properties."]
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
#[doc = "Azure data explorer system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataExplorerSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure data explorer scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDataExplorerScanRulesetProperties>,
}
impl AzureDataExplorerSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure data source properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataSourceProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "Resource group."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Subscription id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Resource id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Data use governance."]
    #[serde(rename = "dataUseGovernance", default, skip_serializing_if = "Option::is_none")]
    pub data_use_governance: Option<azure_data_source_properties::DataUseGovernance>,
}
impl AzureDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_data_source_properties {
    use super::*;
    #[doc = "Data use governance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataUseGovernance")]
    pub enum DataUseGovernance {
        Disabled,
        DisabledByAnotherAccount,
        Enabled,
        EnabledAtAncestorScope,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataUseGovernance {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataUseGovernance {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataUseGovernance {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("DataUseGovernance", 0u32, "Disabled"),
                Self::DisabledByAnotherAccount => serializer.serialize_unit_variant("DataUseGovernance", 1u32, "DisabledByAnotherAccount"),
                Self::Enabled => serializer.serialize_unit_variant("DataUseGovernance", 2u32, "Enabled"),
                Self::EnabledAtAncestorScope => serializer.serialize_unit_variant("DataUseGovernance", 3u32, "EnabledAtAncestorScope"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure file service credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileServiceCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure file service credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureFileServiceCredentialScanProperties>,
}
impl AzureFileServiceCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure file service credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileServiceCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
}
impl AzureFileServiceCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The azure file service data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileServiceDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of azure file service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureFileServiceProperties>,
}
impl AzureFileServiceDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of azure file service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileServiceProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The endpoint of azure file service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AzureFileServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure file service scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileServiceScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure file service scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureFileServiceScanRulesetProperties>,
}
impl AzureFileServiceScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure file service scan ruleset properties."]
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
#[doc = "Azure file service system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileServiceSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure file service scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureFileServiceScanRulesetProperties>,
}
impl AzureFileServiceSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "The Azure Key Vault connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVault {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Azure Key Vault connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureKeyVaultProperties>,
}
impl AzureKeyVault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Azure Key Vault connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureKeyVaultList {
    #[doc = "List of Azure Key Vault connections."]
    pub value: Vec<AzureKeyVault>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of Azure Key Vault connections."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for AzureKeyVaultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureKeyVaultList {
    pub fn new(value: Vec<AzureKeyVault>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "Azure Key Vault connection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVaultProperties {
    #[doc = "The base URL of the Azure Key Vault."]
    #[serde(rename = "baseUrl", default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[doc = "The description of the Azure Key Vault connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AzureKeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure MySQL credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMySqlCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure MySQL credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMySqlCredentialScanProperties>,
}
impl AzureMySqlCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure MySQL credential scan properties."]
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
#[doc = "Azure MySQL data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMySqlDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "Azure MySQL data source properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMySqlProperties>,
}
impl AzureMySqlDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure MySQL data source properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMySqlProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The server endpoint of azure my sql."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[doc = "The port of azure my sql."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl AzureMySqlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure MySQL scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMySqlScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The endpoint of the Azure MySQL server."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[doc = "The port of the Azure MySQL server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
}
impl AzureMySqlScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure MySQL scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMySqlScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure MySQL scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMySqlScanRulesetProperties>,
}
impl AzureMySqlScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure MySQL scan ruleset properties."]
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
#[doc = "Azure MySQL system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMySqlSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure MySQL scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMySqlScanRulesetProperties>,
}
impl AzureMySqlSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure Postgre SQL credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePostgreSqlCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure Postgre SQL credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzurePostgreSqlCredentialScanProperties>,
}
impl AzurePostgreSqlCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure Postgre SQL credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePostgreSqlCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[doc = "The endpoint of Azure Postgre SQL server."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[doc = "The database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The port of Azure Postgre SQL server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The SSL mode of Azure Postgre SQL server."]
    #[serde(rename = "sslMode", default, skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<i32>,
}
impl AzurePostgreSqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure Postgre SQL data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePostgreSqlDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Azure Postgre SQL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzurePostgreSqlProperties>,
}
impl AzurePostgreSqlDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of Azure Postgre SQL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePostgreSqlProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The server endpoint of Azure Postgre SQL."]
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
#[doc = "Azure Postgre SQL scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePostgreSqlScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure Postgre SQL scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzurePostgreSqlScanRulesetProperties>,
}
impl AzurePostgreSqlScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure Postgre SQL scan ruleset properties."]
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
#[doc = "Azure Postgre SQL system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePostgreSqlSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure Postgre SQL scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzurePostgreSqlScanRulesetProperties>,
}
impl AzurePostgreSqlSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure resource group credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure resource group credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureResourceGroupCredentialScanProperties>,
}
impl AzureResourceGroupCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure resource group credential scan properties."]
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
#[doc = "Azure resource group data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "Azure resource group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureResourceGroupProperties>,
}
impl AzureResourceGroupDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure resource group MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure resource group MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureResourceGroupMsiScanProperties>,
}
impl AzureResourceGroupMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure resource group MSI scan properties."]
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
#[doc = "Azure resource group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceGroupProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "Subscription id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource group."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Resource id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Data use governance."]
    #[serde(rename = "dataUseGovernance", default, skip_serializing_if = "Option::is_none")]
    pub data_use_governance: Option<azure_resource_group_properties::DataUseGovernance>,
}
impl AzureResourceGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_resource_group_properties {
    use super::*;
    #[doc = "Data use governance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataUseGovernance")]
    pub enum DataUseGovernance {
        Disabled,
        DisabledByAnotherAccount,
        Enabled,
        EnabledAtAncestorScope,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataUseGovernance {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataUseGovernance {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataUseGovernance {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("DataUseGovernance", 0u32, "Disabled"),
                Self::DisabledByAnotherAccount => serializer.serialize_unit_variant("DataUseGovernance", 1u32, "DisabledByAnotherAccount"),
                Self::Enabled => serializer.serialize_unit_variant("DataUseGovernance", 2u32, "Enabled"),
                Self::EnabledAtAncestorScope => serializer.serialize_unit_variant("DataUseGovernance", 3u32, "EnabledAtAncestorScope"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure resource group scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure resource group scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureResourceGroupScanRulesetProperties>,
}
impl AzureResourceGroupScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure resource group scan ruleset properties."]
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
#[doc = "Azure resource group system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceGroupSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure resource group scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureResourceGroupScanRulesetProperties>,
}
impl AzureResourceGroupSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure SQL credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlCredentialScanProperties {
    #[serde(flatten)]
    pub azure_sql_scan_properties: AzureSqlScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
}
impl AzureSqlCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure SQL data warehouse credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure SQL data warehouse credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDataWarehouseCredentialScanProperties>,
}
impl AzureSqlDataWarehouseCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure SQL data warehouse credential scan properties."]
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
#[doc = "The Azure SQL data warehouse data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Azure SQL data warehouse."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDataWarehouseProperties>,
}
impl AzureSqlDataWarehouseDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure SQL data warehouse MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure SQL data warehouse MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDataWarehouseMsiScanProperties>,
}
impl AzureSqlDataWarehouseMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure SQL data warehouse MSI scan properties."]
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
#[doc = "The properties of Azure SQL data warehouse."]
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
#[doc = "Azure SQL data warehouse scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure SQL data warehouse scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDataWarehouseScanRulesetProperties>,
}
impl AzureSqlDataWarehouseScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure SQL data warehouse scan ruleset properties."]
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
#[doc = "Azure SQL data warehouse system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDataWarehouseSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure SQL data warehouse scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDataWarehouseScanRulesetProperties>,
}
impl AzureSqlDataWarehouseSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure SQL database credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure SQL database credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseCredentialScanProperties>,
}
impl AzureSqlDatabaseCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure SQL database credential scan properties."]
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
#[doc = "The Azure SQL database data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Azure SQL database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseProperties>,
}
impl AzureSqlDatabaseDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure SQL database managed instance credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure SQL database managed instance credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseManagedInstanceCredentialScanProperties>,
}
impl AzureSqlDatabaseManagedInstanceCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure SQL database managed instance credential scan properties."]
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
#[doc = "The Azure SQL database managed instance data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Azure SQL database managed instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseManagedInstanceProperties>,
}
impl AzureSqlDatabaseManagedInstanceDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure SQL database managed instance MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure SQL database managed instance MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseManagedInstanceMsiScanProperties>,
}
impl AzureSqlDatabaseManagedInstanceMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure SQL database managed instance MSI scan properties."]
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
#[doc = "The properties of Azure SQL database managed instance."]
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
#[doc = "Azure SQL db managed instance scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure SQL db managed instance scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseManagedInstanceScanRulesetProperties>,
}
impl AzureSqlDatabaseManagedInstanceScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure SQL db managed instance scan ruleset properties."]
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
#[doc = "Azure SQL database managed instance system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseManagedInstanceSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure SQL db managed instance scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseManagedInstanceScanRulesetProperties>,
}
impl AzureSqlDatabaseManagedInstanceSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure SQL database MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure SQL database MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseMsiScanProperties>,
}
impl AzureSqlDatabaseMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure SQL database MSI scan properties."]
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
#[doc = "The properties of Azure SQL database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The server endpoint of Azure SQL database."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
}
impl AzureSqlDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure SQL db scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure SQL db scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseScanRulesetProperties>,
}
impl AzureSqlDatabaseScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure SQL db scan ruleset properties."]
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
#[doc = "Azure SQL db system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure SQL db scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseScanRulesetProperties>,
}
impl AzureSqlDatabaseSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure SQL scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The endpoint of Azure SQL server."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[doc = "The database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
impl AzureSqlScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Storage credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure Storage credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureStorageCredentialScanProperties>,
}
impl AzureStorageCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure Storage credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageCredentialScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
}
impl AzureStorageCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure Storage data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of Azure Storage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureStorageProperties>,
}
impl AzureStorageDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure Storage MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure Storage MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureStorageMsiScanProperties>,
}
impl AzureStorageMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure Storage MSI scan properties."]
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
#[doc = "The properties of Azure Storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The endpoint of Azure Storage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl AzureStorageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Storage scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure Storage scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureStorageScanRulesetProperties>,
}
impl AzureStorageScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure Storage scan ruleset properties."]
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
#[doc = "Azure Storage system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure Storage scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureStorageScanRulesetProperties>,
}
impl AzureStorageSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure subscription credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure subscription credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSubscriptionCredentialScanProperties>,
}
impl AzureSubscriptionCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure subscription credential scan properties."]
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
#[doc = "Azure subscription data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "Properties of azure subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSubscriptionProperties>,
}
impl AzureSubscriptionDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure subscription MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure subscription MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSubscriptionMsiScanProperties>,
}
impl AzureSubscriptionMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure subscription MSI scan properties."]
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
#[doc = "Properties of azure subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSubscriptionProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "The subscription ID of azure subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource ID of azure subscription."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The data use governance of azure subscription."]
    #[serde(rename = "dataUseGovernance", default, skip_serializing_if = "Option::is_none")]
    pub data_use_governance: Option<azure_subscription_properties::DataUseGovernance>,
}
impl AzureSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_subscription_properties {
    use super::*;
    #[doc = "The data use governance of azure subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataUseGovernance")]
    pub enum DataUseGovernance {
        Disabled,
        DisabledByAnotherAccount,
        Enabled,
        EnabledAtAncestorScope,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataUseGovernance {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataUseGovernance {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataUseGovernance {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("DataUseGovernance", 0u32, "Disabled"),
                Self::DisabledByAnotherAccount => serializer.serialize_unit_variant("DataUseGovernance", 1u32, "DisabledByAnotherAccount"),
                Self::Enabled => serializer.serialize_unit_variant("DataUseGovernance", 2u32, "Enabled"),
                Self::EnabledAtAncestorScope => serializer.serialize_unit_variant("DataUseGovernance", 3u32, "EnabledAtAncestorScope"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure subscription scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure subscription scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSubscriptionScanRulesetProperties>,
}
impl AzureSubscriptionScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure subscription scan ruleset properties."]
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
#[doc = "Azure subscription system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSubscriptionSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure subscription scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSubscriptionScanRulesetProperties>,
}
impl AzureSubscriptionSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure synapse credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure synapse credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseCredentialScanProperties>,
}
impl AzureSynapseCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure synapse credential scan properties."]
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
#[doc = "Azure synapse data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "Azure synapse data source properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseProperties>,
}
impl AzureSynapseDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure synapse MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure synapse MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseMsiScanProperties>,
}
impl AzureSynapseMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure synapse MSI scan properties."]
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
#[doc = "Azure synapse data source properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The SQL endpoint of azure synapse."]
    #[serde(rename = "sqlEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub sql_endpoint: Option<String>,
    #[doc = "The SQL on demand endpoint of azure synapse."]
    #[serde(rename = "sqlOnDemandEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub sql_on_demand_endpoint: Option<String>,
}
impl AzureSynapseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure synapse scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure synapse scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseScanRulesetProperties>,
}
impl AzureSynapseScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure synapse scan ruleset properties."]
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
#[doc = "Azure Synapse System scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure synapse scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseScanRulesetProperties>,
}
impl AzureSynapseSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure synapse workspace credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure synapse workspace credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseWorkspaceCredentialScanProperties>,
}
impl AzureSynapseWorkspaceCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure synapse workspace credential scan properties."]
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
#[doc = "Azure synapse workspace data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "Azure synapse workspace data source properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseWorkspaceProperties>,
}
impl AzureSynapseWorkspaceDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Azure synapse workspace MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Azure synapse workspace MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseWorkspaceMsiScanProperties>,
}
impl AzureSynapseWorkspaceMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Azure synapse workspace MSI scan properties."]
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
#[doc = "Azure synapse workspace data source properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseWorkspaceProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The dedicated SQL endpoint of azure synapse workspace."]
    #[serde(rename = "dedicatedSqlEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_sql_endpoint: Option<String>,
    #[doc = "The serverless SQL endpoint of azure synapse workspace."]
    #[serde(rename = "serverlessSqlEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub serverless_sql_endpoint: Option<String>,
}
impl AzureSynapseWorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure synapse workspace scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Azure synapse workspace scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseWorkspaceScanRulesetProperties>,
}
impl AzureSynapseWorkspaceScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Azure synapse workspace scan ruleset properties."]
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
#[doc = "Azure synapse workspace system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseWorkspaceSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Azure synapse workspace scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseWorkspaceScanRulesetProperties>,
}
impl AzureSynapseWorkspaceSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Credential type that uses Basic authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicAuthAzureKeyVaultCredential {
    #[serde(flatten)]
    pub credential: Credential,
    #[doc = "Properties of user pass credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserPassCredentialProperties>,
}
impl BasicAuthAzureKeyVaultCredential {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            properties: None,
        }
    }
}
#[doc = "The classification rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassificationRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl ClassificationRule {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
        }
    }
}
#[doc = "The kind of classification rule."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ClassificationRuleUnion {
    Custom(CustomClassificationRule),
    System(SystemClassificationRule),
}
#[doc = "List of classification rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassificationRuleList {
    #[doc = "List of classification rules."]
    pub value: Vec<ClassificationRuleUnion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of classification rules."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ClassificationRuleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ClassificationRuleList {
    pub fn new(value: Vec<ClassificationRuleUnion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "The properties of classification rule pattern."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ClassificationRulePatternUnion {
    Regex(RegexClassificationRulePattern),
}
#[doc = "The reference to collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionReference {
    #[doc = "The last modified time of collection reference."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[doc = "The reference name of collection reference."]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[doc = "The type of collection reference."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CollectionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration runtime reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedVia {
    #[doc = "The reference name of the integration runtime."]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[doc = "The type of the integration runtime."]
    #[serde(rename = "integrationRuntimeType", default, skip_serializing_if = "Option::is_none")]
    pub integration_runtime_type: Option<String>,
}
impl ConnectedVia {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of connection state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionStateProperties {
    #[doc = "ActionsRequired for a private link connection."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
    #[doc = "Description of a private link connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Status of a private link connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ConnectionStateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Credential type that uses consumer provided key and secret for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerKeyAuthAzureKeyVaultCredential {
    #[serde(flatten)]
    pub credential: Credential,
    #[doc = "The properties of consumer key credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConsumerKeyCredentialProperties>,
}
impl ConsumerKeyAuthAzureKeyVaultCredential {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            properties: None,
        }
    }
}
#[doc = "The properties of consumer key credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerKeyCredentialProperties {
    #[doc = "The type properties of key vault secret consumer key credential."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<KeyVaultSecretConsumerKeyCredentialTypeProperties>,
    #[doc = "Description of credential properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ConsumerKeyCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Credential {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl Credential {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
        }
    }
}
#[doc = "The kind of credential."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CredentialUnion {
    AccountKey(AccountKeyAuthAzureKeyVaultCredential),
    BasicAuth(BasicAuthAzureKeyVaultCredential),
    ConsumerKeyAuth(ConsumerKeyAuthAzureKeyVaultCredential),
    DelegatedAuth(DelegatedAuthAzureKeyVaultCredential),
    ManagedIdentity(ManagedIdentityAzureKeyVaultCredential),
    #[serde(rename = "AmazonARN")]
    AmazonArn(RoleArnCredential),
    ServicePrincipal(ServicePrincipalAzureKeyVaultCredential),
    SqlAuth(SqlAuthAzureKeyVaultCredential),
}
#[doc = "List of credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialList {
    #[doc = "List of credentials."]
    pub value: Vec<CredentialUnion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of credentials."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for CredentialList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CredentialList {
    pub fn new(value: Vec<CredentialUnion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "The credential reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialReference {
    #[doc = "The reference name of the credential."]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[doc = "The type of the credential."]
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
    #[doc = "The type of the credential."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CredentialType")]
    pub enum CredentialType {
        AccountKey,
        ServicePrincipal,
        BasicAuth,
        SqlAuth,
        #[serde(rename = "AmazonARN")]
        AmazonArn,
        ConsumerKeyAuth,
        DelegatedAuth,
        ManagedIdentity,
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
                Self::ConsumerKeyAuth => serializer.serialize_unit_variant("CredentialType", 5u32, "ConsumerKeyAuth"),
                Self::DelegatedAuth => serializer.serialize_unit_variant("CredentialType", 6u32, "DelegatedAuth"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CredentialType", 7u32, "ManagedIdentity"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Rule of custom classification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomClassificationRule {
    #[serde(flatten)]
    pub classification_rule: ClassificationRule,
    #[doc = "The properties of custom classification rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomClassificationRuleProperties>,
}
impl CustomClassificationRule {
    pub fn new(classification_rule: ClassificationRule) -> Self {
        Self {
            classification_rule,
            properties: None,
        }
    }
}
#[doc = "The properties of custom classification rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomClassificationRuleProperties {
    #[serde(rename = "minimumPercentageMatch", default, skip_serializing_if = "Option::is_none")]
    pub minimum_percentage_match: Option<f64>,
    #[doc = "The action of classification rule."]
    #[serde(rename = "classificationAction", default, skip_serializing_if = "Option::is_none")]
    pub classification_action: Option<custom_classification_rule_properties::ClassificationAction>,
    #[doc = "The data patterns of custom classification rule."]
    #[serde(
        rename = "dataPatterns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_patterns: Vec<ClassificationRulePatternUnion>,
    #[doc = "The column patterns of custom classification rule."]
    #[serde(
        rename = "columnPatterns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub column_patterns: Vec<ClassificationRulePatternUnion>,
    #[doc = "The description of custom classification rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The version of custom classification rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "The classification name of custom classification rule."]
    #[serde(rename = "classificationName", default, skip_serializing_if = "Option::is_none")]
    pub classification_name: Option<String>,
    #[doc = "The rule status of custom classification rule."]
    #[serde(rename = "ruleStatus", default, skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<custom_classification_rule_properties::RuleStatus>,
    #[doc = "The create time of custom classification rule."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The last modified time of custom classification rule."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl CustomClassificationRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_classification_rule_properties {
    use super::*;
    #[doc = "The action of classification rule."]
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
    #[doc = "The rule status of custom classification rule."]
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
#[doc = "Custom file extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomFileExtension {
    #[doc = "Custom file type."]
    #[serde(rename = "customFileType", default, skip_serializing_if = "Option::is_none")]
    pub custom_file_type: Option<CustomFileType>,
    #[doc = "The description of the custom file extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The flag to indicate whether the custom file extension is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The file extension of the custom file extension."]
    #[serde(rename = "fileExtension", default, skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
}
impl CustomFileExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom file type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomFileType {
    #[doc = "The built-in file type of the custom file type."]
    #[serde(rename = "builtInType", default, skip_serializing_if = "Option::is_none")]
    pub built_in_type: Option<custom_file_type::BuiltInType>,
    #[doc = "The custom delimiter of the custom file type."]
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
    #[doc = "The built-in file type of the custom file type."]
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
#[doc = "The data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Scans of this data source."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scans: Vec<ScanUnion>,
    #[doc = "The creation type."]
    #[serde(rename = "creationType", default, skip_serializing_if = "Option::is_none")]
    pub creation_type: Option<data_source::CreationType>,
}
impl DataSource {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            scans: Vec::new(),
            creation_type: None,
        }
    }
}
pub mod data_source {
    use super::*;
    #[doc = "The creation type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreationType")]
    pub enum CreationType {
        Manual,
        AutoNative,
        AutoManaged,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Manual => serializer.serialize_unit_variant("CreationType", 0u32, "Manual"),
                Self::AutoNative => serializer.serialize_unit_variant("CreationType", 1u32, "AutoNative"),
                Self::AutoManaged => serializer.serialize_unit_variant("CreationType", 2u32, "AutoManaged"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The data source type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DataSourceUnion {
    AdlsGen1(AdlsGen1DataSource),
    AdlsGen2(AdlsGen2DataSource),
    AmazonAccount(AmazonAccountDataSource),
    AmazonPostgreSql(AmazonPostgreSqlDataSource),
    AmazonS3(AmazonS3DataSource),
    AmazonSql(AmazonSqlDataSource),
    AzureCosmosDb(AzureCosmosDbDataSource),
    AzureDataExplorer(AzureDataExplorerDataSource),
    AzureFileService(AzureFileServiceDataSource),
    AzureMySql(AzureMySqlDataSource),
    AzurePostgreSql(AzurePostgreSqlDataSource),
    AzureResourceGroup(AzureResourceGroupDataSource),
    AzureSqlDataWarehouse(AzureSqlDataWarehouseDataSource),
    AzureSqlDatabase(AzureSqlDatabaseDataSource),
    AzureSqlDatabaseManagedInstance(AzureSqlDatabaseManagedInstanceDataSource),
    AzureStorage(AzureStorageDataSource),
    AzureSubscription(AzureSubscriptionDataSource),
    AzureSynapse(AzureSynapseDataSource),
    AzureSynapseWorkspace(AzureSynapseWorkspaceDataSource),
    Oracle(OracleDataSource),
    #[serde(rename = "PowerBI")]
    PowerBi(PowerBiDataSource),
    SapEcc(SapEccDataSource),
    SapS4Hana(SapS4HanaDataSource),
    SqlServerDatabase(SqlServerDatabaseDataSource),
    Teradata(TeradataDataSource),
}
#[doc = "The data source identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceIdentifier {
    #[doc = "The item path."]
    #[serde(rename = "itemPath", default, skip_serializing_if = "Option::is_none")]
    pub item_path: Option<ItemPath>,
    #[doc = "The qualified name."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<QualifiedName>,
    #[doc = "The data source name."]
    #[serde(rename = "dataSourceName", default, skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[doc = "The guid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The resource identifier."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl DataSourceIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of data sources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSourceList {
    #[doc = "List of data sources."]
    pub value: Vec<DataSourceUnion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of data sources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for DataSourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataSourceList {
    pub fn new(value: Vec<DataSourceUnion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "Properties of data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceProperties {
    #[doc = "The create time of data source."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The last modified time of data source."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[doc = "The reference to collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<CollectionReference>,
    #[doc = "Data source collection moving state."]
    #[serde(rename = "dataSourceCollectionMovingState", default, skip_serializing_if = "Option::is_none")]
    pub data_source_collection_moving_state: Option<data_source_properties::DataSourceCollectionMovingState>,
}
impl DataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_source_properties {
    use super::*;
    #[doc = "Data source collection moving state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSourceCollectionMovingState")]
    pub enum DataSourceCollectionMovingState {
        Active,
        Moving,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSourceCollectionMovingState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSourceCollectionMovingState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSourceCollectionMovingState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("DataSourceCollectionMovingState", 0u32, "Active"),
                Self::Moving => serializer.serialize_unit_variant("DataSourceCollectionMovingState", 1u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("DataSourceCollectionMovingState", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The data source type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataSourceType")]
pub enum DataSourceType {
    None,
    Fabric,
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
    ArcEnabledSqlServer,
    AmazonPostgreSql,
    AzurePostgreSql,
    Databricks,
    SqlServerDatabase,
    AzureSqlDatabaseManagedInstance,
    AzureSqlDataWarehouse,
    AzureMySql,
    Hdfs,
    TableauServer,
    AzureStorage,
    Teradata,
    Oracle,
    PostgreSql,
    AmazonRedShift,
    DatabricksHms,
    SapS4Hana,
    SapEcc,
    Snowflake,
    #[serde(rename = "PowerBI")]
    PowerBi,
    Trident,
    Dataverse,
    DatabricksUnityCatalog,
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
            Self::Fabric => serializer.serialize_unit_variant("DataSourceType", 1u32, "Fabric"),
            Self::AzureSubscription => serializer.serialize_unit_variant("DataSourceType", 2u32, "AzureSubscription"),
            Self::AzureResourceGroup => serializer.serialize_unit_variant("DataSourceType", 3u32, "AzureResourceGroup"),
            Self::AzureSynapseWorkspace => serializer.serialize_unit_variant("DataSourceType", 4u32, "AzureSynapseWorkspace"),
            Self::AzureSynapse => serializer.serialize_unit_variant("DataSourceType", 5u32, "AzureSynapse"),
            Self::AdlsGen1 => serializer.serialize_unit_variant("DataSourceType", 6u32, "AdlsGen1"),
            Self::AdlsGen2 => serializer.serialize_unit_variant("DataSourceType", 7u32, "AdlsGen2"),
            Self::AmazonAccount => serializer.serialize_unit_variant("DataSourceType", 8u32, "AmazonAccount"),
            Self::AmazonS3 => serializer.serialize_unit_variant("DataSourceType", 9u32, "AmazonS3"),
            Self::AmazonSql => serializer.serialize_unit_variant("DataSourceType", 10u32, "AmazonSql"),
            Self::AzureCosmosDb => serializer.serialize_unit_variant("DataSourceType", 11u32, "AzureCosmosDb"),
            Self::AzureDataExplorer => serializer.serialize_unit_variant("DataSourceType", 12u32, "AzureDataExplorer"),
            Self::AzureFileService => serializer.serialize_unit_variant("DataSourceType", 13u32, "AzureFileService"),
            Self::AzureSqlDatabase => serializer.serialize_unit_variant("DataSourceType", 14u32, "AzureSqlDatabase"),
            Self::ArcEnabledSqlServer => serializer.serialize_unit_variant("DataSourceType", 15u32, "ArcEnabledSqlServer"),
            Self::AmazonPostgreSql => serializer.serialize_unit_variant("DataSourceType", 16u32, "AmazonPostgreSql"),
            Self::AzurePostgreSql => serializer.serialize_unit_variant("DataSourceType", 17u32, "AzurePostgreSql"),
            Self::Databricks => serializer.serialize_unit_variant("DataSourceType", 18u32, "Databricks"),
            Self::SqlServerDatabase => serializer.serialize_unit_variant("DataSourceType", 19u32, "SqlServerDatabase"),
            Self::AzureSqlDatabaseManagedInstance => {
                serializer.serialize_unit_variant("DataSourceType", 20u32, "AzureSqlDatabaseManagedInstance")
            }
            Self::AzureSqlDataWarehouse => serializer.serialize_unit_variant("DataSourceType", 21u32, "AzureSqlDataWarehouse"),
            Self::AzureMySql => serializer.serialize_unit_variant("DataSourceType", 22u32, "AzureMySql"),
            Self::Hdfs => serializer.serialize_unit_variant("DataSourceType", 23u32, "Hdfs"),
            Self::TableauServer => serializer.serialize_unit_variant("DataSourceType", 24u32, "TableauServer"),
            Self::AzureStorage => serializer.serialize_unit_variant("DataSourceType", 25u32, "AzureStorage"),
            Self::Teradata => serializer.serialize_unit_variant("DataSourceType", 26u32, "Teradata"),
            Self::Oracle => serializer.serialize_unit_variant("DataSourceType", 27u32, "Oracle"),
            Self::PostgreSql => serializer.serialize_unit_variant("DataSourceType", 28u32, "PostgreSql"),
            Self::AmazonRedShift => serializer.serialize_unit_variant("DataSourceType", 29u32, "AmazonRedShift"),
            Self::DatabricksHms => serializer.serialize_unit_variant("DataSourceType", 30u32, "DatabricksHms"),
            Self::SapS4Hana => serializer.serialize_unit_variant("DataSourceType", 31u32, "SapS4Hana"),
            Self::SapEcc => serializer.serialize_unit_variant("DataSourceType", 32u32, "SapEcc"),
            Self::Snowflake => serializer.serialize_unit_variant("DataSourceType", 33u32, "Snowflake"),
            Self::PowerBi => serializer.serialize_unit_variant("DataSourceType", 34u32, "PowerBI"),
            Self::Trident => serializer.serialize_unit_variant("DataSourceType", 35u32, "Trident"),
            Self::Dataverse => serializer.serialize_unit_variant("DataSourceType", 36u32, "Dataverse"),
            Self::DatabricksUnityCatalog => serializer.serialize_unit_variant("DataSourceType", 37u32, "DatabricksUnityCatalog"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Credential type that uses Client ID for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DelegatedAuthAzureKeyVaultCredential {
    #[serde(flatten)]
    pub credential: Credential,
    #[doc = "The properties of delegated auth credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DelegatedAuthCredentialProperties>,
}
impl DelegatedAuthAzureKeyVaultCredential {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            properties: None,
        }
    }
}
#[doc = "The properties of delegated auth credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedAuthCredentialProperties {
    #[doc = "The type properties of key vault secret delegated auth credential."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<KeyVaultSecretDelegatedAuthCredentialTypeProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl DelegatedAuthCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The discovery execution details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoveryExecutionDetails {
    #[doc = "The discovery start time."]
    #[serde(rename = "discoveryStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub discovery_start_time: Option<time::OffsetDateTime>,
    #[doc = "The discovery end time."]
    #[serde(rename = "discoveryEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub discovery_end_time: Option<time::OffsetDateTime>,
    #[doc = "The discovery status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<discovery_execution_details::Status>,
    #[doc = "The discovery statistics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<serde_json::Value>,
    #[doc = "Indicates whether the error log is available."]
    #[serde(rename = "isErrorLogAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_error_log_available: Option<bool>,
}
impl DiscoveryExecutionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod discovery_execution_details {
    use super::*;
    #[doc = "The discovery status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Queued,
        Running,
        Succeeded,
        Failed,
        Cancelled,
        Delayed,
        Throttled,
        CompletedWithExceptions,
        CompleteWithWarning,
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
                Self::Queued => serializer.serialize_unit_variant("Status", 0u32, "Queued"),
                Self::Running => serializer.serialize_unit_variant("Status", 1u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 4u32, "Cancelled"),
                Self::Delayed => serializer.serialize_unit_variant("Status", 5u32, "Delayed"),
                Self::Throttled => serializer.serialize_unit_variant("Status", 6u32, "Throttled"),
                Self::CompletedWithExceptions => serializer.serialize_unit_variant("Status", 7u32, "CompletedWithExceptions"),
                Self::CompleteWithWarning => serializer.serialize_unit_variant("Status", 8u32, "CompleteWithWarning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The request of enabling interactive query for integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnableInteractiveQueryForIntegrationRuntimeRequest {
    #[doc = "The interactive querying auto termination time in minutes."]
    #[serde(rename = "autoTerminationMinutes", default, skip_serializing_if = "Option::is_none")]
    pub auto_termination_minutes: Option<i32>,
}
impl EnableInteractiveQueryForIntegrationRuntimeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorInfo {
    #[doc = "A unique error code that identifies the specific error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable error message that provides more details about the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The specific component that the error is associated with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error info detail."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorInfo>,
}
impl ErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorModel {
    #[doc = "A unique error code that identifies the specific error."]
    pub code: String,
    #[doc = "A human-readable error message that provides more details about the error."]
    pub message: String,
    #[doc = "The specific component that the error is associated with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of nested ErrorModel objects that provides additional error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorModel>,
}
impl ErrorModel {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "The error response model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseModel {
    #[doc = "The error model."]
    pub error: ErrorModel,
}
impl azure_core::Continuable for ErrorResponseModel {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseModel {
    pub fn new(error: ErrorModel) -> Self {
        Self { error }
    }
}
#[doc = "Expanding resource scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpandingResourceScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "Resource types of scan properties in expanding resources."]
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<ExpandingResourceScanPropertiesResourceTypes>,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
}
impl ExpandingResourceScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource types of scan properties in expanding resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpandingResourceScanPropertiesResourceTypes {
    #[doc = "Resource type filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub none: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureSubscription", default, skip_serializing_if = "Option::is_none")]
    pub azure_subscription: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_group: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureSynapseWorkspace", default, skip_serializing_if = "Option::is_none")]
    pub azure_synapse_workspace: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureSynapse", default, skip_serializing_if = "Option::is_none")]
    pub azure_synapse: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "adlsGen1", default, skip_serializing_if = "Option::is_none")]
    pub adls_gen1: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "adlsGen2", default, skip_serializing_if = "Option::is_none")]
    pub adls_gen2: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "amazonAccount", default, skip_serializing_if = "Option::is_none")]
    pub amazon_account: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "amazonS3", default, skip_serializing_if = "Option::is_none")]
    pub amazon_s3: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "amazonSql", default, skip_serializing_if = "Option::is_none")]
    pub amazon_sql: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureCosmosDb", default, skip_serializing_if = "Option::is_none")]
    pub azure_cosmos_db: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureDataExplorer", default, skip_serializing_if = "Option::is_none")]
    pub azure_data_explorer: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureFileService", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_service: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureSqlDatabase", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_database: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "amazonPostgreSql", default, skip_serializing_if = "Option::is_none")]
    pub amazon_postgre_sql: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azurePostgreSql", default, skip_serializing_if = "Option::is_none")]
    pub azure_postgre_sql: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "sqlServerDatabase", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_database: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureSqlDatabaseManagedInstance", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_database_managed_instance: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureSqlDataWarehouse", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_data_warehouse: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureMySql", default, skip_serializing_if = "Option::is_none")]
    pub azure_my_sql: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "azureStorage", default, skip_serializing_if = "Option::is_none")]
    pub azure_storage: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teradata: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oracle: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "sapS4Hana", default, skip_serializing_if = "Option::is_none")]
    pub sap_s4_hana: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "sapEcc", default, skip_serializing_if = "Option::is_none")]
    pub sap_ecc: Option<ResourceTypeFilter>,
    #[doc = "Resource type filter."]
    #[serde(rename = "powerBI", default, skip_serializing_if = "Option::is_none")]
    pub power_bi: Option<ResourceTypeFilter>,
}
impl ExpandingResourceScanPropertiesResourceTypes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The extended properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedProperties {
    #[doc = "The subscription identifier."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl ExtendedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filter {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The filter properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FilterProperties>,
}
impl Filter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The filter properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterProperties {
    #[doc = "The exclude uri prefixes."]
    #[serde(rename = "excludeUriPrefixes")]
    pub exclude_uri_prefixes: Vec<String>,
    #[doc = "The include uri prefixes."]
    #[serde(rename = "includeUriPrefixes")]
    pub include_uri_prefixes: Vec<String>,
    #[doc = "The exclude regexes."]
    #[serde(
        rename = "excludeRegexes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exclude_regexes: Vec<String>,
    #[doc = "The include regexes."]
    #[serde(
        rename = "includeRegexes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub include_regexes: Vec<String>,
}
impl FilterProperties {
    pub fn new(exclude_uri_prefixes: Vec<String>, include_uri_prefixes: Vec<String>) -> Self {
        Self {
            exclude_uri_prefixes,
            include_uri_prefixes,
            exclude_regexes: Vec::new(),
            include_regexes: Vec::new(),
        }
    }
}
#[doc = "The ingestion execution details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngestionExecutionDetails {
    #[doc = "The ingestion start time."]
    #[serde(rename = "ingestionStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub ingestion_start_time: Option<time::OffsetDateTime>,
    #[doc = "The ingestion end time."]
    #[serde(rename = "ingestionEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub ingestion_end_time: Option<time::OffsetDateTime>,
    #[doc = "The ingestion status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ingestion_execution_details::Status>,
    #[doc = "The ingestion statistics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<serde_json::Value>,
    #[doc = "The estimated time remaining in seconds."]
    #[serde(rename = "estimatedTimeRemainingInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_seconds: Option<i64>,
    #[doc = "Indicates whether the error log is available."]
    #[serde(rename = "isErrorLogAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_error_log_available: Option<bool>,
    #[doc = "The last updated time."]
    #[serde(rename = "lastUpdatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_at: Option<time::OffsetDateTime>,
}
impl IngestionExecutionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ingestion_execution_details {
    use super::*;
    #[doc = "The ingestion status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        None,
        Succeeded,
        Queued,
        InProgress,
        SourceUnknown,
        PartialSucceeded,
        Failed,
        Canceled,
        Canceling,
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
                Self::None => serializer.serialize_unit_variant("Status", 0u32, "None"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Queued => serializer.serialize_unit_variant("Status", 2u32, "Queued"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 3u32, "InProgress"),
                Self::SourceUnknown => serializer.serialize_unit_variant("Status", 4u32, "SourceUnknown"),
                Self::PartialSucceeded => serializer.serialize_unit_variant("Status", 5u32, "PartialSucceeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 6u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 7u32, "Canceled"),
                Self::Canceling => serializer.serialize_unit_variant("Status", 8u32, "Canceling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Purview nested object which serves as a compute resource for activities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntime {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl IntegrationRuntime {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
        }
    }
}
#[doc = "The type of integration runtime."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum IntegrationRuntimeUnion {
    Managed(ManagedIntegrationRuntime),
    SelfHosted(SelfHostedIntegrationRuntime),
}
#[doc = "The integration runtime authentication keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeAuthKeys {
    #[doc = "The primary integration runtime authentication key."]
    #[serde(rename = "authKey1", default, skip_serializing_if = "Option::is_none")]
    pub auth_key1: Option<String>,
    #[doc = "The secondary integration runtime authentication key."]
    #[serde(rename = "authKey2", default, skip_serializing_if = "Option::is_none")]
    pub auth_key2: Option<String>,
}
impl IntegrationRuntimeAuthKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of integration runtime resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeList {
    #[doc = "List of integration runtimes."]
    pub value: Vec<IntegrationRuntimeUnion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of integration runtimes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for IntegrationRuntimeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IntegrationRuntimeList {
    pub fn new(value: Vec<IntegrationRuntimeUnion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "Parameters to regenerate the authentication key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeRegenerateKeyParameters {
    #[doc = "The name of the authentication key to regenerate."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<integration_runtime_regenerate_key_parameters::KeyName>,
}
impl IntegrationRuntimeRegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod integration_runtime_regenerate_key_parameters {
    use super::*;
    #[doc = "The name of the authentication key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyName")]
    pub enum KeyName {
        #[serde(rename = "authKey1")]
        AuthKey1,
        #[serde(rename = "authKey2")]
        AuthKey2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AuthKey1 => serializer.serialize_unit_variant("KeyName", 0u32, "authKey1"),
                Self::AuthKey2 => serializer.serialize_unit_variant("KeyName", 1u32, "authKey2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Integration runtime status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeStatus {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl IntegrationRuntimeStatus {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
        }
    }
}
#[doc = "The type of integration runtime."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum IntegrationRuntimeStatusUnion {
    Managed(ManagedIntegrationRuntimeStatus),
    SelfHosted(SelfHostedIntegrationRuntimeStatus),
}
#[doc = "The type of integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationRuntimeType")]
pub enum IntegrationRuntimeType {
    Managed,
    SelfHosted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationRuntimeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationRuntimeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationRuntimeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Managed => serializer.serialize_unit_variant("IntegrationRuntimeType", 0u32, "Managed"),
            Self::SelfHosted => serializer.serialize_unit_variant("IntegrationRuntimeType", 1u32, "SelfHosted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Interactive query properties of managed integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InteractiveQuery {
    #[doc = "Auto termination minutes of interactive query."]
    #[serde(rename = "autoTerminationMinutes", default, skip_serializing_if = "Option::is_none")]
    pub auto_termination_minutes: Option<i32>,
    #[doc = "Status of interactive query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl InteractiveQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The item path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ItemPath {
    #[doc = "The path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The extended properties."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<ExtendedProperties>,
}
impl ItemPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key vault secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecret {
    #[doc = "The type of key vault secret."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The secret name of key vault secret."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    #[doc = "The secret version of key vault secret."]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
    #[doc = "The store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<Store>,
}
impl KeyVaultSecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of key vault secret account key credential type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretAccountKeyCredentialTypeProperties {
    #[doc = "The key vault secret."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<KeyVaultSecret>,
}
impl KeyVaultSecretAccountKeyCredentialTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type properties of key vault secret consumer key credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretConsumerKeyCredentialTypeProperties {
    #[doc = "User name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "The key vault secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<KeyVaultSecret>,
    #[doc = "The consumer key of key vault secret consumer key credential type properties."]
    #[serde(rename = "consumerKey", default, skip_serializing_if = "Option::is_none")]
    pub consumer_key: Option<String>,
    #[doc = "The key vault secret."]
    #[serde(rename = "consumerSecret", default, skip_serializing_if = "Option::is_none")]
    pub consumer_secret: Option<KeyVaultSecret>,
}
impl KeyVaultSecretConsumerKeyCredentialTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type properties of key vault secret delegated auth credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretDelegatedAuthCredentialTypeProperties {
    #[doc = "Credential type that uses Account ID, External ID and Role ARN for authentication."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "User name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "The key vault secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<KeyVaultSecret>,
}
impl KeyVaultSecretDelegatedAuthCredentialTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type properties of key vault secret managed identity Azure Key Vault credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretManagedIdentityAzureKeyVaultCredentialTypeProperties {
    #[doc = "The principal ID of key vault secret managed identity Azure Key Vault credential type properties."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of key vault secret managed identity Azure Key Vault credential type properties."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The resource ID of key vault secret managed identity Azure Key Vault credential type properties."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl KeyVaultSecretManagedIdentityAzureKeyVaultCredentialTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type properties of key vault secret service principal credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretServicePrinipalCredentialTypeProperties {
    #[doc = "The service principal ID of key vault secret service principal credential type properties."]
    #[serde(rename = "servicePrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_id: Option<String>,
    #[doc = "The key vault secret."]
    #[serde(rename = "servicePrincipalKey", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_key: Option<KeyVaultSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}
impl KeyVaultSecretServicePrinipalCredentialTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of key vault secret user pass credential type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretUserPassCredentialTypeProperties {
    #[doc = "User name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "The key vault secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<KeyVaultSecret>,
}
impl KeyVaultSecretUserPassCredentialTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Credential type that uses User assigned managed identities for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIdentityAzureKeyVaultCredential {
    #[serde(flatten)]
    pub credential: Credential,
    #[doc = "The properties of managed identity Azure Key Vault credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedIdentityAzureKeyVaultCredentialProperties>,
}
impl ManagedIdentityAzureKeyVaultCredential {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            properties: None,
        }
    }
}
#[doc = "The properties of managed identity Azure Key Vault credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentityAzureKeyVaultCredentialProperties {
    #[doc = "The type properties of key vault secret managed identity Azure Key Vault credential."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<KeyVaultSecretManagedIdentityAzureKeyVaultCredentialTypeProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ManagedIdentityAzureKeyVaultCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIntegrationRuntime {
    #[serde(flatten)]
    pub integration_runtime: IntegrationRuntime,
    #[doc = "The managed integration runtime properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedIntegrationRuntimeProperties>,
}
impl ManagedIntegrationRuntime {
    pub fn new(integration_runtime: IntegrationRuntime) -> Self {
        Self {
            integration_runtime,
            properties: None,
        }
    }
}
#[doc = "The compute resource properties for managed integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeComputeProperties {
    #[doc = "The location for managed integration runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ManagedIntegrationRuntimeComputeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed integration runtime properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeProperties {
    #[doc = "Managed integration runtime type properties."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<ManagedIntegrationRuntimeTypeProperties>,
    #[doc = "The managed virtual network reference."]
    #[serde(rename = "managedVirtualNetworkReference", default, skip_serializing_if = "Option::is_none")]
    pub managed_virtual_network_reference: Option<ManagedVirtualNetworkReference>,
    #[doc = "The managed integration runtime description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ManagedIntegrationRuntimeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed integration runtime status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIntegrationRuntimeStatus {
    #[serde(flatten)]
    pub integration_runtime_status: IntegrationRuntimeStatus,
    #[doc = "Managed integration runtime status properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedIntegrationRuntimeStatusProperties>,
}
impl ManagedIntegrationRuntimeStatus {
    pub fn new(integration_runtime_status: IntegrationRuntimeStatus) -> Self {
        Self {
            integration_runtime_status,
            properties: None,
        }
    }
}
#[doc = "Managed integration runtime status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeStatusProperties {
    #[doc = "Managed integration runtime status type properties."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<ManagedIntegrationRuntimeStatusTypeProperties>,
    #[doc = "Managed integration runtime state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl ManagedIntegrationRuntimeStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed integration runtime status type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeStatusTypeProperties {
    #[doc = "The time at which the integration runtime was created, in ISO8601 format."]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "Interactive query properties of managed integration runtime."]
    #[serde(rename = "interactiveQuery", default, skip_serializing_if = "Option::is_none")]
    pub interactive_query: Option<InteractiveQuery>,
}
impl ManagedIntegrationRuntimeStatusTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed integration runtime type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeTypeProperties {
    #[doc = "The compute resource properties for managed integration runtime."]
    #[serde(rename = "computeProperties", default, skip_serializing_if = "Option::is_none")]
    pub compute_properties: Option<ManagedIntegrationRuntimeComputeProperties>,
}
impl ManagedIntegrationRuntimeTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed private endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedPrivateEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of managed private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedPrivateEndpointProperties>,
}
impl ManagedPrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of managed private endpoint resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedPrivateEndpointList {
    #[doc = "List of managed private endpoints."]
    pub value: Vec<ManagedPrivateEndpoint>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of managed private endpoints."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ManagedPrivateEndpointList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedPrivateEndpointList {
    pub fn new(value: Vec<ManagedPrivateEndpoint>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "The properties of managed private endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedPrivateEndpointProperties {
    #[doc = "The properties of connection state."]
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<ConnectionStateProperties>,
    #[doc = "The fqdns of managed private endpoint."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fqdns: Vec<String>,
    #[doc = "The group identifier of managed private endpoint."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource identifier of managed private endpoint."]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "The provisioning state of managed private endpoint."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ManagedPrivateEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedVirtualNetwork {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of managed virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedVirtualNetworkProperties>,
}
impl ManagedVirtualNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of managed virtual network resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedVirtualNetworkList {
    #[doc = "List of managed virtual networks."]
    pub value: Vec<ManagedVirtualNetwork>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of managed virtual networks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ManagedVirtualNetworkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedVirtualNetworkList {
    pub fn new(value: Vec<ManagedVirtualNetwork>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "The properties of managed virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedVirtualNetworkProperties {
    #[doc = "The ID of the VNet that this integration runtime will join."]
    #[serde(rename = "vNetId", default, skip_serializing_if = "Option::is_none")]
    pub v_net_id: Option<String>,
    #[doc = "The alias of managed virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}
impl ManagedVirtualNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed virtual network reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedVirtualNetworkReference {
    #[doc = "Reference ManagedVirtualNetwork name."]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[doc = "Managed virtual network reference type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ManagedVirtualNetworkReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Miti scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MitiScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "The maximum memory allowed in GB."]
    #[serde(rename = "maximumMemoryAllowedInGb", default, skip_serializing_if = "Option::is_none")]
    pub maximum_memory_allowed_in_gb: Option<String>,
    #[doc = "The miti cache."]
    #[serde(rename = "mitiCache", default, skip_serializing_if = "Option::is_none")]
    pub miti_cache: Option<String>,
}
impl MitiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The notification model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Notification {
    #[doc = "Notification message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Notification code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}
impl Notification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[doc = "The scan result identifier."]
    #[serde(rename = "scanResultId", default, skip_serializing_if = "Option::is_none")]
    pub scan_result_id: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Scan operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_response::Status>,
    #[doc = "The error info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_response {
    use super::*;
    #[doc = "Scan operation status."]
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
#[doc = "The oracle data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of oracle."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OracleProperties>,
}
impl OracleDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Oracle credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleOracleCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Oracle credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OracleOracleCredentialScanProperties>,
}
impl OracleOracleCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Oracle credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleOracleCredentialScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "driverLocation", default, skip_serializing_if = "Option::is_none")]
    pub driver_location: Option<String>,
    #[serde(rename = "storedProcedureDetails", default, skip_serializing_if = "Option::is_none")]
    pub stored_procedure_details: Option<String>,
}
impl OracleOracleCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Oracle user pass scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleOracleUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Oracle user pass scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OracleOracleUserPassScanProperties>,
}
impl OracleOracleUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Oracle user pass scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleOracleUserPassScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[doc = "The username of Oracle server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password of Oracle server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "The driver location."]
    #[serde(rename = "driverLocation", default, skip_serializing_if = "Option::is_none")]
    pub driver_location: Option<String>,
}
impl OracleOracleUserPassScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of oracle."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "The host of Oracle server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The port of Oracle server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "The service of Oracle server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}
impl OracleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Oracle scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Oracle scan rule set properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OracleScanRulesetProperties>,
}
impl OracleScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Oracle scan rule set properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleScanRulesetProperties {
    #[serde(flatten)]
    pub scanning_rule_scan_ruleset_properties: ScanningRuleScanRulesetProperties,
}
impl OracleScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Oracle system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Oracle scan rule set properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OracleScanRulesetProperties>,
}
impl OracleSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Power BI data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "Power BI properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PowerBiProperties>,
}
impl PowerBiDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "Power BI delegated scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiDelegatedScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Power BI delegated scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PowerBiDelegatedScanProperties>,
}
impl PowerBiDelegatedScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Power BI delegated scan properties."]
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
#[doc = "Power BI MSI scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiMsiScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Power BI MSI scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PowerBiMsiScanProperties>,
}
impl PowerBiMsiScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Power BI MSI scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PowerBiMsiScanProperties {
    #[serde(flatten)]
    pub scan_properties: ScanProperties,
    #[doc = "Whether to include personal workspaces or not."]
    #[serde(rename = "includePersonalWorkspaces", default, skip_serializing_if = "Option::is_none")]
    pub include_personal_workspaces: Option<bool>,
}
impl PowerBiMsiScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Power BI properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PowerBiProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "The tenant of Power BI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}
impl PowerBiProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Power BI scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Power BI scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PowerBiScanRulesetProperties>,
}
impl PowerBiScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Power BI scan ruleset properties."]
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
#[doc = "Power BI system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Power BI scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PowerBiScanRulesetProperties>,
}
impl PowerBiSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "The proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The qualified name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QualifiedName {
    #[doc = "The type name."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The extended properties."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<ExtendedProperties>,
}
impl QualifiedName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schedule of recurrence."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecurrenceSchedule {
    #[doc = "The minutes of recurrence schedule."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub minutes: Vec<i32>,
    #[doc = "The hours of recurrence schedule."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hours: Vec<i32>,
    #[doc = "The week days of recurrence schedule."]
    #[serde(
        rename = "weekDays",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub week_days: Vec<String>,
    #[doc = "Month days of recurrence schedule."]
    #[serde(
        rename = "monthDays",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub month_days: Vec<i32>,
    #[doc = "The monthly occurrences of recurrence schedule."]
    #[serde(
        rename = "monthlyOccurrences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub monthly_occurrences: Vec<RecurrenceScheduleOccurrence>,
}
impl RecurrenceSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The occurrence of recurrence schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecurrenceScheduleOccurrence {
    #[doc = "The day of recurrence schedule occurrence."]
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
    #[doc = "The day of recurrence schedule occurrence."]
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
#[doc = "Pattern of regex classification rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegexClassificationRulePattern {
    #[doc = "The pattern of regex classification rule pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}
impl RegexClassificationRulePattern {
    pub fn new() -> Self {
        Self { pattern: None }
    }
}
#[doc = "Resource name filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNameFilter {
    #[serde(
        rename = "excludePrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exclude_prefixes: Vec<String>,
    #[serde(
        rename = "includePrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub include_prefixes: Vec<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<String>,
}
impl ResourceNameFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource type filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeFilter {
    #[doc = "The name of the scan ruleset."]
    #[serde(rename = "scanRulesetName", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_name: Option<String>,
    #[doc = "The type of the scan ruleset."]
    #[serde(rename = "scanRulesetType", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_type: Option<resource_type_filter::ScanRulesetType>,
    #[doc = "Resource name filter."]
    #[serde(rename = "resourceNameFilter", default, skip_serializing_if = "Option::is_none")]
    pub resource_name_filter: Option<ResourceNameFilter>,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
}
impl ResourceTypeFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_type_filter {
    use super::*;
    #[doc = "The type of the scan ruleset."]
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
#[doc = "Credential type that uses Account ID, External ID and Role ARN for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleArnCredential {
    #[serde(flatten)]
    pub credential: Credential,
    #[doc = "The properties of role ARN credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleArnCredentialProperties>,
}
impl RoleArnCredential {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            properties: None,
        }
    }
}
#[doc = "The properties of role ARN credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleArnCredentialProperties {
    #[doc = "The type properties of role ARN credential."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<RoleArnCredentialTypeProperties>,
    #[doc = "The description of role ARN credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl RoleArnCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type properties of role ARN credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleArnCredentialTypeProperties {
    #[doc = "The role ARN of role ARN credential type properties."]
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}
impl RoleArnCredentialTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SAP ECC data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "SAP ECC properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapEccProperties>,
}
impl SapEccDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "SAP ECC properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapEccProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "The application server of SAP ECC."]
    #[serde(rename = "applicationServer", default, skip_serializing_if = "Option::is_none")]
    pub application_server: Option<String>,
    #[doc = "The system number of SAP ECC."]
    #[serde(rename = "systemNumber", default, skip_serializing_if = "Option::is_none")]
    pub system_number: Option<String>,
}
impl SapEccProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SAP ECC credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccSapEccCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "SAP ECC credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapEccSapEccCredentialScanProperties>,
}
impl SapEccSapEccCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "SAP ECC credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapEccSapEccCredentialScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[serde(rename = "jCoLibraryPath", default, skip_serializing_if = "Option::is_none")]
    pub j_co_library_path: Option<String>,
}
impl SapEccSapEccCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SAP ECC user pass scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccSapEccUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "SAP ECC user pass scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapEccSapEccUserPassScanProperties>,
}
impl SapEccSapEccUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "SAP ECC user pass scan properties."]
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
#[doc = "SAP ECC scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "SAP ECC scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapEccScanRulesetProperties>,
}
impl SapEccScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "SAP ECC scan ruleset properties."]
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
#[doc = "SAP ECC system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapEccSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "SAP ECC scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapEccScanRulesetProperties>,
}
impl SapEccSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "SAP S/4HANA data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "SAP S/4HANA data source properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapS4HanaProperties>,
}
impl SapS4HanaDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "SAP S/4HANA data source properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapS4HanaProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "The application server of SAP S/4HANA."]
    #[serde(rename = "applicationServer", default, skip_serializing_if = "Option::is_none")]
    pub application_server: Option<String>,
    #[doc = "The system number of SAP S/4HANA."]
    #[serde(rename = "systemNumber", default, skip_serializing_if = "Option::is_none")]
    pub system_number: Option<String>,
}
impl SapS4HanaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SAP S/4HANA credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaSapS4HanaCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "SAP S/4HANA credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapS4HanaSapS4HanaCredentialScanProperties>,
}
impl SapS4HanaSapS4HanaCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "SAP S/4HANA credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapS4HanaSapS4HanaCredentialScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[doc = "The client ID of SAP S/4HANA server."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[doc = "The JCo library path."]
    #[serde(rename = "jCoLibraryPath", default, skip_serializing_if = "Option::is_none")]
    pub j_co_library_path: Option<String>,
}
impl SapS4HanaSapS4HanaCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SAP S/4HANA user pass scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaSapS4HanaUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "SAP S/4HANA user pass scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapS4HanaSapS4HanaUserPassScanProperties>,
}
impl SapS4HanaSapS4HanaUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "SAP S/4HANA user pass scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapS4HanaSapS4HanaUserPassScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[doc = "The client ID of SAP S/4HANA server."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The username of SAP S/4HANA server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password of SAP S/4HANA server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The JCo library path."]
    #[serde(rename = "jCoLibraryPath", default, skip_serializing_if = "Option::is_none")]
    pub j_co_library_path: Option<String>,
}
impl SapS4HanaSapS4HanaUserPassScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SAP S/4HANA scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "SAP S/4HANA scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapS4HanaScanRulesetProperties>,
}
impl SapS4HanaScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "SAP S/4HANA scan ruleset properties."]
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
#[doc = "SAP S/4HANA system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapS4HanaSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "SAP S/4HANA scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapS4HanaScanRulesetProperties>,
}
impl SapS4HanaSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "The scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scan {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The scan result with ingestion."]
    #[serde(rename = "lastRunResult", default, skip_serializing_if = "Option::is_none")]
    pub last_run_result: Option<ScanResultWithIngestion>,
    #[doc = "The scan identifier."]
    #[serde(rename = "scanId", default, skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[doc = "The data source identifier."]
    #[serde(rename = "dataSourceIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub data_source_identifier: Option<DataSourceIdentifier>,
    #[doc = "The data source name."]
    #[serde(rename = "dataSourceName", default, skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[doc = "The creation type."]
    #[serde(rename = "creationType", default, skip_serializing_if = "Option::is_none")]
    pub creation_type: Option<scan::CreationType>,
}
impl Scan {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            last_run_result: None,
            scan_id: None,
            data_source_identifier: None,
            data_source_name: None,
            creation_type: None,
        }
    }
}
pub mod scan {
    use super::*;
    #[doc = "The creation type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreationType")]
    pub enum CreationType {
        Manual,
        AutoNative,
        AutoManaged,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Manual => serializer.serialize_unit_variant("CreationType", 0u32, "Manual"),
                Self::AutoNative => serializer.serialize_unit_variant("CreationType", 1u32, "AutoNative"),
                Self::AutoManaged => serializer.serialize_unit_variant("CreationType", 2u32, "AutoManaged"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of scan, vary by datasource type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ScanUnion {
    AdlsGen1Credential(AdlsGen1CredentialScan),
    AdlsGen1Msi(AdlsGen1MsiScan),
    AdlsGen2Credential(AdlsGen2CredentialScan),
    AdlsGen2Msi(AdlsGen2MsiScan),
    AmazonAccountCredential(AmazonAccountCredentialScan),
    AmazonPostgreSqlCredential(AmazonPostgreSqlCredentialScan),
    AmazonS3Credential(AmazonS3CredentialScan),
    #[serde(rename = "AmazonS3RoleARN")]
    AmazonS3RoleArn(AmazonS3RoleArnScan),
    AmazonSqlCredential(AmazonSqlCredentialScan),
    AzureCosmosDbCredential(AzureCosmosDbCredentialScan),
    AzureDataExplorerCredential(AzureDataExplorerCredentialScan),
    AzureDataExplorerMsi(AzureDataExplorerMsiScan),
    AzureFileServiceCredential(AzureFileServiceCredentialScan),
    AzureMySqlCredential(AzureMySqlCredentialScan),
    AzurePostgreSqlCredential(AzurePostgreSqlCredentialScan),
    AzureResourceGroupCredential(AzureResourceGroupCredentialScan),
    AzureResourceGroupMsi(AzureResourceGroupMsiScan),
    AzureSqlDataWarehouseCredential(AzureSqlDataWarehouseCredentialScan),
    AzureSqlDataWarehouseMsi(AzureSqlDataWarehouseMsiScan),
    AzureSqlDatabaseCredential(AzureSqlDatabaseCredentialScan),
    AzureSqlDatabaseManagedInstanceCredential(AzureSqlDatabaseManagedInstanceCredentialScan),
    AzureSqlDatabaseManagedInstanceMsi(AzureSqlDatabaseManagedInstanceMsiScan),
    AzureSqlDatabaseMsi(AzureSqlDatabaseMsiScan),
    AzureStorageCredential(AzureStorageCredentialScan),
    AzureStorageMsi(AzureStorageMsiScan),
    AzureSubscriptionCredential(AzureSubscriptionCredentialScan),
    AzureSubscriptionMsi(AzureSubscriptionMsiScan),
    AzureSynapseCredential(AzureSynapseCredentialScan),
    AzureSynapseMsi(AzureSynapseMsiScan),
    AzureSynapseWorkspaceCredential(AzureSynapseWorkspaceCredentialScan),
    AzureSynapseWorkspaceMsi(AzureSynapseWorkspaceMsiScan),
    OracleOracleCredential(OracleOracleCredentialScan),
    OracleOracleUserPass(OracleOracleUserPassScan),
    #[serde(rename = "PowerBIDelegated")]
    PowerBiDelegated(PowerBiDelegatedScan),
    #[serde(rename = "PowerBIMsi")]
    PowerBiMsi(PowerBiMsiScan),
    SapEccSapEccCredential(SapEccSapEccCredentialScan),
    SapEccSapEccUserPass(SapEccSapEccUserPassScan),
    SapS4HanaSapS4HanaCredential(SapS4HanaSapS4HanaCredentialScan),
    SapS4HanaSapS4HanaUserPass(SapS4HanaSapS4HanaUserPassScan),
    SqlServerDatabaseCredential(SqlServerDatabaseCredentialScan),
    TeradataTeradataCredential(TeradataTeradataCredentialScan),
    TeradataTeradataUserPass(TeradataTeradataUserPassScan),
    TeradataUserPass(TeradataUserPassScan),
}
#[doc = "The scan diagnostics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanDiagnostics {
    #[doc = "Notifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notifications: Vec<Notification>,
    #[doc = "Exception count map."]
    #[serde(rename = "exceptionCountMap", default, skip_serializing_if = "Option::is_none")]
    pub exception_count_map: Option<serde_json::Value>,
}
impl ScanDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of scan history."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScanHistoryList {
    #[doc = "List of scan history."]
    pub value: Vec<ScanResultWithIngestion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of scan history."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl ScanHistoryList {
    pub fn new(value: Vec<ScanResultWithIngestion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "List of scan history with ingestion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScanHistoryListWithIngestion {
    #[doc = "List of scan history with ingestion."]
    pub value: Vec<ScanResultWithIngestion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of scan history with ingestion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ScanHistoryListWithIngestion {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScanHistoryListWithIngestion {
    pub fn new(value: Vec<ScanResultWithIngestion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "List of scans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScanList {
    #[doc = "List of scans."]
    pub value: Vec<ScanUnion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of scans."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ScanList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScanList {
    pub fn new(value: Vec<ScanUnion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "Scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanProperties {
    #[doc = "The name of the scan ruleset."]
    #[serde(rename = "scanRulesetName", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_name: Option<String>,
    #[doc = "The name of the business rule set."]
    #[serde(rename = "businessRuleSetName", default, skip_serializing_if = "Option::is_none")]
    pub business_rule_set_name: Option<String>,
    #[doc = "The type of the scan ruleset."]
    #[serde(rename = "scanRulesetType", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_type: Option<scan_properties::ScanRulesetType>,
    #[doc = "The reference to collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<CollectionReference>,
    #[doc = "The domain of the scan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The number of workers of the scan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<i32>,
    #[doc = "The creation time of the scan."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The last modified time of the scan."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[doc = "The integration runtime reference."]
    #[serde(rename = "connectedVia", default, skip_serializing_if = "Option::is_none")]
    pub connected_via: Option<ConnectedVia>,
    #[doc = "The flag to indicate whether the scan is a preset scan or not."]
    #[serde(rename = "isPresetScan", default, skip_serializing_if = "Option::is_none")]
    pub is_preset_scan: Option<bool>,
    #[doc = "The flag to indicate whether the live view is enabled or not."]
    #[serde(rename = "isLiveViewEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_live_view_enabled: Option<bool>,
    #[doc = "The number of parallel scans."]
    #[serde(rename = "parallelScanCount", default, skip_serializing_if = "Option::is_none")]
    pub parallel_scan_count: Option<i32>,
    #[doc = "The log level of the scan."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
}
impl ScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scan_properties {
    use super::*;
    #[doc = "The type of the scan ruleset."]
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
#[doc = "The scan result with ingestion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanResultWithIngestion {
    #[doc = "The discovery execution details."]
    #[serde(rename = "discoveryExecutionDetails", default, skip_serializing_if = "Option::is_none")]
    pub discovery_execution_details: Option<DiscoveryExecutionDetails>,
    #[doc = "The ingestion execution details."]
    #[serde(rename = "ingestionExecutionDetails", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_execution_details: Option<IngestionExecutionDetails>,
    #[doc = "The parent scan result identifier."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The scan result identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ingestion job identifier."]
    #[serde(rename = "ingestionJobId", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_job_id: Option<String>,
    #[doc = "The resource identifier."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The scan result status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<scan_result_with_ingestion::Status>,
    #[doc = "The scan diagnostics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<ScanDiagnostics>,
    #[doc = "The scan start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The scan end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The scan ruleset version."]
    #[serde(rename = "scanRulesetVersion", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_version: Option<i32>,
    #[doc = "The scan ruleset type."]
    #[serde(rename = "scanRulesetType", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_type: Option<scan_result_with_ingestion::ScanRulesetType>,
    #[doc = "Scan level type."]
    #[serde(rename = "scanLevelType", default, skip_serializing_if = "Option::is_none")]
    pub scan_level_type: Option<scan_result_with_ingestion::ScanLevelType>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The error model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorModel>,
    #[doc = "The run type."]
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    #[doc = "The data source type."]
    #[serde(rename = "dataSourceType", default, skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<DataSourceType>,
}
impl ScanResultWithIngestion {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scan_result_with_ingestion {
    use super::*;
    #[doc = "The scan result status."]
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
    #[doc = "The scan ruleset type."]
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
    #[doc = "Scan level type."]
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
}
#[doc = "The scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScanRuleset {
    #[serde(flatten)]
    pub versioned_scan_ruleset: VersionedScanRuleset,
}
impl ScanRuleset {
    pub fn new() -> Self {
        Self {
            versioned_scan_ruleset: VersionedScanRuleset::default(),
        }
    }
}
#[doc = "The data source type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ScanRulesetUnion {
    AdlsGen1(AdlsGen1ScanRuleset),
    AdlsGen2(AdlsGen2ScanRuleset),
    AmazonAccount(AmazonAccountScanRuleset),
    AmazonPostgreSql(AmazonPostgreSqlScanRuleset),
    AmazonS3(AmazonS3ScanRuleset),
    AmazonSql(AmazonSqlScanRuleset),
    AzureCosmosDb(AzureCosmosDbScanRuleset),
    AzureDataExplorer(AzureDataExplorerScanRuleset),
    AzureFileService(AzureFileServiceScanRuleset),
    AzureMySql(AzureMySqlScanRuleset),
    AzurePostgreSql(AzurePostgreSqlScanRuleset),
    AzureResourceGroup(AzureResourceGroupScanRuleset),
    AzureSqlDataWarehouse(AzureSqlDataWarehouseScanRuleset),
    AzureSqlDatabaseManagedInstance(AzureSqlDatabaseManagedInstanceScanRuleset),
    AzureSqlDatabase(AzureSqlDatabaseScanRuleset),
    AzureStorage(AzureStorageScanRuleset),
    AzureSubscription(AzureSubscriptionScanRuleset),
    AzureSynapse(AzureSynapseScanRuleset),
    AzureSynapseWorkspace(AzureSynapseWorkspaceScanRuleset),
    Oracle(OracleScanRuleset),
    #[serde(rename = "PowerBI")]
    PowerBi(PowerBiScanRuleset),
    SapEcc(SapEccScanRuleset),
    SapS4Hana(SapS4HanaScanRuleset),
    SqlServerDatabase(SqlServerDatabaseScanRuleset),
    Teradata(TeradataScanRuleset),
}
#[doc = "List of scan rulesets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScanRulesetList {
    #[doc = "List of scan rulesets."]
    pub value: Vec<ScanRulesetUnion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of scan rulesets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for ScanRulesetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScanRulesetList {
    pub fn new(value: Vec<ScanRulesetUnion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "Scan ruleset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanRulesetProperties {
    #[doc = "The time at which the scan ruleset was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The system classifications that are excluded from the scan ruleset."]
    #[serde(
        rename = "excludedSystemClassifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub excluded_system_classifications: Vec<String>,
    #[doc = "The custom classification rule names that are included in the scan ruleset."]
    #[serde(
        rename = "includedCustomClassificationRuleNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub included_custom_classification_rule_names: Vec<String>,
    #[doc = "The time at which the scan ruleset was last modified."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl ScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scanning rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanningRule {
    #[serde(
        rename = "fileExtensions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_extensions: Vec<String>,
    #[doc = "The custom file extensions of the scanning rule."]
    #[serde(
        rename = "customFileExtensions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_file_extensions: Vec<CustomFileExtension>,
}
impl ScanningRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scanning rule scan ruleset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanningRuleScanRulesetProperties {
    #[serde(flatten)]
    pub scan_ruleset_properties: ScanRulesetProperties,
    #[doc = "Scanning rule."]
    #[serde(rename = "scanningRule", default, skip_serializing_if = "Option::is_none")]
    pub scanning_rule: Option<ScanningRule>,
}
impl ScanningRuleScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Self-hosted integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfHostedIntegrationRuntime {
    #[serde(flatten)]
    pub integration_runtime: IntegrationRuntime,
    #[doc = "The self-hosted integration runtime properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SelfHostedIntegrationRuntimeProperties>,
}
impl SelfHostedIntegrationRuntime {
    pub fn new(integration_runtime: IntegrationRuntime) -> Self {
        Self {
            integration_runtime,
            properties: None,
        }
    }
}
#[doc = "Properties of Self-hosted integration runtime node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfHostedIntegrationRuntimeNode {
    #[doc = "Name of the integration runtime node."]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "Machine name of the integration runtime node."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "URI for the host machine of the integration runtime."]
    #[serde(rename = "hostServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub host_service_uri: Option<String>,
    #[doc = "Status of the integration runtime node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The integration runtime capabilities dictionary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[doc = "Status of the integration runtime node version."]
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[doc = "Version of the integration runtime node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The time at which the integration runtime node was registered in ISO8601 format."]
    #[serde(rename = "registerTime", default, with = "azure_core::date::rfc3339::option")]
    pub register_time: Option<time::OffsetDateTime>,
    #[doc = "The most recent time at which the integration runtime was connected in ISO8601 format."]
    #[serde(rename = "lastConnectTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_connect_time: Option<time::OffsetDateTime>,
    #[doc = "The time at which the integration runtime will expire in ISO8601 format."]
    #[serde(rename = "expiryTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "The time the node last started up."]
    #[serde(rename = "lastStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_start_time: Option<time::OffsetDateTime>,
    #[doc = "The integration runtime node last stop time."]
    #[serde(rename = "lastStopTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_stop_time: Option<time::OffsetDateTime>,
    #[doc = "The result of the last integration runtime node update."]
    #[serde(rename = "lastUpdateResult", default, skip_serializing_if = "Option::is_none")]
    pub last_update_result: Option<String>,
    #[doc = "The last time for the integration runtime node update start."]
    #[serde(rename = "lastStartUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_start_update_time: Option<time::OffsetDateTime>,
    #[doc = "The last time for the integration runtime node update end."]
    #[serde(rename = "lastEndUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_end_update_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether this node is the active dispatcher for integration runtime requests."]
    #[serde(rename = "isActiveDispatcher", default, skip_serializing_if = "Option::is_none")]
    pub is_active_dispatcher: Option<bool>,
    #[doc = "The concurrent jobs limit of self-hosted integration runtime node."]
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i32>,
    #[doc = "The max concurrent jobs of self-hosted integration runtime node."]
    #[serde(rename = "maxConcurrentJobs", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_jobs: Option<i32>,
}
impl SelfHostedIntegrationRuntimeNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The self-hosted integration runtime properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfHostedIntegrationRuntimeProperties {
    #[doc = "The self-hosted integration runtime description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SelfHostedIntegrationRuntimeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Self-hosted integration runtime status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfHostedIntegrationRuntimeStatus {
    #[serde(flatten)]
    pub integration_runtime_status: IntegrationRuntimeStatus,
    #[doc = "Self-hosted integration runtime status properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SelfHostedIntegrationRuntimeStatusProperties>,
}
impl SelfHostedIntegrationRuntimeStatus {
    pub fn new(integration_runtime_status: IntegrationRuntimeStatus) -> Self {
        Self {
            integration_runtime_status,
            properties: None,
        }
    }
}
#[doc = "Self-hosted integration runtime status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfHostedIntegrationRuntimeStatusProperties {
    #[doc = "Self-hosted integration runtime status type properties."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<SelfHostedIntegrationRuntimeStatusTypeProperties>,
    #[doc = "Self-hosted integration runtime state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl SelfHostedIntegrationRuntimeStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Self-hosted integration runtime status type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfHostedIntegrationRuntimeStatusTypeProperties {
    #[doc = "The time at which the integration runtime was created, in ISO8601 format."]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The task queue id of the integration runtime."]
    #[serde(rename = "taskQueueId", default, skip_serializing_if = "Option::is_none")]
    pub task_queue_id: Option<String>,
    #[doc = "It is used to set the encryption mode for node-node communication channel (when more than 2 self-hosted integration runtime nodes exist)."]
    #[serde(rename = "internalChannelEncryption", default, skip_serializing_if = "Option::is_none")]
    pub internal_channel_encryption: Option<String>,
    #[doc = "Version of the integration runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "List of nodes for this integration runtime."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub nodes: Vec<SelfHostedIntegrationRuntimeNode>,
    #[doc = "The date at which the integration runtime will be scheduled to update, in ISO8601 format."]
    #[serde(rename = "scheduledUpdateDate", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_update_date: Option<time::OffsetDateTime>,
    #[doc = "The time in the date scheduled by service to update the integration runtime, e.g., PT03H is 3 hours."]
    #[serde(rename = "updateDelayOffset", default, skip_serializing_if = "Option::is_none")]
    pub update_delay_offset: Option<String>,
    #[doc = "The local time zone offset in hours."]
    #[serde(rename = "localTimeZoneOffset", default, skip_serializing_if = "Option::is_none")]
    pub local_time_zone_offset: Option<String>,
    #[doc = "Object with additional information about integration runtime capabilities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[doc = "The URLs for the services used in integration runtime backend service."]
    #[serde(
        rename = "serviceUrls",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_urls: Vec<String>,
    #[doc = "Whether Self-hosted integration runtime auto update has been turned on."]
    #[serde(rename = "autoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<self_hosted_integration_runtime_status_type_properties::AutoUpdate>,
    #[doc = "Status of the integration runtime version."]
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[doc = "The version that the integration runtime is going to update to."]
    #[serde(rename = "pushedVersion", default, skip_serializing_if = "Option::is_none")]
    pub pushed_version: Option<String>,
    #[doc = "The latest version on download center."]
    #[serde(rename = "latestVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[doc = "The estimated time when the self-hosted integration runtime will be updated."]
    #[serde(rename = "autoUpdateETA", default, with = "azure_core::date::rfc3339::option")]
    pub auto_update_eta: Option<time::OffsetDateTime>,
}
impl SelfHostedIntegrationRuntimeStatusTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod self_hosted_integration_runtime_status_type_properties {
    use super::*;
    #[doc = "Whether Self-hosted integration runtime auto update has been turned on."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutoUpdate")]
    pub enum AutoUpdate {
        On,
        Off,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutoUpdate {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutoUpdate {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutoUpdate {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::On => serializer.serialize_unit_variant("AutoUpdate", 0u32, "On"),
                Self::Off => serializer.serialize_unit_variant("AutoUpdate", 1u32, "Off"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Credential type that uses Tenant ID and Service principal ID for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalAzureKeyVaultCredential {
    #[serde(flatten)]
    pub credential: Credential,
    #[doc = "The properties of service principal Azure Key Vault credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicePrincipalAzureKeyVaultCredentialProperties>,
}
impl ServicePrincipalAzureKeyVaultCredential {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            properties: None,
        }
    }
}
#[doc = "The properties of service principal Azure Key Vault credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalAzureKeyVaultCredentialProperties {
    #[doc = "The type properties of key vault secret service principal credential."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<KeyVaultSecretServicePrinipalCredentialTypeProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ServicePrincipalAzureKeyVaultCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Credential type that uses Sql for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlAuthAzureKeyVaultCredential {
    #[serde(flatten)]
    pub credential: Credential,
    #[doc = "Properties of user pass credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserPassCredentialProperties>,
}
impl SqlAuthAzureKeyVaultCredential {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            properties: None,
        }
    }
}
#[doc = "SQL server database credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "SQL server database credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerDatabaseCredentialScanProperties>,
}
impl SqlServerDatabaseCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "SQL server database credential scan properties."]
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
#[doc = "The sql server database data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of sql server database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerDatabaseProperties>,
}
impl SqlServerDatabaseDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of sql server database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerDatabaseProperties {
    #[serde(flatten)]
    pub azure_data_source_properties: AzureDataSourceProperties,
    #[doc = "The server endpoint of sql server database."]
    #[serde(rename = "serverEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
}
impl SqlServerDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sql server database scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Sql server database scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerDatabaseScanRulesetProperties>,
}
impl SqlServerDatabaseScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Sql server database scan ruleset properties."]
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
#[doc = "Sql server database system scan rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Sql server database scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerDatabaseScanRulesetProperties>,
}
impl SqlServerDatabaseSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "The store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Store {
    #[doc = "The reference name of store."]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[doc = "The type of store."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Store {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rule of system classification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemClassificationRule {
    #[serde(flatten)]
    pub classification_rule: ClassificationRule,
    #[doc = "The properties of system classification rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SystemClassificationRuleProperties>,
}
impl SystemClassificationRule {
    pub fn new(classification_rule: ClassificationRule) -> Self {
        Self {
            classification_rule,
            properties: None,
        }
    }
}
#[doc = "The properties of system classification rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemClassificationRuleProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The version of system classification rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "The classification name of system classification rule."]
    #[serde(rename = "classificationName", default, skip_serializing_if = "Option::is_none")]
    pub classification_name: Option<String>,
    #[doc = "The rule status of system classification rule."]
    #[serde(rename = "ruleStatus", default, skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<system_classification_rule_properties::RuleStatus>,
    #[doc = "The create time of system classification rule."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The last modified time of system classification rule."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemClassificationRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_classification_rule_properties {
    use super::*;
    #[doc = "The rule status of system classification rule."]
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
#[doc = "The system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemScanRuleset {
    #[serde(flatten)]
    pub versioned_scan_ruleset: VersionedScanRuleset,
}
impl SystemScanRuleset {
    pub fn new() -> Self {
        Self {
            versioned_scan_ruleset: VersionedScanRuleset::default(),
        }
    }
}
#[doc = "The data source type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SystemScanRulesetUnion {
    AdlsGen1(AdlsGen1SystemScanRuleset),
    AdlsGen2(AdlsGen2SystemScanRuleset),
    AmazonAccount(AmazonAccountSystemScanRuleset),
    AmazonPostgreSql(AmazonPostgreSqlSystemScanRuleset),
    AmazonS3(AmazonS3SystemScanRuleset),
    AmazonSql(AmazonSqlSystemScanRuleset),
    AzureCosmosDb(AzureCosmosDbSystemScanRuleset),
    AzureDataExplorer(AzureDataExplorerSystemScanRuleset),
    AzureFileService(AzureFileServiceSystemScanRuleset),
    AzureMySql(AzureMySqlSystemScanRuleset),
    AzurePostgreSql(AzurePostgreSqlSystemScanRuleset),
    AzureResourceGroup(AzureResourceGroupSystemScanRuleset),
    AzureSqlDataWarehouse(AzureSqlDataWarehouseSystemScanRuleset),
    AzureSqlDatabaseManagedInstance(AzureSqlDatabaseManagedInstanceSystemScanRuleset),
    AzureSqlDatabase(AzureSqlDatabaseSystemScanRuleset),
    AzureStorage(AzureStorageSystemScanRuleset),
    AzureSubscription(AzureSubscriptionSystemScanRuleset),
    AzureSynapse(AzureSynapseSystemScanRuleset),
    AzureSynapseWorkspace(AzureSynapseWorkspaceSystemScanRuleset),
    Oracle(OracleSystemScanRuleset),
    #[serde(rename = "PowerBI")]
    PowerBi(PowerBiSystemScanRuleset),
    SapEcc(SapEccSystemScanRuleset),
    SapS4Hana(SapS4HanaSystemScanRuleset),
    SqlServerDatabase(SqlServerDatabaseSystemScanRuleset),
    Teradata(TeradataSystemScanRuleset),
}
#[doc = "List of system scan rulesets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemScanRulesetList {
    #[doc = "List of system scan rulesets."]
    pub value: Vec<SystemScanRulesetUnion>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of system scan rulesets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for SystemScanRulesetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SystemScanRulesetList {
    pub fn new(value: Vec<SystemScanRulesetUnion>) -> Self {
        Self {
            value,
            next_link: None,
            count: None,
        }
    }
}
#[doc = "The teradata data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataDataSource {
    #[serde(flatten)]
    pub data_source: DataSource,
    #[doc = "The properties of teradata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TeradataProperties>,
}
impl TeradataDataSource {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            data_source,
            properties: None,
        }
    }
}
#[doc = "The properties of teradata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataProperties {
    #[serde(flatten)]
    pub data_source_properties: DataSourceProperties,
    #[doc = "The host of teradata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}
impl TeradataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Teradata scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataScanRuleset {
    #[serde(flatten)]
    pub scan_ruleset: ScanRuleset,
    #[doc = "Teradata scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TeradataScanRulesetProperties>,
}
impl TeradataScanRuleset {
    pub fn new(scan_ruleset: ScanRuleset) -> Self {
        Self {
            scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Teradata scan ruleset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataScanRulesetProperties {
    #[serde(flatten)]
    pub scanning_rule_scan_ruleset_properties: ScanningRuleScanRulesetProperties,
}
impl TeradataScanRulesetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Teradata system scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataSystemScanRuleset {
    #[serde(flatten)]
    pub system_scan_ruleset: SystemScanRuleset,
    #[doc = "Teradata scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TeradataScanRulesetProperties>,
}
impl TeradataSystemScanRuleset {
    pub fn new(system_scan_ruleset: SystemScanRuleset) -> Self {
        Self {
            system_scan_ruleset,
            properties: None,
        }
    }
}
#[doc = "Teradata credential scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataTeradataCredentialScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Teradata credential scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TeradataTeradataCredentialScanProperties>,
}
impl TeradataTeradataCredentialScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Teradata credential scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataTeradataCredentialScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[doc = "The credential reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "driverLocation", default, skip_serializing_if = "Option::is_none")]
    pub driver_location: Option<String>,
    #[serde(rename = "storedProcedureDetails", default, skip_serializing_if = "Option::is_none")]
    pub stored_procedure_details: Option<String>,
}
impl TeradataTeradataCredentialScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Teradata user pass scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataTeradataUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Teradata user pass scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TeradataTeradataUserPassScanProperties>,
}
impl TeradataTeradataUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Teradata user pass scan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeradataTeradataUserPassScanProperties {
    #[serde(flatten)]
    pub miti_scan_properties: MitiScanProperties,
    #[doc = "The username of Teradata server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password of Teradata server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "The driver location."]
    #[serde(rename = "driverLocation", default, skip_serializing_if = "Option::is_none")]
    pub driver_location: Option<String>,
}
impl TeradataTeradataUserPassScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Teradata user pass scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeradataUserPassScan {
    #[serde(flatten)]
    pub scan: Scan,
    #[doc = "Teradata user pass scan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TeradataUserPassScanProperties>,
}
impl TeradataUserPassScan {
    pub fn new(scan: Scan) -> Self {
        Self { scan, properties: None }
    }
}
#[doc = "Teradata user pass scan properties."]
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
#[doc = "The trigger object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Trigger {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties detail of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TriggerProperties>,
    #[doc = "The validation error info of trigger."]
    #[serde(rename = "validationErrorInfo", default, skip_serializing_if = "Option::is_none")]
    pub validation_error_info: Option<String>,
}
impl Trigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties detail of trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerProperties {
    #[doc = "The recurrence of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<TriggerRecurrence>,
    #[doc = "The recurrence interval of trigger."]
    #[serde(rename = "recurrenceInterval", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_interval: Option<String>,
    #[doc = "The create time of trigger."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The last modified time of trigger."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[doc = "The last scheduled time of trigger."]
    #[serde(rename = "lastScheduled", default, with = "azure_core::date::rfc3339::option")]
    pub last_scheduled: Option<time::OffsetDateTime>,
    #[doc = "The state of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<trigger_properties::State>,
    #[doc = "The scan level of trigger."]
    #[serde(rename = "scanLevel", default, skip_serializing_if = "Option::is_none")]
    pub scan_level: Option<trigger_properties::ScanLevel>,
    #[doc = "The incremental scan start time of trigger."]
    #[serde(rename = "incrementalScanStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub incremental_scan_start_time: Option<time::OffsetDateTime>,
}
impl TriggerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod trigger_properties {
    use super::*;
    #[doc = "The state of trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for State {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "The scan level of trigger."]
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
#[doc = "The recurrence of trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerRecurrence {
    #[doc = "The frequency of trigger recurrence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<trigger_recurrence::Frequency>,
    #[doc = "The interval of trigger recurrence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[doc = "The start time of trigger recurrence."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of trigger recurrence."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Schedule of recurrence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<RecurrenceSchedule>,
    #[doc = "The time zone of trigger recurrence."]
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
    #[doc = "The frequency of trigger recurrence."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Frequency")]
    pub enum Frequency {
        Week,
        Month,
        Day,
        Hour,
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
                Self::Day => serializer.serialize_unit_variant("Frequency", 2u32, "Day"),
                Self::Hour => serializer.serialize_unit_variant("Frequency", 3u32, "Hour"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of user pass credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserPassCredentialProperties {
    #[doc = "Properties of key vault secret user pass credential type."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<KeyVaultSecretUserPassCredentialTypeProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl UserPassCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The versioned scan ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionedScanRuleset {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Versioned scan ruleset properties."]
    #[serde(rename = "scanRulesetType", default, skip_serializing_if = "Option::is_none")]
    pub scan_ruleset_type: Option<versioned_scan_ruleset::ScanRulesetType>,
    #[doc = "Status of versioned scan ruleset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<versioned_scan_ruleset::Status>,
    #[doc = "Version of versioned scan ruleset properties."]
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
    #[doc = "Versioned scan ruleset properties."]
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
    #[doc = "Status of versioned scan ruleset properties."]
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
