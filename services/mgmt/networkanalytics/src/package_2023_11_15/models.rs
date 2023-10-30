#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The details for storage account sas creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSas {
    #[doc = "Sas token start timestamp."]
    #[serde(rename = "startTimeStamp", with = "azure_core::date::rfc3339")]
    pub start_time_stamp: time::OffsetDateTime,
    #[doc = "Sas token expiry timestamp."]
    #[serde(rename = "expiryTimeStamp", with = "azure_core::date::rfc3339")]
    pub expiry_time_stamp: time::OffsetDateTime,
    #[doc = "Ip Address"]
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}
impl AccountSas {
    pub fn new(start_time_stamp: time::OffsetDateTime, expiry_time_stamp: time::OffsetDateTime, ip_address: String) -> Self {
        Self {
            start_time_stamp,
            expiry_time_stamp,
            ip_address,
        }
    }
}
#[doc = "Details of storage account sas token ."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSasToken {
    #[doc = "Field to specify storage account sas token."]
    #[serde(rename = "storageAccountSasToken")]
    pub storage_account_sas_token: String,
}
impl AccountSasToken {
    pub fn new(storage_account_sas_token: String) -> Self {
        Self { storage_account_sas_token }
    }
}
#[doc = "Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Possible values are any combination of Logging|Metrics|AzureServices (For example, \"Logging, Metrics\"), or None to bypass none of those traffics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Bypass")]
pub enum Bypass {
    None,
    Logging,
    Metrics,
    AzureServices,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Bypass {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Bypass {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Bypass {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("Bypass", 0u32, "None"),
            Self::Logging => serializer.serialize_unit_variant("Bypass", 1u32, "Logging"),
            Self::Metrics => serializer.serialize_unit_variant("Bypass", 2u32, "Metrics"),
            Self::AzureServices => serializer.serialize_unit_variant("Bypass", 3u32, "AzureServices"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details of Consumption Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumptionEndpointsProperties {
    #[doc = "Ingestion url to upload the data."]
    #[serde(rename = "ingestionUrl", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_url: Option<String>,
    #[doc = "Resource Id of ingestion endpoint."]
    #[serde(rename = "ingestionResourceId", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_resource_id: Option<String>,
    #[doc = "Url to consume file type."]
    #[serde(rename = "fileAccessUrl", default, skip_serializing_if = "Option::is_none")]
    pub file_access_url: Option<String>,
    #[doc = "Resource Id of file access endpoint."]
    #[serde(rename = "fileAccessResourceId", default, skip_serializing_if = "Option::is_none")]
    pub file_access_resource_id: Option<String>,
    #[doc = "Url to consume the processed data."]
    #[serde(rename = "queryUrl", default, skip_serializing_if = "Option::is_none")]
    pub query_url: Option<String>,
    #[doc = "Resource Id of query endpoint."]
    #[serde(rename = "queryResourceId", default, skip_serializing_if = "Option::is_none")]
    pub query_resource_id: Option<String>,
}
impl ConsumptionEndpointsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details for container sas creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerSaS {
    #[doc = "Sas token start timestamp."]
    #[serde(rename = "startTimeStamp", with = "azure_core::date::rfc3339")]
    pub start_time_stamp: time::OffsetDateTime,
    #[doc = "Sas token expiry timestamp."]
    #[serde(rename = "expiryTimeStamp", with = "azure_core::date::rfc3339")]
    pub expiry_time_stamp: time::OffsetDateTime,
    #[doc = "Ip Address"]
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}
impl ContainerSaS {
    pub fn new(start_time_stamp: time::OffsetDateTime, expiry_time_stamp: time::OffsetDateTime, ip_address: String) -> Self {
        Self {
            start_time_stamp,
            expiry_time_stamp,
            ip_address,
        }
    }
}
#[doc = "Details of storage container account sas token ."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerSasToken {
    #[doc = "Field to specify storage container sas token."]
    #[serde(rename = "storageContainerSasToken")]
    pub storage_container_sas_token: String,
}
impl ContainerSasToken {
    pub fn new(storage_container_sas_token: String) -> Self {
        Self {
            storage_container_sas_token,
        }
    }
}
#[doc = "The data type state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ControlState")]
pub enum ControlState {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ControlState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ControlState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ControlState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("ControlState", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("ControlState", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The data product resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataProduct {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The data product properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataProductProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl DataProduct {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Data Product Information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataProductInformation {
    #[doc = "Name of data product."]
    #[serde(rename = "dataProductName")]
    pub data_product_name: String,
    #[doc = "Description about data product."]
    pub description: String,
    #[doc = "Version information of data product."]
    #[serde(rename = "dataProductVersions")]
    pub data_product_versions: Vec<DataProductVersion>,
}
impl DataProductInformation {
    pub fn new(data_product_name: String, description: String, data_product_versions: Vec<DataProductVersion>) -> Self {
        Self {
            data_product_name,
            description,
            data_product_versions,
        }
    }
}
#[doc = "The response of a DataProduct list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataProductListResult {
    #[doc = "The DataProduct items on this page"]
    pub value: Vec<DataProduct>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataProductListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataProductListResult {
    pub fn new(value: Vec<DataProduct>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Data Product Network rule set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataProductNetworkAcls {
    #[doc = "Virtual Network Rule"]
    #[serde(rename = "virtualNetworkRule")]
    pub virtual_network_rule: Vec<VirtualNetworkRule>,
    #[doc = "IP rule with specific IP or IP range in CIDR format."]
    #[serde(rename = "ipRules")]
    pub ip_rules: Vec<IpRules>,
    #[doc = "The list of query ips in the format of CIDR allowed to connect to query/visualization endpoint."]
    #[serde(rename = "allowedQueryIpRangeList")]
    pub allowed_query_ip_range_list: Vec<String>,
    #[doc = "Specifies the default action of allow or deny when no other rules match."]
    #[serde(rename = "defaultAction")]
    pub default_action: DefaultAction,
}
impl DataProductNetworkAcls {
    pub fn new(
        virtual_network_rule: Vec<VirtualNetworkRule>,
        ip_rules: Vec<IpRules>,
        allowed_query_ip_range_list: Vec<String>,
        default_action: DefaultAction,
    ) -> Self {
        Self {
            virtual_network_rule,
            ip_rules,
            allowed_query_ip_range_list,
            default_action,
        }
    }
}
#[doc = "The data product properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataProductProperties {
    #[doc = "The resource GUID property of the data product resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Data product publisher name."]
    pub publisher: String,
    #[doc = "Product name of data product."]
    pub product: String,
    #[doc = "Major version of data product."]
    #[serde(rename = "majorVersion")]
    pub major_version: String,
    #[doc = "List of name or email associated with data product resource deployment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub owners: Vec<String>,
    #[doc = "The data type state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redundancy: Option<ControlState>,
    #[doc = "Purview account url for data product to connect to."]
    #[serde(rename = "purviewAccount", default, skip_serializing_if = "Option::is_none")]
    pub purview_account: Option<String>,
    #[doc = "Purview collection url for data product to connect to."]
    #[serde(rename = "purviewCollection", default, skip_serializing_if = "Option::is_none")]
    pub purview_collection: Option<String>,
    #[doc = "The data type state"]
    #[serde(rename = "privateLinksEnabled", default, skip_serializing_if = "Option::is_none")]
    pub private_links_enabled: Option<ControlState>,
    #[doc = "The data type state"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<ControlState>,
    #[doc = "The data type state"]
    #[serde(rename = "customerManagedKeyEncryptionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub customer_managed_key_encryption_enabled: Option<ControlState>,
    #[doc = "Encryption key details."]
    #[serde(rename = "customerEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key: Option<EncryptionKeyDetails>,
    #[doc = "Data Product Network rule set"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub networkacls: Option<DataProductNetworkAcls>,
    #[doc = "ManagedResourceGroup related properties"]
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<ManagedResourceGroupConfiguration>,
    #[doc = "List of available minor versions of the data product resource."]
    #[serde(
        rename = "availableMinorVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub available_minor_versions: Vec<String>,
    #[doc = "Current configured minor version of the data product resource."]
    #[serde(rename = "currentMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_minor_version: Option<String>,
    #[doc = "Documentation link for the data product based on definition file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[doc = "Details of Consumption Properties"]
    #[serde(rename = "consumptionEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub consumption_endpoints: Option<ConsumptionEndpointsProperties>,
    #[doc = "Key vault url."]
    #[serde(rename = "keyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
}
impl DataProductProperties {
    pub fn new(publisher: String, product: String, major_version: String) -> Self {
        Self {
            resource_guid: None,
            provisioning_state: None,
            publisher,
            product,
            major_version,
            owners: Vec::new(),
            redundancy: None,
            purview_account: None,
            purview_collection: None,
            private_links_enabled: None,
            public_network_access: None,
            customer_managed_key_encryption_enabled: None,
            customer_encryption_key: None,
            networkacls: None,
            managed_resource_group_configuration: None,
            available_minor_versions: Vec::new(),
            current_minor_version: None,
            documentation: None,
            consumption_endpoints: None,
            key_vault_url: None,
        }
    }
}
#[doc = "The type used for update operations of the DataProduct."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProductUpdate {
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the DataProduct."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataProductUpdateProperties>,
}
impl DataProductUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the DataProduct."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProductUpdateProperties {
    #[doc = "List of name or email associated with data product resource deployment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub owners: Vec<String>,
    #[doc = "Purview account url for data product to connect to."]
    #[serde(rename = "purviewAccount", default, skip_serializing_if = "Option::is_none")]
    pub purview_account: Option<String>,
    #[doc = "Purview collection url for data product to connect to."]
    #[serde(rename = "purviewCollection", default, skip_serializing_if = "Option::is_none")]
    pub purview_collection: Option<String>,
    #[doc = "The data type state"]
    #[serde(rename = "privateLinksEnabled", default, skip_serializing_if = "Option::is_none")]
    pub private_links_enabled: Option<ControlState>,
    #[doc = "Current configured minor version of the data product resource."]
    #[serde(rename = "currentMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_minor_version: Option<String>,
}
impl DataProductUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data type state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataProductUserRole")]
pub enum DataProductUserRole {
    Reader,
    SensitiveReader,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataProductUserRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataProductUserRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataProductUserRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Reader => serializer.serialize_unit_variant("DataProductUserRole", 0u32, "Reader"),
            Self::SensitiveReader => serializer.serialize_unit_variant("DataProductUserRole", 1u32, "SensitiveReader"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Data Product Version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataProductVersion {
    #[doc = "Version of data product"]
    pub version: String,
}
impl DataProductVersion {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}
#[doc = "The data catalog resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProductsCatalog {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Details for data catalog properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataProductsCatalogProperties>,
}
impl DataProductsCatalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DataProductsCatalog list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataProductsCatalogListResult {
    #[doc = "The DataProductsCatalog items on this page"]
    pub value: Vec<DataProductsCatalog>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataProductsCatalogListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataProductsCatalogListResult {
    pub fn new(value: Vec<DataProductsCatalog>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details for data catalog properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataProductsCatalogProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The data product publisher information."]
    pub publishers: Vec<PublisherInformation>,
}
impl DataProductsCatalogProperties {
    pub fn new(publishers: Vec<PublisherInformation>) -> Self {
        Self {
            provisioning_state: None,
            publishers,
        }
    }
}
#[doc = "The data type resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataType {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The data type properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataTypeProperties>,
}
impl DataType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DataType list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataTypeListResult {
    #[doc = "The DataType items on this page"]
    pub value: Vec<DataType>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataTypeListResult {
    pub fn new(value: Vec<DataType>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The data type properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataTypeProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The data type state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DataTypeState>,
    #[doc = "Reason for the state of data type."]
    #[serde(rename = "stateReason", default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[doc = "Field for storage output retention in days."]
    #[serde(rename = "storageOutputRetention", default, skip_serializing_if = "Option::is_none")]
    pub storage_output_retention: Option<i32>,
    #[doc = "Field for database cache retention in days."]
    #[serde(rename = "databaseCacheRetention", default, skip_serializing_if = "Option::is_none")]
    pub database_cache_retention: Option<i32>,
    #[doc = "Field for database data retention in days."]
    #[serde(rename = "databaseRetention", default, skip_serializing_if = "Option::is_none")]
    pub database_retention: Option<i32>,
    #[doc = "Url for data visualization."]
    #[serde(rename = "visualizationUrl", default, skip_serializing_if = "Option::is_none")]
    pub visualization_url: Option<String>,
}
impl DataTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data type state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataTypeState")]
pub enum DataTypeState {
    Stopped,
    Running,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataTypeState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataTypeState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataTypeState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Stopped => serializer.serialize_unit_variant("DataTypeState", 0u32, "Stopped"),
            Self::Running => serializer.serialize_unit_variant("DataTypeState", 1u32, "Running"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type used for update operations of the DataType."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataTypeUpdate {
    #[doc = "The updatable properties of the DataType."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataTypeUpdateProperties>,
}
impl DataTypeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the DataType."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataTypeUpdateProperties {
    #[doc = "The data type state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DataTypeState>,
    #[doc = "Field for storage output retention in days."]
    #[serde(rename = "storageOutputRetention", default, skip_serializing_if = "Option::is_none")]
    pub storage_output_retention: Option<i32>,
    #[doc = "Field for database cache retention in days."]
    #[serde(rename = "databaseCacheRetention", default, skip_serializing_if = "Option::is_none")]
    pub database_cache_retention: Option<i32>,
    #[doc = "Field for database data retention in days."]
    #[serde(rename = "databaseRetention", default, skip_serializing_if = "Option::is_none")]
    pub database_retention: Option<i32>,
}
impl DataTypeUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the default action of allow or deny when no other rules match."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DefaultAction")]
pub enum DefaultAction {
    Allow,
    Deny,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DefaultAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DefaultAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DefaultAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Allow => serializer.serialize_unit_variant("DefaultAction", 0u32, "Allow"),
            Self::Deny => serializer.serialize_unit_variant("DefaultAction", 1u32, "Deny"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Encryption key details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionKeyDetails {
    #[doc = "The Uri of the key vault."]
    #[serde(rename = "keyVaultUri")]
    pub key_vault_uri: String,
    #[doc = "The name of the key vault key."]
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[doc = "The version of the key vault key."]
    #[serde(rename = "keyVersion")]
    pub key_version: String,
}
impl EncryptionKeyDetails {
    pub fn new(key_vault_uri: String, key_name: String, key_version: String) -> Self {
        Self {
            key_vault_uri,
            key_name,
            key_version,
        }
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP rule with specific IP or IP range in CIDR format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRules {
    #[doc = "IP Rules Value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The action of virtual network rule."]
    pub action: String,
}
impl IpRules {
    pub fn new(action: String) -> Self {
        Self { value: None, action }
    }
}
#[doc = "Details for KeyVault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultInfo {
    #[doc = "key vault url."]
    #[serde(rename = "keyVaultUrl")]
    pub key_vault_url: String,
}
impl KeyVaultInfo {
    pub fn new(key_vault_url: String) -> Self {
        Self { key_vault_url }
    }
}
#[doc = "list role assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListRoleAssignments {
    #[doc = "Count of role assignments."]
    pub count: i32,
    #[doc = "list of role assignments"]
    #[serde(rename = "roleAssignmentResponse")]
    pub role_assignment_response: Vec<RoleAssignmentDetail>,
}
impl ListRoleAssignments {
    pub fn new(count: i32, role_assignment_response: Vec<RoleAssignmentDetail>) -> Self {
        Self {
            count,
            role_assignment_response,
        }
    }
}
#[doc = "ManagedResourceGroup related properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedResourceGroupConfiguration {
    #[doc = "Name of managed resource group"]
    pub name: String,
    #[doc = "Managed Resource Group location"]
    pub location: String,
}
impl ManagedResourceGroupConfiguration {
    pub fn new(name: String, location: String) -> Self {
        Self { name, location }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned, UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the current operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
    Accepted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details for Publisher Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublisherInformation {
    #[doc = "Name of the publisher."]
    #[serde(rename = "publisherName")]
    pub publisher_name: String,
    #[doc = "Data product information."]
    #[serde(rename = "dataProducts")]
    pub data_products: Vec<DataProductInformation>,
}
impl PublisherInformation {
    pub fn new(publisher_name: String, data_products: Vec<DataProductInformation>) -> Self {
        Self {
            publisher_name,
            data_products,
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Access Rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceAccessRules {
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "Resource ID"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}
impl ResourceAccessRules {
    pub fn new(tenant_id: String, resource_id: String) -> Self {
        Self { tenant_id, resource_id }
    }
}
#[doc = "The details for role assignment common properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentCommonProperties {
    #[doc = "Role Id of the Built-In Role"]
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[doc = "Object ID of the AAD principal or security-group."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "User name."]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[doc = "Data Type Scope at which the role assignment is created."]
    #[serde(rename = "dataTypeScope")]
    pub data_type_scope: Vec<String>,
    #[doc = "Type of the principal Id: User, Group or ServicePrincipal"]
    #[serde(rename = "principalType")]
    pub principal_type: String,
    #[doc = "The data type state"]
    pub role: DataProductUserRole,
}
impl RoleAssignmentCommonProperties {
    pub fn new(
        role_id: String,
        principal_id: String,
        user_name: String,
        data_type_scope: Vec<String>,
        principal_type: String,
        role: DataProductUserRole,
    ) -> Self {
        Self {
            role_id,
            principal_id,
            user_name,
            data_type_scope,
            principal_type,
            role,
        }
    }
}
#[doc = "The details for role assignment response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentDetail {
    #[doc = "Role Id of the Built-In Role"]
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[doc = "Object ID of the AAD principal or security-group."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "User name."]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[doc = "Data Type Scope at which the role assignment is created."]
    #[serde(rename = "dataTypeScope")]
    pub data_type_scope: Vec<String>,
    #[doc = "Type of the principal Id: User, Group or ServicePrincipal"]
    #[serde(rename = "principalType")]
    pub principal_type: String,
    #[doc = "The data type state"]
    pub role: DataProductUserRole,
    #[doc = "Id of role assignment request"]
    #[serde(rename = "roleAssignmentId")]
    pub role_assignment_id: String,
}
impl RoleAssignmentDetail {
    pub fn new(
        role_id: String,
        principal_id: String,
        user_name: String,
        data_type_scope: Vec<String>,
        principal_type: String,
        role: DataProductUserRole,
        role_assignment_id: String,
    ) -> Self {
        Self {
            role_id,
            principal_id,
            user_name,
            data_type_scope,
            principal_type,
            role,
            role_assignment_id,
        }
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The available API versions for the Microsoft.NetworkAnalytics RP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Versions")]
pub enum Versions {
    #[serde(rename = "2023-11-15")]
    N2023_11_15,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Versions {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Versions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Versions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N2023_11_15 => serializer.serialize_unit_variant("Versions", 0u32, "2023-11-15"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Virtual Network Rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[doc = "Resource ID of a subnet"]
    pub id: String,
    #[doc = "The action of virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "Gets the state of virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self {
            id,
            action: None,
            state: None,
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
