# azure_security_keyvault_certificates

- **Description**: Rust wrappers around Microsoft Azure REST APIs - Azure Key Vault Certificates
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
pub use azure_security_keyvault_certificates::generated::clients::certificate_client::CertificateClient;
pub use azure_security_keyvault_certificates::clients::CertificateClientOptions;
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
    pub struct CertificateClient {
    }
    impl CertificateClient {
        pub fn begin_create_certificate(&self, certificate_name: &str, parameters: RequestContent<CreateCertificateParameters>, options: Option<CertificateClientCreateCertificateOptions<'_>>) -> Result<Poller<CertificateOperation>>;
        pub fn new(endpoint: &str, credential: Arc<dyn TokenCredential>, options: Option<CertificateClientOptions>) -> Result<Self>;
    }
    impl CertificateClient {
        pub async fn backup_certificate(&self, certificate_name: &str, options: Option<CertificateClientBackupCertificateOptions<'_>>) -> Result<Response<BackupCertificateResult>>;
        pub async fn delete_certificate(&self, certificate_name: &str, options: Option<CertificateClientDeleteCertificateOptions<'_>>) -> Result<Response<DeletedCertificate>>;
        pub async fn delete_certificate_operation(&self, certificate_name: &str, options: Option<CertificateClientDeleteCertificateOperationOptions<'_>>) -> Result<Response<CertificateOperation>>;
        pub async fn delete_contacts(&self, options: Option<CertificateClientDeleteContactsOptions<'_>>) -> Result<Response<Contacts>>;
        pub async fn delete_issuer(&self, issuer_name: &str, options: Option<CertificateClientDeleteIssuerOptions<'_>>) -> Result<Response<Issuer>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get_certificate(&self, certificate_name: &str, options: Option<CertificateClientGetCertificateOptions<'_>>) -> Result<Response<Certificate>>;
        pub async fn get_certificate_operation(&self, certificate_name: &str, options: Option<CertificateClientGetCertificateOperationOptions<'_>>) -> Result<Response<CertificateOperation>>;
        pub async fn get_certificate_policy(&self, certificate_name: &str, options: Option<CertificateClientGetCertificatePolicyOptions<'_>>) -> Result<Response<CertificatePolicy>>;
        pub async fn get_contacts(&self, options: Option<CertificateClientGetContactsOptions<'_>>) -> Result<Response<Contacts>>;
        pub async fn get_deleted_certificate(&self, certificate_name: &str, options: Option<CertificateClientGetDeletedCertificateOptions<'_>>) -> Result<Response<DeletedCertificate>>;
        pub async fn get_issuer(&self, issuer_name: &str, options: Option<CertificateClientGetIssuerOptions<'_>>) -> Result<Response<Issuer>>;
        pub async fn import_certificate(&self, certificate_name: &str, parameters: RequestContent<ImportCertificateParameters>, options: Option<CertificateClientImportCertificateOptions<'_>>) -> Result<Response<Certificate>>;
        pub fn list_certificate_properties(&self, options: Option<CertificateClientListCertificatePropertiesOptions<'_>>) -> Result<Pager<ListCertificatePropertiesResult>>;
        pub fn list_certificate_properties_versions(&self, certificate_name: &str, options: Option<CertificateClientListCertificatePropertiesVersionsOptions<'_>>) -> Result<Pager<ListCertificatePropertiesResult>>;
        pub fn list_deleted_certificate_properties(&self, options: Option<CertificateClientListDeletedCertificatePropertiesOptions<'_>>) -> Result<Pager<ListDeletedCertificatePropertiesResult>>;
        pub fn list_issuer_properties(&self, options: Option<CertificateClientListIssuerPropertiesOptions<'_>>) -> Result<Pager<ListIssuerPropertiesResult>>;
        pub async fn merge_certificate(&self, certificate_name: &str, parameters: RequestContent<MergeCertificateParameters>, options: Option<CertificateClientMergeCertificateOptions<'_>>) -> Result<Response<Certificate>>;
        pub async fn purge_deleted_certificate(&self, certificate_name: &str, options: Option<CertificateClientPurgeDeletedCertificateOptions<'_>>) -> Result<Response<(), NoFormat>>;
        pub async fn recover_deleted_certificate(&self, certificate_name: &str, options: Option<CertificateClientRecoverDeletedCertificateOptions<'_>>) -> Result<Response<Certificate>>;
        pub async fn restore_certificate(&self, parameters: RequestContent<RestoreCertificateParameters>, options: Option<CertificateClientRestoreCertificateOptions<'_>>) -> Result<Response<Certificate>>;
        pub async fn set_contacts(&self, contacts: RequestContent<Contacts>, options: Option<CertificateClientSetContactsOptions<'_>>) -> Result<Response<Contacts>>;
        pub async fn set_issuer(&self, issuer_name: &str, parameter: RequestContent<SetIssuerParameters>, options: Option<CertificateClientSetIssuerOptions<'_>>) -> Result<Response<Issuer>>;
        pub async fn update_certificate_operation(&self, certificate_name: &str, certificate_operation: RequestContent<UpdateCertificateOperationParameter>, options: Option<CertificateClientUpdateCertificateOperationOptions<'_>>) -> Result<Response<CertificateOperation>>;
        pub async fn update_certificate_policy(&self, certificate_name: &str, certificate_policy: RequestContent<CertificatePolicy>, options: Option<CertificateClientUpdateCertificatePolicyOptions<'_>>) -> Result<Response<CertificatePolicy>>;
        pub async fn update_certificate_properties(&self, certificate_name: &str, parameters: RequestContent<UpdateCertificatePropertiesParameters>, options: Option<CertificateClientUpdateCertificatePropertiesOptions<'_>>) -> Result<Response<Certificate>>;
        pub async fn update_issuer(&self, issuer_name: &str, parameter: RequestContent<UpdateIssuerParameters>, options: Option<CertificateClientUpdateIssuerOptions<'_>>) -> Result<Response<Issuer>>;
    }
    #[derive(Clone, Debug)]
    pub struct CertificateClientOptions {
        pub api_version: String,
        pub client_options: azure_core::http::ClientOptions,
        pub verify_challenge_resource: Option<bool>,
    }
    impl Default for CertificateClientOptions {
        fn default() -> Self;
    }
}
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct AdministratorContact {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct BackupCertificateResult {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub value: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Certificate {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CertificateAttributes>,
        #[serde(default, deserialize_with = "base64::option::deserialize", serialize_with = "base64::option::serialize", skip_serializing_if = "Option::is_none")]
        pub cer: Option<Vec<u8>>,
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub kid: Option<String>,
        #[serde(skip_serializing)]
        pub policy: Option<CertificatePolicy>,
        #[serde(rename = "preserveCertOrder", skip_serializing_if = "Option::is_none")]
        pub preserve_cert_order: Option<bool>,
        #[serde(skip_serializing)]
        pub sid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "x5t", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub x509_thumbprint: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct CertificateAttributes {
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
    pub struct CertificateClientBackupCertificateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientCreateCertificateOptions<'a> {
        pub method_options: azure_core::http::poller::PollerOptions<'a>,
    }
    impl<'a> CertificateClientCreateCertificateOptions<'a> {
        #[must_use]
        pub fn into_owned(self) -> CertificateClientCreateCertificateOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientDeleteCertificateOperationOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientDeleteCertificateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientDeleteContactsOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientDeleteIssuerOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientGetCertificateOperationOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientGetCertificateOptions<'a> {
        pub certificate_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientGetCertificatePolicyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientGetContactsOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientGetDeletedCertificateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientGetIssuerOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientImportCertificateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientListCertificatePropertiesOptions<'a> {
        pub include_pending: Option<bool>,
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl CertificateClientListCertificatePropertiesOptions<'_> {
        pub fn into_owned(self) -> CertificateClientListCertificatePropertiesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientListCertificatePropertiesVersionsOptions<'a> {
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl CertificateClientListCertificatePropertiesVersionsOptions<'_> {
        pub fn into_owned(self) -> CertificateClientListCertificatePropertiesVersionsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientListDeletedCertificatePropertiesOptions<'a> {
        pub include_pending: Option<bool>,
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl CertificateClientListDeletedCertificatePropertiesOptions<'_> {
        pub fn into_owned(self) -> CertificateClientListDeletedCertificatePropertiesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientListIssuerPropertiesOptions<'a> {
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl CertificateClientListIssuerPropertiesOptions<'_> {
        pub fn into_owned(self) -> CertificateClientListIssuerPropertiesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientMergeCertificateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientPurgeDeletedCertificateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientRecoverDeletedCertificateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientRestoreCertificateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientSetContactsOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientSetIssuerOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientUpdateCertificateOperationOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientUpdateCertificatePolicyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientUpdateCertificatePropertiesOptions<'a> {
        pub certificate_version: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CertificateClientUpdateIssuerOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct CertificateOperation {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cancellation_requested: Option<bool>,
        #[serde(default, deserialize_with = "base64::option::deserialize", serialize_with = "base64::option::serialize", skip_serializing_if = "Option::is_none")]
        pub csr: Option<Vec<u8>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<KeyVaultErrorError>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
        pub issuer_parameters: Option<IssuerParameters>,
        #[serde(rename = "preserveCertOrder", skip_serializing_if = "Option::is_none")]
        pub preserve_cert_order: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status_details: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
    }
    impl ResourceExt for crate::models::CertificateOperation {
        fn resource_id(&self) -> Result<ResourceId>;
    }
    impl StatusMonitor for CertificateOperation {
        type Format = JsonFormat;
        type Output = Certificate;
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct CertificatePolicy {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CertificateAttributes>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
        pub issuer_parameters: Option<IssuerParameters>,
        #[serde(rename = "key_props", skip_serializing_if = "Option::is_none")]
        pub key_properties: Option<KeyProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lifetime_actions: Option<Vec<LifetimeAction>>,
        #[serde(rename = "platformManaged", skip_serializing_if = "Option::is_none")]
        pub platform_managed: Option<PlatformManaged>,
        #[serde(rename = "secret_props", skip_serializing_if = "Option::is_none")]
        pub secret_properties: Option<SecretProperties>,
        #[serde(rename = "x509_props", skip_serializing_if = "Option::is_none")]
        pub x509_certificate_properties: Option<X509CertificateProperties>,
    }
    impl TryFrom<CertificatePolicy> for azure_core::http::RequestContent<super::CertificatePolicy> {
        type Error = Error;
        fn try_from(value: CertificatePolicy) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct CertificateProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CertificateAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "x5t", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub x509_thumbprint: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Contact {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Contacts {
        #[serde(rename = "contacts", skip_serializing_if = "Option::is_none")]
        pub contact_list: Option<Vec<Contact>>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
    }
    impl TryFrom<Contacts> for azure_core::http::RequestContent<super::Contacts> {
        type Error = Error;
        fn try_from(value: Contacts) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct CreateCertificateParameters {
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub certificate_attributes: Option<CertificateAttributes>,
        #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
        pub certificate_policy: Option<CertificatePolicy>,
        #[serde(rename = "preserveCertOrder", skip_serializing_if = "Option::is_none")]
        pub preserve_cert_order: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<CreateCertificateParameters> for azure_core::http::RequestContent<CreateCertificateParameters> {
        type Error = Error;
        fn try_from(value: CreateCertificateParameters) -> azure_core::Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedCertificate {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CertificateAttributes>,
        #[serde(default, deserialize_with = "base64::option::deserialize", serialize_with = "base64::option::serialize", skip_serializing_if = "Option::is_none")]
        pub cer: Option<Vec<u8>>,
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(default, rename = "deletedDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub deleted_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub kid: Option<String>,
        #[serde(skip_serializing)]
        pub policy: Option<CertificatePolicy>,
        #[serde(rename = "preserveCertOrder", skip_serializing_if = "Option::is_none")]
        pub preserve_cert_order: Option<bool>,
        #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
        pub recovery_id: Option<String>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub sid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "x5t", serialize_with = "base64::option::serialize_url_safe", skip_serializing)]
        pub x509_thumbprint: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedCertificateProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<CertificateAttributes>,
        #[serde(default, rename = "deletedDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub deleted_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
        pub recovery_id: Option<String>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::unix_time::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "x5t", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub x509_thumbprint: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ImportCertificateParameters {
        #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
        pub base64_encoded_certificate: Option<String>,
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub certificate_attributes: Option<CertificateAttributes>,
        #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
        pub certificate_policy: Option<CertificatePolicy>,
        #[serde(rename = "pwd", skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[serde(rename = "preserveCertOrder", skip_serializing_if = "Option::is_none")]
        pub preserve_cert_order: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<ImportCertificateParameters> for azure_core::http::RequestContent<super::ImportCertificateParameters> {
        type Error = Error;
        fn try_from(value: ImportCertificateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Issuer {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<IssuerAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credentials: Option<IssuerCredentials>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(rename = "org_details", skip_serializing_if = "Option::is_none")]
        pub organization_details: Option<OrganizationDetails>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct IssuerAttributes {
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub created: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub updated: Option<azure_core::time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct IssuerCredentials {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
        #[serde(rename = "pwd", skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct IssuerParameters {
        #[serde(rename = "cert_transparency", skip_serializing_if = "Option::is_none")]
        pub certificate_transparency: Option<bool>,
        #[serde(rename = "cty", skip_serializing_if = "Option::is_none")]
        pub certificate_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct IssuerProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyProperties {
        #[serde(rename = "crv", skip_serializing_if = "Option::is_none")]
        pub curve: Option<super::CurveName>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exportable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[serde(rename = "kty", skip_serializing_if = "Option::is_none")]
        pub key_type: Option<super::KeyType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reuse_key: Option<bool>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct KeyVaultError {
        #[serde(skip_serializing)]
        pub error: Option<KeyVaultErrorError>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultErrorError {
        #[serde(skip_serializing)]
        pub code: Option<String>,
        #[serde(rename = "innererror", skip_serializing)]
        pub inner_error: Option<Box<KeyVaultErrorError>>,
        #[serde(skip_serializing)]
        pub message: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub days_before_expiry: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lifetime_percentage: Option<i32>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct LifetimeActionType {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action_type: Option<super::CertificatePolicyAction>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ListCertificatePropertiesResult {
        #[serde(rename = "nextLink", skip_serializing)]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<CertificateProperties>,
    }
    impl Page for super::ListCertificatePropertiesResult {
        type IntoIter = <Vec<CertificateProperties> as IntoIterator>::IntoIter;
        type Item = CertificateProperties;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ListDeletedCertificatePropertiesResult {
        #[serde(rename = "nextLink", skip_serializing)]
        pub next_link: Option<String>,
        #[serde(default, skip_serializing)]
        pub value: Vec<DeletedCertificateProperties>,
    }
    impl Page for super::ListDeletedCertificatePropertiesResult {
        type IntoIter = <Vec<DeletedCertificateProperties> as IntoIterator>::IntoIter;
        type Item = DeletedCertificateProperties;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ListIssuerPropertiesResult {
        #[serde(rename = "nextLink", skip_serializing)]
        pub next_link: Option<String>,
        #[serde(default, skip_serializing)]
        pub value: Vec<IssuerProperties>,
    }
    impl Page for super::ListIssuerPropertiesResult {
        type IntoIter = <Vec<IssuerProperties> as IntoIterator>::IntoIter;
        type Item = IssuerProperties;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MergeCertificateParameters {
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub certificate_attributes: Option<CertificateAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(default, rename = "x5c", skip_serializing_if = "Option::is_none", with = "models_serde::option_vec_encoded_bytes_std")]
        pub x509_certificates: Option<Vec<Vec<u8>>>,
    }
    impl TryFrom<MergeCertificateParameters> for azure_core::http::RequestContent<super::MergeCertificateParameters> {
        type Error = Error;
        fn try_from(value: MergeCertificateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct OrganizationDetails {
        #[serde(rename = "admin_details", skip_serializing_if = "Option::is_none")]
        pub admin_contacts: Option<Vec<AdministratorContact>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct PlatformManaged {
        #[serde(rename = "certificateUsage", skip_serializing_if = "Option::is_none")]
        pub certificate_usage: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metadata: Option<std::collections::HashMap<String, azure_core::Value>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct RestoreCertificateParameters {
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", rename = "value", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub certificate_backup: Option<Vec<u8>>,
    }
    impl TryFrom<RestoreCertificateParameters> for azure_core::http::RequestContent<super::RestoreCertificateParameters> {
        type Error = Error;
        fn try_from(value: RestoreCertificateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SecretProperties {
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SetIssuerParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<IssuerAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credentials: Option<IssuerCredentials>,
        #[serde(rename = "org_details", skip_serializing_if = "Option::is_none")]
        pub organization_details: Option<OrganizationDetails>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
    }
    impl TryFrom<SetIssuerParameters> for azure_core::http::RequestContent<super::SetIssuerParameters> {
        type Error = Error;
        fn try_from(value: SetIssuerParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SubjectAlternativeNames {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dns_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub emails: Option<Vec<String>>,
        #[serde(rename = "ipAddresses", skip_serializing_if = "Option::is_none")]
        pub ip_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub uris: Option<Vec<String>>,
        #[serde(rename = "upns", skip_serializing_if = "Option::is_none")]
        pub user_principal_names: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct UpdateCertificateOperationParameter {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cancellation_requested: Option<bool>,
    }
    impl TryFrom<UpdateCertificateOperationParameter> for azure_core::http::RequestContent<super::UpdateCertificateOperationParameter> {
        type Error = Error;
        fn try_from(value: UpdateCertificateOperationParameter) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct UpdateCertificatePropertiesParameters {
        #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
        pub certificate_attributes: Option<CertificateAttributes>,
        #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
        pub certificate_policy: Option<CertificatePolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<UpdateCertificatePropertiesParameters> for azure_core::http::RequestContent<super::UpdateCertificatePropertiesParameters> {
        type Error = Error;
        fn try_from(value: UpdateCertificatePropertiesParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct UpdateIssuerParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<IssuerAttributes>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credentials: Option<IssuerCredentials>,
        #[serde(rename = "org_details", skip_serializing_if = "Option::is_none")]
        pub organization_details: Option<OrganizationDetails>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
    }
    impl TryFrom<UpdateIssuerParameters> for azure_core::http::RequestContent<super::UpdateIssuerParameters> {
        type Error = Error;
        fn try_from(value: UpdateIssuerParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct X509CertificateProperties {
        #[serde(rename = "ekus", skip_serializing_if = "Option::is_none")]
        pub enhanced_key_usage: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key_usage: Option<Vec<super::KeyUsageType>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subject: Option<String>,
        #[serde(rename = "sans", skip_serializing_if = "Option::is_none")]
        pub subject_alternative_names: Option<SubjectAlternativeNames>,
        #[serde(rename = "validity_months", skip_serializing_if = "Option::is_none")]
        pub validity_in_months: Option<i32>,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CertificatePolicyAction {
        AutoRenew,
        EmailContacts,
    }
    impl AsRef<str> for super::CertificatePolicyAction {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::CertificatePolicyAction {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::CertificatePolicyAction {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::CertificatePolicyAction {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::CertificatePolicyAction {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
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
    pub enum KeyUsageType {
        CRlSign,
        DataEncipherment,
        DecipherOnly,
        DigitalSignature,
        EncipherOnly,
        KeyAgreement,
        KeyCertSign,
        KeyEncipherment,
        NonRepudiation,
        UnknownValue(String),
    }
    impl AsRef<str> for super::KeyUsageType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::KeyUsageType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::KeyUsageType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::KeyUsageType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a KeyUsageType> for &'a str {
        fn from(e: &'a KeyUsageType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::KeyUsageType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
}
```
