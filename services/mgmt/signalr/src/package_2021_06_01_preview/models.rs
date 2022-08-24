#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Default action when no other rule matches"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AclAction")]
pub enum AclAction {
    Allow,
    Deny,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AclAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AclAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AclAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Allow => serializer.serialize_unit_variant("AclAction", 0u32, "Allow"),
            Self::Deny => serializer.serialize_unit_variant("AclAction", 1u32, "Deny"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifications of the Dimension of metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[doc = "The public facing name of the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Name of the dimension as it appears in MDM."]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[doc = "A Boolean flag indicating whether this dimension should be included for the shoebox export scenario."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl Dimension {
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
#[doc = "FeatureFlags is the supported features of Azure SignalR service.\r\n- ServiceMode: Flag for backend server for SignalR service. Values allowed: \"Default\": have your own backend server; \"Serverless\": your application doesn't have a backend server; \"Classic\": for backward compatibility. Support both Default and Serverless mode but not recommended; \"PredefinedOnly\": for future use.\r\n- EnableConnectivityLogs: \"true\"/\"false\", to enable/disable the connectivity log category respectively.\r\n- EnableMessagingLogs: \"true\"/\"false\", to enable/disable the connectivity log category respectively.\r\n- EnableLiveTrace: Live Trace allows you to know what's happening inside Azure SignalR service, it will give you live traces in real time, it will be helpful when you developing your own Azure SignalR based web application or self-troubleshooting some issues. Please note that live traces are counted as outbound messages that will be charged. Values allowed: \"true\"/\"false\", to enable/disable live trace feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FeatureFlags")]
pub enum FeatureFlags {
    ServiceMode,
    EnableConnectivityLogs,
    EnableMessagingLogs,
    EnableLiveTrace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FeatureFlags {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FeatureFlags {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FeatureFlags {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ServiceMode => serializer.serialize_unit_variant("FeatureFlags", 0u32, "ServiceMode"),
            Self::EnableConnectivityLogs => serializer.serialize_unit_variant("FeatureFlags", 1u32, "EnableConnectivityLogs"),
            Self::EnableMessagingLogs => serializer.serialize_unit_variant("FeatureFlags", 2u32, "EnableMessagingLogs"),
            Self::EnableLiveTrace => serializer.serialize_unit_variant("FeatureFlags", 3u32, "EnableLiveTrace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The keyType to regenerate. Must be either 'primary' or 'secondary'(case-insensitive)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KeyType")]
pub enum KeyType {
    Primary,
    Secondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KeyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Primary => serializer.serialize_unit_variant("KeyType", 0u32, "Primary"),
            Self::Secondary => serializer.serialize_unit_variant("KeyType", 1u32, "Secondary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifications of the Logs for Azure Monitoring."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the log."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent managed identities used for request and response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentity {
    #[doc = "Represent the identity type: systemAssigned, userAssigned, None"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityType>,
    #[doc = "Get or set the user assigned identities"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
    #[doc = "Get the principal id for the system assigned identity.\r\nOnly be used in response."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Get the tenant id for the system assigned identity.\r\nOnly be used in response"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed identity settings for upstream."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentitySettings {
    #[doc = "The Resource indicating the App ID URI of the target resource.\r\nIt also appears in the aud (audience) claim of the issued token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl ManagedIdentitySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent the identity type: systemAssigned, userAssigned, None"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedIdentityType")]
pub enum ManagedIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedIdentityType", 2u32, "UserAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifications of the Metrics for Azure Monitoring."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localized friendly description of the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The unit that makes sense for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Only provide one value for this field. Valid values: Average, Minimum, Maximum, Total, Count."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Optional. If set to true, then zero will be returned for time duration where no metric is emitted/published. \r\nEx. a metric that returns the number of times a particular error code was emitted. The error code may not appear \r\noften, instead of the RP publishing 0, Shoebox can auto fill in 0s for time periods where nothing was emitted."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<String>,
    #[doc = "The name of the metric category that the metric belongs to. A metric can only belong to a single category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The dimensions of the metrics."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to check name availability. It contains a flag and possible reason of failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[doc = "Indicates whether the name is available or not."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason of the availability. Required if name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The message of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data POST-ed to the nameAvailability action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityParameters {
    #[doc = "The resource type. Can be \"Microsoft.SignalRService/SignalR\" or \"Microsoft.SignalRService/webPubSub\""]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The resource name to validate. e.g.\"my-resource-name\""]
    pub name: String,
}
impl NameAvailabilityParameters {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Network ACL"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAcl {
    #[doc = "Allowed request types. The value can be one or more of: ClientConnection, ServerConnection, RESTAPI."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow: Vec<SignalRRequestType>,
    #[doc = "Denied request types. The value can be one or more of: ClientConnection, ServerConnection, RESTAPI."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deny: Vec<SignalRRequestType>,
}
impl NetworkAcl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation supported by resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation with format: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If the operation is a data action. (for data plane rbac)"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The object that describes a operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Optional. The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Extra Operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that describes a operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Friendly name of the resource provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource type on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The localized friendly name for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly description for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list REST API operations. It contains a list of operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of operations supported by the resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extra Operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "An object that describes a specification."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Full qualified Id of the private endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ACL for a private endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointAcl {
    #[serde(flatten)]
    pub network_acl: NetworkAcl,
    #[doc = "Name of the private endpoint connection"]
    pub name: String,
}
impl PrivateEndpointAcl {
    pub fn new(name: String) -> Self {
        Self {
            network_acl: NetworkAcl::default(),
            name,
        }
    }
}
#[doc = "A private endpoint connection to an azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Private endpoint connection properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionList {
    #[doc = "The list of the private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "Request URL that can be used to query next page of private endpoint connections. Returned when the total number of requested private endpoint connections exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Private endpoint"]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "Connection state of the private endpoint connection"]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Private link resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains a list of PrivateLinkResource and a possible link to query more results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceList {
    #[doc = "List of PrivateLinkResource"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateLinkResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private link resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "Group Id of the private link resource"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Required members of the private link resource"]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "Required private DNS zone names"]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
    #[doc = "The list of resources that are onboarded to private link service"]
    #[serde(rename = "shareablePrivateLinkResourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub shareable_private_link_resource_types: Vec<ShareablePrivateLinkResourceType>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection state of the private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "Indicates whether the connection has been Approved/Rejected/Removed by the owner of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateLinkServiceConnectionStatus>,
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
#[doc = "Indicates whether the connection has been Approved/Rejected/Removed by the owner of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateLinkServiceConnectionStatus")]
pub enum PrivateLinkServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateLinkServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateLinkServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateLinkServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Failed,
    Canceled,
    Running,
    Creating,
    Updating,
    Deleting,
    Moving,
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
            Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
            Self::Running => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Running"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
            Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Moving"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a ARM proxy resource. It will have everything other than required location and tags"]
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
#[doc = "Parameters describes the request to regenerate access keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeyParameters {
    #[doc = "The keyType to regenerate. Must be either 'primary' or 'secondary'(case-insensitive)."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
}
impl RegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core properties of ARM resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource - e.g. \"Microsoft.SignalRService/SignalR\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing information of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[doc = "The name of the SKU. Required.\r\n\r\nAllowed values: Standard_S1, Free_F1"]
    pub name: String,
    #[doc = "Optional tier of this particular SKU. 'Standard' or 'Free'. \r\n\r\n`Basic` is deprecated, use `Standard` instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SignalRSkuTier>,
    #[doc = "Not used. Retained for future use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "Not used. Retained for future use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Optional, integer. The unit count of the resource. 1 by default.\r\n\r\nIf present, following values are allowed:\r\n    Free: 1\r\n    Standard: 1,2,5,10,20,50,100"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl ResourceSku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "The settings for the Upstream when the service is in server-less mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessUpstreamSettings {
    #[doc = "Gets or sets the list of Upstream URL templates. Order matters, and the first matching template takes effects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub templates: Vec<UpstreamTemplate>,
}
impl ServerlessUpstreamSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the service - e.g. \"SignalR\" for \"Microsoft.SignalRService/SignalR\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceKind")]
pub enum ServiceKind {
    SignalR,
    RawWebSockets,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SignalR => serializer.serialize_unit_variant("ServiceKind", 0u32, "SignalR"),
            Self::RawWebSockets => serializer.serialize_unit_variant("ServiceKind", 1u32, "RawWebSockets"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An object that describes a specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Specifications of the Metrics for Azure Monitoring."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[doc = "Specifications of the Logs for Azure Monitoring."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a resource type that has been onboarded to private link service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareablePrivateLinkResourceProperties {
    #[doc = "The description of the resource type that has been onboarded to private link service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource provider group id for the resource that has been onboarded to private link service"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The resource provider type for the resource that has been onboarded to private link service"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ShareablePrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a  resource type that has been onboarded to private link service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareablePrivateLinkResourceType {
    #[doc = "The name of the resource type that has been onboarded to private link service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the properties of a resource type that has been onboarded to private link service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ShareablePrivateLinkResourceProperties>,
}
impl ShareablePrivateLinkResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Shared Private Link Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Describes the properties of an existing Shared Private Link Resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedPrivateLinkResourceProperties>,
}
impl SharedPrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of shared private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResourceList {
    #[doc = "The list of the shared private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedPrivateLinkResource>,
    #[doc = "Request URL that can be used to query next page of private endpoint connections. Returned when the total number of requested private endpoint connections exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedPrivateLinkResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SharedPrivateLinkResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an existing Shared Private Link Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPrivateLinkResourceProperties {
    #[doc = "The group id from the provider of resource the shared private link resource is for"]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "The resource id of the resource the shared private link resource is for"]
    #[serde(rename = "privateLinkResourceId")]
    pub private_link_resource_id: String,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The request message for requesting approval of the shared private link resource"]
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[doc = "Status of the shared private link resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SharedPrivateLinkResourceStatus>,
}
impl SharedPrivateLinkResourceProperties {
    pub fn new(group_id: String, private_link_resource_id: String) -> Self {
        Self {
            group_id,
            private_link_resource_id,
            provisioning_state: None,
            request_message: None,
            status: None,
        }
    }
}
#[doc = "Status of the shared private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SharedPrivateLinkResourceStatus")]
pub enum SharedPrivateLinkResourceStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    Timeout,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SharedPrivateLinkResourceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SharedPrivateLinkResourceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SharedPrivateLinkResourceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 3u32, "Disconnected"),
            Self::Timeout => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 4u32, "Timeout"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Cross-Origin Resource Sharing (CORS) settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRCorsSettings {
    #[doc = "Gets or sets the list of origins that should be allowed to make cross-origin calls (for example: http://example.com:12345). Use \"*\" to allow all. If omitted, allow all by default."]
    #[serde(rename = "allowedOrigins", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_origins: Vec<String>,
}
impl SignalRCorsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Feature of a resource, which controls the runtime behavior."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalRFeature {
    #[doc = "FeatureFlags is the supported features of Azure SignalR service.\r\n- ServiceMode: Flag for backend server for SignalR service. Values allowed: \"Default\": have your own backend server; \"Serverless\": your application doesn't have a backend server; \"Classic\": for backward compatibility. Support both Default and Serverless mode but not recommended; \"PredefinedOnly\": for future use.\r\n- EnableConnectivityLogs: \"true\"/\"false\", to enable/disable the connectivity log category respectively.\r\n- EnableMessagingLogs: \"true\"/\"false\", to enable/disable the connectivity log category respectively.\r\n- EnableLiveTrace: Live Trace allows you to know what's happening inside Azure SignalR service, it will give you live traces in real time, it will be helpful when you developing your own Azure SignalR based web application or self-troubleshooting some issues. Please note that live traces are counted as outbound messages that will be charged. Values allowed: \"true\"/\"false\", to enable/disable live trace feature."]
    pub flag: FeatureFlags,
    #[doc = "Value of the feature flag. See Azure SignalR service document https://docs.microsoft.com/azure/azure-signalr/ for allowed values."]
    pub value: String,
    #[doc = "Optional properties related to this feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SignalRFeature {
    pub fn new(flag: FeatureFlags, value: String) -> Self {
        Self {
            flag,
            value,
            properties: None,
        }
    }
}
#[doc = "A class represents the access keys of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRKeys {
    #[doc = "The primary access key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary access key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Connection string constructed via the primaryKey"]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "Connection string constructed via the secondaryKey"]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
}
impl SignalRKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network ACLs for the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRNetworkAcLs {
    #[doc = "Default action when no other rule matches"]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<AclAction>,
    #[doc = "Network ACL"]
    #[serde(rename = "publicNetwork", default, skip_serializing_if = "Option::is_none")]
    pub public_network: Option<NetworkAcl>,
    #[doc = "ACLs for requests from private endpoints"]
    #[serde(rename = "privateEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoints: Vec<PrivateEndpointAcl>,
}
impl SignalRNetworkAcLs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class that describes the properties of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The publicly accessible IP of the resource."]
    #[serde(rename = "externalIP", default, skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    #[doc = "FQDN of the service instance."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The publicly accessible port of the resource which is designed for browser/client side usage."]
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[doc = "The publicly accessible port of the resource which is designed for customer server side usage."]
    #[serde(rename = "serverPort", default, skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[doc = "Version of the resource. Probably you need the same or higher version of client SDKs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Private endpoint connections to the resource."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The list of shared private link resources."]
    #[serde(rename = "sharedPrivateLinkResources", default, skip_serializing_if = "Vec::is_empty")]
    pub shared_private_link_resources: Vec<SharedPrivateLinkResource>,
    #[doc = "TLS settings for the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<SignalRTlsSettings>,
    #[doc = "List of the featureFlags.\r\n\r\nFeatureFlags that are not included in the parameters for the update operation will not be modified.\r\nAnd the response will only include featureFlags that are explicitly set. \r\nWhen a featureFlag is not explicitly set, its globally default value will be used\r\nBut keep in mind, the default value doesn't mean \"false\". It varies in terms of different FeatureFlags."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<SignalRFeature>,
    #[doc = "Cross-Origin Resource Sharing (CORS) settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<SignalRCorsSettings>,
    #[doc = "The settings for the Upstream when the service is in server-less mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<ServerlessUpstreamSettings>,
    #[doc = "Network ACLs for the resource"]
    #[serde(rename = "networkACLs", default, skip_serializing_if = "Option::is_none")]
    pub network_ac_ls: Option<SignalRNetworkAcLs>,
    #[doc = "Enable or disable public network access. Default to \"Enabled\".\r\nWhen it's Enabled, network ACLs still apply.\r\nWhen it's Disabled, public network access is always disabled no matter what you set in network ACLs."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
    #[doc = "DisableLocalAuth\r\nEnable or disable local auth with AccessKey\r\nWhen set as true, connection with AccessKey=xxx won't work."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[doc = "DisableLocalAuth\r\nEnable or disable aad auth\r\nWhen set as true, connection with AuthType=aad won't work."]
    #[serde(rename = "disableAadAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_aad_auth: Option<bool>,
}
impl SignalRProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Allowed request types. The value can be one or more of: ClientConnection, ServerConnection, RESTAPI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SignalRRequestType")]
pub enum SignalRRequestType {
    ClientConnection,
    ServerConnection,
    #[serde(rename = "RESTAPI")]
    Restapi,
    Trace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SignalRRequestType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SignalRRequestType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SignalRRequestType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ClientConnection => serializer.serialize_unit_variant("SignalRRequestType", 0u32, "ClientConnection"),
            Self::ServerConnection => serializer.serialize_unit_variant("SignalRRequestType", 1u32, "ServerConnection"),
            Self::Restapi => serializer.serialize_unit_variant("SignalRRequestType", 2u32, "RESTAPI"),
            Self::Trace => serializer.serialize_unit_variant("SignalRRequestType", 3u32, "Trace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class represent a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The billing information of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[doc = "A class that describes the properties of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SignalRProperties>,
    #[doc = "The kind of the service - e.g. \"SignalR\" for \"Microsoft.SignalRService/SignalR\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ServiceKind>,
    #[doc = "A class represent managed identities used for request and response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SignalRResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of resources and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRResourceList {
    #[doc = "List of the resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SignalRResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SignalRResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SignalRResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional tier of this particular SKU. 'Standard' or 'Free'. \r\n\r\n`Basic` is deprecated, use `Standard` instead."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SignalRSkuTier")]
pub enum SignalRSkuTier {
    Free,
    Basic,
    Standard,
    Premium,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SignalRSkuTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SignalRSkuTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SignalRSkuTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Free => serializer.serialize_unit_variant("SignalRSkuTier", 0u32, "Free"),
            Self::Basic => serializer.serialize_unit_variant("SignalRSkuTier", 1u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("SignalRSkuTier", 2u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("SignalRSkuTier", 3u32, "Premium"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "TLS settings for the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRTlsSettings {
    #[doc = "Request client certificate during TLS handshake if enabled"]
    #[serde(rename = "clientCertEnabled", default, skip_serializing_if = "Option::is_none")]
    pub client_cert_enabled: Option<bool>,
}
impl SignalRTlsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that describes a specific usage of the resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRUsage {
    #[doc = "Fully qualified ARM resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Current value for the usage quota."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The maximum permitted value for the usage quota. If there is no limit, this value will be -1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "Localizable String object containing the name and a localized value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SignalRUsageName>,
    #[doc = "Representing the units of the usage quota. Possible values are: Count, Bytes, Seconds, Percent, CountPerSecond, BytesPerSecond."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl SignalRUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of the resource usages and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRUsageList {
    #[doc = "List of the resource usages"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SignalRUsage>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SignalRUsageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SignalRUsageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localizable String object containing the name and a localized value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRUsageName {
    #[doc = "The identifier of the usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Localized name of the usage."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl SignalRUsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The GEO location of the resource. e.g. West US | East US | North Central US | South Central US."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags of the service which is a list of key value pairs that describe the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Upstream auth settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpstreamAuthSettings {
    #[doc = "Gets or sets the type of auth. None or ManagedIdentity is supported now."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpstreamAuthType>,
    #[doc = "Managed identity settings for upstream."]
    #[serde(rename = "managedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub managed_identity: Option<ManagedIdentitySettings>,
}
impl UpstreamAuthSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the type of auth. None or ManagedIdentity is supported now."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpstreamAuthType")]
pub enum UpstreamAuthType {
    None,
    ManagedIdentity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpstreamAuthType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpstreamAuthType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpstreamAuthType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("UpstreamAuthType", 0u32, "None"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("UpstreamAuthType", 1u32, "ManagedIdentity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Upstream template item settings. It defines the Upstream URL of the incoming requests.\r\nThe template defines the pattern of the event, the hub or the category of the incoming request that matches current URL template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpstreamTemplate {
    #[doc = "Gets or sets the matching pattern for hub names. If not set, it matches any hub.\r\nThere are 3 kind of patterns supported:\r\n    1. \"*\", it to matches any hub name\r\n    2. Combine multiple hubs with \",\", for example \"hub1,hub2\", it matches \"hub1\" and \"hub2\"\r\n    3. The single hub name, for example, \"hub1\", it matches \"hub1\""]
    #[serde(rename = "hubPattern", default, skip_serializing_if = "Option::is_none")]
    pub hub_pattern: Option<String>,
    #[doc = "Gets or sets the matching pattern for event names. If not set, it matches any event.\r\nThere are 3 kind of patterns supported:\r\n    1. \"*\", it to matches any event name\r\n    2. Combine multiple events with \",\", for example \"connect,disconnect\", it matches event \"connect\" and \"disconnect\"\r\n    3. The single event name, for example, \"connect\", it matches \"connect\""]
    #[serde(rename = "eventPattern", default, skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[doc = "Gets or sets the matching pattern for category names. If not set, it matches any category.\r\nThere are 3 kind of patterns supported:\r\n    1. \"*\", it to matches any category name\r\n    2. Combine multiple categories with \",\", for example \"connections,messages\", it matches category \"connections\" and \"messages\"\r\n    3. The single category name, for example, \"connections\", it matches the category \"connections\""]
    #[serde(rename = "categoryPattern", default, skip_serializing_if = "Option::is_none")]
    pub category_pattern: Option<String>,
    #[doc = "Gets or sets the Upstream URL template. You can use 3 predefined parameters {hub}, {category} {event} inside the template, the value of the Upstream URL is dynamically calculated when the client request comes in.\r\nFor example, if the urlTemplate is `http://example.com/{hub}/api/{event}`, with a client request from hub `chat` connects, it will first POST to this URL: `http://example.com/chat/api/connect`."]
    #[serde(rename = "urlTemplate")]
    pub url_template: String,
    #[doc = "Upstream auth settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<UpstreamAuthSettings>,
}
impl UpstreamTemplate {
    pub fn new(url_template: String) -> Self {
        Self {
            hub_pattern: None,
            event_pattern: None,
            category_pattern: None,
            url_template,
            auth: None,
        }
    }
}
#[doc = "Properties of user assigned identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentityProperty {
    #[doc = "Get the principal id for the user assigned identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Get the client id for the user assigned identity"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentityProperty {
    pub fn new() -> Self {
        Self::default()
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
