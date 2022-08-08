#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents an instance of an Analysis Services resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServer {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of Analysis Services resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnalysisServicesServerProperties>,
}
impl AnalysisServicesServer {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "An object that represents a set of mutable Analysis Services resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisServicesServerMutableProperties {
    #[doc = "An array of administrator user identities."]
    #[serde(rename = "asAdministrators", default, skip_serializing_if = "Option::is_none")]
    pub as_administrators: Option<ServerAdministrators>,
    #[doc = "The SAS container URI to the backup container."]
    #[serde(rename = "backupBlobContainerUri", default, skip_serializing_if = "Option::is_none")]
    pub backup_blob_container_uri: Option<String>,
    #[doc = "The gateway details."]
    #[serde(rename = "gatewayDetails", default, skip_serializing_if = "Option::is_none")]
    pub gateway_details: Option<GatewayDetails>,
    #[doc = "An array of firewall rules."]
    #[serde(rename = "ipV4FirewallSettings", default, skip_serializing_if = "Option::is_none")]
    pub ip_v4_firewall_settings: Option<IPv4FirewallSettings>,
    #[doc = "How the read-write server's participation in the query pool is controlled.<br/>It can have the following values: <ul><li>readOnly - indicates that the read-write server is intended not to participate in query operations</li><li>all - indicates that the read-write server can participate in query operations</li></ul>Specifying readOnly when capacity is 1 results in error."]
    #[serde(rename = "querypoolConnectionMode", default, skip_serializing_if = "Option::is_none")]
    pub querypool_connection_mode: Option<analysis_services_server_mutable_properties::QuerypoolConnectionMode>,
    #[doc = "The managed mode of the server (0 = not managed, 1 = managed)."]
    #[serde(rename = "managedMode", default, skip_serializing_if = "Option::is_none")]
    pub managed_mode: Option<analysis_services_server_mutable_properties::ManagedMode>,
    #[doc = "The server monitor mode for AS server"]
    #[serde(rename = "serverMonitorMode", default, skip_serializing_if = "Option::is_none")]
    pub server_monitor_mode: Option<analysis_services_server_mutable_properties::ServerMonitorMode>,
}
impl AnalysisServicesServerMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod analysis_services_server_mutable_properties {
    use super::*;
    #[doc = "How the read-write server's participation in the query pool is controlled.<br/>It can have the following values: <ul><li>readOnly - indicates that the read-write server is intended not to participate in query operations</li><li>all - indicates that the read-write server can participate in query operations</li></ul>Specifying readOnly when capacity is 1 results in error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QuerypoolConnectionMode {
        All,
        ReadOnly,
    }
    impl Default for QuerypoolConnectionMode {
        fn default() -> Self {
            Self::All
        }
    }
    #[doc = "The managed mode of the server (0 = not managed, 1 = managed)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ManagedMode {}
    #[doc = "The server monitor mode for AS server"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServerMonitorMode {}
}
#[doc = "Properties of Analysis Services resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisServicesServerProperties {
    #[serde(flatten)]
    pub analysis_services_server_mutable_properties: AnalysisServicesServerMutableProperties,
    #[doc = "The current state of Analysis Services resource. The state is to indicate more states outside of resource provisioning."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<analysis_services_server_properties::State>,
    #[doc = "The current deployment state of Analysis Services resource. The provisioningState is to indicate states for resource provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<analysis_services_server_properties::ProvisioningState>,
    #[doc = "The full name of the Analysis Services resource."]
    #[serde(rename = "serverFullName", default, skip_serializing_if = "Option::is_none")]
    pub server_full_name: Option<String>,
    #[doc = "Represents the SKU name and Azure pricing tier for Analysis Services resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
