#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure Traffic Collector resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureTrafficCollector {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Azure Traffic Collector resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureTrafficCollectorPropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl AzureTrafficCollector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the ListTrafficCollectors API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureTrafficCollectorListResult {
    #[doc = "A list of Traffic Collector resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureTrafficCollector>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureTrafficCollectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AzureTrafficCollectorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Traffic Collector resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureTrafficCollectorPropertiesFormat {
    #[doc = "Collector Policies for Azure Traffic Collector."]
    #[serde(rename = "collectorPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub collector_policies: Vec<CollectorPolicy>,
    #[doc = "Resource reference properties."]
    #[serde(rename = "virtualHub", default, skip_serializing_if = "Option::is_none")]
    pub virtual_hub: Option<ResourceReference>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl AzureTrafficCollectorPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the service."]
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
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collector policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectorPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Collection policy properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CollectorPolicyPropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl CollectorPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the ListCollectorPolicies API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectorPolicyListResult {
    #[doc = "A list of collection policies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CollectorPolicy>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CollectorPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CollectorPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectorPolicyPropertiesFormat {
    #[doc = "Ingestion Policy properties."]
    #[serde(rename = "ingestionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_policy: Option<IngestionPolicyPropertiesFormat>,
    #[doc = "Emission policies."]
    #[serde(rename = "emissionPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub emission_policies: Vec<EmissionPoliciesPropertiesFormat>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl CollectorPolicyPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Emission policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmissionPoliciesPropertiesFormat {
    #[doc = "Emission format type."]
    #[serde(rename = "emissionType", default, skip_serializing_if = "Option::is_none")]
    pub emission_type: Option<emission_policies_properties_format::EmissionType>,
    #[doc = "Emission policy destinations."]
    #[serde(rename = "emissionDestinations", default, skip_serializing_if = "Vec::is_empty")]
    pub emission_destinations: Vec<EmissionPolicyDestination>,
}
impl EmissionPoliciesPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod emission_policies_properties_format {
    use super::*;
    #[doc = "Emission format type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EmissionType")]
    pub enum EmissionType {
        #[serde(rename = "IPFIX")]
        Ipfix,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EmissionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EmissionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EmissionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ipfix => serializer.serialize_unit_variant("EmissionType", 0u32, "IPFIX"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Emission policy destination properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmissionPolicyDestination {
    #[doc = "Emission destination type."]
    #[serde(rename = "destinationType", default, skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<emission_policy_destination::DestinationType>,
}
impl EmissionPolicyDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod emission_policy_destination {
    use super::*;
    #[doc = "Emission destination type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DestinationType")]
    pub enum DestinationType {
        AzureMonitor,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DestinationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DestinationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DestinationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureMonitor => serializer.serialize_unit_variant("DestinationType", 0u32, "AzureMonitor"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Ingestion Policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngestionPolicyPropertiesFormat {
    #[doc = "The ingestion type."]
    #[serde(rename = "ingestionType", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_type: Option<ingestion_policy_properties_format::IngestionType>,
    #[doc = "Ingestion Sources."]
    #[serde(rename = "ingestionSources", default, skip_serializing_if = "Vec::is_empty")]
    pub ingestion_sources: Vec<IngestionSourcesPropertiesFormat>,
}
impl IngestionPolicyPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ingestion_policy_properties_format {
    use super::*;
    #[doc = "The ingestion type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IngestionType")]
    pub enum IngestionType {
        #[serde(rename = "IPFIX")]
        Ipfix,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IngestionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IngestionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IngestionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ipfix => serializer.serialize_unit_variant("IngestionType", 0u32, "IPFIX"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Ingestion policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngestionSourcesPropertiesFormat {
    #[doc = "Ingestion source type."]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<ingestion_sources_properties_format::SourceType>,
    #[doc = "Resource ID."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl IngestionSourcesPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ingestion_sources_properties_format {
    use super::*;
    #[doc = "Ingestion source type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceType")]
    pub enum SourceType {
        Resource,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Resource => serializer.serialize_unit_variant("SourceType", 0u32, "Resource"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure Traffic Collector REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft NetworkFunction."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Azure Traffic Collector operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Azure Traffic Collector resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Updating,
    Deleting,
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
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource reference properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceReference {
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
#[doc = "Tags object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common resource representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
