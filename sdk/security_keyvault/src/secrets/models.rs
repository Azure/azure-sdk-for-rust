use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KeyVaultSecretBaseIdentifierAttributedRaw {
    pub enabled: bool,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated: DateTime<Utc>,
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
pub(crate) struct KeyVaultGetSecretResponse {
    pub value: String,
    pub id: String,
    pub attributes: KeyVaultGetSecretResponseAttributes,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetSecretResponseAttributes {
    pub enabled: bool,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub exp: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated: DateTime<Utc>,
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
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}

#[derive(Debug)]
pub struct KeyVaultSecret {
    pub id: String,
    pub value: String,
    pub enabled: bool,
    pub expires_on: Option<DateTime<Utc>>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}
