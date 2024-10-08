#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsError {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AzureCoreFoundationsError>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<AzureCoreFoundationsInnerError>,
}
impl AzureCoreFoundationsError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreFoundationsInnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AzureCoreFoundationsInnerError>>,
}
impl AzureCoreFoundationsInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum describing allowed operation states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureCoreFoundationsOperationState")]
pub enum AzureCoreFoundationsOperationState {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureCoreFoundationsOperationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureCoreFoundationsOperationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureCoreFoundationsOperationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 0u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 1u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 3u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 4u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Composition types."]
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
    pub status: Option<i32>,
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
#[doc = "Keys serve as identifiers for key-values and are used to store and retrieve corresponding values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Key {
    #[doc = "The name of the key."]
    pub name: String,
}
impl Key {
    pub fn new(name: String) -> Self {
        Self { name }
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
#[doc = "A key-value pair representing application settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    #[doc = "The key of the key-value."]
    pub key: String,
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
    pub last_modified: Option<::time::OffsetDateTime>,
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
    pub fn new(key: String) -> Self {
        Self {
            key,
            label: None,
            content_type: None,
            value: None,
            last_modified: None,
            tags: None,
            locked: None,
            etag: None,
        }
    }
}
#[doc = "Key-value fields."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KeyValueFields")]
pub enum KeyValueFields {
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "content_type")]
    ContentType,
    #[serde(rename = "value")]
    Value,
    #[serde(rename = "last_modified")]
    LastModified,
    #[serde(rename = "tags")]
    Tags,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "etag")]
    Etag,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KeyValueFields {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KeyValueFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KeyValueFields {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Key => serializer.serialize_unit_variant("KeyValueFields", 0u32, "key"),
            Self::Label => serializer.serialize_unit_variant("KeyValueFields", 1u32, "label"),
            Self::ContentType => serializer.serialize_unit_variant("KeyValueFields", 2u32, "content_type"),
            Self::Value => serializer.serialize_unit_variant("KeyValueFields", 3u32, "value"),
            Self::LastModified => serializer.serialize_unit_variant("KeyValueFields", 4u32, "last_modified"),
            Self::Tags => serializer.serialize_unit_variant("KeyValueFields", 5u32, "tags"),
            Self::Locked => serializer.serialize_unit_variant("KeyValueFields", 6u32, "locked"),
            Self::Etag => serializer.serialize_unit_variant("KeyValueFields", 7u32, "etag"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enables filtering of key-values. Syntax reference:\nhttps://aka.ms/azconfig/docs/restapisnapshots"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValueFilter {
    #[doc = "Filters key-values by their key field."]
    pub key: String,
    #[doc = "Filters key-values by their label field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Filters key-values by their tags field."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl KeyValueFilter {
    pub fn new(key: String) -> Self {
        Self {
            key,
            label: None,
            tags: Vec::new(),
        }
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
    #[doc = "An identifier representing the returned state of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
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
#[doc = "Labels are used to group key-values."]
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
#[doc = "Label fields."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LabelFields")]
pub enum LabelFields {
    #[serde(rename = "name")]
    Name,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LabelFields {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LabelFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LabelFields {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Name => serializer.serialize_unit_variant("LabelFields", 0u32, "name"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "Enum describing allowed operation states."]
    pub status: AzureCoreFoundationsOperationState,
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureCoreFoundationsError>,
}
impl OperationDetails {
    pub fn new(id: String, status: AzureCoreFoundationsOperationState) -> Self {
        Self { id, status, error: None }
    }
}
#[doc = "A snapshot is a named, immutable subset of an App Configuration store's key-values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[doc = "The name of the snapshot."]
    pub name: String,
    #[doc = "Snapshot status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SnapshotStatus>,
    #[doc = "A list of filters used to filter the key-values included in the snapshot."]
    pub filters: Vec<KeyValueFilter>,
    #[doc = "Composition types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composition_type: Option<CompositionType>,
    #[doc = "The time that the snapshot was created."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<::time::OffsetDateTime>,
    #[doc = "The time that the snapshot will expire."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expires: Option<::time::OffsetDateTime>,
    #[doc = "The amount of time, in seconds, that a snapshot will remain in the archived\nstate before expiring. This property is only writable during the creation of a\nsnapshot. If not specified, the default lifetime of key-value revisions will be\nused."]
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
    pub fn new(name: String, filters: Vec<KeyValueFilter>) -> Self {
        Self {
            name,
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
#[doc = "Snapshot fields."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SnapshotFields")]
pub enum SnapshotFields {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "filters")]
    Filters,
    #[serde(rename = "composition_type")]
    CompositionType,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "expires")]
    Expires,
    #[serde(rename = "retention_period")]
    RetentionPeriod,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "items_count")]
    ItemsCount,
    #[serde(rename = "tags")]
    Tags,
    #[serde(rename = "etag")]
    Etag,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SnapshotFields {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SnapshotFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SnapshotFields {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Name => serializer.serialize_unit_variant("SnapshotFields", 0u32, "name"),
            Self::Status => serializer.serialize_unit_variant("SnapshotFields", 1u32, "status"),
            Self::Filters => serializer.serialize_unit_variant("SnapshotFields", 2u32, "filters"),
            Self::CompositionType => serializer.serialize_unit_variant("SnapshotFields", 3u32, "composition_type"),
            Self::Created => serializer.serialize_unit_variant("SnapshotFields", 4u32, "created"),
            Self::Expires => serializer.serialize_unit_variant("SnapshotFields", 5u32, "expires"),
            Self::RetentionPeriod => serializer.serialize_unit_variant("SnapshotFields", 6u32, "retention_period"),
            Self::Size => serializer.serialize_unit_variant("SnapshotFields", 7u32, "size"),
            Self::ItemsCount => serializer.serialize_unit_variant("SnapshotFields", 8u32, "items_count"),
            Self::Tags => serializer.serialize_unit_variant("SnapshotFields", 9u32, "tags"),
            Self::Etag => serializer.serialize_unit_variant("SnapshotFields", 10u32, "etag"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Snapshot status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SnapshotStatus")]
pub enum SnapshotStatus {
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
impl FromStr for SnapshotStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SnapshotStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SnapshotStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provisioning => serializer.serialize_unit_variant("SnapshotStatus", 0u32, "provisioning"),
            Self::Ready => serializer.serialize_unit_variant("SnapshotStatus", 1u32, "ready"),
            Self::Archived => serializer.serialize_unit_variant("SnapshotStatus", 2u32, "archived"),
            Self::Failed => serializer.serialize_unit_variant("SnapshotStatus", 3u32, "failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Parameters used to update a snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotUpdateParameters {
    #[doc = "Snapshot status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SnapshotStatus>,
}
impl SnapshotUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
