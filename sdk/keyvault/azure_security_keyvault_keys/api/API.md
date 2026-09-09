# azure_security_keyvault_keys

- **Description**: Rust wrappers around Microsoft Azure REST APIs - Azure Key Vault Keys
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
pub use azure_security_keyvault_keys::generated::clients::key_client::KeyClient;
pub use azure_security_keyvault_keys::clients::KeyClientOptions;
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
    pub struct KeyClient {
    }
    impl KeyClient {
        fn new(endpoint: &str, credential: Arc<dyn TokenCredential>, options: Option<KeyClientOptions>) -> Result<Self>;
    }
    impl KeyClient {
        async fn backup_key(&self, key_name: &str, options: Option<KeyClientBackupKeyOptions<'_>>) -> Result<Response<BackupKeyResult>>;
        async fn create_key(&self, key_name: &str, parameters: RequestContent<CreateKeyParameters>, options: Option<KeyClientCreateKeyOptions<'_>>) -> Result<Response<Key>>;
        async fn decrypt(&self, key_name: &str, key_version: &str, parameters: RequestContent<KeyOperationParameters>, options: Option<KeyClientDecryptOptions<'_>>) -> Result<Response<KeyOperationResult>>;
        async fn delete_key(&self, key_name: &str, options: Option<KeyClientDeleteKeyOptions<'_>>) -> Result<Response<DeletedKey>>;
        async fn encrypt(&self, key_name: &str, parameters: RequestContent<KeyOperationParameters>, options: Option<KeyClientEncryptOptions<'_>>) -> Result<Response<KeyOperationResult>>;
        fn endpoint(&self) -> &Url;
        async fn get_deleted_key(&self, key_name: &str, options: Option<KeyClientGetDeletedKeyOptions<'_>>) -> Result<Response<DeletedKey>>;
        async fn get_key(&self, key_name: &str, options: Option<KeyClientGetKeyOptions<'_>>) -> Result<Response<Key>>;
        async fn get_key_attestation(&self, key_name: &str, options: Option<KeyClientGetKeyAttestationOptions<'_>>) -> Result<Response<Key>>;
        async fn get_key_rotation_policy(&self, key_name: &str, options: Option<KeyClientGetKeyRotationPolicyOptions<'_>>) -> Result<Response<KeyRotationPolicy>>;
        async fn get_random_bytes(&self, parameters: RequestContent<GetRandomBytesParameters>, options: Option<KeyClientGetRandomBytesOptions<'_>>) -> Result<Response<RandomBytes>>;
        async fn import_key(&self, key_name: &str, parameters: RequestContent<ImportKeyParameters>, options: Option<KeyClientImportKeyOptions<'_>>) -> Result<Response<Key>>;
        fn list_deleted_key_properties(&self, options: Option<KeyClientListDeletedKeyPropertiesOptions<'_>>) -> Result<Pager<ListDeletedKeyPropertiesResult>>;
        fn list_key_properties(&self, options: Option<KeyClientListKeyPropertiesOptions<'_>>) -> Result<Pager<ListKeyPropertiesResult>>;
        fn list_key_properties_versions(&self, key_name: &str, options: Option<KeyClientListKeyPropertiesVersionsOptions<'_>>) -> Result<Pager<ListKeyPropertiesResult>>;
        async fn purge_deleted_key(&self, key_name: &str, options: Option<KeyClientPurgeDeletedKeyOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn recover_deleted_key(&self, key_name: &str, options: Option<KeyClientRecoverDeletedKeyOptions<'_>>) -> Result<Response<Key>>;
        async fn release(&self, key_name: &str, parameters: RequestContent<ReleaseParameters>, options: Option<KeyClientReleaseOptions<'_>>) -> Result<Response<KeyReleaseResult>>;
        async fn restore_key(&self, parameters: RequestContent<RestoreKeyParameters>, options: Option<KeyClientRestoreKeyOptions<'_>>) -> Result<Response<Key>>;
        async fn rotate_key(&self, key_name: &str, options: Option<KeyClientRotateKeyOptions<'_>>) -> Result<Response<Key>>;
        async fn sign(&self, key_name: &str, parameters: RequestContent<SignParameters>, options: Option<KeyClientSignOptions<'_>>) -> Result<Response<KeyOperationResult>>;
        async fn unwrap_key(&self, key_name: &str, key_version: &str, parameters: RequestContent<KeyOperationParameters>, options: Option<KeyClientUnwrapKeyOptions<'_>>) -> Result<Response<KeyOperationResult>>;
        async fn update_key_properties(&self, key_name: &str, parameters: RequestContent<UpdateKeyPropertiesParameters>, options: Option<KeyClientUpdateKeyPropertiesOptions<'_>>) -> Result<Response<Key>>;
        async fn update_key_rotation_policy(&self, key_name: &str, key_rotation_policy: RequestContent<KeyRotationPolicy>, options: Option<KeyClientUpdateKeyRotationPolicyOptions<'_>>) -> Result<Response<KeyRotationPolicy>>;
        async fn verify(&self, key_name: &str, key_version: &str, parameters: RequestContent<VerifyParameters>, options: Option<KeyClientVerifyOptions<'_>>) -> Result<Response<KeyVerifyResult>>;
        async fn wrap_key(&self, key_name: &str, parameters: RequestContent<KeyOperationParameters>, options: Option<KeyClientWrapKeyOptions<'_>>) -> Result<Response<KeyOperationResult>>;
    }
    #[derive(Clone, Debug)]
    pub struct KeyClientOptions {
        pub api_version: String,
        pub client_options: azure_core::http::ClientOptions,
        pub verify_challenge_resource: Option<bool>,
    }
    impl Default for KeyClientOptions {
        fn default() -> Self;
    }
}
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct BackupKeyResult {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub value: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct CreateKeyParameters {
        #[serde(rename = "crv", skip_serializing_if = "Option::is_none")]
        pub curve: Option<super::CurveName>,
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub key_attributes: Option<KeyAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key_ops: Option<Vec<super::KeyOperation>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kty: Option<super::KeyType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub public_exponent: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub release_policy: Option<KeyReleasePolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<CreateKeyParameters> for azure_core::http::RequestContent<super::CreateKeyParameters> {
        type Error = Error;
        fn try_from(value: CreateKeyParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedKey {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<KeyAttributes>,
        #[serde(default, rename = "deletedDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub deleted_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<JsonWebKey>,
        #[serde(skip_serializing)]
        pub managed: Option<bool>,
        #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
        pub recovery_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub release_policy: Option<KeyReleasePolicy>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedKeyProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<KeyAttributes>,
        #[serde(default, rename = "deletedDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub deleted_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kid: Option<String>,
        #[serde(skip_serializing)]
        pub managed: Option<bool>,
        #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
        pub recovery_id: Option<String>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct GetRandomBytesParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,
    }
    impl TryFrom<GetRandomBytesParameters> for azure_core::http::RequestContent<super::GetRandomBytesParameters> {
        type Error = Error;
        fn try_from(value: GetRandomBytesParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ImportKeyParameters {
        #[serde(rename = "Hsm", skip_serializing_if = "Option::is_none")]
        pub hsm: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<JsonWebKey>,
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub key_attributes: Option<KeyAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub release_policy: Option<KeyReleasePolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<ImportKeyParameters> for azure_core::http::RequestContent<super::ImportKeyParameters> {
        type Error = Error;
        fn try_from(value: ImportKeyParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct JsonWebKey {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub crv: Option<super::CurveName>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub d: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub dp: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub dq: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub e: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub k: Option<Vec<u8>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key_ops: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kty: Option<super::KeyType>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub n: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub p: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub q: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub qi: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "key_hsm", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub t: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub x: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub y: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Key {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<KeyAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<JsonWebKey>,
        #[serde(skip_serializing)]
        pub managed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub release_policy: Option<KeyReleasePolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyAttestation {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "certificatePemFile", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub certificate_pem_file: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "privateKeyAttestation", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub private_key_attestation: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "publicKeyAttestation", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub public_key_attestation: Option<Vec<u8>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyAttributes {
        #[serde(skip_serializing)]
        pub attestation: Option<KeyAttestation>,
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub created: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(default, rename = "exp", skip_serializing_if = "Option::is_none", with = "azure_core::time::unix_time::option")]
        pub expires: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exportable: Option<bool>,
        #[serde(rename = "hsmPlatform", skip_serializing)]
        pub hsm_platform: Option<String>,
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
    pub struct KeyClientBackupKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientCreateKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientDecryptOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientDeleteKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientEncryptOptions<'a> {
        pub key_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientGetDeletedKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientGetKeyAttestationOptions<'a> {
        pub key_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientGetKeyOptions<'a> {
        pub key_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientGetKeyRotationPolicyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientGetRandomBytesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientImportKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientListDeletedKeyPropertiesOptions<'a> {
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyClientListDeletedKeyPropertiesOptions<'_> {
        fn into_owned(self) -> KeyClientListDeletedKeyPropertiesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientListKeyPropertiesOptions<'a> {
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyClientListKeyPropertiesOptions<'_> {
        fn into_owned(self) -> KeyClientListKeyPropertiesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientListKeyPropertiesVersionsOptions<'a> {
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyClientListKeyPropertiesVersionsOptions<'_> {
        fn into_owned(self) -> KeyClientListKeyPropertiesVersionsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientPurgeDeletedKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientRecoverDeletedKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientReleaseOptions<'a> {
        pub key_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientRestoreKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientRotateKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientSignOptions<'a> {
        pub key_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientUnwrapKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientUpdateKeyPropertiesOptions<'a> {
        pub key_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientUpdateKeyRotationPolicyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientVerifyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyClientWrapKeyOptions<'a> {
        pub key_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyOperationParameters {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "aad", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub additional_authenticated_data: Option<Vec<u8>>,
        #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
        pub algorithm: Option<super::EncryptionAlgorithm>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "tag", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub authentication_tag: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub iv: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub value: Option<Vec<u8>>,
    }
    impl TryFrom<KeyOperationParameters> for azure_core::http::RequestContent<super::KeyOperationParameters> {
        type Error = Error;
        fn try_from(value: KeyOperationParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyOperationResult {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "aad", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub additional_authenticated_data: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "tag", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub authentication_tag: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub iv: Option<Vec<u8>>,
        #[serde(skip_serializing)]
        pub kid: Option<String>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "value", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub result: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<KeyAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kid: Option<String>,
        #[serde(skip_serializing)]
        pub managed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyReleasePolicy {
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "data", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub encoded_policy: Option<Vec<u8>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub immutable: Option<bool>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyReleaseResult {
        #[serde(skip_serializing)]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyRotationPolicy {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<KeyRotationPolicyAttributes>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(rename = "lifetimeActions", skip_serializing_if = "Option::is_none")]
        pub lifetime_actions: Option<Vec<LifetimeAction>>,
    }
    impl TryFrom<KeyRotationPolicy> for azure_core::http::RequestContent<super::KeyRotationPolicy> {
        type Error = Error;
        fn try_from(value: KeyRotationPolicy) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyRotationPolicyAttributes {
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub created: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "expiryTime", skip_serializing_if = "Option::is_none")]
        pub expiry_time: Option<String>,
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub updated: Option<azure_core::time::OffsetDateTime>,
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
    pub struct KeyVerifyResult {
        #[serde(skip_serializing)]
        pub value: Option<bool>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct LifetimeAction {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<LifetimeActionType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger: Option<LifetimeActionTrigger>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct LifetimeActionTrigger {
        #[serde(rename = "timeAfterCreate", skip_serializing_if = "Option::is_none")]
        pub time_after_create: Option<String>,
        #[serde(rename = "timeBeforeExpiry", skip_serializing_if = "Option::is_none")]
        pub time_before_expiry: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct LifetimeActionType {
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_prop: Option<super::KeyRotationPolicyAction>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ListDeletedKeyPropertiesResult {
        #[serde(rename = "nextLink", skip_serializing)]
        pub next_link: Option<String>,
        #[serde(default, skip_serializing)]
        pub value: Vec<DeletedKeyProperties>,
    }
    impl Page for super::ListDeletedKeyPropertiesResult {
        type IntoIter = <Vec<DeletedKeyProperties> as IntoIterator>::IntoIter;
        type Item = DeletedKeyProperties;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ListKeyPropertiesResult {
        #[serde(rename = "nextLink", skip_serializing)]
        pub next_link: Option<String>,
        #[serde(default, skip_serializing)]
        pub value: Vec<KeyProperties>,
    }
    impl Page for super::ListKeyPropertiesResult {
        type IntoIter = <Vec<KeyProperties> as IntoIterator>::IntoIter;
        type Item = KeyProperties;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct RandomBytes {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub value: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ReleaseParameters {
        #[serde(rename = "enc", skip_serializing_if = "Option::is_none")]
        pub algorithm: Option<super::KeyEncryptionAlgorithm>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
        #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
        pub target_attestation_token: Option<String>,
    }
    impl TryFrom<ReleaseParameters> for azure_core::http::RequestContent<super::ReleaseParameters> {
        type Error = Error;
        fn try_from(value: ReleaseParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct RestoreKeyParameters {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "value", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub key_backup: Option<Vec<u8>>,
    }
    impl TryFrom<RestoreKeyParameters> for azure_core::http::RequestContent<super::RestoreKeyParameters> {
        type Error = Error;
        fn try_from(value: RestoreKeyParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SignParameters {
        #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
        pub algorithm: Option<super::SignatureAlgorithm>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub value: Option<Vec<u8>>,
    }
    impl TryFrom<SignParameters> for azure_core::http::RequestContent<super::SignParameters> {
        type Error = Error;
        fn try_from(value: SignParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct UpdateKeyPropertiesParameters {
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub key_attributes: Option<KeyAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key_ops: Option<Vec<super::KeyOperation>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub release_policy: Option<KeyReleasePolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<UpdateKeyPropertiesParameters> for azure_core::http::RequestContent<super::UpdateKeyPropertiesParameters> {
        type Error = Error;
        fn try_from(value: UpdateKeyPropertiesParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VerifyParameters {
        #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
        pub algorithm: Option<super::SignatureAlgorithm>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub digest: Option<Vec<u8>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "value", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub signature: Option<Vec<u8>>,
    }
    impl TryFrom<VerifyParameters> for azure_core::http::RequestContent<super::VerifyParameters> {
        type Error = Error;
        fn try_from(value: VerifyParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum CurveName {
        P256,
        P256K,
        P384,
        P521,
        UnknownValue(String),
    }
    impl AsRef<str> for super::CurveName {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::CurveName {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::CurveName {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::CurveName {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a CurveName> for &'a str {
        fn from(e: &'a CurveName) -> Self;
    }
    impl<'de> Deserialize<'de> for super::CurveName {
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
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum EncryptionAlgorithm {
        A128Cbc,
        A128Cbcpad,
        A128Gcm,
        A128Kw,
        A192Cbc,
        A192Cbcpad,
        A192Gcm,
        A192Kw,
        A256Cbc,
        A256Cbcpad,
        A256Gcm,
        A256Kw,
        CkmAesKeyWrap,
        CkmAesKeyWrapPad,
        Rsa1_5,
        RsaOaep,
        RsaOaep256,
        UnknownValue(String),
    }
    impl AsRef<str> for super::EncryptionAlgorithm {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::EncryptionAlgorithm {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::EncryptionAlgorithm {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::EncryptionAlgorithm {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a EncryptionAlgorithm> for &'a str {
        fn from(e: &'a EncryptionAlgorithm) -> Self;
    }
    impl<'de> Deserialize<'de> for super::EncryptionAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum KeyEncryptionAlgorithm {
        CkmRsaAesKeyWrap,
        RsaAesKeyWrap256,
        RsaAesKeyWrap384,
        UnknownValue(String),
    }
    impl AsRef<str> for super::KeyEncryptionAlgorithm {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::KeyEncryptionAlgorithm {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::KeyEncryptionAlgorithm {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::KeyEncryptionAlgorithm {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a KeyEncryptionAlgorithm> for &'a str {
        fn from(e: &'a KeyEncryptionAlgorithm) -> Self;
    }
    impl<'de> Deserialize<'de> for super::KeyEncryptionAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum KeyOperation {
        Decrypt,
        Encrypt,
        Export,
        Import,
        Sign,
        UnwrapKey,
        Verify,
        WrapKey,
        UnknownValue(String),
    }
    impl AsRef<str> for super::KeyOperation {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::KeyOperation {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::KeyOperation {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::KeyOperation {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a KeyOperation> for &'a str {
        fn from(e: &'a KeyOperation) -> Self;
    }
    impl<'de> Deserialize<'de> for super::KeyOperation {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum KeyRotationPolicyAction {
        Notify,
        Rotate,
    }
    impl AsRef<str> for super::KeyRotationPolicyAction {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::KeyRotationPolicyAction {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::KeyRotationPolicyAction {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::KeyRotationPolicyAction {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::KeyRotationPolicyAction {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum KeyType {
        Ec,
        EcHsm,
        Oct,
        OctHsm,
        Rsa,
        RsaHsm,
        UnknownValue(String),
    }
    impl AsRef<str> for super::KeyType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::KeyType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::KeyType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::KeyType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a KeyType> for &'a str {
        fn from(e: &'a KeyType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::KeyType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SignatureAlgorithm {
        Es256,
        Es256K,
        Es384,
        Es512,
        Hs256,
        Hs384,
        Hs512,
        Ps256,
        Ps384,
        Ps512,
        Rs256,
        Rs384,
        Rs512,
        Rsnull,
        UnknownValue(String),
    }
    impl AsRef<str> for super::SignatureAlgorithm {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SignatureAlgorithm {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SignatureAlgorithm {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SignatureAlgorithm {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a SignatureAlgorithm> for &'a str {
        fn from(e: &'a SignatureAlgorithm) -> Self;
    }
    impl<'de> Deserialize<'de> for super::SignatureAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
}
```
