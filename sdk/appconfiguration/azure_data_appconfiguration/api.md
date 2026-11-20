# azure_data_appconfiguration

- **Description**: Microsoft Azure App Configuration client library for Rust
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
#![crate_name = "azure_data_appconfiguration"]
#![crate_type = "lib"]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub struct AppConfigurationClient {
}
impl AppConfigurationClient {
    pub fn begin_create_snapshot(&self, content_type: CreateSnapshotRequestContentType, name: &str, entity: RequestContent<Snapshot>, options: Option<AppConfigurationClientBeginCreateSnapshotOptions<'_>>) -> Result<Poller<AppConfigurationClientBeginCreateSnapshotOperationStatus>>;
    pub async fn check_key_value(&self, key: &str, options: Option<AppConfigurationClientCheckKeyValueOptions<'_>>) -> Result<Response<AppConfigurationClientCheckKeyValueResult, NoFormat>>;
    pub async fn check_key_values(&self, options: Option<AppConfigurationClientCheckKeyValuesOptions<'_>>) -> Result<Response<AppConfigurationClientCheckKeyValuesResult, NoFormat>>;
    pub async fn check_keys(&self, options: Option<AppConfigurationClientCheckKeysOptions<'_>>) -> Result<Response<AppConfigurationClientCheckKeysResult, NoFormat>>;
    pub async fn check_labels(&self, options: Option<AppConfigurationClientCheckLabelsOptions<'_>>) -> Result<Response<AppConfigurationClientCheckLabelsResult, NoFormat>>;
    pub async fn check_revisions(&self, options: Option<AppConfigurationClientCheckRevisionsOptions<'_>>) -> Result<Response<AppConfigurationClientCheckRevisionsResult, NoFormat>>;
    pub async fn check_snapshot(&self, name: &str, options: Option<AppConfigurationClientCheckSnapshotOptions<'_>>) -> Result<Response<AppConfigurationClientCheckSnapshotResult, NoFormat>>;
    pub async fn check_snapshots(&self, options: Option<AppConfigurationClientCheckSnapshotsOptions<'_>>) -> Result<Response<AppConfigurationClientCheckSnapshotsResult, NoFormat>>;
    pub async fn delete_key_value(&self, key: &str, options: Option<AppConfigurationClientDeleteKeyValueOptions<'_>>) -> Result<Response<KeyValue>>;
    pub async fn delete_lock(&self, key: &str, options: Option<AppConfigurationClientDeleteLockOptions<'_>>) -> Result<Response<KeyValue>>;
    pub fn endpoint(&self) -> &Url;
    pub async fn get_key_value(&self, key: &str, options: Option<AppConfigurationClientGetKeyValueOptions<'_>>) -> Result<Response<KeyValue>>;
    pub async fn get_operation_details(&self, snapshot: &str, options: Option<AppConfigurationClientGetOperationDetailsOptions<'_>>) -> Result<Response<OperationDetails>>;
    pub async fn get_snapshot(&self, name: &str, options: Option<AppConfigurationClientGetSnapshotOptions<'_>>) -> Result<Response<Snapshot>>;
    pub fn list_key_values(&self, options: Option<AppConfigurationClientListKeyValuesOptions<'_>>) -> Result<Pager<KeyValueListResult>>;
    pub fn list_keys(&self, options: Option<AppConfigurationClientListKeysOptions<'_>>) -> Result<Pager<KeyListResult>>;
    pub fn list_labels(&self, options: Option<AppConfigurationClientListLabelsOptions<'_>>) -> Result<Pager<LabelListResult>>;
    pub fn list_revisions(&self, options: Option<AppConfigurationClientListRevisionsOptions<'_>>) -> Result<Pager<KeyValueListResult>>;
    pub fn list_snapshots(&self, options: Option<AppConfigurationClientListSnapshotsOptions<'_>>) -> Result<Pager<SnapshotListResult>>;
    pub fn new(endpoint: &str, credential: Arc<dyn TokenCredential>, options: Option<AppConfigurationClientOptions>) -> Result<Self>;
    pub async fn put_key_value(&self, content_type: PutKeyValueRequestContentType, key: &str, options: Option<AppConfigurationClientPutKeyValueOptions<'_>>) -> Result<Response<KeyValue>>;
    pub async fn put_lock(&self, key: &str, options: Option<AppConfigurationClientPutLockOptions<'_>>) -> Result<Response<KeyValue>>;
    pub async fn update_snapshot(&self, content_type: UpdateSnapshotRequestContentType, name: &str, entity: RequestContent<SnapshotUpdateParameters>, options: Option<AppConfigurationClientUpdateSnapshotOptions<'_>>) -> Result<Response<Snapshot>>;
}
#[derive(Clone, Debug)]
pub struct AppConfigurationClientOptions {
    pub api_version: String,
    pub client_options: azure_core::http::ClientOptions,
}
impl Default for AppConfigurationClientOptions {
    fn default() -> Self;
}
pub mod clients {
    pub struct AppConfigurationClient {
    }
    impl AppConfigurationClient {
        pub fn begin_create_snapshot(&self, content_type: CreateSnapshotRequestContentType, name: &str, entity: RequestContent<Snapshot>, options: Option<AppConfigurationClientBeginCreateSnapshotOptions<'_>>) -> Result<Poller<AppConfigurationClientBeginCreateSnapshotOperationStatus>>;
        pub async fn check_key_value(&self, key: &str, options: Option<AppConfigurationClientCheckKeyValueOptions<'_>>) -> Result<Response<AppConfigurationClientCheckKeyValueResult, NoFormat>>;
        pub async fn check_key_values(&self, options: Option<AppConfigurationClientCheckKeyValuesOptions<'_>>) -> Result<Response<AppConfigurationClientCheckKeyValuesResult, NoFormat>>;
        pub async fn check_keys(&self, options: Option<AppConfigurationClientCheckKeysOptions<'_>>) -> Result<Response<AppConfigurationClientCheckKeysResult, NoFormat>>;
        pub async fn check_labels(&self, options: Option<AppConfigurationClientCheckLabelsOptions<'_>>) -> Result<Response<AppConfigurationClientCheckLabelsResult, NoFormat>>;
        pub async fn check_revisions(&self, options: Option<AppConfigurationClientCheckRevisionsOptions<'_>>) -> Result<Response<AppConfigurationClientCheckRevisionsResult, NoFormat>>;
        pub async fn check_snapshot(&self, name: &str, options: Option<AppConfigurationClientCheckSnapshotOptions<'_>>) -> Result<Response<AppConfigurationClientCheckSnapshotResult, NoFormat>>;
        pub async fn check_snapshots(&self, options: Option<AppConfigurationClientCheckSnapshotsOptions<'_>>) -> Result<Response<AppConfigurationClientCheckSnapshotsResult, NoFormat>>;
        pub async fn delete_key_value(&self, key: &str, options: Option<AppConfigurationClientDeleteKeyValueOptions<'_>>) -> Result<Response<KeyValue>>;
        pub async fn delete_lock(&self, key: &str, options: Option<AppConfigurationClientDeleteLockOptions<'_>>) -> Result<Response<KeyValue>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get_key_value(&self, key: &str, options: Option<AppConfigurationClientGetKeyValueOptions<'_>>) -> Result<Response<KeyValue>>;
        pub async fn get_operation_details(&self, snapshot: &str, options: Option<AppConfigurationClientGetOperationDetailsOptions<'_>>) -> Result<Response<OperationDetails>>;
        pub async fn get_snapshot(&self, name: &str, options: Option<AppConfigurationClientGetSnapshotOptions<'_>>) -> Result<Response<Snapshot>>;
        pub fn list_key_values(&self, options: Option<AppConfigurationClientListKeyValuesOptions<'_>>) -> Result<Pager<KeyValueListResult>>;
        pub fn list_keys(&self, options: Option<AppConfigurationClientListKeysOptions<'_>>) -> Result<Pager<KeyListResult>>;
        pub fn list_labels(&self, options: Option<AppConfigurationClientListLabelsOptions<'_>>) -> Result<Pager<LabelListResult>>;
        pub fn list_revisions(&self, options: Option<AppConfigurationClientListRevisionsOptions<'_>>) -> Result<Pager<KeyValueListResult>>;
        pub fn list_snapshots(&self, options: Option<AppConfigurationClientListSnapshotsOptions<'_>>) -> Result<Pager<SnapshotListResult>>;
        pub fn new(endpoint: &str, credential: Arc<dyn TokenCredential>, options: Option<AppConfigurationClientOptions>) -> Result<Self>;
        pub async fn put_key_value(&self, content_type: PutKeyValueRequestContentType, key: &str, options: Option<AppConfigurationClientPutKeyValueOptions<'_>>) -> Result<Response<KeyValue>>;
        pub async fn put_lock(&self, key: &str, options: Option<AppConfigurationClientPutLockOptions<'_>>) -> Result<Response<KeyValue>>;
        pub async fn update_snapshot(&self, content_type: UpdateSnapshotRequestContentType, name: &str, entity: RequestContent<SnapshotUpdateParameters>, options: Option<AppConfigurationClientUpdateSnapshotOptions<'_>>) -> Result<Response<Snapshot>>;
    }
    #[derive(Clone, Debug)]
    pub struct AppConfigurationClientOptions {
        pub api_version: String,
        pub client_options: azure_core::http::ClientOptions,
    }
    impl Default for AppConfigurationClientOptions {
        fn default() -> Self;
    }
}
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct AppConfigurationClientBeginCreateSnapshotOperationStatus {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<azure_core::error::ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::OperationState>,
    }
    impl StatusMonitor for super::AppConfigurationClientBeginCreateSnapshotOperationStatus {
        type Format = JsonFormat;
        type Output = Snapshot;
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientBeginCreateSnapshotOptions<'a> {
        pub method_options: azure_core::http::poller::PollerOptions<'a>,
        pub sync_token: Option<String>,
    }
    impl AppConfigurationClientBeginCreateSnapshotOptions<'_> {
        pub fn into_owned(self) -> AppConfigurationClientBeginCreateSnapshotOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientCheckKeyValueOptions<'a> {
        pub accept_datetime: Option<String>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::KeyValueFields>>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    #[derive(Debug)]
    pub struct AppConfigurationClientCheckKeyValueResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientCheckKeyValuesOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub key: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::KeyValueFields>>,
        pub snapshot: Option<String>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    #[derive(Debug)]
    pub struct AppConfigurationClientCheckKeyValuesResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientCheckKeysOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub name: Option<String>,
        pub sync_token: Option<String>,
    }
    #[derive(Debug)]
    pub struct AppConfigurationClientCheckKeysResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientCheckLabelsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub name: Option<String>,
        pub select: Option<Vec<super::LabelFields>>,
        pub sync_token: Option<String>,
    }
    #[derive(Debug)]
    pub struct AppConfigurationClientCheckLabelsResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientCheckRevisionsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub key: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::KeyValueFields>>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    #[derive(Debug)]
    pub struct AppConfigurationClientCheckRevisionsResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientCheckSnapshotOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Debug)]
    pub struct AppConfigurationClientCheckSnapshotResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientCheckSnapshotsOptions<'a> {
        pub after: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Debug)]
    pub struct AppConfigurationClientCheckSnapshotsResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientDeleteKeyValueOptions<'a> {
        pub if_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientDeleteLockOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientGetKeyValueOptions<'a> {
        pub accept_datetime: Option<String>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::KeyValueFields>>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientGetOperationDetailsOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientGetSnapshotOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::SnapshotFields>>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientListKeyValuesOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub key: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub select: Option<Vec<super::KeyValueFields>>,
        pub snapshot: Option<String>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    impl AppConfigurationClientListKeyValuesOptions<'_> {
        pub fn into_owned(self) -> AppConfigurationClientListKeyValuesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientListKeysOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub name: Option<String>,
        pub sync_token: Option<String>,
    }
    impl AppConfigurationClientListKeysOptions<'_> {
        pub fn into_owned(self) -> AppConfigurationClientListKeysOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientListLabelsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub name: Option<String>,
        pub select: Option<Vec<super::LabelFields>>,
        pub sync_token: Option<String>,
    }
    impl AppConfigurationClientListLabelsOptions<'_> {
        pub fn into_owned(self) -> AppConfigurationClientListLabelsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientListRevisionsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub key: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub select: Option<Vec<super::KeyValueFields>>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    impl AppConfigurationClientListRevisionsOptions<'_> {
        pub fn into_owned(self) -> AppConfigurationClientListRevisionsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientListSnapshotsOptions<'a> {
        pub after: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub name: Option<String>,
        pub select: Option<Vec<super::SnapshotFields>>,
        pub status: Option<Vec<super::SnapshotStatus>>,
        pub sync_token: Option<String>,
    }
    impl AppConfigurationClientListSnapshotsOptions<'_> {
        pub fn into_owned(self) -> AppConfigurationClientListSnapshotsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientPutKeyValueOptions<'a> {
        pub entity: Option<azure_core::http::RequestContent<super::KeyValue>>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientPutLockOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppConfigurationClientUpdateSnapshotOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub detail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Key {
        #[serde(skip_serializing)]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyListResult {
        #[serde(default)]
        pub items: Vec<Key>,
        #[serde(rename = "@nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    #[async_trait]
    impl Page for super::KeyListResult {
        type IntoIter = <Vec<Key> as IntoIterator>::IntoIter;
        type Item = Key;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyValue {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub etag: Option<String>,
        #[serde(skip_serializing)]
        pub key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc3339::option")]
        pub last_modified: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl TryFrom<KeyValue> for azure_core::http::RequestContent<super::KeyValue> {
        type Error = Error;
        fn try_from(value: KeyValue) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyValueFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyValueListResult {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub etag: Option<String>,
        #[serde(default)]
        pub items: Vec<KeyValue>,
        #[serde(rename = "@nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    #[async_trait]
    impl Page for super::KeyValueListResult {
        type IntoIter = <Vec<KeyValue> as IntoIterator>::IntoIter;
        type Item = KeyValue;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Label {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct LabelListResult {
        #[serde(default)]
        pub items: Vec<Label>,
        #[serde(rename = "@nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    #[async_trait]
    impl Page for super::LabelListResult {
        type IntoIter = <Vec<Label> as IntoIterator>::IntoIter;
        type Item = Label;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct OperationDetails {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<azure_core::error::ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::OperationState>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Snapshot {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub composition_type: Option<super::CompositionType>,
        #[serde(default, skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub created: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing)]
        pub etag: Option<String>,
        #[serde(default, skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub expires: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filters: Option<Vec<KeyValueFilter>>,
        #[serde(skip_serializing)]
        pub items_count: Option<i64>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub retention_period: Option<i64>,
        #[serde(skip_serializing)]
        pub size: Option<i64>,
        #[serde(skip_serializing)]
        pub status: Option<super::SnapshotStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<Snapshot> for azure_core::http::RequestContent<super::Snapshot> {
        type Error = Error;
        fn try_from(value: Snapshot) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct SnapshotListResult {
        #[serde(default)]
        pub items: Vec<Snapshot>,
        #[serde(rename = "@nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    #[async_trait]
    impl Page for super::SnapshotListResult {
        type IntoIter = <Vec<Snapshot> as IntoIterator>::IntoIter;
        type Item = Snapshot;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SnapshotUpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::SnapshotStatus>,
    }
    impl TryFrom<SnapshotUpdateParameters> for azure_core::http::RequestContent<super::SnapshotUpdateParameters> {
        type Error = Error;
        fn try_from(value: SnapshotUpdateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum CompositionType {
        Key,
        KeyLabel,
        UnknownValue(String),
    }
    impl AsRef<str> for super::CompositionType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::CompositionType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::CompositionType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::CompositionType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a CompositionType> for &'a str {
        fn from(e: &'a CompositionType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::CompositionType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CreateSnapshotRequestContentType {
        ApplicationJson,
        ApplicationVndMicrosoftAppconfigSnapshotJson,
    }
    impl AsRef<str> for super::CreateSnapshotRequestContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::CreateSnapshotRequestContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::CreateSnapshotRequestContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::CreateSnapshotRequestContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::CreateSnapshotRequestContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum GetKeyValueResponseContentType {
        ApplicationProblemJson,
        ApplicationVndMicrosoftAppconfigKvJson,
    }
    impl AsRef<str> for super::GetKeyValueResponseContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GetKeyValueResponseContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GetKeyValueResponseContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GetKeyValueResponseContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::GetKeyValueResponseContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum GetKeyValuesResponseContentType {
        ApplicationProblemJson,
        ApplicationVndMicrosoftAppconfigKvsetJson,
    }
    impl AsRef<str> for super::GetKeyValuesResponseContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GetKeyValuesResponseContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GetKeyValuesResponseContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GetKeyValuesResponseContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::GetKeyValuesResponseContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum GetKeysResponseContentType {
        ApplicationProblemJson,
        ApplicationVndMicrosoftAppconfigKeysetJson,
    }
    impl AsRef<str> for super::GetKeysResponseContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GetKeysResponseContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GetKeysResponseContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GetKeysResponseContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::GetKeysResponseContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum GetLabelsResponseContentType {
        ApplicationProblemJson,
        ApplicationVndMicrosoftAppconfigLabelsetJson,
    }
    impl AsRef<str> for super::GetLabelsResponseContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GetLabelsResponseContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GetLabelsResponseContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GetLabelsResponseContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::GetLabelsResponseContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum GetSnapshotResponseContentType {
        ApplicationProblemJson,
        ApplicationVndMicrosoftAppconfigSnapshotJson,
    }
    impl AsRef<str> for super::GetSnapshotResponseContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GetSnapshotResponseContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GetSnapshotResponseContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GetSnapshotResponseContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::GetSnapshotResponseContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum GetSnapshotsResponseContentType {
        ApplicationProblemJson,
        ApplicationVndMicrosoftAppconfigSnapshotsetJson,
    }
    impl AsRef<str> for super::GetSnapshotsResponseContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GetSnapshotsResponseContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GetSnapshotsResponseContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GetSnapshotsResponseContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::GetSnapshotsResponseContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum KeyValueFields {
        ContentType,
        Description,
        Etag,
        Key,
        Label,
        LastModified,
        Locked,
        Tags,
        Value,
        UnknownValue(String),
    }
    impl AsRef<str> for super::KeyValueFields {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::KeyValueFields {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::KeyValueFields {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::KeyValueFields {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a KeyValueFields> for &'a str {
        fn from(e: &'a KeyValueFields) -> Self;
    }
    impl<'de> Deserialize<'de> for super::KeyValueFields {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum LabelFields {
        Name,
        UnknownValue(String),
    }
    impl AsRef<str> for super::LabelFields {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::LabelFields {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::LabelFields {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::LabelFields {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a LabelFields> for &'a str {
        fn from(e: &'a LabelFields) -> Self;
    }
    impl<'de> Deserialize<'de> for super::LabelFields {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum OperationState {
        Canceled,
        Failed,
        NotStarted,
        Running,
        Succeeded,
        UnknownValue(String),
    }
    impl AsRef<str> for super::OperationState {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::OperationState {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::OperationState {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::OperationState {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a OperationState> for &'a str {
        fn from(e: &'a OperationState) -> Self;
    }
    impl<'de> Deserialize<'de> for super::OperationState {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PutKeyValueRequestContentType {
        ApplicationAnyJson,
        ApplicationJson,
        ApplicationJsonPatchJson,
        ApplicationVndMicrosoftAppconfigKvJson,
        ApplicationVndMicrosoftAppconfigKvsetJson,
        TextJson,
    }
    impl AsRef<str> for super::PutKeyValueRequestContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::PutKeyValueRequestContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::PutKeyValueRequestContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::PutKeyValueRequestContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::PutKeyValueRequestContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SnapshotFields {
        CompositionType,
        Created,
        Description,
        Etag,
        Expires,
        Filters,
        ItemsCount,
        Name,
        RetentionPeriod,
        Size,
        Status,
        Tags,
        UnknownValue(String),
    }
    impl AsRef<str> for super::SnapshotFields {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SnapshotFields {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SnapshotFields {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SnapshotFields {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a SnapshotFields> for &'a str {
        fn from(e: &'a SnapshotFields) -> Self;
    }
    impl<'de> Deserialize<'de> for super::SnapshotFields {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SnapshotStatus {
        Archived,
        Failed,
        Provisioning,
        Ready,
        UnknownValue(String),
    }
    impl AsRef<str> for super::SnapshotStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SnapshotStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SnapshotStatus {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SnapshotStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a SnapshotStatus> for &'a str {
        fn from(e: &'a SnapshotStatus) -> Self;
    }
    impl<'de> Deserialize<'de> for super::SnapshotStatus {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum UpdateSnapshotRequestContentType {
        ApplicationJson,
        ApplicationMergePatchJson,
    }
    impl AsRef<str> for super::UpdateSnapshotRequestContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::UpdateSnapshotRequestContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::UpdateSnapshotRequestContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::UpdateSnapshotRequestContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::UpdateSnapshotRequestContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    pub trait AppConfigurationClientBeginCreateSnapshotOperationStatusHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetSnapshotResponseContentType>>;
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn link(&self) -> Result<Option<String>>;
        fn operation_location(&self) -> Result<Option<String>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait AppConfigurationClientCheckKeyValueResultHeaders: private::Sealed {
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait AppConfigurationClientCheckKeyValuesResultHeaders: private::Sealed {
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait AppConfigurationClientCheckKeysResultHeaders: private::Sealed {
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait AppConfigurationClientCheckLabelsResultHeaders: private::Sealed {
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait AppConfigurationClientCheckRevisionsResultHeaders: private::Sealed {
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait AppConfigurationClientCheckSnapshotResultHeaders: private::Sealed {
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn link(&self) -> Result<Option<String>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait AppConfigurationClientCheckSnapshotsResultHeaders: private::Sealed {
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait KeyListResultHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetKeysResponseContentType>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait KeyValueHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetKeyValueResponseContentType>>;
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn request_id(&self) -> Result<Option<String>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait KeyValueListResultHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetKeyValuesResponseContentType>>;
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait LabelListResultHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetLabelsResponseContentType>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait SnapshotHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetSnapshotResponseContentType>>;
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn link(&self) -> Result<Option<String>>;
        fn request_id(&self) -> Result<Option<String>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait SnapshotListResultHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetSnapshotsResponseContentType>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
}
```
