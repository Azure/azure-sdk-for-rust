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
#[doc = "Parameters supplied to the CreateOrUpdate Environment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    #[doc = "The sku determines the capacity of the environment, the SLA (in queries-per-minute and total capacity), and the billing rate."]
    pub sku: Sku,
    #[doc = "Properties used to create an environment."]
    pub properties: EnvironmentCreationProperties,
}
impl EnvironmentCreateOrUpdateParameters {
    pub fn new(
        create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
        sku: Sku,
        properties: EnvironmentCreationProperties,
    ) -> Self {
        Self {
            create_or_update_tracked_resource_properties,
            sku,
            properties,
        }
    }
}
#[doc = "Properties used to create an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentCreationProperties {
    #[doc = "ISO8601 timespan specifying the minimum number of days the environment's events will be available for query."]
    #[serde(rename = "dataRetentionTime")]
    pub data_retention_time: String,
    #[doc = "The behavior the Time Series Insights service should take when the environment's capacity has been exceeded. If \"PauseIngress\" is specified, new events will not be read from the event source. If \"PurgeOldData\" is specified, new events will continue to be read and old events will be deleted from the environment. The default behavior is PurgeOldData."]
    #[serde(rename = "storageLimitExceededBehavior", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit_exceeded_behavior: Option<environment_creation_properties::StorageLimitExceededBehavior>,
    #[doc = "The list of partition keys according to which the data in the environment will be ordered."]
    #[serde(rename = "partitionKeyProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_key_properties: Vec<PartitionKeyProperty>,
}
impl EnvironmentCreationProperties {
    pub fn new(data_retention_time: String) -> Self {
        Self {
            data_retention_time,
            storage_limit_exceeded_behavior: None,
            partition_key_properties: Vec::new(),
        }
    }
}
pub mod environment_creation_properties {
    use super::*;
    #[doc = "The behavior the Time Series Insights service should take when the environment's capacity has been exceeded. If \"PauseIngress\" is specified, new events will not be read from the event source. If \"PurgeOldData\" is specified, new events will continue to be read and old events will be deleted from the environment. The default behavior is PurgeOldData."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageLimitExceededBehavior {
        PurgeOldData,
        PauseIngress,
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
#[doc = "An object that represents a set of mutable environment resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentMutableProperties {
    #[doc = "ISO8601 timespan specifying the minimum number of days the environment's events will be available for query."]
    #[serde(rename = "dataRetentionTime", default, skip_serializing_if = "Option::is_none")]
    pub data_retention_time: Option<String>,
    #[doc = "The behavior the Time Series Insights service should take when the environment's capacity has been exceeded. If \"PauseIngress\" is specified, new events will not be read from the event source. If \"PurgeOldData\" is specified, new events will continue to be read and old events will be deleted from the environment. The default behavior is PurgeOldData."]
    #[serde(rename = "storageLimitExceededBehavior", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit_exceeded_behavior: Option<environment_mutable_properties::StorageLimitExceededBehavior>,
    #[doc = "The list of event properties which will be used to partition data in the environment."]
    #[serde(rename = "partitionKeyProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_key_properties: Vec<PartitionKeyProperty>,
}
impl EnvironmentMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_mutable_properties {
    use super::*;
    #[doc = "The behavior the Time Series Insights service should take when the environment's capacity has been exceeded. If \"PauseIngress\" is specified, new events will not be read from the event source. If \"PurgeOldData\" is specified, new events will continue to be read and old events will be deleted from the environment. The default behavior is PurgeOldData."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageLimitExceededBehavior {
        PurgeOldData,
        PauseIngress,
    }
}
#[doc = "An environment is a set of time-series data available for query, and is the top level Azure Time Series Insights resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The sku determines the capacity of the environment, the SLA (in queries-per-minute and total capacity), and the billing rate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentResourceProperties>,
}
impl EnvironmentResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "Properties of the environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentResourceProperties {
    #[serde(flatten)]
    pub environment_creation_properties: EnvironmentCreationProperties,
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
    pub fn new(environment_creation_properties: EnvironmentCreationProperties) -> Self {
        Self {
            environment_creation_properties,
            resource_properties: ResourceProperties::default(),
            data_access_id: None,
            data_access_fqdn: None,
            status: None,
        }
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
}
impl EnvironmentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update Environment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentUpdateParameters {
    #[doc = "The sku determines the capacity of the environment, the SLA (in queries-per-minute and total capacity), and the billing rate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Key-value pairs of additional properties for the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An object that represents a set of mutable environment resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentMutableProperties>,
}
impl EnvironmentUpdateParameters {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubEventSourceUpdateParameters {
    #[serde(flatten)]
    pub event_source_update_parameters: EventSourceUpdateParameters,
    #[doc = "An object that represents a set of mutable EventHub event source resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubEventSourceMutableProperties>,
}
impl EventHubEventSourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
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
}
impl EventSourceCreateOrUpdateParameters {
    pub fn new(
        create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
        kind: event_source_create_or_update_parameters::Kind,
    ) -> Self {
        Self {
            create_or_update_tracked_resource_properties,
            kind,
        }
    }
}
pub mod event_source_create_or_update_parameters {
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
    #[doc = "An object that represents the local timestamp property. It contains the format of local timestamp that needs to be used and the corresponding timezone offset information. If a value isn't specified for localTimestamp, or if null, then the local timestamp will not be ingressed with the events."]
    #[serde(rename = "localTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub local_timestamp: Option<LocalTimestamp>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSourceUpdateParameters {
    #[doc = "Key-value pairs of additional properties for the event source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl EventSourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
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
    pub enum State {
        Disabled,
        Ready,
        Running,
        Paused,
        Unknown,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTHubEventSourceUpdateParameters {
    #[serde(flatten)]
    pub event_source_update_parameters: EventSourceUpdateParameters,
    #[doc = "An object that represents a set of mutable IoTHub event source resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTHubEventSourceMutableProperties>,
}
impl IoTHubEventSourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
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
    pub enum Format {
        Embedded,
        Iana,
        TimeSpan,
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
#[doc = "A Time Series Insights REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains the localized display information for this particular operation / action."]
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
#[doc = "The structure of the property that a partition key can have. An environment can have multiple such properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionKeyProperty {
    #[doc = "The name of the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the property."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<partition_key_property::Type>,
}
impl PartitionKeyProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod partition_key_property {
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
#[doc = "Provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Succeeded,
    Failed,
    Deleting,
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
    pub enum DataStringComparisonBehavior {
        Ordinal,
        OrdinalIgnoreCase,
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
    pub enum Type {
        String,
        Double,
        Bool,
        DateTime,
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
#[doc = "Time Series Insights resource"]
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
#[doc = "The sku determines the capacity of the environment, the SLA (in queries-per-minute and total capacity), and the billing rate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of this SKU."]
    pub name: sku::Name,
    #[doc = "The capacity of the sku. This value can be changed to support scale out of environments after they have been created."]
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
    pub enum Name {
        S1,
        S2,
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
