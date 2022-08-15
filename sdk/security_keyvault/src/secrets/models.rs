use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
pub struct KeyVaultSecretBaseIdentifierAttributedRaw {
    pub enabled: bool,
    #[serde(with = "azure_core::date::timestamp")]
    pub created: OffsetDateTime,
    #[serde(with = "azure_core::date::timestamp")]
    pub updated: OffsetDateTime,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultSecretBaseIdentifierRaw {
    pub id: String,
    pub attributes: KeyVaultSecretBaseIdentifierAttributedRaw,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultGetSecretsResponse {
    pub value: Vec<KeyVaultSecretBaseIdentifierRaw>,
    #[serde(rename = "nextLink")]
    pub next_link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultGetSecretResponse {
    pub value: String,
    pub id: String,
    pub attributes: KeyVaultGetSecretResponseAttributes,
}

#[derive(Deserialize, Debug)]
pub struct KeyVaultGetSecretResponseAttributes {
    pub enabled: bool,
    #[serde(default)]
    #[serde(with = "azure_core::date::timestamp::option", rename = "exp")]
    pub expires_on: Option<OffsetDateTime>,
    #[serde(with = "azure_core::date::timestamp", rename = "created")]
    pub created_on: OffsetDateTime,
    #[serde(with = "azure_core::date::timestamp", rename = "updated")]
    pub updated_on: OffsetDateTime,
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
