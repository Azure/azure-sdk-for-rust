#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure App Configuration error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The type of the error."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "A brief summary of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The name of the parameter that resulted in the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A detailed description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = "The HTTP status code that the error maps to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "An object containing specific information about an error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "An object containing specific information about an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing specific information about an error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<InnerError>>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Key {
    #[doc = "The name of the key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Key {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a list request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyListResult {
    #[doc = "The collection value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<Key>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl KeyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyValue {
    #[doc = "The key of the key-value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The label the key-value belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The content type of the value stored within the key-value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The value of the key-value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "A date representing the last time the key-value was modified."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "The tags of the key-value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Indicates whether the key-value is locked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[doc = "A value representing the current state of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl KeyValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enables filtering of key-values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValueFilter {
    #[doc = "Filters key-values by their key field."]
    pub key: String,
    #[doc = "Filters key-values by their label field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
impl KeyValueFilter {
    pub fn new(key: String) -> Self {
        Self { key, label: None }
    }
}
#[doc = "The result of a list request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyValueListResult {
    #[doc = "The collection value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<KeyValue>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KeyValueListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl KeyValueListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Label {
    #[doc = "The name of the label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Label {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a list request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabelListResult {
    #[doc = "The collection value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<Label>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LabelListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LabelListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a long running operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDetails {
    #[doc = "The unique id of the operation."]
    pub id: String,
    #[doc = "The current status of the operation"]
    pub status: operation_details::Status,
    #[doc = "The details of an error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationDetails {
    pub fn new(id: String, status: operation_details::Status) -> Self {
        Self { id, status, error: None }
    }
}
pub mod operation_details {
    use super::*;
    #[doc = "The current status of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        Running,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[doc = "The name of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The current status of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<snapshot::Status>,
    #[doc = "A list of filters used to filter the key-values included in the snapshot."]
    pub filters: Vec<KeyValueFilter>,
    #[doc = "The composition type describes how the key-values within the snapshot are composed. The 'key' composition type ensures there are no two key-values containing the same key. The 'key_label' composition type ensures there are no two key-values containing the same key and label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composition_type: Option<snapshot::CompositionType>,
    #[doc = "The time that the snapshot was created."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The time that the snapshot will expire."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expires: Option<time::OffsetDateTime>,
    #[doc = "The amount of time, in seconds, that a snapshot will remain in the archived state before expiring. This property is only writable during the creation of a snapshot. If not specified, the default lifetime of key-value revisions will be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i64>,
    #[doc = "The size in bytes of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The amount of key-values in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items_count: Option<i64>,
    #[doc = "The tags of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "A value representing the current state of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Snapshot {
    pub fn new(filters: Vec<KeyValueFilter>) -> Self {
        Self {
            name: None,
            status: None,
            filters,
            composition_type: None,
            created: None,
            expires: None,
            retention_period: None,
            size: None,
            items_count: None,
            tags: None,
            etag: None,
        }
    }
}
pub mod snapshot {
    use super::*;
    #[doc = "The current status of the snapshot."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "archived")]
        Archived,
        #[serde(rename = "failed")]
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
                Self::Provisioning => serializer.serialize_unit_variant("Status", 0u32, "provisioning"),
                Self::Ready => serializer.serialize_unit_variant("Status", 1u32, "ready"),
                Self::Archived => serializer.serialize_unit_variant("Status", 2u32, "archived"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The composition type describes how the key-values within the snapshot are composed. The 'key' composition type ensures there are no two key-values containing the same key. The 'key_label' composition type ensures there are no two key-values containing the same key and label."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CompositionType")]
    pub enum CompositionType {
        #[serde(rename = "key")]
        Key,
        #[serde(rename = "key_label")]
        KeyLabel,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CompositionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CompositionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CompositionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Key => serializer.serialize_unit_variant("CompositionType", 0u32, "key"),
                Self::KeyLabel => serializer.serialize_unit_variant("CompositionType", 1u32, "key_label"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result of a snapshot list request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotListResult {
    #[doc = "The collection value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<Snapshot>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SnapshotListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SnapshotListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters used to update a snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotUpdateParameters {
    #[doc = "The desired status of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<snapshot_update_parameters::Status>,
}
impl SnapshotUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod snapshot_update_parameters {
    use super::*;
    #[doc = "The desired status of the snapshot."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "archived")]
        Archived,
        #[serde(rename = "failed")]
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
                Self::Provisioning => serializer.serialize_unit_variant("Status", 0u32, "provisioning"),
                Self::Ready => serializer.serialize_unit_variant("Status", 1u32, "ready"),
                Self::Archived => serializer.serialize_unit_variant("Status", 2u32, "archived"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
