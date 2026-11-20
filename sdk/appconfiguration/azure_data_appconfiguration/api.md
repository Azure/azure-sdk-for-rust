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
pub struct ConfigurationClient {
}
impl ConfigurationClient {
    pub fn begin_create_snapshot(&self, content_type: CreateSnapshotRequestContentType, name: &str, entity: RequestContent<ConfigurationSnapshot>, options: Option<ConfigurationClientBeginCreateSnapshotOptions<'_>>) -> Result<Poller<ConfigurationClientBeginCreateSnapshotOperationStatus>>;
    pub async fn check_configuration_setting(&self, key: &str, options: Option<ConfigurationClientCheckConfigurationSettingOptions<'_>>) -> Result<Response<ConfigurationClientCheckConfigurationSettingResult, NoFormat>>;
    pub async fn check_configuration_settings(&self, options: Option<ConfigurationClientCheckConfigurationSettingsOptions<'_>>) -> Result<Response<ConfigurationClientCheckConfigurationSettingsResult, NoFormat>>;
    pub async fn check_keys(&self, options: Option<ConfigurationClientCheckKeysOptions<'_>>) -> Result<Response<ConfigurationClientCheckKeysResult, NoFormat>>;
    pub async fn check_labels(&self, options: Option<ConfigurationClientCheckLabelsOptions<'_>>) -> Result<Response<ConfigurationClientCheckLabelsResult, NoFormat>>;
    pub async fn check_revisions(&self, options: Option<ConfigurationClientCheckRevisionsOptions<'_>>) -> Result<Response<ConfigurationClientCheckRevisionsResult, NoFormat>>;
    pub async fn check_snapshot(&self, name: &str, options: Option<ConfigurationClientCheckSnapshotOptions<'_>>) -> Result<Response<ConfigurationClientCheckSnapshotResult, NoFormat>>;
    pub async fn check_snapshots(&self, options: Option<ConfigurationClientCheckSnapshotsOptions<'_>>) -> Result<Response<ConfigurationClientCheckSnapshotsResult, NoFormat>>;
    pub async fn create_read_only_lock(&self, key: &str, options: Option<ConfigurationClientCreateReadOnlyLockOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
    pub async fn delete_configuration_setting(&self, key: &str, options: Option<ConfigurationClientDeleteConfigurationSettingOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
    pub async fn delete_read_only_lock(&self, key: &str, options: Option<ConfigurationClientDeleteReadOnlyLockOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
    pub fn endpoint(&self) -> &Url;
    pub async fn get_configuration_setting(&self, key: &str, options: Option<ConfigurationClientGetConfigurationSettingOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
    pub async fn get_operation_details(&self, snapshot: &str, options: Option<ConfigurationClientGetOperationDetailsOptions<'_>>) -> Result<Response<OperationDetails>>;
    pub async fn get_snapshot(&self, name: &str, options: Option<ConfigurationClientGetSnapshotOptions<'_>>) -> Result<Response<ConfigurationSnapshot>>;
    pub fn list_configuration_settings(&self, options: Option<ConfigurationClientListConfigurationSettingsOptions<'_>>) -> Result<Pager<ConfigurationSettingListResult>>;
    pub fn list_keys(&self, options: Option<ConfigurationClientListKeysOptions<'_>>) -> Result<Pager<KeyListResult>>;
    pub fn list_labels(&self, options: Option<ConfigurationClientListLabelsOptions<'_>>) -> Result<Pager<SettingLabelListResult>>;
    pub fn list_revisions(&self, options: Option<ConfigurationClientListRevisionsOptions<'_>>) -> Result<Pager<ConfigurationSettingListResult>>;
    pub fn list_snapshots(&self, options: Option<ConfigurationClientListSnapshotsOptions<'_>>) -> Result<Pager<SnapshotListResult>>;
    pub fn new(endpoint: &str, credential: Arc<dyn TokenCredential>, options: Option<ConfigurationClientOptions>) -> Result<Self>;
    pub async fn set_configuration_setting(&self, content_type: SetConfigurationSettingRequestContentType, key: &str, options: Option<ConfigurationClientSetConfigurationSettingOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
    pub async fn update_snapshot_status(&self, content_type: UpdateSnapshotRequestContentType, name: &str, entity: RequestContent<SnapshotUpdateParameters>, options: Option<ConfigurationClientUpdateSnapshotStatusOptions<'_>>) -> Result<Response<ConfigurationSnapshot>>;
}
#[derive(Clone, Debug)]
pub struct ConfigurationClientOptions {
    pub api_version: String,
    pub client_options: azure_core::http::ClientOptions,
}
impl Default for ConfigurationClientOptions {
    fn default() -> Self;
}
pub mod clients {
    pub struct ConfigurationClient {
    }
    impl ConfigurationClient {
        pub fn begin_create_snapshot(&self, content_type: CreateSnapshotRequestContentType, name: &str, entity: RequestContent<ConfigurationSnapshot>, options: Option<ConfigurationClientBeginCreateSnapshotOptions<'_>>) -> Result<Poller<ConfigurationClientBeginCreateSnapshotOperationStatus>>;
        pub async fn check_configuration_setting(&self, key: &str, options: Option<ConfigurationClientCheckConfigurationSettingOptions<'_>>) -> Result<Response<ConfigurationClientCheckConfigurationSettingResult, NoFormat>>;
        pub async fn check_configuration_settings(&self, options: Option<ConfigurationClientCheckConfigurationSettingsOptions<'_>>) -> Result<Response<ConfigurationClientCheckConfigurationSettingsResult, NoFormat>>;
        pub async fn check_keys(&self, options: Option<ConfigurationClientCheckKeysOptions<'_>>) -> Result<Response<ConfigurationClientCheckKeysResult, NoFormat>>;
        pub async fn check_labels(&self, options: Option<ConfigurationClientCheckLabelsOptions<'_>>) -> Result<Response<ConfigurationClientCheckLabelsResult, NoFormat>>;
        pub async fn check_revisions(&self, options: Option<ConfigurationClientCheckRevisionsOptions<'_>>) -> Result<Response<ConfigurationClientCheckRevisionsResult, NoFormat>>;
        pub async fn check_snapshot(&self, name: &str, options: Option<ConfigurationClientCheckSnapshotOptions<'_>>) -> Result<Response<ConfigurationClientCheckSnapshotResult, NoFormat>>;
        pub async fn check_snapshots(&self, options: Option<ConfigurationClientCheckSnapshotsOptions<'_>>) -> Result<Response<ConfigurationClientCheckSnapshotsResult, NoFormat>>;
        pub async fn create_read_only_lock(&self, key: &str, options: Option<ConfigurationClientCreateReadOnlyLockOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
        pub async fn delete_configuration_setting(&self, key: &str, options: Option<ConfigurationClientDeleteConfigurationSettingOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
        pub async fn delete_read_only_lock(&self, key: &str, options: Option<ConfigurationClientDeleteReadOnlyLockOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get_configuration_setting(&self, key: &str, options: Option<ConfigurationClientGetConfigurationSettingOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
        pub async fn get_operation_details(&self, snapshot: &str, options: Option<ConfigurationClientGetOperationDetailsOptions<'_>>) -> Result<Response<OperationDetails>>;
        pub async fn get_snapshot(&self, name: &str, options: Option<ConfigurationClientGetSnapshotOptions<'_>>) -> Result<Response<ConfigurationSnapshot>>;
        pub fn list_configuration_settings(&self, options: Option<ConfigurationClientListConfigurationSettingsOptions<'_>>) -> Result<Pager<ConfigurationSettingListResult>>;
        pub fn list_keys(&self, options: Option<ConfigurationClientListKeysOptions<'_>>) -> Result<Pager<KeyListResult>>;
        pub fn list_labels(&self, options: Option<ConfigurationClientListLabelsOptions<'_>>) -> Result<Pager<SettingLabelListResult>>;
        pub fn list_revisions(&self, options: Option<ConfigurationClientListRevisionsOptions<'_>>) -> Result<Pager<ConfigurationSettingListResult>>;
        pub fn list_snapshots(&self, options: Option<ConfigurationClientListSnapshotsOptions<'_>>) -> Result<Pager<SnapshotListResult>>;
        pub fn new(endpoint: &str, credential: Arc<dyn TokenCredential>, options: Option<ConfigurationClientOptions>) -> Result<Self>;
        pub async fn set_configuration_setting(&self, content_type: SetConfigurationSettingRequestContentType, key: &str, options: Option<ConfigurationClientSetConfigurationSettingOptions<'_>>) -> Result<Response<ConfigurationSetting>>;
        pub async fn update_snapshot_status(&self, content_type: UpdateSnapshotRequestContentType, name: &str, entity: RequestContent<SnapshotUpdateParameters>, options: Option<ConfigurationClientUpdateSnapshotStatusOptions<'_>>) -> Result<Response<ConfigurationSnapshot>>;
    }
    #[derive(Clone, Debug)]
    pub struct ConfigurationClientOptions {
        pub api_version: String,
        pub client_options: azure_core::http::ClientOptions,
    }
    impl Default for ConfigurationClientOptions {
        fn default() -> Self;
    }
}
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ConfigurationClientBeginCreateSnapshotOperationStatus {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<azure_core::error::ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::OperationState>,
    }
    impl StatusMonitor for super::ConfigurationClientBeginCreateSnapshotOperationStatus {
        type Format = JsonFormat;
        type Output = ConfigurationSnapshot;
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientBeginCreateSnapshotOptions<'a> {
        pub method_options: azure_core::http::poller::PollerOptions<'a>,
        pub sync_token: Option<String>,
    }
    impl ConfigurationClientBeginCreateSnapshotOptions<'_> {
        pub fn into_owned(self) -> ConfigurationClientBeginCreateSnapshotOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientCheckConfigurationSettingOptions<'a> {
        pub accept_datetime: Option<String>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::SettingFields>>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    #[derive(Debug)]
    pub struct ConfigurationClientCheckConfigurationSettingResult;
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientCheckConfigurationSettingsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub key: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::SettingFields>>,
        pub snapshot: Option<String>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    #[derive(Debug)]
    pub struct ConfigurationClientCheckConfigurationSettingsResult;
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientCheckKeysOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub name: Option<String>,
        pub sync_token: Option<String>,
    }
    #[derive(Debug)]
    pub struct ConfigurationClientCheckKeysResult;
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientCheckLabelsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub name: Option<String>,
        pub select: Option<Vec<super::SettingLabelFields>>,
        pub sync_token: Option<String>,
    }
    #[derive(Debug)]
    pub struct ConfigurationClientCheckLabelsResult;
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientCheckRevisionsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub key: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::SettingFields>>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    #[derive(Debug)]
    pub struct ConfigurationClientCheckRevisionsResult;
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientCheckSnapshotOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Debug)]
    pub struct ConfigurationClientCheckSnapshotResult;
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientCheckSnapshotsOptions<'a> {
        pub after: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Debug)]
    pub struct ConfigurationClientCheckSnapshotsResult;
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientCreateReadOnlyLockOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientDeleteConfigurationSettingOptions<'a> {
        pub if_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientDeleteReadOnlyLockOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientGetConfigurationSettingOptions<'a> {
        pub accept_datetime: Option<String>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::SettingFields>>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientGetOperationDetailsOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientGetSnapshotOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub select: Option<Vec<super::SnapshotFields>>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientListConfigurationSettingsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub key: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub select: Option<Vec<super::SettingFields>>,
        pub snapshot: Option<String>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    impl ConfigurationClientListConfigurationSettingsOptions<'_> {
        pub fn into_owned(self) -> ConfigurationClientListConfigurationSettingsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientListKeysOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub name: Option<String>,
        pub sync_token: Option<String>,
    }
    impl ConfigurationClientListKeysOptions<'_> {
        pub fn into_owned(self) -> ConfigurationClientListKeysOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientListLabelsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub name: Option<String>,
        pub select: Option<Vec<super::SettingLabelFields>>,
        pub sync_token: Option<String>,
    }
    impl ConfigurationClientListLabelsOptions<'_> {
        pub fn into_owned(self) -> ConfigurationClientListLabelsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientListRevisionsOptions<'a> {
        pub accept_datetime: Option<String>,
        pub after: Option<String>,
        pub key: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub select: Option<Vec<super::SettingFields>>,
        pub sync_token: Option<String>,
        pub tags: Option<Vec<String>>,
    }
    impl ConfigurationClientListRevisionsOptions<'_> {
        pub fn into_owned(self) -> ConfigurationClientListRevisionsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientListSnapshotsOptions<'a> {
        pub after: Option<String>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub name: Option<String>,
        pub select: Option<Vec<super::SnapshotFields>>,
        pub status: Option<Vec<super::ConfigurationSnapshotStatus>>,
        pub sync_token: Option<String>,
    }
    impl ConfigurationClientListSnapshotsOptions<'_> {
        pub fn into_owned(self) -> ConfigurationClientListSnapshotsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientSetConfigurationSettingOptions<'a> {
        pub entity: Option<azure_core::http::RequestContent<super::ConfigurationSetting>>,
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub label: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConfigurationClientUpdateSnapshotStatusOptions<'a> {
        pub if_match: Option<String>,
        pub if_none_match: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub sync_token: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ConfigurationSetting {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub etag: Option<String>,
        #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
        pub is_read_only: Option<bool>,
        #[serde(skip_serializing)]
        pub key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc3339::option")]
        pub last_modified: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl TryFrom<ConfigurationSetting> for azure_core::http::RequestContent<super::ConfigurationSetting> {
        type Error = Error;
        fn try_from(value: ConfigurationSetting) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ConfigurationSettingListResult {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub etag: Option<String>,
        #[serde(default)]
        pub items: Vec<ConfigurationSetting>,
        #[serde(rename = "@nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    #[async_trait]
    impl Page for super::ConfigurationSettingListResult {
        type IntoIter = <Vec<ConfigurationSetting> as IntoIterator>::IntoIter;
        type Item = ConfigurationSetting;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ConfigurationSettingsFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ConfigurationSnapshot {
        #[serde(default, rename = "created", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub created_on: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing)]
        pub etag: Option<String>,
        #[serde(default, rename = "expires", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub expires_on: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filters: Option<Vec<ConfigurationSettingsFilter>>,
        #[serde(rename = "items_count", skip_serializing)]
        pub item_count: Option<i64>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub retention_period: Option<i64>,
        #[serde(rename = "size", skip_serializing)]
        pub size_in_bytes: Option<i64>,
        #[serde(rename = "composition_type", skip_serializing_if = "Option::is_none")]
        pub snapshot_composition: Option<super::SnapshotComposition>,
        #[serde(skip_serializing)]
        pub status: Option<super::ConfigurationSnapshotStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<ConfigurationSnapshot> for azure_core::http::RequestContent<super::ConfigurationSnapshot> {
        type Error = Error;
        fn try_from(value: ConfigurationSnapshot) -> Result<Self>;
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
    #[non_exhaustive]
    pub struct SettingLabel {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct SettingLabelListResult {
        #[serde(default)]
        pub items: Vec<SettingLabel>,
        #[serde(rename = "@nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    #[async_trait]
    impl Page for super::SettingLabelListResult {
        type IntoIter = <Vec<SettingLabel> as IntoIterator>::IntoIter;
        type Item = SettingLabel;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct SnapshotListResult {
        #[serde(default)]
        pub items: Vec<ConfigurationSnapshot>,
        #[serde(rename = "@nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    #[async_trait]
    impl Page for super::SnapshotListResult {
        type IntoIter = <Vec<ConfigurationSnapshot> as IntoIterator>::IntoIter;
        type Item = ConfigurationSnapshot;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SnapshotUpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ConfigurationSnapshotStatus>,
    }
    impl TryFrom<SnapshotUpdateParameters> for azure_core::http::RequestContent<super::SnapshotUpdateParameters> {
        type Error = Error;
        fn try_from(value: SnapshotUpdateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ConfigurationSnapshotStatus {
        Archived,
        Failed,
        Provisioning,
        Ready,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ConfigurationSnapshotStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ConfigurationSnapshotStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ConfigurationSnapshotStatus {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ConfigurationSnapshotStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ConfigurationSnapshotStatus> for &'a str {
        fn from(e: &'a ConfigurationSnapshotStatus) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ConfigurationSnapshotStatus {
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
    pub enum SetConfigurationSettingRequestContentType {
        ApplicationAnyJson,
        ApplicationJson,
        ApplicationJsonPatchJson,
        ApplicationVndMicrosoftAppconfigKvJson,
        ApplicationVndMicrosoftAppconfigKvsetJson,
        TextJson,
    }
    impl AsRef<str> for super::SetConfigurationSettingRequestContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SetConfigurationSettingRequestContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SetConfigurationSettingRequestContentType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SetConfigurationSettingRequestContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::SetConfigurationSettingRequestContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SettingFields {
        ContentType,
        Description,
        Etag,
        IsReadOnly,
        Key,
        Label,
        LastModified,
        Tags,
        Value,
        UnknownValue(String),
    }
    impl AsRef<str> for super::SettingFields {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SettingFields {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SettingFields {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SettingFields {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a SettingFields> for &'a str {
        fn from(e: &'a SettingFields) -> Self;
    }
    impl<'de> Deserialize<'de> for super::SettingFields {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SettingLabelFields {
        Name,
        UnknownValue(String),
    }
    impl AsRef<str> for super::SettingLabelFields {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SettingLabelFields {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SettingLabelFields {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SettingLabelFields {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a SettingLabelFields> for &'a str {
        fn from(e: &'a SettingLabelFields) -> Self;
    }
    impl<'de> Deserialize<'de> for super::SettingLabelFields {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SnapshotComposition {
        Key,
        KeyLabel,
        UnknownValue(String),
    }
    impl AsRef<str> for super::SnapshotComposition {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SnapshotComposition {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SnapshotComposition {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SnapshotComposition {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a SnapshotComposition> for &'a str {
        fn from(e: &'a SnapshotComposition) -> Self;
    }
    impl<'de> Deserialize<'de> for super::SnapshotComposition {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SnapshotFields {
        CreatedOn,
        Description,
        Etag,
        ExpiresOn,
        Filters,
        ItemCount,
        Name,
        RetentionPeriod,
        SizeInBytes,
        SnapshotComposition,
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
    pub trait ConfigurationClientBeginCreateSnapshotOperationStatusHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetSnapshotResponseContentType>>;
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn link(&self) -> Result<Option<String>>;
        fn operation_location(&self) -> Result<Option<String>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationClientCheckConfigurationSettingResultHeaders: private::Sealed {
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationClientCheckConfigurationSettingsResultHeaders: private::Sealed {
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationClientCheckKeysResultHeaders: private::Sealed {
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationClientCheckLabelsResultHeaders: private::Sealed {
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationClientCheckRevisionsResultHeaders: private::Sealed {
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationClientCheckSnapshotResultHeaders: private::Sealed {
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn link(&self) -> Result<Option<String>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationClientCheckSnapshotsResultHeaders: private::Sealed {
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationSettingHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetKeyValueResponseContentType>>;
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn request_id(&self) -> Result<Option<String>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationSettingListResultHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetKeyValuesResponseContentType>>;
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait ConfigurationSnapshotHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetSnapshotResponseContentType>>;
        fn etag_header(&self) -> Result<Option<Etag>>;
        fn link(&self) -> Result<Option<String>>;
        fn request_id(&self) -> Result<Option<String>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait KeyListResultHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetKeysResponseContentType>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait SettingLabelListResultHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetLabelsResponseContentType>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
    pub trait SnapshotListResultHeaders: private::Sealed {
        fn content_type(&self) -> Result<Option<GetSnapshotsResponseContentType>>;
        fn sync_token(&self) -> Result<Option<String>>;
    }
}
```
