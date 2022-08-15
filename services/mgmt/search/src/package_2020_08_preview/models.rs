#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Response containing the primary and secondary admin API keys for a given Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminKeyResult {
    #[doc = "The primary admin API key of the search service."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary admin API key of the search service."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl AdminKeyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of a long running asynchronous shared private link resource operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncOperationResult {
    #[doc = "The current status of the long running asynchronous shared private link resource operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<async_operation_result::Status>,
}
impl AsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod async_operation_result {
    use super::*;
    #[doc = "The current status of the long running asynchronous shared private link resource operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Running,
        Succeeded,
        Failed,
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
                Self::Running => serializer.serialize_unit_variant("Status", 0u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Input of check name availability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityInput {
    #[doc = "The search service name to validate. Search service names must only contain lowercase letters, digits or dashes, cannot use dash as the first two or last one characters, cannot contain consecutive dashes, and must be between 2 and 60 characters in length."]
    pub name: String,
    #[doc = "The type of the resource whose name is to be validated. This value must always be 'searchServices'."]
    #[serde(rename = "type")]
    pub type_: check_name_availability_input::Type,
}
impl CheckNameAvailabilityInput {
    pub fn new(name: String, type_: check_name_availability_input::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_availability_input {
    use super::*;
    #[doc = "The type of the resource whose name is to be validated. This value must always be 'searchServices'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "searchServices")]
        SearchServices,
    }
}
#[doc = "Output of check name availability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityOutput {
    #[doc = "A value indicating whether the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the name is not available. 'Invalid' indicates the name provided does not match the naming requirements (incorrect length, unsupported characters, etc.). 'AlreadyExists' indicates that the name is already in use and is therefore unavailable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_output::Reason>,
    #[doc = "A message that explains why the name is invalid and provides resource naming requirements. Available only if 'Invalid' is returned in the 'reason' property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_output {
    use super::*;
    #[doc = "The reason why the name is not available. 'Invalid' indicates the name provided does not match the naming requirements (incorrect length, unsupported characters, etc.). 'AlreadyExists' indicates that the name is already in use and is therefore unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains information about an API error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Describes a particular API error with an error code and a message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a particular API error with an error code and a message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An error code that describes the error condition more precisely than an HTTP status code. Can be used to programmatically handle specific error cases."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message that describes the error in detail and provides debugging information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error (for example, the name of the property in error)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Contains nested errors that are related to this error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
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
        None,
        SystemAssigned,
    }
}
#[doc = "The IP restriction rule of the Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpRule {
    #[doc = "Value corresponding to a single IPv4 address (eg., 123.1.2.3) or an IP range in CIDR format (eg., 123.1.2.3/24) to be allowed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl IpRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response containing the query API keys for a given Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListQueryKeysResult {
    #[doc = "The query keys for the Azure Cognitive Search service."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QueryKey>,
    #[doc = "Request URL that can be used to query next page of query keys. Returned when the total number of requested query keys exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListQueryKeysResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListQueryKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network specific rules that determine how the Azure Cognitive Search service may be reached."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[doc = "A list of IP restriction rules that defines the inbound network(s) with allowing access to the search service endpoint. At the meantime, all other public IP networks are blocked by the firewall. These restriction rules are applied only when the 'publicNetworkAccess' of the search service is 'enabled'; otherwise, traffic over public interface is not allowed even with any public IP rules, and private endpoint connections would be the exclusive access method."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation. This name is of the form {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that describes the operation."]
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
    #[doc = "The object that describes the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The friendly name of the resource provider."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The operation type: read, write, delete, listKeys/action, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The resource type on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The friendly name of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The result of the request to list REST API operations. It contains a list of operations and a URL  to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of operations supported by the resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL to get the next set of operation list results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an existing Private Endpoint connection to the Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of an existing Private Endpoint connection to the Azure Cognitive Search service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response containing a list of Private Endpoint connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "The list of Private Endpoint connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "Request URL that can be used to query next page of private endpoint connections. Returned when the total number of requested private endpoint connections exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an existing Private Endpoint connection to the Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The private endpoint resource from Microsoft.Network provider."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<private_endpoint_connection_properties::PrivateEndpoint>,
    #[doc = "Describes the current state of an existing Private Link Service connection to the Azure Private Endpoint."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<private_endpoint_connection_properties::PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[doc = "The private endpoint resource from Microsoft.Network provider."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PrivateEndpoint {
        #[doc = "The resource id of the private endpoint resource from Microsoft.Network provider."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl PrivateEndpoint {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Describes the current state of an existing Private Link Service connection to the Azure Private Endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PrivateLinkServiceConnectionState {
        #[doc = "Status of the the private link service connection. Can be Pending, Approved, Rejected, or Disconnected."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<private_link_service_connection_state::Status>,
        #[doc = "The description for the private link service connection state."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "A description of any extra actions that may be required."]
        #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
        pub actions_required: Option<String>,
    }
    impl PrivateLinkServiceConnectionState {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod private_link_service_connection_state {
        use super::*;
        #[doc = "Status of the the private link service connection. Can be Pending, Approved, Rejected, or Disconnected."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Pending,
            Approved,
            Rejected,
            Disconnected,
        }
    }
}
#[doc = "Describes a supported private link resource for the Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a supported private link resource for the Azure Cognitive Search service. For a given API version, this represents the 'supported' groupIds when creating a shared private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a supported private link resource for the Azure Cognitive Search service. For a given API version, this represents the 'supported' groupIds when creating a shared private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The group ID of the private link resource."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The list of required members of the private link resource."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The list of required DNS zone names of the private link resource."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
    #[doc = "The list of resources that are onboarded to private link service, that are supported by Azure Cognitive Search."]
    #[serde(rename = "shareablePrivateLinkResourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub shareable_private_link_resource_types: Vec<ShareablePrivateLinkResourceType>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response containing a list of supported Private Link Resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourcesResult {
    #[doc = "The list of supported Private Link Resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl azure_core::Continuable for PrivateLinkResourcesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateLinkResourcesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an API key for a given Azure Cognitive Search service that has permissions for query operations only."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryKey {
    #[doc = "The name of the query API key; may be empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the query API key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl QueryKey {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Describes an Azure Cognitive Search service and its current state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the search service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SearchServiceProperties>,
    #[doc = "Defines the SKU of an Azure Cognitive Search Service, which determines price tier and capacity limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl SearchService {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            sku: None,
            identity: None,
        }
    }
}
#[doc = "Response containing a list of Azure Cognitive Search services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchServiceListResult {
    #[doc = "The list of Search services."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SearchService>,
    #[doc = "Request URL that can be used to query next page of search services. Returned when the total number of requested search services exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SearchServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SearchServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchServiceProperties {
    #[doc = "The number of replicas in the search service. If specified, it must be a value between 1 and 12 inclusive for standard SKUs or between 1 and 3 inclusive for basic SKU."]
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[doc = "The number of partitions in the search service; if specified, it can be 1, 2, 3, 4, 6, or 12. Values greater than 1 are only valid for standard SKUs. For 'standard3' services with hostingMode set to 'highDensity', the allowed values are between 1 and 3."]
    #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
    #[doc = "Applicable only for the standard3 SKU. You can set this property to enable up to 3 high density partitions that allow up to 1000 indexes, which is much higher than the maximum indexes allowed for any other SKU. For the standard3 SKU, the value is either 'default' or 'highDensity'. For all other SKUs, this value must be 'default'."]
    #[serde(rename = "hostingMode", default, skip_serializing_if = "Option::is_none")]
    pub hosting_mode: Option<search_service_properties::HostingMode>,
    #[doc = "This value can be set to 'enabled' to avoid breaking changes on existing customer resources and templates. If set to 'disabled', traffic over public interface is not allowed, and private endpoint connections would be the exclusive access method."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<search_service_properties::PublicNetworkAccess>,
    #[doc = "The status of the search service. Possible values include: 'running': The search service is running and no provisioning operations are underway. 'provisioning': The search service is being provisioned or scaled up or down. 'deleting': The search service is being deleted. 'degraded': The search service is degraded. This can occur when the underlying search units are not healthy. The search service is most likely operational, but performance might be slow and some requests might be dropped. 'disabled': The search service is disabled. In this state, the service will reject all API requests. 'error': The search service is in an error state. If your service is in the degraded, disabled, or error states, it means the Azure Cognitive Search team is actively investigating the underlying issue. Dedicated services in these states are still chargeable based on the number of search units provisioned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<search_service_properties::Status>,
    #[doc = "The details of the search service status."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "The state of the last provisioning operation performed on the search service. Provisioning is an intermediate state that occurs while service capacity is being established. After capacity is set up, provisioningState changes to either 'succeeded' or 'failed'. Client applications can poll provisioning status (the recommended polling interval is from 30 seconds to one minute) by using the Get Search Service operation to see when an operation is completed. If you are using the free service, this value tends to come back as 'succeeded' directly in the call to Create search service. This is because the free service uses capacity that is already set up."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<search_service_properties::ProvisioningState>,
    #[doc = "Network specific rules that determine how the Azure Cognitive Search service may be reached."]
    #[serde(rename = "networkRuleSet", default, skip_serializing_if = "Option::is_none")]
    pub network_rule_set: Option<NetworkRuleSet>,
    #[doc = "The list of private endpoint connections to the Azure Cognitive Search service."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The list of shared private link resources managed by the Azure Cognitive Search service."]
    #[serde(rename = "sharedPrivateLinkResources", default, skip_serializing_if = "Vec::is_empty")]
    pub shared_private_link_resources: Vec<SharedPrivateLinkResource>,
}
impl SearchServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod search_service_properties {
    use super::*;
    #[doc = "Applicable only for the standard3 SKU. You can set this property to enable up to 3 high density partitions that allow up to 1000 indexes, which is much higher than the maximum indexes allowed for any other SKU. For the standard3 SKU, the value is either 'default' or 'highDensity'. For all other SKUs, this value must be 'default'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostingMode {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "highDensity")]
        HighDensity,
    }
    impl Default for HostingMode {
        fn default() -> Self {
            Self::Default
        }
    }
    #[doc = "This value can be set to 'enabled' to avoid breaking changes on existing customer resources and templates. If set to 'disabled', traffic over public interface is not allowed, and private endpoint connections would be the exclusive access method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "The status of the search service. Possible values include: 'running': The search service is running and no provisioning operations are underway. 'provisioning': The search service is being provisioned or scaled up or down. 'deleting': The search service is being deleted. 'degraded': The search service is degraded. This can occur when the underlying search units are not healthy. The search service is most likely operational, but performance might be slow and some requests might be dropped. 'disabled': The search service is disabled. In this state, the service will reject all API requests. 'error': The search service is in an error state. If your service is in the degraded, disabled, or error states, it means the Azure Cognitive Search team is actively investigating the underlying issue. Dedicated services in these states are still chargeable based on the number of search units provisioned."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "degraded")]
        Degraded,
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "error")]
        Error,
    }
    #[doc = "The state of the last provisioning operation performed on the search service. Provisioning is an intermediate state that occurs while service capacity is being established. After capacity is set up, provisioningState changes to either 'succeeded' or 'failed'. Client applications can poll provisioning status (the recommended polling interval is from 30 seconds to one minute) by using the Get Search Service operation to see when an operation is completed. If you are using the free service, this value tends to come back as 'succeeded' directly in the call to Create search service. This is because the free service uses capacity that is already set up."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "failed")]
        Failed,
    }
}
#[doc = "The parameters used to update an Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchServiceUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the search service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SearchServiceProperties>,
    #[doc = "Defines the SKU of an Azure Cognitive Search Service, which determines price tier and capacity limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The geographic location of the resource. This must be one of the supported and registered Azure Geo Regions (for example, West US, East US, Southeast Asia, and so forth). This property is required when creating a new resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags to help categorize the resource in the Azure portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl SearchServiceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a resource type that has been onboarded to private link service, supported by Azure Cognitive Search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareablePrivateLinkResourceProperties {
    #[doc = "The resource provider type for the resource that has been onboarded to private link service, supported by Azure Cognitive Search."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource provider group id for the resource that has been onboarded to private link service, supported by Azure Cognitive Search."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The description of the resource type that has been onboarded to private link service, supported by Azure Cognitive Search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ShareablePrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an resource type that has been onboarded to private link service, supported by Azure Cognitive Search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareablePrivateLinkResourceType {
    #[doc = "The name of the resource type that has been onboarded to private link service, supported by Azure Cognitive Search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the properties of a resource type that has been onboarded to private link service, supported by Azure Cognitive Search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ShareablePrivateLinkResourceProperties>,
}
impl ShareablePrivateLinkResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Shared Private Link Resource managed by the Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of an existing Shared Private Link Resource managed by the Azure Cognitive Search service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedPrivateLinkResourceProperties>,
}
impl SharedPrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response containing a list of Shared Private Link Resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResourceListResult {
    #[doc = "The list of Shared Private Link Resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedPrivateLinkResource>,
    #[doc = "The URL to get the next set of shared private link resources, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedPrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SharedPrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an existing Shared Private Link Resource managed by the Azure Cognitive Search service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResourceProperties {
    #[doc = "The resource id of the resource the shared private link resource is for."]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "The group id from the provider of resource the shared private link resource is for."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The request message for requesting approval of the shared private link resource."]
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[doc = "Optional. Can be used to specify the Azure Resource Manager location of the resource to which a shared private link is to be created. This is only required for those resources whose DNS configuration are regional (such as Azure Kubernetes Service)."]
    #[serde(rename = "resourceRegion", default, skip_serializing_if = "Option::is_none")]
    pub resource_region: Option<String>,
    #[doc = "Status of the shared private link resource. Can be Pending, Approved, Rejected or Disconnected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<shared_private_link_resource_properties::Status>,
    #[doc = "The provisioning state of the shared private link resource. Can be Updating, Deleting, Failed, Succeeded or Incomplete."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<shared_private_link_resource_properties::ProvisioningState>,
}
impl SharedPrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod shared_private_link_resource_properties {
    use super::*;
    #[doc = "Status of the shared private link resource. Can be Pending, Approved, Rejected or Disconnected."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
    }
    #[doc = "The provisioning state of the shared private link resource. Can be Updating, Deleting, Failed, Succeeded or Incomplete."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Updating,
        Deleting,
        Failed,
        Succeeded,
        Incomplete,
    }
}
#[doc = "Defines the SKU of an Azure Cognitive Search Service, which determines price tier and capacity limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "The SKU of the search service. Valid values include: 'free': Shared service. 'basic': Dedicated service with up to 3 replicas. 'standard': Dedicated service with up to 12 partitions and 12 replicas. 'standard2': Similar to standard, but with more capacity per search unit. 'standard3': The largest Standard offering with up to 12 partitions and 12 replicas (or up to 3 partitions with more indexes if you also set the hostingMode property to 'highDensity'). 'storage_optimized_l1': Supports 1TB per partition, up to 12 partitions. 'storage_optimized_l2': Supports 2TB per partition, up to 12 partitions.'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku {
    use super::*;
    #[doc = "The SKU of the search service. Valid values include: 'free': Shared service. 'basic': Dedicated service with up to 3 replicas. 'standard': Dedicated service with up to 12 partitions and 12 replicas. 'standard2': Similar to standard, but with more capacity per search unit. 'standard3': The largest Standard offering with up to 12 partitions and 12 replicas (or up to 3 partitions with more indexes if you also set the hostingMode property to 'highDensity'). 'storage_optimized_l1': Supports 1TB per partition, up to 12 partitions. 'storage_optimized_l2': Supports 2TB per partition, up to 12 partitions.'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "free")]
        Free,
        #[serde(rename = "basic")]
        Basic,
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "standard2")]
        Standard2,
        #[serde(rename = "standard3")]
        Standard3,
        #[serde(rename = "storage_optimized_l1")]
        StorageOptimizedL1,
        #[serde(rename = "storage_optimized_l2")]
        StorageOptimizedL2,
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