impl AnalysisServicesServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod analysis_services_server_properties {
    use super::*;
    #[doc = "The current state of Analysis Services resource. The state is to indicate more states outside of resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
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
                Self::Deleting => serializer.serialize_unit_variant("State", 0u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 2u32, "Failed"),
                Self::Paused => serializer.serialize_unit_variant("State", 3u32, "Paused"),
                Self::Suspended => serializer.serialize_unit_variant("State", 4u32, "Suspended"),
                Self::Provisioning => serializer.serialize_unit_variant("State", 5u32, "Provisioning"),
                Self::Updating => serializer.serialize_unit_variant("State", 6u32, "Updating"),
                Self::Suspending => serializer.serialize_unit_variant("State", 7u32, "Suspending"),
                Self::Pausing => serializer.serialize_unit_variant("State", 8u32, "Pausing"),
                Self::Resuming => serializer.serialize_unit_variant("State", 9u32, "Resuming"),
                Self::Preparing => serializer.serialize_unit_variant("State", 10u32, "Preparing"),
                Self::Scaling => serializer.serialize_unit_variant("State", 11u32, "Scaling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current deployment state of Analysis Services resource. The provisioningState is to indicate states for resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Paused => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Paused"),
                Self::Suspended => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Suspended"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::Suspending => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Suspending"),
                Self::Pausing => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Pausing"),
                Self::Resuming => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Resuming"),
                Self::Preparing => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Preparing"),
                Self::Scaling => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Scaling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Provision request specification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisServicesServerUpdateParameters {
    #[doc = "Represents the SKU name and Azure pricing tier for Analysis Services resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[doc = "Key-value pairs of additional provisioning properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An object that represents a set of mutable Analysis Services resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnalysisServicesServerMutableProperties>,
}
impl AnalysisServicesServerUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An array of Analysis Services resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServers {
    #[doc = "An array of Analysis Services resources."]
    pub value: Vec<AnalysisServicesServer>,
}
impl azure_core::Continuable for AnalysisServicesServers {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AnalysisServicesServers {
    pub fn new(value: Vec<AnalysisServicesServer>) -> Self {
        Self { value }
    }
}
#[doc = "Details of server name request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckServerNameAvailabilityParameters {
    #[doc = "Name for checking availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of azure analysis services."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckServerNameAvailabilityParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The checking result of server name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckServerNameAvailabilityResult {
    #[doc = "Indicator of available of the server name."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason of unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed message of the request unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckServerNameAvailabilityResult {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
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
pub mod error_response {
    use super::*;
    #[doc = "The error object"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "The error details."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<ErrorDetail>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The gateway details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayDetails {
    #[doc = "Gateway resource to be associated with the server."]
    #[serde(rename = "gatewayResourceId", default, skip_serializing_if = "Option::is_none")]
    pub gateway_resource_id: Option<String>,
    #[doc = "Gateway object id from in the DMTS cluster for the gateway resource."]
    #[serde(rename = "gatewayObjectId", default, skip_serializing_if = "Option::is_none")]
    pub gateway_object_id: Option<String>,
    #[doc = "Uri of the DMTS cluster."]
    #[serde(rename = "dmtsClusterUri", default, skip_serializing_if = "Option::is_none")]
    pub dmts_cluster_uri: Option<String>,
}
impl GatewayDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detail of gateway errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayError {
    #[doc = "Error code of list gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message of list gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl GatewayError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of gateway is error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayListStatusError {
    #[doc = "Detail of gateway errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<GatewayError>,
}
impl GatewayListStatusError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of gateway is live."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayListStatusLive {
    #[doc = "Live message of list gateway. Status: 0 - Live"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<gateway_list_status_live::Status>,
}
impl GatewayListStatusLive {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod gateway_list_status_live {
    use super::*;
    #[doc = "Live message of list gateway. Status: 0 - Live"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {}
}
#[doc = "The detail of firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IPv4FirewallRule {
    #[doc = "The rule name."]
    #[serde(rename = "firewallRuleName", default, skip_serializing_if = "Option::is_none")]
    pub firewall_rule_name: Option<String>,
    #[doc = "The start range of IPv4."]
    #[serde(rename = "rangeStart", default, skip_serializing_if = "Option::is_none")]
    pub range_start: Option<String>,
    #[doc = "The end range of IPv4."]
    #[serde(rename = "rangeEnd", default, skip_serializing_if = "Option::is_none")]
    pub range_end: Option<String>,
}
impl IPv4FirewallRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An array of firewall rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IPv4FirewallSettings {
    #[doc = "An array of firewall rules."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<IPv4FirewallRule>,
    #[doc = "The indicator of enabling PBI service."]
    #[serde(rename = "enablePowerBIService", default, skip_serializing_if = "Option::is_none")]
    pub enable_power_bi_service: Option<bool>,
}
impl IPv4FirewallSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Consumption REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The origin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Consumption."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: UsageDetail, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation object."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of listing consumption operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of analysis services operations supported by the Microsoft.AnalysisServices resource provider."]
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
    #[doc = "Describes the format of Error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an instance of an Analysis Services resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "An identifier that represents the Analysis Services resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the Analysis Services resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the Analysis Services resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Location of the Analysis Services resource."]
    pub location: String,
    #[doc = "Represents the SKU name and Azure pricing tier for Analysis Services resource."]
    pub sku: ResourceSku,
    #[doc = "Key-value pairs of additional resource provisioning properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String, sku: ResourceSku) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            sku,
            tags: None,
        }
    }
}
#[doc = "Represents the SKU name and Azure pricing tier for Analysis Services resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[doc = "Name of the SKU level."]
    pub name: String,
    #[doc = "The name of the Azure pricing tier to which the SKU applies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<resource_sku::Tier>,
    #[doc = "The number of instances in the read only query pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl ResourceSku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            capacity: None,
        }
    }
}
pub mod resource_sku {
    use super::*;
    #[doc = "The name of the Azure pricing tier to which the SKU applies."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Development,
        Basic,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Development => serializer.serialize_unit_variant("Tier", 0u32, "Development"),
                Self::Basic => serializer.serialize_unit_variant("Tier", 1u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Tier", 2u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An array of administrator user identities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerAdministrators {
    #[doc = "An array of administrator user identities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}
impl ServerAdministrators {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents SKU details for existing resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDetailsForExistingResource {
    #[doc = "Represents the SKU name and Azure pricing tier for Analysis Services resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
impl SkuDetailsForExistingResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents enumerating SKUs for existing resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuEnumerationForExistingResourceResult {
    #[doc = "The collection of available SKUs for existing resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDetailsForExistingResource>,
}
impl SkuEnumerationForExistingResourceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents enumerating SKUs for new resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuEnumerationForNewResourceResult {
    #[doc = "The collection of available SKUs for new resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSku>,
}
impl SkuEnumerationForNewResourceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
