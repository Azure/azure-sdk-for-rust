use azure_core::auth::Secret;
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
pub struct KeyVaultCertificateBaseIdentifierAttributes {
    pub enabled: bool,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "exp")]
    pub expires_on: Option<OffsetDateTime>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "nbf")]
    pub not_before: Option<OffsetDateTime>,
    #[serde(with = "azure_core::date::timestamp", rename = "created")]
    pub created_on: OffsetDateTime,
    #[serde(with = "azure_core::date::timestamp", rename = "updated")]
    pub updated_on: OffsetDateTime,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultCertificateBaseIdentifier {
    pub id: String,
    #[allow(unused)]
    pub x5t: String,
    pub attributes: KeyVaultCertificateBaseIdentifierAttributes,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultGetCertificatesResponse {
    pub value: Vec<KeyVaultCertificateBaseIdentifier>,
    #[serde(rename = "nextLink")]
    pub next_link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultGetCertificateResponse {
    #[serde(rename = "kid")]
    pub key_id: String,
    #[serde(rename = "sid")]
    pub secret_id: String,
    pub x5t: String,
    pub cer: Secret,
    pub id: String,
    pub attributes: KeyVaultGetCertificateResponseAttributes,
    pub policy: KeyVaultGetCertificateResponsePolicy,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultGetCertificateResponseAttributes {
    pub enabled: bool,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "exp")]
    pub expires_on: Option<OffsetDateTime>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "nbf")]
    pub not_before: Option<OffsetDateTime>,
    #[serde(with = "azure_core::date::timestamp", rename = "created")]
    pub created_on: OffsetDateTime,
    #[serde(with = "azure_core::date::timestamp", rename = "updated")]
    pub updated_on: OffsetDateTime,
    #[serde(rename = "recoveryLevel")]
    #[allow(unused)]
    pub recovery_level: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct KeyVaultGetCertificateResponsePolicy {
    pub id: String,
    pub key_props: KeyVaultGetCertificateResponsePolicyKeyProperties,
    pub secret_props: KeyVaultGetCertificateResponsePolicySecretProperties,
    pub x509_props: KeyVaultGetCertificateResponsePolicyX509Properties,
    pub issuer: KeyVaultGetCertificateResponsePolicyIssuer,
    pub attributes: KeyVaultGetCertificateResponsePolicyAttributes,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct KeyVaultGetCertificateResponsePolicyKeyProperties {
    pub exportable: bool,
    pub kty: String,
    pub key_size: u64,
    pub reuse_key: bool,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultGetCertificateResponsePolicySecretProperties {
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct KeyVaultGetCertificateResponsePolicyX509Properties {
    pub subject: String,
    pub validity_months: u64,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct KeyVaultGetCertificateResponsePolicyIssuer {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct KeyVaultGetCertificateResponsePolicyAttributes {
    pub enabled: bool,
    #[serde(with = "azure_core::date::timestamp", rename = "created")]
    pub created_on: OffsetDateTime,
    #[serde(with = "azure_core::date::timestamp", rename = "updated")]
    pub updated_on: OffsetDateTime,
}

#[derive(Deserialize, Debug)]
pub struct CertificateBackupResponse {
    pub value: String,
}

#[derive(Debug)]
pub struct CertificateProperties {
    pub id: String,
    pub name: String,
    pub version: String,
    pub not_before: Option<OffsetDateTime>,
    pub expires_on: Option<OffsetDateTime>,
    pub created_on: OffsetDateTime,
    pub updated_on: OffsetDateTime,
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct CertificateOperationResponse {
    pub cancellation_requested: bool,
    pub csr: String,
    pub id: String,
    pub issuer: IssuerParameters,
    pub request_id: String,
    pub status: String,
    pub status_details: String,
}

#[derive(Deserialize, Debug)]
pub struct IssuerParameters {
    pub name: String,
}
