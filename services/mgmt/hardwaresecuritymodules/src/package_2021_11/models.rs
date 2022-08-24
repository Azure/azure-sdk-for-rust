#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The API entity reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiEntityReference {
    #[doc = "The ARM resource id in the form of /subscriptions/{SubscriptionId}/resourceGroups/{ResourceGroupName}/..."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ApiEntityReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedHsm {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of dedicated hsm resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of the dedicated hsm"]
    pub properties: DedicatedHsmProperties,
}
impl DedicatedHsm {
    pub fn new(resource: Resource, properties: DedicatedHsmProperties) -> Self {
        Self {
            resource,
            system_data: None,
            properties,
        }
    }
}
#[doc = "The error exception."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHsmError {
    #[doc = "The key vault server error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl azure_core::Continuable for DedicatedHsmError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DedicatedHsmError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of dedicated HSMs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHsmListResult {
    #[doc = "The list of dedicated HSMs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DedicatedHsm>,
    #[doc = "The URL to get the next set of dedicated hsms."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DedicatedHsmListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DedicatedHsmListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHsmOperation {
    #[doc = "The name of the Dedicated HSM Resource Provider Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets a value indicating whether it is a data plane action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "The display string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<dedicated_hsm_operation::Display>,
}
impl DedicatedHsmOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dedicated_hsm_operation {
    use super::*;
    #[doc = "The display string."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The Resource Provider of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The object that represents the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Dedicated HSM Provider operations. It contains a list of operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHsmOperationListResult {
    #[doc = "List of Dedicated HSM Resource Provider operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DedicatedHsmOperation>,
}
impl azure_core::Continuable for DedicatedHsmOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DedicatedHsmOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patchable properties of the dedicated HSM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHsmPatchParameters {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DedicatedHsmPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the dedicated hsm"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHsmProperties {
    #[doc = "The network profile definition."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "The network profile definition."]
    #[serde(rename = "managementNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub management_network_profile: Option<NetworkProfile>,
    #[doc = "This field will be used when RP does not support Availability zones."]
    #[serde(rename = "stampId", default, skip_serializing_if = "Option::is_none")]
    pub stamp_id: Option<String>,
    #[doc = "Resource Status Message."]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "Provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<dedicated_hsm_properties::ProvisioningState>,
}
impl DedicatedHsmProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dedicated_hsm_properties {
    use super::*;
    #[doc = "Provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Provisioning,
        Allocating,
        Connecting,
        Failed,
        CheckingQuota,
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
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Provisioning"),
                Self::Allocating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Allocating"),
                Self::Connecting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Connecting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::CheckingQuota => serializer.serialize_unit_variant("ProvisioningState", 5u32, "CheckingQuota"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A domain name that dedicated hsm services are reaching at."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The Ports and Protocols used when connecting to domainName."]
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connect information from the dedicated hsm service to a single endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[doc = "An IP Address that Domain Name currently resolves to."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The port an endpoint is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The protocol used for connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "Description of the detail"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl EndpointDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key vault server error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The key vault server error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<Error>>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdentityType")]
pub enum IdentityType {
    User,
    Application,
    ManagedIdentity,
    Key,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("IdentityType", 0u32, "User"),
            Self::Application => serializer.serialize_unit_variant("IdentityType", 1u32, "Application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("IdentityType", 2u32, "ManagedIdentity"),
            Self::Key => serializer.serialize_unit_variant("IdentityType", 3u32, "Key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The network interface definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[doc = "The ARM resource id in the form of /subscriptions/{SubscriptionId}/resourceGroups/{ResourceGroupName}/..."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Private Ip address of the interface"]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network profile definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "The API entity reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ApiEntityReference>,
    #[doc = "Specifies the list of resource Ids for the network interfaces associated with the dedicated HSM."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Egress endpoints which dedicated hsm service connects to for common purpose."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[doc = "The category of endpoints accessed by the dedicated hsm service, e.g. azure-resource-management, apiserver, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints that dedicated hsm service connects to"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of OutboundEnvironmentEndpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEnvironmentEndpointCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundEnvironmentEndpointCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OutboundEnvironmentEndpointCollection {
    pub fn new(value: Vec<OutboundEnvironmentEndpoint>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Dedicated HSM resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The Azure Resource Manager resource ID for the dedicated HSM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the dedicated HSM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of the dedicated HSM."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The supported Azure location where the dedicated HSM should be created."]
    pub location: String,
    #[doc = "SKU of the dedicated HSM"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The Dedicated Hsm zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            sku: None,
            zones: Vec::new(),
            tags: None,
        }
    }
}
#[doc = "List of dedicated HSM resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListResult {
    #[doc = "The list of dedicated HSM resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Resource>,
    #[doc = "The URL to get the next set of dedicated HSM resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU of the dedicated HSM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "SKU of the dedicated HSM"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku {
    use super::*;
    #[doc = "SKU of the dedicated HSM"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "SafeNet Luna Network HSM A790")]
        SafeNetLunaNetworkHsmA790,
        #[serde(rename = "payShield10K_LMK1_CPS60")]
        PayShield10KLmk1Cps60,
        #[serde(rename = "payShield10K_LMK1_CPS250")]
        PayShield10KLmk1Cps250,
        #[serde(rename = "payShield10K_LMK1_CPS2500")]
        PayShield10KLmk1Cps2500,
        #[serde(rename = "payShield10K_LMK2_CPS60")]
        PayShield10KLmk2Cps60,
        #[serde(rename = "payShield10K_LMK2_CPS250")]
        PayShield10KLmk2Cps250,
        #[serde(rename = "payShield10K_LMK2_CPS2500")]
        PayShield10KLmk2Cps2500,
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
                Self::SafeNetLunaNetworkHsmA790 => serializer.serialize_unit_variant("Name", 0u32, "SafeNet Luna Network HSM A790"),
                Self::PayShield10KLmk1Cps60 => serializer.serialize_unit_variant("Name", 1u32, "payShield10K_LMK1_CPS60"),
                Self::PayShield10KLmk1Cps250 => serializer.serialize_unit_variant("Name", 2u32, "payShield10K_LMK1_CPS250"),
                Self::PayShield10KLmk1Cps2500 => serializer.serialize_unit_variant("Name", 3u32, "payShield10K_LMK1_CPS2500"),
                Self::PayShield10KLmk2Cps60 => serializer.serialize_unit_variant("Name", 4u32, "payShield10K_LMK2_CPS60"),
                Self::PayShield10KLmk2Cps250 => serializer.serialize_unit_variant("Name", 5u32, "payShield10K_LMK2_CPS250"),
                Self::PayShield10KLmk2Cps2500 => serializer.serialize_unit_variant("Name", 6u32, "payShield10K_LMK2_CPS2500"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of dedicated hsm resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created dedicated hsm resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<IdentityType>,
    #[doc = "The timestamp of dedicated hsm resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified dedicated hsm resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<IdentityType>,
    #[doc = "The timestamp of dedicated hsm resource last modification (UTC)."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
