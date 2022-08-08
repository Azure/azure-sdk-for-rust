#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The collection of Connectivity related groups and policies within the Managed Network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectivityCollection {
    #[doc = "The collection of connectivity related Managed Network Groups within the Managed Network"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ManagedNetworkGroup>,
    #[doc = "The collection of Managed Network Peering Policies within the Managed Network"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peerings: Vec<ManagedNetworkPeeringPolicy>,
}
impl ConnectivityCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error response that indicates why an operation has failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
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
#[doc = "Properties of a Hub and Spoke Peering Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HubAndSpokePeeringPolicyProperties {
    #[serde(flatten)]
    pub managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties,
    #[doc = "Generic pointer to a resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hub: Option<ResourceId>,
    #[doc = "Gets or sets the spokes group IDs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spokes: Vec<ResourceId>,
}
impl HubAndSpokePeeringPolicyProperties {
    pub fn new(managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties) -> Self {
        Self {
            managed_network_peering_policy_properties,
            hub: None,
            spokes: Vec::new(),
        }
    }
}
#[doc = "The Managed Network resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of Managed Network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkProperties>,
}
impl ManagedNetwork {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The Managed Network Group resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a Managed Network Group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkGroupProperties>,
    #[doc = "Responsibility role under which this Managed Network Group will be created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<managed_network_group::Kind>,
}
impl ManagedNetworkGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_network_group {
    use super::*;
    #[doc = "Responsibility role under which this Managed Network Group will be created"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Connectivity,
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
                Self::Connectivity => serializer.serialize_unit_variant("Kind", 0u32, "Connectivity"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the request to list Managed Network Groups. It contains a list of groups and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkGroupListResult {
    #[doc = "Gets a page of ManagedNetworkGroup"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetworkGroup>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedNetworkGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedNetworkGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Managed Network Group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkGroupProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[doc = "The collection of management groups covered by the Managed Network"]
    #[serde(rename = "managementGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<ResourceId>,
    #[doc = "The collection of subscriptions covered by the Managed Network"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<ResourceId>,
    #[doc = "The collection of virtual nets covered by the Managed Network"]
    #[serde(rename = "virtualNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_networks: Vec<ResourceId>,
    #[doc = "The collection of  subnets covered by the Managed Network"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<ResourceId>,
}
impl ManagedNetworkGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list Managed Network. It contains a list of Managed Networks and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkListResult {
    #[doc = "Gets a page of ManagedNetworks"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetwork>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedNetworkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Managed Network Peering Policy resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkPeeringPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a Managed Network Peering Policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkPeeringPolicyProperties>,
}
impl ManagedNetworkPeeringPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list Managed Network Peering Policies. It contains a list of policies and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkPeeringPolicyListResult {
    #[doc = "Gets a page of Peering Policies"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetworkPeeringPolicy>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedNetworkPeeringPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedNetworkPeeringPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Managed Network Peering Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkPeeringPolicyProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[doc = "Gets or sets the connectivity type of a network structure policy"]
    #[serde(rename = "type")]
    pub type_: managed_network_peering_policy_properties::Type,
    #[doc = "Generic pointer to a resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hub: Option<ResourceId>,
    #[doc = "Gets or sets the spokes group IDs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spokes: Vec<ResourceId>,
    #[doc = "Gets or sets the mesh group IDs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mesh: Vec<ResourceId>,
}
impl ManagedNetworkPeeringPolicyProperties {
    pub fn new(type_: managed_network_peering_policy_properties::Type) -> Self {
        Self {
            resource_properties: ResourceProperties::default(),
            type_,
            hub: None,
            spokes: Vec::new(),
            mesh: Vec::new(),
        }
    }
}
pub mod managed_network_peering_policy_properties {
    use super::*;
    #[doc = "Gets or sets the connectivity type of a network structure policy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        HubAndSpokeTopology,
        MeshTopology,
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
                Self::HubAndSpokeTopology => serializer.serialize_unit_variant("Type", 0u32, "HubAndSpokeTopology"),
                Self::MeshTopology => serializer.serialize_unit_variant("Type", 1u32, "MeshTopology"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of Managed Network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[doc = "Scope of a Managed Network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[doc = "The collection of Connectivity related groups and policies within the Managed Network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<ConnectivityCollection>,
}
impl ManagedNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update Tags of Managed Network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagedNetworkUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Mesh Peering Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeshPeeringPolicyProperties {
    #[serde(flatten)]
    pub managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties,
    #[doc = "Gets or sets the mesh group IDs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mesh: Vec<ResourceId>,
}
impl MeshPeeringPolicyProperties {
    pub fn new(managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties) -> Self {
        Self {
            managed_network_peering_policy_properties,
            mesh: Vec::new(),
        }
    }
}
#[doc = "REST API operation"]
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
        #[doc = "Service provider: Microsoft.ManagedNetwork"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
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
#[doc = "Result of the request to list Managed Network operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Resource Provider operations supported by the Managed Network resource provider."]
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
#[doc = "The general resource model definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Generic pointer to a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceId {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base for resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProperties {
    #[doc = "Provisioning state of the ManagedNetwork resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<resource_properties::ProvisioningState>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_properties {
    use super::*;
    #[doc = "Provisioning state of the ManagedNetwork resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Updating,
        Deleting,
        Failed,
        Succeeded,
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
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Scope of a Managed Network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scope {
    #[doc = "The collection of management groups covered by the Managed Network"]
    #[serde(rename = "managementGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<ResourceId>,
    #[doc = "The collection of subscriptions covered by the Managed Network"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<ResourceId>,
    #[doc = "The collection of virtual nets covered by the Managed Network"]
    #[serde(rename = "virtualNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_networks: Vec<ResourceId>,
    #[doc = "The collection of  subnets covered by the Managed Network"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<ResourceId>,
}
impl Scope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Managed Network resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of Managed Network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopeAssignmentProperties>,
}
impl ScopeAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list ScopeAssignment. It contains a list of groups and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeAssignmentListResult {
    #[doc = "Gets a page of ScopeAssignment"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScopeAssignment>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScopeAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScopeAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Managed Network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeAssignmentProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[doc = "The managed network ID with scope will be assigned to."]
    #[serde(rename = "assignedManagedNetwork", default, skip_serializing_if = "Option::is_none")]
    pub assigned_managed_network: Option<String>,
}
impl ScopeAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
        }
    }
}
