# azure_security_keyvault_secrets

- **Description**: Rust wrappers around Microsoft Azure REST APIs - Azure Key Vault Secrets
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
pub use azure_security_keyvault_secrets::generated::clients::secret_client::SecretClient;
pub use azure_security_keyvault_secrets::clients::SecretClientOptions;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceId {
    pub source_id: String,
    pub vault_url: String,
    pub name: String,
    pub version: Option<String>,
}
impl FromStr for ResourceId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self>;
}
impl TryFrom<&Url> for ResourceId {
    type Error = Error;
    fn try_from(url: &Url) -> Result<Self>;
}
impl TryFrom<Url> for ResourceId {
    type Error = Error;
    fn try_from(url: Url) -> Result<Self>;
}
pub trait ResourceExt {
    fn resource_id(&self) -> Result<ResourceId>;
}
pub mod clients {
    pub struct SecretClient {
    }
    impl SecretClient {
        fn new(endpoint: &str, credential: Arc<dyn TokenCredential>, options: Option<SecretClientOptions>) -> Result<Self>;
    }
    impl SecretClient {
        async fn backup_secret(&self, secret_name: &str, options: Option<SecretClientBackupSecretOptions<'_>>) -> Result<Response<BackupSecretResult>>;
        async fn delete_secret(&self, secret_name: &str, options: Option<SecretClientDeleteSecretOptions<'_>>) -> Result<Response<DeletedSecret>>;
        fn endpoint(&self) -> &Url;
        async fn get_deleted_secret(&self, secret_name: &str, options: Option<SecretClientGetDeletedSecretOptions<'_>>) -> Result<Response<DeletedSecret>>;
        async fn get_secret(&self, secret_name: &str, options: Option<SecretClientGetSecretOptions<'_>>) -> Result<Response<Secret>>;
        fn list_deleted_secret_properties(&self, options: Option<SecretClientListDeletedSecretPropertiesOptions<'_>>) -> Result<Pager<ListDeletedSecretPropertiesResult>>;
        fn list_secret_properties(&self, options: Option<SecretClientListSecretPropertiesOptions<'_>>) -> Result<Pager<ListSecretPropertiesResult>>;
        fn list_secret_properties_versions(&self, secret_name: &str, options: Option<SecretClientListSecretPropertiesVersionsOptions<'_>>) -> Result<Pager<ListSecretPropertiesResult>>;
        async fn purge_deleted_secret(&self, secret_name: &str, options: Option<SecretClientPurgeDeletedSecretOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn recover_deleted_secret(&self, secret_name: &str, options: Option<SecretClientRecoverDeletedSecretOptions<'_>>) -> Result<Response<Secret>>;
        async fn restore_secret(&self, parameters: RequestContent<RestoreSecretParameters>, options: Option<SecretClientRestoreSecretOptions<'_>>) -> Result<Response<Secret>>;
        async fn set_secret(&self, secret_name: &str, parameters: RequestContent<SetSecretParameters>, options: Option<SecretClientSetSecretOptions<'_>>) -> Result<Response<Secret>>;
        async fn update_secret_properties(&self, secret_name: &str, parameters: RequestContent<UpdateSecretPropertiesParameters>, options: Option<SecretClientUpdateSecretPropertiesOptions<'_>>) -> Result<Response<Secret>>;
    }
    #[derive(Clone, Debug)]
    pub struct SecretClientOptions {
        pub api_version: String,
        pub client_options: azure_core::http::ClientOptions,
        pub verify_challenge_resource: Option<bool>,
    }
    impl Default for SecretClientOptions {
        fn default() -> Self;
    }
}
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct BackupSecretResult {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub value: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedSecret {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<SecretAttributes>,
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(default, rename = "deletedDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub deleted_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub kid: Option<String>,
        #[serde(skip_serializing)]
        pub managed: Option<bool>,
        #[serde(rename = "previousVersion", skip_serializing_if = "Option::is_none")]
        pub previous_version: Option<String>,
        #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
        pub recovery_id: Option<String>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedSecretProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<SecretAttributes>,
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(default, rename = "deletedDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub deleted_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub managed: Option<bool>,
        #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
        pub recovery_id: Option<String>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct KeyVaultError {
        #[serde(skip_serializing)]
        pub error: Option<KeyVaultErrorError>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct KeyVaultErrorError {
        #[serde(skip_serializing)]
        pub code: Option<String>,
        #[serde(rename = "innererror", skip_serializing)]
        pub inner_error: Option<Box<KeyVaultErrorError>>,
        #[serde(skip_serializing)]
        pub message: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ListDeletedSecretPropertiesResult {
        #[serde(rename = "nextLink", skip_serializing)]
        pub next_link: Option<String>,
        #[serde(default, skip_serializing)]
        pub value: Vec<DeletedSecretProperties>,
    }
    impl Page for super::ListDeletedSecretPropertiesResult {
        type IntoIter = <Vec<DeletedSecretProperties> as IntoIterator>::IntoIter;
        type Item = DeletedSecretProperties;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ListSecretPropertiesResult {
        #[serde(rename = "nextLink", skip_serializing)]
        pub next_link: Option<String>,
        #[serde(default, skip_serializing)]
        pub value: Vec<SecretProperties>,
    }
    impl Page for super::ListSecretPropertiesResult {
        type IntoIter = <Vec<SecretProperties> as IntoIterator>::IntoIter;
        type Item = SecretProperties;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct RestoreSecretParameters {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "value", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub secret_backup: Option<Vec<u8>>,
    }
    impl TryFrom<RestoreSecretParameters> for azure_core::http::RequestContent<super::RestoreSecretParameters> {
        type Error = Error;
        fn try_from(value: RestoreSecretParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Secret {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<SecretAttributes>,
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub kid: Option<String>,
        #[serde(skip_serializing)]
        pub managed: Option<bool>,
        #[serde(rename = "previousVersion", skip_serializing_if = "Option::is_none")]
        pub previous_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SecretAttributes {
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub created: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(default, rename = "exp", skip_serializing_if = "Option::is_none", with = "azure_core::time::unix_time::option")]
        pub expires: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, rename = "nbf", skip_serializing_if = "Option::is_none", with = "azure_core::time::unix_time::option")]
        pub not_before: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "recoverableDays", skip_serializing)]
        pub recoverable_days: Option<i32>,
        #[serde(rename = "recoveryLevel", skip_serializing)]
        pub recovery_level: Option<super::DeletionRecoveryLevel>,
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub updated: Option<azure_core::time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientBackupSecretOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientDeleteSecretOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientGetDeletedSecretOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientGetSecretOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub out_content_type: Option<super::ContentType>,
        pub secret_version: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientListDeletedSecretPropertiesOptions<'a> {
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl SecretClientListDeletedSecretPropertiesOptions<'_> {
        fn into_owned(self) -> SecretClientListDeletedSecretPropertiesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientListSecretPropertiesOptions<'a> {
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl SecretClientListSecretPropertiesOptions<'_> {
        fn into_owned(self) -> SecretClientListSecretPropertiesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientListSecretPropertiesVersionsOptions<'a> {
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl SecretClientListSecretPropertiesVersionsOptions<'_> {
        fn into_owned(self) -> SecretClientListSecretPropertiesVersionsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientPurgeDeletedSecretOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientRecoverDeletedSecretOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientRestoreSecretOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientSetSecretOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SecretClientUpdateSecretPropertiesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub secret_version: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct SecretProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<SecretAttributes>,
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub managed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SetSecretParameters {
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub secret_attributes: Option<SecretAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl TryFrom<SetSecretParameters> for azure_core::http::RequestContent<super::SetSecretParameters> {
        type Error = Error;
        fn try_from(value: SetSecretParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct UpdateSecretPropertiesParameters {
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub secret_attributes: Option<SecretAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<UpdateSecretPropertiesParameters> for azure_core::http::RequestContent<super::UpdateSecretPropertiesParameters> {
        type Error = Error;
        fn try_from(value: UpdateSecretPropertiesParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ContentType {
        Pem,
        Pfx,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ContentType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ContentType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ContentType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ContentType> for &'a str {
        fn from(e: &'a ContentType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ContentType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum DeletionRecoveryLevel {
        CustomizedRecoverable,
        CustomizedRecoverableProtectedSubscription,
        CustomizedRecoverablePurgeable,
        Purgeable,
        Recoverable,
        RecoverableProtectedSubscription,
        RecoverablePurgeable,
        UnknownValue(String),
    }
    impl AsRef<str> for super::DeletionRecoveryLevel {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::DeletionRecoveryLevel {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::DeletionRecoveryLevel {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::DeletionRecoveryLevel {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a DeletionRecoveryLevel> for &'a str {
        fn from(e: &'a DeletionRecoveryLevel) -> Self;
    }
    impl<'de> Deserialize<'de> for super::DeletionRecoveryLevel {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
}
```
