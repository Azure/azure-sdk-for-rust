#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "This is the base type that represents an advanced filter. To configure an advanced filter, do not directly instantiate an object of this class. Instead, instantiate an object of a derived class such as BoolEqualsAdvancedFilter, NumberInAdvancedFilter, StringEqualsAdvancedFilter etc. depending on the type of the key based on which you want to filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvancedFilter {
    #[doc = "The field/property in the event based on which you want to filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl AdvancedFilter {
    pub fn new() -> Self {
        Self { key: None }
    }
}
#[doc = "The operator type used for filtering, e.g., NumberIn, StringContains, BoolEquals and others."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operatorType")]
pub enum AdvancedFilterUnion {
    BoolEquals(BoolEqualsAdvancedFilter),
    IsNotNull(IsNotNullAdvancedFilter),
    IsNullOrUndefined(IsNullOrUndefinedAdvancedFilter),
    NumberGreaterThan(NumberGreaterThanAdvancedFilter),
    NumberGreaterThanOrEquals(NumberGreaterThanOrEqualsAdvancedFilter),
    NumberIn(NumberInAdvancedFilter),
    NumberInRange(NumberInRangeAdvancedFilter),
    NumberLessThan(NumberLessThanAdvancedFilter),
    NumberLessThanOrEquals(NumberLessThanOrEqualsAdvancedFilter),
    NumberNotIn(NumberNotInAdvancedFilter),
    NumberNotInRange(NumberNotInRangeAdvancedFilter),
    StringBeginsWith(StringBeginsWithAdvancedFilter),
    StringContains(StringContainsAdvancedFilter),
    StringEndsWith(StringEndsWithAdvancedFilter),
    StringIn(StringInAdvancedFilter),
    StringNotBeginsWith(StringNotBeginsWithAdvancedFilter),
    StringNotContains(StringNotContainsAdvancedFilter),
    StringNotEndsWith(StringNotEndsWithAdvancedFilter),
    StringNotIn(StringNotInAdvancedFilter),
}
#[doc = "Information about the azure function destination for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionEventSubscriptionDestination {
    #[doc = "The properties that represent the Azure Function destination of an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureFunctionEventSubscriptionDestinationProperties>,
}
impl AzureFunctionEventSubscriptionDestination {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "The properties that represent the Azure Function destination of an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFunctionEventSubscriptionDestinationProperties {
    #[doc = "The Azure Resource Id that represents the endpoint of the Azure Function destination of an event subscription."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Maximum number of events per batch."]
    #[serde(rename = "maxEventsPerBatch", default, skip_serializing_if = "Option::is_none")]
    pub max_events_per_batch: Option<i32>,
    #[doc = "Preferred batch size in Kilobytes."]
    #[serde(rename = "preferredBatchSizeInKilobytes", default, skip_serializing_if = "Option::is_none")]
    pub preferred_batch_size_in_kilobytes: Option<i32>,
    #[doc = "Delivery attribute details."]
    #[serde(
        rename = "deliveryAttributeMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delivery_attribute_mappings: Vec<DeliveryAttributeMappingUnion>,
}
impl AzureFunctionEventSubscriptionDestinationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BoolEquals Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoolEqualsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The boolean filter value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
impl BoolEqualsAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            value: None,
        }
    }
}
#[doc = "ConnectionState information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionState {
    #[doc = "Status of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<connection_state::Status>,
    #[doc = "Description of the connection state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Actions required (if any)."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl ConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_state {
    use super::*;
    #[doc = "Status of the connection."]
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
#[doc = "Type of the endpoint for the dead letter destination"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "endpointType")]
pub enum DeadLetterDestinationUnion {
    StorageBlob(StorageBlobDeadLetterDestination),
}
#[doc = "Information about the deadletter destination with resource identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeadLetterWithResourceIdentity {
    #[doc = "The identity information with the event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EventSubscriptionIdentity>,
    #[doc = "Information about the dead letter destination for an event subscription. To configure a deadletter destination, do not directly instantiate an object of this class. Instead, instantiate an object of a derived class. Currently, StorageBlobDeadLetterDestination is the only class that derives from this class."]
    #[serde(rename = "deadLetterDestination", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_destination: Option<DeadLetterDestinationUnion>,
}
impl DeadLetterWithResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the Get delivery attributes operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeliveryAttributeListResult {
    #[doc = "A collection of DeliveryAttributeMapping"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeliveryAttributeMappingUnion>,
}
impl DeliveryAttributeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Delivery attribute mapping details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryAttributeMapping {
    #[doc = "Name of the delivery attribute or header."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DeliveryAttributeMapping {
    pub fn new() -> Self {
        Self { name: None }
    }
}
#[doc = "Type of the delivery attribute or header name."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DeliveryAttributeMappingUnion {
    Dynamic(DynamicDeliveryAttributeMapping),
    Static(StaticDeliveryAttributeMapping),
}
#[doc = "Information about the delivery for an event subscription with resource identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeliveryWithResourceIdentity {
    #[doc = "The identity information with the event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EventSubscriptionIdentity>,
    #[doc = "Information about the destination for an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<EventSubscriptionDestinationUnion>,
}
impl DeliveryWithResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "EventGrid Domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the Event Grid Domain Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The identity information for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityInfo>,
}
impl Domain {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
            identity: None,
        }
    }
}
#[doc = "Properties of the Event Grid Domain Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainProperties {
    #[doc = "List of private endpoint connections."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Provisioning state of the Event Grid Domain Resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<domain_properties::ProvisioningState>,
    #[doc = "Endpoint for the domain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "This determines the format that Event Grid should expect for incoming events published to the domain."]
    #[serde(rename = "inputSchema", default, skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<domain_properties::InputSchema>,
    #[doc = "By default, Event Grid expects events to be in the Event Grid event schema. Specifying an input schema mapping enables publishing to Event Grid using a custom input schema. Currently, the only supported type of InputSchemaMapping is 'JsonInputSchemaMapping'."]
    #[serde(rename = "inputSchemaMapping", default, skip_serializing_if = "Option::is_none")]
    pub input_schema_mapping: Option<InputSchemaMappingUnion>,
    #[doc = "Metric resource id for the domain."]
    #[serde(rename = "metricResourceId", default, skip_serializing_if = "Option::is_none")]
    pub metric_resource_id: Option<String>,
    #[doc = "This determines if traffic is allowed over public network. By default it is enabled. \r\nYou can further restrict to specific IPs by configuring <seealso cref=\"P:Microsoft.Azure.Events.ResourceProvider.Common.Contracts.DomainProperties.InboundIpRules\" />"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<domain_properties::PublicNetworkAccess>,
    #[doc = "This can be used to restrict traffic from specific IPs instead of all IPs. Note: These are considered only if PublicNetworkAccess is enabled."]
    #[serde(
        rename = "inboundIpRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_ip_rules: Vec<InboundIpRule>,
    #[doc = "This boolean is used to enable or disable local auth. Default value is false. When the property is set to true, only AAD token will be used to authenticate if user is allowed to publish to the domain."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[doc = "This Boolean is used to specify the creation mechanism for 'all' the Event Grid Domain Topics associated with this Event Grid Domain resource.\r\nIn this context, creation of domain topic can be auto-managed (when true) or self-managed (when false). The default value for this property is true.\r\nWhen this property is null or set to true, Event Grid is responsible of automatically creating the domain topic when the first event subscription is\r\ncreated at the scope of the domain topic. If this property is set to false, then creating the first event subscription will require creating a domain topic\r\nby the user. The self-management mode can be used if the user wants full control of when the domain topic is created, while auto-managed mode provides the\r\nflexibility to perform less operations and manage fewer resources by the user. Also, note that in auto-managed creation mode, user is allowed to create the\r\ndomain topic on demand if needed."]
    #[serde(rename = "autoCreateTopicWithFirstSubscription", default, skip_serializing_if = "Option::is_none")]
    pub auto_create_topic_with_first_subscription: Option<bool>,
    #[doc = "This Boolean is used to specify the deletion mechanism for 'all' the Event Grid Domain Topics associated with this Event Grid Domain resource.\r\nIn this context, deletion of domain topic can be auto-managed (when true) or self-managed (when false). The default value for this property is true.\r\nWhen this property is set to true, Event Grid is responsible of automatically deleting the domain topic when the last event subscription at the scope\r\nof the domain topic is deleted. If this property is set to false, then the user needs to manually delete the domain topic when it is no longer needed\r\n(e.g., when last event subscription is deleted and the resource needs to be cleaned up). The self-management mode can be used if the user wants full\r\ncontrol of when the domain topic needs to be deleted, while auto-managed mode provides the flexibility to perform less operations and manage fewer\r\nresources by the user."]
    #[serde(rename = "autoDeleteTopicWithLastSubscription", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_topic_with_last_subscription: Option<bool>,
}
impl DomainProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod domain_properties {
    use super::*;
    #[doc = "Provisioning state of the Event Grid Domain Resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This determines the format that Event Grid should expect for incoming events published to the domain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InputSchema")]
    pub enum InputSchema {
        EventGridSchema,
        CustomEventSchema,
        #[serde(rename = "CloudEventSchemaV1_0")]
        CloudEventSchemaV10,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InputSchema {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InputSchema {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InputSchema {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EventGridSchema => serializer.serialize_unit_variant("InputSchema", 0u32, "EventGridSchema"),
                Self::CustomEventSchema => serializer.serialize_unit_variant("InputSchema", 1u32, "CustomEventSchema"),
                Self::CloudEventSchemaV10 => serializer.serialize_unit_variant("InputSchema", 2u32, "CloudEventSchemaV1_0"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for InputSchema {
        fn default() -> Self {
            Self::EventGridSchema
        }
    }
    #[doc = "This determines if traffic is allowed over public network. By default it is enabled. \r\nYou can further restrict to specific IPs by configuring <seealso cref=\"P:Microsoft.Azure.Events.ResourceProvider.Common.Contracts.DomainProperties.InboundIpRules\" />"]
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
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Domain regenerate share access key request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainRegenerateKeyRequest {
    #[doc = "Key name to regenerate key1 or key2."]
    #[serde(rename = "keyName")]
    pub key_name: String,
}
impl DomainRegenerateKeyRequest {
    pub fn new(key_name: String) -> Self {
        Self { key_name }
    }
}
#[doc = "Shared access keys of the Domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainSharedAccessKeys {
    #[doc = "Shared access key1 for the domain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[doc = "Shared access key2 for the domain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
impl DomainSharedAccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Domain Topic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainTopic {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the Domain Topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainTopicProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DomainTopic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Domain Topic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainTopicProperties {
    #[doc = "Provisioning state of the domain topic."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<domain_topic_properties::ProvisioningState>,
}
impl DomainTopicProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod domain_topic_properties {
    use super::*;
    #[doc = "Provisioning state of the domain topic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the List Domain Topics operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainTopicsListResult {
    #[doc = "A collection of Domain Topics."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DomainTopic>,
    #[doc = "A link for the next page of domain topics."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DomainTopicsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DomainTopicsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of domain update parameter properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainUpdateParameterProperties {
    #[doc = "This determines if traffic is allowed over public network. By default it is enabled. \r\nYou can further restrict to specific IPs by configuring <seealso cref=\"P:Microsoft.Azure.Events.ResourceProvider.Common.Contracts.DomainUpdateParameterProperties.InboundIpRules\" />"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<domain_update_parameter_properties::PublicNetworkAccess>,
    #[doc = "This can be used to restrict traffic from specific IPs instead of all IPs. Note: These are considered only if PublicNetworkAccess is enabled."]
    #[serde(
        rename = "inboundIpRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_ip_rules: Vec<InboundIpRule>,
    #[doc = "This boolean is used to enable or disable local auth. Default value is false. When the property is set to true, only AAD token will be used to authenticate if user is allowed to publish to the domain."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[doc = "This Boolean is used to specify the creation mechanism for 'all' the Event Grid Domain Topics associated with this Event Grid Domain resource.\r\nIn this context, creation of domain topic can be auto-managed (when true) or self-managed (when false). The default value for this property is true.\r\nWhen this property is null or set to true, Event Grid is responsible of automatically creating the domain topic when the first event subscription is\r\ncreated at the scope of the domain topic. If this property is set to false, then creating the first event subscription will require creating a domain topic\r\nby the user. The self-management mode can be used if the user wants full control of when the domain topic is created, while auto-managed mode provides the\r\nflexibility to perform less operations and manage fewer resources by the user. Also, note that in auto-managed creation mode, user is allowed to create the\r\ndomain topic on demand if needed."]
    #[serde(rename = "autoCreateTopicWithFirstSubscription", default, skip_serializing_if = "Option::is_none")]
    pub auto_create_topic_with_first_subscription: Option<bool>,
    #[doc = "This Boolean is used to specify the deletion mechanism for 'all' the Event Grid Domain Topics associated with this Event Grid Domain resource.\r\nIn this context, deletion of domain topic can be auto-managed (when true) or self-managed (when false). The default value for this property is true.\r\nWhen this property is set to true, Event Grid is responsible of automatically deleting the domain topic when the last event subscription at the scope\r\nof the domain topic is deleted. If this property is set to false, then the user needs to manually delete the domain topic when it is no longer needed\r\n(e.g., when last event subscription is deleted and the resource needs to be cleaned up). The self-management mode can be used if the user wants full\r\ncontrol of when the domain topic needs to be deleted, while auto-managed mode provides the flexibility to perform less operations and manage fewer\r\nresources by the user."]
    #[serde(rename = "autoDeleteTopicWithLastSubscription", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_topic_with_last_subscription: Option<bool>,
}
impl DomainUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod domain_update_parameter_properties {
    use super::*;
    #[doc = "This determines if traffic is allowed over public network. By default it is enabled. \r\nYou can further restrict to specific IPs by configuring <seealso cref=\"P:Microsoft.Azure.Events.ResourceProvider.Common.Contracts.DomainUpdateParameterProperties.InboundIpRules\" />"]
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
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Properties of the Domain update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainUpdateParameters {
    #[doc = "Tags of the domains resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Information of domain update parameter properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainUpdateParameterProperties>,
    #[doc = "The identity information for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityInfo>,
}
impl DomainUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List Domains operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainsListResult {
    #[doc = "A collection of Domains."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Domain>,
    #[doc = "A link for the next page of domains."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DomainsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DomainsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dynamic delivery attribute mapping details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicDeliveryAttributeMapping {
    #[serde(flatten)]
    pub delivery_attribute_mapping: DeliveryAttributeMapping,
    #[doc = "Properties of dynamic delivery attribute mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DynamicDeliveryAttributeMappingProperties>,
}
impl DynamicDeliveryAttributeMapping {
    pub fn new(delivery_attribute_mapping: DeliveryAttributeMapping) -> Self {
        Self {
            delivery_attribute_mapping,
            properties: None,
        }
    }
}
#[doc = "Properties of dynamic delivery attribute mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DynamicDeliveryAttributeMappingProperties {
    #[doc = "JSON path in the event which contains attribute value."]
    #[serde(rename = "sourceField", default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
}
impl DynamicDeliveryAttributeMappingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the event hub destination for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSubscriptionDestination {
    #[doc = "The properties for a event hub destination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubEventSubscriptionDestinationProperties>,
}
impl EventHubEventSubscriptionDestination {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "The properties for a event hub destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubEventSubscriptionDestinationProperties {
    #[doc = "The Azure Resource Id that represents the endpoint of an Event Hub destination of an event subscription."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Delivery attribute details."]
    #[serde(
        rename = "deliveryAttributeMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delivery_attribute_mappings: Vec<DeliveryAttributeMappingUnion>,
}
impl EventHubEventSubscriptionDestinationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event Subscription"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSubscription {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the Event Subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventSubscriptionProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl EventSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the endpoint for the event subscription destination."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "endpointType")]
pub enum EventSubscriptionDestinationUnion {
    AzureFunction(AzureFunctionEventSubscriptionDestination),
    EventHub(EventHubEventSubscriptionDestination),
    HybridConnection(HybridConnectionEventSubscriptionDestination),
    ServiceBusQueue(ServiceBusQueueEventSubscriptionDestination),
    ServiceBusTopic(ServiceBusTopicEventSubscriptionDestination),
    StorageQueue(StorageQueueEventSubscriptionDestination),
    WebHook(WebHookEventSubscriptionDestination),
}
#[doc = "Filter for the Event Subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSubscriptionFilter {
    #[doc = "An optional string to filter events for an event subscription based on a resource path prefix.\r\nThe format of this depends on the publisher of the events.\r\nWildcard characters are not supported in this path."]
    #[serde(rename = "subjectBeginsWith", default, skip_serializing_if = "Option::is_none")]
    pub subject_begins_with: Option<String>,
    #[doc = "An optional string to filter events for an event subscription based on a resource path suffix.\r\nWildcard characters are not supported in this path."]
    #[serde(rename = "subjectEndsWith", default, skip_serializing_if = "Option::is_none")]
    pub subject_ends_with: Option<String>,
    #[doc = "A list of applicable event types that need to be part of the event subscription. If it is desired to subscribe to all default event types, set the IncludedEventTypes to null."]
    #[serde(
        rename = "includedEventTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub included_event_types: Vec<String>,
    #[doc = "Specifies if the SubjectBeginsWith and SubjectEndsWith properties of the filter\r\nshould be compared in a case sensitive manner."]
    #[serde(rename = "isSubjectCaseSensitive", default, skip_serializing_if = "Option::is_none")]
    pub is_subject_case_sensitive: Option<bool>,
    #[doc = "Allows advanced filters to be evaluated against an array of values instead of expecting a singular value."]
    #[serde(rename = "enableAdvancedFilteringOnArrays", default, skip_serializing_if = "Option::is_none")]
    pub enable_advanced_filtering_on_arrays: Option<bool>,
    #[doc = "An array of advanced filters that are used for filtering event subscriptions."]
    #[serde(
        rename = "advancedFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub advanced_filters: Vec<AdvancedFilterUnion>,
}
impl EventSubscriptionFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Full endpoint url of an event subscription"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSubscriptionFullUrl {
    #[doc = "The URL that represents the endpoint of the destination of an event subscription."]
    #[serde(rename = "endpointUrl", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}
impl EventSubscriptionFullUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The identity information with the event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSubscriptionIdentity {
    #[doc = "The type of managed identity used. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user-assigned identities. The type 'None' will remove any identity."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<event_subscription_identity::Type>,
    #[doc = "The user identity associated with the resource."]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl EventSubscriptionIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_subscription_identity {
    use super::*;
    #[doc = "The type of managed identity used. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user-assigned identities. The type 'None' will remove any identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
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
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 1u32, "UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the Event Subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSubscriptionProperties {
    #[doc = "Name of the topic of the event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[doc = "Provisioning state of the event subscription."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<event_subscription_properties::ProvisioningState>,
    #[doc = "Information about the destination for an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<EventSubscriptionDestinationUnion>,
    #[doc = "Information about the delivery for an event subscription with resource identity."]
    #[serde(rename = "deliveryWithResourceIdentity", default, skip_serializing_if = "Option::is_none")]
    pub delivery_with_resource_identity: Option<DeliveryWithResourceIdentity>,
    #[doc = "Filter for the Event Subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventSubscriptionFilter>,
    #[doc = "List of user defined labels."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<String>,
    #[doc = "Expiration time of the event subscription."]
    #[serde(rename = "expirationTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The event delivery schema for the event subscription."]
    #[serde(rename = "eventDeliverySchema", default, skip_serializing_if = "Option::is_none")]
    pub event_delivery_schema: Option<event_subscription_properties::EventDeliverySchema>,
    #[doc = "Information about the retry policy for an event subscription."]
    #[serde(rename = "retryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[doc = "Information about the dead letter destination for an event subscription. To configure a deadletter destination, do not directly instantiate an object of this class. Instead, instantiate an object of a derived class. Currently, StorageBlobDeadLetterDestination is the only class that derives from this class."]
    #[serde(rename = "deadLetterDestination", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_destination: Option<DeadLetterDestinationUnion>,
    #[doc = "Information about the deadletter destination with resource identity."]
    #[serde(rename = "deadLetterWithResourceIdentity", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_with_resource_identity: Option<DeadLetterWithResourceIdentity>,
}
impl EventSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_subscription_properties {
    use super::*;
    #[doc = "Provisioning state of the event subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
        AwaitingManualAction,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::AwaitingManualAction => serializer.serialize_unit_variant("ProvisioningState", 6u32, "AwaitingManualAction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The event delivery schema for the event subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EventDeliverySchema")]
    pub enum EventDeliverySchema {
        EventGridSchema,
        CustomInputSchema,
        #[serde(rename = "CloudEventSchemaV1_0")]
        CloudEventSchemaV10,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EventDeliverySchema {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EventDeliverySchema {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EventDeliverySchema {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EventGridSchema => serializer.serialize_unit_variant("EventDeliverySchema", 0u32, "EventGridSchema"),
                Self::CustomInputSchema => serializer.serialize_unit_variant("EventDeliverySchema", 1u32, "CustomInputSchema"),
                Self::CloudEventSchemaV10 => serializer.serialize_unit_variant("EventDeliverySchema", 2u32, "CloudEventSchemaV1_0"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for EventDeliverySchema {
        fn default() -> Self {
            Self::EventGridSchema
        }
    }
}
#[doc = "Properties of the Event Subscription update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSubscriptionUpdateParameters {
    #[doc = "Information about the destination for an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<EventSubscriptionDestinationUnion>,
    #[doc = "Information about the delivery for an event subscription with resource identity."]
    #[serde(rename = "deliveryWithResourceIdentity", default, skip_serializing_if = "Option::is_none")]
    pub delivery_with_resource_identity: Option<DeliveryWithResourceIdentity>,
    #[doc = "Filter for the Event Subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventSubscriptionFilter>,
    #[doc = "List of user defined labels."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<String>,
    #[doc = "Information about the expiration time for the event subscription."]
    #[serde(rename = "expirationTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The event delivery schema for the event subscription."]
    #[serde(rename = "eventDeliverySchema", default, skip_serializing_if = "Option::is_none")]
    pub event_delivery_schema: Option<event_subscription_update_parameters::EventDeliverySchema>,
    #[doc = "Information about the retry policy for an event subscription."]
    #[serde(rename = "retryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[doc = "Information about the dead letter destination for an event subscription. To configure a deadletter destination, do not directly instantiate an object of this class. Instead, instantiate an object of a derived class. Currently, StorageBlobDeadLetterDestination is the only class that derives from this class."]
    #[serde(rename = "deadLetterDestination", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_destination: Option<DeadLetterDestinationUnion>,
    #[doc = "Information about the deadletter destination with resource identity."]
    #[serde(rename = "deadLetterWithResourceIdentity", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_with_resource_identity: Option<DeadLetterWithResourceIdentity>,
}
impl EventSubscriptionUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_subscription_update_parameters {
    use super::*;
    #[doc = "The event delivery schema for the event subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EventDeliverySchema")]
    pub enum EventDeliverySchema {
        EventGridSchema,
        CustomInputSchema,
        #[serde(rename = "CloudEventSchemaV1_0")]
        CloudEventSchemaV10,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EventDeliverySchema {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EventDeliverySchema {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EventDeliverySchema {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EventGridSchema => serializer.serialize_unit_variant("EventDeliverySchema", 0u32, "EventGridSchema"),
                Self::CustomInputSchema => serializer.serialize_unit_variant("EventDeliverySchema", 1u32, "CustomInputSchema"),
                Self::CloudEventSchemaV10 => serializer.serialize_unit_variant("EventDeliverySchema", 2u32, "CloudEventSchemaV1_0"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the List EventSubscriptions operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSubscriptionsListResult {
    #[doc = "A collection of EventSubscriptions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EventSubscription>,
    #[doc = "A link for the next page of event subscriptions"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventSubscriptionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EventSubscriptionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event Type for a subject under a topic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventType {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the event type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventTypeProperties>,
}
impl EventType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the event type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventTypeProperties {
    #[doc = "Display name of the event type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of the event type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Url of the schema for this event type."]
    #[serde(rename = "schemaUrl", default, skip_serializing_if = "Option::is_none")]
    pub schema_url: Option<String>,
    #[doc = "IsInDefaultSet flag of the event type."]
    #[serde(rename = "isInDefaultSet", default, skip_serializing_if = "Option::is_none")]
    pub is_in_default_set: Option<bool>,
}
impl EventTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List Event Types operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventTypesListResult {
    #[doc = "A collection of event types"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EventType>,
}
impl azure_core::Continuable for EventTypesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl EventTypesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event grid Extension Topic. This is used for getting Event Grid related metrics for Azure resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTopic {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the Extension Topic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtensionTopicProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ExtensionTopic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Extension Topic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTopicProperties {
    #[doc = "Description of the extension topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "System topic resource id which is mapped to the source."]
    #[serde(rename = "systemTopic", default, skip_serializing_if = "Option::is_none")]
    pub system_topic: Option<String>,
}
impl ExtensionTopicProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the HybridConnection destination for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridConnectionEventSubscriptionDestination {
    #[doc = "The properties for a hybrid connection destination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HybridConnectionEventSubscriptionDestinationProperties>,
}
impl HybridConnectionEventSubscriptionDestination {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "The properties for a hybrid connection destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionEventSubscriptionDestinationProperties {
    #[doc = "The Azure Resource ID of an hybrid connection that is the destination of an event subscription."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Delivery attribute details."]
    #[serde(
        rename = "deliveryAttributeMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delivery_attribute_mappings: Vec<DeliveryAttributeMappingUnion>,
}
impl HybridConnectionEventSubscriptionDestinationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The identity information for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityInfo {
    #[doc = "The type of managed identity used. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user-assigned identities. The type 'None' will remove any identity."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_info::Type>,
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The list of user identities associated with the resource. The user identity dictionary key references will be ARM resource ids in the form:\r\n'/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'.\r\nThis property is currently not used and reserved for future usage."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl IdentityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_info {
    use super::*;
    #[doc = "The type of managed identity used. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user-assigned identities. The type 'None' will remove any identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
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
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssigned, UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundIpRule {
    #[doc = "IP Address in CIDR notation e.g., 10.0.0.0/8."]
    #[serde(rename = "ipMask", default, skip_serializing_if = "Option::is_none")]
    pub ip_mask: Option<String>,
    #[doc = "Action to perform based on the match or no match of the IpMask."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<inbound_ip_rule::Action>,
}
impl InboundIpRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod inbound_ip_rule {
    use super::*;
    #[doc = "Action to perform based on the match or no match of the IpMask."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Type of the custom mapping"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "inputSchemaMappingType")]
pub enum InputSchemaMappingUnion {
    Json(JsonInputSchemaMapping),
}
#[doc = "IsNotNull Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsNotNullAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
}
impl IsNotNullAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self { advanced_filter }
    }
}
#[doc = "IsNullOrUndefined Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsNullOrUndefinedAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
}
impl IsNullOrUndefinedAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self { advanced_filter }
    }
}
#[doc = "This is used to express the source of an input schema mapping for a single target field in the Event Grid Event schema. This is currently used in the mappings for the 'id', 'topic' and 'eventtime' properties. This represents a field in the input event schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonField {
    #[doc = "Name of a field in the input event schema that's to be used as the source of a mapping."]
    #[serde(rename = "sourceField", default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
}
impl JsonField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is used to express the source of an input schema mapping for a single target field\r\nin the Event Grid Event schema. This is currently used in the mappings for the 'subject',\r\n'eventtype' and 'dataversion' properties. This represents a field in the input event schema\r\nalong with a default value to be used, and at least one of these two properties should be provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonFieldWithDefault {
    #[doc = "Name of a field in the input event schema that's to be used as the source of a mapping."]
    #[serde(rename = "sourceField", default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[doc = "The default value to be used for mapping when a SourceField is not provided or if there's no property with the specified name in the published JSON event payload."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}
impl JsonFieldWithDefault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This enables publishing to Event Grid using a custom input schema. This can be used to map properties from a custom input JSON schema to the Event Grid event schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonInputSchemaMapping {
    #[doc = "This can be used to map properties of a source schema (or default values, for certain supported properties) to properties of the EventGridEvent schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JsonInputSchemaMappingProperties>,
}
impl JsonInputSchemaMapping {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "This can be used to map properties of a source schema (or default values, for certain supported properties) to properties of the EventGridEvent schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonInputSchemaMappingProperties {
    #[doc = "This is used to express the source of an input schema mapping for a single target field in the Event Grid Event schema. This is currently used in the mappings for the 'id', 'topic' and 'eventtime' properties. This represents a field in the input event schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<JsonField>,
    #[doc = "This is used to express the source of an input schema mapping for a single target field in the Event Grid Event schema. This is currently used in the mappings for the 'id', 'topic' and 'eventtime' properties. This represents a field in the input event schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<JsonField>,
    #[doc = "This is used to express the source of an input schema mapping for a single target field in the Event Grid Event schema. This is currently used in the mappings for the 'id', 'topic' and 'eventtime' properties. This represents a field in the input event schema."]
    #[serde(rename = "eventTime", default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<JsonField>,
    #[doc = "This is used to express the source of an input schema mapping for a single target field\r\nin the Event Grid Event schema. This is currently used in the mappings for the 'subject',\r\n'eventtype' and 'dataversion' properties. This represents a field in the input event schema\r\nalong with a default value to be used, and at least one of these two properties should be provided."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<JsonFieldWithDefault>,
    #[doc = "This is used to express the source of an input schema mapping for a single target field\r\nin the Event Grid Event schema. This is currently used in the mappings for the 'subject',\r\n'eventtype' and 'dataversion' properties. This represents a field in the input event schema\r\nalong with a default value to be used, and at least one of these two properties should be provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<JsonFieldWithDefault>,
    #[doc = "This is used to express the source of an input schema mapping for a single target field\r\nin the Event Grid Event schema. This is currently used in the mappings for the 'subject',\r\n'eventtype' and 'dataversion' properties. This represents a field in the input event schema\r\nalong with a default value to be used, and at least one of these two properties should be provided."]
    #[serde(rename = "dataVersion", default, skip_serializing_if = "Option::is_none")]
    pub data_version: Option<JsonFieldWithDefault>,
}
impl JsonInputSchemaMappingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NumberGreaterThan Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberGreaterThanAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The filter value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl NumberGreaterThanAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            value: None,
        }
    }
}
#[doc = "NumberGreaterThanOrEquals Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberGreaterThanOrEqualsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The filter value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl NumberGreaterThanOrEqualsAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            value: None,
        }
    }
}
#[doc = "NumberIn Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberInAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<f64>,
}
impl NumberInAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "NumberInRange Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberInRangeAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<Vec<f64>>,
}
impl NumberInRangeAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "NumberLessThan Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberLessThanAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The filter value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl NumberLessThanAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            value: None,
        }
    }
}
#[doc = "NumberLessThanOrEquals Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberLessThanOrEqualsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The filter value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl NumberLessThanOrEqualsAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            value: None,
        }
    }
}
#[doc = "NumberNotIn Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberNotInAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<f64>,
}
impl NumberNotInAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "NumberNotInRange Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberNotInRangeAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<Vec<f64>>,
}
impl NumberNotInRangeAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "Represents an operation returned by the GetOperations request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Information about an operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "This Boolean is used to determine if the operation is a data plane action or not."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Properties of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationInfo {
    #[doc = "Name of the provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List Operations operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResult {
    #[doc = "A collection of operations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PrivateEndpoint information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the list of all private endpoint connections operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "A collection of private endpoint connection resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "A link for the next page of private endpoint connection resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "PrivateEndpoint information."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "GroupIds from the private link service resource."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "ConnectionState information."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<ConnectionState>,
    #[doc = "Provisioning state of the Private Endpoint Connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[doc = "Provisioning state of the Private Endpoint Connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information of the private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[doc = "Fully qualified identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List private link resources operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourcesListResult {
    #[doc = "A collection of private link resources"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "A link for the next page of private link resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourcesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateLinkResourcesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the retry policy for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetryPolicy {
    #[doc = "Maximum number of delivery retry attempts for events."]
    #[serde(rename = "maxDeliveryAttempts", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_attempts: Option<i32>,
    #[doc = "Time To Live (in minutes) for events."]
    #[serde(rename = "eventTimeToLiveInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub event_time_to_live_in_minutes: Option<i32>,
}
impl RetryPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the service bus destination for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueEventSubscriptionDestination {
    #[doc = "The properties that represent the Service Bus destination of an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceBusQueueEventSubscriptionDestinationProperties>,
}
impl ServiceBusQueueEventSubscriptionDestination {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "The properties that represent the Service Bus destination of an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusQueueEventSubscriptionDestinationProperties {
    #[doc = "The Azure Resource Id that represents the endpoint of the Service Bus destination of an event subscription."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Delivery attribute details."]
    #[serde(
        rename = "deliveryAttributeMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delivery_attribute_mappings: Vec<DeliveryAttributeMappingUnion>,
}
impl ServiceBusQueueEventSubscriptionDestinationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the service bus topic destination for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicEventSubscriptionDestination {
    #[doc = "The properties that represent the Service Bus Topic destination of an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceBusTopicEventSubscriptionDestinationProperties>,
}
impl ServiceBusTopicEventSubscriptionDestination {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "The properties that represent the Service Bus Topic destination of an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusTopicEventSubscriptionDestinationProperties {
    #[doc = "The Azure Resource Id that represents the endpoint of the Service Bus Topic destination of an event subscription."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Delivery attribute details."]
    #[serde(
        rename = "deliveryAttributeMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delivery_attribute_mappings: Vec<DeliveryAttributeMappingUnion>,
}
impl ServiceBusTopicEventSubscriptionDestinationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Static delivery attribute mapping details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticDeliveryAttributeMapping {
    #[serde(flatten)]
    pub delivery_attribute_mapping: DeliveryAttributeMapping,
    #[doc = "Properties of static delivery attribute mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StaticDeliveryAttributeMappingProperties>,
}
impl StaticDeliveryAttributeMapping {
    pub fn new(delivery_attribute_mapping: DeliveryAttributeMapping) -> Self {
        Self {
            delivery_attribute_mapping,
            properties: None,
        }
    }
}
#[doc = "Properties of static delivery attribute mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticDeliveryAttributeMappingProperties {
    #[doc = "Value of the delivery attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Boolean flag to tell if the attribute contains sensitive information ."]
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
}
impl StaticDeliveryAttributeMappingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the storage blob based dead letter destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageBlobDeadLetterDestination {
    #[doc = "Properties of the storage blob based dead letter destination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageBlobDeadLetterDestinationProperties>,
}
impl StorageBlobDeadLetterDestination {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Properties of the storage blob based dead letter destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageBlobDeadLetterDestinationProperties {
    #[doc = "The Azure Resource ID of the storage account that is the destination of the deadletter events"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The name of the Storage blob container that is the destination of the deadletter events"]
    #[serde(rename = "blobContainerName", default, skip_serializing_if = "Option::is_none")]
    pub blob_container_name: Option<String>,
}
impl StorageBlobDeadLetterDestinationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the storage queue destination for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageQueueEventSubscriptionDestination {
    #[doc = "The properties for a storage queue destination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageQueueEventSubscriptionDestinationProperties>,
}
impl StorageQueueEventSubscriptionDestination {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "The properties for a storage queue destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageQueueEventSubscriptionDestinationProperties {
    #[doc = "The Azure Resource ID of the storage account that contains the queue that is the destination of an event subscription."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The name of the Storage queue under a storage account that is the destination of an event subscription."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[doc = "Storage queue message time to live in seconds."]
    #[serde(rename = "queueMessageTimeToLiveInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub queue_message_time_to_live_in_seconds: Option<i64>,
}
impl StorageQueueEventSubscriptionDestinationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "StringBeginsWith Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringBeginsWithAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StringBeginsWithAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "StringContains Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringContainsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StringContainsAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "StringEndsWith Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringEndsWithAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StringEndsWithAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "StringIn Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringInAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StringInAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "StringNotBeginsWith Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringNotBeginsWithAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StringNotBeginsWithAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "StringNotContains Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringNotContainsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StringNotContainsAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "StringNotEndsWith Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringNotEndsWithAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StringNotEndsWithAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "StringNotIn Advanced Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringNotInAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[doc = "The set of filter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StringNotInAdvancedFilter {
    pub fn new(advanced_filter: AdvancedFilter) -> Self {
        Self {
            advanced_filter,
            values: Vec::new(),
        }
    }
}
#[doc = "EventGrid System Topic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemTopic {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the System Topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SystemTopicProperties>,
    #[doc = "The identity information for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityInfo>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SystemTopic {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
            system_data: None,
        }
    }
}
#[doc = "Properties of the System Topic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemTopicProperties {
    #[doc = "Provisioning state of the system topic."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<system_topic_properties::ProvisioningState>,
    #[doc = "Source for the system topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "TopicType for the system topic."]
    #[serde(rename = "topicType", default, skip_serializing_if = "Option::is_none")]
    pub topic_type: Option<String>,
    #[doc = "Metric resource id for the system topic."]
    #[serde(rename = "metricResourceId", default, skip_serializing_if = "Option::is_none")]
    pub metric_resource_id: Option<String>,
}
impl SystemTopicProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_topic_properties {
    use super::*;
    #[doc = "Provisioning state of the system topic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the System Topic update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemTopicUpdateParameters {
    #[doc = "Tags of the system topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The identity information for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityInfo>,
}
impl SystemTopicUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List System topics operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemTopicsListResult {
    #[doc = "A collection of system Topics."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SystemTopic>,
    #[doc = "A link for the next page of topics."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SystemTopicsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SystemTopicsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "EventGrid Topic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the Topic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopicProperties>,
    #[doc = "The identity information for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityInfo>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Topic {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
            system_data: None,
        }
    }
}
#[doc = "Properties of the Topic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicProperties {
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Provisioning state of the topic."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<topic_properties::ProvisioningState>,
    #[doc = "Endpoint for the topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "This determines the format that Event Grid should expect for incoming events published to the topic."]
    #[serde(rename = "inputSchema", default, skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<topic_properties::InputSchema>,
    #[doc = "By default, Event Grid expects events to be in the Event Grid event schema. Specifying an input schema mapping enables publishing to Event Grid using a custom input schema. Currently, the only supported type of InputSchemaMapping is 'JsonInputSchemaMapping'."]
    #[serde(rename = "inputSchemaMapping", default, skip_serializing_if = "Option::is_none")]
    pub input_schema_mapping: Option<InputSchemaMappingUnion>,
    #[doc = "Metric resource id for the topic."]
    #[serde(rename = "metricResourceId", default, skip_serializing_if = "Option::is_none")]
    pub metric_resource_id: Option<String>,
    #[doc = "This determines if traffic is allowed over public network. By default it is enabled. \r\nYou can further restrict to specific IPs by configuring <seealso cref=\"P:Microsoft.Azure.Events.ResourceProvider.Common.Contracts.TopicProperties.InboundIpRules\" />"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<topic_properties::PublicNetworkAccess>,
    #[doc = "This can be used to restrict traffic from specific IPs instead of all IPs. Note: These are considered only if PublicNetworkAccess is enabled."]
    #[serde(
        rename = "inboundIpRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_ip_rules: Vec<InboundIpRule>,
    #[doc = "This boolean is used to enable or disable local auth. Default value is false. When the property is set to true, only AAD token will be used to authenticate if user is allowed to publish to the topic."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
}
impl TopicProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod topic_properties {
    use super::*;
    #[doc = "Provisioning state of the topic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This determines the format that Event Grid should expect for incoming events published to the topic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InputSchema")]
    pub enum InputSchema {
        EventGridSchema,
        CustomEventSchema,
        #[serde(rename = "CloudEventSchemaV1_0")]
        CloudEventSchemaV10,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InputSchema {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InputSchema {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InputSchema {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EventGridSchema => serializer.serialize_unit_variant("InputSchema", 0u32, "EventGridSchema"),
                Self::CustomEventSchema => serializer.serialize_unit_variant("InputSchema", 1u32, "CustomEventSchema"),
                Self::CloudEventSchemaV10 => serializer.serialize_unit_variant("InputSchema", 2u32, "CloudEventSchemaV1_0"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for InputSchema {
        fn default() -> Self {
            Self::EventGridSchema
        }
    }
    #[doc = "This determines if traffic is allowed over public network. By default it is enabled. \r\nYou can further restrict to specific IPs by configuring <seealso cref=\"P:Microsoft.Azure.Events.ResourceProvider.Common.Contracts.TopicProperties.InboundIpRules\" />"]
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
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Topic regenerate share access key request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicRegenerateKeyRequest {
    #[doc = "Key name to regenerate key1 or key2"]
    #[serde(rename = "keyName")]
    pub key_name: String,
}
impl TopicRegenerateKeyRequest {
    pub fn new(key_name: String) -> Self {
        Self { key_name }
    }
}
#[doc = "Shared access keys of the Topic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicSharedAccessKeys {
    #[doc = "Shared access key1 for the topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[doc = "Shared access key2 for the topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
impl TopicSharedAccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a topic type info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicTypeInfo {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a topic type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopicTypeProperties>,
}
impl TopicTypeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a topic type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicTypeProperties {
    #[doc = "Namespace of the provider of the topic type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Display Name for the topic type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of the topic type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Region type of the resource."]
    #[serde(rename = "resourceRegionType", default, skip_serializing_if = "Option::is_none")]
    pub resource_region_type: Option<topic_type_properties::ResourceRegionType>,
    #[doc = "Provisioning state of the topic type"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<topic_type_properties::ProvisioningState>,
    #[doc = "List of locations supported by this topic type."]
    #[serde(
        rename = "supportedLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_locations: Vec<String>,
    #[doc = "Source resource format."]
    #[serde(rename = "sourceResourceFormat", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_format: Option<String>,
    #[doc = "Supported source scopes."]
    #[serde(
        rename = "supportedScopesForSource",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_scopes_for_source: Vec<String>,
}
impl TopicTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod topic_type_properties {
    use super::*;
    #[doc = "Region type of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceRegionType")]
    pub enum ResourceRegionType {
        RegionalResource,
        GlobalResource,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceRegionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceRegionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceRegionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RegionalResource => serializer.serialize_unit_variant("ResourceRegionType", 0u32, "RegionalResource"),
                Self::GlobalResource => serializer.serialize_unit_variant("ResourceRegionType", 1u32, "GlobalResource"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the topic type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the List Topic Types operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicTypesListResult {
    #[doc = "A collection of topic types"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TopicTypeInfo>,
}
impl azure_core::Continuable for TopicTypesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl TopicTypesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of topic update parameter properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicUpdateParameterProperties {
    #[doc = "This determines if traffic is allowed over public network. By default it is enabled. \r\nYou can further restrict to specific IPs by configuring <seealso cref=\"P:Microsoft.Azure.Events.ResourceProvider.Common.Contracts.TopicUpdateParameterProperties.InboundIpRules\" />"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<topic_update_parameter_properties::PublicNetworkAccess>,
    #[doc = "This can be used to restrict traffic from specific IPs instead of all IPs. Note: These are considered only if PublicNetworkAccess is enabled."]
    #[serde(
        rename = "inboundIpRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_ip_rules: Vec<InboundIpRule>,
    #[doc = "This boolean is used to enable or disable local auth. Default value is false. When the property is set to true, only AAD token will be used to authenticate if user is allowed to publish to the topic."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
}
impl TopicUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod topic_update_parameter_properties {
    use super::*;
    #[doc = "This determines if traffic is allowed over public network. By default it is enabled. \r\nYou can further restrict to specific IPs by configuring <seealso cref=\"P:Microsoft.Azure.Events.ResourceProvider.Common.Contracts.TopicUpdateParameterProperties.InboundIpRules\" />"]
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
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Properties of the Topic update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicUpdateParameters {
    #[doc = "Tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The identity information for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityInfo>,
    #[doc = "Information of topic update parameter properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopicUpdateParameterProperties>,
}
impl TopicUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List Topics operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopicsListResult {
    #[doc = "A collection of Topics"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Topic>,
    #[doc = "A link for the next page of topics"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TopicsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TopicsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a Tracked Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Location of the resource."]
    pub location: String,
    #[doc = "Tags of the resource."]
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
#[doc = "The information about the user identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentityProperties {
    #[doc = "The principal id of user assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client id of user assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserIdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the webhook destination for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebHookEventSubscriptionDestination {
    #[doc = "Information about the webhook destination properties for an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebHookEventSubscriptionDestinationProperties>,
}
impl WebHookEventSubscriptionDestination {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Information about the webhook destination properties for an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebHookEventSubscriptionDestinationProperties {
    #[doc = "The URL that represents the endpoint of the destination of an event subscription."]
    #[serde(rename = "endpointUrl", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[doc = "The base URL that represents the endpoint of the destination of an event subscription."]
    #[serde(rename = "endpointBaseUrl", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_base_url: Option<String>,
    #[doc = "Maximum number of events per batch."]
    #[serde(rename = "maxEventsPerBatch", default, skip_serializing_if = "Option::is_none")]
    pub max_events_per_batch: Option<i32>,
    #[doc = "Preferred batch size in Kilobytes."]
    #[serde(rename = "preferredBatchSizeInKilobytes", default, skip_serializing_if = "Option::is_none")]
    pub preferred_batch_size_in_kilobytes: Option<i32>,
    #[doc = "The Azure Active Directory Tenant ID to get the access token that will be included as the bearer token in delivery requests."]
    #[serde(rename = "azureActiveDirectoryTenantId", default, skip_serializing_if = "Option::is_none")]
    pub azure_active_directory_tenant_id: Option<String>,
    #[doc = "The Azure Active Directory Application ID or URI to get the access token that will be included as the bearer token in delivery requests."]
    #[serde(
        rename = "azureActiveDirectoryApplicationIdOrUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_active_directory_application_id_or_uri: Option<String>,
    #[doc = "Delivery attribute details."]
    #[serde(
        rename = "deliveryAttributeMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delivery_attribute_mappings: Vec<DeliveryAttributeMappingUnion>,
}
impl WebHookEventSubscriptionDestinationProperties {
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
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
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
