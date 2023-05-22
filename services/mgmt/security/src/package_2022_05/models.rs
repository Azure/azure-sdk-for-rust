#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The alert sync setting properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertSyncSettingProperties {
    #[doc = "Is the alert sync setting enabled"]
    pub enabled: bool,
}
impl AlertSyncSettingProperties {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[doc = "Represents an alert sync setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertSyncSettings {
    #[serde(flatten)]
    pub setting: Setting,
    #[doc = "The alert sync setting properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertSyncSettingProperties>,
}
impl AlertSyncSettings {
    pub fn new(setting: Setting) -> Self {
        Self { setting, properties: None }
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
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
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
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
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data export setting properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSettingProperties {
    #[doc = "Is the data export setting enabled"]
    pub enabled: bool,
}
impl DataExportSettingProperties {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[doc = "Represents a data export setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSettings {
    #[serde(flatten)]
    pub setting: Setting,
    #[doc = "The data export setting properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataExportSettingProperties>,
}
impl DataExportSettings {
    pub fn new(setting: Setting) -> Self {
        Self { setting, properties: None }
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
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the security setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "the kind of the settings string"]
    pub kind: setting::Kind,
}
impl Setting {
    pub fn new(kind: setting::Kind) -> Self {
        Self {
            resource: Resource::default(),
            kind,
        }
    }
}
pub mod setting {
    use super::*;
    #[doc = "the kind of the settings string"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        DataExportSettings,
        AlertSuppressionSetting,
        AlertSyncSettings,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DataExportSettings => serializer.serialize_unit_variant("Kind", 0u32, "DataExportSettings"),
                Self::AlertSuppressionSetting => serializer.serialize_unit_variant("Kind", 1u32, "AlertSuppressionSetting"),
                Self::AlertSyncSettings => serializer.serialize_unit_variant("Kind", 2u32, "AlertSyncSettings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Subscription settings list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingsList {
    #[doc = "The settings list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Setting>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SettingsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SettingsList {
    pub fn new() -> Self {
        Self::default()
    }
}
