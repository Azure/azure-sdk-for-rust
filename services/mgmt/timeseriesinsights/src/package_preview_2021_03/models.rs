#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyCreateOrUpdateParameters {
    pub properties: AccessPolicyResourceProperties,
}
impl AccessPolicyCreateOrUpdateParameters {
    pub fn new(properties: AccessPolicyResourceProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The response of the List access policies operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyListResponse {
    #[doc = "Result of the List access policies operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessPolicyResource>,
}
impl AccessPolicyListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents a set of mutable access policy resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyMutableProperties {
    #[doc = "An description of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of roles the principal is assigned on the environment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}
impl AccessPolicyMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An access policy is used to grant users and applications access to the environment. Roles are assigned to service principals in Azure Active Directory. These roles define the actions the principal can perform through the Time Series Insights data plane APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyResourceProperties>,
}
impl AccessPolicyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyResourceProperties {
    #[doc = "The objectId of the principal in Azure Active Directory."]
    #[serde(rename = "principalObjectId", default, skip_serializing_if = "Option::is_none")]
    pub principal_object_id: Option<String>,
    #[doc = "An description of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of roles the principal is assigned on the environment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}
impl AccessPolicyResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyUpdateParameters {
    #[doc = "An object that represents a set of mutable access policy resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyMutableProperties>,
}
impl AccessPolicyUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an event source that reads events from an event broker in Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureEventSourceProperties {
    #[serde(flatten)]
    pub event_source_common_properties: EventSourceCommonProperties,
    #[doc = "The resource id of the event source in Azure Resource Manager."]
    #[serde(rename = "eventSourceResourceId")]
    pub event_source_resource_id: String,
}
impl AzureEventSourceProperties {
    pub fn new(event_source_resource_id: String) -> Self {
        Self {
            event_source_common_properties: EventSourceCommonProperties::default(),
            event_source_resource_id,
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
#[doc = "Properties required to create any resource tracked by Azure Resource Manager."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateTrackedResourceProperties {
    #[doc = "The location of the resource."]
    pub location: String,
    #[doc = "Key-value pairs of additional properties for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CreateOrUpdateTrackedResourceProperties {
    pub fn new(location: String) -> Self {
        Self { location, tags: None }
    }
}
#[doc = "Dimension of blobs, possibly be blob type or access tier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[doc = "Display name of dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the CreateOrUpdate Environment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    #[doc = "The kind of the environment."]
    pub kind: environment_create_or_update_parameters::Kind,
    #[doc = "The sku determines the type of environment, either Gen1 (S1 or S2) or Gen2 (L1). For Gen1 environments the sku determines the capacity of the environment, the ingress rate, and the billing rate."]
    pub sku: Sku,
}
impl EnvironmentCreateOrUpdateParameters {
    pub fn new(
        create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
        kind: environment_create_or_update_parameters::Kind,
        sku: Sku,
    ) -> Self {
        Self {
            create_or_update_tracked_resource_properties,
            kind,
            sku,
        }
    }
}
pub mod environment_create_or_update_parameters {
    use super::*;
    #[doc = "The kind of the environment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Gen1,
        Gen2,
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
                Self::Gen1 => serializer.serialize_unit_variant("Kind", 0u32, "Gen1"),
                Self::Gen2 => serializer.serialize_unit_variant("Kind", 1u32, "Gen2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response of the List Environments operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentListResponse {
    #[doc = "Result of the List Environments operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnvironmentResource>,
}
impl EnvironmentListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An environment is a set of time-series data available for query, and is the top level Azure Time Series Insights resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The sku determines the type of environment, either Gen1 (S1 or S2) or Gen2 (L1). For Gen1 environments the sku determines the capacity of the environment, the ingress rate, and the billing rate."]
    pub sku: Sku,
    #[doc = "The kind of the environment."]
    pub kind: environment_resource::Kind,
}
impl EnvironmentResource {
    pub fn new(tracked_resource: TrackedResource, sku: Sku, kind: environment_resource::Kind) -> Self {
        Self {
            tracked_resource,
            sku,
            kind,
        }
    }
}
pub mod environment_resource {
    use super::*;
    #[doc = "The kind of the environment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Gen1,
        Gen2,
    }
}
#[doc = "Properties of the environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentResourceProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[doc = "An id used to access the environment data, e.g. to query the environment's events or upload reference data for the environment."]
    #[serde(rename = "dataAccessId", default, skip_serializing_if = "Option::is_none")]
    pub data_access_id: Option<String>,
    #[doc = "The fully qualified domain name used to access the environment data, e.g. to query the environment's events or upload reference data for the environment."]
    #[serde(rename = "dataAccessFqdn", default, skip_serializing_if = "Option::is_none")]
    pub data_access_fqdn: Option<String>,
    #[doc = "An object that represents the status of the environment, and its internal state in the Time Series Insights service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnvironmentStatus>,
}
impl EnvironmentResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that contains the details about an environment's state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentStateDetails {
    #[doc = "Contains the code that represents the reason of an environment being in a particular state. Can be used to programmatically handle specific cases."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message that describes the state in detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl EnvironmentStateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents the status of the environment, and its internal state in the Time Series Insights service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentStatus {
    #[doc = "An object that represents the status of ingress on an environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<IngressEnvironmentStatus>,
    #[doc = "An object that represents the status of warm storage on an environment."]
    #[serde(rename = "warmStorage", default, skip_serializing_if = "Option::is_none")]
    pub warm_storage: Option<WarmStorageEnvironmentStatus>,
}
impl EnvironmentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update Environment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentUpdateParameters {
    #[doc = "The kind of the environment."]
    pub kind: environment_update_parameters::Kind,
    #[doc = "Key-value pairs of additional properties for the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl EnvironmentUpdateParameters {
    pub fn new(kind: environment_update_parameters::Kind) -> Self {
        Self { kind, tags: None }
    }
}
pub mod environment_update_parameters {
    use super::*;
    #[doc = "The kind of the environment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Gen1,
        Gen2,
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
                Self::Gen1 => serializer.serialize_unit_variant("Kind", 0u32, "Gen1"),
                Self::Gen2 => serializer.serialize_unit_variant("Kind", 1u32, "Gen2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the EventHub event source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCommonProperties {
    #[serde(flatten)]
    pub azure_event_source_properties: AzureEventSourceProperties,
    #[doc = "The name of the service bus that contains the event hub."]
    #[serde(rename = "serviceBusNamespace")]
    pub service_bus_namespace: String,
    #[doc = "The name of the event hub."]
    #[serde(rename = "eventHubName")]
    pub event_hub_name: String,
    #[doc = "The name of the event hub's consumer group that holds the partitions from which events will be read."]
    #[serde(rename = "consumerGroupName")]
    pub consumer_group_name: String,
    #[doc = "The name of the SAS key that grants the Time Series Insights service access to the event hub. The shared access policies for this key must grant 'Listen' permissions to the event hub."]
    #[serde(rename = "keyName")]
    pub key_name: String,
}
impl EventHubEventSourceCommonProperties {
    pub fn new(
        azure_event_source_properties: AzureEventSourceProperties,
        service_bus_namespace: String,
        event_hub_name: String,
        consumer_group_name: String,
        key_name: String,
    ) -> Self {
        Self {
            azure_event_source_properties,
            service_bus_namespace,
            event_hub_name,
            consumer_group_name,
            key_name,
        }
    }
}
#[doc = "Parameters supplied to the Create or Update Event Source operation for an EventHub event source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub event_source_create_or_update_parameters: EventSourceCreateOrUpdateParameters,
    #[doc = "Properties of the EventHub event source that are required on create or update requests."]
    pub properties: EventHubEventSourceCreationProperties,
}
impl EventHubEventSourceCreateOrUpdateParameters {
    pub fn new(
        event_source_create_or_update_parameters: EventSourceCreateOrUpdateParameters,
        properties: EventHubEventSourceCreationProperties,
    ) -> Self {
        Self {
            event_source_create_or_update_parameters,
            properties,
        }
    }
}
#[doc = "Properties of the EventHub event source that are required on create or update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCreationProperties {
    #[serde(flatten)]
    pub event_hub_event_source_common_properties: EventHubEventSourceCommonProperties,
    #[doc = "The value of the shared access key that grants the Time Series Insights service read access to the event hub. This property is not shown in event source responses."]
    #[serde(rename = "sharedAccessKey")]
    pub shared_access_key: String,
}
impl EventHubEventSourceCreationProperties {
    pub fn new(event_hub_event_source_common_properties: EventHubEventSourceCommonProperties, shared_access_key: String) -> Self {
        Self {
            event_hub_event_source_common_properties,
            shared_access_key,
        }
    }
}
#[doc = "An object that represents a set of mutable EventHub event source resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubEventSourceMutableProperties {
    #[serde(flatten)]
    pub event_source_mutable_properties: EventSourceMutableProperties,
    #[doc = "The value of the shared access key that grants the Time Series Insights service read access to the event hub. This property is not shown in event source responses."]
    #[serde(rename = "sharedAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_key: Option<String>,
}
impl EventHubEventSourceMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An event source that receives its data from an Azure EventHub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceResource {
    #[serde(flatten)]
    pub event_source_resource: EventSourceResource,
    #[doc = "Properties of the EventHub event source resource."]
    pub properties: EventHubEventSourceResourceProperties,
}
impl EventHubEventSourceResource {
    pub fn new(event_source_resource: EventSourceResource, properties: EventHubEventSourceResourceProperties) -> Self {
        Self {
            event_source_resource,
            properties,
        }
    }
}
#[doc = "Properties of the EventHub event source resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceResourceProperties {
    #[serde(flatten)]
    pub event_hub_event_source_common_properties: EventHubEventSourceCommonProperties,
}
impl EventHubEventSourceResourceProperties {
    pub fn new(event_hub_event_source_common_properties: EventHubEventSourceCommonProperties) -> Self {
        Self {
            event_hub_event_source_common_properties,
        }
    }
}
#[doc = "Parameters supplied to the Update Event Source operation to update an EventHub event source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceUpdateParameters {
    #[serde(flatten)]
    pub event_source_update_parameters: EventSourceUpdateParameters,
    #[doc = "An object that represents a set of mutable EventHub event source resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubEventSourceMutableProperties>,
}
impl EventHubEventSourceUpdateParameters {
    pub fn new(event_source_update_parameters: EventSourceUpdateParameters) -> Self {
        Self {
            event_source_update_parameters,
            properties: None,
        }
    }
}
#[doc = "Properties of the event source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSourceCommonProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[doc = "The event property that will be used as the event source's timestamp. If a value isn't specified for timestampPropertyName, or if null or empty-string is specified, the event creation time will be used."]
    #[serde(rename = "timestampPropertyName", default, skip_serializing_if = "Option::is_none")]
    pub timestamp_property_name: Option<String>,
    #[doc = "An object that represents the local timestamp property. It contains the format of local timestamp that needs to be used and the corresponding timezone offset information. If a value isn't specified for localTimestamp, or if null, then the local timestamp will not be ingressed with the events."]
    #[serde(rename = "localTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub local_timestamp: Option<LocalTimestamp>,
    #[doc = "An object that contains the details about the starting point in time to ingest events."]
    #[serde(rename = "ingressStartAt", default, skip_serializing_if = "Option::is_none")]
    pub ingress_start_at: Option<IngressStartAtProperties>,
}
impl EventSourceCommonProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Create or Update Event Source operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    #[doc = "The kind of the event source."]
    pub kind: event_source_create_or_update_parameters::Kind,
    #[doc = "An object that represents the local timestamp property. It contains the format of local timestamp that needs to be used and the corresponding timezone offset information. If a value isn't specified for localTimestamp, or if null, then the local timestamp will not be ingressed with the events."]
    #[serde(rename = "localTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub local_timestamp: Option<LocalTimestamp>,
}
impl EventSourceCreateOrUpdateParameters {
    pub fn new(
        create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
        kind: event_source_create_or_update_parameters::Kind,
    ) -> Self {
        Self {
            create_or_update_tracked_resource_properties,
            kind,
            local_timestamp: None,
        }
    }
}
pub mod event_source_create_or_update_parameters {
    use super::*;
    #[doc = "The kind of the event source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "Microsoft.EventHub")]
        MicrosoftEventHub,
        #[serde(rename = "Microsoft.IoTHub")]
        MicrosoftIoTHub,
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
                Self::MicrosoftEventHub => serializer.serialize_unit_variant("Kind", 0u32, "Microsoft.EventHub"),
                Self::MicrosoftIoTHub => serializer.serialize_unit_variant("Kind", 1u32, "Microsoft.IoTHub"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response of the List EventSources operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSourceListResponse {
    #[doc = "Result of the List EventSources operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventSourceResource>,
}
impl EventSourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents a set of mutable event source resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSourceMutableProperties {
    #[doc = "The event property that will be used as the event source's timestamp. If a value isn't specified for timestampPropertyName, or if null or empty-string is specified, the event creation time will be used."]
    #[serde(rename = "timestampPropertyName", default, skip_serializing_if = "Option::is_none")]
    pub timestamp_property_name: Option<String>,
}
impl EventSourceMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An environment receives data from one or more event sources. Each event source has associated connection info that allows the Time Series Insights ingress pipeline to connect to and pull data from the event source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The kind of the event source."]
    pub kind: event_source_resource::Kind,
}
impl EventSourceResource {
    pub fn new(tracked_resource: TrackedResource, kind: event_source_resource::Kind) -> Self {
        Self { tracked_resource, kind }
    }
}
pub mod event_source_resource {
    use super::*;
    #[doc = "The kind of the event source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "Microsoft.EventHub")]
        MicrosoftEventHub,
        #[serde(rename = "Microsoft.IoTHub")]
        MicrosoftIoTHub,
    }
}
#[doc = "Parameters supplied to the Update Event Source operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceUpdateParameters {
    #[doc = "The kind of the event source."]
    pub kind: event_source_update_parameters::Kind,
    #[doc = "Key-value pairs of additional properties for the event source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl EventSourceUpdateParameters {
    pub fn new(kind: event_source_update_parameters::Kind) -> Self {
        Self { kind, tags: None }
    }
}
pub mod event_source_update_parameters {
    use super::*;
    #[doc = "The kind of the event source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "Microsoft.EventHub")]
        MicrosoftEventHub,
        #[serde(rename = "Microsoft.IoTHub")]
        MicrosoftIoTHub,
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
                Self::MicrosoftEventHub => serializer.serialize_unit_variant("Kind", 0u32, "Microsoft.EventHub"),
                Self::MicrosoftIoTHub => serializer.serialize_unit_variant("Kind", 1u32, "Microsoft.IoTHub"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters supplied to the Create or Update Environment operation for a Gen1 environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentCreateOrUpdateParameters {
    #[serde(flatten)]
    pub environment_create_or_update_parameters: EnvironmentCreateOrUpdateParameters,
    #[doc = "Properties used to create a Gen1 environment."]
    pub properties: Gen1EnvironmentCreationProperties,
}
impl Gen1EnvironmentCreateOrUpdateParameters {
    pub fn new(
        environment_create_or_update_parameters: EnvironmentCreateOrUpdateParameters,
        properties: Gen1EnvironmentCreationProperties,
    ) -> Self {
        Self {
            environment_create_or_update_parameters,
            properties,
        }
    }
}
#[doc = "Properties used to create a Gen1 environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentCreationProperties {
    #[doc = "ISO8601 timespan specifying the minimum number of days the environment's events will be available for query."]
    #[serde(rename = "dataRetentionTime")]
    pub data_retention_time: String,
    #[doc = "The behavior the Time Series Insights service should take when the environment's capacity has been exceeded. If \"PauseIngress\" is specified, new events will not be read from the event source. If \"PurgeOldData\" is specified, new events will continue to be read and old events will be deleted from the environment. The default behavior is PurgeOldData."]
    #[serde(rename = "storageLimitExceededBehavior", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit_exceeded_behavior: Option<gen1_environment_creation_properties::StorageLimitExceededBehavior>,
    #[doc = "The list of event properties which will be used to partition data in the environment. Currently, only a single partition key property is supported."]
    #[serde(rename = "partitionKeyProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_key_properties: Vec<TimeSeriesIdProperty>,
}
impl Gen1EnvironmentCreationProperties {
    pub fn new(data_retention_time: String) -> Self {
        Self {
            data_retention_time,
            storage_limit_exceeded_behavior: None,
            partition_key_properties: Vec::new(),
        }
    }
}
pub mod gen1_environment_creation_properties {
    use super::*;
    #[doc = "The behavior the Time Series Insights service should take when the environment's capacity has been exceeded. If \"PauseIngress\" is specified, new events will not be read from the event source. If \"PurgeOldData\" is specified, new events will continue to be read and old events will be deleted from the environment. The default behavior is PurgeOldData."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageLimitExceededBehavior")]
    pub enum StorageLimitExceededBehavior {
        PurgeOldData,
        PauseIngress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageLimitExceededBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageLimitExceededBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageLimitExceededBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PurgeOldData => serializer.serialize_unit_variant("StorageLimitExceededBehavior", 0u32, "PurgeOldData"),
                Self::PauseIngress => serializer.serialize_unit_variant("StorageLimitExceededBehavior", 1u32, "PauseIngress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An object that represents a set of mutable Gen1 environment resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Gen1EnvironmentMutableProperties {
    #[doc = "ISO8601 timespan specifying the minimum number of days the environment's events will be available for query."]
    #[serde(rename = "dataRetentionTime", default, skip_serializing_if = "Option::is_none")]
    pub data_retention_time: Option<String>,
    #[doc = "The behavior the Time Series Insights service should take when the environment's capacity has been exceeded. If \"PauseIngress\" is specified, new events will not be read from the event source. If \"PurgeOldData\" is specified, new events will continue to be read and old events will be deleted from the environment. The default behavior is PurgeOldData."]
    #[serde(rename = "storageLimitExceededBehavior", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit_exceeded_behavior: Option<gen1_environment_mutable_properties::StorageLimitExceededBehavior>,
}
impl Gen1EnvironmentMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod gen1_environment_mutable_properties {
    use super::*;
    #[doc = "The behavior the Time Series Insights service should take when the environment's capacity has been exceeded. If \"PauseIngress\" is specified, new events will not be read from the event source. If \"PurgeOldData\" is specified, new events will continue to be read and old events will be deleted from the environment. The default behavior is PurgeOldData."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageLimitExceededBehavior")]
    pub enum StorageLimitExceededBehavior {
        PurgeOldData,
        PauseIngress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageLimitExceededBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageLimitExceededBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageLimitExceededBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PurgeOldData => serializer.serialize_unit_variant("StorageLimitExceededBehavior", 0u32, "PurgeOldData"),
                Self::PauseIngress => serializer.serialize_unit_variant("StorageLimitExceededBehavior", 1u32, "PauseIngress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An environment is a set of time-series data available for query, and is the top level Azure Time Series Insights resource. Gen1 environments have data retention limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentResource {
    #[serde(flatten)]
    pub environment_resource: EnvironmentResource,
    #[doc = "Properties of the Gen1 environment."]
    pub properties: Gen1EnvironmentResourceProperties,
}
impl Gen1EnvironmentResource {
    pub fn new(environment_resource: EnvironmentResource, properties: Gen1EnvironmentResourceProperties) -> Self {
        Self {
            environment_resource,
            properties,
        }
    }
}
#[doc = "Properties of the Gen1 environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentResourceProperties {
    #[serde(flatten)]
    pub gen1_environment_creation_properties: Gen1EnvironmentCreationProperties,
    #[serde(flatten)]
    pub environment_resource_properties: EnvironmentResourceProperties,
}
impl Gen1EnvironmentResourceProperties {
    pub fn new(gen1_environment_creation_properties: Gen1EnvironmentCreationProperties) -> Self {
        Self {
            gen1_environment_creation_properties,
            environment_resource_properties: EnvironmentResourceProperties::default(),
        }
    }
}
#[doc = "Parameters supplied to the Update Environment operation to update a Gen1 environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentUpdateParameters {
    #[serde(flatten)]
    pub environment_update_parameters: EnvironmentUpdateParameters,
    #[doc = "The sku determines the type of environment, either Gen1 (S1 or S2) or Gen2 (L1). For Gen1 environments the sku determines the capacity of the environment, the ingress rate, and the billing rate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "An object that represents a set of mutable Gen1 environment resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Gen1EnvironmentMutableProperties>,
}
impl Gen1EnvironmentUpdateParameters {
    pub fn new(environment_update_parameters: EnvironmentUpdateParameters) -> Self {
        Self {
            environment_update_parameters,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "Parameters supplied to the Create or Update Environment operation for a Gen2 environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentCreateOrUpdateParameters {
    #[serde(flatten)]
    pub environment_create_or_update_parameters: EnvironmentCreateOrUpdateParameters,
    #[doc = "Properties used to create a Gen2 environment."]
    pub properties: Gen2EnvironmentCreationProperties,
}
impl Gen2EnvironmentCreateOrUpdateParameters {
    pub fn new(
        environment_create_or_update_parameters: EnvironmentCreateOrUpdateParameters,
        properties: Gen2EnvironmentCreationProperties,
    ) -> Self {
        Self {
            environment_create_or_update_parameters,
            properties,
        }
    }
}
#[doc = "Properties used to create a Gen2 environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentCreationProperties {
    #[doc = "The list of event properties which will be used to define the environment's time series id."]
    #[serde(rename = "timeSeriesIdProperties")]
    pub time_series_id_properties: Vec<TimeSeriesIdProperty>,
    #[doc = "The storage configuration provides the connection details that allows the Time Series Insights service to connect to the customer storage account that is used to store the environment's data."]
    #[serde(rename = "storageConfiguration")]
    pub storage_configuration: Gen2StorageConfigurationInput,
    #[doc = "The warm store configuration provides the details to create a warm store cache that will retain a copy of the environment's data available for faster query."]
    #[serde(rename = "warmStoreConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub warm_store_configuration: Option<WarmStoreConfigurationProperties>,
    #[doc = "This value can be set to 'enabled' to avoid breaking changes on existing customer resources and templates. If set to 'disabled', traffic over public interface is not allowed, and private endpoint connections would be the exclusive access method."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<gen2_environment_creation_properties::PublicNetworkAccess>,
    #[doc = "The list of private endpoint connections to the environment."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl Gen2EnvironmentCreationProperties {
    pub fn new(time_series_id_properties: Vec<TimeSeriesIdProperty>, storage_configuration: Gen2StorageConfigurationInput) -> Self {
        Self {
            time_series_id_properties,
            storage_configuration,
            warm_store_configuration: None,
            public_network_access: None,
            private_endpoint_connections: Vec::new(),
        }
    }
}
pub mod gen2_environment_creation_properties {
    use super::*;
    #[doc = "This value can be set to 'enabled' to avoid breaking changes on existing customer resources and templates. If set to 'disabled', traffic over public interface is not allowed, and private endpoint connections would be the exclusive access method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
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
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "An object that represents a set of mutable Gen2 environment resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Gen2EnvironmentMutableProperties {
    #[doc = "The storage configuration provides the connection details that allows the Time Series Insights service to connect to the customer storage account that is used to store the environment's data."]
    #[serde(rename = "storageConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub storage_configuration: Option<Gen2StorageConfigurationMutableProperties>,
    #[doc = "The warm store configuration provides the details to create a warm store cache that will retain a copy of the environment's data available for faster query."]
    #[serde(rename = "warmStoreConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub warm_store_configuration: Option<WarmStoreConfigurationProperties>,
}
impl Gen2EnvironmentMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An environment is a set of time-series data available for query, and is the top level Azure Time Series Insights resource. Gen2 environments do not have set data retention limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentResource {
    #[serde(flatten)]
    pub environment_resource: EnvironmentResource,
    #[doc = "Properties of the Gen2 environment."]
    pub properties: Gen2EnvironmentResourceProperties,
}
impl Gen2EnvironmentResource {
    pub fn new(environment_resource: EnvironmentResource, properties: Gen2EnvironmentResourceProperties) -> Self {
        Self {
            environment_resource,
            properties,
        }
    }
}
#[doc = "Properties of the Gen2 environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentResourceProperties {
    #[serde(flatten)]
    pub environment_resource_properties: EnvironmentResourceProperties,
    #[doc = "The list of event properties which will be used to define the environment's time series id."]
    #[serde(rename = "timeSeriesIdProperties")]
    pub time_series_id_properties: Vec<TimeSeriesIdProperty>,
    #[doc = "The storage configuration provides the non-secret connection details about the customer storage account that is used to store the environment's data."]
    #[serde(rename = "storageConfiguration")]
    pub storage_configuration: Gen2StorageConfigurationOutput,
    #[doc = "The warm store configuration provides the details to create a warm store cache that will retain a copy of the environment's data available for faster query."]
    #[serde(rename = "warmStoreConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub warm_store_configuration: Option<WarmStoreConfigurationProperties>,
    #[doc = "If 'enabled', public network access is allowed. If 'disabled', traffic over public interface is not allowed, and private endpoint connections would be the exclusive access method."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<gen2_environment_resource_properties::PublicNetworkAccess>,
    #[doc = "The list of private endpoint connections to the environment."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl Gen2EnvironmentResourceProperties {
    pub fn new(time_series_id_properties: Vec<TimeSeriesIdProperty>, storage_configuration: Gen2StorageConfigurationOutput) -> Self {
        Self {
            environment_resource_properties: EnvironmentResourceProperties::default(),
            time_series_id_properties,
            storage_configuration,
            warm_store_configuration: None,
            public_network_access: None,
            private_endpoint_connections: Vec::new(),
        }
    }
}
pub mod gen2_environment_resource_properties {
    use super::*;
    #[doc = "If 'enabled', public network access is allowed. If 'disabled', traffic over public interface is not allowed, and private endpoint connections would be the exclusive access method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
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
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Parameters supplied to the Update Environment operation to update a Gen2 environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentUpdateParameters {
    #[serde(flatten)]
    pub environment_update_parameters: EnvironmentUpdateParameters,
    #[doc = "An object that represents a set of mutable Gen2 environment resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Gen2EnvironmentMutableProperties>,
}
impl Gen2EnvironmentUpdateParameters {
    pub fn new(environment_update_parameters: EnvironmentUpdateParameters) -> Self {
        Self {
            environment_update_parameters,
            properties: None,
        }
    }
}
#[doc = "The storage configuration provides the connection details that allows the Time Series Insights service to connect to the customer storage account that is used to store the environment's data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2StorageConfigurationInput {
    #[doc = "The name of the storage account that will hold the environment's Gen2 data."]
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "The value of the management key that grants the Time Series Insights service write access to the storage account. This property is not shown in environment responses."]
    #[serde(rename = "managementKey")]
    pub management_key: String,
}
impl Gen2StorageConfigurationInput {
    pub fn new(account_name: String, management_key: String) -> Self {
        Self {
            account_name,
            management_key,
        }
    }
}
#[doc = "The storage configuration provides the connection details that allows the Time Series Insights service to connect to the customer storage account that is used to store the environment's data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2StorageConfigurationMutableProperties {
    #[doc = "The value of the management key that grants the Time Series Insights service write access to the storage account. This property is not shown in environment responses."]
    #[serde(rename = "managementKey")]
    pub management_key: String,
}
impl Gen2StorageConfigurationMutableProperties {
    pub fn new(management_key: String) -> Self {
        Self { management_key }
    }
}
#[doc = "The storage configuration provides the non-secret connection details about the customer storage account that is used to store the environment's data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2StorageConfigurationOutput {
    #[doc = "The name of the storage account that will hold the environment's Gen2 data."]
    #[serde(rename = "accountName")]
    pub account_name: String,
}
impl Gen2StorageConfigurationOutput {
    pub fn new(account_name: String) -> Self {
        Self { account_name }
    }
}
#[doc = "An object that represents the status of ingress on an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngressEnvironmentStatus {
    #[doc = "This string represents the state of ingress operations on an environment. It can be \"Disabled\", \"Ready\", \"Running\", \"Paused\" or \"Unknown\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ingress_environment_status::State>,
    #[doc = "An object that contains the details about an environment's state."]
    #[serde(rename = "stateDetails", default, skip_serializing_if = "Option::is_none")]
    pub state_details: Option<EnvironmentStateDetails>,
}
impl IngressEnvironmentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ingress_environment_status {
    use super::*;
    #[doc = "This string represents the state of ingress operations on an environment. It can be \"Disabled\", \"Ready\", \"Running\", \"Paused\" or \"Unknown\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Disabled,
        Ready,
        Running,
        Paused,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("State", 0u32, "Disabled"),
                Self::Ready => serializer.serialize_unit_variant("State", 1u32, "Ready"),
                Self::Running => serializer.serialize_unit_variant("State", 2u32, "Running"),
                Self::Paused => serializer.serialize_unit_variant("State", 3u32, "Paused"),
                Self::Unknown => serializer.serialize_unit_variant("State", 4u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An object that contains the details about the starting point in time to ingest events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngressStartAtProperties {
    #[doc = "The type of the ingressStartAt, It can be \"EarliestAvailable\", \"EventSourceCreationTime\", \"CustomEnqueuedTime\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ingress_start_at_properties::Type>,
    #[doc = "ISO8601 UTC datetime with seconds precision (milliseconds are optional), specifying the date and time that will be the starting point for Events to be consumed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
impl IngressStartAtProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ingress_start_at_properties {
    use super::*;
    #[doc = "The type of the ingressStartAt, It can be \"EarliestAvailable\", \"EventSourceCreationTime\", \"CustomEnqueuedTime\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        EarliestAvailable,
        EventSourceCreationTime,
        CustomEnqueuedTime,
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
                Self::EarliestAvailable => serializer.serialize_unit_variant("Type", 0u32, "EarliestAvailable"),
                Self::EventSourceCreationTime => serializer.serialize_unit_variant("Type", 1u32, "EventSourceCreationTime"),
                Self::CustomEnqueuedTime => serializer.serialize_unit_variant("Type", 2u32, "CustomEnqueuedTime"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the IoTHub event source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCommonProperties {
    #[serde(flatten)]
    pub azure_event_source_properties: AzureEventSourceProperties,
    #[doc = "The name of the iot hub."]
    #[serde(rename = "iotHubName")]
    pub iot_hub_name: String,
    #[doc = "The name of the iot hub's consumer group that holds the partitions from which events will be read."]
    #[serde(rename = "consumerGroupName")]
    pub consumer_group_name: String,
    #[doc = "The name of the Shared Access Policy key that grants the Time Series Insights service access to the iot hub. This shared access policy key must grant 'service connect' permissions to the iot hub."]
    #[serde(rename = "keyName")]
    pub key_name: String,
}
impl IoTHubEventSourceCommonProperties {
    pub fn new(
        azure_event_source_properties: AzureEventSourceProperties,
        iot_hub_name: String,
        consumer_group_name: String,
        key_name: String,
    ) -> Self {
        Self {
            azure_event_source_properties,
            iot_hub_name,
            consumer_group_name,
            key_name,
        }
    }
}
#[doc = "Parameters supplied to the Create or Update Event Source operation for an IoTHub event source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub event_source_create_or_update_parameters: EventSourceCreateOrUpdateParameters,
    #[doc = "Properties of the IoTHub event source that are required on create or update requests."]
    pub properties: IoTHubEventSourceCreationProperties,
}
impl IoTHubEventSourceCreateOrUpdateParameters {
    pub fn new(
        event_source_create_or_update_parameters: EventSourceCreateOrUpdateParameters,
        properties: IoTHubEventSourceCreationProperties,
    ) -> Self {
        Self {
            event_source_create_or_update_parameters,
            properties,
        }
    }
}
#[doc = "Properties of the IoTHub event source that are required on create or update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCreationProperties {
    #[serde(flatten)]
    pub io_t_hub_event_source_common_properties: IoTHubEventSourceCommonProperties,
    #[doc = "The value of the Shared Access Policy key that grants the Time Series Insights service read access to the iot hub. This property is not shown in event source responses."]
    #[serde(rename = "sharedAccessKey")]
    pub shared_access_key: String,
}
impl IoTHubEventSourceCreationProperties {
    pub fn new(io_t_hub_event_source_common_properties: IoTHubEventSourceCommonProperties, shared_access_key: String) -> Self {
        Self {
            io_t_hub_event_source_common_properties,
            shared_access_key,
        }
    }
}
#[doc = "An object that represents a set of mutable IoTHub event source resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTHubEventSourceMutableProperties {
    #[serde(flatten)]
    pub event_source_mutable_properties: EventSourceMutableProperties,
    #[doc = "The value of the shared access key that grants the Time Series Insights service read access to the iot hub. This property is not shown in event source responses."]
    #[serde(rename = "sharedAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_key: Option<String>,
}
impl IoTHubEventSourceMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An event source that receives its data from an Azure IoTHub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceResource {
    #[serde(flatten)]
    pub event_source_resource: EventSourceResource,
    #[doc = "Properties of the IoTHub event source resource."]
    pub properties: IoTHubEventSourceResourceProperties,
}
impl IoTHubEventSourceResource {
    pub fn new(event_source_resource: EventSourceResource, properties: IoTHubEventSourceResourceProperties) -> Self {
        Self {
            event_source_resource,
            properties,
        }
    }
}
#[doc = "Properties of the IoTHub event source resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceResourceProperties {
    #[serde(flatten)]
    pub io_t_hub_event_source_common_properties: IoTHubEventSourceCommonProperties,
}
impl IoTHubEventSourceResourceProperties {
    pub fn new(io_t_hub_event_source_common_properties: IoTHubEventSourceCommonProperties) -> Self {
        Self {
            io_t_hub_event_source_common_properties,
        }
    }
}
#[doc = "Parameters supplied to the Update Event Source operation to update an IoTHub event source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceUpdateParameters {
    #[serde(flatten)]
    pub event_source_update_parameters: EventSourceUpdateParameters,
    #[doc = "An object that represents a set of mutable IoTHub event source resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTHubEventSourceMutableProperties>,
}
impl IoTHubEventSourceUpdateParameters {
    pub fn new(event_source_update_parameters: EventSourceUpdateParameters) -> Self {
        Self {
            event_source_update_parameters,
            properties: None,
        }
    }
}
#[doc = "An object that represents the local timestamp property. It contains the format of local timestamp that needs to be used and the corresponding timezone offset information. If a value isn't specified for localTimestamp, or if null, then the local timestamp will not be ingressed with the events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalTimestamp {
    #[doc = "An enum that represents the format of the local timestamp property that needs to be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<local_timestamp::Format>,
    #[doc = "An object that represents the offset information for the local timestamp format specified. Should not be specified for LocalTimestampFormat - Embedded."]
    #[serde(rename = "timeZoneOffset", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_offset: Option<local_timestamp::TimeZoneOffset>,
}
impl LocalTimestamp {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod local_timestamp {
    use super::*;
    #[doc = "An enum that represents the format of the local timestamp property that needs to be set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        Embedded,
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
                Self::Embedded => serializer.serialize_unit_variant("Format", 0u32, "Embedded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "An object that represents the offset information for the local timestamp format specified. Should not be specified for LocalTimestampFormat - Embedded."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TimeZoneOffset {
        #[doc = "The event property that will be contain the offset information to calculate the local timestamp. When the LocalTimestampFormat is Iana, the property name will contain the name of the column which contains IANA Timezone Name (eg: Americas/Los Angeles). When LocalTimestampFormat is Timespan, it contains the name of property which contains values representing the offset (eg: P1D or 1.00:00:00)"]
        #[serde(rename = "propertyName", default, skip_serializing_if = "Option::is_none")]
        pub property_name: Option<String>,
    }
    impl TimeZoneOffset {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The specification of an Azure Monitoring log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Log name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Log display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Retention policy of a resource metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of metric specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of metric specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Display description of metric specification."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Unit could be Bytes or Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Dimensions of blobs, including blob type and access tier."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[doc = "Aggregation type could be Average."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Retention policies of a resource metric."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<MetricAvailability>,
    #[doc = "The category this metric specification belong to, could be Capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Account Resource Id."]
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Time Series Insights REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains the localized display information for this particular operation / action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of operation, include metric specifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Contains the localized display information for this particular operation / action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly form of the resource type related to this action/operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The localized friendly name for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The localized friendly description for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Time Series Insights operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Time Series Insights operations supported by the Microsoft.TimeSeriesInsights resource provider."]
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
#[doc = "Properties of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "One property of operation, include metric specifications."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[doc = "The ARM identifier for Private Endpoint Connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A list of private links resource group ids"]
    #[serde(rename = "groupIds", default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<PrivateEndpointGroupIds>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            provisioning_state: None,
            private_endpoint: None,
            group_ids: None,
            private_link_service_connection_state,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
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
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type PrivateEndpointGroupIds = Vec<String>;
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
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
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
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
#[doc = "Provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Succeeded,
    Failed,
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
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    #[doc = "Properties used to create a reference data set."]
    pub properties: ReferenceDataSetCreationProperties,
}
impl ReferenceDataSetCreateOrUpdateParameters {
    pub fn new(
        create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
        properties: ReferenceDataSetCreationProperties,
    ) -> Self {
        Self {
            create_or_update_tracked_resource_properties,
            properties,
        }
    }
}
#[doc = "Properties used to create a reference data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetCreationProperties {
    #[doc = "The list of key properties for the reference data set."]
    #[serde(rename = "keyProperties")]
    pub key_properties: Vec<ReferenceDataSetKeyProperty>,
    #[doc = "The reference data set key comparison behavior can be set using this property. By default, the value is 'Ordinal' - which means case sensitive key comparison will be performed while joining reference data with events or while adding new reference data. When 'OrdinalIgnoreCase' is set, case insensitive comparison will be used."]
    #[serde(rename = "dataStringComparisonBehavior", default, skip_serializing_if = "Option::is_none")]
    pub data_string_comparison_behavior: Option<reference_data_set_creation_properties::DataStringComparisonBehavior>,
}
impl ReferenceDataSetCreationProperties {
    pub fn new(key_properties: Vec<ReferenceDataSetKeyProperty>) -> Self {
        Self {
            key_properties,
            data_string_comparison_behavior: None,
        }
    }
}
pub mod reference_data_set_creation_properties {
    use super::*;
    #[doc = "The reference data set key comparison behavior can be set using this property. By default, the value is 'Ordinal' - which means case sensitive key comparison will be performed while joining reference data with events or while adding new reference data. When 'OrdinalIgnoreCase' is set, case insensitive comparison will be used."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataStringComparisonBehavior")]
    pub enum DataStringComparisonBehavior {
        Ordinal,
        OrdinalIgnoreCase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataStringComparisonBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataStringComparisonBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataStringComparisonBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ordinal => serializer.serialize_unit_variant("DataStringComparisonBehavior", 0u32, "Ordinal"),
                Self::OrdinalIgnoreCase => serializer.serialize_unit_variant("DataStringComparisonBehavior", 1u32, "OrdinalIgnoreCase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A key property for the reference data set. A reference data set can have multiple key properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceDataSetKeyProperty {
    #[doc = "The name of the key property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the key property."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<reference_data_set_key_property::Type>,
}
impl ReferenceDataSetKeyProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reference_data_set_key_property {
    use super::*;
    #[doc = "The type of the key property."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        String,
        Double,
        Bool,
        DateTime,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "String"),
                Self::Double => serializer.serialize_unit_variant("Type", 1u32, "Double"),
                Self::Bool => serializer.serialize_unit_variant("Type", 2u32, "Bool"),
                Self::DateTime => serializer.serialize_unit_variant("Type", 3u32, "DateTime"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response of the List Reference Data Sets operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceDataSetListResponse {
    #[doc = "Result of the List Reference Data Sets operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReferenceDataSetResource>,
}
impl ReferenceDataSetListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference data set provides metadata about the events in an environment. Metadata in the reference data set will be joined with events as they are read from event sources. The metadata that makes up the reference data set is uploaded or modified through the Time Series Insights data plane APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the reference data set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReferenceDataSetResourceProperties>,
}
impl ReferenceDataSetResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Properties of the reference data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetResourceProperties {
    #[serde(flatten)]
    pub reference_data_set_creation_properties: ReferenceDataSetCreationProperties,
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
}
impl ReferenceDataSetResourceProperties {
    pub fn new(reference_data_set_creation_properties: ReferenceDataSetCreationProperties) -> Self {
        Self {
            reference_data_set_creation_properties,
            resource_properties: ResourceProperties::default(),
        }
    }
}
#[doc = "Parameters supplied to the Update Reference Data Set operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceDataSetUpdateParameters {
    #[doc = "Key-value pairs of additional properties for the reference data set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ReferenceDataSetUpdateParameters {
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
#[doc = "Properties that are common to all tracked resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The time the resource was created."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
}
impl ResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "One property of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Metric specifications of operation."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[doc = "A list of Azure Monitoring log definitions."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sku determines the type of environment, either Gen1 (S1 or S2) or Gen2 (L1). For Gen1 environments the sku determines the capacity of the environment, the ingress rate, and the billing rate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of this SKU."]
    pub name: sku::Name,
    #[doc = "The capacity of the sku. For Gen1 environments, this value can be changed to support scale out of environments after they have been created."]
    pub capacity: i32,
}
impl Sku {
    pub fn new(name: sku::Name, capacity: i32) -> Self {
        Self { name, capacity }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The name of this SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        S1,
        S2,
        P1,
        L1,
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
                Self::S1 => serializer.serialize_unit_variant("Name", 0u32, "S1"),
                Self::S2 => serializer.serialize_unit_variant("Name", 1u32, "S2"),
                Self::P1 => serializer.serialize_unit_variant("Name", 2u32, "P1"),
                Self::L1 => serializer.serialize_unit_variant("Name", 3u32, "L1"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The structure of the property that a time series id can have. An environment can have multiple such properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesIdProperty {
    #[doc = "The name of the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the property."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<time_series_id_property::Type>,
}
impl TimeSeriesIdProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod time_series_id_property {
    use super::*;
    #[doc = "The type of the property."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        String,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "String"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Time Series Insights resource that is tracked by Azure Resource Manager."]
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
#[doc = "An object that represents the status of warm storage on an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WarmStorageEnvironmentStatus {
    #[doc = "An object that contains the status of warm storage properties usage."]
    #[serde(rename = "propertiesUsage", default, skip_serializing_if = "Option::is_none")]
    pub properties_usage: Option<WarmStoragePropertiesUsage>,
}
impl WarmStorageEnvironmentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that contains the status of warm storage properties usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WarmStoragePropertiesUsage {
    #[doc = "This string represents the state of warm storage properties usage. It can be \"Ok\", \"Error\", \"Unknown\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<warm_storage_properties_usage::State>,
    #[doc = "An object that contains the details about warm storage properties usage state."]
    #[serde(rename = "stateDetails", default, skip_serializing_if = "Option::is_none")]
    pub state_details: Option<WarmStoragePropertiesUsageStateDetails>,
}
impl WarmStoragePropertiesUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod warm_storage_properties_usage {
    use super::*;
    #[doc = "This string represents the state of warm storage properties usage. It can be \"Ok\", \"Error\", \"Unknown\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Ok,
        Error,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("State", 0u32, "Ok"),
                Self::Error => serializer.serialize_unit_variant("State", 1u32, "Error"),
                Self::Unknown => serializer.serialize_unit_variant("State", 2u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An object that contains the details about warm storage properties usage state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WarmStoragePropertiesUsageStateDetails {
    #[doc = "A value that represents the number of properties used by the environment for S1/S2 SKU and number of properties used by Warm Store for PAYG SKU"]
    #[serde(rename = "currentCount", default, skip_serializing_if = "Option::is_none")]
    pub current_count: Option<i32>,
    #[doc = "A value that represents the maximum number of properties used allowed by the environment for S1/S2 SKU and maximum number of properties allowed by Warm Store for PAYG SKU."]
    #[serde(rename = "maxCount", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
}
impl WarmStoragePropertiesUsageStateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The warm store configuration provides the details to create a warm store cache that will retain a copy of the environment's data available for faster query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarmStoreConfigurationProperties {
    #[doc = "ISO8601 timespan specifying the number of days the environment's events will be available for query from the warm store."]
    #[serde(rename = "dataRetention")]
    pub data_retention: String,
}
impl WarmStoreConfigurationProperties {
    pub fn new(data_retention: String) -> Self {
        Self { data_retention }
    }
}
