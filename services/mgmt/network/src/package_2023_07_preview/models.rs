#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Access to be allowed or denied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Access")]
pub enum Access {
    Allow,
    Deny,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Access {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Access {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Access {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Allow => serializer.serialize_unit_variant("Access", 0u32, "Allow"),
            Self::Deny => serializer.serialize_unit_variant("Access", 1u32, "Deny"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VPN client authentication method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AuthenticationMethod")]
pub enum AuthenticationMethod {
    #[serde(rename = "EAPTLS")]
    Eaptls,
    #[serde(rename = "EAPMSCHAPv2")]
    EapmschaPv2,
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
            Self::Eaptls => serializer.serialize_unit_variant("AuthenticationMethod", 0u32, "EAPTLS"),
            Self::EapmschaPv2 => serializer.serialize_unit_variant("AuthenticationMethod", 1u32, "EAPMSCHAPv2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response body contains the status of the specified asynchronous operation, indicating whether it has succeeded, is in progress, or has failed. Note that this status is distinct from the HTTP status code returned for the Get Operation Status operation itself. If the asynchronous operation succeeded, the response body includes the HTTP status code for the successful request. If the asynchronous operation failed, the response body includes the HTTP status code for the failed request and error information regarding the failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAsyncOperationResult {
    #[doc = "Status of the Azure async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<azure_async_operation_result::Status>,
    #[doc = "Common error representation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl AzureAsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_async_operation_result {
    use super::*;
    #[doc = "Status of the Azure async operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 0u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetails>,
    #[doc = "Inner error message."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error details representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ExtendedLocation complex type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The name of the extended location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The supported ExtendedLocation types. Currently only EdgeZone is supported in Microsoft.Network resources."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ExtendedLocationType>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The supported ExtendedLocation types. Currently only EdgeZone is supported in Microsoft.Network resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExtendedLocationType")]
pub enum ExtendedLocationType {
    EdgeZone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExtendedLocationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExtendedLocationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExtendedLocationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EdgeZone => serializer.serialize_unit_variant("ExtendedLocationType", 0u32, "EdgeZone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "IP address allocation method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IpAllocationMethod")]
pub enum IpAllocationMethod {
    Static,
    Dynamic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IpAllocationMethod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IpAllocationMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IpAllocationMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Static => serializer.serialize_unit_variant("IpAllocationMethod", 0u32, "Static"),
            Self::Dynamic => serializer.serialize_unit_variant("IpAllocationMethod", 1u32, "Dynamic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "IP address version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IpVersion")]
pub enum IpVersion {
    IPv4,
    IPv6,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IpVersion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IpVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IpVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IPv4 => serializer.serialize_unit_variant("IpVersion", 0u32, "IPv4"),
            Self::IPv6 => serializer.serialize_unit_variant("IpVersion", 1u32, "IPv6"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServiceIdentity {
    #[doc = "The principal id of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<managed_service_identity::Type>,
    #[doc = "The list of user identities associated with resource. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedServiceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_service_identity {
    use super::*;
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "The current provisioning state of NSP Link/LinkReference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NspLinkProvisioningState")]
pub enum NspLinkProvisioningState {
    Succeeded,
    Creating,
    Updating,
    Deleting,
    Accepted,
    Failed,
    WaitForRemoteCompletion,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NspLinkProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NspLinkProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NspLinkProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("NspLinkProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("NspLinkProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("NspLinkProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("NspLinkProvisioningState", 3u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("NspLinkProvisioningState", 4u32, "Accepted"),
            Self::Failed => serializer.serialize_unit_variant("NspLinkProvisioningState", 5u32, "Failed"),
            Self::WaitForRemoteCompletion => serializer.serialize_unit_variant("NspLinkProvisioningState", 6u32, "WaitForRemoteCompletion"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NspProvisioningState")]
pub enum NspProvisioningState {
    Succeeded,
    Creating,
    Updating,
    Deleting,
    Accepted,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NspProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NspProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NspProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("NspProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("NspProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("NspProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("NspProvisioningState", 3u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("NspProvisioningState", 4u32, "Accepted"),
            Self::Failed => serializer.serialize_unit_variant("NspProvisioningState", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The Network Security Perimeter resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityPerimeter {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of network security perimeter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkSecurityPerimeterProperties>,
    #[doc = "The location in which NSP is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Network security perimeter identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NetworkSecurityPerimeter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list NetworkSecurityPerimeter. It contains a list of network security perimeters and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityPerimeterListResult {
    #[doc = "Gets a page of NetworkSecurityPerimeter"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkSecurityPerimeter>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkSecurityPerimeterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkSecurityPerimeterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of network security perimeter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityPerimeterProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<NspProvisioningState>,
    #[doc = "perimeter guid of the network security perimeter."]
    #[serde(rename = "perimeterGuid", default, skip_serializing_if = "Option::is_none")]
    pub perimeter_guid: Option<String>,
}
impl NetworkSecurityPerimeterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The NSP access rule resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspAccessRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of NSP access rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NspAccessRuleProperties>,
    #[doc = "The name of the access rule that is unique within a profile. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "NSP access rule identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NspAccessRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list NSP access rules. Contains a list of NSP access rules and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspAccessRuleListResult {
    #[doc = "Gets a page of NSP access rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NspAccessRule>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NspAccessRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NspAccessRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of NSP access rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspAccessRuleProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<NspProvisioningState>,
    #[doc = "Direction that specifies whether the access rules is inbound/outbound."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<nsp_access_rule_properties::Direction>,
    #[doc = "Inbound address prefixes (IPv4/IPv6)"]
    #[serde(
        rename = "addressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub address_prefixes: Vec<String>,
    #[doc = "Outbound rules fully qualified domain name format."]
    #[serde(
        rename = "fullyQualifiedDomainNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fully_qualified_domain_names: Vec<String>,
    #[doc = "List of subscription ids"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscriptions: Vec<SubscriptionId>,
    #[doc = "Rule specified by the perimeter id."]
    #[serde(
        rename = "networkSecurityPerimeters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_security_perimeters: Vec<PerimeterBasedAccessRule>,
    #[doc = "Outbound rules email address format."]
    #[serde(
        rename = "emailAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_addresses: Vec<String>,
    #[doc = "Outbound rules phone number format."]
    #[serde(
        rename = "phoneNumbers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub phone_numbers: Vec<String>,
}
impl NspAccessRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nsp_access_rule_properties {
    use super::*;
    #[doc = "Direction that specifies whether the access rules is inbound/outbound."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        Inbound,
        Outbound,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Direction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inbound => serializer.serialize_unit_variant("Direction", 0u32, "Inbound"),
                Self::Outbound => serializer.serialize_unit_variant("Direction", 1u32, "Outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request object for NSP reconcile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspAccessRuleReconcile {}
impl NspAccessRuleReconcile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The NSP resource association resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspAssociation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NspAssociationProperties>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "NSP resource association identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NspAssociation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspAssociationProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<NspProvisioningState>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "privateLinkResource", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource: Option<SubResource>,
    #[doc = "Reference to another subresource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<SubResource>,
    #[doc = "Access mode on the association."]
    #[serde(rename = "accessMode", default, skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<nsp_association_properties::AccessMode>,
    #[doc = "Specifies if there are provisioning issues"]
    #[serde(rename = "hasProvisioningIssues", default, skip_serializing_if = "Option::is_none")]
    pub has_provisioning_issues: Option<String>,
}
impl NspAssociationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nsp_association_properties {
    use super::*;
    #[doc = "Access mode on the association."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessMode")]
    pub enum AccessMode {
        Learning,
        Enforced,
        Audit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Learning => serializer.serialize_unit_variant("AccessMode", 0u32, "Learning"),
                Self::Enforced => serializer.serialize_unit_variant("AccessMode", 1u32, "Enforced"),
                Self::Audit => serializer.serialize_unit_variant("AccessMode", 2u32, "Audit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request object for NSP association."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspAssociationReconcile {}
impl NspAssociationReconcile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list NSP resource associations. Contains a list of NSP resource associations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspAssociationsListResult {
    #[doc = "Gets a page of NSP resource associations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NspAssociation>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NspAssociationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NspAssociationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network security perimeter link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of NSP Link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NspLinkProperties>,
}
impl NspLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list NSP link resources. Contains a list of NSP link resources and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspLinkListResult {
    #[doc = "Gets a page of NSP Link resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NspLink>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NspLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NspLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of NSP Link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspLinkProperties {
    #[doc = "The current provisioning state of NSP Link/LinkReference."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<NspLinkProvisioningState>,
    #[doc = "Perimeter ARM Id for the remote NSP with which the link gets created in Auto-approval mode. It should be used when the NSP admin have Microsoft.Network/networkSecurityPerimeters/linkPerimeter/action permission on the remote NSP resource."]
    #[serde(rename = "autoApprovedRemotePerimeterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub auto_approved_remote_perimeter_resource_id: Option<String>,
    #[doc = "Remote NSP Guid with which the link gets created."]
    #[serde(rename = "remotePerimeterGuid", default, skip_serializing_if = "Option::is_none")]
    pub remote_perimeter_guid: Option<String>,
    #[doc = "Remote NSP location with which the link gets created."]
    #[serde(rename = "remotePerimeterLocation", default, skip_serializing_if = "Option::is_none")]
    pub remote_perimeter_location: Option<String>,
    #[doc = "Local Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles. It's default value is ['*']."]
    #[serde(
        rename = "localInboundProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub local_inbound_profiles: Vec<String>,
    #[doc = "Local Outbound profile names from which Outbound is allowed. In current version, it is readonly property and it's value is set to ['*'] to allow outbound from all profiles. In later version, user will be able to modify it."]
    #[serde(
        rename = "localOutboundProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub local_outbound_profiles: Vec<String>,
    #[doc = "Remote Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles. This property can only be updated in auto-approval mode. It's default value is ['*']."]
    #[serde(
        rename = "remoteInboundProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub remote_inbound_profiles: Vec<String>,
    #[doc = "Remote Outbound profile names from which Outbound is allowed. In current version, it is readonly property and it's value is set to ['*'] to allow outbound from all profiles. In later version, user will be able to modify it."]
    #[serde(
        rename = "remoteOutboundProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub remote_outbound_profiles: Vec<String>,
    #[doc = "A message passed to the owner of the remote NSP link resource with this connection request. In case of Auto-approved flow, it is default to 'Auto Approved'. Restricted to 140 chars."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The NSP link state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<nsp_link_properties::Status>,
}
impl NspLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nsp_link_properties {
    use super::*;
    #[doc = "The NSP link state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Approved,
        Pending,
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
                Self::Approved => serializer.serialize_unit_variant("Status", 0u32, "Approved"),
                Self::Pending => serializer.serialize_unit_variant("Status", 1u32, "Pending"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 3u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The network security perimeter linkReference resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspLinkReference {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of NSP LinkReference resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NspLinkReferenceProperties>,
}
impl NspLinkReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list NSP linkReference resources. Contains a list of NSP linkReference resources and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspLinkReferenceListResult {
    #[doc = "Gets a page of NSP LinkReference resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NspLinkReference>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NspLinkReferenceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NspLinkReferenceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of NSP LinkReference resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspLinkReferenceProperties {
    #[doc = "The current provisioning state of NSP Link/LinkReference."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<NspLinkProvisioningState>,
    #[doc = "Perimeter ARM Id for the remote NSP with which the link is created."]
    #[serde(rename = "remotePerimeterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub remote_perimeter_resource_id: Option<String>,
    #[doc = "Remote NSP Guid with which the link is created."]
    #[serde(rename = "remotePerimeterGuid", default, skip_serializing_if = "Option::is_none")]
    pub remote_perimeter_guid: Option<String>,
    #[doc = "Remote NSP location with which the link gets created."]
    #[serde(rename = "remotePerimeterLocation", default, skip_serializing_if = "Option::is_none")]
    pub remote_perimeter_location: Option<String>,
    #[doc = "Local Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles. It's default value is ['*']."]
    #[serde(
        rename = "localInboundProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub local_inbound_profiles: Vec<String>,
    #[doc = "Local Outbound profile names from which Outbound is allowed. Use ['*'] to allow outbound from all profiles. It's default value is ['*']."]
    #[serde(
        rename = "localOutboundProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub local_outbound_profiles: Vec<String>,
    #[doc = "Remote Inbound profile names to which Inbound is allowed. ['*'] value implies inbound is allowed to all profiles at remote perimeter. This property can only be updated from remote perimeter."]
    #[serde(
        rename = "remoteInboundProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub remote_inbound_profiles: Vec<String>,
    #[doc = "Remote Outbound profile names from which Outbound is allowed. ['*'] value implies outbound is allowed from all profiles at remote perimeter. This property can only be updated from remote perimeter."]
    #[serde(
        rename = "remoteOutboundProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub remote_outbound_profiles: Vec<String>,
    #[doc = "A message sent by the remote NSP link admin for connection request. In case of Auto-approved flow, it is default to 'Auto Approved'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The NSP linkReference state. It cannot be changed if link is created in auto-approval mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<nsp_link_reference_properties::Status>,
}
impl NspLinkReferenceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nsp_link_reference_properties {
    use super::*;
    #[doc = "The NSP linkReference state. It cannot be changed if link is created in auto-approval mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Approved,
        Pending,
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
                Self::Approved => serializer.serialize_unit_variant("Status", 0u32, "Approved"),
                Self::Pending => serializer.serialize_unit_variant("Status", 1u32, "Pending"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 3u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The network security perimeter profile resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspProfile {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of NSP profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NspProfileProperties>,
    #[doc = "The name of the profile resource that is unique within a perimeter. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Identifier of the network security perimeter profile in ARM id format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NspProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list NSP profiles. Contains a list of NSP profiles and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspProfileListResult {
    #[doc = "Gets a page of NSP profile"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NspProfile>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NspProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NspProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of NSP profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspProfileProperties {
    #[doc = "Version number that increases with every update to access rules within the profile."]
    #[serde(rename = "accessRulesVersion", default, skip_serializing_if = "Option::is_none")]
    pub access_rules_version: Option<String>,
    #[doc = "Version number that increases with every update to diagnostic settings within the profile."]
    #[serde(rename = "diagnosticSettingsVersion", default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_settings_version: Option<String>,
}
impl NspProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource that is onboarded to use network security perimeter. Also referred as perimeter associable resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerimeterAssociableResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the perimeter associable resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PerimeterAssociableResourceProperties>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Identifier of the perimeter associable resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PerimeterAssociableResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the perimeter associable resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerimeterAssociableResourceProperties {
    #[doc = "A friendly name for the properties of perimeter associable resources."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Resource type/provider name."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Public DNS zone names of the resources."]
    #[serde(
        rename = "publicDnsZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub public_dns_zones: Vec<String>,
}
impl PerimeterAssociableResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of perimeter associable resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerimeterAssociableResourcesListResult {
    #[doc = "Gets paged list of perimeter associable resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PerimeterAssociableResource>,
    #[doc = "Gets the URL to get the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PerimeterAssociableResourcesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PerimeterAssociableResourcesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerimeterBasedAccessRule {
    #[doc = "NSP id in the ARM id format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource guid of the NSP supplied."]
    #[serde(rename = "perimeterGuid", default, skip_serializing_if = "Option::is_none")]
    pub perimeter_guid: Option<String>,
    #[doc = "Location of the NSP supplied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl PerimeterBasedAccessRule {
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
#[doc = "Proxy resource representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common resource representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to another subresource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionId {
    #[doc = "Subscription id in the ARM id format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubscriptionId {
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
    #[doc = "The type of identity that last modified the resource."]
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
#[doc = "Update tags request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateTagsRequest {
    #[doc = "Network security perimeter identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "List of tags for Network Security Perimeter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateTagsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
