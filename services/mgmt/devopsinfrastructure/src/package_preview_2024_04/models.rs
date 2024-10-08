#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The agent profile of the machines in the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentProfile {
    #[doc = "Defines pool buffer."]
    #[serde(rename = "resourcePredictions", default, skip_serializing_if = "Option::is_none")]
    pub resource_predictions: Option<ResourcePredictions>,
    #[doc = "Determines how the stand-by scheme should be provided."]
    #[serde(rename = "resourcePredictionsProfile", default, skip_serializing_if = "Option::is_none")]
    pub resource_predictions_profile: Option<ResourcePredictionsProfileUnion>,
}
impl AgentProfile {
    pub fn new() -> Self {
        Self {
            resource_predictions: None,
            resource_predictions_profile: None,
        }
    }
}
#[doc = "Discriminator property for AgentProfile."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum AgentProfileUnion {
    Stateful(Stateful),
    Stateless(StatelessAgentProfile),
}
#[doc = "The stand-by agent scheme is determined based on historical demand."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomaticResourcePredictionsProfile {
    #[doc = "Determines the balance between cost and performance."]
    #[serde(rename = "predictionPreference", default, skip_serializing_if = "Option::is_none")]
    pub prediction_preference: Option<PredictionPreference>,
}
impl AutomaticResourcePredictionsProfile {
    pub fn new() -> Self {
        Self {
            prediction_preference: None,
        }
    }
}
pub type AzureCoreAzureLocation = String;
#[doc = "Azure DevOps organization profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsOrganizationProfile {
    #[doc = "The list of Azure DevOps organizations the pool should be present in."]
    pub organizations: Vec<Organization>,
    #[doc = "Defines the type of Azure DevOps pool permission."]
    #[serde(rename = "permissionProfile", default, skip_serializing_if = "Option::is_none")]
    pub permission_profile: Option<AzureDevOpsPermissionProfile>,
}
impl AzureDevOpsOrganizationProfile {
    pub fn new(organizations: Vec<Organization>) -> Self {
        Self {
            organizations,
            permission_profile: None,
        }
    }
}
#[doc = "Defines the type of Azure DevOps pool permission."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsPermissionProfile {
    #[doc = "Determines who has admin permissions to the Azure DevOps pool."]
    pub kind: AzureDevOpsPermissionType,
    #[doc = "User email addresses"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<String>,
    #[doc = "Group email addresses"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<String>,
}
impl AzureDevOpsPermissionProfile {
    pub fn new(kind: AzureDevOpsPermissionType) -> Self {
        Self {
            kind,
            users: Vec::new(),
            groups: Vec::new(),
        }
    }
}
#[doc = "Determines who has admin permissions to the Azure DevOps pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureDevOpsPermissionType")]
pub enum AzureDevOpsPermissionType {
    Inherit,
    CreatorOnly,
    SpecificAccounts,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureDevOpsPermissionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureDevOpsPermissionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureDevOpsPermissionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Inherit => serializer.serialize_unit_variant("AzureDevOpsPermissionType", 0u32, "Inherit"),
            Self::CreatorOnly => serializer.serialize_unit_variant("AzureDevOpsPermissionType", 1u32, "CreatorOnly"),
            Self::SpecificAccounts => serializer.serialize_unit_variant("AzureDevOpsPermissionType", 2u32, "SpecificAccounts"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of caching in a data disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CachingType")]
pub enum CachingType {
    None,
    ReadOnly,
    ReadWrite,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CachingType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CachingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CachingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("CachingType", 0u32, "None"),
            Self::ReadOnly => serializer.serialize_unit_variant("CachingType", 1u32, "ReadOnly"),
            Self::ReadWrite => serializer.serialize_unit_variant("CachingType", 2u32, "ReadWrite"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The data disk of the VMSS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDisk {
    #[doc = "The type of caching in a data disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<CachingType>,
    #[doc = "The initial disk size in gigabytes."]
    #[serde(rename = "diskSizeGiB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gi_b: Option<i32>,
    #[doc = "StorageAccountType enums"]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<StorageAccountType>,
    #[doc = "The drive letter for the empty data disk. If not specified, it will be the first available letter."]
    #[serde(rename = "driveLetter", default, skip_serializing_if = "Option::is_none")]
    pub drive_letter: Option<String>,
}
impl DataDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure SKU of the machines in the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevOpsAzureSku {
    #[doc = "The Azure SKU name of the machines in the pool."]
    pub name: String,
}
impl DevOpsAzureSku {
    pub fn new(name: String) -> Self {
        Self { name }
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
#[doc = "Discriminator property for FabricProfile."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum FabricProfileUnion {
    Vmss(VmssFabricProfile),
}
#[doc = "Defines a GitHub organization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitHubOrganization {
    #[doc = "The GitHub organization URL in which the pool should be created."]
    pub url: String,
    #[doc = "Optional list of repositories in which the pool should be created."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub repositories: Vec<String>,
}
impl GitHubOrganization {
    pub fn new(url: String) -> Self {
        Self {
            url,
            repositories: Vec::new(),
        }
    }
}
#[doc = "GitHub organization profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitHubOrganizationProfile {
    #[doc = "The list of GitHub organizations/repositories the pool should be present in."]
    pub organizations: Vec<GitHubOrganization>,
}
impl GitHubOrganizationProfile {
    pub fn new(organizations: Vec<GitHubOrganization>) -> Self {
        Self { organizations }
    }
}
#[doc = "An image version object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Details of the ImageVersionProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageVersionProperties>,
}
impl ImageVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ImageVersion list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageVersionListResult {
    #[doc = "The ImageVersion items on this page"]
    pub value: Vec<ImageVersion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImageVersionListResult {
    pub fn new(value: Vec<ImageVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of the ImageVersionProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageVersionProperties {
    #[doc = "Version of the image."]
    pub version: String,
}
impl ImageVersionProperties {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}
#[doc = "Determines how the service should be run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LogonType")]
pub enum LogonType {
    Service,
    Interactive,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LogonType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LogonType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LogonType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Service => serializer.serialize_unit_variant("LogonType", 0u32, "Service"),
            Self::Interactive => serializer.serialize_unit_variant("LogonType", 1u32, "Interactive"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned,UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Customer provides the stand-by agent scheme."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualResourcePredictionsProfile {}
impl ManualResourcePredictionsProfile {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "The network profile of the machines in the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProfile {
    #[doc = "The subnet id on which to put all machines created in the pool."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
}
impl NetworkProfile {
    pub fn new(subnet_id: String) -> Self {
        Self { subnet_id }
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
#[doc = "Defines an Azure DevOps organization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    #[doc = "The Azure DevOps organization URL in which the pool should be created."]
    pub url: String,
    #[doc = "Optional list of projects in which the pool should be created."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub projects: Vec<String>,
    #[doc = "How many machines can be created at maximum in this organization out of the maximumConcurrency of the pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
}
impl Organization {
    pub fn new(url: String) -> Self {
        Self {
            url,
            projects: Vec::new(),
            parallelism: None,
        }
    }
}
#[doc = "Discriminator property for OrganizationProfile."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum OrganizationProfileUnion {
    AzureDevOps(AzureDevOpsOrganizationProfile),
    GitHub(GitHubOrganizationProfile),
}
#[doc = "The storage account type of the OS disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsDiskStorageAccountType")]
pub enum OsDiskStorageAccountType {
    Standard,
    Premium,
    #[serde(rename = "StandardSSD")]
    StandardSsd,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OsDiskStorageAccountType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OsDiskStorageAccountType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OsDiskStorageAccountType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("OsDiskStorageAccountType", 0u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("OsDiskStorageAccountType", 1u32, "Premium"),
            Self::StandardSsd => serializer.serialize_unit_variant("OsDiskStorageAccountType", 2u32, "StandardSSD"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The OS profile of the machines in the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "The secret management settings of the machines in the pool."]
    #[serde(rename = "secretsManagementSettings", default, skip_serializing_if = "Option::is_none")]
    pub secrets_management_settings: Option<SecretsManagementSettings>,
    #[doc = "Determines how the service should be run."]
    #[serde(rename = "logonType", default, skip_serializing_if = "Option::is_none")]
    pub logon_type: Option<LogonType>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged collection of Quota items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedQuota {
    #[doc = "The Quota items on this page"]
    pub value: Vec<Quota>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedQuota {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedQuota {
    pub fn new(value: Vec<Quota>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Concrete tracked resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Pool properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl Pool {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "The VM image of the machines in the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolImage {
    #[doc = "The resource id of the image."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The image to use from a well-known set of images made available to customers."]
    #[serde(rename = "wellKnownImageName", default, skip_serializing_if = "Option::is_none")]
    pub well_known_image_name: Option<String>,
    #[doc = "List of aliases to reference the image by."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aliases: Vec<String>,
    #[doc = "The percentage of the buffer to be allocated to this image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer: Option<String>,
}
impl PoolImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Pool list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolListResult {
    #[doc = "The Pool items on this page"]
    pub value: Vec<Pool>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PoolListResult {
    pub fn new(value: Vec<Pool>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Pool properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Defines how many resources can there be created at any given time."]
    #[serde(rename = "maximumConcurrency")]
    pub maximum_concurrency: i32,
    #[doc = "Defines the organization in which the pool will be used."]
    #[serde(rename = "organizationProfile")]
    pub organization_profile: OrganizationProfileUnion,
    #[doc = "The agent profile of the machines in the pool."]
    #[serde(rename = "agentProfile")]
    pub agent_profile: AgentProfileUnion,
    #[doc = "Defines the type of fabric the agent will run on."]
    #[serde(rename = "fabricProfile")]
    pub fabric_profile: FabricProfileUnion,
    #[doc = "The resource id of the DevCenter Project the pool belongs to."]
    #[serde(rename = "devCenterProjectResourceId")]
    pub dev_center_project_resource_id: String,
}
impl PoolProperties {
    pub fn new(
        maximum_concurrency: i32,
        organization_profile: OrganizationProfileUnion,
        agent_profile: AgentProfileUnion,
        fabric_profile: FabricProfileUnion,
        dev_center_project_resource_id: String,
    ) -> Self {
        Self {
            provisioning_state: None,
            maximum_concurrency,
            organization_profile,
            agent_profile,
            fabric_profile,
            dev_center_project_resource_id,
        }
    }
}
#[doc = "The type used for update operations of the Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolUpdate {
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the Pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolUpdateProperties>,
}
impl PoolUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolUpdateProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Defines how many resources can there be created at any given time."]
    #[serde(rename = "maximumConcurrency", default, skip_serializing_if = "Option::is_none")]
    pub maximum_concurrency: Option<i32>,
    #[doc = "Defines the organization in which the pool will be used."]
    #[serde(rename = "organizationProfile", default, skip_serializing_if = "Option::is_none")]
    pub organization_profile: Option<OrganizationProfileUnion>,
    #[doc = "The agent profile of the machines in the pool."]
    #[serde(rename = "agentProfile", default, skip_serializing_if = "Option::is_none")]
    pub agent_profile: Option<AgentProfileUnion>,
    #[doc = "Defines the type of fabric the agent will run on."]
    #[serde(rename = "fabricProfile", default, skip_serializing_if = "Option::is_none")]
    pub fabric_profile: Option<FabricProfileUnion>,
    #[doc = "The resource id of the DevCenter Project the pool belongs to."]
    #[serde(rename = "devCenterProjectResourceId", default, skip_serializing_if = "Option::is_none")]
    pub dev_center_project_resource_id: Option<String>,
}
impl PoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Determines the balance between cost and performance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PredictionPreference")]
pub enum PredictionPreference {
    Balanced,
    MostCostEffective,
    MoreCostEffective,
    MorePerformance,
    BestPerformance,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PredictionPreference {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PredictionPreference {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PredictionPreference {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Balanced => serializer.serialize_unit_variant("PredictionPreference", 0u32, "Balanced"),
            Self::MostCostEffective => serializer.serialize_unit_variant("PredictionPreference", 1u32, "MostCostEffective"),
            Self::MoreCostEffective => serializer.serialize_unit_variant("PredictionPreference", 2u32, "MoreCostEffective"),
            Self::MorePerformance => serializer.serialize_unit_variant("PredictionPreference", 3u32, "MorePerformance"),
            Self::BestPerformance => serializer.serialize_unit_variant("PredictionPreference", 4u32, "BestPerformance"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the current operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
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
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Describes Resource Quota"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quota {
    #[doc = "The Quota Names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<QuotaName>,
    #[doc = "Fully qualified ARM resource id"]
    pub id: String,
    #[doc = "The unit of usage measurement."]
    pub unit: String,
    #[doc = "The current usage of the resource."]
    #[serde(rename = "currentValue")]
    pub current_value: i64,
    #[doc = "The maximum permitted usage of the resource."]
    pub limit: i64,
}
impl Quota {
    pub fn new(id: String, unit: String, current_value: i64, limit: i64) -> Self {
        Self {
            name: None,
            id,
            unit,
            current_value,
            limit,
        }
    }
}
#[doc = "The Quota Names"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaName {
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl QuotaName {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A ResourceDetailsObject"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceDetailsObject {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Details of the ResourceDetailsObject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceDetailsObjectProperties>,
}
impl ResourceDetailsObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ResourceDetailsObject list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceDetailsObjectListResult {
    #[doc = "The ResourceDetailsObject items on this page"]
    pub value: Vec<ResourceDetailsObject>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceDetailsObjectListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceDetailsObjectListResult {
    pub fn new(value: Vec<ResourceDetailsObject>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of the ResourceDetailsObject."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceDetailsObjectProperties {
    #[doc = "The status of the machine resource."]
    pub status: ResourceStatus,
    #[doc = "The image name of the resource."]
    pub image: String,
    #[doc = "The version of the image running on the resource."]
    #[serde(rename = "imageVersion")]
    pub image_version: String,
}
impl ResourceDetailsObjectProperties {
    pub fn new(status: ResourceStatus, image: String, image_version: String) -> Self {
        Self {
            status,
            image,
            image_version,
        }
    }
}
#[doc = "Defines pool buffer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcePredictions {}
impl ResourcePredictions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Determines how the stand-by scheme should be provided."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ResourcePredictionsProfileUnion {
    Automatic(AutomaticResourcePredictionsProfile),
    Manual(ManualResourcePredictionsProfile),
}
#[doc = "Determines how the stand-by scheme should be provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourcePredictionsProfileType")]
pub enum ResourcePredictionsProfileType {
    Manual,
    Automatic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourcePredictionsProfileType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourcePredictionsProfileType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourcePredictionsProfileType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Manual => serializer.serialize_unit_variant("ResourcePredictionsProfileType", 0u32, "Manual"),
            Self::Automatic => serializer.serialize_unit_variant("ResourcePredictionsProfileType", 1u32, "Automatic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A ResourceSku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a ResourceSku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceSkuProperties>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes The SKU capabilities object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuCapabilities {
    #[doc = "The name of the SKU capability."]
    pub name: String,
    #[doc = "The value of the SKU capability."]
    pub value: String,
}
impl ResourceSkuCapabilities {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "The response of a ResourceSku list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuListResult {
    #[doc = "The ResourceSku items on this page"]
    pub value: Vec<ResourceSku>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceSkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceSkuListResult {
    pub fn new(value: Vec<ResourceSku>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes an available Compute SKU Location Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuLocationInfo {
    #[doc = "Represents an Azure geography region where supported resource providers live."]
    pub location: AzureCoreAzureLocation,
    #[doc = "List of availability zones where the SKU is supported."]
    pub zones: Vec<String>,
    #[doc = "Gets details of capabilities available to a SKU in specific zones."]
    #[serde(rename = "zoneDetails")]
    pub zone_details: Vec<ResourceSkuZoneDetails>,
}
impl ResourceSkuLocationInfo {
    pub fn new(location: AzureCoreAzureLocation, zones: Vec<String>, zone_details: Vec<ResourceSkuZoneDetails>) -> Self {
        Self {
            location,
            zones,
            zone_details,
        }
    }
}
#[doc = "Properties of a ResourceSku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuProperties {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[doc = "The tier of virtual machines in a scale set"]
    pub tier: String,
    #[doc = "The size of the SKU."]
    pub size: String,
    #[doc = "The family of the SKU."]
    pub family: String,
    #[doc = "The set of locations that the SKU is available."]
    pub locations: Vec<AzureCoreAzureLocation>,
    #[doc = "A list of locations and availability zones in those locations where the SKU is available"]
    #[serde(rename = "locationInfo")]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[doc = "Name value pairs to describe the capability."]
    pub capabilities: Vec<ResourceSkuCapabilities>,
    #[doc = "The restrictions of the SKU."]
    pub restrictions: Vec<ResourceSkuRestrictions>,
}
impl ResourceSkuProperties {
    pub fn new(
        resource_type: String,
        tier: String,
        size: String,
        family: String,
        locations: Vec<AzureCoreAzureLocation>,
        location_info: Vec<ResourceSkuLocationInfo>,
        capabilities: Vec<ResourceSkuCapabilities>,
        restrictions: Vec<ResourceSkuRestrictions>,
    ) -> Self {
        Self {
            resource_type,
            tier,
            size,
            family,
            locations,
            location_info,
            capabilities,
            restrictions,
        }
    }
}
#[doc = "Describes an available Compute SKU Restriction Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictionInfo {
    #[doc = "Locations where the SKU is restricted"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<AzureCoreAzureLocation>,
    #[doc = "List of availability zones where the SKU is restricted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
}
impl ResourceSkuRestrictionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restrictions of the SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuRestrictions {
    #[doc = "Describes the kind of SKU restrictions that can exist"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ResourceSkuRestrictionsType>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    pub values: Vec<String>,
    #[doc = "Describes an available Compute SKU Restriction Information."]
    #[serde(rename = "restrictionInfo")]
    pub restriction_info: ResourceSkuRestrictionInfo,
    #[doc = "Describes the reason for SKU restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<ResourceSkuRestrictionsReasonCode>,
}
impl ResourceSkuRestrictions {
    pub fn new(values: Vec<String>, restriction_info: ResourceSkuRestrictionInfo) -> Self {
        Self {
            type_: None,
            values,
            restriction_info,
            reason_code: None,
        }
    }
}
#[doc = "Describes the reason for SKU restriction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceSkuRestrictionsReasonCode")]
pub enum ResourceSkuRestrictionsReasonCode {
    QuotaId,
    NotAvailableForSubscription,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceSkuRestrictionsReasonCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceSkuRestrictionsReasonCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceSkuRestrictionsReasonCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::QuotaId => serializer.serialize_unit_variant("ResourceSkuRestrictionsReasonCode", 0u32, "QuotaId"),
            Self::NotAvailableForSubscription => {
                serializer.serialize_unit_variant("ResourceSkuRestrictionsReasonCode", 1u32, "NotAvailableForSubscription")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the kind of SKU restrictions that can exist"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceSkuRestrictionsType")]
pub enum ResourceSkuRestrictionsType {
    Location,
    Zone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceSkuRestrictionsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceSkuRestrictionsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceSkuRestrictionsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Location => serializer.serialize_unit_variant("ResourceSkuRestrictionsType", 0u32, "Location"),
            Self::Zone => serializer.serialize_unit_variant("ResourceSkuRestrictionsType", 1u32, "Zone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes The zonal capabilities of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuZoneDetails {
    #[doc = "Gets the set of zones that the SKU is available in with the specified capabilities."]
    pub name: Vec<String>,
    #[doc = "A list of capabilities that are available for the SKU in the specified list of zones."]
    pub capabilities: Vec<ResourceSkuCapabilities>,
}
impl ResourceSkuZoneDetails {
    pub fn new(name: Vec<String>, capabilities: Vec<ResourceSkuCapabilities>) -> Self {
        Self { name, capabilities }
    }
}
#[doc = "The status of the machine resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceStatus")]
pub enum ResourceStatus {
    Ready,
    NotReady,
    Allocated,
    PendingReturn,
    Returned,
    Leased,
    Provisioning,
    Updating,
    Starting,
    PendingReimage,
    Reimaging,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ready => serializer.serialize_unit_variant("ResourceStatus", 0u32, "Ready"),
            Self::NotReady => serializer.serialize_unit_variant("ResourceStatus", 1u32, "NotReady"),
            Self::Allocated => serializer.serialize_unit_variant("ResourceStatus", 2u32, "Allocated"),
            Self::PendingReturn => serializer.serialize_unit_variant("ResourceStatus", 3u32, "PendingReturn"),
            Self::Returned => serializer.serialize_unit_variant("ResourceStatus", 4u32, "Returned"),
            Self::Leased => serializer.serialize_unit_variant("ResourceStatus", 5u32, "Leased"),
            Self::Provisioning => serializer.serialize_unit_variant("ResourceStatus", 6u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ResourceStatus", 7u32, "Updating"),
            Self::Starting => serializer.serialize_unit_variant("ResourceStatus", 8u32, "Starting"),
            Self::PendingReimage => serializer.serialize_unit_variant("ResourceStatus", 9u32, "PendingReimage"),
            Self::Reimaging => serializer.serialize_unit_variant("ResourceStatus", 10u32, "Reimaging"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The secret management settings of the machines in the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretsManagementSettings {
    #[doc = "Where to store certificates on the machine."]
    #[serde(rename = "certificateStoreLocation", default, skip_serializing_if = "Option::is_none")]
    pub certificate_store_location: Option<String>,
    #[doc = "The list of certificates to install on all machines in the pool."]
    #[serde(rename = "observedCertificates")]
    pub observed_certificates: Vec<String>,
    #[doc = "Defines if the key of the certificates should be exportable."]
    #[serde(rename = "keyExportable")]
    pub key_exportable: bool,
}
impl SecretsManagementSettings {
    pub fn new(observed_certificates: Vec<String>, key_exportable: bool) -> Self {
        Self {
            certificate_store_location: None,
            observed_certificates,
            key_exportable,
        }
    }
}
#[doc = "Stateful profile meaning that the machines will be returned to the pool after running a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stateful {
    #[serde(flatten)]
    pub agent_profile: AgentProfile,
    #[doc = "How long should stateful machines be kept around. The maximum is one week."]
    #[serde(rename = "maxAgentLifetime", default, skip_serializing_if = "Option::is_none")]
    pub max_agent_lifetime: Option<String>,
    #[doc = "How long should the machine be kept around after it ran a workload when there are no stand-by agents. The maximum is one week."]
    #[serde(rename = "gracePeriodTimeSpan", default, skip_serializing_if = "Option::is_none")]
    pub grace_period_time_span: Option<String>,
}
impl Stateful {
    pub fn new(agent_profile: AgentProfile) -> Self {
        Self {
            agent_profile,
            max_agent_lifetime: None,
            grace_period_time_span: None,
        }
    }
}
#[doc = "Stateless profile meaning that the machines will be cleaned up after running a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessAgentProfile {
    #[serde(flatten)]
    pub agent_profile: AgentProfile,
}
impl StatelessAgentProfile {
    pub fn new(agent_profile: AgentProfile) -> Self {
        Self { agent_profile }
    }
}
#[doc = "StorageAccountType enums"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StorageAccountType")]
pub enum StorageAccountType {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "StandardSSD_LRS")]
    StandardSsdLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(rename = "StandardSSD_ZRS")]
    StandardSsdZrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StorageAccountType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StorageAccountType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StorageAccountType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StandardLrs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "Standard_LRS"),
            Self::PremiumLrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "Premium_LRS"),
            Self::StandardSsdLrs => serializer.serialize_unit_variant("StorageAccountType", 2u32, "StandardSSD_LRS"),
            Self::PremiumZrs => serializer.serialize_unit_variant("StorageAccountType", 3u32, "Premium_ZRS"),
            Self::StandardSsdZrs => serializer.serialize_unit_variant("StorageAccountType", 4u32, "StandardSSD_ZRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The storage profile of the VMSS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "The storage account type of the OS disk."]
    #[serde(rename = "osDiskStorageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_storage_account_type: Option<OsDiskStorageAccountType>,
    #[doc = "A list of empty data disks to attach."]
    #[serde(
        rename = "dataDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_disks: Vec<DataDisk>,
}
impl StorageProfile {
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
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The agents will run on Virtual Machine Scale Sets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmssFabricProfile {
    #[doc = "The Azure SKU of the machines in the pool."]
    pub sku: DevOpsAzureSku,
    #[doc = "The VM images of the machines in the pool."]
    pub images: Vec<PoolImage>,
    #[doc = "The OS profile of the machines in the pool."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "The storage profile of the VMSS."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "The network profile of the machines in the pool."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}
impl VmssFabricProfile {
    pub fn new(sku: DevOpsAzureSku, images: Vec<PoolImage>) -> Self {
        Self {
            sku,
            images,
            os_profile: None,
            storage_profile: None,
            network_profile: None,
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
