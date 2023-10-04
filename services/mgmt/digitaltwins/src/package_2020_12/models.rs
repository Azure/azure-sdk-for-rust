#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The result returned from a database check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameRequest {
    #[doc = "Resource name."]
    pub name: String,
    #[doc = "The type of resource, for instance Microsoft.DigitalTwins/digitalTwinsInstances."]
    #[serde(rename = "type")]
    pub type_: check_name_request::Type,
}
impl CheckNameRequest {
    pub fn new(name: String, type_: check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_request {
    use super::*;
    #[doc = "The type of resource, for instance Microsoft.DigitalTwins/digitalTwinsInstances."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DigitalTwins/digitalTwinsInstances")]
        MicrosoftDigitalTwinsDigitalTwinsInstances,
    }
}
#[doc = "The result returned from a check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameResult {
    #[doc = "Specifies a Boolean value that indicates if the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Message indicating an unavailable name due to a conflict, or a description of the naming rules that are violated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Message providing the reason why the given name is invalid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_result::Reason>,
}
impl CheckNameResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_result {
    use super::*;
    #[doc = "Message providing the reason why the given name is invalid."]
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
#[doc = "The properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<connection_properties::ProvisioningState>,
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<serde_json::Value>,
    #[doc = "The list of group ids for the private endpoint connection."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<GroupId>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<serde_json::Value>,
}
impl ConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_properties {
    use super::*;
    #[doc = "The provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Pending,
        Approved,
        Rejected,
        Disconnected,
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
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The current state of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionState {
    #[doc = "The status of a private endpoint connection."]
    pub status: connection_state::Status,
    #[doc = "The description for the current state of a private endpoint connection."]
    pub description: String,
    #[doc = "Actions required for a private endpoint connection."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl ConnectionState {
    pub fn new(status: connection_state::Status, description: String) -> Self {
        Self {
            status,
            description,
            actions_required: None,
        }
    }
}
pub mod connection_state {
    use super::*;
    #[doc = "The status of a private endpoint connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("Status", 1u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 3u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The description of the DigitalTwins service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsDescription {
    #[serde(flatten)]
    pub digital_twins_resource: DigitalTwinsResource,
    #[doc = "The properties of a DigitalTwinsInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DigitalTwinsProperties>,
}
impl DigitalTwinsDescription {
    pub fn new(digital_twins_resource: DigitalTwinsResource) -> Self {
        Self {
            digital_twins_resource,
            properties: None,
        }
    }
}
#[doc = "A list of DigitalTwins description objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsDescriptionListResult {
    #[doc = "The link used to get the next page of DigitalTwins description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of DigitalTwins description objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DigitalTwinsDescription>,
}
impl azure_core::Continuable for DigitalTwinsDescriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DigitalTwinsDescriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DigitalTwinsInstance endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsEndpointResource {
    #[serde(flatten)]
    pub external_resource: ExternalResource,
    #[doc = "Properties related to Digital Twins Endpoint"]
    pub properties: DigitalTwinsEndpointResourcePropertiesUnion,
}
impl DigitalTwinsEndpointResource {
    pub fn new(properties: DigitalTwinsEndpointResourcePropertiesUnion) -> Self {
        Self {
            external_resource: ExternalResource::default(),
            properties,
        }
    }
}
#[doc = "A list of DigitalTwinsInstance Endpoints with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsEndpointResourceListResult {
    #[doc = "The link used to get the next page of DigitalTwinsInstance Endpoints."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of DigitalTwinsInstance Endpoints."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DigitalTwinsEndpointResource>,
}
impl azure_core::Continuable for DigitalTwinsEndpointResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DigitalTwinsEndpointResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties related to Digital Twins Endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsEndpointResourceProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<digital_twins_endpoint_resource_properties::ProvisioningState>,
    #[doc = "Time when the Endpoint was added to DigitalTwinsInstance."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Specifies the authentication type being used for connecting to the endpoint."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<digital_twins_endpoint_resource_properties::AuthenticationType>,
    #[doc = "Dead letter storage secret for key-based authentication. Will be obfuscated during read."]
    #[serde(rename = "deadLetterSecret", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_secret: Option<String>,
    #[doc = "Dead letter storage URL for identity-based authentication."]
    #[serde(rename = "deadLetterUri", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_uri: Option<String>,
}
impl DigitalTwinsEndpointResourceProperties {
    pub fn new() -> Self {
        Self {
            provisioning_state: None,
            created_time: None,
            authentication_type: None,
            dead_letter_secret: None,
            dead_letter_uri: None,
        }
    }
}
pub mod digital_twins_endpoint_resource_properties {
    use super::*;
    #[doc = "The provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Provisioning,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
        Deleted,
        Warning,
        Suspending,
        Restoring,
        Moving,
        Disabled,
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
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Provisioning"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleted"),
                Self::Warning => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Warning"),
                Self::Suspending => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Suspending"),
                Self::Restoring => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Restoring"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Moving"),
                Self::Disabled => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the authentication type being used for connecting to the endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        KeyBased,
        IdentityBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyBased => serializer.serialize_unit_variant("AuthenticationType", 0u32, "KeyBased"),
                Self::IdentityBased => serializer.serialize_unit_variant("AuthenticationType", 1u32, "IdentityBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The type of Digital Twins endpoint"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "endpointType")]
