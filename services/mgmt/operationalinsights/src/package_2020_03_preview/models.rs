#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Service Tier details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableServiceTier {
    #[doc = "The name of the Service Tier."]
    #[serde(rename = "serviceTier", default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<available_service_tier::ServiceTier>,
    #[doc = "True if the Service Tier is enabled for the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The minimum retention for the Service Tier, in days."]
    #[serde(rename = "minimumRetention", default, skip_serializing_if = "Option::is_none")]
    pub minimum_retention: Option<i64>,
    #[doc = "The maximum retention for the Service Tier, in days."]
    #[serde(rename = "maximumRetention", default, skip_serializing_if = "Option::is_none")]
    pub maximum_retention: Option<i64>,
    #[doc = "The default retention for the Service Tier, in days."]
    #[serde(rename = "defaultRetention", default, skip_serializing_if = "Option::is_none")]
    pub default_retention: Option<i64>,
    #[doc = "The capacity reservation level in GB per day. Returned for the Capacity Reservation Service Tier."]
    #[serde(rename = "capacityReservationLevel", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_level: Option<i64>,
    #[doc = "Time when the sku was last updated for the workspace. Returned for the Capacity Reservation Service Tier."]
    #[serde(rename = "lastSkuUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_sku_update: Option<String>,
}
impl AvailableServiceTier {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod available_service_tier {
    use super::*;
    #[doc = "The name of the Service Tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceTier")]
    pub enum ServiceTier {
        Free,
        Standard,
        Premium,
        PerNode,
        #[serde(rename = "PerGB2018")]
        PerGb2018,
        Standalone,
        CapacityReservation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Free => serializer.serialize_unit_variant("ServiceTier", 0u32, "Free"),
                Self::Standard => serializer.serialize_unit_variant("ServiceTier", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("ServiceTier", 2u32, "Premium"),
                Self::PerNode => serializer.serialize_unit_variant("ServiceTier", 3u32, "PerNode"),
                Self::PerGb2018 => serializer.serialize_unit_variant("ServiceTier", 4u32, "PerGB2018"),
                Self::Standalone => serializer.serialize_unit_variant("ServiceTier", 5u32, "Standalone"),
                Self::CapacityReservation => serializer.serialize_unit_variant("ServiceTier", 6u32, "CapacityReservation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource model definition for an Azure Resource Manager resource with an etag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl AzureEntityResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The top level Log Analytics cluster resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "The cluster sku definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[doc = "Cluster properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl Cluster {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterErrorResponse {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for ClusterErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ClusterErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list clusters operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterListResult {
    #[doc = "The link used to get the next page of recommendations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of Log Analytics clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
}
impl azure_core::Continuable for ClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The top level Log Analytics cluster resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPatch {
    #[doc = "Log Analytics cluster patch properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterPatchProperties>,
    #[doc = "The cluster sku definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ClusterPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log Analytics cluster patch properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPatchProperties {
    #[doc = "The key vault properties."]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
impl ClusterPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cluster properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[doc = "The link used to get the next page of recommendations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The ID associated with the cluster."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The provisioning state of the cluster."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
    #[doc = "The key vault properties."]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_properties {
    use super::*;
    #[doc = "The provisioning state of the cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        Canceled,
        Deleting,
        ProvisioningAccount,
        Updating,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::ProvisioningAccount => serializer.serialize_unit_variant("ProvisioningState", 5u32, "ProvisioningAccount"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The cluster sku definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterSku {
    #[doc = "The capacity value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[doc = "The name of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<cluster_sku::Name>,
}
impl ClusterSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_sku {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        CapacityReservation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CapacityReservation => serializer.serialize_unit_variant("Name", 0u32, "CapacityReservation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The core summary of a search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreSummary {
    #[doc = "The status of a core summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The number of documents of a core summary."]
    #[serde(rename = "numberOfDocuments")]
    pub number_of_documents: i64,
}
impl CoreSummary {
    pub fn new(number_of_documents: i64) -> Self {
        Self {
            status: None,
            number_of_documents,
        }
    }
}
#[doc = "Data collector log top level resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectorLog {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Data collector log properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataCollectorLogProperties>,
}
impl DataCollectorLog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data collector log properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectorLogProperties {
    #[doc = "Table's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DataCollectorLogProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data collector log tables collection, all tables are scoped to the specified workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectorLogsListResult {
    #[doc = "data collector log collection."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataCollectorLog>,
}
impl azure_core::Continuable for DataCollectorLogsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DataCollectorLogsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The top level data export resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataExport {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Data Export properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataExportProperties>,
}
impl DataExport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataExportErrorResponse {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for DataExportErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DataExportErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list data exports."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataExportListResult {
    #[doc = "List of data export instances within a workspace.."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataExport>,
}
impl azure_core::Continuable for DataExportListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DataExportListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Export properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportProperties {
    #[doc = "The data export rule ID."]
    #[serde(rename = "dataExportId", default, skip_serializing_if = "Option::is_none")]
    pub data_export_id: Option<String>,
    #[doc = "An array of tables to export, for example: [“Heartbeat, SecurityEvent”]."]
    #[serde(rename = "tableNames")]
    pub table_names: Vec<String>,
    #[doc = "Destination properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[doc = "Active when enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[doc = "The latest data export rule modification time."]
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[doc = "Date and time when the export was last modified."]
    #[serde(rename = "lastModifiedDate", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
}
impl DataExportProperties {
    pub fn new(table_names: Vec<String>) -> Self {
        Self {
            data_export_id: None,
            table_names,
            destination: None,
            enable: None,
            created_date: None,
            last_modified_date: None,
        }
    }
}
#[doc = "Datasources under OMS Workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "JSON object"]
    pub properties: Object,
    #[doc = "The ETag of the data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The kind of the DataSource."]
    pub kind: DataSourceKind,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DataSource {
    pub fn new(properties: Object, kind: DataSourceKind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            etag: None,
            kind,
            tags: None,
        }
    }
}
#[doc = "DataSource filter. Right now, only filter by kind is supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceFilter {
    #[doc = "The kind of the DataSource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<DataSourceKind>,
}
impl DataSourceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the DataSource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataSourceKind")]
pub enum DataSourceKind {
    WindowsEvent,
    WindowsPerformanceCounter,
    #[serde(rename = "IISLogs")]
    IisLogs,
    LinuxSyslog,
    LinuxSyslogCollection,
    LinuxPerformanceObject,
    LinuxPerformanceCollection,
    CustomLog,
    CustomLogCollection,
    AzureAuditLog,
    AzureActivityLog,
    GenericDataSource,
    ChangeTrackingCustomPath,
    ChangeTrackingPath,
    ChangeTrackingServices,
    ChangeTrackingDataTypeConfiguration,
    ChangeTrackingDefaultRegistry,
    ChangeTrackingRegistry,
    ChangeTrackingLinuxPath,
    LinuxChangeTrackingPath,
    ChangeTrackingContentLocation,
    WindowsTelemetry,
    Office365,
    SecurityWindowsBaselineConfiguration,
    SecurityCenterSecurityWindowsBaselineConfiguration,
    SecurityEventCollectionConfiguration,
    SecurityInsightsSecurityEventCollectionConfiguration,
    ImportComputerGroup,
    NetworkMonitoring,
    Itsm,
    DnsAnalytics,
    ApplicationInsights,
    SqlDataClassification,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataSourceKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataSourceKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataSourceKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WindowsEvent => serializer.serialize_unit_variant("DataSourceKind", 0u32, "WindowsEvent"),
            Self::WindowsPerformanceCounter => serializer.serialize_unit_variant("DataSourceKind", 1u32, "WindowsPerformanceCounter"),
            Self::IisLogs => serializer.serialize_unit_variant("DataSourceKind", 2u32, "IISLogs"),
            Self::LinuxSyslog => serializer.serialize_unit_variant("DataSourceKind", 3u32, "LinuxSyslog"),
            Self::LinuxSyslogCollection => serializer.serialize_unit_variant("DataSourceKind", 4u32, "LinuxSyslogCollection"),
            Self::LinuxPerformanceObject => serializer.serialize_unit_variant("DataSourceKind", 5u32, "LinuxPerformanceObject"),
            Self::LinuxPerformanceCollection => serializer.serialize_unit_variant("DataSourceKind", 6u32, "LinuxPerformanceCollection"),
            Self::CustomLog => serializer.serialize_unit_variant("DataSourceKind", 7u32, "CustomLog"),
            Self::CustomLogCollection => serializer.serialize_unit_variant("DataSourceKind", 8u32, "CustomLogCollection"),
            Self::AzureAuditLog => serializer.serialize_unit_variant("DataSourceKind", 9u32, "AzureAuditLog"),
            Self::AzureActivityLog => serializer.serialize_unit_variant("DataSourceKind", 10u32, "AzureActivityLog"),
            Self::GenericDataSource => serializer.serialize_unit_variant("DataSourceKind", 11u32, "GenericDataSource"),
            Self::ChangeTrackingCustomPath => serializer.serialize_unit_variant("DataSourceKind", 12u32, "ChangeTrackingCustomPath"),
            Self::ChangeTrackingPath => serializer.serialize_unit_variant("DataSourceKind", 13u32, "ChangeTrackingPath"),
            Self::ChangeTrackingServices => serializer.serialize_unit_variant("DataSourceKind", 14u32, "ChangeTrackingServices"),
            Self::ChangeTrackingDataTypeConfiguration => {
                serializer.serialize_unit_variant("DataSourceKind", 15u32, "ChangeTrackingDataTypeConfiguration")
            }
            Self::ChangeTrackingDefaultRegistry => {
                serializer.serialize_unit_variant("DataSourceKind", 16u32, "ChangeTrackingDefaultRegistry")
            }
            Self::ChangeTrackingRegistry => serializer.serialize_unit_variant("DataSourceKind", 17u32, "ChangeTrackingRegistry"),
            Self::ChangeTrackingLinuxPath => serializer.serialize_unit_variant("DataSourceKind", 18u32, "ChangeTrackingLinuxPath"),
            Self::LinuxChangeTrackingPath => serializer.serialize_unit_variant("DataSourceKind", 19u32, "LinuxChangeTrackingPath"),
            Self::ChangeTrackingContentLocation => {
                serializer.serialize_unit_variant("DataSourceKind", 20u32, "ChangeTrackingContentLocation")
            }
            Self::WindowsTelemetry => serializer.serialize_unit_variant("DataSourceKind", 21u32, "WindowsTelemetry"),
            Self::Office365 => serializer.serialize_unit_variant("DataSourceKind", 22u32, "Office365"),
            Self::SecurityWindowsBaselineConfiguration => {
                serializer.serialize_unit_variant("DataSourceKind", 23u32, "SecurityWindowsBaselineConfiguration")
            }
            Self::SecurityCenterSecurityWindowsBaselineConfiguration => {
                serializer.serialize_unit_variant("DataSourceKind", 24u32, "SecurityCenterSecurityWindowsBaselineConfiguration")
            }
            Self::SecurityEventCollectionConfiguration => {
                serializer.serialize_unit_variant("DataSourceKind", 25u32, "SecurityEventCollectionConfiguration")
            }
            Self::SecurityInsightsSecurityEventCollectionConfiguration => {
                serializer.serialize_unit_variant("DataSourceKind", 26u32, "SecurityInsightsSecurityEventCollectionConfiguration")
            }
            Self::ImportComputerGroup => serializer.serialize_unit_variant("DataSourceKind", 27u32, "ImportComputerGroup"),
            Self::NetworkMonitoring => serializer.serialize_unit_variant("DataSourceKind", 28u32, "NetworkMonitoring"),
            Self::Itsm => serializer.serialize_unit_variant("DataSourceKind", 29u32, "Itsm"),
            Self::DnsAnalytics => serializer.serialize_unit_variant("DataSourceKind", 30u32, "DnsAnalytics"),
            Self::ApplicationInsights => serializer.serialize_unit_variant("DataSourceKind", 31u32, "ApplicationInsights"),
            Self::SqlDataClassification => serializer.serialize_unit_variant("DataSourceKind", 32u32, "SqlDataClassification"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list data source by workspace operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceListResult {
    #[doc = "A list of datasources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataSource>,
    #[doc = "The link (url) to the next page of datasources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataSourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataSourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Destination properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    #[doc = "The destination resource ID. This can be copied from the Properties entry of the destination resource in Azure."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The type of the destination resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<destination::Type>,
    #[doc = "Destination meta data."]
    #[serde(rename = "metaData", default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<DestinationMetaData>,
}
impl Destination {
    pub fn new(resource_id: String) -> Self {
        Self {
            resource_id,
            type_: None,
            meta_data: None,
        }
    }
}
pub mod destination {
    use super::*;
    #[doc = "The type of the destination resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        StorageAccount,
        EventHub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StorageAccount => serializer.serialize_unit_variant("Type", 0u32, "StorageAccount"),
                Self::EventHub => serializer.serialize_unit_variant("Type", 1u32, "EventHub"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Destination meta data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DestinationMetaData {
    #[doc = "Optional. Allows to define an Event Hub name. Not applicable when destination is Storage Account."]
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
}
impl DestinationMetaData {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Contains details when the response code indicates an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorContract {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for ErrorContract {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type")]
    pub type_: identity::Type,
}
impl Identity {
    pub fn new(type_: identity::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[doc = "Intelligence Pack containing a string name and boolean indicating if it's enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntelligencePack {
    #[doc = "The name of the intelligence pack."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The enabled boolean for the intelligence pack."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The display name of the intelligence pack."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl IntelligencePack {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The top level Linked service resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedService {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Linked service properties."]
    pub properties: LinkedServiceProperties,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl LinkedService {
    pub fn new(properties: LinkedServiceProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            tags: None,
        }
    }
}
#[doc = "The list linked service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedServiceListResult {
    #[doc = "The list of linked service instances"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedService>,
}
impl azure_core::Continuable for LinkedServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LinkedServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Linked service properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedServiceProperties {
    #[doc = "The resource id of the resource that will be linked to the workspace. This should be used for linking resources which require read access"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The resource id of the resource that will be linked to the workspace. This should be used for linking resources which require write access"]
    #[serde(rename = "writeAccessResourceId", default, skip_serializing_if = "Option::is_none")]
    pub write_access_resource_id: Option<String>,
    #[doc = "The provisioning state of the linked service."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<linked_service_properties::ProvisioningState>,
}
impl LinkedServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linked_service_properties {
    use super::*;
    #[doc = "The provisioning state of the linked service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
        ProvisioningAccount,
        Updating,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::ProvisioningAccount => serializer.serialize_unit_variant("ProvisioningState", 2u32, "ProvisioningAccount"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list linked storage accounts service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedStorageAccountsListResult {
    #[doc = "A list of linked storage accounts instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedStorageAccountsResource>,
}
impl azure_core::Continuable for LinkedStorageAccountsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LinkedStorageAccountsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Linked storage accounts properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedStorageAccountsProperties {
    #[doc = "Linked storage accounts type."]
    #[serde(rename = "dataSourceType", default, skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<linked_storage_accounts_properties::DataSourceType>,
    #[doc = "Linked storage accounts resources ids."]
    #[serde(rename = "storageAccountIds", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_account_ids: Vec<String>,
}
impl LinkedStorageAccountsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linked_storage_accounts_properties {
    use super::*;
    #[doc = "Linked storage accounts type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataSourceType {
        CustomLogs,
        AzureWatson,
        Query,
        Ingestion,
        Alerts,
    }
}
#[doc = "Linked storage accounts top level resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedStorageAccountsResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Linked storage accounts properties."]
    pub properties: LinkedStorageAccountsProperties,
}
impl LinkedStorageAccountsResource {
    pub fn new(properties: LinkedStorageAccountsProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "A management group that is connected to a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroup {
    #[doc = "Management group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupProperties>,
}
impl ManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Management group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupProperties {
    #[doc = "The number of servers connected to the management group."]
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
    #[doc = "Gets or sets a value indicating whether the management group is a gateway."]
    #[serde(rename = "isGateway", default, skip_serializing_if = "Option::is_none")]
    pub is_gateway: Option<bool>,
    #[doc = "The name of the management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The unique ID of the management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The datetime that the management group was created."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The last datetime that the management group received data."]
    #[serde(rename = "dataReceived", with = "azure_core::date::rfc3339::option")]
    pub data_received: Option<time::OffsetDateTime>,
    #[doc = "The version of System Center that is managing the management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The SKU of System Center that is managing the management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}
impl ManagementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name of a metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricName {
    #[doc = "The system name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the metric."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl MetricName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "JSON object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Object {}
impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operation of OperationalInsights resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft OperationsManagement."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list solution operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of solution operations supported by the OperationsManagement resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private link scope resource reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkScopedResource {
    #[doc = "The full resource Id of the private link scope resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The private link scope unique Identifier."]
    #[serde(rename = "scopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}
impl PrivateLinkScopedResource {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The network access type for operating on the Log Analytics Workspace. By default it is Enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublicNetworkAccessType")]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublicNetworkAccessType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublicNetworkAccessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublicNetworkAccessType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for PublicNetworkAccessType {
    fn default() -> Self {
        Self::Enabled
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Value object for saved search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedSearch {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The ETag of the saved search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Value object for saved search results."]
    pub properties: SavedSearchProperties,
}
impl SavedSearch {
    pub fn new(properties: SavedSearchProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            etag: None,
            properties,
        }
    }
}
#[doc = "Value object for saved search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedSearchProperties {
    #[doc = "The category of the saved search. This helps the user to find a saved search faster. "]
    pub category: String,
    #[doc = "Saved search display name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The query expression for the saved search."]
    pub query: String,
    #[doc = "The function alias if query serves as a function."]
    #[serde(rename = "functionAlias", default, skip_serializing_if = "Option::is_none")]
    pub function_alias: Option<String>,
    #[doc = "The optional function parameters if query serves as a function. Value should be in the following format: 'param-name1:type1 = default_value1, param-name2:type2 = default_value2'. For more examples and proper syntax please refer to https://docs.microsoft.com/en-us/azure/kusto/query/functions/user-defined-functions."]
    #[serde(rename = "functionParameters", default, skip_serializing_if = "Option::is_none")]
    pub function_parameters: Option<String>,
    #[doc = "The version number of the query language. The current version is 2 and is the default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "The tags attached to the saved search."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}
impl SavedSearchProperties {
    pub fn new(category: String, display_name: String, query: String) -> Self {
        Self {
            category,
            display_name,
            query,
            function_alias: None,
            function_parameters: None,
            version: None,
            tags: Vec::new(),
        }
    }
}
#[doc = "The saved search list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavedSearchesListResult {
    #[doc = "The array of result values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SavedSearch>,
}
impl SavedSearchesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The get schema operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchGetSchemaResponse {
    #[doc = "Metadata for search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SearchMetadata>,
    #[doc = "The array of result values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SearchSchemaValue>,
}
impl SearchGetSchemaResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchMetadata {
    #[doc = "The request id of the search."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The search result type."]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
    #[doc = "The total number of search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[doc = "The number of top search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    #[doc = "The id of the search results request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The core summaries."]
    #[serde(rename = "coreSummaries", default, skip_serializing_if = "Vec::is_empty")]
    pub core_summaries: Vec<CoreSummary>,
    #[doc = "The status of the search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The start time for the search."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The time of last update."]
    #[serde(rename = "lastUpdated", with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The ETag of the search results."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "How the results are sorted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<SearchSort>,
    #[doc = "The request time."]
    #[serde(rename = "requestTime", default, skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,
    #[doc = "The aggregated value field."]
    #[serde(rename = "aggregatedValueField", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_value_field: Option<String>,
    #[doc = "The aggregated grouping fields."]
    #[serde(rename = "aggregatedGroupingFields", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_grouping_fields: Option<String>,
    #[doc = "The sum of all aggregates returned in the result set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum: Option<i64>,
    #[doc = "The max of all aggregates returned in the result set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[doc = "Schema metadata for search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<SearchMetadataSchema>,
}
impl SearchMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema metadata for search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchMetadataSchema {
    #[doc = "The name of the metadata schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the metadata schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl SearchMetadataSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Value object for schema results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSchemaValue {
    #[doc = "The name of the schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of the schema."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The boolean that indicates the field is searchable as free text."]
    pub indexed: bool,
    #[doc = "The boolean that indicates whether or not the field is stored."]
    pub stored: bool,
    #[doc = "The boolean that indicates whether or not the field is a facet."]
    pub facet: bool,
    #[doc = "The array of workflows containing the field."]
    #[serde(rename = "ownerType", default, skip_serializing_if = "Vec::is_empty")]
    pub owner_type: Vec<String>,
}
impl SearchSchemaValue {
    pub fn new(indexed: bool, stored: bool, facet: bool) -> Self {
        Self {
            name: None,
            display_name: None,
            type_: None,
            indexed,
            stored,
            facet,
            owner_type: Vec::new(),
        }
    }
}
#[doc = "The sort parameters for search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchSort {
    #[doc = "The name of the field the search query is sorted on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The sort order of the search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<search_sort::Order>,
}
impl SearchSort {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod search_sort {
    use super::*;
    #[doc = "The sort order of the search."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Order")]
    pub enum Order {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Order {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Order {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Order {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Asc => serializer.serialize_unit_variant("Order", 0u32, "asc"),
                Self::Desc => serializer.serialize_unit_variant("Order", 1u32, "desc"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The shared keys for a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedKeys {
    #[doc = "The primary shared key of a workspace."]
    #[serde(rename = "primarySharedKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_shared_key: Option<String>,
    #[doc = "The secondary shared key of a workspace."]
    #[serde(rename = "secondarySharedKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_shared_key: Option<String>,
}
impl SharedKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a storage account connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[doc = "The Azure Resource Manager ID of the storage account resource."]
    pub id: String,
    #[doc = "The storage account key."]
    pub key: String,
}
impl StorageAccount {
    pub fn new(id: String, key: String) -> Self {
        Self { id, key }
    }
}
#[doc = "The top level storage insight resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageInsight {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Storage insight properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageInsightProperties>,
    #[doc = "The ETag of the storage insight."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl StorageInsight {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list storage insights operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageInsightListResult {
    #[doc = "A list of storage insight items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageInsight>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for StorageInsightListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl StorageInsightListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage insight properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInsightProperties {
    #[doc = "The names of the blob containers that the workspace should read"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<String>,
    #[doc = "The names of the Azure tables that the workspace should read"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,
    #[doc = "Describes a storage account connection."]
    #[serde(rename = "storageAccount")]
    pub storage_account: StorageAccount,
    #[doc = "The status of the storage insight."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StorageInsightStatus>,
}
impl StorageInsightProperties {
    pub fn new(storage_account: StorageAccount) -> Self {
        Self {
            containers: Vec::new(),
            tables: Vec::new(),
            storage_account,
            status: None,
        }
    }
}
#[doc = "The status of the storage insight."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInsightStatus {
    #[doc = "The state of the storage insight connection to the workspace"]
    pub state: storage_insight_status::State,
    #[doc = "Description of the state of the storage insight."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl StorageInsightStatus {
    pub fn new(state: storage_insight_status::State) -> Self {
        Self { state, description: None }
    }
}
pub mod storage_insight_status {
    use super::*;
    #[doc = "The state of the storage insight connection to the workspace"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        #[serde(rename = "OK")]
        Ok,
        #[serde(rename = "ERROR")]
        Error,
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
                Self::Ok => serializer.serialize_unit_variant("State", 0u32, "OK"),
                Self::Error => serializer.serialize_unit_variant("State", 1u32, "ERROR"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workspace data table definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Table {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Table properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TableProperties>,
}
impl Table {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Table properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableProperties {
    #[doc = "The data table data retention in days, between 30 and 730. Setting this property to null will default to the workspace retention."]
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
}
impl TableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list tables operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TablesListResult {
    #[doc = "A list of data tables."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Table>,
}
impl azure_core::Continuable for TablesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl TablesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A tag of a saved search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    #[doc = "The tag name."]
    pub name: String,
    #[doc = "The tag value."]
    pub value: String,
}
impl Tag {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
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
#[doc = "A metric describing the usage of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageMetric {
    #[doc = "The name of a metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The units used for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The current value of the metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "The quota limit for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "The time that the metric's value will reset."]
    #[serde(rename = "nextResetTime", with = "azure_core::date::rfc3339::option")]
    pub next_reset_time: Option<time::OffsetDateTime>,
    #[doc = "The quota period that determines the length of time between value resets."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
}
impl UsageMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The top level Workspace resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Workspace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[doc = "The ETag of the workspace."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl Workspace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            e_tag: None,
        }
    }
}
#[doc = "The daily volume cap for ingestion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceCapping {
    #[doc = "The workspace daily quota for ingestion. -1 means unlimited."]
    #[serde(rename = "dailyQuotaGb", default, skip_serializing_if = "Option::is_none")]
    pub daily_quota_gb: Option<f64>,
    #[doc = "The time when the quota will be rest."]
    #[serde(rename = "quotaNextResetTime", default, skip_serializing_if = "Option::is_none")]
    pub quota_next_reset_time: Option<String>,
    #[doc = "The status of data ingestion for this workspace."]
    #[serde(rename = "dataIngestionStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_ingestion_status: Option<workspace_capping::DataIngestionStatus>,
}
impl WorkspaceCapping {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_capping {
    use super::*;
    #[doc = "The status of data ingestion for this workspace."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataIngestionStatus")]
    pub enum DataIngestionStatus {
        RespectQuota,
        ForceOn,
        ForceOff,
        OverQuota,
        SubscriptionSuspended,
        ApproachingQuota,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataIngestionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataIngestionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataIngestionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RespectQuota => serializer.serialize_unit_variant("DataIngestionStatus", 0u32, "RespectQuota"),
                Self::ForceOn => serializer.serialize_unit_variant("DataIngestionStatus", 1u32, "ForceOn"),
                Self::ForceOff => serializer.serialize_unit_variant("DataIngestionStatus", 2u32, "ForceOff"),
                Self::OverQuota => serializer.serialize_unit_variant("DataIngestionStatus", 3u32, "OverQuota"),
                Self::SubscriptionSuspended => serializer.serialize_unit_variant("DataIngestionStatus", 4u32, "SubscriptionSuspended"),
                Self::ApproachingQuota => serializer.serialize_unit_variant("DataIngestionStatus", 5u32, "ApproachingQuota"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list workspace management groups operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListManagementGroupsResult {
    #[doc = "Gets or sets a list of management groups attached to the workspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementGroup>,
}
impl azure_core::Continuable for WorkspaceListManagementGroupsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkspaceListManagementGroupsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list workspaces operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "A list of workspaces."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
}
impl azure_core::Continuable for WorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list workspace usages operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListUsagesResult {
    #[doc = "Gets or sets a list of usage metrics for a workspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageMetric>,
}
impl azure_core::Continuable for WorkspaceListUsagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkspaceListUsagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The top level Workspace resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePatch {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "Workspace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[doc = "Resource tags. Optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WorkspacePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[doc = "The provisioning state of the workspace."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
    #[doc = "This is a read-only property. Represents the ID associated with the workspace."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The SKU (tier) of a workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<WorkspaceSku>,
    #[doc = "The workspace data retention in days. -1 means Unlimited retention for the Unlimited Sku. 730 days is the maximum allowed for all other Skus. "]
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
    #[doc = "The daily volume cap for ingestion."]
    #[serde(rename = "workspaceCapping", default, skip_serializing_if = "Option::is_none")]
    pub workspace_capping: Option<WorkspaceCapping>,
    #[doc = "The network access type for operating on the Log Analytics Workspace. By default it is Enabled"]
    #[serde(rename = "publicNetworkAccessForIngestion", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_ingestion: Option<PublicNetworkAccessType>,
    #[doc = "The network access type for operating on the Log Analytics Workspace. By default it is Enabled"]
    #[serde(rename = "publicNetworkAccessForQuery", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_query: Option<PublicNetworkAccessType>,
    #[doc = "List of linked private link scope resources."]
    #[serde(rename = "privateLinkScopedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_scoped_resources: Vec<PrivateLinkScopedResource>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "The provisioning state of the workspace."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        Canceled,
        Deleting,
        ProvisioningAccount,
        Updating,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::ProvisioningAccount => serializer.serialize_unit_variant("ProvisioningState", 5u32, "ProvisioningAccount"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the body of a purge request for an App Insights Workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeBody {
    #[doc = "Table from which to purge data."]
    pub table: String,
    #[doc = "The set of columns and filters (queries) to run over them to purge the resulting data."]
    pub filters: Vec<WorkspacePurgeBodyFilters>,
}
impl WorkspacePurgeBody {
    pub fn new(table: String, filters: Vec<WorkspacePurgeBodyFilters>) -> Self {
        Self { table, filters }
    }
}
#[doc = "User-defined filters to return data which will be purged from the table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePurgeBodyFilters {
    #[doc = "The column of the table over which the given query should run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[doc = "A query operator to evaluate over the provided column and value(s). Supported operators are ==, =~, in, in~, >, >=, <, <=, between, and have the same behavior as they would in a KQL query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[doc = "the value for the operator to function over. This can be a number (e.g., > 100), a string (timestamp >= '2017-09-01') or array of values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "When filtering over custom dimensions, this key will be used as the name of the custom dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl WorkspacePurgeBodyFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response containing operationId for a specific purge action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeResponse {
    #[doc = "Id to use when querying for status for a particular purge operation."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
impl WorkspacePurgeResponse {
    pub fn new(operation_id: String) -> Self {
        Self { operation_id }
    }
}
#[doc = "Response containing status for a specific purge operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeStatusResponse {
    #[doc = "Status of the operation represented by the requested Id."]
    pub status: workspace_purge_status_response::Status,
}
impl WorkspacePurgeStatusResponse {
    pub fn new(status: workspace_purge_status_response::Status) -> Self {
        Self { status }
    }
}
pub mod workspace_purge_status_response {
    use super::*;
    #[doc = "Status of the operation represented by the requested Id."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "completed")]
        Completed,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "pending"),
                Self::Completed => serializer.serialize_unit_variant("Status", 1u32, "completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The SKU (tier) of a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSku {
    #[doc = "The name of the SKU."]
    pub name: workspace_sku::Name,
    #[doc = "The capacity reservation level for this workspace, when CapacityReservation sku is selected."]
    #[serde(rename = "capacityReservationLevel", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_level: Option<i32>,
    #[doc = "The maximum capacity reservation level available for this workspace, when CapacityReservation sku is selected."]
    #[serde(rename = "maxCapacityReservationLevel", default, skip_serializing_if = "Option::is_none")]
    pub max_capacity_reservation_level: Option<i32>,
    #[doc = "The last time when the sku was updated."]
    #[serde(rename = "lastSkuUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_sku_update: Option<String>,
}
impl WorkspaceSku {
    pub fn new(name: workspace_sku::Name) -> Self {
        Self {
            name,
            capacity_reservation_level: None,
            max_capacity_reservation_level: None,
            last_sku_update: None,
        }
    }
}
pub mod workspace_sku {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Free,
        Standard,
        Premium,
        PerNode,
        #[serde(rename = "PerGB2018")]
        PerGb2018,
        Standalone,
        CapacityReservation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Free => serializer.serialize_unit_variant("Name", 0u32, "Free"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Name", 2u32, "Premium"),
                Self::PerNode => serializer.serialize_unit_variant("Name", 3u32, "PerNode"),
                Self::PerGb2018 => serializer.serialize_unit_variant("Name", 4u32, "PerGB2018"),
                Self::Standalone => serializer.serialize_unit_variant("Name", 5u32, "Standalone"),
                Self::CapacityReservation => serializer.serialize_unit_variant("Name", 6u32, "CapacityReservation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The key vault properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "The Key Vault uri which holds they key associated with the Log Analytics cluster."]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[doc = "The name of the key associated with the Log Analytics cluster."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "The version of the key associated with the Log Analytics cluster."]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
