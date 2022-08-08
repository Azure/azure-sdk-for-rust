#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Detailed HANA operation information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Display {
    #[doc = "The localized friendly form of the resource provider name. This form is also expected to include the publisher/company responsible. Use Title Casing. Begin with \"Microsoft\" for 1st party services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The localized friendly form of the resource type related to this action/operation. This form should match the public documentation for the resource provider. Use Title Casing. For examples, refer to the “name” section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The localized friendly name for the operation as shown to the user. This name should be concise (to fit in drop downs), but clear (self-documenting). Use Title Casing and include the entity/resource to which it applies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly description for the operation as shown to the user. This description should be thorough, yet concise. It will be used in tool-tips and detailed views."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX. Default value is 'user,system'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Display {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Describes the error object."]
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
    #[doc = "Describes the error object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code"]
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
#[doc = "HANA operation information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object. This name should match the action name that appears in RBAC / the event service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Detailed HANA operation information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of HANA operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of HANA operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A provider instance associated with a SAP monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderInstance {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes the properties of a provider instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProviderInstanceProperties>,
}
impl ProviderInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List provider instances operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderInstanceListResult {
    #[doc = "The list of provider instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderInstance>,
    #[doc = "The URL to get the next set of provider instances."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a provider instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderInstanceProperties {
    #[doc = "The type of provider instance."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "A JSON string containing the properties of the provider instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[doc = "A JSON string containing metadata of the provider instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[doc = "State of provisioning of the provider instance"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<provider_instance_properties::ProvisioningState>,
}
impl ProviderInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provider_instance_properties {
    use super::*;
    #[doc = "State of provisioning of the provider instance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Migrating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Migrating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "SAP monitor info on Azure (ARM properties and SAP monitor properties)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapMonitor {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of a SAP monitor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapMonitorProperties>,
}
impl SapMonitor {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response from the List SAP monitors operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapMonitorListResult {
    #[doc = "The list of SAP monitors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SapMonitor>,
    #[doc = "The URL to get the next set of SAP monitors."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapMonitorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SapMonitorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a SAP monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapMonitorProperties {
    #[doc = "State of provisioning of the HanaInstance"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<sap_monitor_properties::ProvisioningState>,
    #[doc = "The name of the resource group the SAP Monitor resources get deployed into."]
    #[serde(rename = "managedResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_name: Option<String>,
    #[doc = "The ARM ID of the Log Analytics Workspace that is used for monitoring"]
    #[serde(rename = "logAnalyticsWorkspaceArmId", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_arm_id: Option<String>,
    #[doc = "The value indicating whether to send analytics to Microsoft"]
    #[serde(rename = "enableCustomerAnalytics", default, skip_serializing_if = "Option::is_none")]
    pub enable_customer_analytics: Option<bool>,
    #[doc = "The workspace ID of the log analytics workspace to be used for monitoring"]
    #[serde(rename = "logAnalyticsWorkspaceId", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_id: Option<String>,
    #[doc = "The shared key of the log analytics workspace that is used for monitoring"]
    #[serde(rename = "logAnalyticsWorkspaceSharedKey", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_shared_key: Option<String>,
    #[doc = "The version of the payload running in the Collector VM"]
    #[serde(rename = "sapMonitorCollectorVersion", default, skip_serializing_if = "Option::is_none")]
    pub sap_monitor_collector_version: Option<String>,
    #[doc = "The subnet which the SAP monitor will be deployed in"]
    #[serde(rename = "monitorSubnet", default, skip_serializing_if = "Option::is_none")]
    pub monitor_subnet: Option<String>,
}
impl SapMonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sap_monitor_properties {
    use super::*;
    #[doc = "State of provisioning of the HanaInstance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Migrating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Migrating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Tags field of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "Tags field of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
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
