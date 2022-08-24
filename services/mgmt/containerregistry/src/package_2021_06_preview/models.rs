#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The activation properties of the connected registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivationProperties {
    #[doc = "The activation status of the connected registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<activation_properties::Status>,
}
impl ActivationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod activation_properties {
    use super::*;
    #[doc = "The activation status of the connected registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Inactive,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Inactive => serializer.serialize_unit_variant("Status", 1u32, "Inactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Active Directory Object that will be used for authenticating the token of a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryObject {
    #[doc = "The user/group/application object ID for Active Directory Object that will be used for authenticating the token of a container registry."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The tenant ID of user/group/application object Active Directory Object that will be used for authenticating the token of a container registry."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ActiveDirectoryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The agent that initiated the event. For most situations, this could be from the authorization context of the request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Actor {
    #[doc = "The subject or username associated with the request context that generated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Actor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The agentpool that has the ARM resource and properties. \r\nThe agentpool will have all information to create an agent pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentPool {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of agent pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgentPoolProperties>,
}
impl AgentPool {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The collection of agent pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AgentPool>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AgentPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AgentPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of agent pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolProperties {
    #[doc = "The count of agent machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "The Tier of agent machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The OS of agent machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<agent_pool_properties::Os>,
    #[doc = "The Virtual Network Subnet Resource Id of the agent machine"]
    #[serde(rename = "virtualNetworkSubnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_subnet_resource_id: Option<String>,
    #[doc = "The provisioning state of this agent pool"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<agent_pool_properties::ProvisioningState>,
}
impl AgentPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_pool_properties {
    use super::*;
    #[doc = "The OS of agent machine"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Os")]
    pub enum Os {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Os {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Os {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Os {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("Os", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("Os", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of this agent pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolPropertiesUpdateParameters {
    #[doc = "The count of agent machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl AgentPoolPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The QueueStatus of Agent Pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolQueueStatus {
    #[doc = "The number of pending runs in the queue"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl AgentPoolQueueStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for updating an agent pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgentPoolPropertiesUpdateParameters>,
    #[doc = "The ARM resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AgentPoolUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that determine the run agent configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentProperties {
    #[doc = "The CPU configuration in terms of number of cores required for the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
}
impl AgentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a run argument."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Argument {
    #[doc = "The name of the argument."]
    pub name: String,
    #[doc = "The value of the argument."]
    pub value: String,
    #[doc = "Flag to indicate whether the argument represents a secret and want to be removed from build logs."]
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
}
impl Argument {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            is_secret: None,
        }
    }
}
#[doc = "The authorization properties for accessing the source code repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthInfo {
    #[doc = "The type of Auth token."]
    #[serde(rename = "tokenType")]
    pub token_type: auth_info::TokenType,
    #[doc = "The access token used to access the source control provider."]
    pub token: String,
    #[doc = "The refresh token used to refresh the access token."]
    #[serde(rename = "refreshToken", default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc = "The scope of the access token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Time in seconds that the token remains valid"]
    #[serde(rename = "expiresIn", default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
}
impl AuthInfo {
    pub fn new(token_type: auth_info::TokenType, token: String) -> Self {
        Self {
            token_type,
            token,
            refresh_token: None,
            scope: None,
            expires_in: None,
        }
    }
}
pub mod auth_info {
    use super::*;
    #[doc = "The type of Auth token."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TokenType")]
    pub enum TokenType {
        #[serde(rename = "PAT")]
        Pat,
        OAuth,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TokenType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TokenType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TokenType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pat => serializer.serialize_unit_variant("TokenType", 0u32, "PAT"),
                Self::OAuth => serializer.serialize_unit_variant("TokenType", 1u32, "OAuth"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The authorization properties for accessing the source code repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthInfoUpdateParameters {
    #[doc = "The type of Auth token."]
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<auth_info_update_parameters::TokenType>,
    #[doc = "The access token used to access the source control provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "The refresh token used to refresh the access token."]
    #[serde(rename = "refreshToken", default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc = "The scope of the access token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Time in seconds that the token remains valid"]
    #[serde(rename = "expiresIn", default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
}
impl AuthInfoUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auth_info_update_parameters {
    use super::*;
    #[doc = "The type of Auth token."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TokenType")]
    pub enum TokenType {
        #[serde(rename = "PAT")]
        Pat,
        OAuth,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TokenType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TokenType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TokenType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pat => serializer.serialize_unit_variant("TokenType", 0u32, "PAT"),
                Self::OAuth => serializer.serialize_unit_variant("TokenType", 1u32, "OAuth"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties that describe a base image dependency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseImageDependency {
    #[doc = "The type of the base image dependency."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<base_image_dependency::Type>,
    #[doc = "The registry login server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "The repository name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The tag name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The sha256-based digest of the image manifest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}
impl BaseImageDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod base_image_dependency {
    use super::*;
    #[doc = "The type of the base image dependency."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        BuildTime,
        RunTime,
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
                Self::BuildTime => serializer.serialize_unit_variant("Type", 0u32, "BuildTime"),
                Self::RunTime => serializer.serialize_unit_variant("Type", 1u32, "RunTime"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The trigger based on base image dependency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseImageTrigger {
    #[doc = "The type of the auto trigger for base image dependency updates."]
    #[serde(rename = "baseImageTriggerType")]
    pub base_image_trigger_type: base_image_trigger::BaseImageTriggerType,
    #[doc = "The endpoint URL for receiving update triggers."]
    #[serde(rename = "updateTriggerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub update_trigger_endpoint: Option<String>,
    #[doc = "Type of Payload body for Base image update triggers."]
    #[serde(rename = "updateTriggerPayloadType", default, skip_serializing_if = "Option::is_none")]
    pub update_trigger_payload_type: Option<base_image_trigger::UpdateTriggerPayloadType>,
    #[doc = "The current status of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<base_image_trigger::Status>,
    #[doc = "The name of the trigger."]
    pub name: String,
}
impl BaseImageTrigger {
    pub fn new(base_image_trigger_type: base_image_trigger::BaseImageTriggerType, name: String) -> Self {
        Self {
            base_image_trigger_type,
            update_trigger_endpoint: None,
            update_trigger_payload_type: None,
            status: None,
            name,
        }
    }
}
pub mod base_image_trigger {
    use super::*;
    #[doc = "The type of the auto trigger for base image dependency updates."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BaseImageTriggerType")]
    pub enum BaseImageTriggerType {
        All,
        Runtime,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BaseImageTriggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BaseImageTriggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BaseImageTriggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("BaseImageTriggerType", 0u32, "All"),
                Self::Runtime => serializer.serialize_unit_variant("BaseImageTriggerType", 1u32, "Runtime"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of Payload body for Base image update triggers."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpdateTriggerPayloadType")]
    pub enum UpdateTriggerPayloadType {
        Default,
        Token,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpdateTriggerPayloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpdateTriggerPayloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpdateTriggerPayloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("UpdateTriggerPayloadType", 0u32, "Default"),
                Self::Token => serializer.serialize_unit_variant("UpdateTriggerPayloadType", 1u32, "Token"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "The properties for updating base image dependency trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseImageTriggerUpdateParameters {
    #[doc = "The type of the auto trigger for base image dependency updates."]
    #[serde(rename = "baseImageTriggerType", default, skip_serializing_if = "Option::is_none")]
    pub base_image_trigger_type: Option<base_image_trigger_update_parameters::BaseImageTriggerType>,
    #[doc = "The endpoint URL for receiving update triggers."]
    #[serde(rename = "updateTriggerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub update_trigger_endpoint: Option<String>,
    #[doc = "Type of Payload body for Base image update triggers."]
    #[serde(rename = "updateTriggerPayloadType", default, skip_serializing_if = "Option::is_none")]
    pub update_trigger_payload_type: Option<base_image_trigger_update_parameters::UpdateTriggerPayloadType>,
    #[doc = "The current status of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<base_image_trigger_update_parameters::Status>,
    #[doc = "The name of the trigger."]
    pub name: String,
}
impl BaseImageTriggerUpdateParameters {
    pub fn new(name: String) -> Self {
        Self {
            base_image_trigger_type: None,
            update_trigger_endpoint: None,
            update_trigger_payload_type: None,
            status: None,
            name,
        }
    }
}
pub mod base_image_trigger_update_parameters {
    use super::*;
    #[doc = "The type of the auto trigger for base image dependency updates."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BaseImageTriggerType")]
    pub enum BaseImageTriggerType {
        All,
        Runtime,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BaseImageTriggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BaseImageTriggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BaseImageTriggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("BaseImageTriggerType", 0u32, "All"),
                Self::Runtime => serializer.serialize_unit_variant("BaseImageTriggerType", 1u32, "Runtime"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of Payload body for Base image update triggers."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpdateTriggerPayloadType")]
    pub enum UpdateTriggerPayloadType {
        Default,
        Token,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpdateTriggerPayloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpdateTriggerPayloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpdateTriggerPayloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("UpdateTriggerPayloadType", 0u32, "Default"),
                Self::Token => serializer.serialize_unit_variant("UpdateTriggerPayloadType", 1u32, "Token"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "The configuration of service URI and custom headers for the webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallbackConfig {
    #[doc = "The service URI for the webhook to post notifications."]
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
    #[doc = "Custom headers that will be added to the webhook notifications."]
    #[serde(rename = "customHeaders", default, skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<serde_json::Value>,
}
impl CallbackConfig {
    pub fn new(service_uri: String) -> Self {
        Self {
            service_uri,
            custom_headers: None,
        }
    }
}
#[doc = "An object that represents a connected registry for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedRegistry {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a connected registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectedRegistryProperties>,
}
impl ConnectedRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list connected registries for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedRegistryListResult {
    #[doc = "The list of connected registries. Since this list may be incomplete, the nextLink field should be used to request the next list of connected registries."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectedRegistry>,
    #[doc = "The URI that can be used to request the next list of connected registries."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectedRegistryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConnectedRegistryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a connected registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectedRegistryProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<connected_registry_properties::ProvisioningState>,
    #[doc = "The mode of the connected registry resource that indicates the permissions of the registry."]
    pub mode: connected_registry_properties::Mode,
    #[doc = "The current version of ACR runtime on the connected registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The current connection state of the connected registry."]
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<connected_registry_properties::ConnectionState>,
    #[doc = "The last activity time of the connected registry."]
    #[serde(rename = "lastActivityTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_activity_time: Option<time::OffsetDateTime>,
    #[doc = "The activation properties of the connected registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation: Option<ActivationProperties>,
    #[doc = "The properties of the connected registry parent."]
    pub parent: ParentProperties,
    #[doc = "The list of the ACR token resource IDs used to authenticate clients to the connected registry."]
    #[serde(rename = "clientTokenIds", default, skip_serializing_if = "Vec::is_empty")]
    pub client_token_ids: Vec<String>,
    #[doc = "The login server properties of the connected registry."]
    #[serde(rename = "loginServer", default, skip_serializing_if = "Option::is_none")]
    pub login_server: Option<LoginServerProperties>,
    #[doc = "The logging properties of the connected registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingProperties>,
    #[doc = "The list of current statuses of the connected registry."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub status_details: Vec<StatusDetailProperties>,
}
impl ConnectedRegistryProperties {
    pub fn new(mode: connected_registry_properties::Mode, parent: ParentProperties) -> Self {
        Self {
            provisioning_state: None,
            mode,
            version: None,
            connection_state: None,
            last_activity_time: None,
            activation: None,
            parent,
            client_token_ids: Vec::new(),
            login_server: None,
            logging: None,
            status_details: Vec::new(),
        }
    }
}
pub mod connected_registry_properties {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The mode of the connected registry resource that indicates the permissions of the registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Registry,
        Mirror,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Registry => serializer.serialize_unit_variant("Mode", 0u32, "Registry"),
                Self::Mirror => serializer.serialize_unit_variant("Mode", 1u32, "Mirror"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current connection state of the connected registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionState")]
    pub enum ConnectionState {
        Online,
        Offline,
        Syncing,
        Unhealthy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Online => serializer.serialize_unit_variant("ConnectionState", 0u32, "Online"),
                Self::Offline => serializer.serialize_unit_variant("ConnectionState", 1u32, "Offline"),
                Self::Syncing => serializer.serialize_unit_variant("ConnectionState", 2u32, "Syncing"),
                Self::Unhealthy => serializer.serialize_unit_variant("ConnectionState", 3u32, "Unhealthy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters for updating a connected registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedRegistryUpdateParameters {
    #[doc = "The parameters for updating token properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectedRegistryUpdateProperties>,
}
impl ConnectedRegistryUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for updating token properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedRegistryUpdateProperties {
    #[doc = "The parameters for updating the sync properties of the connected registry with its parent."]
    #[serde(rename = "syncProperties", default, skip_serializing_if = "Option::is_none")]
    pub sync_properties: Option<SyncUpdateProperties>,
    #[doc = "The logging properties of the connected registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingProperties>,
    #[doc = "The list of the ACR token resource IDs used to authenticate clients to the connected registry."]
    #[serde(rename = "clientTokenIds", default, skip_serializing_if = "Vec::is_empty")]
    pub client_token_ids: Vec<String>,
}
impl ConnectedRegistryUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters that describes a set of credentials that will be used when a run is invoked."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Credentials {
    #[doc = "Describes the credential parameters for accessing the source registry."]
    #[serde(rename = "sourceRegistry", default, skip_serializing_if = "Option::is_none")]
    pub source_registry: Option<SourceRegistryCredentials>,
    #[doc = "Describes the credential parameters for accessing other custom registries. The key\r\nfor the dictionary item will be the registry login server (myregistry.azurecr.io) and\r\nthe value of the item will be the registry credentials for accessing the registry."]
    #[serde(rename = "customRegistries", default, skip_serializing_if = "Option::is_none")]
    pub custom_registries: Option<serde_json::Value>,
}
impl Credentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the credentials that will be used to access a custom registry during a run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRegistryCredentials {
    #[doc = "Describes the properties of a secret object value."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<SecretObject>,
    #[doc = "Describes the properties of a secret object value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<SecretObject>,
    #[doc = "Indicates the managed identity assigned to the custom credential. If a user-assigned identity\r\nthis value is the Client ID. If a system-assigned identity, the value will be `system`. In\r\nthe case of a system-assigned identity, the Client ID will be determined by the runner. This\r\nidentity may be used to authenticate to key vault to retrieve credentials or it may be the only \r\nsource of authentication used for accessing the registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
}
impl CustomRegistryCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for a docker quick build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerBuildRequest {
    #[serde(flatten)]
    pub run_request: RunRequest,
    #[doc = "The fully qualified image names including the repository and tag."]
    #[serde(rename = "imageNames", default, skip_serializing_if = "Vec::is_empty")]
    pub image_names: Vec<String>,
    #[doc = "The value of this property indicates whether the image built should be pushed to the registry or not."]
    #[serde(rename = "isPushEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_push_enabled: Option<bool>,
    #[doc = "The value of this property indicates whether the image cache is enabled or not."]
    #[serde(rename = "noCache", default, skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,
    #[doc = "The Docker file path relative to the source location."]
    #[serde(rename = "dockerFilePath")]
    pub docker_file_path: String,
    #[doc = "The name of the target build stage for the docker build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The collection of override arguments to be used when executing the run."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<Argument>,
    #[doc = "Run timeout in seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[doc = "The platform properties against which the run has to happen."]
    pub platform: PlatformProperties,
    #[doc = "The properties that determine the run agent configuration."]
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[doc = "The URL(absolute or relative) of the source context. It can be an URL to a tar or git repository.\r\nIf it is relative URL, the relative path should be obtained from calling listBuildSourceUploadUrl API."]
    #[serde(rename = "sourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[doc = "The parameters that describes a set of credentials that will be used when a run is invoked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}
impl DockerBuildRequest {
    pub fn new(run_request: RunRequest, docker_file_path: String, platform: PlatformProperties) -> Self {
        Self {
            run_request,
            image_names: Vec::new(),
            is_push_enabled: None,
            no_cache: None,
            docker_file_path,
            target: None,
            arguments: Vec::new(),
            timeout: None,
            platform,
            agent_configuration: None,
            source_location: None,
            credentials: None,
        }
    }
}
#[doc = "The Docker build step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerBuildStep {
    #[serde(flatten)]
    pub task_step_properties: TaskStepProperties,
    #[doc = "The fully qualified image names including the repository and tag."]
    #[serde(rename = "imageNames", default, skip_serializing_if = "Vec::is_empty")]
    pub image_names: Vec<String>,
    #[doc = "The value of this property indicates whether the image built should be pushed to the registry or not."]
    #[serde(rename = "isPushEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_push_enabled: Option<bool>,
    #[doc = "The value of this property indicates whether the image cache is enabled or not."]
    #[serde(rename = "noCache", default, skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,
    #[doc = "The Docker file path relative to the source context."]
    #[serde(rename = "dockerFilePath")]
    pub docker_file_path: String,
    #[doc = "The name of the target build stage for the docker build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The collection of override arguments to be used when executing this build step."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<Argument>,
}
impl DockerBuildStep {
    pub fn new(task_step_properties: TaskStepProperties, docker_file_path: String) -> Self {
        Self {
            task_step_properties,
            image_names: Vec::new(),
            is_push_enabled: None,
            no_cache: None,
            docker_file_path,
            target: None,
            arguments: Vec::new(),
        }
    }
}
#[doc = "The properties for updating a docker build step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerBuildStepUpdateParameters {
    #[serde(flatten)]
    pub task_step_update_parameters: TaskStepUpdateParameters,
    #[doc = "The fully qualified image names including the repository and tag."]
    #[serde(rename = "imageNames", default, skip_serializing_if = "Vec::is_empty")]
    pub image_names: Vec<String>,
    #[doc = "The value of this property indicates whether the image built should be pushed to the registry or not."]
    #[serde(rename = "isPushEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_push_enabled: Option<bool>,
    #[doc = "The value of this property indicates whether the image cache is enabled or not."]
    #[serde(rename = "noCache", default, skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,
    #[doc = "The Docker file path relative to the source context."]
    #[serde(rename = "dockerFilePath", default, skip_serializing_if = "Option::is_none")]
    pub docker_file_path: Option<String>,
    #[doc = "The collection of override arguments to be used when executing this build step."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<Argument>,
    #[doc = "The name of the target build stage for the docker build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl DockerBuildStepUpdateParameters {
    pub fn new(task_step_update_parameters: TaskStepUpdateParameters) -> Self {
        Self {
            task_step_update_parameters,
            image_names: Vec::new(),
            is_push_enabled: None,
            no_cache: None,
            docker_file_path: None,
            arguments: Vec::new(),
            target: None,
        }
    }
}
#[doc = "The parameters for a quick task run request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodedTaskRunRequest {
    #[serde(flatten)]
    pub run_request: RunRequest,
    #[doc = "Base64 encoded value of the template/definition file content."]
    #[serde(rename = "encodedTaskContent")]
    pub encoded_task_content: String,
    #[doc = "Base64 encoded value of the parameters/values file content."]
    #[serde(rename = "encodedValuesContent", default, skip_serializing_if = "Option::is_none")]
    pub encoded_values_content: Option<String>,
    #[doc = "The collection of overridable values that can be passed when running a task."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
    #[doc = "Run timeout in seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[doc = "The platform properties against which the run has to happen."]
    pub platform: PlatformProperties,
    #[doc = "The properties that determine the run agent configuration."]
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[doc = "The URL(absolute or relative) of the source context. It can be an URL to a tar or git repository.\r\nIf it is relative URL, the relative path should be obtained from calling listBuildSourceUploadUrl API."]
    #[serde(rename = "sourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[doc = "The parameters that describes a set of credentials that will be used when a run is invoked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}
impl EncodedTaskRunRequest {
    pub fn new(run_request: RunRequest, encoded_task_content: String, platform: PlatformProperties) -> Self {
        Self {
            run_request,
            encoded_task_content,
            encoded_values_content: None,
            values: Vec::new(),
            timeout: None,
            platform,
            agent_configuration: None,
            source_location: None,
            credentials: None,
        }
    }
}
#[doc = "The properties of a encoded task step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodedTaskStep {
    #[serde(flatten)]
    pub task_step_properties: TaskStepProperties,
    #[doc = "Base64 encoded value of the template/definition file content."]
    #[serde(rename = "encodedTaskContent")]
    pub encoded_task_content: String,
    #[doc = "Base64 encoded value of the parameters/values file content."]
    #[serde(rename = "encodedValuesContent", default, skip_serializing_if = "Option::is_none")]
    pub encoded_values_content: Option<String>,
    #[doc = "The collection of overridable values that can be passed when running a task."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
impl EncodedTaskStep {
    pub fn new(task_step_properties: TaskStepProperties, encoded_task_content: String) -> Self {
        Self {
            task_step_properties,
            encoded_task_content,
            encoded_values_content: None,
            values: Vec::new(),
        }
    }
}
#[doc = "The properties for updating encoded task step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodedTaskStepUpdateParameters {
    #[serde(flatten)]
    pub task_step_update_parameters: TaskStepUpdateParameters,
    #[doc = "Base64 encoded value of the template/definition file content."]
    #[serde(rename = "encodedTaskContent", default, skip_serializing_if = "Option::is_none")]
    pub encoded_task_content: Option<String>,
    #[doc = "Base64 encoded value of the parameters/values file content."]
    #[serde(rename = "encodedValuesContent", default, skip_serializing_if = "Option::is_none")]
    pub encoded_values_content: Option<String>,
    #[doc = "The collection of overridable values that can be passed when running a task."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
impl EncodedTaskStepUpdateParameters {
    pub fn new(task_step_update_parameters: TaskStepUpdateParameters) -> Self {
        Self {
            task_step_update_parameters,
            encoded_task_content: None,
            encoded_values_content: None,
            values: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionProperty {
    #[doc = "Indicates whether or not the encryption is enabled for container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<encryption_property::Status>,
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
impl EncryptionProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_property {
    use super::*;
    #[doc = "Indicates whether or not the encryption is enabled for container registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An error response from the Azure Container Registry service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "An error response from the Azure Container Registry service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
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
#[doc = "An error response from the Azure Container Registry service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    #[doc = "error code."]
    pub code: String,
    #[doc = "error message."]
    pub message: String,
    #[doc = "target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "an array of additional nested error response info objects, as described by this contract."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<InnerErrorDescription>,
}
impl ErrorResponseBody {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "The event for a webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[serde(flatten)]
    pub event_info: EventInfo,
    #[doc = "The event request message sent to the service URI."]
    #[serde(rename = "eventRequestMessage", default, skip_serializing_if = "Option::is_none")]
    pub event_request_message: Option<EventRequestMessage>,
    #[doc = "The event response message received from the service URI."]
    #[serde(rename = "eventResponseMessage", default, skip_serializing_if = "Option::is_none")]
    pub event_response_message: Option<EventResponseMessage>,
}
impl Event {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The content of the event request message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventContent {
    #[doc = "The event ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The action that encompasses the provided event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The target of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
    #[doc = "The request that generated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    #[doc = "The agent that initiated the event. For most situations, this could be from the authorization context of the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<Actor>,
    #[doc = "The registry node that generated the event. Put differently, while the actor initiates the event, the source generates it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}
impl EventContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The basic information of an event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventInfo {
    #[doc = "The event ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EventInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list events for a webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventListResult {
    #[doc = "The list of events. Since this list may be incomplete, the nextLink field should be used to request the next list of events."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Event>,
    #[doc = "The URI that can be used to request the next list of events."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EventListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The event request message sent to the service URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventRequestMessage {
    #[doc = "The content of the event request message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<EventContent>,
    #[doc = "The headers of the event request message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[doc = "The HTTP method used to send the event request message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "The URI used to send the event request message."]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[doc = "The HTTP message version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl EventRequestMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The event response message received from the service URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventResponseMessage {
    #[doc = "The content of the event response message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The headers of the event response message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[doc = "The reason phrase of the event response message."]
    #[serde(rename = "reasonPhrase", default, skip_serializing_if = "Option::is_none")]
    pub reason_phrase: Option<String>,
    #[doc = "The status code of the event response message."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[doc = "The HTTP message version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl EventResponseMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents an export pipeline for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportPipeline {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The location of the export pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Managed identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The properties of an export pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportPipelineProperties>,
}
impl ExportPipeline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list export pipelines for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportPipelineListResult {
    #[doc = "The list of export pipelines. Since this list may be incomplete, the nextLink field should be used to request the next list of export pipelines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExportPipeline>,
    #[doc = "The URI that can be used to request the next list of pipeline runs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExportPipelineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExportPipelineListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an export pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportPipelineProperties {
    #[doc = "The properties of the export pipeline target."]
    pub target: ExportPipelineTargetProperties,
    #[doc = "The list of all options configured for the pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<String>,
    #[doc = "The provisioning state of the pipeline at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<export_pipeline_properties::ProvisioningState>,
}
impl ExportPipelineProperties {
    pub fn new(target: ExportPipelineTargetProperties) -> Self {
        Self {
            target,
            options: Vec::new(),
            provisioning_state: None,
        }
    }
}
pub mod export_pipeline_properties {
    use super::*;
    #[doc = "The provisioning state of the pipeline at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the export pipeline target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportPipelineTargetProperties {
    #[doc = "The type of target for the export pipeline."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The target uri of the export pipeline.\r\nWhen 'AzureStorageBlob': \"https://accountName.blob.core.windows.net/containerName/blobName\"\r\nWhen 'AzureStorageBlobContainer':  \"https://accountName.blob.core.windows.net/containerName\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "They key vault secret uri to obtain the target storage SAS token."]
    #[serde(rename = "keyVaultUri")]
    pub key_vault_uri: String,
}
impl ExportPipelineTargetProperties {
    pub fn new(key_vault_uri: String) -> Self {
        Self {
            type_: None,
            uri: None,
            key_vault_uri,
        }
    }
}
#[doc = "The export policy for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportPolicy {
    #[doc = "The value that indicates whether the policy is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<export_policy::Status>,
}
impl ExportPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export_policy {
    use super::*;
    #[doc = "The value that indicates whether the policy is enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "The request parameters for a scheduling run against a task file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileTaskRunRequest {
    #[serde(flatten)]
    pub run_request: RunRequest,
    #[doc = "The template/definition file path relative to the source."]
    #[serde(rename = "taskFilePath")]
    pub task_file_path: String,
    #[doc = "The values/parameters file path relative to the source."]
    #[serde(rename = "valuesFilePath", default, skip_serializing_if = "Option::is_none")]
    pub values_file_path: Option<String>,
    #[doc = "The collection of overridable values that can be passed when running a task."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
    #[doc = "Run timeout in seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[doc = "The platform properties against which the run has to happen."]
    pub platform: PlatformProperties,
    #[doc = "The properties that determine the run agent configuration."]
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[doc = "The URL(absolute or relative) of the source context. It can be an URL to a tar or git repository.\r\nIf it is relative URL, the relative path should be obtained from calling listBuildSourceUploadUrl API."]
    #[serde(rename = "sourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[doc = "The parameters that describes a set of credentials that will be used when a run is invoked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}
impl FileTaskRunRequest {
    pub fn new(run_request: RunRequest, task_file_path: String, platform: PlatformProperties) -> Self {
        Self {
            run_request,
            task_file_path,
            values_file_path: None,
            values: Vec::new(),
            timeout: None,
            platform,
            agent_configuration: None,
            source_location: None,
            credentials: None,
        }
    }
}
#[doc = "The properties of a task step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileTaskStep {
    #[serde(flatten)]
    pub task_step_properties: TaskStepProperties,
    #[doc = "The task template/definition file path relative to the source context."]
    #[serde(rename = "taskFilePath")]
    pub task_file_path: String,
    #[doc = "The task values/parameters file path relative to the source context."]
    #[serde(rename = "valuesFilePath", default, skip_serializing_if = "Option::is_none")]
    pub values_file_path: Option<String>,
    #[doc = "The collection of overridable values that can be passed when running a task."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
impl FileTaskStep {
    pub fn new(task_step_properties: TaskStepProperties, task_file_path: String) -> Self {
        Self {
            task_step_properties,
            task_file_path,
            values_file_path: None,
            values: Vec::new(),
        }
    }
}
#[doc = "The properties of updating a task step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileTaskStepUpdateParameters {
    #[serde(flatten)]
    pub task_step_update_parameters: TaskStepUpdateParameters,
    #[doc = "The task template/definition file path relative to the source context."]
    #[serde(rename = "taskFilePath", default, skip_serializing_if = "Option::is_none")]
    pub task_file_path: Option<String>,
    #[doc = "The values/parameters file path relative to the source context."]
    #[serde(rename = "valuesFilePath", default, skip_serializing_if = "Option::is_none")]
    pub values_file_path: Option<String>,
    #[doc = "The collection of overridable values that can be passed when running a task."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
impl FileTaskStepUpdateParameters {
    pub fn new(task_step_update_parameters: TaskStepUpdateParameters) -> Self {
        Self {
            task_step_update_parameters,
            task_file_path: None,
            values_file_path: None,
            values: Vec::new(),
        }
    }
}
#[doc = "The parameters used to generate credentials for a specified token or user of a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateCredentialsParameters {
    #[doc = "The resource ID of the token for which credentials have to be generated."]
    #[serde(rename = "tokenId", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[doc = "The expiry date of the generated credentials after which the credentials become invalid."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
    #[doc = "Specifies name of the password which should be regenerated if any -- password1 or password2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<generate_credentials_parameters::Name>,
}
impl GenerateCredentialsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod generate_credentials_parameters {
    use super::*;
    #[doc = "Specifies name of the password which should be regenerated if any -- password1 or password2."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "password1")]
        Password1,
        #[serde(rename = "password2")]
        Password2,
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
                Self::Password1 => serializer.serialize_unit_variant("Name", 0u32, "password1"),
                Self::Password2 => serializer.serialize_unit_variant("Name", 1u32, "password2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response from the GenerateCredentials operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateCredentialsResult {
    #[doc = "The username for a container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The list of passwords for a container registry."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<TokenPassword>,
}
impl GenerateCredentialsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP rule with specific IP or IP range in CIDR format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    #[doc = "The action of IP ACL rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ip_rule::Action>,
    #[doc = "Specifies the IP or IP range in CIDR format. Only IPV4 address is allowed."]
    pub value: String,
}
impl IpRule {
    pub fn new(value: String) -> Self {
        Self { action: None, value }
    }
}
pub mod ip_rule {
    use super::*;
    #[doc = "The action of IP ACL rule."]
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
    impl Default for Action {
        fn default() -> Self {
            Self::Allow
        }
    }
}
#[doc = "Managed identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProperties {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_properties::Type>,
    #[doc = "The list of user identities associated with the resource. The user identity \r\ndictionary key references will be ARM resource ids in the form: \r\n'/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/\r\n    providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl IdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_properties {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "Properties for a registry image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDescriptor {
    #[doc = "The registry login server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "The repository name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The tag name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The sha256-based digest of the image manifest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}
impl ImageDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The image update trigger that caused a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageUpdateTrigger {
    #[doc = "The unique ID of the trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The timestamp when the image update happened."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The list of image updates that caused the build."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ImageDescriptor>,
}
impl ImageUpdateTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportImageParameters {
    pub source: ImportSource,
    #[doc = "List of strings of the form repo[:tag]. When tag is omitted the source will be used (or 'latest' if source tag is also omitted)."]
    #[serde(rename = "targetTags", default, skip_serializing_if = "Vec::is_empty")]
    pub target_tags: Vec<String>,
    #[doc = "List of strings of repository names to do a manifest only copy. No tag will be created."]
    #[serde(rename = "untaggedTargetRepositories", default, skip_serializing_if = "Vec::is_empty")]
    pub untagged_target_repositories: Vec<String>,
    #[doc = "When Force, any existing target tags will be overwritten. When NoForce, any existing target tags will fail the operation before any copying begins."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<import_image_parameters::Mode>,
}
impl ImportImageParameters {
    pub fn new(source: ImportSource) -> Self {
        Self {
            source,
            target_tags: Vec::new(),
            untagged_target_repositories: Vec::new(),
            mode: None,
        }
    }
}
pub mod import_image_parameters {
    use super::*;
    #[doc = "When Force, any existing target tags will be overwritten. When NoForce, any existing target tags will fail the operation before any copying begins."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        NoForce,
        Force,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoForce => serializer.serialize_unit_variant("Mode", 0u32, "NoForce"),
                Self::Force => serializer.serialize_unit_variant("Mode", 1u32, "Force"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Mode {
        fn default() -> Self {
            Self::NoForce
        }
    }
}
#[doc = "An object that represents an import pipeline for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportPipeline {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The location of the import pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Managed identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The properties of an import pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImportPipelineProperties>,
}
impl ImportPipeline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list import pipelines for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportPipelineListResult {
    #[doc = "The list of import pipelines. Since this list may be incomplete, the nextLink field should be used to request the next list of import pipelines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ImportPipeline>,
    #[doc = "The URI that can be used to request the next list of pipeline runs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImportPipelineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ImportPipelineListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an import pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportPipelineProperties {
    #[doc = "The properties of the import pipeline source."]
    pub source: ImportPipelineSourceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<PipelineTriggerProperties>,
    #[doc = "The list of all options configured for the pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<String>,
    #[doc = "The provisioning state of the pipeline at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<import_pipeline_properties::ProvisioningState>,
}
impl ImportPipelineProperties {
    pub fn new(source: ImportPipelineSourceProperties) -> Self {
        Self {
            source,
            trigger: None,
            options: Vec::new(),
            provisioning_state: None,
        }
    }
}
pub mod import_pipeline_properties {
    use super::*;
    #[doc = "The provisioning state of the pipeline at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the import pipeline source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportPipelineSourceProperties {
    #[doc = "The type of source for the import pipeline."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<import_pipeline_source_properties::Type>,
    #[doc = "The source uri of the import pipeline.\r\nWhen 'AzureStorageBlob': \"https://accountName.blob.core.windows.net/containerName/blobName\"\r\nWhen 'AzureStorageBlobContainer': \"https://accountName.blob.core.windows.net/containerName\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "They key vault secret uri to obtain the source storage SAS token."]
    #[serde(rename = "keyVaultUri")]
    pub key_vault_uri: String,
}
impl ImportPipelineSourceProperties {
    pub fn new(key_vault_uri: String) -> Self {
        Self {
            type_: None,
            uri: None,
            key_vault_uri,
        }
    }
}
pub mod import_pipeline_source_properties {
    use super::*;
    #[doc = "The type of source for the import pipeline."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        AzureStorageBlobContainer,
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
                Self::AzureStorageBlobContainer => serializer.serialize_unit_variant("Type", 0u32, "AzureStorageBlobContainer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::AzureStorageBlobContainer
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportSource {
    #[doc = "The resource identifier of the source Azure Container Registry."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The address of the source registry (e.g. 'mcr.microsoft.com')."]
    #[serde(rename = "registryUri", default, skip_serializing_if = "Option::is_none")]
    pub registry_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ImportSourceCredentials>,
    #[doc = "Repository name of the source image.\r\nSpecify an image by repository ('hello-world'). This will use the 'latest' tag.\r\nSpecify an image by tag ('hello-world:latest').\r\nSpecify an image by sha256-based manifest digest ('hello-world@sha256:abc123')."]
    #[serde(rename = "sourceImage")]
    pub source_image: String,
}
impl ImportSource {
    pub fn new(source_image: String) -> Self {
        Self {
            resource_id: None,
            registry_uri: None,
            credentials: None,
            source_image,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportSourceCredentials {
    #[doc = "The username to authenticate with the source registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password used to authenticate with the source registry."]
    pub password: String,
}
impl ImportSourceCredentials {
    pub fn new(password: String) -> Self {
        Self { username: None, password }
    }
}
#[doc = "inner error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InnerErrorDescription {
    #[doc = "error code."]
    pub code: String,
    #[doc = "error message."]
    pub message: String,
    #[doc = "target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl InnerErrorDescription {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "Key vault uri to access the encryption key."]
    #[serde(rename = "keyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub key_identifier: Option<String>,
    #[doc = "The fully qualified key identifier that includes the version of the key that is actually used for encryption."]
    #[serde(rename = "versionedKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub versioned_key_identifier: Option<String>,
    #[doc = "The client id of the identity which will be used to access key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "Auto key rotation status for a CMK enabled registry."]
    #[serde(rename = "keyRotationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub key_rotation_enabled: Option<bool>,
    #[doc = "Timestamp of the last successful key rotation."]
    #[serde(rename = "lastKeyRotationTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub last_key_rotation_timestamp: Option<time::OffsetDateTime>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The logging properties of the connected registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoggingProperties {
    #[doc = "The verbosity of logs persisted on the connected registry."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<logging_properties::LogLevel>,
    #[doc = "Indicates whether audit logs are enabled on the connected registry."]
    #[serde(rename = "auditLogStatus", default, skip_serializing_if = "Option::is_none")]
    pub audit_log_status: Option<logging_properties::AuditLogStatus>,
}
impl LoggingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod logging_properties {
    use super::*;
    #[doc = "The verbosity of logs persisted on the connected registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LogLevel")]
    pub enum LogLevel {
        Debug,
        Information,
        Warning,
        Error,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LogLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LogLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LogLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Debug => serializer.serialize_unit_variant("LogLevel", 0u32, "Debug"),
                Self::Information => serializer.serialize_unit_variant("LogLevel", 1u32, "Information"),
                Self::Warning => serializer.serialize_unit_variant("LogLevel", 2u32, "Warning"),
                Self::Error => serializer.serialize_unit_variant("LogLevel", 3u32, "Error"),
                Self::None => serializer.serialize_unit_variant("LogLevel", 4u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for LogLevel {
        fn default() -> Self {
            Self::Information
        }
    }
    #[doc = "Indicates whether audit logs are enabled on the connected registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuditLogStatus")]
    pub enum AuditLogStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuditLogStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuditLogStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuditLogStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("AuditLogStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("AuditLogStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AuditLogStatus {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "The login server properties of the connected registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoginServerProperties {
    #[doc = "The host of the connected registry. Can be FQDN or IP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The TLS properties of the connected registry login server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsProperties>,
}
impl LoginServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network rule set for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[doc = "The default action of allow or deny when no other rules match."]
    #[serde(rename = "defaultAction")]
    pub default_action: network_rule_set::DefaultAction,
    #[doc = "The virtual network rules."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[doc = "The IP ACL rules."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
}
impl NetworkRuleSet {
    pub fn new(default_action: network_rule_set::DefaultAction) -> Self {
        Self {
            default_action,
            virtual_network_rules: Vec::new(),
            ip_rules: Vec::new(),
        }
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "The default action of allow or deny when no other rules match."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultAction")]
    pub enum DefaultAction {
        Allow,
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DefaultAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DefaultAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DefaultAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("DefaultAction", 0u32, "Allow"),
                Self::Deny => serializer.serialize_unit_variant("DefaultAction", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for DefaultAction {
        fn default() -> Self {
            Self::Allow
        }
    }
}
#[doc = "The definition of a container registry operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDefinition {
    #[doc = "The origin information of the container registry operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display information for a container registry operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayDefinition>,
    #[doc = "The definition of Azure Monitoring properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationPropertiesDefinition>,
    #[doc = "This property indicates if the operation is an action or a data action\r\nref: https://docs.microsoft.com/en-us/azure/role-based-access-control/role-definitions#management-and-data-operations"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl OperationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The display information for a container registry operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayDefinition {
    #[doc = "The resource provider name: Microsoft.ContainerRegistry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The operation that users can perform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The description for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplayDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list container registry operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of container registry operations. Since this list may be incomplete, the nextLink field should be used to request the next list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationDefinition>,
    #[doc = "The URI that can be used to request the next list of container registry operations."]
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
#[doc = "The definition of Azure Monitoring log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationLogSpecificationDefinition {
    #[doc = "Log name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Log display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Log blob duration."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl OperationLogSpecificationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of Azure Monitoring metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetricSpecificationDefinition {
    #[doc = "Metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metric display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Metric description."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Metric unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Metric aggregation type."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Internal metric name."]
    #[serde(rename = "internalMetricName", default, skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
}
impl OperationMetricSpecificationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of Azure Monitoring properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationPropertiesDefinition {
    #[doc = "The definition of Azure Monitoring list."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationServiceSpecificationDefinition>,
}
impl OperationPropertiesDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of Azure Monitoring list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationServiceSpecificationDefinition {
    #[doc = "A list of Azure Monitoring metrics definition."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetricSpecificationDefinition>,
    #[doc = "A list of Azure Monitoring log definitions."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<OperationLogSpecificationDefinition>,
}
impl OperationServiceSpecificationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OverrideTaskStepProperties {
    #[doc = "The source context against which run has to be queued."]
    #[serde(rename = "contextPath", default, skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[doc = "The file against which run has to be queued."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[doc = "Gets or sets the collection of override arguments to be used when\r\nexecuting a build step."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<Argument>,
    #[doc = "The name of the target build stage for the docker build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The collection of overridable values that can be passed when running a Task."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
    #[doc = "Base64 encoded update trigger token that will be attached with the base image trigger webhook."]
    #[serde(rename = "updateTriggerToken", default, skip_serializing_if = "Option::is_none")]
    pub update_trigger_token: Option<String>,
}
impl OverrideTaskStepProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the connected registry parent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParentProperties {
    #[doc = "The resource ID of the parent to which the connected registry will be associated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The sync properties of the connected registry with its parent."]
    #[serde(rename = "syncProperties")]
    pub sync_properties: SyncProperties,
}
impl ParentProperties {
    pub fn new(sync_properties: SyncProperties) -> Self {
        Self { id: None, sync_properties }
    }
}
#[doc = "An object that represents a pipeline run for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRun {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PipelineRunProperties>,
}
impl PipelineRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list pipeline runs for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRunListResult {
    #[doc = "The list of pipeline runs. Since this list may be incomplete, the nextLink field should be used to request the next list of pipeline runs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PipelineRun>,
    #[doc = "The URI that can be used to request the next list of pipeline runs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PipelineRunListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PipelineRunListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRunProperties {
    #[doc = "The provisioning state of a pipeline run."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<pipeline_run_properties::ProvisioningState>,
    #[doc = "The request properties provided for a pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<PipelineRunRequest>,
    #[doc = "The response properties returned for a pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<PipelineRunResponse>,
    #[doc = "How the pipeline run should be forced to recreate even if the pipeline run configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
}
impl PipelineRunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pipeline_run_properties {
    use super::*;
    #[doc = "The provisioning state of a pipeline run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The request properties provided for a pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRunRequest {
    #[doc = "The resource ID of the pipeline to run."]
    #[serde(rename = "pipelineResourceId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_resource_id: Option<String>,
    #[doc = "List of source artifacts to be transferred by the pipeline. \r\nSpecify an image by repository ('hello-world'). This will use the 'latest' tag.\r\nSpecify an image by tag ('hello-world:latest').\r\nSpecify an image by sha256-based manifest digest ('hello-world@sha256:abc123')."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PipelineRunSourceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<PipelineRunTargetProperties>,
    #[doc = "The digest of the tar used to transfer the artifacts."]
    #[serde(rename = "catalogDigest", default, skip_serializing_if = "Option::is_none")]
    pub catalog_digest: Option<String>,
}
impl PipelineRunRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response properties returned for a pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRunResponse {
    #[doc = "The current status of the pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The artifacts imported in the pipeline run."]
    #[serde(rename = "importedArtifacts", default, skip_serializing_if = "Vec::is_empty")]
    pub imported_artifacts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<ProgressProperties>,
    #[doc = "The time the pipeline run started."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The time the pipeline run finished."]
    #[serde(rename = "finishTime", default, with = "azure_core::date::rfc3339::option")]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The properties of the import pipeline source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ImportPipelineSourceProperties>,
    #[doc = "The properties of the export pipeline target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ExportPipelineTargetProperties>,
    #[doc = "The digest of the tar used to transfer the artifacts."]
    #[serde(rename = "catalogDigest", default, skip_serializing_if = "Option::is_none")]
    pub catalog_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<PipelineTriggerDescriptor>,
    #[doc = "The detailed error message for the pipeline run in the case of failure."]
    #[serde(rename = "pipelineRunErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_run_error_message: Option<String>,
}
impl PipelineRunResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRunSourceProperties {
    #[doc = "The type of the source."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<pipeline_run_source_properties::Type>,
    #[doc = "The name of the source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PipelineRunSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pipeline_run_source_properties {
    use super::*;
    #[doc = "The type of the source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        AzureStorageBlob,
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
                Self::AzureStorageBlob => serializer.serialize_unit_variant("Type", 0u32, "AzureStorageBlob"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::AzureStorageBlob
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRunTargetProperties {
    #[doc = "The type of the target."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<pipeline_run_target_properties::Type>,
    #[doc = "The name of the target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PipelineRunTargetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pipeline_run_target_properties {
    use super::*;
    #[doc = "The type of the target."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        AzureStorageBlob,
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
                Self::AzureStorageBlob => serializer.serialize_unit_variant("Type", 0u32, "AzureStorageBlob"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::AzureStorageBlob
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineSourceTriggerDescriptor {
    #[doc = "The timestamp when the source update happened."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl PipelineSourceTriggerDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineSourceTriggerProperties {
    #[doc = "The current status of the source trigger."]
    pub status: pipeline_source_trigger_properties::Status,
}
impl PipelineSourceTriggerProperties {
    pub fn new(status: pipeline_source_trigger_properties::Status) -> Self {
        Self { status }
    }
}
pub mod pipeline_source_trigger_properties {
    use super::*;
    #[doc = "The current status of the source trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTriggerDescriptor {
    #[serde(rename = "sourceTrigger", default, skip_serializing_if = "Option::is_none")]
    pub source_trigger: Option<PipelineSourceTriggerDescriptor>,
}
impl PipelineTriggerDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTriggerProperties {
    #[serde(rename = "sourceTrigger", default, skip_serializing_if = "Option::is_none")]
    pub source_trigger: Option<PipelineSourceTriggerProperties>,
}
impl PipelineTriggerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The platform properties against which the run has to happen."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformProperties {
    #[doc = "The operating system type required for the run."]
    pub os: platform_properties::Os,
    #[doc = "The OS architecture."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<platform_properties::Architecture>,
    #[doc = "Variant of the CPU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<platform_properties::Variant>,
}
impl PlatformProperties {
    pub fn new(os: platform_properties::Os) -> Self {
        Self {
            os,
            architecture: None,
            variant: None,
        }
    }
}
pub mod platform_properties {
    use super::*;
    #[doc = "The operating system type required for the run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Os")]
    pub enum Os {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Os {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Os {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Os {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("Os", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("Os", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The OS architecture."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Architecture")]
    pub enum Architecture {
        #[serde(rename = "amd64")]
        Amd64,
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "386")]
        N386,
        #[serde(rename = "arm")]
        Arm,
        #[serde(rename = "arm64")]
        Arm64,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Architecture {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Architecture {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Architecture {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Amd64 => serializer.serialize_unit_variant("Architecture", 0u32, "amd64"),
                Self::X86 => serializer.serialize_unit_variant("Architecture", 1u32, "x86"),
                Self::N386 => serializer.serialize_unit_variant("Architecture", 2u32, "386"),
                Self::Arm => serializer.serialize_unit_variant("Architecture", 3u32, "arm"),
                Self::Arm64 => serializer.serialize_unit_variant("Architecture", 4u32, "arm64"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Variant of the CPU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Variant")]
    pub enum Variant {
        #[serde(rename = "v6")]
        V6,
        #[serde(rename = "v7")]
        V7,
        #[serde(rename = "v8")]
        V8,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Variant {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Variant {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Variant {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V6 => serializer.serialize_unit_variant("Variant", 0u32, "v6"),
                Self::V7 => serializer.serialize_unit_variant("Variant", 1u32, "v7"),
                Self::V8 => serializer.serialize_unit_variant("Variant", 2u32, "v8"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties for updating the platform configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlatformUpdateParameters {
    #[doc = "The operating system type required for the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<platform_update_parameters::Os>,
    #[doc = "The OS architecture."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<platform_update_parameters::Architecture>,
    #[doc = "Variant of the CPU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<platform_update_parameters::Variant>,
}
impl PlatformUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod platform_update_parameters {
    use super::*;
    #[doc = "The operating system type required for the run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Os")]
    pub enum Os {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Os {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Os {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Os {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("Os", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("Os", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The OS architecture."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Architecture")]
    pub enum Architecture {
        #[serde(rename = "amd64")]
        Amd64,
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "386")]
        N386,
        #[serde(rename = "arm")]
        Arm,
        #[serde(rename = "arm64")]
        Arm64,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Architecture {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Architecture {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Architecture {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Amd64 => serializer.serialize_unit_variant("Architecture", 0u32, "amd64"),
                Self::X86 => serializer.serialize_unit_variant("Architecture", 1u32, "x86"),
                Self::N386 => serializer.serialize_unit_variant("Architecture", 2u32, "386"),
                Self::Arm => serializer.serialize_unit_variant("Architecture", 3u32, "arm"),
                Self::Arm64 => serializer.serialize_unit_variant("Architecture", 4u32, "arm64"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Variant of the CPU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Variant")]
    pub enum Variant {
        #[serde(rename = "v6")]
        V6,
        #[serde(rename = "v7")]
        V7,
        #[serde(rename = "v8")]
        V8,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Variant {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Variant {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Variant {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V6 => serializer.serialize_unit_variant("Variant", 0u32, "v6"),
                Self::V7 => serializer.serialize_unit_variant("Variant", 1u32, "v7"),
                Self::V8 => serializer.serialize_unit_variant("Variant", 2u32, "v8"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The policies for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Policies {
    #[doc = "The quarantine policy for a container registry."]
    #[serde(rename = "quarantinePolicy", default, skip_serializing_if = "Option::is_none")]
    pub quarantine_policy: Option<QuarantinePolicy>,
    #[doc = "The content trust policy for a container registry."]
    #[serde(rename = "trustPolicy", default, skip_serializing_if = "Option::is_none")]
    pub trust_policy: Option<TrustPolicy>,
    #[doc = "The retention policy for a container registry."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
    #[doc = "The export policy for a container registry."]
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<ExportPolicy>,
}
impl Policies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "This is private endpoint resource created with Microsoft.Network resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents a private endpoint connection for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list private endpoint connections for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "The list of private endpoint connections. Since this list may be incomplete, the nextLink field should be used to request the next list of private endpoint connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "The URI that can be used to request the next list of private endpoint connections."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "The state of a private link service connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
    #[doc = "The provisioning state of private endpoint connection resource."]
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
    #[doc = "The provisioning state of private endpoint connection resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A resource that supports private link capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[doc = "The resource type is private link resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list private link resources for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "The list of private link resources. Since this list may be incomplete, the nextLink field should be used to request the next list of private link resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "The URI that can be used to request the next list of private link resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of a private link service connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private link service connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<private_link_service_connection_state::Status>,
    #[doc = "The description for connection status. For example if connection is rejected it can indicate reason for rejection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<private_link_service_connection_state::ActionsRequired>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "The private link service connection status."]
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
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionsRequired")]
    pub enum ActionsRequired {
        None,
        Recreate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionsRequired {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionsRequired {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionsRequired {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ActionsRequired", 0u32, "None"),
                Self::Recreate => serializer.serialize_unit_variant("ActionsRequired", 1u32, "Recreate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProgressProperties {
    #[doc = "The percentage complete of the copy operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
}
impl ProgressProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM proxy resource. It will have everything other than required location and tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The quarantine policy for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuarantinePolicy {
    #[doc = "The value that indicates whether the policy is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<quarantine_policy::Status>,
}
impl QuarantinePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod quarantine_policy {
    use super::*;
    #[doc = "The value that indicates whether the policy is enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "The parameters used to regenerate the login credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateCredentialParameters {
    #[doc = "Specifies name of the password which should be regenerated -- password or password2."]
    pub name: regenerate_credential_parameters::Name,
}
impl RegenerateCredentialParameters {
    pub fn new(name: regenerate_credential_parameters::Name) -> Self {
        Self { name }
    }
}
pub mod regenerate_credential_parameters {
    use super::*;
    #[doc = "Specifies name of the password which should be regenerated -- password or password2."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "password")]
        Password,
        #[serde(rename = "password2")]
        Password2,
    }
}
#[doc = "An object that represents a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Registry {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU of a container registry."]
    pub sku: Sku,
    #[doc = "Managed identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The properties of a container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryProperties>,
}
impl Registry {
    pub fn new(resource: Resource, sku: Sku) -> Self {
        Self {
            resource,
            sku,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "The response from the ListCredentials operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryListCredentialsResult {
    #[doc = "The username for a container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The list of passwords for a container registry."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<RegistryPassword>,
}
impl RegistryListCredentialsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list container registries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryListResult {
    #[doc = "The list of container registries. Since this list may be incomplete, the nextLink field should be used to request the next list of container registries."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Registry>,
    #[doc = "The URI that can be used to request the next list of container registries."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RegistryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RegistryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request to check whether a container registry name is available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryNameCheckRequest {
    #[doc = "The name of the container registry."]
    pub name: String,
    #[doc = "The resource type of the container registry. This field must be set to 'Microsoft.ContainerRegistry/registries'."]
    #[serde(rename = "type")]
    pub type_: registry_name_check_request::Type,
}
impl RegistryNameCheckRequest {
    pub fn new(name: String, type_: registry_name_check_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod registry_name_check_request {
    use super::*;
    #[doc = "The resource type of the container registry. This field must be set to 'Microsoft.ContainerRegistry/registries'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.ContainerRegistry/registries")]
        MicrosoftContainerRegistryRegistries,
    }
}
#[doc = "The result of a request to check the availability of a container registry name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryNameStatus {
    #[doc = "The value that indicates whether the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "If any, the reason that the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "If any, the error message that provides more detail for the reason that the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl RegistryNameStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The login password for the container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryPassword {
    #[doc = "The password name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<registry_password::Name>,
    #[doc = "The password value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl RegistryPassword {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod registry_password {
    use super::*;
    #[doc = "The password name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "password")]
        Password,
        #[serde(rename = "password2")]
        Password2,
    }
}
#[doc = "The properties of a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryProperties {
    #[doc = "The URL that can be used to log into the container registry."]
    #[serde(rename = "loginServer", default, skip_serializing_if = "Option::is_none")]
    pub login_server: Option<String>,
    #[doc = "The creation date of the container registry in ISO8601 format."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the container registry at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<registry_properties::ProvisioningState>,
    #[doc = "The status of an Azure resource at the time the operation was called."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The value that indicates whether the admin user is enabled."]
    #[serde(rename = "adminUserEnabled", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_enabled: Option<bool>,
    #[doc = "The network rule set for a container registry."]
    #[serde(rename = "networkRuleSet", default, skip_serializing_if = "Option::is_none")]
    pub network_rule_set: Option<NetworkRuleSet>,
    #[doc = "The policies for a container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Policies>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperty>,
    #[doc = "Enable a single data endpoint per region for serving data."]
    #[serde(rename = "dataEndpointEnabled", default, skip_serializing_if = "Option::is_none")]
    pub data_endpoint_enabled: Option<bool>,
    #[doc = "List of host names that will serve data when dataEndpointEnabled is true."]
    #[serde(rename = "dataEndpointHostNames", default, skip_serializing_if = "Vec::is_empty")]
    pub data_endpoint_host_names: Vec<String>,
    #[doc = "List of private endpoint connections for a container registry."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Whether or not public network access is allowed for the container registry."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<registry_properties::PublicNetworkAccess>,
    #[doc = "Whether to allow trusted Azure services to access a network restricted registry."]
    #[serde(rename = "networkRuleBypassOptions", default, skip_serializing_if = "Option::is_none")]
    pub network_rule_bypass_options: Option<registry_properties::NetworkRuleBypassOptions>,
    #[doc = "Whether or not zone redundancy is enabled for this container registry"]
    #[serde(rename = "zoneRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundancy: Option<registry_properties::ZoneRedundancy>,
    #[doc = "Enables registry-wide pull from unauthenticated clients."]
    #[serde(rename = "anonymousPullEnabled", default, skip_serializing_if = "Option::is_none")]
    pub anonymous_pull_enabled: Option<bool>,
}
impl RegistryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod registry_properties {
    use super::*;
    #[doc = "The provisioning state of the container registry at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether or not public network access is allowed for the container registry."]
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
    #[doc = "Whether to allow trusted Azure services to access a network restricted registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkRuleBypassOptions")]
    pub enum NetworkRuleBypassOptions {
        AzureServices,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NetworkRuleBypassOptions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NetworkRuleBypassOptions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NetworkRuleBypassOptions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureServices => serializer.serialize_unit_variant("NetworkRuleBypassOptions", 0u32, "AzureServices"),
                Self::None => serializer.serialize_unit_variant("NetworkRuleBypassOptions", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for NetworkRuleBypassOptions {
        fn default() -> Self {
            Self::AzureServices
        }
    }
    #[doc = "Whether or not zone redundancy is enabled for this container registry"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ZoneRedundancy")]
    pub enum ZoneRedundancy {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ZoneRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ZoneRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ZoneRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ZoneRedundancy", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ZoneRedundancy", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ZoneRedundancy {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "The parameters for updating the properties of a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryPropertiesUpdateParameters {
    #[doc = "The value that indicates whether the admin user is enabled."]
    #[serde(rename = "adminUserEnabled", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_enabled: Option<bool>,
    #[doc = "The network rule set for a container registry."]
    #[serde(rename = "networkRuleSet", default, skip_serializing_if = "Option::is_none")]
    pub network_rule_set: Option<NetworkRuleSet>,
    #[doc = "The policies for a container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Policies>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperty>,
    #[doc = "Enable a single data endpoint per region for serving data."]
    #[serde(rename = "dataEndpointEnabled", default, skip_serializing_if = "Option::is_none")]
    pub data_endpoint_enabled: Option<bool>,
    #[doc = "Whether or not public network access is allowed for the container registry."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<registry_properties_update_parameters::PublicNetworkAccess>,
    #[doc = "Whether to allow trusted Azure services to access a network restricted registry."]
    #[serde(rename = "networkRuleBypassOptions", default, skip_serializing_if = "Option::is_none")]
    pub network_rule_bypass_options: Option<registry_properties_update_parameters::NetworkRuleBypassOptions>,
    #[doc = "Enables registry-wide pull from unauthenticated clients."]
    #[serde(rename = "anonymousPullEnabled", default, skip_serializing_if = "Option::is_none")]
    pub anonymous_pull_enabled: Option<bool>,
}
impl RegistryPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod registry_properties_update_parameters {
    use super::*;
    #[doc = "Whether or not public network access is allowed for the container registry."]
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
    #[doc = "Whether to allow trusted Azure services to access a network restricted registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkRuleBypassOptions")]
    pub enum NetworkRuleBypassOptions {
        AzureServices,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NetworkRuleBypassOptions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NetworkRuleBypassOptions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NetworkRuleBypassOptions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureServices => serializer.serialize_unit_variant("NetworkRuleBypassOptions", 0u32, "AzureServices"),
                Self::None => serializer.serialize_unit_variant("NetworkRuleBypassOptions", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for NetworkRuleBypassOptions {
        fn default() -> Self {
            Self::AzureServices
        }
    }
}
#[doc = "The parameters for updating a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryUpdateParameters {
    #[doc = "Managed identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The tags for the container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The SKU of a container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The parameters for updating the properties of a container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryPropertiesUpdateParameters>,
}
impl RegistryUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The quota usage for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryUsage {
    #[doc = "The name of the usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The limit of the usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The current value of the usage."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The unit of measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<registry_usage::Unit>,
}
impl RegistryUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod registry_usage {
    use super::*;
    #[doc = "The unit of measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        Bytes,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::Bytes => serializer.serialize_unit_variant("Unit", 1u32, "Bytes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result of a request to get container registry quota usages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryUsageListResult {
    #[doc = "The list of container registry quota usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegistryUsage>,
}
impl RegistryUsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents a replication for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Replication {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationProperties>,
}
impl Replication {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The result of a request to list replications for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationListResult {
    #[doc = "The list of replications. Since this list may be incomplete, the nextLink field should be used to request the next list of replications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Replication>,
    #[doc = "The URI that can be used to request the next list of replications."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReplicationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReplicationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a replication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationProperties {
    #[doc = "The provisioning state of the replication at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<replication_properties::ProvisioningState>,
    #[doc = "The status of an Azure resource at the time the operation was called."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "Specifies whether the replication's regional endpoint is enabled. Requests will not be routed to a replication whose regional endpoint is disabled, however its data will continue to be synced with other replications."]
    #[serde(rename = "regionEndpointEnabled", default, skip_serializing_if = "Option::is_none")]
    pub region_endpoint_enabled: Option<bool>,
    #[doc = "Whether or not zone redundancy is enabled for this container registry replication"]
    #[serde(rename = "zoneRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundancy: Option<replication_properties::ZoneRedundancy>,
}
impl ReplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replication_properties {
    use super::*;
    #[doc = "The provisioning state of the replication at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether or not zone redundancy is enabled for this container registry replication"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ZoneRedundancy")]
    pub enum ZoneRedundancy {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ZoneRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ZoneRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ZoneRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ZoneRedundancy", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ZoneRedundancy", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ZoneRedundancy {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "The parameters for updating a replication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationUpdateParameters {
    #[doc = "The tags for the replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationUpdateParametersProperties>,
}
impl ReplicationUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationUpdateParametersProperties {
    #[doc = "Specifies whether the replication's regional endpoint is enabled. Requests will not be routed to a replication whose regional endpoint is disabled, however its data will continue to be synced with other replications."]
    #[serde(rename = "regionEndpointEnabled", default, skip_serializing_if = "Option::is_none")]
    pub region_endpoint_enabled: Option<bool>,
}
impl ReplicationUpdateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request that generated the event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Request {
    #[doc = "The ID of the request that initiated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The IP or hostname and possibly port of the client connection that initiated the event. This is the RemoteAddr from the standard http request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[doc = "The externally accessible hostname of the registry instance, as specified by the http host header on incoming requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The request method that generated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "The user agent header of the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useragent: Option<String>,
}
impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource. This cannot be changed after the resource is created."]
    pub location: String,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            system_data: None,
        }
    }
}
#[doc = "The retention policy for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionPolicy {
    #[doc = "The number of days to retain an untagged manifest after which it gets purged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[doc = "The timestamp when the policy was last updated."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "The value that indicates whether the policy is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<retention_policy::Status>,
}
impl RetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod retention_policy {
    use super::*;
    #[doc = "The value that indicates whether the policy is enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "Run resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Run {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties for a run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunProperties>,
}
impl Run {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties that are enabled for Odata querying on runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunFilter {
    #[doc = "The unique identifier for the run."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The type of run."]
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<run_filter::RunType>,
    #[doc = "The current status of the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<run_filter::Status>,
    #[doc = "The create time for a run."]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The time the run finished."]
    #[serde(rename = "finishTime", default, with = "azure_core::date::rfc3339::option")]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The list of comma-separated image manifests that were generated from the run. This is applicable if the run is of\r\nbuild type."]
    #[serde(rename = "outputImageManifests", default, skip_serializing_if = "Option::is_none")]
    pub output_image_manifests: Option<String>,
    #[doc = "The value that indicates whether archiving is enabled or not."]
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
    #[doc = "The name of the task that the run corresponds to."]
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[doc = "The name of the agent pool that the run corresponds to."]
    #[serde(rename = "agentPoolName", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool_name: Option<String>,
}
impl RunFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_filter {
    use super::*;
    #[doc = "The type of run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunType")]
    pub enum RunType {
        QuickBuild,
        QuickRun,
        AutoBuild,
        AutoRun,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::QuickBuild => serializer.serialize_unit_variant("RunType", 0u32, "QuickBuild"),
                Self::QuickRun => serializer.serialize_unit_variant("RunType", 1u32, "QuickRun"),
                Self::AutoBuild => serializer.serialize_unit_variant("RunType", 2u32, "AutoBuild"),
                Self::AutoRun => serializer.serialize_unit_variant("RunType", 3u32, "AutoRun"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of the run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Queued,
        Started,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Error,
        Timeout,
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
                Self::Queued => serializer.serialize_unit_variant("Status", 0u32, "Queued"),
                Self::Started => serializer.serialize_unit_variant("Status", 1u32, "Started"),
                Self::Running => serializer.serialize_unit_variant("Status", 2u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 5u32, "Canceled"),
                Self::Error => serializer.serialize_unit_variant("Status", 6u32, "Error"),
                Self::Timeout => serializer.serialize_unit_variant("Status", 7u32, "Timeout"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result of get log link operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunGetLogResult {
    #[doc = "The link to logs for a run on a azure container registry."]
    #[serde(rename = "logLink", default, skip_serializing_if = "Option::is_none")]
    pub log_link: Option<String>,
    #[doc = "The link to logs in registry for a run on a azure container registry."]
    #[serde(rename = "logArtifactLink", default, skip_serializing_if = "Option::is_none")]
    pub log_artifact_link: Option<String>,
}
impl RunGetLogResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Run>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RunListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RunListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for a run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunProperties {
    #[doc = "The unique identifier for the run."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The current status of the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<run_properties::Status>,
    #[doc = "The last updated time for the run."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "The type of run."]
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<run_properties::RunType>,
    #[doc = "The dedicated agent pool for the run."]
    #[serde(rename = "agentPoolName", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool_name: Option<String>,
    #[doc = "The time the run was scheduled."]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The time the run started."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The time the run finished."]
    #[serde(rename = "finishTime", default, with = "azure_core::date::rfc3339::option")]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The list of all images that were generated from the run. This is applicable if the run generates base image dependencies."]
    #[serde(rename = "outputImages", default, skip_serializing_if = "Vec::is_empty")]
    pub output_images: Vec<ImageDescriptor>,
    #[doc = "The task against which run was scheduled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    #[doc = "The image update trigger that caused a build."]
    #[serde(rename = "imageUpdateTrigger", default, skip_serializing_if = "Option::is_none")]
    pub image_update_trigger: Option<ImageUpdateTrigger>,
    #[doc = "The source trigger that caused a run."]
    #[serde(rename = "sourceTrigger", default, skip_serializing_if = "Option::is_none")]
    pub source_trigger: Option<SourceTriggerDescriptor>,
    #[serde(rename = "timerTrigger", default, skip_serializing_if = "Option::is_none")]
    pub timer_trigger: Option<TimerTriggerDescriptor>,
    #[doc = "The platform properties against which the run has to happen."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformProperties>,
    #[doc = "The properties that determine the run agent configuration."]
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[doc = "The scope of the credentials that were used to login to the source registry during this run."]
    #[serde(rename = "sourceRegistryAuth", default, skip_serializing_if = "Option::is_none")]
    pub source_registry_auth: Option<String>,
    #[doc = "The list of custom registries that were logged in during this run."]
    #[serde(rename = "customRegistries", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_registries: Vec<String>,
    #[doc = "The error message received from backend systems after the run is scheduled."]
    #[serde(rename = "runErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub run_error_message: Option<String>,
    #[doc = "The update trigger token passed for the Run."]
    #[serde(rename = "updateTriggerToken", default, skip_serializing_if = "Option::is_none")]
    pub update_trigger_token: Option<String>,
    #[doc = "Properties for a registry image."]
    #[serde(rename = "logArtifact", default, skip_serializing_if = "Option::is_none")]
    pub log_artifact: Option<ImageDescriptor>,
    #[doc = "The provisioning state of a run."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<run_properties::ProvisioningState>,
    #[doc = "The value that indicates whether archiving is enabled or not."]
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
}
impl RunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_properties {
    use super::*;
    #[doc = "The current status of the run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Queued,
        Started,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Error,
        Timeout,
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
                Self::Queued => serializer.serialize_unit_variant("Status", 0u32, "Queued"),
                Self::Started => serializer.serialize_unit_variant("Status", 1u32, "Started"),
                Self::Running => serializer.serialize_unit_variant("Status", 2u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 5u32, "Canceled"),
                Self::Error => serializer.serialize_unit_variant("Status", 6u32, "Error"),
                Self::Timeout => serializer.serialize_unit_variant("Status", 7u32, "Timeout"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunType")]
    pub enum RunType {
        QuickBuild,
        QuickRun,
        AutoBuild,
        AutoRun,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::QuickBuild => serializer.serialize_unit_variant("RunType", 0u32, "QuickBuild"),
                Self::QuickRun => serializer.serialize_unit_variant("RunType", 1u32, "QuickRun"),
                Self::AutoBuild => serializer.serialize_unit_variant("RunType", 2u32, "AutoBuild"),
                Self::AutoRun => serializer.serialize_unit_variant("RunType", 3u32, "AutoRun"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of a run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The request parameters for scheduling a run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunRequest {
    #[doc = "The type of the run request."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The value that indicates whether archiving is enabled for the run or not."]
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
    #[doc = "The dedicated agent pool for the run."]
    #[serde(rename = "agentPoolName", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool_name: Option<String>,
    #[doc = "The template that describes the repository and tag information for run log artifact."]
    #[serde(rename = "logTemplate", default, skip_serializing_if = "Option::is_none")]
    pub log_template: Option<String>,
}
impl RunRequest {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            is_archive_enabled: None,
            agent_pool_name: None,
            log_template: None,
        }
    }
}
#[doc = "The set of run properties that can be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunUpdateParameters {
    #[doc = "The value that indicates whether archiving is enabled or not."]
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
}
impl RunUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents a scope map for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeMap {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a scope map."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopeMapProperties>,
}
impl ScopeMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list scope maps for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeMapListResult {
    #[doc = "The list of scope maps. Since this list may be incomplete, the nextLink field should be used to request the next list of scope maps."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScopeMap>,
    #[doc = "The URI that can be used to request the next list of scope maps."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScopeMapListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScopeMapListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a scope map."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeMapProperties {
    #[doc = "The user friendly description of the scope map."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The type of the scope map. E.g. BuildIn scope map."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The creation date of scope map."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<scope_map_properties::ProvisioningState>,
    #[doc = "The list of scoped permissions for registry artifacts.\r\nE.g. repositories/repository-name/content/read,\r\nrepositories/repository-name/metadata/write"]
    pub actions: Vec<String>,
}
impl ScopeMapProperties {
    pub fn new(actions: Vec<String>) -> Self {
        Self {
            description: None,
            type_: None,
            creation_date: None,
            provisioning_state: None,
            actions,
        }
    }
}
pub mod scope_map_properties {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The update parameters for scope map properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeMapPropertiesUpdateParameters {
    #[doc = "The user friendly description of the scope map."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of scope permissions for registry artifacts.\r\nE.g. repositories/repository-name/pull, \r\nrepositories/repository-name/delete"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
}
impl ScopeMapPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for updating the scope map."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeMapUpdateParameters {
    #[doc = "The update parameters for scope map properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopeMapPropertiesUpdateParameters>,
}
impl ScopeMapUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a secret object value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretObject {
    #[doc = "The value of the secret. The format of this value will be determined\r\nbased on the type of the secret object. If the type is Opaque, the value will be\r\nused as is without any modification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The type of the secret object which determines how the value of the secret object has to be\r\ninterpreted."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<secret_object::Type>,
}
impl SecretObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod secret_object {
    use super::*;
    #[doc = "The type of the secret object which determines how the value of the secret object has to be\r\ninterpreted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Opaque,
        Vaultsecret,
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
                Self::Opaque => serializer.serialize_unit_variant("Type", 0u32, "Opaque"),
                Self::Vaultsecret => serializer.serialize_unit_variant("Type", 1u32, "Vaultsecret"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a overridable value that can be passed to a task template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetValue {
    #[doc = "The name of the overridable value."]
    pub name: String,
    #[doc = "The overridable value."]
    pub value: String,
    #[doc = "Flag to indicate whether the value represents a secret or not."]
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
}
impl SetValue {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            is_secret: None,
        }
    }
}
#[doc = "The SKU of a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The SKU name of the container registry. Required for registry creation."]
    pub name: sku::Name,
    #[doc = "The SKU tier based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The SKU name of the container registry. Required for registry creation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Classic,
        Basic,
        Standard,
        Premium,
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
                Self::Classic => serializer.serialize_unit_variant("Name", 0u32, "Classic"),
                Self::Basic => serializer.serialize_unit_variant("Name", 1u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 2u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Name", 3u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SKU tier based on the SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Classic,
        Basic,
        Standard,
        Premium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Classic => serializer.serialize_unit_variant("Tier", 0u32, "Classic"),
                Self::Basic => serializer.serialize_unit_variant("Tier", 1u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Tier", 2u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Tier", 3u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The registry node that generated the event. Put differently, while the actor initiates the event, the source generates it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Source {
    #[doc = "The IP or hostname and the port of the registry node that generated the event. Generally, this will be resolved by os.Hostname() along with the running port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[doc = "The running instance of an application. Changes after each restart."]
    #[serde(rename = "instanceID", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}
impl Source {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the source code repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceProperties {
    #[doc = "The type of source control service."]
    #[serde(rename = "sourceControlType")]
    pub source_control_type: source_properties::SourceControlType,
    #[doc = "The full URL to the source code repository"]
    #[serde(rename = "repositoryUrl")]
    pub repository_url: String,
    #[doc = "The branch name of the source code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "The authorization properties for accessing the source code repository."]
    #[serde(rename = "sourceControlAuthProperties", default, skip_serializing_if = "Option::is_none")]
    pub source_control_auth_properties: Option<AuthInfo>,
}
impl SourceProperties {
    pub fn new(source_control_type: source_properties::SourceControlType, repository_url: String) -> Self {
        Self {
            source_control_type,
            repository_url,
            branch: None,
            source_control_auth_properties: None,
        }
    }
}
pub mod source_properties {
    use super::*;
    #[doc = "The type of source control service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceControlType")]
    pub enum SourceControlType {
        Github,
        VisualStudioTeamService,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceControlType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceControlType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceControlType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Github => serializer.serialize_unit_variant("SourceControlType", 0u32, "Github"),
                Self::VisualStudioTeamService => serializer.serialize_unit_variant("SourceControlType", 1u32, "VisualStudioTeamService"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the credential parameters for accessing the source registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceRegistryCredentials {
    #[doc = "The authentication mode which determines the source registry login scope. The credentials for the source registry\r\nwill be generated using the given scope. These credentials will be used to login to\r\nthe source registry during the run."]
    #[serde(rename = "loginMode", default, skip_serializing_if = "Option::is_none")]
    pub login_mode: Option<source_registry_credentials::LoginMode>,
}
impl SourceRegistryCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_registry_credentials {
    use super::*;
    #[doc = "The authentication mode which determines the source registry login scope. The credentials for the source registry\r\nwill be generated using the given scope. These credentials will be used to login to\r\nthe source registry during the run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoginMode")]
    pub enum LoginMode {
        None,
        Default,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LoginMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LoginMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LoginMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("LoginMode", 0u32, "None"),
                Self::Default => serializer.serialize_unit_variant("LoginMode", 1u32, "Default"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a source based trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceTrigger {
    #[doc = "The properties of the source code repository."]
    #[serde(rename = "sourceRepository")]
    pub source_repository: SourceProperties,
    #[doc = "The source event corresponding to the trigger."]
    #[serde(rename = "sourceTriggerEvents")]
    pub source_trigger_events: Vec<String>,
    #[doc = "The current status of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<source_trigger::Status>,
    #[doc = "The name of the trigger."]
    pub name: String,
}
impl SourceTrigger {
    pub fn new(source_repository: SourceProperties, source_trigger_events: Vec<String>, name: String) -> Self {
        Self {
            source_repository,
            source_trigger_events,
            status: None,
            name,
        }
    }
}
pub mod source_trigger {
    use super::*;
    #[doc = "The current status of trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "The source trigger that caused a run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceTriggerDescriptor {
    #[doc = "The unique ID of the trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The event type of the trigger."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "The unique ID that identifies a commit."]
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[doc = "The unique ID that identifies pull request."]
    #[serde(rename = "pullRequestId", default, skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    #[doc = "The repository URL."]
    #[serde(rename = "repositoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[doc = "The branch name in the repository."]
    #[serde(rename = "branchName", default, skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[doc = "The source control provider type."]
    #[serde(rename = "providerType", default, skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}
impl SourceTriggerDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for updating a source based trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceTriggerUpdateParameters {
    #[doc = "The properties for updating the source code repository."]
    #[serde(rename = "sourceRepository", default, skip_serializing_if = "Option::is_none")]
    pub source_repository: Option<SourceUpdateParameters>,
    #[doc = "The source event corresponding to the trigger."]
    #[serde(rename = "sourceTriggerEvents", default, skip_serializing_if = "Vec::is_empty")]
    pub source_trigger_events: Vec<String>,
    #[doc = "The current status of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<source_trigger_update_parameters::Status>,
    #[doc = "The name of the trigger."]
    pub name: String,
}
impl SourceTriggerUpdateParameters {
    pub fn new(name: String) -> Self {
        Self {
            source_repository: None,
            source_trigger_events: Vec::new(),
            status: None,
            name,
        }
    }
}
pub mod source_trigger_update_parameters {
    use super::*;
    #[doc = "The current status of trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "The properties for updating the source code repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceUpdateParameters {
    #[doc = "The type of source control service."]
    #[serde(rename = "sourceControlType", default, skip_serializing_if = "Option::is_none")]
    pub source_control_type: Option<source_update_parameters::SourceControlType>,
    #[doc = "The full URL to the source code repository"]
    #[serde(rename = "repositoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[doc = "The branch name of the source code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "The authorization properties for accessing the source code repository."]
    #[serde(rename = "sourceControlAuthProperties", default, skip_serializing_if = "Option::is_none")]
    pub source_control_auth_properties: Option<AuthInfoUpdateParameters>,
}
impl SourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_update_parameters {
    use super::*;
    #[doc = "The type of source control service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceControlType")]
    pub enum SourceControlType {
        Github,
        VisualStudioTeamService,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceControlType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceControlType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceControlType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Github => serializer.serialize_unit_variant("SourceControlType", 0u32, "Github"),
                Self::VisualStudioTeamService => serializer.serialize_unit_variant("SourceControlType", 1u32, "VisualStudioTeamService"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a response to source upload request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceUploadDefinition {
    #[doc = "The URL where the client can upload the source."]
    #[serde(rename = "uploadUrl", default, skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    #[doc = "The relative path to the source. This is used to submit the subsequent queue build request."]
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}
impl SourceUploadDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of an Azure resource at the time the operation was called."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Status {
    #[doc = "The short label for the status."]
    #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<String>,
    #[doc = "The detailed message for the status, including alerts and error messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The timestamp when the status was changed to the current value."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl Status {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status detail properties of the connected registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusDetailProperties {
    #[doc = "The component of the connected registry corresponding to the status."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The code of the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The description of the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The timestamp of the status."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The correlation ID of the status."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
}
impl StatusDetailProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sync properties of the connected registry with its parent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncProperties {
    #[doc = "The resource ID of the ACR token used to authenticate the connected registry to its parent during sync."]
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[doc = "The cron expression indicating the schedule that the connected registry will sync with its parent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[doc = "The time window during which sync is enabled for each schedule occurrence. Specify the duration using the format P[n]Y[n]M[n]DT[n]H[n]M[n]S as per ISO8601."]
    #[serde(rename = "syncWindow", default, skip_serializing_if = "Option::is_none")]
    pub sync_window: Option<String>,
    #[doc = "The period of time for which a message is available to sync before it is expired. Specify the duration using the format P[n]Y[n]M[n]DT[n]H[n]M[n]S as per ISO8601."]
    #[serde(rename = "messageTtl")]
    pub message_ttl: String,
    #[doc = "The last time a sync occurred between the connected registry and its parent."]
    #[serde(rename = "lastSyncTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_sync_time: Option<time::OffsetDateTime>,
    #[doc = "The gateway endpoint used by the connected registry to communicate with its parent."]
    #[serde(rename = "gatewayEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub gateway_endpoint: Option<String>,
}
impl SyncProperties {
    pub fn new(token_id: String, message_ttl: String) -> Self {
        Self {
            token_id,
            schedule: None,
            sync_window: None,
            message_ttl,
            last_sync_time: None,
            gateway_endpoint: None,
        }
    }
}
#[doc = "The parameters for updating the sync properties of the connected registry with its parent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncUpdateProperties {
    #[doc = "The cron expression indicating the schedule that the connected registry will sync with its parent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[doc = "The time window during which sync is enabled for each schedule occurrence. Specify the duration using the format P[n]Y[n]M[n]DT[n]H[n]M[n]S as per ISO8601."]
    #[serde(rename = "syncWindow", default, skip_serializing_if = "Option::is_none")]
    pub sync_window: Option<String>,
    #[doc = "The period of time for which a message is available to sync before it is expired. Specify the duration using the format P[n]Y[n]M[n]DT[n]H[n]M[n]S as per ISO8601."]
    #[serde(rename = "messageTtl", default, skip_serializing_if = "Option::is_none")]
    pub message_ttl: Option<String>,
}
impl SyncUpdateProperties {
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
    #[doc = "The timestamp of resource modification (UTC)."]
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
#[doc = "The target of the event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Target {
    #[doc = "The MIME type of the referenced object."]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "The number of bytes of the content. Same as Length field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The digest of the content, as defined by the Registry V2 HTTP API Specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "The number of bytes of the content. Same as Size field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[doc = "The repository name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The direct URL to the content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The tag name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The name of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Target {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The task that has the ARM resource and task properties. \r\nThe task will have all information to schedule a run against it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Managed identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The properties of a task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskProperties>,
}
impl Task {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "The collection of tasks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Task>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TaskListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TaskListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskProperties {
    #[doc = "The provisioning state of the task."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<task_properties::ProvisioningState>,
    #[doc = "The creation date of task."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The current status of task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<task_properties::Status>,
    #[doc = "The platform properties against which the run has to happen."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformProperties>,
    #[doc = "The properties that determine the run agent configuration."]
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[doc = "The dedicated agent pool for the task."]
    #[serde(rename = "agentPoolName", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool_name: Option<String>,
    #[doc = "Run timeout in seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[doc = "Base properties for any task step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<TaskStepProperties>,
    #[doc = "The properties of a trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TriggerProperties>,
    #[doc = "The parameters that describes a set of credentials that will be used when a run is invoked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[doc = "The template that describes the repository and tag information for run log artifact."]
    #[serde(rename = "logTemplate", default, skip_serializing_if = "Option::is_none")]
    pub log_template: Option<String>,
    #[doc = "The value of this property indicates whether the task resource is system task or not."]
    #[serde(rename = "isSystemTask", default, skip_serializing_if = "Option::is_none")]
    pub is_system_task: Option<bool>,
}
impl TaskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_properties {
    use super::*;
    #[doc = "The provisioning state of the task."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of task."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties for updating a task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskPropertiesUpdateParameters {
    #[doc = "The current status of task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<task_properties_update_parameters::Status>,
    #[doc = "The properties for updating the platform configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformUpdateParameters>,
    #[doc = "The properties that determine the run agent configuration."]
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[doc = "The dedicated agent pool for the task."]
    #[serde(rename = "agentPoolName", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool_name: Option<String>,
    #[doc = "Run timeout in seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[doc = "Base properties for updating any task step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<TaskStepUpdateParameters>,
    #[doc = "The properties for updating triggers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TriggerUpdateParameters>,
    #[doc = "The parameters that describes a set of credentials that will be used when a run is invoked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[doc = "The template that describes the repository and tag information for run log artifact."]
    #[serde(rename = "logTemplate", default, skip_serializing_if = "Option::is_none")]
    pub log_template: Option<String>,
}
impl TaskPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_properties_update_parameters {
    use super::*;
    #[doc = "The current status of task."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The task run that has the ARM resource and properties. \r\nThe task run will have the information of request and result of a run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskRun {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Managed identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The properties of task run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskRunProperties>,
    #[doc = "The location of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TaskRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The collection of task runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskRunListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TaskRun>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TaskRunListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TaskRunListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of task run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskRunProperties {
    #[doc = "The provisioning state of this task run"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<task_run_properties::ProvisioningState>,
    #[doc = "The request parameters for scheduling a run."]
    #[serde(rename = "runRequest", default, skip_serializing_if = "Option::is_none")]
    pub run_request: Option<RunRequest>,
    #[doc = "Run resource properties"]
    #[serde(rename = "runResult", default, skip_serializing_if = "Option::is_none")]
    pub run_result: Option<Run>,
    #[doc = "How the run should be forced to rerun even if the run request configuration has not changed"]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
}
impl TaskRunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_run_properties {
    use super::*;
    #[doc = "The provisioning state of this task run"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a task run update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskRunPropertiesUpdateParameters {
    #[doc = "The request parameters for scheduling a run."]
    #[serde(rename = "runRequest", default, skip_serializing_if = "Option::is_none")]
    pub run_request: Option<RunRequest>,
    #[doc = "How the run should be forced to rerun even if the run request configuration has not changed"]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
}
impl TaskRunPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for a task run request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskRunRequest {
    #[serde(flatten)]
    pub run_request: RunRequest,
    #[doc = "The resource ID of task against which run has to be queued."]
    #[serde(rename = "taskId")]
    pub task_id: String,
    #[serde(rename = "overrideTaskStepProperties", default, skip_serializing_if = "Option::is_none")]
    pub override_task_step_properties: Option<OverrideTaskStepProperties>,
}
impl TaskRunRequest {
    pub fn new(run_request: RunRequest, task_id: String) -> Self {
        Self {
            run_request,
            task_id,
            override_task_step_properties: None,
        }
    }
}
#[doc = "The parameters for updating a task run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskRunUpdateParameters {
    #[doc = "Managed identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The properties of a task run update parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskRunPropertiesUpdateParameters>,
    #[doc = "The location of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The ARM resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TaskRunUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base properties for any task step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskStepProperties {
    #[doc = "The type of the step."]
    #[serde(rename = "type")]
    pub type_: task_step_properties::Type,
    #[doc = "List of base image dependencies for a step."]
    #[serde(rename = "baseImageDependencies", default, skip_serializing_if = "Vec::is_empty")]
    pub base_image_dependencies: Vec<BaseImageDependency>,
    #[doc = "The URL(absolute or relative) of the source context for the task step."]
    #[serde(rename = "contextPath", default, skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[doc = "The token (git PAT or SAS token of storage account blob) associated with the context for a step."]
    #[serde(rename = "contextAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub context_access_token: Option<String>,
}
impl TaskStepProperties {
    pub fn new(type_: task_step_properties::Type) -> Self {
        Self {
            type_,
            base_image_dependencies: Vec::new(),
            context_path: None,
            context_access_token: None,
        }
    }
}
pub mod task_step_properties {
    use super::*;
    #[doc = "The type of the step."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Docker,
        FileTask,
        EncodedTask,
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
                Self::Docker => serializer.serialize_unit_variant("Type", 0u32, "Docker"),
                Self::FileTask => serializer.serialize_unit_variant("Type", 1u32, "FileTask"),
                Self::EncodedTask => serializer.serialize_unit_variant("Type", 2u32, "EncodedTask"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base properties for updating any task step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskStepUpdateParameters {
    #[doc = "The type of the step."]
    #[serde(rename = "type")]
    pub type_: task_step_update_parameters::Type,
    #[doc = "The URL(absolute or relative) of the source context for the task step."]
    #[serde(rename = "contextPath", default, skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[doc = "The token (git PAT or SAS token of storage account blob) associated with the context for a step."]
    #[serde(rename = "contextAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub context_access_token: Option<String>,
}
impl TaskStepUpdateParameters {
    pub fn new(type_: task_step_update_parameters::Type) -> Self {
        Self {
            type_,
            context_path: None,
            context_access_token: None,
        }
    }
}
pub mod task_step_update_parameters {
    use super::*;
    #[doc = "The type of the step."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Docker,
        FileTask,
        EncodedTask,
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
                Self::Docker => serializer.serialize_unit_variant("Type", 0u32, "Docker"),
                Self::FileTask => serializer.serialize_unit_variant("Type", 1u32, "FileTask"),
                Self::EncodedTask => serializer.serialize_unit_variant("Type", 2u32, "EncodedTask"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters for updating a task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskUpdateParameters {
    #[doc = "Managed identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The properties for updating a task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskPropertiesUpdateParameters>,
    #[doc = "The ARM resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TaskUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a timer trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimerTrigger {
    #[doc = "The CRON expression for the task schedule"]
    pub schedule: String,
    #[doc = "The current status of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<timer_trigger::Status>,
    #[doc = "The name of the trigger."]
    pub name: String,
}
impl TimerTrigger {
    pub fn new(schedule: String, name: String) -> Self {
        Self {
            schedule,
            status: None,
            name,
        }
    }
}
pub mod timer_trigger {
    use super::*;
    #[doc = "The current status of trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimerTriggerDescriptor {
    #[doc = "The timer trigger name that caused the run."]
    #[serde(rename = "timerTriggerName", default, skip_serializing_if = "Option::is_none")]
    pub timer_trigger_name: Option<String>,
    #[doc = "The occurrence that triggered the run."]
    #[serde(rename = "scheduleOccurrence", default, skip_serializing_if = "Option::is_none")]
    pub schedule_occurrence: Option<String>,
}
impl TimerTriggerDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for updating a timer trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimerTriggerUpdateParameters {
    #[doc = "The CRON expression for the task schedule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[doc = "The current status of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<timer_trigger_update_parameters::Status>,
    #[doc = "The name of the trigger."]
    pub name: String,
}
impl TimerTriggerUpdateParameters {
    pub fn new(name: String) -> Self {
        Self {
            schedule: None,
            status: None,
            name,
        }
    }
}
pub mod timer_trigger_update_parameters {
    use super::*;
    #[doc = "The current status of trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "The TLS certificate properties of the connected registry login server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TlsCertificateProperties {
    #[doc = "The type of certificate location."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<tls_certificate_properties::Type>,
    #[doc = "Indicates the location of the certificates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TlsCertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tls_certificate_properties {
    use super::*;
    #[doc = "The type of certificate location."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        LocalDirectory,
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
                Self::LocalDirectory => serializer.serialize_unit_variant("Type", 0u32, "LocalDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The TLS properties of the connected registry login server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TlsProperties {
    #[doc = "Indicates whether HTTPS is enabled for the login server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<tls_properties::Status>,
    #[doc = "The TLS certificate properties of the connected registry login server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<TlsCertificateProperties>,
}
impl TlsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tls_properties {
    use super::*;
    #[doc = "Indicates whether HTTPS is enabled for the login server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An object that represents a token for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Token {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TokenProperties>,
}
impl Token {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a certificate used for authenticating a token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenCertificate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<token_certificate::Name>,
    #[doc = "The expiry datetime of the certificate."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
    #[doc = "The thumbprint of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Base 64 encoded string of the public certificate1 in PEM format that will be used for authenticating the token."]
    #[serde(rename = "encodedPemCertificate", default, skip_serializing_if = "Option::is_none")]
    pub encoded_pem_certificate: Option<String>,
}
impl TokenCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod token_certificate {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "certificate1")]
        Certificate1,
        #[serde(rename = "certificate2")]
        Certificate2,
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
                Self::Certificate1 => serializer.serialize_unit_variant("Name", 0u32, "certificate1"),
                Self::Certificate2 => serializer.serialize_unit_variant("Name", 1u32, "certificate2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the credentials that can be used for authenticating the token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenCredentialsProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<TokenCertificate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<TokenPassword>,
}
impl TokenCredentialsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list tokens for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenListResult {
    #[doc = "The list of tokens. Since this list may be incomplete, the nextLink field should be used to request the next list of tokens."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Token>,
    #[doc = "The URI that can be used to request the next list of tokens."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TokenListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TokenListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The password that will be used for authenticating the token of a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenPassword {
    #[doc = "The creation datetime of the password."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The expiry datetime of the password."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
    #[doc = "The password name \"password1\" or \"password2\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<token_password::Name>,
    #[doc = "The password value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TokenPassword {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod token_password {
    use super::*;
    #[doc = "The password name \"password1\" or \"password2\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "password1")]
        Password1,
        #[serde(rename = "password2")]
        Password2,
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
                Self::Password1 => serializer.serialize_unit_variant("Name", 0u32, "password1"),
                Self::Password2 => serializer.serialize_unit_variant("Name", 1u32, "password2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenProperties {
    #[doc = "The creation date of scope map."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<token_properties::ProvisioningState>,
    #[doc = "The resource ID of the scope map to which the token will be associated with."]
    #[serde(rename = "scopeMapId", default, skip_serializing_if = "Option::is_none")]
    pub scope_map_id: Option<String>,
    #[doc = "The properties of the credentials that can be used for authenticating the token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TokenCredentialsProperties>,
    #[doc = "The status of the token example enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<token_properties::Status>,
}
impl TokenProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod token_properties {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the token example enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters for updating a token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenUpdateParameters {
    #[doc = "The parameters for updating token properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TokenUpdateProperties>,
}
impl TokenUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for updating token properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenUpdateProperties {
    #[doc = "The resource ID of the scope map to which the token will be associated with."]
    #[serde(rename = "scopeMapId", default, skip_serializing_if = "Option::is_none")]
    pub scope_map_id: Option<String>,
    #[doc = "The status of the token example enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<token_update_properties::Status>,
    #[doc = "The properties of the credentials that can be used for authenticating the token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TokenCredentialsProperties>,
}
impl TokenUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod token_update_properties {
    use super::*;
    #[doc = "The status of the token example enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerProperties {
    #[doc = "The collection of timer triggers."]
    #[serde(rename = "timerTriggers", default, skip_serializing_if = "Vec::is_empty")]
    pub timer_triggers: Vec<TimerTrigger>,
    #[doc = "The collection of triggers based on source code repository."]
    #[serde(rename = "sourceTriggers", default, skip_serializing_if = "Vec::is_empty")]
    pub source_triggers: Vec<SourceTrigger>,
    #[doc = "The trigger based on base image dependency."]
    #[serde(rename = "baseImageTrigger", default, skip_serializing_if = "Option::is_none")]
    pub base_image_trigger: Option<BaseImageTrigger>,
}
impl TriggerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for updating triggers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerUpdateParameters {
    #[doc = "The collection of timer triggers."]
    #[serde(rename = "timerTriggers", default, skip_serializing_if = "Vec::is_empty")]
    pub timer_triggers: Vec<TimerTriggerUpdateParameters>,
    #[doc = "The collection of triggers based on source code repository."]
    #[serde(rename = "sourceTriggers", default, skip_serializing_if = "Vec::is_empty")]
    pub source_triggers: Vec<SourceTriggerUpdateParameters>,
    #[doc = "The properties for updating base image dependency trigger."]
    #[serde(rename = "baseImageTrigger", default, skip_serializing_if = "Option::is_none")]
    pub base_image_trigger: Option<BaseImageTriggerUpdateParameters>,
}
impl TriggerUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The content trust policy for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustPolicy {
    #[doc = "The type of trust policy."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<trust_policy::Type>,
    #[doc = "The value that indicates whether the policy is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<trust_policy::Status>,
}
impl TrustPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod trust_policy {
    use super::*;
    #[doc = "The type of trust policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Notary,
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
                Self::Notary => serializer.serialize_unit_variant("Type", 0u32, "Notary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Notary
        }
    }
    #[doc = "The value that indicates whether the policy is enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
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
#[doc = "Virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[doc = "The action of virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<virtual_network_rule::Action>,
    #[doc = "Resource ID of a subnet, for example: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets/{subnetName}."]
    pub id: String,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self { action: None, id }
    }
}
pub mod virtual_network_rule {
    use super::*;
    #[doc = "The action of virtual network rule."]
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
    impl Default for Action {
        fn default() -> Self {
            Self::Allow
        }
    }
}
#[doc = "An object that represents a webhook for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookProperties>,
}
impl Webhook {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The parameters for creating a webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookCreateParameters {
    #[doc = "The tags for the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The location of the webhook. This cannot be changed after the resource is created."]
    pub location: String,
    #[doc = "The parameters for creating the properties of a webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookPropertiesCreateParameters>,
}
impl WebhookCreateParameters {
    pub fn new(location: String) -> Self {
        Self {
            tags: None,
            location,
            properties: None,
        }
    }
}
#[doc = "The result of a request to list webhooks for a container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookListResult {
    #[doc = "The list of webhooks. Since this list may be incomplete, the nextLink field should be used to request the next list of webhooks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Webhook>,
    #[doc = "The URI that can be used to request the next list of webhooks."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebhookListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WebhookListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookProperties {
    #[doc = "The status of the webhook at the time the operation was called."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<webhook_properties::Status>,
    #[doc = "The scope of repositories where the event can be triggered. For example, 'foo:*' means events for all tags under repository 'foo'. 'foo:bar' means events for 'foo:bar' only. 'foo' is equivalent to 'foo:latest'. Empty means all events."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The list of actions that trigger the webhook to post notifications."]
    pub actions: Vec<String>,
    #[doc = "The provisioning state of the webhook at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<webhook_properties::ProvisioningState>,
}
impl WebhookProperties {
    pub fn new(actions: Vec<String>) -> Self {
        Self {
            status: None,
            scope: None,
            actions,
            provisioning_state: None,
        }
    }
}
pub mod webhook_properties {
    use super::*;
    #[doc = "The status of the webhook at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the webhook at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters for creating the properties of a webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookPropertiesCreateParameters {
    #[doc = "The service URI for the webhook to post notifications."]
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
    #[doc = "Custom headers that will be added to the webhook notifications."]
    #[serde(rename = "customHeaders", default, skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<serde_json::Value>,
    #[doc = "The status of the webhook at the time the operation was called."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<webhook_properties_create_parameters::Status>,
    #[doc = "The scope of repositories where the event can be triggered. For example, 'foo:*' means events for all tags under repository 'foo'. 'foo:bar' means events for 'foo:bar' only. 'foo' is equivalent to 'foo:latest'. Empty means all events."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The list of actions that trigger the webhook to post notifications."]
    pub actions: Vec<String>,
}
impl WebhookPropertiesCreateParameters {
    pub fn new(service_uri: String, actions: Vec<String>) -> Self {
        Self {
            service_uri,
            custom_headers: None,
            status: None,
            scope: None,
            actions,
        }
    }
}
pub mod webhook_properties_create_parameters {
    use super::*;
    #[doc = "The status of the webhook at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters for updating the properties of a webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookPropertiesUpdateParameters {
    #[doc = "The service URI for the webhook to post notifications."]
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[doc = "Custom headers that will be added to the webhook notifications."]
    #[serde(rename = "customHeaders", default, skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<serde_json::Value>,
    #[doc = "The status of the webhook at the time the operation was called."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<webhook_properties_update_parameters::Status>,
    #[doc = "The scope of repositories where the event can be triggered. For example, 'foo:*' means events for all tags under repository 'foo'. 'foo:bar' means events for 'foo:bar' only. 'foo' is equivalent to 'foo:latest'. Empty means all events."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The list of actions that trigger the webhook to post notifications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
}
impl WebhookPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod webhook_properties_update_parameters {
    use super::*;
    #[doc = "The status of the webhook at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters for updating a webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookUpdateParameters {
    #[doc = "The tags for the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The parameters for updating the properties of a webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookPropertiesUpdateParameters>,
}
impl WebhookUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
