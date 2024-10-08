#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Asset definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the asset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssetProperties>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl Asset {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "Asset Endpoint Profile definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetEndpointProfile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the Asset Endpoint Profile properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssetEndpointProfileProperties>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl AssetEndpointProfile {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a AssetEndpointProfile list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetEndpointProfileListResult {
    #[doc = "The AssetEndpointProfile items on this page"]
    pub value: Vec<AssetEndpointProfile>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssetEndpointProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssetEndpointProfileListResult {
    pub fn new(value: Vec<AssetEndpointProfile>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Defines the Asset Endpoint Profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetEndpointProfileProperties {
    #[doc = "Globally unique, immutable, non-reusable id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The local valid URI specifying the network address/DNS name of a southbound device. The scheme part of the targetAddress URI specifies the type of the device. The additionalConfiguration field holds further connector type specific configuration."]
    #[serde(rename = "targetAddress")]
    pub target_address: String,
    #[doc = "Defines the configuration for the connector type that is being used with the endpoint profile."]
    #[serde(rename = "endpointProfileType")]
    pub endpoint_profile_type: String,
    #[doc = "Definition of the client authentication mechanism to the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Authentication>,
    #[doc = "Stringified JSON that contains connectivity type specific further configuration (e.g. OPC UA, Modbus, ONVIF)."]
    #[serde(rename = "additionalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<String>,
    #[doc = "Reference to a discovered asset endpoint profile. Populated only if the asset endpoint profile has been created from discovery flow. Discovered asset endpoint profile name must be provided."]
    #[serde(rename = "discoveredAssetEndpointProfileRef", default, skip_serializing_if = "Option::is_none")]
    pub discovered_asset_endpoint_profile_ref: Option<String>,
    #[doc = "Defines the asset endpoint profile status properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssetEndpointProfileStatus>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl AssetEndpointProfileProperties {
    pub fn new(target_address: String, endpoint_profile_type: String) -> Self {
        Self {
            uuid: None,
            target_address,
            endpoint_profile_type,
            authentication: None,
            additional_configuration: None,
            discovered_asset_endpoint_profile_ref: None,
            status: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Defines the asset endpoint profile status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetEndpointProfileStatus {
    #[doc = "Array object to transfer and persist errors that originate from the Edge."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<AssetEndpointProfileStatusError>,
}
impl AssetEndpointProfileStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the asset endpoint profile status error properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetEndpointProfileStatusError {
    #[doc = "Error code for classification of errors (ex: 400, 404, 500, etc.)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[doc = "Human readable helpful error message to provide additional context for error (ex: “targetAddress 'foo' is not a valid url”)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl AssetEndpointProfileStatusError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the AssetEndpointProfile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetEndpointProfileUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the AssetEndpointProfile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssetEndpointProfileUpdateProperties>,
}
impl AssetEndpointProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the AssetEndpointProfile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetEndpointProfileUpdateProperties {
    #[doc = "The local valid URI specifying the network address/DNS name of a southbound device. The scheme part of the targetAddress URI specifies the type of the device. The additionalConfiguration field holds further connector type specific configuration."]
    #[serde(rename = "targetAddress", default, skip_serializing_if = "Option::is_none")]
    pub target_address: Option<String>,
    #[doc = "Defines the configuration for the connector type that is being used with the endpoint profile."]
    #[serde(rename = "endpointProfileType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_profile_type: Option<String>,
    #[doc = "Definition of the client authentication mechanism to the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<AuthenticationUpdate>,
    #[doc = "Stringified JSON that contains connectivity type specific further configuration (e.g. OPC UA, Modbus, ONVIF)."]
    #[serde(rename = "additionalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<String>,
}
impl AssetEndpointProfileUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Asset list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetListResult {
    #[doc = "The Asset items on this page"]
    pub value: Vec<Asset>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssetListResult {
    pub fn new(value: Vec<Asset>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Defines the asset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetProperties {
    #[doc = "Globally unique, immutable, non-reusable id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Enabled/Disabled status of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Asset id provided by the customer."]
    #[serde(rename = "externalAssetId", default, skip_serializing_if = "Option::is_none")]
    pub external_asset_id: Option<String>,
    #[doc = "Human-readable display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Human-readable description of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A reference to the asset endpoint profile (connection information) used by brokers to connect to an endpoint that provides data points for this asset. Must provide asset endpoint profile name."]
    #[serde(rename = "assetEndpointProfileRef")]
    pub asset_endpoint_profile_ref: String,
    #[doc = "An integer that is incremented each time the resource is modified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "Asset manufacturer name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[doc = "Asset manufacturer URI."]
    #[serde(rename = "manufacturerUri", default, skip_serializing_if = "Option::is_none")]
    pub manufacturer_uri: Option<String>,
    #[doc = "Asset model name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "Asset product code."]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "Revision number of the hardware."]
    #[serde(rename = "hardwareRevision", default, skip_serializing_if = "Option::is_none")]
    pub hardware_revision: Option<String>,
    #[doc = "Revision number of the software."]
    #[serde(rename = "softwareRevision", default, skip_serializing_if = "Option::is_none")]
    pub software_revision: Option<String>,
    #[doc = "Reference to the documentation."]
    #[serde(rename = "documentationUri", default, skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,
    #[doc = "Asset serial number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "A set of key-value pairs that contain custom attributes set by the customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "Reference to a list of discovered assets. Populated only if the asset has been created from discovery flow. Discovered asset names must be provided."]
    #[serde(
        rename = "discoveredAssetRefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub discovered_asset_refs: Vec<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all datasets. Each dataset can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultDatasetsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_datasets_configuration: Option<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all events. Each event can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultEventsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_events_configuration: Option<String>,
    #[doc = "Object that describes the topic information."]
    #[serde(rename = "defaultTopic", default, skip_serializing_if = "Option::is_none")]
    pub default_topic: Option<Topic>,
    #[doc = "Array of datasets that are part of the asset. Each dataset describes the data points that make up the set."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub datasets: Vec<Dataset>,
    #[doc = "Array of events that are part of the asset. Each event can have per-event configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<Event>,
    #[doc = "Defines the asset status properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssetStatus>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl AssetProperties {
    pub fn new(asset_endpoint_profile_ref: String) -> Self {
        Self {
            uuid: None,
            enabled: None,
            external_asset_id: None,
            display_name: None,
            description: None,
            asset_endpoint_profile_ref,
            version: None,
            manufacturer: None,
            manufacturer_uri: None,
            model: None,
            product_code: None,
            hardware_revision: None,
            software_revision: None,
            documentation_uri: None,
            serial_number: None,
            attributes: None,
            discovered_asset_refs: Vec::new(),
            default_datasets_configuration: None,
            default_events_configuration: None,
            default_topic: None,
            datasets: Vec::new(),
            events: Vec::new(),
            status: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Defines the asset status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetStatus {
    #[doc = "Array object to transfer and persist errors that originate from the Edge."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<AssetStatusError>,
    #[doc = "A read only incremental counter indicating the number of times the configuration has been modified from the perspective of the current actual (Edge) state of the Asset. Edge would be the only writer of this value and would sync back up to the cloud. In steady state, this should equal version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "Array of dataset statuses that describe the status of each dataset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub datasets: Vec<AssetStatusDataset>,
    #[doc = "Array of event statuses that describe the status of each event."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<AssetStatusEvent>,
}
impl AssetStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the asset status dataset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetStatusDataset {
    #[doc = "The name of the dataset. Must be unique within the status.datasets array. This name is used to correlate between the spec and status dataset information."]
    pub name: String,
    #[doc = "Defines the message schema reference properties."]
    #[serde(rename = "messageSchemaReference", default, skip_serializing_if = "Option::is_none")]
    pub message_schema_reference: Option<MessageSchemaReference>,
}
impl AssetStatusDataset {
    pub fn new(name: String) -> Self {
        Self {
            name,
            message_schema_reference: None,
        }
    }
}
#[doc = "Defines the asset status error properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetStatusError {
    #[doc = "Error code for classification of errors (ex: 400, 404, 500, etc.)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[doc = "Human readable helpful error message to provide additional context for error (ex: “capability Id 'foo' does not exist”)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl AssetStatusError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the asset status event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetStatusEvent {
    #[doc = "The name of the event. Must be unique within the status.events array. This name is used to correlate between the spec and status event information."]
    pub name: String,
    #[doc = "Defines the message schema reference properties."]
    #[serde(rename = "messageSchemaReference", default, skip_serializing_if = "Option::is_none")]
    pub message_schema_reference: Option<MessageSchemaReference>,
}
impl AssetStatusEvent {
    pub fn new(name: String) -> Self {
        Self {
            name,
            message_schema_reference: None,
        }
    }
}
#[doc = "The type used for update operations of the Asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the Asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssetUpdateProperties>,
}
impl AssetUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetUpdateProperties {
    #[doc = "Enabled/Disabled status of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Human-readable display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Human-readable description of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Asset manufacturer name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[doc = "Asset manufacturer URI."]
    #[serde(rename = "manufacturerUri", default, skip_serializing_if = "Option::is_none")]
    pub manufacturer_uri: Option<String>,
    #[doc = "Asset model name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "Asset product code."]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "Revision number of the hardware."]
    #[serde(rename = "hardwareRevision", default, skip_serializing_if = "Option::is_none")]
    pub hardware_revision: Option<String>,
    #[doc = "Revision number of the software."]
    #[serde(rename = "softwareRevision", default, skip_serializing_if = "Option::is_none")]
    pub software_revision: Option<String>,
    #[doc = "Reference to the documentation."]
    #[serde(rename = "documentationUri", default, skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,
    #[doc = "Asset serial number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "A set of key-value pairs that contain custom attributes set by the customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all datasets. Each dataset can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultDatasetsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_datasets_configuration: Option<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all events. Each event can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultEventsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_events_configuration: Option<String>,
    #[doc = "Object that describes the topic information."]
    #[serde(rename = "defaultTopic", default, skip_serializing_if = "Option::is_none")]
    pub default_topic: Option<TopicUpdate>,
    #[doc = "Array of datasets that are part of the asset. Each dataset describes the data points that make up the set."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub datasets: Vec<Dataset>,
    #[doc = "Array of events that are part of the asset. Each event can have per-event configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<Event>,
}
impl AssetUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the client authentication mechanism to the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authentication {
    #[doc = "Defines the method to authenticate the user of the client at the server."]
    pub method: authentication::Method,
    #[doc = "The credentials for authentication mode UsernamePassword."]
    #[serde(rename = "usernamePasswordCredentials", default, skip_serializing_if = "Option::is_none")]
    pub username_password_credentials: Option<UsernamePasswordCredentials>,
    #[doc = "The x509 certificate for authentication mode Certificate."]
    #[serde(rename = "x509Credentials", default, skip_serializing_if = "Option::is_none")]
    pub x509_credentials: Option<X509Credentials>,
}
impl Authentication {
    pub fn new(method: authentication::Method) -> Self {
        Self {
            method,
            username_password_credentials: None,
            x509_credentials: None,
        }
    }
}
pub mod authentication {
    use super::*;
    #[doc = "Defines the method to authenticate the user of the client at the server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Method")]
    pub enum Method {
        Anonymous,
        Certificate,
        UsernamePassword,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Method {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Method {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Method {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Anonymous => serializer.serialize_unit_variant("Method", 0u32, "Anonymous"),
                Self::Certificate => serializer.serialize_unit_variant("Method", 1u32, "Certificate"),
                Self::UsernamePassword => serializer.serialize_unit_variant("Method", 2u32, "UsernamePassword"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Method {
        fn default() -> Self {
            Self::Certificate
        }
    }
}
#[doc = "The method to authenticate the user of the client at the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AuthenticationMethod")]
pub enum AuthenticationMethod {
    Anonymous,
    Certificate,
    UsernamePassword,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AuthenticationMethod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AuthenticationMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AuthenticationMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Anonymous => serializer.serialize_unit_variant("AuthenticationMethod", 0u32, "Anonymous"),
            Self::Certificate => serializer.serialize_unit_variant("AuthenticationMethod", 1u32, "Certificate"),
            Self::UsernamePassword => serializer.serialize_unit_variant("AuthenticationMethod", 2u32, "UsernamePassword"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Definition of the client authentication mechanism to the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthenticationUpdate {
    #[doc = "Defines the method to authenticate the user of the client at the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<authentication_update::Method>,
    #[doc = "The credentials for authentication mode UsernamePassword."]
    #[serde(rename = "usernamePasswordCredentials", default, skip_serializing_if = "Option::is_none")]
    pub username_password_credentials: Option<UsernamePasswordCredentialsUpdate>,
    #[doc = "The x509 certificate for authentication mode Certificate."]
    #[serde(rename = "x509Credentials", default, skip_serializing_if = "Option::is_none")]
    pub x509_credentials: Option<X509CredentialsUpdate>,
}
impl AuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod authentication_update {
    use super::*;
    #[doc = "Defines the method to authenticate the user of the client at the server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Method")]
    pub enum Method {
        Anonymous,
        Certificate,
        UsernamePassword,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Method {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Method {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Method {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Anonymous => serializer.serialize_unit_variant("Method", 0u32, "Anonymous"),
                Self::Certificate => serializer.serialize_unit_variant("Method", 1u32, "Certificate"),
                Self::UsernamePassword => serializer.serialize_unit_variant("Method", 2u32, "UsernamePassword"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Method {
        fn default() -> Self {
            Self::Certificate
        }
    }
}
#[doc = "billingContainer Model as Azure resource whose sole purpose is to keep track of billables resources under a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingContainer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the billingContainer properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingContainerProperties>,
    #[doc = "Resource ETag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl BillingContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a BillingContainer list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingContainerListResult {
    #[doc = "The BillingContainer items on this page"]
    pub value: Vec<BillingContainer>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingContainerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingContainerListResult {
    pub fn new(value: Vec<BillingContainer>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Defines the billingContainer properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingContainerProperties {
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl BillingContainerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the data point properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPoint {
    #[serde(flatten)]
    pub data_point_base: DataPointBase,
    #[doc = "An indication of how the data point should be mapped to OpenTelemetry."]
    #[serde(rename = "observabilityMode", default, skip_serializing_if = "Option::is_none")]
    pub observability_mode: Option<data_point::ObservabilityMode>,
}
impl DataPoint {
    pub fn new(data_point_base: DataPointBase) -> Self {
        Self {
            data_point_base,
            observability_mode: None,
        }
    }
}
pub mod data_point {
    use super::*;
    #[doc = "An indication of how the data point should be mapped to OpenTelemetry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObservabilityMode")]
    pub enum ObservabilityMode {
        None,
        Counter,
        Gauge,
        Histogram,
        Log,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ObservabilityMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ObservabilityMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ObservabilityMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ObservabilityMode", 0u32, "None"),
                Self::Counter => serializer.serialize_unit_variant("ObservabilityMode", 1u32, "Counter"),
                Self::Gauge => serializer.serialize_unit_variant("ObservabilityMode", 2u32, "Gauge"),
                Self::Histogram => serializer.serialize_unit_variant("ObservabilityMode", 3u32, "Histogram"),
                Self::Log => serializer.serialize_unit_variant("ObservabilityMode", 4u32, "Log"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ObservabilityMode {
        fn default() -> Self {
            Self::None
        }
    }
}
#[doc = "Defines the data point properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPointBase {
    #[doc = "The name of the data point."]
    pub name: String,
    #[doc = "The address of the source of the data in the asset (e.g. URL) so that a client can access the data source on the asset."]
    #[serde(rename = "dataSource")]
    pub data_source: String,
    #[doc = "Stringified JSON that contains connector-specific configuration for the data point. For OPC UA, this could include configuration like, publishingInterval, samplingInterval, and queueSize."]
    #[serde(rename = "dataPointConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub data_point_configuration: Option<String>,
}
impl DataPointBase {
    pub fn new(name: String, data_source: String) -> Self {
        Self {
            name,
            data_source,
            data_point_configuration: None,
        }
    }
}
#[doc = "Defines the dataset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    #[doc = "Name of the dataset."]
    pub name: String,
    #[doc = "Stringified JSON that contains connector-specific JSON string that describes configuration for the specific dataset."]
    #[serde(rename = "datasetConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub dataset_configuration: Option<String>,
    #[doc = "Object that describes the topic information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<Topic>,
    #[doc = "Array of data points that are part of the dataset. Each data point can have per-data point configuration."]
    #[serde(
        rename = "dataPoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_points: Vec<DataPoint>,
}
impl Dataset {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dataset_configuration: None,
            topic: None,
            data_points: Vec::new(),
        }
    }
}
#[doc = "Discovered Asset definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredAsset {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the discovered asset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiscoveredAssetProperties>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl DiscoveredAsset {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "Discovered Asset Endpoint Profile definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredAssetEndpointProfile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the Discovered Asset Endpoint Profile properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiscoveredAssetEndpointProfileProperties>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl DiscoveredAssetEndpointProfile {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a DiscoveredAssetEndpointProfile list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredAssetEndpointProfileListResult {
    #[doc = "The DiscoveredAssetEndpointProfile items on this page"]
    pub value: Vec<DiscoveredAssetEndpointProfile>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiscoveredAssetEndpointProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DiscoveredAssetEndpointProfileListResult {
    pub fn new(value: Vec<DiscoveredAssetEndpointProfile>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Defines the Discovered Asset Endpoint Profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredAssetEndpointProfileProperties {
    #[doc = "The local valid URI specifying the network address/DNS name of a southbound device. The scheme part of the targetAddress URI specifies the type of the device. The additionalConfiguration field holds further connector type specific configuration."]
    #[serde(rename = "targetAddress")]
    pub target_address: String,
    #[doc = "Stringified JSON that contains connectivity type specific further configuration (e.g. OPC UA, Modbus, ONVIF)."]
    #[serde(rename = "additionalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<String>,
    #[doc = "List of supported authentication methods supported by the target server."]
    #[serde(
        rename = "supportedAuthenticationMethods",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_authentication_methods: Vec<AuthenticationMethod>,
    #[doc = "Defines the configuration for the connector type that is being used with the endpoint profile."]
    #[serde(rename = "endpointProfileType")]
    pub endpoint_profile_type: String,
    #[doc = "Identifier used to detect changes in the asset endpoint profile."]
    #[serde(rename = "discoveryId")]
    pub discovery_id: String,
    #[doc = "An integer that is incremented each time the resource is modified."]
    pub version: i64,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DiscoveredAssetEndpointProfileProperties {
    pub fn new(target_address: String, endpoint_profile_type: String, discovery_id: String, version: i64) -> Self {
        Self {
            target_address,
            additional_configuration: None,
            supported_authentication_methods: Vec::new(),
            endpoint_profile_type,
            discovery_id,
            version,
            provisioning_state: None,
        }
    }
}
#[doc = "The type used for update operations of the DiscoveredAssetEndpointProfile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoveredAssetEndpointProfileUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the DiscoveredAssetEndpointProfile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiscoveredAssetEndpointProfileUpdateProperties>,
}
impl DiscoveredAssetEndpointProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the DiscoveredAssetEndpointProfile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoveredAssetEndpointProfileUpdateProperties {
    #[doc = "The local valid URI specifying the network address/DNS name of a southbound device. The scheme part of the targetAddress URI specifies the type of the device. The additionalConfiguration field holds further connector type specific configuration."]
    #[serde(rename = "targetAddress", default, skip_serializing_if = "Option::is_none")]
    pub target_address: Option<String>,
    #[doc = "Stringified JSON that contains connectivity type specific further configuration (e.g. OPC UA, Modbus, ONVIF)."]
    #[serde(rename = "additionalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<String>,
    #[doc = "List of supported authentication methods supported by the target server."]
    #[serde(
        rename = "supportedAuthenticationMethods",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_authentication_methods: Vec<AuthenticationMethod>,
    #[doc = "Defines the configuration for the connector type that is being used with the endpoint profile."]
    #[serde(rename = "endpointProfileType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_profile_type: Option<String>,
    #[doc = "Identifier used to detect changes in the asset endpoint profile."]
    #[serde(rename = "discoveryId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_id: Option<String>,
    #[doc = "An integer that is incremented each time the resource is modified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl DiscoveredAssetEndpointProfileUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DiscoveredAsset list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredAssetListResult {
    #[doc = "The DiscoveredAsset items on this page"]
    pub value: Vec<DiscoveredAsset>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiscoveredAssetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DiscoveredAssetListResult {
    pub fn new(value: Vec<DiscoveredAsset>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Defines the discovered asset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredAssetProperties {
    #[doc = "A reference to the asset endpoint profile (connection information) used by brokers to connect to an endpoint that provides data points for this asset. Must provide asset endpoint profile name."]
    #[serde(rename = "assetEndpointProfileRef")]
    pub asset_endpoint_profile_ref: String,
    #[doc = "Identifier used to detect changes in the asset."]
    #[serde(rename = "discoveryId")]
    pub discovery_id: String,
    #[doc = "An integer that is incremented each time the resource is modified."]
    pub version: i64,
    #[doc = "Asset manufacturer name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[doc = "Asset manufacturer URI."]
    #[serde(rename = "manufacturerUri", default, skip_serializing_if = "Option::is_none")]
    pub manufacturer_uri: Option<String>,
    #[doc = "Asset model name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "Asset product code."]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "Revision number of the hardware."]
    #[serde(rename = "hardwareRevision", default, skip_serializing_if = "Option::is_none")]
    pub hardware_revision: Option<String>,
    #[doc = "Revision number of the software."]
    #[serde(rename = "softwareRevision", default, skip_serializing_if = "Option::is_none")]
    pub software_revision: Option<String>,
    #[doc = "Reference to the documentation."]
    #[serde(rename = "documentationUri", default, skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,
    #[doc = "Asset serial number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all datasets. Each dataset can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultDatasetsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_datasets_configuration: Option<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all events. Each event can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultEventsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_events_configuration: Option<String>,
    #[doc = "Object that describes the topic information."]
    #[serde(rename = "defaultTopic", default, skip_serializing_if = "Option::is_none")]
    pub default_topic: Option<Topic>,
    #[doc = "Array of datasets that are part of the asset. Each dataset spec describes the data points that make up the set."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub datasets: Vec<DiscoveredDataset>,
    #[doc = "Array of events that are part of the asset. Each event can have per-event configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<DiscoveredEvent>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DiscoveredAssetProperties {
    pub fn new(asset_endpoint_profile_ref: String, discovery_id: String, version: i64) -> Self {
        Self {
            asset_endpoint_profile_ref,
            discovery_id,
            version,
            manufacturer: None,
            manufacturer_uri: None,
            model: None,
            product_code: None,
            hardware_revision: None,
            software_revision: None,
            documentation_uri: None,
            serial_number: None,
            default_datasets_configuration: None,
            default_events_configuration: None,
            default_topic: None,
            datasets: Vec::new(),
            events: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "The type used for update operations of the DiscoveredAsset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoveredAssetUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the DiscoveredAsset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiscoveredAssetUpdateProperties>,
}
impl DiscoveredAssetUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the DiscoveredAsset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoveredAssetUpdateProperties {
    #[doc = "Identifier used to detect changes in the asset."]
    #[serde(rename = "discoveryId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_id: Option<String>,
    #[doc = "An integer that is incremented each time the resource is modified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "Asset manufacturer name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[doc = "Asset manufacturer URI."]
    #[serde(rename = "manufacturerUri", default, skip_serializing_if = "Option::is_none")]
    pub manufacturer_uri: Option<String>,
    #[doc = "Asset model name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "Asset product code."]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "Revision number of the hardware."]
    #[serde(rename = "hardwareRevision", default, skip_serializing_if = "Option::is_none")]
    pub hardware_revision: Option<String>,
    #[doc = "Revision number of the software."]
    #[serde(rename = "softwareRevision", default, skip_serializing_if = "Option::is_none")]
    pub software_revision: Option<String>,
    #[doc = "Reference to the documentation."]
    #[serde(rename = "documentationUri", default, skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,
    #[doc = "Asset serial number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all datasets. Each dataset can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultDatasetsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_datasets_configuration: Option<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all events. Each event can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultEventsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_events_configuration: Option<String>,
    #[doc = "Object that describes the topic information."]
    #[serde(rename = "defaultTopic", default, skip_serializing_if = "Option::is_none")]
    pub default_topic: Option<TopicUpdate>,
    #[doc = "Array of datasets that are part of the asset. Each dataset spec describes the data points that make up the set."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub datasets: Vec<DiscoveredDataset>,
    #[doc = "Array of events that are part of the asset. Each event can have per-event configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<DiscoveredEvent>,
}
impl DiscoveredAssetUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the data point properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredDataPoint {
    #[doc = "The name of the data point."]
    pub name: String,
    #[doc = "The address of the source of the data in the asset (e.g. URL) so that a client can access the data source on the asset."]
    #[serde(rename = "dataSource")]
    pub data_source: String,
    #[doc = "Stringified JSON that contains connector-specific configuration for the data point. For OPC UA, this could include configuration like, publishingInterval, samplingInterval, and queueSize."]
    #[serde(rename = "dataPointConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub data_point_configuration: Option<String>,
    #[doc = "UTC timestamp indicating when the data point was added or modified."]
    #[serde(rename = "lastUpdatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_on: Option<::time::OffsetDateTime>,
}
impl DiscoveredDataPoint {
    pub fn new(name: String, data_source: String) -> Self {
        Self {
            name,
            data_source,
            data_point_configuration: None,
            last_updated_on: None,
        }
    }
}
#[doc = "Defines the dataset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredDataset {
    #[doc = "Name of the dataset."]
    pub name: String,
    #[doc = "Stringified JSON that contains connector-specific properties that describes configuration for the specific dataset."]
    #[serde(rename = "datasetConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub dataset_configuration: Option<String>,
    #[doc = "Object that describes the topic information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<Topic>,
    #[doc = "Array of data points that are part of the dataset. Each data point can have per-data point configuration."]
    #[serde(
        rename = "dataPoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_points: Vec<DiscoveredDataPoint>,
}
impl DiscoveredDataset {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dataset_configuration: None,
            topic: None,
            data_points: Vec::new(),
        }
    }
}
#[doc = "Defines the event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredEvent {
    #[doc = "The name of the event."]
    pub name: String,
    #[doc = "The address of the notifier of the event in the asset (e.g. URL) so that a client can access the event on the asset."]
    #[serde(rename = "eventNotifier")]
    pub event_notifier: String,
    #[doc = "Stringified JSON that contains connector-specific configuration for the event. For OPC UA, this could include configuration like, publishingInterval, samplingInterval, and queueSize."]
    #[serde(rename = "eventConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub event_configuration: Option<String>,
    #[doc = "Object that describes the topic information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<Topic>,
    #[doc = "UTC timestamp indicating when the event was added or modified."]
    #[serde(rename = "lastUpdatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_on: Option<::time::OffsetDateTime>,
}
impl DiscoveredEvent {
    pub fn new(name: String, event_notifier: String) -> Self {
        Self {
            name,
            event_notifier,
            event_configuration: None,
            topic: None,
            last_updated_on: None,
        }
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
#[doc = "Defines the event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub event_base: EventBase,
    #[doc = "An indication of how the event should be mapped to OpenTelemetry."]
    #[serde(rename = "observabilityMode", default, skip_serializing_if = "Option::is_none")]
    pub observability_mode: Option<event::ObservabilityMode>,
}
impl Event {
    pub fn new(event_base: EventBase) -> Self {
        Self {
            event_base,
            observability_mode: None,
        }
    }
}
pub mod event {
    use super::*;
    #[doc = "An indication of how the event should be mapped to OpenTelemetry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObservabilityMode")]
    pub enum ObservabilityMode {
        None,
        Log,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ObservabilityMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ObservabilityMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ObservabilityMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ObservabilityMode", 0u32, "None"),
                Self::Log => serializer.serialize_unit_variant("ObservabilityMode", 1u32, "Log"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ObservabilityMode {
        fn default() -> Self {
            Self::None
        }
    }
}
#[doc = "Defines the event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventBase {
    #[doc = "The name of the event."]
    pub name: String,
    #[doc = "The address of the notifier of the event in the asset (e.g. URL) so that a client can access the event on the asset."]
    #[serde(rename = "eventNotifier")]
    pub event_notifier: String,
    #[doc = "Stringified JSON that contains connector-specific configuration for the event. For OPC UA, this could include configuration like, publishingInterval, samplingInterval, and queueSize."]
    #[serde(rename = "eventConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub event_configuration: Option<String>,
    #[doc = "Object that describes the topic information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<Topic>,
}
impl EventBase {
    pub fn new(name: String, event_notifier: String) -> Self {
        Self {
            name,
            event_notifier,
            event_configuration: None,
            topic: None,
        }
    }
}
#[doc = "The extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedLocation {
    #[doc = "The extended location type."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The extended location name."]
    pub name: String,
}
impl ExtendedLocation {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Defines the schema format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Format")]
pub enum Format {
    #[serde(rename = "JsonSchema/draft-07")]
    JsonSchemaDraft07,
    #[serde(rename = "Delta/1.0")]
    Delta10,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Format {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Format {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::JsonSchemaDraft07 => serializer.serialize_unit_variant("Format", 0u32, "JsonSchema/draft-07"),
            Self::Delta10 => serializer.serialize_unit_variant("Format", 1u32, "Delta/1.0"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the message schema reference properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageSchemaReference {
    #[doc = "The message schema registry namespace."]
    #[serde(rename = "schemaRegistryNamespace")]
    pub schema_registry_namespace: String,
    #[doc = "The message schema name."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    #[doc = "The message schema version."]
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
}
impl MessageSchemaReference {
    pub fn new(schema_registry_namespace: String, schema_name: String, schema_version: String) -> Self {
        Self {
            schema_registry_namespace,
            schema_name,
            schema_version,
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
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusResult {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Fully qualified ID of the resource against which the original async operation was started."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status."]
    pub status: String,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<OperationStatusResult>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationStatusResult {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            resource_id: None,
            name: None,
            status,
            percent_complete: None,
            start_time: None,
            end_time: None,
            operations: Vec::new(),
            error: None,
        }
    }
}
#[doc = "The provisioning status of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Accepted,
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
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Schema definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schema {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the schema properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SchemaProperties>,
}
impl Schema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Schema list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaListResult {
    #[doc = "The Schema items on this page"]
    pub value: Vec<Schema>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SchemaListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SchemaListResult {
    pub fn new(value: Vec<Schema>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Defines the schema properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaProperties {
    #[doc = "Globally unique, immutable, non-reusable id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Human-readable display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Human-readable description of the schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Defines the schema format."]
    pub format: Format,
    #[doc = "Defines the schema type."]
    #[serde(rename = "schemaType")]
    pub schema_type: SchemaType,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Schema tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SchemaProperties {
    pub fn new(format: Format, schema_type: SchemaType) -> Self {
        Self {
            uuid: None,
            display_name: None,
            description: None,
            format,
            schema_type,
            provisioning_state: None,
            tags: None,
        }
    }
}
#[doc = "Schema registry definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaRegistry {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the schema registry properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SchemaRegistryProperties>,
    #[doc = "Managed service identity (either system assigned, or none)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SystemAssignedServiceIdentity>,
}
impl SchemaRegistry {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "The response of a SchemaRegistry list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaRegistryListResult {
    #[doc = "The SchemaRegistry items on this page"]
    pub value: Vec<SchemaRegistry>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SchemaRegistryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SchemaRegistryListResult {
    pub fn new(value: Vec<SchemaRegistry>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Defines the schema registry properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaRegistryProperties {
    #[doc = "Globally unique, immutable, non-reusable id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Schema registry namespace. Uniquely identifies a schema registry within a tenant."]
    pub namespace: String,
    #[doc = "Human-readable display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Human-readable description of the schema registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Storage Account's Container URL where schemas will be stored."]
    #[serde(rename = "storageAccountContainerUrl")]
    pub storage_account_container_url: String,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SchemaRegistryProperties {
    pub fn new(namespace: String, storage_account_container_url: String) -> Self {
        Self {
            uuid: None,
            namespace,
            display_name: None,
            description: None,
            storage_account_container_url,
            provisioning_state: None,
        }
    }
}
#[doc = "The type used for update operations of the SchemaRegistry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaRegistryUpdate {
    #[doc = "Managed service identity (either system assigned, or none)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SystemAssignedServiceIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the SchemaRegistry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SchemaRegistryUpdateProperties>,
}
impl SchemaRegistryUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the SchemaRegistry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaRegistryUpdateProperties {
    #[doc = "Human-readable display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Human-readable description of the schema registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SchemaRegistryUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the schema type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SchemaType")]
pub enum SchemaType {
    MessageSchema,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SchemaType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SchemaType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SchemaType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MessageSchema => serializer.serialize_unit_variant("SchemaType", 0u32, "MessageSchema"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema version's definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the schema version properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SchemaVersionProperties>,
}
impl SchemaVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SchemaVersion list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaVersionListResult {
    #[doc = "The SchemaVersion items on this page"]
    pub value: Vec<SchemaVersion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SchemaVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SchemaVersionListResult {
    pub fn new(value: Vec<SchemaVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Defines the schema version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaVersionProperties {
    #[doc = "Globally unique, immutable, non-reusable id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Human-readable description of the schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Schema content."]
    #[serde(rename = "schemaContent")]
    pub schema_content: String,
    #[doc = "Hash of the schema content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SchemaVersionProperties {
    pub fn new(schema_content: String) -> Self {
        Self {
            uuid: None,
            description: None,
            schema_content,
            hash: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Managed service identity (either system assigned, or none)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemAssignedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (either system assigned, or none)."]
    #[serde(rename = "type")]
    pub type_: SystemAssignedServiceIdentityType,
}
impl SystemAssignedServiceIdentity {
    pub fn new(type_: SystemAssignedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
#[doc = "Type of managed service identity (either system assigned, or none)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SystemAssignedServiceIdentityType")]
pub enum SystemAssignedServiceIdentityType {
    None,
    SystemAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SystemAssignedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SystemAssignedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SystemAssignedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("SystemAssignedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("SystemAssignedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Object that describes the topic information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    #[doc = "The topic path for messages published to an MQTT broker."]
    pub path: String,
    #[doc = "When set to 'Keep', messages published to an MQTT broker will have the retain flag set. Default: 'Never'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain: Option<topic::Retain>,
}
impl Topic {
    pub fn new(path: String) -> Self {
        Self { path, retain: None }
    }
}
pub mod topic {
    use super::*;
    #[doc = "When set to 'Keep', messages published to an MQTT broker will have the retain flag set. Default: 'Never'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Retain")]
    pub enum Retain {
        Keep,
        Never,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Retain {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Retain {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Retain {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Keep => serializer.serialize_unit_variant("Retain", 0u32, "Keep"),
                Self::Never => serializer.serialize_unit_variant("Retain", 1u32, "Never"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Retain {
        fn default() -> Self {
            Self::Never
        }
    }
}
#[doc = "Object that describes the topic information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicUpdate {
    #[doc = "The topic path for messages published to an MQTT broker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "When set to 'Keep', messages published to an MQTT broker will have the retain flag set. Default: 'Never'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain: Option<topic_update::Retain>,
}
impl TopicUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod topic_update {
    use super::*;
    #[doc = "When set to 'Keep', messages published to an MQTT broker will have the retain flag set. Default: 'Never'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Retain")]
    pub enum Retain {
        Keep,
        Never,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Retain {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Retain {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Retain {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Keep => serializer.serialize_unit_variant("Retain", 0u32, "Keep"),
                Self::Never => serializer.serialize_unit_variant("Retain", 1u32, "Never"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Retain {
        fn default() -> Self {
            Self::Never
        }
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
#[doc = "The credentials for authentication mode UsernamePassword."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsernamePasswordCredentials {
    #[doc = "The name of the secret containing the username."]
    #[serde(rename = "usernameSecretName")]
    pub username_secret_name: String,
    #[doc = "The name of the secret containing the password."]
    #[serde(rename = "passwordSecretName")]
    pub password_secret_name: String,
}
impl UsernamePasswordCredentials {
    pub fn new(username_secret_name: String, password_secret_name: String) -> Self {
        Self {
            username_secret_name,
            password_secret_name,
        }
    }
}
#[doc = "The credentials for authentication mode UsernamePassword."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsernamePasswordCredentialsUpdate {
    #[doc = "The name of the secret containing the username."]
    #[serde(rename = "usernameSecretName", default, skip_serializing_if = "Option::is_none")]
    pub username_secret_name: Option<String>,
    #[doc = "The name of the secret containing the password."]
    #[serde(rename = "passwordSecretName", default, skip_serializing_if = "Option::is_none")]
    pub password_secret_name: Option<String>,
}
impl UsernamePasswordCredentialsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The x509 certificate for authentication mode Certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509Credentials {
    #[doc = "The name of the secret containing the certificate and private key (e.g. stored as .der/.pem or .der/.pfx)."]
    #[serde(rename = "certificateSecretName")]
    pub certificate_secret_name: String,
}
impl X509Credentials {
    pub fn new(certificate_secret_name: String) -> Self {
        Self { certificate_secret_name }
    }
}
#[doc = "The x509 certificate for authentication mode Certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509CredentialsUpdate {
    #[doc = "The name of the secret containing the certificate and private key (e.g. stored as .der/.pem or .der/.pfx)."]
    #[serde(rename = "certificateSecretName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_secret_name: Option<String>,
}
impl X509CredentialsUpdate {
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
