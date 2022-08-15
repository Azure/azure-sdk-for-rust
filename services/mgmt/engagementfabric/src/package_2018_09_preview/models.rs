#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The EngagementFabric account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
}
impl Account {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self { tracked_resource }
    }
}
#[doc = "The list of the EngagementFabric accounts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountList {
    #[doc = "EngagementFabric accounts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
}
impl azure_core::Continuable for AccountList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The patch of EngagementFabric account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPatch {
    #[doc = "The tags of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AccountPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The EngagementFabric channel"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Channel {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "The EngagementFabric channel properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChannelProperties>,
}
impl Channel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of the EngagementFabric channels"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelList {
    #[doc = "EngagementFabric channels"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Channel>,
}
impl azure_core::Continuable for ChannelList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ChannelList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The EngagementFabric channel properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelProperties {
    #[doc = "The channel type"]
    #[serde(rename = "channelType")]
    pub channel_type: String,
    #[doc = "The functions to be enabled for the channel"]
    #[serde(rename = "channelFunctions", default, skip_serializing_if = "Vec::is_empty")]
    pub channel_functions: Vec<String>,
    #[doc = "The channel credentials"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,
}
impl ChannelProperties {
    pub fn new(channel_type: String) -> Self {
        Self {
            channel_type,
            channel_functions: Vec::new(),
            credentials: None,
        }
    }
}
#[doc = "EngagementFabric channel description"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelTypeDescription {
    #[doc = "Channel type"]
    #[serde(rename = "channelType", default, skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[doc = "Text description for the channel"]
    #[serde(rename = "channelDescription", default, skip_serializing_if = "Option::is_none")]
    pub channel_description: Option<String>,
    #[doc = "All the available functions for the channel"]
    #[serde(rename = "channelFunctions", default, skip_serializing_if = "Vec::is_empty")]
    pub channel_functions: Vec<String>,
}
impl ChannelTypeDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of the EngagementFabric channel descriptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelTypeDescriptionList {
    #[doc = "Channel descriptions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ChannelTypeDescription>,
}
impl ChannelTypeDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameter for name availability check"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameter {
    #[doc = "The name to be checked"]
    pub name: String,
    #[doc = "The fully qualified resource type for the name to be checked"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl CheckNameAvailabilityParameter {
    pub fn new(name: String, type_: String) -> Self {
        Self { name, type_ }
    }
}
#[doc = "The result of name availability check"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "The name to be checked"]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason of name availability result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<CheckNameUnavailableReason>,
    #[doc = "The message if name is unavailable"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reason of name availability result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CheckNameUnavailableReason")]
pub enum CheckNameUnavailableReason {
    Invalid,
    AlreadyExists,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CheckNameUnavailableReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CheckNameUnavailableReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CheckNameUnavailableReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("CheckNameUnavailableReason", 0u32, "Invalid"),
            Self::AlreadyExists => serializer.serialize_unit_variant("CheckNameUnavailableReason", 1u32, "AlreadyExists"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The default error response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Content of the default error response"]
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
#[doc = "Content of the default error response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "The error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The list of additional details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The description of the EngagementFabric account key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyDescription {
    #[doc = "The name of the key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The rank of the EngagementFabric account key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<KeyRank>,
    #[doc = "The value of the key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl KeyDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of the EngagementFabric account keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyDescriptionList {
    #[doc = "Account keys"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<KeyDescription>,
}
impl azure_core::Continuable for KeyDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl KeyDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rank of the EngagementFabric account key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KeyRank")]
pub enum KeyRank {
    PrimaryKey,
    SecondaryKey,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KeyRank {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KeyRank {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KeyRank {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PrimaryKey => serializer.serialize_unit_variant("KeyRank", 0u32, "PrimaryKey"),
            Self::SecondaryKey => serializer.serialize_unit_variant("KeyRank", 1u32, "SecondaryKey"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The EngagementFabric operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the EngagementFabric operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display information of the EngagementFabric operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The display information of the EngagementFabric operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The resource provider namespace of the EngagementFabric operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource type of the EngagementFabric operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The name of the EngagementFabric operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The description of the EngagementFabric operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of the EngagementFabric operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "The EngagementFabric operations"]
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
#[doc = "The base model for the proxy-only Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyOnlyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyOnlyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameter to regenerate single EngagementFabric account key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeyParameter {
    #[doc = "The name of key to be regenerated"]
    pub name: String,
    #[doc = "The rank of the EngagementFabric account key"]
    pub rank: KeyRank,
}
impl RegenerateKeyParameter {
    pub fn new(name: String, rank: KeyRank) -> Self {
        Self { name, rank }
    }
}
#[doc = "The base model for Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The ID of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The fully qualified type of the resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The EngagementFabric SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU"]
    pub name: String,
    #[doc = "The price tier of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
    }
}
#[doc = "The EngagementFabric SKU description of given resource type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDescription {
    #[doc = "The fully qualified resource type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The price tier of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The set of locations that the SKU is available"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Locations and zones"]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<SkuLocationInfoItem>,
    #[doc = "The restrictions because of which SKU cannot be used"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<serde_json::Value>,
}
impl SkuDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of the EngagementFabric SKU descriptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDescriptionList {
    #[doc = "SKU descriptions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDescription>,
}
impl azure_core::Continuable for SkuDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SkuDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Locations and zones info for SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuLocationInfoItem {
    #[doc = "The available location of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The available zone of the SKU"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl SkuLocationInfoItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base model for the tracked Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The location of the resource"]
    pub location: String,
    #[doc = "The tags of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The EngagementFabric SKU"]
    pub sku: Sku,
}
impl TrackedResource {
    pub fn new(location: String, sku: Sku) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
            sku,
        }
    }
}
