use crate::prelude::*;
use azure_core::auth::TokenCredential;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SecretClient {
    pub(crate) client: KeyvaultClient,
}

impl SecretClient {
    pub fn new(
        vault_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let client = KeyvaultClient::new(vault_url, token_credential)?;
        Ok(Self { client })
    }

    pub(crate) fn new_with_client(client: KeyvaultClient) -> Self {
        Self { client }
    }

    /// Gets a secret from the Key Vault.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     let secret = client.get("SECRET_NAME").into_future().await.unwrap();
    ///     dbg!(&secret);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn get<N>(&self, name: N) -> GetSecretBuilder
    where
        N: Into<String>,
    {
        GetSecretBuilder::new(self.clone(), name.into())
    }

    /// Sets the value of a secret in the Key Vault.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     client.set("SECRET_NAME", "NEW_VALUE").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn set<N, S>(&self, name: N, value: S) -> SetSecretBuilder
    where
        N: Into<String>,
        S: Into<String>,
    {
        SetSecretBuilder::new(self.clone(), name.into(), value.into())
    }

    /// Updates the metadata associated with a secret
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     client.update("SECRET_NAME").enabled(false).into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn update<N>(&self, name: N) -> UpdateSecretBuilder
    where
        N: Into<String>,
    {
        UpdateSecretBuilder::new(self.clone(), name.into())
    }

    /// Gets all the versions for a secret in the Key Vault.
    //
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     let secret_versions = client.get_versions("SECRET_NAME").into_future().await.unwrap();
    ///     dbg!(&secret_versions);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn get_versions<N>(&self, name: N) -> GetSecretVersionsBuilder
    where
        N: Into<String>,
    {
        GetSecretVersionsBuilder::new(self.clone(), name.into())
    }

    /// Restores a backed up secret and all its versions.
    /// This operation requires the secrets/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     client.backup("SECRET_NAME").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn backup<N>(&self, name: N) -> BackupSecretBuilder
    where
        N: Into<String>,
    {
        BackupSecretBuilder::new(self.clone(), name.into())
    }

    /// Deletes a secret in the Key Vault.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     client.delete("SECRET_NAME").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn delete<N>(&self, name: N) -> DeleteSecretBuilder
    where
        N: Into<String>,
    {
        DeleteSecretBuilder::new(self.clone(), name.into())
    }

    /// Lists all the secrets in the Key Vault.
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     std::sync::Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     let secrets = client.list_secrets().into_future().await.unwrap();
    ///     dbg!(&secrets);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn list_secrets(&self) -> ListSecretsBuilder {
        ListSecretsBuilder::new(self.clone())
    }

    /// Restores a backed up secret and all its versions.
    /// This operation requires the secrets/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     std::sync::Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     client.restore_secret("KUF6dXJlS2V5VmF1bHRTZWNyZXRCYWNrdXBWMS5taW").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn restore_secret<S>(&self, backup_blob: S) -> RestoreSecretBuilder
    where
        S: Into<String>,
    {
        RestoreSecretBuilder::new(self.clone(), backup_blob.into())
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use crate::mock_client;
    use crate::prelude::API_VERSION;
    use crate::tests::MockCredential;
    use azure_core::date;
    use mockito::{mock, Matcher};
    use serde_json::json;
    use std::time::Duration;
    use time::OffsetDateTime;

    #[tokio::test]
    async fn get_secret() -> azure_core::Result<()> {
        let time_created = OffsetDateTime::now_utc() - date::duration_from_days(7);
        let time_updated = OffsetDateTime::now_utc();
        let _m = mock("GET", "/secrets/test-secret/")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": "secret-value",
                    "id": "https://test-keyvault.vault.azure.net/secrets/test-secret/4387e9f3d6e14c459867679a90fd0f79",
                    "attributes": {
                        "enabled": true,
                        "created": time_created.unix_timestamp(),
                        "updated": time_updated.unix_timestamp(),
                        "recoveryLevel": "Recoverable+Purgeable"
                    }
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential::new();
        dbg!(mockito::server_url());
        let client = mock_client!(&"test-keyvault", creds);
        let secret_client = client.secret_client();

        let secret = secret_client.get("test-secret").into_future().await?;

        assert_eq!("secret-value", secret.value);
        assert_eq!(
            "https://test-keyvault.vault.azure.net/secrets/test-secret/4387e9f3d6e14c459867679a90fd0f79",
            secret.id
        );
        assert!(date::diff(time_created, secret.created_on) < Duration::from_secs(1));
        assert!(date::diff(time_updated, secret.updated_on) < Duration::from_secs(1));
        Ok(())
    }

    #[tokio::test]
    async fn get_secret_versions() -> azure_core::Result<()> {
        let time_created_1 = OffsetDateTime::now_utc() - date::duration_from_days(7);
        let time_updated_1 = OffsetDateTime::now_utc();
        let time_created_2 = OffsetDateTime::now_utc() - date::duration_from_days(9);
        let time_updated_2 = OffsetDateTime::now_utc() - date::duration_from_days(2);

        let _m1 = mock("GET", "/secrets/test-secret/versions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api-version".into(), API_VERSION.into()),
                // Matcher::UrlEncoded("maxresults".into(), DEFAULT_MAX_RESULTS.to_string()),
            ]))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": [{
                        "id": "https://test-keyvault.vault.azure.net/secrets/test-secret/VERSION_1",
                        "attributes": {
                            "enabled": true,
                            "created": time_created_1.unix_timestamp(),
                            "updated": time_updated_1.unix_timestamp(),
                        }
                    }],
                    "nextLink": format!("{}/secrets/text-secret/versions?api-version={}&maxresults=1&$skiptoken=SKIP_TOKEN_MOCK", mockito::server_url(), API_VERSION)
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
                            "created": time_created_2.unix_timestamp(),
                            "updated": time_updated_2.unix_timestamp(),
                        }
                    }],
                    "nextLink": null
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential::new();
        let secret_client = mock_client!(&"test-keyvault", creds).secret_client();

        let secret_versions = secret_client
            .get_versions("test-secret")
            .into_future()
            .await?;

        let secret_1 = &secret_versions[0];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/secrets/test-secret/VERSION_1",
            secret_1.id
        );
        assert!(date::diff(time_created_1, secret_1.created_on) < Duration::from_secs(1));
        assert!(date::diff(time_updated_1, secret_1.updated_on) < Duration::from_secs(1));

        let secret_2 = &secret_versions[1];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/secrets/test-secret/VERSION_2",
            secret_2.id
        );
        assert!(date::diff(time_created_2, secret_2.created_on) < Duration::from_secs(1));
        assert!(date::diff(time_updated_2, secret_2.updated_on) < Duration::from_secs(1));
        Ok(())
    }
}
