#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The complex type of the extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceManagerCommonTypesExtendedLocation {
    #[doc = "The name of the extended location."]
    pub name: String,
    #[doc = "The supported ExtendedLocation types."]
    #[serde(rename = "type")]
    pub type_: AzureResourceManagerCommonTypesExtendedLocationType,
}
impl AzureResourceManagerCommonTypesExtendedLocation {
    pub fn new(name: String, type_: AzureResourceManagerCommonTypesExtendedLocationType) -> Self {
        Self { name, type_ }
    }
}
#[doc = "The supported ExtendedLocation types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureResourceManagerCommonTypesExtendedLocationType")]
pub enum AzureResourceManagerCommonTypesExtendedLocationType {
    EdgeZone,
    CustomLocation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureResourceManagerCommonTypesExtendedLocationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureResourceManagerCommonTypesExtendedLocationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureResourceManagerCommonTypesExtendedLocationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EdgeZone => serializer.serialize_unit_variant("AzureResourceManagerCommonTypesExtendedLocationType", 0u32, "EdgeZone"),
            Self::CustomLocation => {
                serializer.serialize_unit_variant("AzureResourceManagerCommonTypesExtendedLocationType", 1u32, "CustomLocation")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An Azure Monitor Workspace definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorWorkspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties that need to be specified to create a new workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMonitorWorkspaceProperties>,
    #[doc = "Resource entity tag (ETag)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl AzureMonitorWorkspace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
        }
    }
}
#[doc = "The response of a AzureMonitorWorkspace list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorWorkspaceListResult {
    #[doc = "The AzureMonitorWorkspace items on this page"]
    pub value: Vec<AzureMonitorWorkspace>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureMonitorWorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureMonitorWorkspaceListResult {
    pub fn new(value: Vec<AzureMonitorWorkspace>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Azure Monitor Workspace Logs Api configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorWorkspaceLogsApiConfig {
    #[doc = "Data collection endpoint ingestion url."]
    #[serde(rename = "dataCollectionEndpointUrl")]
    pub data_collection_endpoint_url: String,
    #[doc = "Stream name in destination. Azure Monitor stream is related to the destination table."]
    pub stream: String,
    #[doc = "Data Collection Rule (DCR) immutable id."]
    #[serde(rename = "dataCollectionRule")]
    pub data_collection_rule: String,
    #[doc = "Schema map for azure monitor for logs."]
    pub schema: SchemaMap,
}
impl AzureMonitorWorkspaceLogsApiConfig {
    pub fn new(data_collection_endpoint_url: String, stream: String, data_collection_rule: String, schema: SchemaMap) -> Self {
        Self {
            data_collection_endpoint_url,
            stream,
            data_collection_rule,
            schema,
        }
    }
}
#[doc = "Azure Monitor Workspace Logs specific configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorWorkspaceLogsExporter {
    #[doc = "Azure Monitor Workspace Logs Api configurations."]
    pub api: AzureMonitorWorkspaceLogsApiConfig,
    #[doc = "Concurrent publishing configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<ConcurrencyConfiguration>,
    #[doc = "Cache configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<CacheConfiguration>,
}
impl AzureMonitorWorkspaceLogsExporter {
    pub fn new(api: AzureMonitorWorkspaceLogsApiConfig) -> Self {
        Self {
            api,
            concurrency: None,
            cache: None,
        }
    }
}
#[doc = "Properties that need to be specified to create a new workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorWorkspaceProperties {
    #[doc = "The immutable ID of the Azure Monitor workspace. This property is read-only."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Information about metrics for the workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    #[doc = "The provisioning state of a resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Settings for data ingestion"]
    #[serde(rename = "defaultIngestionSettings", default, skip_serializing_if = "Option::is_none")]
    pub default_ingestion_settings: Option<IngestionSettings>,
    #[doc = "List of private endpoint connections."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "State of the public network access."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
}
impl AzureMonitorWorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the AzureMonitorWorkspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorWorkspaceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the AzureMonitorWorkspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMonitorWorkspaceUpdateProperties>,
}
impl AzureMonitorWorkspaceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the AzureMonitorWorkspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorWorkspaceUpdateProperties {
    #[doc = "Information about metrics for the workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
}
impl AzureMonitorWorkspaceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Batch processor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchProcessor {
    #[doc = "Size of the batch."]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[doc = "Timeout in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}
impl BatchProcessor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cache configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheConfiguration {
    #[doc = "Max storage usage in megabytes."]
    #[serde(rename = "maxStorageUsage", default, skip_serializing_if = "Option::is_none")]
    pub max_storage_usage: Option<i32>,
    #[doc = "Retention period in minutes."]
    #[serde(rename = "retentionPeriod", default, skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
}
impl CacheConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Concurrent publishing configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConcurrencyConfiguration {
    #[doc = "Number of parallel workers processing the log queues."]
    #[serde(rename = "workerCount", default, skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<i32>,
    #[doc = "Size of the queue for log batches."]
    #[serde(rename = "batchQueueSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_queue_size: Option<i32>,
}
impl ConcurrencyConfiguration {
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
#[doc = "Exporter Info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Exporter {
    #[doc = "The exporter type."]
    #[serde(rename = "type")]
    pub type_: ExporterType,
    #[doc = "The name of exporter."]
    pub name: String,
    #[doc = "Azure Monitor Workspace Logs specific configurations."]
    #[serde(rename = "azureMonitorWorkspaceLogs", default, skip_serializing_if = "Option::is_none")]
    pub azure_monitor_workspace_logs: Option<AzureMonitorWorkspaceLogsExporter>,
    #[doc = "Base exporter using TCP as transport protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<TcpExporter>,
}
impl Exporter {
    pub fn new(type_: ExporterType, name: String) -> Self {
        Self {
            type_,
            name,
            azure_monitor_workspace_logs: None,
            tcp: None,
        }
    }
}
#[doc = "The exporter type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExporterType")]
pub enum ExporterType {
    AzureMonitorWorkspaceLogs,
    PipelineGroup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExporterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExporterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExporterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureMonitorWorkspaceLogs => serializer.serialize_unit_variant("ExporterType", 0u32, "AzureMonitorWorkspaceLogs"),
            Self::PipelineGroup => serializer.serialize_unit_variant("ExporterType", 1u32, "PipelineGroup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The mode of the external networking."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExternalNetworkingMode")]
pub enum ExternalNetworkingMode {
    LoadBalancerOnly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExternalNetworkingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExternalNetworkingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExternalNetworkingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LoadBalancerOnly => serializer.serialize_unit_variant("ExternalNetworkingMode", 0u32, "LoadBalancerOnly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Settings for data ingestion"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngestionSettings {
    #[doc = "The Azure resource Id of the default data collection rule for this workspace."]
    #[serde(rename = "dataCollectionRuleResourceId", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_rule_resource_id: Option<String>,
    #[doc = "The Azure resource Id of the default data collection endpoint for this workspace."]
    #[serde(rename = "dataCollectionEndpointResourceId", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_endpoint_resource_id: Option<String>,
}
impl IngestionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about metrics for the workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metrics {
    #[doc = "The Prometheus query endpoint for the workspace"]
    #[serde(rename = "prometheusQueryEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub prometheus_query_endpoint: Option<String>,
    #[doc = "An internal identifier for the metrics container. Only to be used by the system"]
    #[serde(rename = "internalId", default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
}
impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Networking configuration for the pipeline group instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkingConfiguration {
    #[doc = "The mode of the external networking."]
    #[serde(rename = "externalNetworkingMode")]
    pub external_networking_mode: ExternalNetworkingMode,
    #[doc = "The address exposed on the cluster. Example: azuremonitorpipeline.contoso.com."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "Networking routes configuration."]
    pub routes: Vec<NetworkingRoute>,
}
impl NetworkingConfiguration {
    pub fn new(external_networking_mode: ExternalNetworkingMode, routes: Vec<NetworkingRoute>) -> Self {
        Self {
            external_networking_mode,
            host: None,
            routes,
        }
    }
}
#[doc = "Networking route configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkingRoute {
    #[doc = "The name of the previously defined receiver."]
    pub receiver: String,
    #[doc = "The port that will be configured externally. If not specified, it will use the port from the receiver definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Route path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Route subdomain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
}
impl NetworkingRoute {
    pub fn new(receiver: String) -> Self {
        Self {
            receiver,
            port: None,
            path: None,
            subdomain: None,
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
#[doc = "OTLP Receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OtlpReceiver {
    #[doc = "OTLP GRPC endpoint definition. Example: 0.0.0.0:<port>."]
    pub endpoint: String,
}
impl OtlpReceiver {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}
#[doc = "Persistence options to all pipelines in the instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersistenceConfigurations {
    #[doc = "The name of the mounted persistent volume."]
    #[serde(rename = "persistentVolumeName")]
    pub persistent_volume_name: String,
}
impl PersistenceConfigurations {
    pub fn new(persistent_volume_name: String) -> Self {
        Self { persistent_volume_name }
    }
}
#[doc = "Pipeline Info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    #[doc = "Name of the pipeline."]
    pub name: String,
    #[doc = "The pipeline type."]
    #[serde(rename = "type")]
    pub type_: PipelineType,
    #[doc = "Reference to receivers configured for the pipeline."]
    pub receivers: Vec<String>,
    #[doc = "Reference to processors configured for the pipeline."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub processors: Vec<String>,
    #[doc = "Reference to exporters configured for the pipeline."]
    pub exporters: Vec<String>,
}
impl Pipeline {
    pub fn new(name: String, type_: PipelineType, receivers: Vec<String>, exporters: Vec<String>) -> Self {
        Self {
            name,
            type_,
            receivers,
            processors: Vec::new(),
            exporters,
        }
    }
}
#[doc = "A pipeline group definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties that need to be specified to create a new pipeline group instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PipelineGroupProperties>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<AzureResourceManagerCommonTypesExtendedLocation>,
}
impl PipelineGroup {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location: None,
        }
    }
}
#[doc = "The response of a PipelineGroup list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineGroupListResult {
    #[doc = "The PipelineGroup items on this page"]
    pub value: Vec<PipelineGroup>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PipelineGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PipelineGroupListResult {
    pub fn new(value: Vec<PipelineGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties that need to be specified to create a new pipeline group instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineGroupProperties {
    #[doc = "Defines the amount of replicas of the pipeline group instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[doc = "The receivers specified for a pipeline group instance."]
    pub receivers: Vec<Receiver>,
    #[doc = "The processors specified for a pipeline group instance."]
    pub processors: Vec<Processor>,
    #[doc = "The exporters specified for a pipeline group instance."]
    pub exporters: Vec<Exporter>,
    #[doc = "Service Info."]
    pub service: Service,
    #[doc = "Networking configurations for the pipeline group instance."]
    #[serde(
        rename = "networkingConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub networking_configurations: Vec<NetworkingConfiguration>,
    #[doc = "The provisioning state of a resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl PipelineGroupProperties {
    pub fn new(receivers: Vec<Receiver>, processors: Vec<Processor>, exporters: Vec<Exporter>, service: Service) -> Self {
        Self {
            replicas: None,
            receivers,
            processors,
            exporters,
            service,
            networking_configurations: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "The type used for update operations of the PipelineGroup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineGroupUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the PipelineGroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PipelineGroupUpdateProperties>,
}
impl PipelineGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the PipelineGroup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineGroupUpdateProperties {
    #[doc = "Defines the amount of replicas of the pipeline group instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[doc = "The receivers specified for a pipeline group instance."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub receivers: Vec<Receiver>,
    #[doc = "The processors specified for a pipeline group instance."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub processors: Vec<Processor>,
    #[doc = "The exporters specified for a pipeline group instance."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exporters: Vec<Exporter>,
    #[doc = "Service Info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[doc = "Networking configurations for the pipeline group instance."]
    #[serde(
        rename = "networkingConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub networking_configurations: Vec<NetworkingConfiguration>,
}
impl PipelineGroupUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The pipeline type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PipelineType")]
pub enum PipelineType {
    #[serde(rename = "logs")]
    Logs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PipelineType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PipelineType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PipelineType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Logs => serializer.serialize_unit_variant("PipelineType", 0u32, "logs"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The group ids for the private endpoint resource."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "The private endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            group_ids: Vec::new(),
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Processor Info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Processor {
    #[doc = "The processor type."]
    #[serde(rename = "type")]
    pub type_: ProcessorType,
    #[doc = "The name of processor."]
    pub name: String,
    #[doc = "Batch processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch: Option<BatchProcessor>,
}
impl Processor {
    pub fn new(type_: ProcessorType, name: String) -> Self {
        Self { type_, name, batch: None }
    }
}
#[doc = "The processor type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProcessorType")]
pub enum ProcessorType {
    Batch,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProcessorType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProcessorType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProcessorType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Batch => serializer.serialize_unit_variant("ProcessorType", 0u32, "Batch"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The provisioning state of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Creating,
    Deleting,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "State of the public network access."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublicNetworkAccess")]
pub enum PublicNetworkAccess {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublicNetworkAccess {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublicNetworkAccess {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublicNetworkAccess {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Receiver Info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Receiver {
    #[doc = "The receiver type."]
    #[serde(rename = "type")]
    pub type_: ReceiverType,
    #[doc = "The name of receiver."]
    pub name: String,
    #[doc = "Base receiver using TCP as transport protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syslog: Option<SyslogReceiver>,
    #[doc = "OTLP Receiver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otlp: Option<OtlpReceiver>,
    #[doc = "Receiver using UDP as transport protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udp: Option<UdpReceiver>,
}
impl Receiver {
    pub fn new(type_: ReceiverType, name: String) -> Self {
        Self {
            type_,
            name,
            syslog: None,
            otlp: None,
            udp: None,
        }
    }
}
#[doc = "The receiver type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReceiverType")]
pub enum ReceiverType {
    Syslog,
    Ama,
    PipelineGroup,
    #[serde(rename = "OTLP")]
    Otlp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReceiverType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReceiverType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReceiverType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Syslog => serializer.serialize_unit_variant("ReceiverType", 0u32, "Syslog"),
            Self::Ama => serializer.serialize_unit_variant("ReceiverType", 1u32, "Ama"),
            Self::PipelineGroup => serializer.serialize_unit_variant("ReceiverType", 2u32, "PipelineGroup"),
            Self::Otlp => serializer.serialize_unit_variant("ReceiverType", 3u32, "OTLP"),
            Self::Udp => serializer.serialize_unit_variant("ReceiverType", 4u32, "UDP"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Record map for schema in azure monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordMap {
    #[doc = "Record Map Key."]
    pub from: String,
    #[doc = "Record Map Value."]
    pub to: String,
}
impl RecordMap {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "Resource map for schema in azure monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMap {
    #[doc = "Resource Map Key."]
    pub from: String,
    #[doc = "Resource Map Value."]
    pub to: String,
}
impl ResourceMap {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}
#[doc = "Schema map for azure monitor for logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaMap {
    #[doc = "Record Map."]
    #[serde(rename = "recordMap")]
    pub record_map: Vec<RecordMap>,
    #[doc = "Resource Map captures information about the entity for which telemetry is recorded. For example, metrics exposed by a Kubernetes container can be linked to a resource that specifies the cluster, namespace, pod, and container name.Resource may capture an entire hierarchy of entity identification. It may describe the host in the cloud and specific container or an application running in the process."]
    #[serde(
        rename = "resourceMap",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_map: Vec<ResourceMap>,
    #[doc = "A scope map is a logical unit of the application code with which the emitted telemetry can be associated."]
    #[serde(
        rename = "scopeMap",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scope_map: Vec<ScopeMap>,
}
impl SchemaMap {
    pub fn new(record_map: Vec<RecordMap>) -> Self {
        Self {
            record_map,
            resource_map: Vec::new(),
            scope_map: Vec::new(),
        }
    }
}
#[doc = "Scope map for schema in azure monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeMap {
    #[doc = "Scope Map Key."]
    pub from: String,
    #[doc = "Scope Map Value."]
    pub to: String,
}
impl ScopeMap {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}
#[doc = "Service Info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[doc = "Pipelines belonging to a given pipeline group."]
    pub pipelines: Vec<Pipeline>,
    #[doc = "Persistence options to all pipelines in the instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistence: Option<PersistenceConfigurations>,
}
impl Service {
    pub fn new(pipelines: Vec<Pipeline>) -> Self {
        Self {
            pipelines,
            persistence: None,
        }
    }
}
#[doc = "Base receiver using TCP as transport protocol."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyslogReceiver {
    #[doc = "Syslog receiver endpoint definition. Example: 0.0.0.0:<port>."]
    pub endpoint: String,
    #[doc = "Protocol to parse syslog messages. Default rfc3164"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<syslog_receiver::Protocol>,
}
impl SyslogReceiver {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint, protocol: None }
    }
}
pub mod syslog_receiver {
    use super::*;
    #[doc = "Protocol to parse syslog messages. Default rfc3164"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "rfc3164")]
        Rfc3164,
        #[serde(rename = "rfc5424")]
        Rfc5424,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Rfc3164 => serializer.serialize_unit_variant("Protocol", 0u32, "rfc3164"),
                Self::Rfc5424 => serializer.serialize_unit_variant("Protocol", 1u32, "rfc5424"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Protocol {
        fn default() -> Self {
            Self::Rfc3164
        }
    }
}
#[doc = "Base exporter using TCP as transport protocol."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TcpExporter {
    #[doc = "TCP url to export."]
    pub url: String,
}
impl TcpExporter {
    pub fn new(url: String) -> Self {
        Self { url }
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
#[doc = "Receiver using UDP as transport protocol."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UdpReceiver {
    #[doc = "TCP endpoint definition. Example: 0.0.0.0:<port>."]
    pub endpoint: String,
    #[doc = "The encoding of the stream being received."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<udp_receiver::Encoding>,
    #[doc = "Max read queue length."]
    #[serde(rename = "readQueueLength", default, skip_serializing_if = "Option::is_none")]
    pub read_queue_length: Option<i32>,
}
impl UdpReceiver {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            encoding: None,
            read_queue_length: None,
        }
    }
}
pub mod udp_receiver {
    use super::*;
    #[doc = "The encoding of the stream being received."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Encoding")]
    pub enum Encoding {
        #[serde(rename = "nop")]
        Nop,
        #[serde(rename = "utf-8")]
        Utf8,
        #[serde(rename = "utf-16le")]
        Utf16le,
        #[serde(rename = "utf-16be")]
        Utf16be,
        #[serde(rename = "ascii")]
        Ascii,
        #[serde(rename = "big5")]
        Big5,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Encoding {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Encoding {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Encoding {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Nop => serializer.serialize_unit_variant("Encoding", 0u32, "nop"),
                Self::Utf8 => serializer.serialize_unit_variant("Encoding", 1u32, "utf-8"),
                Self::Utf16le => serializer.serialize_unit_variant("Encoding", 2u32, "utf-16le"),
                Self::Utf16be => serializer.serialize_unit_variant("Encoding", 3u32, "utf-16be"),
                Self::Ascii => serializer.serialize_unit_variant("Encoding", 4u32, "ascii"),
                Self::Big5 => serializer.serialize_unit_variant("Encoding", 5u32, "big5"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Encoding {
        fn default() -> Self {
            Self::Nop
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
