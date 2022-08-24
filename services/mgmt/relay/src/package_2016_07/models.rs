#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Description of a Namespace AuthorizationRules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "AuthorizationRule properties."]
    pub properties: AuthorizationRuleProperties,
}
impl AuthorizationRule {
    pub fn new(properties: AuthorizationRuleProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Namespace/Relay Connection String"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationRuleKeys {
    #[doc = "PrimaryConnectionString of the created Namespace AuthorizationRule."]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "SecondaryConnectionString of the created Namespace AuthorizationRule"]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[doc = "A base64-encoded 256-bit primary key for signing and validating the SAS token"]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "A base64-encoded 256-bit secondary key for signing and validating the SAS token"]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "A string that describes the authorization rule"]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
impl AuthorizationRuleKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationRuleListResult {
    #[doc = "Result of the List AuthorizationRules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AuthorizationRule>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of AuthorizationRules"]
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
#[doc = "AuthorizationRule properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRuleProperties {
    #[doc = "The rights associated with the rule."]
    pub rights: Vec<String>,
}
impl AuthorizationRuleProperties {
    pub fn new(rights: Vec<String>) -> Self {
        Self { rights }
    }
}
#[doc = "Description of a Check Name availability request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailability {
    #[doc = "The Name to check the namespce name availability and The namespace name can contain only letters, numbers, and hyphens. The namespace must start with a letter, and it must end with a letter or number."]
    pub name: String,
}
impl CheckNameAvailability {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "Description of a Check Name availability request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Value indicating namespace is availability, true if the namespace is available; otherwise, false."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Specifies the reason for the unavailability of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
    #[doc = "The detailed info regarding the reason associated with the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
#[doc = "Description of HybridConnection Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the HybridConnection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HybridConnectionProperties>,
}
impl HybridConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List HybridConnection operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionListResult {
    #[doc = "Result of the List HybridConnection ."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HybridConnection>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of HybridConnection operation"]
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
#[doc = "Properties of the HybridConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionProperties {
    #[doc = "The time the HybridConnection was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The time the namespace was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "The number of listeners for this HybridConnection. min : 1 and max:25 supported"]
    #[serde(rename = "listenerCount", default, skip_serializing_if = "Option::is_none")]
    pub listener_count: Option<i32>,
    #[doc = "true if client authorization is needed for this HybridConnection; otherwise, false."]
    #[serde(rename = "requiresClientAuthorization", default, skip_serializing_if = "Option::is_none")]
    pub requires_client_authorization: Option<bool>,
    #[doc = "usermetadata is a placeholder to store user-defined string data for the HybridConnection endpoint.e.g. it can be used to store  descriptive data, such as list of teams and their contact information also user-defined configuration settings can be stored."]
    #[serde(rename = "userMetadata", default, skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<String>,
}
impl HybridConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A EventHub REST API operation"]
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
        #[doc = "Service provider: Microsoft.EventHub"]
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
#[doc = "Result of the request to list EventHub operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of EventHub operations supported by the Microsoft.EventHub resource provider."]
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
#[doc = "Parameters supplied to the Regenerate Authorization Rule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeysParameters {
    #[doc = "Key that needs to be regenerated."]
    #[serde(rename = "policyKey", default, skip_serializing_if = "Option::is_none")]
    pub policy_key: Option<regenerate_keys_parameters::PolicyKey>,
}
impl RegenerateKeysParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regenerate_keys_parameters {
    use super::*;
    #[doc = "Key that needs to be regenerated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PolicyKey")]
    pub enum PolicyKey {
        PrimaryKey,
        SecondaryKey,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PolicyKey {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PolicyKey {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PolicyKey {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PrimaryKey => serializer.serialize_unit_variant("PolicyKey", 0u32, "PrimaryKey"),
                Self::SecondaryKey => serializer.serialize_unit_variant("PolicyKey", 1u32, "SecondaryKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Description of a Namespace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelayNamespace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Sku of the Namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of the Namespace."]
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
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayNamespaceListResult {
    #[doc = "Result of the List Namespace operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RelayNamespace>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Namespaces"]
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
#[doc = "Properties of the Namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayNamespaceProperties {
    #[doc = "Provisioning state of the namespace."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The time the namespace was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The time the namespace was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "Endpoint you can use to perform Service Bus operations."]
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[doc = "Identifier for Azure Insights metrics"]
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
}
impl RelayNamespaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Patch Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayNamespaceUpdateParameter {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Sku of the Namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl RelayNamespaceUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku of the Namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Name of this Sku"]
    pub name: sku::Name,
    #[doc = "The tier of this particular SKU"]
    pub tier: sku::Tier,
}
impl Sku {
    pub fn new(name: sku::Name, tier: sku::Tier) -> Self {
        Self { name, tier }
    }
}
pub mod sku {
    use super::*;
    #[doc = "Name of this Sku"]
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
    #[doc = "The tier of this particular SKU"]
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
#[doc = "Definition of Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
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
#[doc = "Description of WcfRelays Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WcfRelay {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the WcfRelay Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WcfRelayProperties>,
}
impl WcfRelay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the WcfRelay Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WcfRelayProperties {
    #[doc = "WCFRelay Type."]
    #[serde(rename = "relayType", default, skip_serializing_if = "Option::is_none")]
    pub relay_type: Option<wcf_relay_properties::RelayType>,
    #[doc = "The time the WCFRelay was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The time the namespace was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "The number of listeners for this relay. min : 1 and max:25 supported"]
    #[serde(rename = "listenerCount", default, skip_serializing_if = "Option::is_none")]
    pub listener_count: Option<i32>,
    #[doc = "true if client authorization is needed for this relay; otherwise, false."]
    #[serde(rename = "requiresClientAuthorization", default, skip_serializing_if = "Option::is_none")]
    pub requires_client_authorization: Option<bool>,
    #[doc = "true if transport security is needed for this relay; otherwise, false."]
    #[serde(rename = "requiresTransportSecurity", default, skip_serializing_if = "Option::is_none")]
    pub requires_transport_security: Option<bool>,
    #[doc = "true if the relay is dynamic; otherwise, false."]
    #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
    #[doc = "usermetadata is a placeholder to store user-defined string data for the HybridConnection endpoint.e.g. it can be used to store  descriptive data, such as list of teams and their contact information also user-defined configuration settings can be stored."]
    #[serde(rename = "userMetadata", default, skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<String>,
}
impl WcfRelayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod wcf_relay_properties {
    use super::*;
    #[doc = "WCFRelay Type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RelayType")]
    pub enum RelayType {
        NetTcp,
        Http,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RelayType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RelayType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RelayType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NetTcp => serializer.serialize_unit_variant("RelayType", 0u32, "NetTcp"),
                Self::Http => serializer.serialize_unit_variant("RelayType", 1u32, "Http"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response of the List WcfRelays operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WcfRelaysListResult {
    #[doc = "Result of the List WcfRelays ."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WcfRelay>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of WcfRelays operation"]
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
