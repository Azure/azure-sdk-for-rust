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
    #[doc = "Definition of the client authentication mechanism to the server."]
    #[serde(rename = "userAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub user_authentication: Option<UserAuthentication>,
    #[doc = "Definition of the authentication mechanism for the southbound connector."]
    #[serde(rename = "transportAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub transport_authentication: Option<TransportAuthentication>,
    #[doc = "Stringified JSON that contains connectivity type specific further configuration (e.g. OPC UA, Modbus, ONVIF)."]
    #[serde(rename = "additionalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl AssetEndpointProfileProperties {
    pub fn new(target_address: String) -> Self {
        Self {
            uuid: None,
            target_address,
            user_authentication: None,
            transport_authentication: None,
            additional_configuration: None,
            provisioning_state: None,
        }
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
    #[doc = "Definition of the client authentication mechanism to the server."]
    #[serde(rename = "userAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub user_authentication: Option<UserAuthenticationUpdate>,
    #[doc = "Definition of the authentication mechanism for the southbound connector."]
    #[serde(rename = "transportAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub transport_authentication: Option<TransportAuthenticationUpdate>,
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
    #[doc = "Resource path to asset type (model) definition."]
    #[serde(rename = "assetType", default, skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
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
    #[doc = "A reference to the asset endpoint profile (connection information) used by brokers to connect to an endpoint that provides data points for this asset. Must have the format <ModuleCR.metadata.namespace>/<ModuleCR.metadata.name>."]
    #[serde(rename = "assetEndpointProfileUri")]
    pub asset_endpoint_profile_uri: String,
    #[doc = "An integer that is incremented each time the resource is modified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
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
    #[doc = "Stringified JSON that contains protocol-specific default configuration for all data points. Each data point can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultDataPointsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_data_points_configuration: Option<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all events. Each event can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultEventsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_events_configuration: Option<String>,
    #[doc = "Array of data points that are part of the asset. Each data point can reference an asset type capability and have per-data point configuration."]
    #[serde(
        rename = "dataPoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_points: Vec<DataPoint>,
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
    pub fn new(asset_endpoint_profile_uri: String) -> Self {
        Self {
            uuid: None,
            asset_type: None,
            enabled: None,
            external_asset_id: None,
            display_name: None,
            description: None,
            asset_endpoint_profile_uri,
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
            default_data_points_configuration: None,
            default_events_configuration: None,
            data_points: Vec::new(),
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
    pub version: Option<i32>,
}
impl AssetStatus {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Resource path to asset type (model) definition."]
    #[serde(rename = "assetType", default, skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
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
    #[doc = "Stringified JSON that contains protocol-specific default configuration for all data points. Each data point can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultDataPointsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_data_points_configuration: Option<String>,
    #[doc = "Stringified JSON that contains connector-specific default configuration for all events. Each event can have its own configuration that overrides the default settings here."]
    #[serde(rename = "defaultEventsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_events_configuration: Option<String>,
    #[doc = "Array of data points that are part of the asset. Each data point can reference an asset type capability and have per-data point configuration."]
    #[serde(
        rename = "dataPoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_points: Vec<DataPoint>,
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
#[doc = "Defines the data point properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPoint {
    #[serde(flatten)]
    pub data_point_base: DataPointBase,
}
impl DataPoint {
    pub fn new(data_point_base: DataPointBase) -> Self {
        Self { data_point_base }
    }
}
#[doc = "Defines the data point properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPointBase {
    #[doc = "The name of the data point."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The address of the source of the data in the asset (e.g. URL) so that a client can access the data source on the asset."]
    #[serde(rename = "dataSource")]
    pub data_source: String,
    #[doc = "The path to the type definition of the capability (e.g. DTMI, OPC UA information model node id, etc.), for example dtmi:com:example:Robot:_contents:__prop1;1."]
    #[serde(rename = "capabilityId", default, skip_serializing_if = "Option::is_none")]
    pub capability_id: Option<String>,
    #[doc = "An indication of how the data point should be mapped to OpenTelemetry."]
    #[serde(rename = "observabilityMode", default, skip_serializing_if = "Option::is_none")]
    pub observability_mode: Option<data_point_base::ObservabilityMode>,
    #[doc = "Stringified JSON that contains connector-specific configuration for the data point. For OPC UA, this could include configuration like, publishingInterval, samplingInterval, and queueSize."]
    #[serde(rename = "dataPointConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub data_point_configuration: Option<String>,
}
impl DataPointBase {
    pub fn new(data_source: String) -> Self {
        Self {
            name: None,
            data_source,
            capability_id: None,
            observability_mode: None,
            data_point_configuration: None,
        }
    }
}
pub mod data_point_base {
    use super::*;
    #[doc = "An indication of how the data point should be mapped to OpenTelemetry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObservabilityMode")]
    pub enum ObservabilityMode {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "counter")]
        Counter,
        #[serde(rename = "gauge")]
        Gauge,
        #[serde(rename = "histogram")]
        Histogram,
        #[serde(rename = "log")]
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
                Self::None => serializer.serialize_unit_variant("ObservabilityMode", 0u32, "none"),
                Self::Counter => serializer.serialize_unit_variant("ObservabilityMode", 1u32, "counter"),
                Self::Gauge => serializer.serialize_unit_variant("ObservabilityMode", 2u32, "gauge"),
                Self::Histogram => serializer.serialize_unit_variant("ObservabilityMode", 3u32, "histogram"),
                Self::Log => serializer.serialize_unit_variant("ObservabilityMode", 4u32, "log"),
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
}
impl Event {
    pub fn new(event_base: EventBase) -> Self {
        Self { event_base }
    }
}
#[doc = "Defines the event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventBase {
    #[doc = "The name of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The address of the notifier of the event in the asset (e.g. URL) so that a client can access the event on the asset."]
    #[serde(rename = "eventNotifier")]
    pub event_notifier: String,
    #[doc = "The path to the type definition of the capability (e.g. DTMI, OPC UA information model node id, etc.), for example dtmi:com:example:Robot:_contents:__prop1;1."]
    #[serde(rename = "capabilityId", default, skip_serializing_if = "Option::is_none")]
    pub capability_id: Option<String>,
    #[doc = "An indication of how the event should be mapped to OpenTelemetry."]
    #[serde(rename = "observabilityMode", default, skip_serializing_if = "Option::is_none")]
    pub observability_mode: Option<event_base::ObservabilityMode>,
    #[doc = "Stringified JSON that contains connector-specific configuration for the event. For OPC UA, this could include configuration like, publishingInterval, samplingInterval, and queueSize."]
    #[serde(rename = "eventConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub event_configuration: Option<String>,
}
impl EventBase {
    pub fn new(event_notifier: String) -> Self {
        Self {
            name: None,
            event_notifier,
            capability_id: None,
            observability_mode: None,
            event_configuration: None,
        }
    }
}
pub mod event_base {
    use super::*;
    #[doc = "An indication of how the event should be mapped to OpenTelemetry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObservabilityMode")]
    pub enum ObservabilityMode {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "log")]
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
                Self::None => serializer.serialize_unit_variant("ObservabilityMode", 0u32, "none"),
                Self::Log => serializer.serialize_unit_variant("ObservabilityMode", 1u32, "log"),
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
#[doc = "Certificate or private key that can be used by the southbound connector connecting to the shop floor/OT device. The accepted extensions are .der for certificates and .pfx/.pem for private keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OwnCertificate {
    #[doc = "Certificate thumbprint."]
    #[serde(rename = "certThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub cert_thumbprint: Option<String>,
    #[doc = "Secret Reference name (cert and private key)."]
    #[serde(rename = "certSecretReference", default, skip_serializing_if = "Option::is_none")]
    pub cert_secret_reference: Option<String>,
    #[doc = "Secret Reference Name (Pfx or Pem password)."]
    #[serde(rename = "certPasswordReference", default, skip_serializing_if = "Option::is_none")]
    pub cert_password_reference: Option<String>,
}
impl OwnCertificate {
    pub fn new() -> Self {
        Self::default()
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
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
#[doc = "Definition of the authentication mechanism for the southbound connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransportAuthentication {
    #[doc = "Defines a reference to a secret which contains all certificates and private keys that can be used by the southbound connector connecting to the shop floor/OT device. The accepted extensions are .der for certificates and .pfx/.pem for private keys."]
    #[serde(rename = "ownCertificates")]
    pub own_certificates: Vec<OwnCertificate>,
}
impl TransportAuthentication {
    pub fn new(own_certificates: Vec<OwnCertificate>) -> Self {
        Self { own_certificates }
    }
}
#[doc = "Definition of the authentication mechanism for the southbound connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransportAuthenticationUpdate {
    #[doc = "Defines a reference to a secret which contains all certificates and private keys that can be used by the southbound connector connecting to the shop floor/OT device. The accepted extensions are .der for certificates and .pfx/.pem for private keys."]
    #[serde(
        rename = "ownCertificates",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub own_certificates: Vec<OwnCertificate>,
}
impl TransportAuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the client authentication mechanism to the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAuthentication {
    #[doc = "Defines the method to authenticate the user of the client at the server."]
    pub mode: user_authentication::Mode,
    #[doc = "The credentials for authentication mode UsernamePassword."]
    #[serde(rename = "usernamePasswordCredentials", default, skip_serializing_if = "Option::is_none")]
    pub username_password_credentials: Option<UsernamePasswordCredentials>,
    #[doc = "The x509 certificate for authentication mode Certificate."]
    #[serde(rename = "x509Credentials", default, skip_serializing_if = "Option::is_none")]
    pub x509_credentials: Option<X509Credentials>,
}
impl UserAuthentication {
    pub fn new(mode: user_authentication::Mode) -> Self {
        Self {
            mode,
            username_password_credentials: None,
            x509_credentials: None,
        }
    }
}
pub mod user_authentication {
    use super::*;
    #[doc = "Defines the method to authenticate the user of the client at the server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Anonymous,
        Certificate,
        UsernamePassword,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Anonymous => serializer.serialize_unit_variant("Mode", 0u32, "Anonymous"),
                Self::Certificate => serializer.serialize_unit_variant("Mode", 1u32, "Certificate"),
                Self::UsernamePassword => serializer.serialize_unit_variant("Mode", 2u32, "UsernamePassword"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Mode {
        fn default() -> Self {
            Self::Certificate
        }
    }
}
#[doc = "Definition of the client authentication mechanism to the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAuthenticationUpdate {
    #[doc = "Defines the method to authenticate the user of the client at the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<user_authentication_update::Mode>,
    #[doc = "The credentials for authentication mode UsernamePassword."]
    #[serde(rename = "usernamePasswordCredentials", default, skip_serializing_if = "Option::is_none")]
    pub username_password_credentials: Option<UsernamePasswordCredentialsUpdate>,
    #[doc = "The x509 certificate for authentication mode Certificate."]
    #[serde(rename = "x509Credentials", default, skip_serializing_if = "Option::is_none")]
    pub x509_credentials: Option<X509CredentialsUpdate>,
}
impl UserAuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod user_authentication_update {
    use super::*;
    #[doc = "Defines the method to authenticate the user of the client at the server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Anonymous,
        Certificate,
        UsernamePassword,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Anonymous => serializer.serialize_unit_variant("Mode", 0u32, "Anonymous"),
                Self::Certificate => serializer.serialize_unit_variant("Mode", 1u32, "Certificate"),
                Self::UsernamePassword => serializer.serialize_unit_variant("Mode", 2u32, "UsernamePassword"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Mode {
        fn default() -> Self {
            Self::Certificate
        }
    }
}
#[doc = "The credentials for authentication mode UsernamePassword."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsernamePasswordCredentials {
    #[doc = "A reference to secret containing the username."]
    #[serde(rename = "usernameReference")]
    pub username_reference: String,
    #[doc = "A reference to secret containing the password."]
    #[serde(rename = "passwordReference")]
    pub password_reference: String,
}
impl UsernamePasswordCredentials {
    pub fn new(username_reference: String, password_reference: String) -> Self {
        Self {
            username_reference,
            password_reference,
        }
    }
}
#[doc = "The credentials for authentication mode UsernamePassword."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsernamePasswordCredentialsUpdate {
    #[doc = "A reference to secret containing the username."]
    #[serde(rename = "usernameReference", default, skip_serializing_if = "Option::is_none")]
    pub username_reference: Option<String>,
    #[doc = "A reference to secret containing the password."]
    #[serde(rename = "passwordReference", default, skip_serializing_if = "Option::is_none")]
    pub password_reference: Option<String>,
}
impl UsernamePasswordCredentialsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The x509 certificate for authentication mode Certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509Credentials {
    #[doc = "A reference to secret containing the certificate and private key (e.g. stored as .der/.pem or .der/.pfx)."]
    #[serde(rename = "certificateReference")]
    pub certificate_reference: String,
}
impl X509Credentials {
    pub fn new(certificate_reference: String) -> Self {
        Self { certificate_reference }
    }
}
#[doc = "The x509 certificate for authentication mode Certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509CredentialsUpdate {
    #[doc = "A reference to secret containing the certificate and private key (e.g. stored as .der/.pem or .der/.pfx)."]
    #[serde(rename = "certificateReference", default, skip_serializing_if = "Option::is_none")]
    pub certificate_reference: Option<String>,
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
