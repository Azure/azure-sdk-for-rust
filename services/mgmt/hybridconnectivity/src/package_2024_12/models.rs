#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The AAD Profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadProfileProperties {
    #[doc = "The arc ingress gateway server app id."]
    #[serde(rename = "serverId")]
    pub server_id: String,
    #[doc = "The target resource home tenant id."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
impl AadProfileProperties {
    pub fn new(server_id: String, tenant_id: String) -> Self {
        Self { server_id, tenant_id }
    }
}
#[doc = "cloud profile for AWS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudProfile {
    #[doc = "Account id for the AWS account."]
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[doc = "List of AWS accounts which need to be excluded."]
    #[serde(
        rename = "excludedAccounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub excluded_accounts: Vec<String>,
    #[doc = "Boolean value that indicates whether the account is organizational or not. True represents organization account, whereas false represents a single account."]
    #[serde(rename = "isOrganizationalAccount", default, skip_serializing_if = "Option::is_none")]
    pub is_organizational_account: Option<bool>,
}
impl AwsCloudProfile {
    pub fn new(account_id: String) -> Self {
        Self {
            account_id,
            excluded_accounts: Vec::new(),
            is_organizational_account: None,
        }
    }
}
#[doc = "cloud profile for AWS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AwsCloudProfileUpdate {
    #[doc = "List of AWS accounts which need to be excluded."]
    #[serde(
        rename = "excludedAccounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub excluded_accounts: Vec<String>,
}
impl AwsCloudProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceManagerCommonTypesTrackedResourceUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AzureResourceManagerCommonTypesTrackedResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of a resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureResourceManagerResourceProvisioningState")]
pub enum AzureResourceManagerResourceProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureResourceManagerResourceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureResourceManagerResourceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureResourceManagerResourceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Cloud Native Type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloudNativeType")]
pub enum CloudNativeType {
    #[serde(rename = "ec2")]
    Ec2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloudNativeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloudNativeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloudNativeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ec2 => serializer.serialize_unit_variant("CloudNativeType", 0u32, "ec2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The endpoint access for the target resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointAccessResource {
    #[doc = "Azure relay hybrid connection access properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relay: Option<RelayNamespaceAccessProperties>,
}
impl EndpointAccessResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Endpoint details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    #[doc = "The type of endpoint."]
    #[serde(rename = "type")]
    pub type_: endpoint_properties::Type,
    #[doc = "The resource Id of the connectivity endpoint (optional)."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl EndpointProperties {
    pub fn new(type_: endpoint_properties::Type) -> Self {
        Self {
            type_,
            resource_id: None,
            provisioning_state: None,
        }
    }
}
pub mod endpoint_properties {
    use super::*;
    #[doc = "The type of endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "custom")]
        Custom,
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
                Self::Default => serializer.serialize_unit_variant("Type", 0u32, "default"),
                Self::Custom => serializer.serialize_unit_variant("Type", 1u32, "custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The endpoint for the target resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Endpoint details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointProperties>,
}
impl EndpointResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointsList {
    #[doc = "The link used to get the next page of endpoints list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of endpoint."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EndpointResource>,
}
impl azure_core::Continuable for EndpointsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EndpointsList {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "ConnectorId and SolutionTypes and their properties to Generate AWS CFT Template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateAwsTemplateRequest {
    #[doc = "The name of public cloud connector"]
    #[serde(rename = "connectorId")]
    pub connector_id: String,
    #[doc = "The list of solution types and their settings"]
    #[serde(
        rename = "solutionTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub solution_types: Vec<SolutionTypeSettings>,
}
impl GenerateAwsTemplateRequest {
    pub fn new(connector_id: String) -> Self {
        Self {
            connector_id,
            solution_types: Vec::new(),
        }
    }
}
#[doc = "Enum of host cloud the public cloud connector is referencing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HostType")]
pub enum HostType {
    #[serde(rename = "AWS")]
    Aws,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HostType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HostType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HostType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Aws => serializer.serialize_unit_variant("HostType", 0u32, "AWS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The ingress gateway access credentials"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngressGatewayResource {
    #[doc = "Azure relay hybrid connection access properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relay: Option<RelayNamespaceAccessProperties>,
    #[doc = "Ingress gateway profile"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<IngressProfileProperties>,
}
impl IngressGatewayResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ingress gateway profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IngressProfileProperties {
    #[doc = "The ingress hostname."]
    pub hostname: String,
    #[doc = "The AAD Profile"]
    #[serde(rename = "aadProfile")]
    pub aad_profile: AadProfileProperties,
}
impl IngressProfileProperties {
    pub fn new(hostname: String, aad_profile: AadProfileProperties) -> Self {
        Self { hostname, aad_profile }
    }
}
#[doc = "Definition of inventory."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryProperties {
    #[doc = "Cloud Native Type enum."]
    #[serde(rename = "cloudNativeType", default, skip_serializing_if = "Option::is_none")]
    pub cloud_native_type: Option<CloudNativeType>,
    #[doc = "Gets or sets the cloud native resource name."]
    #[serde(rename = "cloudNativeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub cloud_native_resource_id: Option<String>,
    #[doc = "Gets or sets the mapped azure resource id."]
    #[serde(rename = "azureResourceId", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_id: Option<String>,
    #[doc = "Solution Configuration Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SolutionConfigurationStatus>,
    #[doc = "Gets or sets the status details."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
}
impl InventoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Concrete proxy resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of inventory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InventoryProperties>,
}
impl InventoryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a InventoryResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryResourceListResult {
    #[doc = "The InventoryResource items on this page"]
    pub value: Vec<InventoryResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InventoryResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InventoryResourceListResult {
    pub fn new(value: Vec<InventoryResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The details of the service for which credentials needs to be returned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListCredentialsRequest {
    #[doc = "The name of the service. If not provided, the request will by pass the generation of service configuration token "]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<list_credentials_request::ServiceName>,
}
impl ListCredentialsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod list_credentials_request {
    use super::*;
    #[doc = "The name of the service. If not provided, the request will by pass the generation of service configuration token "]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceName")]
    pub enum ServiceName {
        #[serde(rename = "SSH")]
        Ssh,
        #[serde(rename = "WAC")]
        Wac,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ssh => serializer.serialize_unit_variant("ServiceName", 0u32, "SSH"),
                Self::Wac => serializer.serialize_unit_variant("ServiceName", 1u32, "WAC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represent ListIngressGatewayCredentials Request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListIngressGatewayCredentialsRequest {
    #[doc = "The name of the service."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<list_ingress_gateway_credentials_request::ServiceName>,
}
impl ListIngressGatewayCredentialsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod list_ingress_gateway_credentials_request {
    use super::*;
    #[doc = "The name of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceName")]
    pub enum ServiceName {
        #[serde(rename = "SSH")]
        Ssh,
        #[serde(rename = "WAC")]
        Wac,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ssh => serializer.serialize_unit_variant("ServiceName", 0u32, "SSH"),
                Self::Wac => serializer.serialize_unit_variant("ServiceName", 1u32, "WAC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represent ManageProxy Request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedProxyRequest {
    #[doc = "The name of the service."]
    pub service: String,
    #[doc = "The target host name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "The name of the service. It is an optional property, if not provided, service configuration tokens issue code would be by passed."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<managed_proxy_request::ServiceName>,
}
impl ManagedProxyRequest {
    pub fn new(service: String) -> Self {
        Self {
            service,
            hostname: None,
            service_name: None,
        }
    }
}
pub mod managed_proxy_request {
    use super::*;
    #[doc = "The name of the service. It is an optional property, if not provided, service configuration tokens issue code would be by passed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceName")]
    pub enum ServiceName {
        #[serde(rename = "SSH")]
        Ssh,
        #[serde(rename = "WAC")]
        Wac,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ssh => serializer.serialize_unit_variant("ServiceName", 0u32, "SSH"),
                Self::Wac => serializer.serialize_unit_variant("ServiceName", 1u32, "WAC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Managed Proxy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedProxyResource {
    #[doc = "The short lived proxy name."]
    pub proxy: String,
    #[doc = "The expiration time of short lived proxy name in unix epoch."]
    #[serde(rename = "expiresOn")]
    pub expires_on: i64,
}
impl ManagedProxyResource {
    pub fn new(proxy: String, expires_on: i64) -> Self {
        Self { proxy, expires_on }
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
#[doc = "Public Cloud Connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicCloudConnector {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of public cloud connectors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PublicCloudConnectorProperties>,
}
impl PublicCloudConnector {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a PublicCloudConnector list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicCloudConnectorListResult {
    #[doc = "The PublicCloudConnector items on this page"]
    pub value: Vec<PublicCloudConnector>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublicCloudConnectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PublicCloudConnectorListResult {
    pub fn new(value: Vec<PublicCloudConnector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of public cloud connectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicCloudConnectorProperties {
    #[doc = "cloud profile for AWS."]
    #[serde(rename = "awsCloudProfile")]
    pub aws_cloud_profile: AwsCloudProfile,
    #[doc = "Enum of host cloud the public cloud connector is referencing."]
    #[serde(rename = "hostType")]
    pub host_type: HostType,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
    #[doc = "Connector primary identifier."]
    #[serde(rename = "connectorPrimaryIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub connector_primary_identifier: Option<String>,
}
impl PublicCloudConnectorProperties {
    pub fn new(aws_cloud_profile: AwsCloudProfile, host_type: HostType) -> Self {
        Self {
            aws_cloud_profile,
            host_type,
            provisioning_state: None,
            connector_primary_identifier: None,
        }
    }
}
#[doc = "Properties of public cloud connectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicCloudConnectorPropertiesUpdate {
    #[doc = "cloud profile for AWS."]
    #[serde(rename = "awsCloudProfile", default, skip_serializing_if = "Option::is_none")]
    pub aws_cloud_profile: Option<AwsCloudProfileUpdate>,
}
impl PublicCloudConnectorPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Public Cloud Connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicCloudConnectorUpdate {
    #[serde(flatten)]
    pub azure_resource_manager_common_types_tracked_resource_update: AzureResourceManagerCommonTypesTrackedResourceUpdate,
    #[doc = "Properties of public cloud connectors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PublicCloudConnectorPropertiesUpdate>,
}
impl PublicCloudConnectorUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure relay hybrid connection access properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelayNamespaceAccessProperties {
    #[doc = "The namespace name."]
    #[serde(rename = "namespaceName")]
    pub namespace_name: String,
    #[doc = "The suffix domain name of relay namespace."]
    #[serde(rename = "namespaceNameSuffix")]
    pub namespace_name_suffix: String,
    #[doc = "Azure Relay hybrid connection name for the resource."]
    #[serde(rename = "hybridConnectionName")]
    pub hybrid_connection_name: String,
    #[doc = "Access key for hybrid connection."]
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[doc = "The expiration of access key in unix time."]
    #[serde(rename = "expiresOn", default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<i64>,
    #[doc = "The token to access the enabled service."]
    #[serde(rename = "serviceConfigurationToken", default, skip_serializing_if = "Option::is_none")]
    pub service_configuration_token: Option<String>,
}
impl RelayNamespaceAccessProperties {
    pub fn new(namespace_name: String, namespace_name_suffix: String, hybrid_connection_name: String) -> Self {
        Self {
            namespace_name,
            namespace_name_suffix,
            hybrid_connection_name,
            access_key: None,
            expires_on: None,
            service_configuration_token: None,
        }
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
#[doc = "The paginated list of serviceConfigurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceConfigurationList {
    #[doc = "The list of service configuration"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServiceConfigurationResource>,
    #[doc = "The link to fetch the next page of connected cluster"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServiceConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service configuration details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceConfigurationProperties {
    #[doc = "Name of the service."]
    #[serde(rename = "serviceName")]
    pub service_name: service_configuration_properties::ServiceName,
    #[doc = "The resource Id of the connectivity endpoint (optional)."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The port on which service is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "The resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<service_configuration_properties::ProvisioningState>,
}
impl ServiceConfigurationProperties {
    pub fn new(service_name: service_configuration_properties::ServiceName) -> Self {
        Self {
            service_name,
            resource_id: None,
            port: None,
            provisioning_state: None,
        }
    }
}
pub mod service_configuration_properties {
    use super::*;
    #[doc = "Name of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceName")]
    pub enum ServiceName {
        #[serde(rename = "SSH")]
        Ssh,
        #[serde(rename = "WAC")]
        Wac,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ssh => serializer.serialize_unit_variant("ServiceName", 0u32, "SSH"),
                Self::Wac => serializer.serialize_unit_variant("ServiceName", 1u32, "WAC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The resource provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Updating,
        Failed,
        Canceled,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Service configuration details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceConfigurationPropertiesPatch {
    #[doc = "The port on which service is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}
impl ServiceConfigurationPropertiesPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The service configuration details associated with the target resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceConfigurationResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Service configuration details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceConfigurationProperties>,
}
impl ServiceConfigurationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The service details under service configuration for the target endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceConfigurationResourcePatch {
    #[doc = "Service configuration details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceConfigurationPropertiesPatch>,
}
impl ServiceConfigurationResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution Configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Solution configuration resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SolutionConfigurationProperties>,
}
impl SolutionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SolutionConfiguration list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionConfigurationListResult {
    #[doc = "The SolutionConfiguration items on this page"]
    pub value: Vec<SolutionConfiguration>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SolutionConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SolutionConfigurationListResult {
    pub fn new(value: Vec<SolutionConfiguration>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Solution configuration resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionConfigurationProperties {
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
    #[doc = "The type of the solution"]
    #[serde(rename = "solutionType")]
    pub solution_type: String,
    #[doc = "Solution settings"]
    #[serde(rename = "solutionSettings", default, skip_serializing_if = "Option::is_none")]
    pub solution_settings: Option<SolutionSettings>,
    #[doc = "Solution Configuration Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SolutionConfigurationStatus>,
    #[doc = "The detailed message of status details"]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "The last time resources were inventoried"]
    #[serde(rename = "lastSyncTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_sync_time: Option<::time::OffsetDateTime>,
}
impl SolutionConfigurationProperties {
    pub fn new(solution_type: String) -> Self {
        Self {
            provisioning_state: None,
            solution_type,
            solution_settings: None,
            status: None,
            status_details: None,
            last_sync_time: None,
        }
    }
}
#[doc = "Solution configuration resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionConfigurationPropertiesUpdate {
    #[doc = "The type of the solution"]
    #[serde(rename = "solutionType", default, skip_serializing_if = "Option::is_none")]
    pub solution_type: Option<String>,
    #[doc = "Solution settings"]
    #[serde(rename = "solutionSettings", default, skip_serializing_if = "Option::is_none")]
    pub solution_settings: Option<SolutionSettings>,
}
impl SolutionConfigurationPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution Configuration Status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SolutionConfigurationStatus")]
pub enum SolutionConfigurationStatus {
    New,
    InProgress,
    Completed,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SolutionConfigurationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SolutionConfigurationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SolutionConfigurationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::New => serializer.serialize_unit_variant("SolutionConfigurationStatus", 0u32, "New"),
            Self::InProgress => serializer.serialize_unit_variant("SolutionConfigurationStatus", 1u32, "InProgress"),
            Self::Completed => serializer.serialize_unit_variant("SolutionConfigurationStatus", 2u32, "Completed"),
            Self::Failed => serializer.serialize_unit_variant("SolutionConfigurationStatus", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Solution Configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionConfigurationUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Solution configuration resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SolutionConfigurationPropertiesUpdate>,
}
impl SolutionConfigurationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionSettings {}
impl SolutionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SolutionTypeEnum")]
pub enum SolutionTypeEnum {
    #[serde(rename = "Microsoft.AssetManagement")]
    MicrosoftAssetManagement,
    #[serde(rename = "Microsoft.HybridCompute.Onboard")]
    MicrosoftHybridComputeOnboard,
    #[serde(rename = "Microsoft.HybridNetwork.VWan.Provision")]
    MicrosoftHybridNetworkVWanProvision,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SolutionTypeEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SolutionTypeEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SolutionTypeEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MicrosoftAssetManagement => serializer.serialize_unit_variant("SolutionTypeEnum", 0u32, "Microsoft.AssetManagement"),
            Self::MicrosoftHybridComputeOnboard => {
                serializer.serialize_unit_variant("SolutionTypeEnum", 1u32, "Microsoft.HybridCompute.Onboard")
            }
            Self::MicrosoftHybridNetworkVWanProvision => {
                serializer.serialize_unit_variant("SolutionTypeEnum", 2u32, "Microsoft.HybridNetwork.VWan.Provision")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Solution type permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionTypePermissions {
    #[doc = "Solution types."]
    #[serde(rename = "solutionType")]
    pub solution_type: SolutionTypeEnum,
    #[doc = "Connection status."]
    pub status: Status,
    #[doc = "The details of the status"]
    #[serde(rename = "statusDetails")]
    pub status_details: String,
}
impl SolutionTypePermissions {
    pub fn new(solution_type: SolutionTypeEnum, status: Status, status_details: String) -> Self {
        Self {
            solution_type,
            status,
            status_details,
        }
    }
}
#[doc = "Definition of Solution type resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionTypeProperties {
    #[doc = "The name of the solution type."]
    #[serde(rename = "solutionType", default, skip_serializing_if = "Option::is_none")]
    pub solution_type: Option<String>,
    #[doc = "Short description of solution type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The locations this solution is supported in."]
    #[serde(
        rename = "supportedAzureRegions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_azure_regions: Vec<String>,
    #[doc = "Array of solution settings and its description."]
    #[serde(
        rename = "solutionSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub solution_settings: Vec<SolutionTypeSettingsProperties>,
}
impl SolutionTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Concrete proxy resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionTypeResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of Solution type resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SolutionTypeProperties>,
}
impl SolutionTypeResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SolutionTypeResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionTypeResourceListResult {
    #[doc = "The SolutionTypeResource items on this page"]
    pub value: Vec<SolutionTypeResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SolutionTypeResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SolutionTypeResourceListResult {
    pub fn new(value: Vec<SolutionTypeResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of Solution Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionTypeSettings {
    #[doc = "The type of the solution"]
    #[serde(rename = "solutionType")]
    pub solution_type: String,
    #[doc = "Solution settings"]
    #[serde(rename = "solutionSettings", default, skip_serializing_if = "Option::is_none")]
    pub solution_settings: Option<SolutionSettings>,
}
impl SolutionTypeSettings {
    pub fn new(solution_type: String) -> Self {
        Self {
            solution_type,
            solution_settings: None,
        }
    }
}
#[doc = "Represent Solution settings properties description array."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionTypeSettingsProperties {
    #[doc = "The name of the solution setting property."]
    pub name: String,
    #[doc = "The UI friendly name of the solution setting property."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Type of the solution setting property, represented as a string."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Description of solution setting property."]
    pub description: String,
    #[doc = "Array of allowed values for this solution settings property."]
    #[serde(rename = "allowedValues")]
    pub allowed_values: Vec<String>,
    #[doc = "Default value for this solution settings property."]
    #[serde(rename = "defaultValue")]
    pub default_value: String,
}
impl SolutionTypeSettingsProperties {
    pub fn new(
        name: String,
        display_name: String,
        type_: String,
        description: String,
        allowed_values: Vec<String>,
        default_value: String,
    ) -> Self {
        Self {
            name,
            display_name,
            type_,
            description,
            allowed_values,
            default_value,
        }
    }
}
#[doc = "Connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Status")]
pub enum Status {
    Connected,
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
            Self::Connected => serializer.serialize_unit_variant("Status", 0u32, "Connected"),
            Self::Disconnected => serializer.serialize_unit_variant("Status", 1u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Public connector permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestPermissionResult {
    #[doc = "Solution type permissions."]
    #[serde(rename = "solutionTypes")]
    pub solution_types: Vec<SolutionTypePermissions>,
}
impl TestPermissionResult {
    pub fn new(solution_types: Vec<SolutionTypePermissions>) -> Self {
        Self { solution_types }
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
