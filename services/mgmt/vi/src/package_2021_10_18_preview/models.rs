#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure Video Analyzer for Media access token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessToken {
    #[doc = "The access token."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
impl AccessToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Video Analyzer for Media account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Azure Video Analyzer for Media account properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountPropertiesForPutRequest>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Account {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
            system_data: None,
        }
    }
}
#[doc = "The parameters used to check the availability of the Video Indexer account name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountCheckNameAvailabilityParameters {
    #[doc = "The VideoIndexer account name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.VideoIndexer/accounts"]
    #[serde(rename = "type")]
    pub type_: account_check_name_availability_parameters::Type,
}
impl AccountCheckNameAvailabilityParameters {
    pub fn new(name: String, type_: account_check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod account_check_name_availability_parameters {
    use super::*;
    #[doc = "The type of resource, Microsoft.VideoIndexer/accounts"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "Microsoft.VideoIndexer/accounts")]
        MicrosoftVideoIndexerAccounts,
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
                Self::MicrosoftVideoIndexerAccounts => serializer.serialize_unit_variant("Type", 0u32, "Microsoft.VideoIndexer/accounts"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list operation response, that contains the data pools and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountList {
    #[doc = "List of accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccountList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Video Analyzer for Media account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPatch {
    #[serde(flatten)]
    pub tags: Tags,
    #[doc = "Azure Video Analyzer for Media account properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountPropertiesForPatchRequest>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl AccountPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Video Analyzer for Media account properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPropertiesForPatchRequest {
    #[doc = "The account's tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The account's data-plane ID"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The media services details"]
    #[serde(rename = "mediaServices", default, skip_serializing_if = "Option::is_none")]
    pub media_services: Option<MediaServicesForPatchRequest>,
    #[doc = "Gets the status of the account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<account_properties_for_patch_request::ProvisioningState>,
}
impl AccountPropertiesForPatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_properties_for_patch_request {
    use super::*;
    #[doc = "Gets the status of the account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
        Provisioning,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Provisioning"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure Video Analyzer for Media account properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPropertiesForPutRequest {
    #[doc = "The account's tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The account's data-plane ID. This can be set only when connecting an existing classic account"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The account's name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The media services details"]
    #[serde(rename = "mediaServices", default, skip_serializing_if = "Option::is_none")]
    pub media_services: Option<MediaServicesForPutRequest>,
    #[doc = "Gets the status of the account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<account_properties_for_put_request::ProvisioningState>,
}
impl AccountPropertiesForPutRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_properties_for_put_request {
    use super::*;
    #[doc = "Gets the status of the account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
        Provisioning,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Provisioning"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Gets a boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets the reason that a Video Indexer account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "Gets an error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "Gets the reason that a Video Indexer account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 0u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "Access token generation request's parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateAccessTokenParameters {
    #[doc = "The requested permission"]
    #[serde(rename = "permissionType")]
    pub permission_type: generate_access_token_parameters::PermissionType,
    #[doc = "The requested media type"]
    pub scope: generate_access_token_parameters::Scope,
    #[doc = "The video ID"]
    #[serde(rename = "videoId", default, skip_serializing_if = "Option::is_none")]
    pub video_id: Option<String>,
    #[doc = "The project ID"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl GenerateAccessTokenParameters {
    pub fn new(permission_type: generate_access_token_parameters::PermissionType, scope: generate_access_token_parameters::Scope) -> Self {
        Self {
            permission_type,
            scope,
            video_id: None,
            project_id: None,
        }
    }
}
pub mod generate_access_token_parameters {
    use super::*;
    #[doc = "The requested permission"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PermissionType")]
    pub enum PermissionType {
        Contributor,
        Reader,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PermissionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PermissionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PermissionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Contributor => serializer.serialize_unit_variant("PermissionType", 0u32, "Contributor"),
                Self::Reader => serializer.serialize_unit_variant("PermissionType", 1u32, "Reader"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The requested media type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scope")]
    pub enum Scope {
        Video,
        Account,
        Project,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Video => serializer.serialize_unit_variant("Scope", 0u32, "Video"),
                Self::Account => serializer.serialize_unit_variant("Scope", 1u32, "Account"),
                Self::Project => serializer.serialize_unit_variant("Scope", 2u32, "Project"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "The media services details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaServicesForPatchRequest {
    #[doc = "The user assigned identity to be used to grant permissions"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl MediaServicesForPatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The media services details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaServicesForPutRequest {
    #[doc = "The media services resource id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The user assigned identity to be used to grant permissions"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl MediaServicesForPutRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation detail payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Indicates the action type."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[doc = "Operation display payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation display payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Resource provider of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Localized friendly name for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Localized friendly description for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operations of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Resource Provider."]
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
#[doc = "Resource tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "Resource tags"]
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
