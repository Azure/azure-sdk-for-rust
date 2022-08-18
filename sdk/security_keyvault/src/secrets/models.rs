use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultSecretBaseIdentifierAttributedRaw {
    pub enabled: bool,
    #[serde(with = "azure_core::date::timestamp")]
    pub created: OffsetDateTime,
    #[serde(with = "azure_core::date::timestamp")]
    pub updated: OffsetDateTime,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultSecretBaseIdentifierRaw {
    pub id: String,
    pub attributes: KeyVaultSecretBaseIdentifierAttributedRaw,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetSecretsResponse {
    pub value: Vec<KeyVaultSecretBaseIdentifierRaw>,
    #[serde(rename = "nextLink")]
    pub next_link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetSecretResponse {
    pub value: String,
    pub id: String,
    pub attributes: KeyVaultGetSecretResponseAttributes,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetSecretResponseAttributes {
    pub enabled: bool,
    #[serde(default, with = "azure_core::date::timestamp::option")]
    pub exp: Option<OffsetDateTime>,
    #[serde(with = "azure_core::date::timestamp")]
    pub created: OffsetDateTime,
    #[serde(with = "azure_core::date::timestamp")]
    pub updated: OffsetDateTime,
    #[serde(rename = "recoveryLevel")]
    #[allow(unused)]
    pub recovery_level: String,
}

#[derive(Debug)]
pub struct KeyVaultSecretBackupBlob {
    pub value: String,
}

#[derive(Debug)]
pub struct KeyVaultSecretBaseIdentifier {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub created_on: OffsetDateTime,
    pub updated_on: OffsetDateTime,
}

#[derive(Debug)]
pub struct KeyVaultSecret {
    pub id: String,
    pub value: String,
    pub enabled: bool,
    pub expires_on: Option<OffsetDateTime>,
    pub created_on: OffsetDateTime,
    pub updated_on: OffsetDateTime,
}