pub enum DigitalTwinsEndpointResourcePropertiesUnion {
    EventGrid(EventGrid),
    EventHub(EventHub),
    ServiceBus(ServiceBus),
}
#[doc = "The managed identity for the DigitalTwinsInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsIdentity {
    #[doc = "The type of Managed Identity used by the DigitalTwinsInstance. Only SystemAssigned is supported."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<digital_twins_identity::Type>,
    #[doc = "The object id of the Managed Identity Resource. This will be sent to the RP from ARM via the x-ms-identity-principal-id header in the PUT request if the resource has a systemAssigned(implicit) identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the Managed Identity Resource. This will be sent to the RP from ARM via the x-ms-client-tenant-id header in the PUT request if the resource has a systemAssigned(implicit) identity"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl DigitalTwinsIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod digital_twins_identity {
    use super::*;
    #[doc = "The type of Managed Identity used by the DigitalTwinsInstance. Only SystemAssigned is supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The description of the DigitalTwins service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsPatchDescription {
    #[doc = "Instance patch properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The managed identity for the DigitalTwinsInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<DigitalTwinsIdentity>,
    #[doc = "The properties of a DigitalTwinsInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DigitalTwinsPatchProperties>,
}
impl DigitalTwinsPatchDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a DigitalTwinsInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsPatchProperties {
    #[doc = "Public network access for the DigitalTwinsInstance."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<digital_twins_patch_properties::PublicNetworkAccess>,
}
impl DigitalTwinsPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod digital_twins_patch_properties {
    use super::*;
    #[doc = "Public network access for the DigitalTwinsInstance."]
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
}
#[doc = "The properties of a DigitalTwinsInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsProperties {
    #[doc = "Time when DigitalTwinsInstance was created."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Time when DigitalTwinsInstance was updated."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<digital_twins_properties::ProvisioningState>,
    #[doc = "Api endpoint to work with DigitalTwinsInstance."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Public network access for the DigitalTwinsInstance."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<digital_twins_properties::PublicNetworkAccess>,
}
impl DigitalTwinsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod digital_twins_properties {
    use super::*;
    #[doc = "The provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Provisioning,
        Deleting,
        Updating,
        Succeeded,
        Failed,
        Canceled,
        Deleted,
        Warning,
        Suspending,
        Restoring,
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
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Provisioning"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleted"),
                Self::Warning => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Warning"),
                Self::Suspending => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Suspending"),
                Self::Restoring => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Restoring"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Moving"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Public network access for the DigitalTwinsInstance."]
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
}
#[doc = "The common properties of a DigitalTwinsInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The managed identity for the DigitalTwinsInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<DigitalTwinsIdentity>,
}
impl DigitalTwinsResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            identity: None,
        }
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "Properties related to EventGrid."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGrid {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[doc = "EventGrid Topic Endpoint"]
    #[serde(rename = "TopicEndpoint")]
    pub topic_endpoint: String,
    #[doc = "EventGrid secondary accesskey. Will be obfuscated during read."]
    #[serde(rename = "accessKey1")]
    pub access_key1: String,
    #[doc = "EventGrid secondary accesskey. Will be obfuscated during read."]
    #[serde(rename = "accessKey2", default, skip_serializing_if = "Option::is_none")]
    pub access_key2: Option<String>,
}
impl EventGrid {
    pub fn new(
        digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
        topic_endpoint: String,
        access_key1: String,
    ) -> Self {
        Self {
            digital_twins_endpoint_resource_properties,
            topic_endpoint,
            access_key1,
            access_key2: None,
        }
    }
}
#[doc = "Properties related to EventHub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHub {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[doc = "PrimaryConnectionString of the endpoint for key-based authentication. Will be obfuscated during read."]
    #[serde(rename = "connectionStringPrimaryKey", default, skip_serializing_if = "Option::is_none")]
    pub connection_string_primary_key: Option<String>,
    #[doc = "SecondaryConnectionString of the endpoint for key-based authentication. Will be obfuscated during read."]
    #[serde(rename = "connectionStringSecondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub connection_string_secondary_key: Option<String>,
    #[doc = "The URL of the EventHub namespace for identity-based authentication. It must include the protocol sb://"]
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    #[doc = "The EventHub name in the EventHub namespace for identity-based authentication."]
    #[serde(rename = "entityPath", default, skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<String>,
}
impl EventHub {
    pub fn new(digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties) -> Self {
        Self {
            digital_twins_endpoint_resource_properties,
            connection_string_primary_key: None,
            connection_string_secondary_key: None,
            endpoint_uri: None,
            entity_path: None,
        }
    }
}
#[doc = "Definition of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Extension resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ExternalResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type GroupId = String;
#[doc = "The group information for creating a private endpoint on Digital Twin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupIdInformation {
    pub properties: serde_json::Value,
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl GroupIdInformation {
    pub fn new(properties: serde_json::Value) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "The properties for a group information object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupIdInformationProperties {
    #[doc = "The group id"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The required members for a specific group id."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The required DNS zones for a specific group id."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl GroupIdInformationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The available private link resources for a Digital Twin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupIdInformationResponse {
    #[doc = "The list of available private link resources for a Digital Twin."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GroupIdInformation>,
}
impl GroupIdInformationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DigitalTwins service REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{read | write | action | delete}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "If the operation is a data action (for data plane rbac)."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft DigitalTwins"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource Type: DigitalTwinsInstances"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly description for the operation,"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of DigitalTwins service operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The link used to get the next page of DigitalTwins description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of DigitalTwins operations supported by the Microsoft.DigitalTwins resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
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
#[doc = "The private endpoint property of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection of a Digital Twin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub properties: serde_json::Value,
}
impl PrivateEndpointConnection {
    pub fn new(properties: serde_json::Value) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "The available private link connections for a Digital Twin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionsResponse {
    #[doc = "The list of available private link connections for a Digital Twin."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties related to ServiceBus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBus {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[doc = "PrimaryConnectionString of the endpoint for key-based authentication. Will be obfuscated during read."]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "SecondaryConnectionString of the endpoint for key-based authentication. Will be obfuscated during read."]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[doc = "The URL of the ServiceBus namespace for identity-based authentication. It must include the protocol sb://"]
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    #[doc = "The ServiceBus Topic name for identity-based authentication"]
    #[serde(rename = "entityPath", default, skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<String>,
}
impl ServiceBus {
    pub fn new(digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties) -> Self {
        Self {
            digital_twins_endpoint_resource_properties,
            primary_connection_string: None,
            secondary_connection_string: None,
            endpoint_uri: None,
            entity_path: None,
        }
    }
}
