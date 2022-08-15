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
    /// use futures::StreamExt;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     let secret_versions = client.get_versions("SECRET_NAME").into_stream().next().await.unwrap();
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
    /// use futures::stream::StreamExt;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///     &"KEYVAULT_URL",
    ///     std::sync::Arc::new(creds),
    ///     ).unwrap().secret_client();
    ///     let secrets = client.list_secrets().into_stream().next().await;
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
