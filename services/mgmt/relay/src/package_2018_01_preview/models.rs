#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Description of the check name availability request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailability {
    #[doc = "The namespace name to check for availability. The namespace name can contain only letters, numbers, and hyphens. The namespace must start with a letter, and it must end with a letter or number."]
    pub name: String,
}
impl CheckNameAvailability {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "Description of the check name availability request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "The detailed info regarding the reason associated with the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Value indicating namespace is available. Returns true if the namespace is available; otherwise, false."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Specifies the reason for the unavailability of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates Relay service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
#[doc = "The response from the List namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NwRuleSetIpRules {
    #[doc = "IP Mask"]
    #[serde(rename = "ipMask", default, skip_serializing_if = "Option::is_none")]
    pub ip_mask: Option<String>,
    #[doc = "The IP Filter Action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<nw_rule_set_ip_rules::Action>,
}
impl NwRuleSetIpRules {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nw_rule_set_ip_rules {
    use super::*;
    #[doc = "The IP Filter Action"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Description of topic resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "NetworkRuleSet properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<network_rule_set::Properties>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "NetworkRuleSet properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Default Action for Network Rule Set"]
        #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
        pub default_action: Option<properties::DefaultAction>,
        #[doc = "List of IpRules"]
        #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_rules: Vec<NwRuleSetIpRules>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Default Action for Network Rule Set"]
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
    }
}
#[doc = "A Relay REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
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
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Relay."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Invoice, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Relay operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Relay operations supported by resource provider."]
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
#[doc = "Operation Status object properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusItem {
    #[doc = "Full identifier of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of private endpoint connection operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl OperationStatusItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint object properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Full identifier of the private endpoint resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Private endpoint connection item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionItem {
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnectionItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Private endpoint object properties."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "An object that represents the approval state of the private link connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Unknown,
    Succeeded,
    Creating,
    Updating,
    UpdatingByProxy,
    Deleting,
    DeletingByProxy,
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
            Self::Unknown => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Updating"),
            Self::UpdatingByProxy => {
                serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 4u32, "UpdatingByProxy")
            }
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 5u32, "Deleting"),
            Self::DeletingByProxy => {
                serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 6u32, "DeletingByProxy")
            }
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
    Disconnected,
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
            Self::Disconnected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "Group identifier of private link resource."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Required member names of private link resource."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "Required DNS zone names of the the private link resource."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents the approval state of the private link connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval or rejection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionRequired", default, skip_serializing_if = "Option::is_none")]
    pub action_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a namespace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelayNamespace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SKU of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RelayNamespaceProperties>,
}
impl RelayNamespace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "The response from the list namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayNamespaceListResult {
    #[doc = "Result of the list namespace operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RelayNamespace>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of namespaces."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RelayNamespaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RelayNamespaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayNamespaceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<relay_namespace_properties::ProvisioningState>,
    #[doc = "The time the namespace was created."]
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The time the namespace was updated."]
    #[serde(rename = "updatedAt", with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "Endpoint you can use to perform Service Bus operations."]
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[doc = "Identifier for Azure Insights metrics."]
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
}
impl RelayNamespaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod relay_namespace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Created,
        Succeeded,
        Deleted,
        Failed,
        Updating,
        Unknown,
    }
}
#[doc = "Description of a namespace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayUpdateParameters {
    #[serde(flatten)]
    pub resource_namespace_patch: ResourceNamespacePatch,
    #[doc = "SKU of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RelayNamespaceProperties>,
}
impl RelayUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNamespacePatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceNamespacePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU of the namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Name of this SKU."]
    pub name: sku::Name,
    #[doc = "The tier of this SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "Name of this SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Standard,
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
                Self::Standard => serializer.serialize_unit_variant("Name", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The tier of this SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
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
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[doc = "Specifies the reason for the unavailability of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UnavailableReason")]
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UnavailableReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UnavailableReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UnavailableReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("UnavailableReason", 0u32, "None"),
            Self::InvalidName => serializer.serialize_unit_variant("UnavailableReason", 1u32, "InvalidName"),
            Self::SubscriptionIsDisabled => serializer.serialize_unit_variant("UnavailableReason", 2u32, "SubscriptionIsDisabled"),
            Self::NameInUse => serializer.serialize_unit_variant("UnavailableReason", 3u32, "NameInUse"),
            Self::NameInLockdown => serializer.serialize_unit_variant("UnavailableReason", 4u32, "NameInLockdown"),
            Self::TooManyNamespaceInCurrentSubscription => {
                serializer.serialize_unit_variant("UnavailableReason", 5u32, "TooManyNamespaceInCurrentSubscription")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
