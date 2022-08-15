#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A class representing the access keys of a CommunicationService."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationServiceKeys {
    #[doc = "The primary access key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary access key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "CommunicationService connection string constructed via the primaryKey"]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "CommunicationService connection string constructed via the secondaryKey"]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
}
impl CommunicationServiceKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class that describes the properties of the CommunicationService."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationServiceProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<communication_service_properties::ProvisioningState>,
    #[doc = "FQDN of the CommunicationService instance."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The location where the communication service stores its data at rest."]
    #[serde(rename = "dataLocation")]
    pub data_location: String,
    #[doc = "Resource ID of an Azure Notification Hub linked to this resource."]
    #[serde(rename = "notificationHubId", default, skip_serializing_if = "Option::is_none")]
    pub notification_hub_id: Option<String>,
    #[doc = "Version of the CommunicationService resource. Probably you need the same or higher version of client SDKs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The immutable resource Id of the communication service."]
    #[serde(rename = "immutableResourceId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_resource_id: Option<String>,
}
impl CommunicationServiceProperties {
    pub fn new(data_location: String) -> Self {
        Self {
            provisioning_state: None,
            host_name: None,
            data_location,
            notification_hub_id: None,
            version: None,
            immutable_resource_id: None,
        }
    }
}
pub mod communication_service_properties {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Succeeded,
        Failed,
        Canceled,
        Running,
        Creating,
        Updating,
        Deleting,
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
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Running"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Moving"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A class representing a CommunicationService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationServiceResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub location_resource: LocationResource,
    #[serde(flatten)]
    pub tagged_resource: TaggedResource,
    #[doc = "A class that describes the properties of the CommunicationService."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommunicationServiceProperties>,
}
impl CommunicationServiceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of CommunicationServices and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationServiceResourceList {
    #[doc = "List of CommunicationService"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommunicationServiceResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CommunicationServiceResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommunicationServiceResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the Dimension of metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[doc = "The public facing name of the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Name of the dimension as it appears in MDM."]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[doc = "A Boolean flag indicating whether this dimension should be included for the shoebox export scenario."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicating why the requested operation could not be performed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
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
pub mod error_response {
    use super::*;
    #[doc = "The error"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Description of an Azure Notification Hub to link to the communication service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkNotificationHubParameters {
    #[doc = "The resource ID of the notification hub"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Connection string for the notification hub"]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
}
impl LinkNotificationHubParameters {
    pub fn new(resource_id: String, connection_string: String) -> Self {
        Self {
            resource_id,
            connection_string,
        }
    }
}
#[doc = "A notification hub that has been linked to the communication service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedNotificationHub {
    #[doc = "The resource ID of the notification hub"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl LinkedNotificationHub {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An ARM resource with its own location (not a global or an inherited location)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationResource {
    #[doc = "The Azure location where the CommunicationService is running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl LocationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the Metrics for Azure Monitoring."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localized friendly description of the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The unit that makes sense for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The method for aggregating the metric."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<metric_specification::AggregationType>,
    #[doc = "Optional. If set to true, then zero will be returned for time duration where no metric is emitted/published. \r\nEx. a metric that returns the number of times a particular error code was emitted. The error code may not appear \r\noften, instead of the RP publishing 0, Shoebox can auto fill in 0s for time periods where nothing was emitted."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<String>,
    #[doc = "The name of the metric category that the metric belongs to. A metric can only belong to a single category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The dimensions of the metrics."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric_specification {
    use super::*;
    #[doc = "The method for aggregating the metric."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AggregationType")]
    pub enum AggregationType {
        Average,
        Minimum,
        Maximum,
        Total,
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AggregationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AggregationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AggregationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Average => serializer.serialize_unit_variant("AggregationType", 0u32, "Average"),
                Self::Minimum => serializer.serialize_unit_variant("AggregationType", 1u32, "Minimum"),
                Self::Maximum => serializer.serialize_unit_variant("AggregationType", 2u32, "Maximum"),
                Self::Total => serializer.serialize_unit_variant("AggregationType", 3u32, "Total"),
                Self::Count => serializer.serialize_unit_variant("AggregationType", 4u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the request to check name availability. It contains a flag and possible reason of failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[doc = "Indicates whether the name is available or not."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason of the availability. Required if name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The message of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data POST-ed to the nameAvailability action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityParameters {
    #[doc = "The resource type. Should be always \"Microsoft.Communication/CommunicationServices\"."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The CommunicationService name to validate. e.g.\"my-CommunicationService-name-here\""]
    pub name: String,
}
impl NameAvailabilityParameters {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "REST API operation supported by CommunicationService resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation with format: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that describes a operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Optional. The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Extra Operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that describes a operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Friendly name of the resource provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource type on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The localized friendly name for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly description for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list REST API operations. It contains a list of operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of operations supported by the resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extra Operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "An object that describes a specification."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current status of an async operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Fully qualified ID for the operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Provisioning state of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_status::Status>,
    #[doc = "The start time of the operation"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Percent of the operation that is complete"]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "Error response indicating why the requested operation could not be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_status {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
        Failed,
        Canceled,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 2u32, "Canceled"),
                Self::Creating => serializer.serialize_unit_variant("Status", 3u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("Status", 4u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("Status", 5u32, "Moving"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters describes the request to regenerate access keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeyParameters {
    #[doc = "The keyType to regenerate. Must be either 'primary' or 'secondary'(case-insensitive)."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<regenerate_key_parameters::KeyType>,
}
impl RegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regenerate_key_parameters {
    use super::*;
    #[doc = "The keyType to regenerate. Must be either 'primary' or 'secondary'(case-insensitive)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        Primary,
        Secondary,
    }
}
#[doc = "The core properties of ARM resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the service - e.g. \"Microsoft.Communication/CommunicationServices\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that describes a specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Specifications of the Metrics for Azure Monitoring."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An ARM resource with that can accept tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaggedResource {
    #[doc = "Tags of the service which is a list of key value pairs that describe the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TaggedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
