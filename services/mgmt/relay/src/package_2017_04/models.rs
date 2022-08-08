#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Namespace/Relay Connection String"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessKeys {
    #[doc = "Primary connection string of the created namespace authorization rule."]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "Secondary connection string of the created namespace authorization rule."]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[doc = "A base64-encoded 256-bit primary key for signing and validating the SAS token."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "A base64-encoded 256-bit secondary key for signing and validating the SAS token."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "A string that describes the authorization rule."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
impl AccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a namespace authorization rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Authorization rule properties."]
    pub properties: authorization_rule::Properties,
}
impl AuthorizationRule {
    pub fn new(properties: authorization_rule::Properties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
pub mod authorization_rule {
    use super::*;
    #[doc = "Authorization rule properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "The rights associated with the rule."]
        pub rights: Vec<String>,
    }
    impl Properties {
        pub fn new(rights: Vec<String>) -> Self {
            Self { rights }
        }
    }
}
#[doc = "The response from the list namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationRuleListResult {
    #[doc = "Result of the list authorization rules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AuthorizationRule>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of authorization rules."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AuthorizationRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AuthorizationRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "Error reponse indicates Relay service is not able to process the incoming request. The reason is provided in the error message."]
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
#[doc = "Description of hybrid connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the HybridConnection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hybrid_connection::Properties>,
}
impl HybridConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hybrid_connection {
    use super::*;
    #[doc = "Properties of the HybridConnection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The time the hybrid connection was created."]
        #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[doc = "The time the namespace was updated."]
        #[serde(rename = "updatedAt", with = "azure_core::date::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[doc = "The number of listeners for this hybrid connection. Note that min : 1 and max:25 are supported."]
        #[serde(rename = "listenerCount", default, skip_serializing_if = "Option::is_none")]
        pub listener_count: Option<i32>,
        #[doc = "Returns true if client authorization is needed for this hybrid connection; otherwise, false."]
        #[serde(rename = "requiresClientAuthorization", default, skip_serializing_if = "Option::is_none")]
        pub requires_client_authorization: Option<bool>,
        #[doc = "The usermetadata is a placeholder to store user-defined string data for the hybrid connection endpoint. For example, it can be used to store descriptive data, such as a list of teams and their contact information. Also, user-defined configuration settings can be stored."]
        #[serde(rename = "userMetadata", default, skip_serializing_if = "Option::is_none")]
        pub user_metadata: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response of the list hybrid connection operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionListResult {
    #[doc = "Result of the list hybrid connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HybridConnection>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list hybrid connection operation."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HybridConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HybridConnectionListResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Parameters supplied to the regenerate authorization rule operation, specifies which key neeeds to be reset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateAccessKeyParameters {
    #[doc = "The access key to regenerate."]
    #[serde(rename = "keyType")]
    pub key_type: regenerate_access_key_parameters::KeyType,
    #[doc = "Optional. If the key value is provided, this is set to key type, or autogenerated key value set for key type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl RegenerateAccessKeyParameters {
    pub fn new(key_type: regenerate_access_key_parameters::KeyType) -> Self {
        Self { key_type, key: None }
    }
}
pub mod regenerate_access_key_parameters {
    use super::*;
    #[doc = "The access key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        PrimaryKey,
        SecondaryKey,
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
    pub enum Name {
        Standard,
    }
    #[doc = "The tier of this SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
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
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
}
#[doc = "Description of the WCF relay resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WcfRelay {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the WCF relay."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<wcf_relay::Properties>,
}
impl WcfRelay {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod wcf_relay {
    use super::*;
    #[doc = "Properties of the WCF relay."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Returns true if the relay is dynamic; otherwise, false."]
        #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
        pub is_dynamic: Option<bool>,
        #[doc = "The time the WCF relay was created."]
        #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[doc = "The time the namespace was updated."]
        #[serde(rename = "updatedAt", with = "azure_core::date::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[doc = "The number of listeners for this relay. Note that min :1 and max:25 are supported."]
        #[serde(rename = "listenerCount", default, skip_serializing_if = "Option::is_none")]
        pub listener_count: Option<i32>,
        #[doc = "WCF relay type."]
        #[serde(rename = "relayType", default, skip_serializing_if = "Option::is_none")]
        pub relay_type: Option<properties::RelayType>,
        #[doc = "Returns true if client authorization is needed for this relay; otherwise, false."]
        #[serde(rename = "requiresClientAuthorization", default, skip_serializing_if = "Option::is_none")]
        pub requires_client_authorization: Option<bool>,
        #[doc = "Returns true if transport security is needed for this relay; otherwise, false."]
        #[serde(rename = "requiresTransportSecurity", default, skip_serializing_if = "Option::is_none")]
        pub requires_transport_security: Option<bool>,
        #[doc = "The usermetadata is a placeholder to store user-defined string data for the WCF Relay endpoint. For example, it can be used to store descriptive data, such as list of teams and their contact information. Also, user-defined configuration settings can be stored."]
        #[serde(rename = "userMetadata", default, skip_serializing_if = "Option::is_none")]
        pub user_metadata: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "WCF relay type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RelayType {
            NetTcp,
            Http,
        }
    }
}
#[doc = "The response of the list WCF relay operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WcfRelaysListResult {
    #[doc = "Result of the list WCF relay operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WcfRelay>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of WCF relays."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WcfRelaysListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WcfRelaysListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
