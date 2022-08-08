#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "EnabledResourceType definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnabledResourceType {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties for EnabledResourceType of a custom location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnabledResourceTypeProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl EnabledResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for EnabledResourceType of a custom location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnabledResourceTypeProperties {
    #[doc = "Cluster Extension ID"]
    #[serde(rename = "clusterExtensionId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_extension_id: Option<String>,
    #[doc = "Cluster Extension Type"]
    #[serde(rename = "extensionType", default, skip_serializing_if = "Option::is_none")]
    pub extension_type: Option<String>,
    #[doc = "Metadata of the Resource Type"]
    #[serde(rename = "typesMetadata", default, skip_serializing_if = "Vec::is_empty")]
    pub types_metadata: Vec<serde_json::Value>,
}
impl EnabledResourceTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of EnabledResourceTypes definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnabledResourceTypesListResult {
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of EnabledResourceTypes available for a customLocation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnabledResourceType>,
}
impl azure_core::Continuable for EnabledResourceTypesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EnabledResourceTypesListResult {
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
#[doc = "Custom Locations definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomLocation {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties for a custom location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomLocationProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl CustomLocation {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "The List Custom Locations operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomLocationListResult {
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Custom Locations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomLocation>,
}
impl azure_core::Continuable for CustomLocationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomLocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom Locations operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomLocationOperation {
    #[doc = "Describes the properties of a Custom Locations Operation Value Display."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<CustomLocationOperationValueDisplay>,
    #[doc = "Is this Operation a data plane operation"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The origin of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl CustomLocationOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Custom Locations Operation Value Display."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomLocationOperationValueDisplay {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl CustomLocationOperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists of Custom Locations operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomLocationOperationsList {
    #[doc = "Next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of customLocationOperation"]
    pub value: Vec<CustomLocationOperation>,
}
impl azure_core::Continuable for CustomLocationOperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomLocationOperationsList {
    pub fn new(value: Vec<CustomLocationOperation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Properties for a custom location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomLocationProperties {
    #[doc = "This is optional input that contains the authentication that should be used to generate the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<custom_location_properties::Authentication>,
    #[doc = "Contains the reference to the add-on that contains charts to deploy CRDs and operators."]
    #[serde(rename = "clusterExtensionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_extension_ids: Vec<String>,
    #[doc = "Display name for the Custom Locations location."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Connected Cluster or AKS Cluster. The Custom Locations RP will perform a checkAccess API for listAdminCredentials permissions."]
    #[serde(rename = "hostResourceId", default, skip_serializing_if = "Option::is_none")]
    pub host_resource_id: Option<String>,
    #[doc = "Type of host the Custom Locations is referencing (Kubernetes, etc...)."]
    #[serde(rename = "hostType", default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<custom_location_properties::HostType>,
    #[doc = "Kubernetes namespace that will be created on the specified cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Provisioning State for the Custom Location."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl CustomLocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_location_properties {
    use super::*;
    #[doc = "This is optional input that contains the authentication that should be used to generate the namespace."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Authentication {
        #[doc = "The type of the Custom Locations authentication"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[doc = "The kubeconfig value."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl Authentication {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Type of host the Custom Locations is referencing (Kubernetes, etc...)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostType")]
    pub enum HostType {
        Kubernetes,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Kubernetes => serializer.serialize_unit_variant("HostType", 0u32, "Kubernetes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Custom Locations patchable resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchableCustomLocations {
    #[doc = "Properties for a custom location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomLocationProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchableCustomLocations {
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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
