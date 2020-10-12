use crate::KeyVaultClient;
use crate::{client::API_VERSION, KeyVaultError};
use azure_auth_aad::TokenCredential;
use anyhow::{Context, Result};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use getset::Getters;
use reqwest::Url;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::fmt;

const DEFAULT_MAX_RESULTS: usize = 25;

/// Reflects the deletion recovery level currently in effect for keys in the current Key Vault.
/// If it contains 'Purgeable' the key can be permanently deleted by a privileged user;
/// otherwise, only the system can purge the key, at the end of the retention interval.
pub enum RecoveryLevel {
    Purgeable,
    Recoverable,
    RecoverableAndProtectedSubscription,
    RecoverableAndPurgeable,
}

impl fmt::Display for RecoveryLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecoveryLevel::Purgeable => write!(f, "Purgeable"),
            RecoveryLevel::Recoverable => write!(f, "Recoverable"),
            RecoveryLevel::RecoverableAndProtectedSubscription => {
                write!(f, "Recoverable+ProtectedSubscription")
            }
            RecoveryLevel::RecoverableAndPurgeable => write!(f, "Recoverable+Purgeable"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultSecretBaseIdentifierAttributedRaw {
    enabled: bool,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultSecretBaseIdentifierRaw {
    id: String,
    attributes: KeyVaultSecretBaseIdentifierAttributedRaw,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetSecretsResponse {
    value: Vec<KeyVaultSecretBaseIdentifierRaw>,
    #[serde(rename = "nextLink")]
    next_link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetSecretResponse {
    value: String,
    id: String,
    attributes: KeyVaultGetSecretResponseAttributes,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetSecretResponseAttributes {
    enabled: bool,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
    #[serde(rename = "recoveryLevel")]
    recovery_level: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultSecretBackupResponseRaw {
    value: String,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct KeyVaultSecretBackupBlob {
    value: String,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct KeyVaultSecretBaseIdentifier {
    id: String,
    name: String,
    enabled: bool,
    time_created: DateTime<Utc>,
    time_updated: DateTime<Utc>,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct KeyVaultSecret {
    id: String,
    value: String,
    enabled: bool,
    time_created: DateTime<Utc>,
    time_updated: DateTime<Utc>,
}

impl<'a, T:TokenCredential> KeyVaultClient<'a, T> {
    /// Gets a secret from the Key Vault.
    /// Note that the latest version is fetched. For a specific version, use `get_version_with_version`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     let secret = client.get_secret(&"SECRET_NAME").await.unwrap();
    ///     dbg!(&secret);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn get_secret(
        &mut self,
        secret_name: &'a str,
    ) -> Result<KeyVaultSecret, KeyVaultError> {
        Ok(self.get_secret_with_version(secret_name, "").await?)
    }

    /// Gets a secret from the Key Vault with a specific version.
    /// If you need the latest version, use `get_secret`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     let secret = client.get_secret_with_version(&"SECRET_NAME", &"SECRET_VERSION").await.unwrap();
    ///     dbg!(&secret);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn get_secret_with_version(
        &mut self,
        secret_name: &'a str,
        secret_version_name: &'a str,
    ) -> Result<KeyVaultSecret, KeyVaultError> {
        let uri = Url::parse_with_params(
            &format!(
                "{}/secrets/{}/{}",
                self.keyvault_endpoint, secret_name, secret_version_name
            ),
            &[("api-version", API_VERSION)],
        )
        .unwrap();
        let resp_body = self.get_authed(uri.to_string()).await?;
        let response = serde_json::from_str::<KeyVaultGetSecretResponse>(&resp_body)
            .with_context(|| format!("Failed to parse response from Key Vault: {}", resp_body))?;
        Ok(KeyVaultSecret {
            enabled: response.attributes.enabled,
            value: response.value,
            time_created: response.attributes.created,
            time_updated: response.attributes.updated,
            id: response.id,
        })
    }

    /// Lists all the secrets in the Key Vault.
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     let secrets = client.list_secrets().await.unwrap();
    ///     dbg!(&secrets);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn list_secrets(
        &mut self,
    ) -> Result<Vec<KeyVaultSecretBaseIdentifier>, KeyVaultError> {
        let mut secrets = Vec::<KeyVaultSecretBaseIdentifier>::new();
        let mut uri = Url::parse_with_params(
            &format!("{}/secrets", self.keyvault_endpoint),
            &[
                ("api-version", API_VERSION),
                ("maxresults", &DEFAULT_MAX_RESULTS.to_string()),
            ],
        )
        .unwrap();

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let response = serde_json::from_str::<KeyVaultGetSecretsResponse>(&resp_body).unwrap();

            secrets.extend(
                response
                    .value
                    .into_iter()
                    .map(|s| KeyVaultSecretBaseIdentifier {
                        id: s.id.to_owned(),
                        name: s.id.to_owned().split("/").last().unwrap().to_owned(),
                        enabled: s.attributes.enabled,
                        time_created: s.attributes.created,
                        time_updated: s.attributes.updated,
                    })
                    .collect::<Vec<KeyVaultSecretBaseIdentifier>>(),
            );

            match response.next_link {
                None => break,
                Some(u) => uri = Url::parse(&u).unwrap(),
            }
        }

        Ok(secrets)
    }

    /// Gets all the versions for a secret in the Key Vault.
    //
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     let secret_versions = client.get_secret_versions(&"SECRET_NAME").await.unwrap();
    ///     dbg!(&secret_versions);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn get_secret_versions(
        &mut self,
        secret_name: &'a str,
    ) -> Result<Vec<KeyVaultSecretBaseIdentifier>, KeyVaultError> {
        let mut secret_versions = Vec::<KeyVaultSecretBaseIdentifier>::new();
        let mut uri = Url::parse_with_params(
            &format!(
                "{}/secrets/{}/versions",
                self.keyvault_endpoint, secret_name
            ),
            &[
                ("api-version", API_VERSION),
                ("maxresults", &DEFAULT_MAX_RESULTS.to_string()),
            ],
        )
        .unwrap();

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let response = serde_json::from_str::<KeyVaultGetSecretsResponse>(&resp_body).unwrap();

            secret_versions.extend(
                response
                    .value
                    .into_iter()
                    .map(|s| KeyVaultSecretBaseIdentifier {
                        id: s.id.to_owned(),
                        name: s.id.to_owned().split("/").last().unwrap().to_owned(),
                        enabled: s.attributes.enabled,
                        time_created: s.attributes.created,
                        time_updated: s.attributes.updated,
                    })
                    .collect::<Vec<KeyVaultSecretBaseIdentifier>>(),
            );
            match response.next_link {
                None => break,
                Some(u) => uri = Url::parse(&u).unwrap(),
            }
        }

        // Return the secret versions sorted by the time modified in descending order.
        secret_versions.sort_by(|a, b| {
            if a.time_updated > b.time_updated {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        Ok(secret_versions)
    }

    /// Sets the value of a secret in the Key Vault.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     client.set_secret(&"SECRET_NAME", &"NEW_VALUE").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn set_secret(
        &mut self,
        secret_name: &'a str,
        new_secret_value: &'a str,
    ) -> Result<(), KeyVaultError> {
        let uri = Url::parse_with_params(
            &format!("{}/secrets/{}", self.keyvault_endpoint, secret_name),
            &[("api-version", API_VERSION)],
        )
        .unwrap();

        let mut request_body = Map::new();
        request_body.insert(
            "value".to_owned(),
            Value::String(new_secret_value.to_owned()),
        );

        self.put_authed(uri.to_string(), Value::Object(request_body).to_string())
            .await?;

        Ok(())
    }

    /// Updates whether a secret version is enabled or not.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - Name of the secret
    /// * `secret_version` - Version of the secret. Use an empty string for the latest version
    /// * `enabled` - New `enabled` value of the secret
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     client.update_secret_enabled(&"SECRET_NAME", &"", true).await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn update_secret_enabled(
        &mut self,
        secret_name: &'a str,
        secret_version: &'a str,
        enabled: bool,
    ) -> Result<(), KeyVaultError> {
        let mut attributes = Map::new();
        attributes.insert("enabled".to_owned(), Value::Bool(enabled));

        self.update_secret(secret_name, secret_version, attributes)
            .await?;

        Ok(())
    }

    /// Updates the [`RecoveryLevel`](RecoveryLevel) of a secret version.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - Name of the secret
    /// * `secret_version` - Version of the secret. Use an empty string for the latest version
    /// * `recovery_level` - New `RecoveryLevel`(RecoveryLevel) value of the secret
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::{KeyVaultClient, RecoveryLevel};
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     client.update_secret_recovery_level(&"SECRET_NAME", &"", RecoveryLevel::Purgeable).await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn update_secret_recovery_level(
        &mut self,
        secret_name: &'a str,
        secret_version: &'a str,
        recovery_level: RecoveryLevel,
    ) -> Result<(), KeyVaultError> {
        let mut attributes = Map::new();
        attributes.insert(
            "enabled".to_owned(),
            Value::String(recovery_level.to_string()),
        );

        self.update_secret(secret_name, secret_version, attributes)
            .await?;

        Ok(())
    }

    /// Updates the expiration time of a secret version.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - Name of the secret
    /// * `secret_version` - Version of the secret. Use an empty string for the latest version
    /// * `expiration_time - New expiration time of the secret
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::{KeyVaultClient, RecoveryLevel};
    /// use tokio::runtime::Runtime;
    /// use chrono::{Utc, Duration};
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     client.update_secret_expiration_time(&"SECRET_NAME", &"", Utc::now() + Duration::days(14)).await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn update_secret_expiration_time(
        &mut self,
        secret_name: &'a str,
        secret_version: &'a str,
        expiration_time: DateTime<Utc>,
    ) -> Result<(), KeyVaultError> {
        let mut attributes = Map::new();
        attributes.insert(
            "exp".to_owned(),
            Value::Number(serde_json::Number::from(expiration_time.timestamp())),
        );

        self.update_secret(secret_name, secret_version, attributes)
            .await?;

        Ok(())
    }

    async fn update_secret(
        &mut self,
        secret_name: &'a str,
        secret_version: &'a str,
        attributes: Map<String, Value>,
    ) -> Result<(), KeyVaultError> {
        let uri = Url::parse_with_params(
            &format!(
                "{}/secrets/{}/{}",
                self.keyvault_endpoint, secret_name, secret_version
            ),
            &[("api-version", API_VERSION)],
        )
        .unwrap();

        let mut request_body = Map::new();
        request_body.insert("attributes".to_owned(), Value::Object(attributes));

        self.patch_authed(uri.to_string(), Value::Object(request_body).to_string())
            .await?;

        Ok(())
    }

    /// Restores a backed up secret and all its versions.
    /// This operation requires the secrets/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     client.restore_secret(&"KUF6dXJlS2V5VmF1bHRTZWNyZXRCYWNrdXBWMS5taW").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn restore_secret(&mut self, backup_blob: &'a str) -> Result<(), KeyVaultError> {
        let uri = Url::parse_with_params(
            &format!("{}/secrets/restore", self.keyvault_endpoint),
            &[("api-version", API_VERSION)],
        )
        .unwrap();

        let mut request_body = Map::new();
        request_body.insert("value".to_owned(), Value::String(backup_blob.to_owned()));

        self.post_authed(
            uri.to_string(),
            Some(Value::Object(request_body).to_string()),
        )
        .await?;

        Ok(())
    }

    /// Restores a backed up secret and all its versions.
    /// This operation requires the secrets/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     client.backup_secret(&"SECRET_NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn backup_secret(
        &mut self,
        secret_name: &'a str,
    ) -> Result<KeyVaultSecretBackupBlob, KeyVaultError> {
        let uri = Url::parse_with_params(
            &format!("{}/secrets/{}/backup", self.keyvault_endpoint, secret_name),
            &[("api-version", API_VERSION)],
        )
        .unwrap();

        let response = self.post_authed(uri.to_string(), None).await?;
        let backup_blob = serde_json::from_str::<KeyVaultSecretBackupResponseRaw>(&response)
            .with_context(|| {
                format!(
                    "Failed to parse response from Key Vault when backing up secret {}: {}",
                    secret_name,
                    response.to_string()
                )
            })?;

        Ok(KeyVaultSecretBackupBlob {
            value: backup_blob.value,
        })
    }

    /// Deletes a secret in the Key Vault.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - Name of the secret
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::{KeyVaultClient, RecoveryLevel};
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let mut client = KeyVaultClient::new(
    ///     &"CLIENT_ID",
    ///     &"CLIENT_SECRET",
    ///     &"TENANT_ID",
    ///     &"KEYVAULT_NAME",
    ///     );
    ///     client.delete_secret(&"SECRET_NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn delete_secret(&mut self, secret_name: &'a str) -> Result<(), KeyVaultError> {
        let uri = Url::parse_with_params(
            &format!("{}/secrets/{}", self.keyvault_endpoint, secret_name),
            &[("api-version", API_VERSION)],
        )
        .unwrap();

        self.delete_authed(uri.to_string()).await?;

        Ok(())
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};
    use mockito::{mock, Matcher};
    use oauth2::AccessToken;
    use serde_json::json;

    fn diff(first: DateTime<Utc>, second: DateTime<Utc>) -> Duration {
        if first > second {
            first - second
        } else {
            second - first
        }
    }

    macro_rules! mock_client {
        ($keyvault_name:expr) => {{
            let mut client = KeyVaultClient::with_aad_token(
                &"",
                &"",
                &"TENANT_ID",
                $keyvault_name,
                AccessToken::new("TOKEN".to_owned()),
                Utc::now() + Duration::days(14),
            );
            client.keyvault_endpoint = mockito::server_url();
            client
        }};
    }

    #[tokio::test]
    async fn get_secret() {
        let time_created = Utc::now() - Duration::days(7);
        let time_updated = Utc::now();
        let _m = mock("GET", "/secrets/test-secret/")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": "secret-value",
                    "id": "https://test-keyvault.vault.azure.net/secrets/test-secret/4387e9f3d6e14c459867679a90fd0f79",
                    "attributes": {
                        "enabled": true,
                        "created": time_created.timestamp(),
                        "updated": time_updated.timestamp(),
                        "recoveryLevel": "Recoverable+Purgeable"
                    }
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let mut client = mock_client!(&"test-keyvault");

        let secret: KeyVaultSecret = client.get_secret(&"test-secret").await.unwrap();

        assert_eq!("secret-value", secret.value());
        assert_eq!(
            "https://test-keyvault.vault.azure.net/secrets/test-secret/4387e9f3d6e14c459867679a90fd0f79",
            secret.id()
        );
        assert_eq!(true, *secret.enabled());
        assert!(diff(time_created, *secret.time_created()) < Duration::seconds(1));
        assert!(diff(time_updated, *secret.time_updated()) < Duration::seconds(1));
    }

    #[tokio::test]
    async fn get_secret_versions() {
        let time_created_1 = Utc::now() - Duration::days(7);
        let time_updated_1 = Utc::now();
        let time_created_2 = Utc::now() - Duration::days(9);
        let time_updated_2 = Utc::now() - Duration::days(2);

        let _m1 = mock("GET", "/secrets/test-secret/versions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api-version".into(), API_VERSION.into()),
                Matcher::UrlEncoded("maxresults".into(), DEFAULT_MAX_RESULTS.to_string()),
            ]))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": [{
                        "id": "https://test-keyvault.vault.azure.net/secrets/test-secret/VERSION_1",
                        "attributes": {
                            "enabled": true,
                            "created": time_created_1.timestamp(),
                            "updated": time_updated_1.timestamp(),
                        }
                    }],
                    "nextLink": format!("{}/secrets/text-secret/versions?api-version={}&maxresults=1&$skiptoken=SKIP_TOKEN_MOCK", mockito::server_url().to_string(), API_VERSION)
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let _m2 = mock("GET", "/secrets/text-secret/versions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api-version".into(), API_VERSION.into()),
                Matcher::UrlEncoded("maxresults".into(), "1".into()),
                Matcher::UrlEncoded("$skiptoken".into(), "SKIP_TOKEN_MOCK".into()),
            ]))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": [{
                        "id": "https://test-keyvault.vault.azure.net/secrets/test-secret/VERSION_2",
                        "attributes": {
                            "enabled": true,
                            "created": time_created_2.timestamp(),
                            "updated": time_updated_2.timestamp(),
                        }
                    }],
                    "nextLink": null
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let mut client = mock_client!(&"test-keyvault");

        let secret_versions = client.get_secret_versions(&"test-secret").await.unwrap();

        let secret_1 = &secret_versions[0];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/secrets/test-secret/VERSION_1",
            secret_1.id()
        );
        assert!(diff(time_created_1, *secret_1.time_created()) < Duration::seconds(1));
        assert!(diff(time_updated_1, *secret_1.time_updated()) < Duration::seconds(1));

        let secret_2 = &secret_versions[1];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/secrets/test-secret/VERSION_2",
            secret_2.id()
        );
        assert!(diff(time_created_2, *secret_2.time_created()) < Duration::seconds(1));
        assert!(diff(time_updated_2, *secret_2.time_updated()) < Duration::seconds(1));
    }
}
