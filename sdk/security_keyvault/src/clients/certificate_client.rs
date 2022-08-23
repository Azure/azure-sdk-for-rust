use crate::prelude::*;
use azure_core::auth::TokenCredential;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CertificateClient {
    pub(crate) keyvault_client: KeyvaultClient,
}

impl CertificateClient {
    pub fn new(
        vault_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let keyvault_client = KeyvaultClient::new(vault_url, token_credential)?;
        Ok(Self::new_with_client(keyvault_client))
    }

    pub(crate) fn new_with_client(keyvault_client: KeyvaultClient) -> Self {
        Self { keyvault_client }
    }

    /// Gets a certificate from the Key Vault.
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
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificate = client.get("NAME").into_future().await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn get<N>(&self, name: N) -> GetCertificateBuilder
    where
        N: Into<String>,
    {
        GetCertificateBuilder::new(self.clone(), name.into())
    }

    /// Gets all the versions for a certificate in the Key Vault.
    //
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use futures::StreamExt;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificate_versions = client.get_versions("NAME").into_stream().next().await.unwrap();
    ///     dbg!(&certificate_versions);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn get_versions<N>(&self, name: N) -> GetCertificateVersionsBuilder
    where
        N: Into<String>,
    {
        GetCertificateVersionsBuilder::new(self.clone(), name.into())
    }

    pub fn update<N>(&self, name: N) -> UpdateCertificatePropertiesBuilder
    where
        N: Into<String>,
    {
        UpdateCertificatePropertiesBuilder::new(self.clone(), name.into())
    }

    /// Restores a backed up certificate and all its versions.
    /// This operation requires the certificates/restore permission.
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
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     client.backup("NAME").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn backup<N>(&self, name: N) -> CertificateBackupBuilder
    where
        N: Into<String>,
    {
        CertificateBackupBuilder::new(self.clone(), name.into())
    }

    /// Deletes a certificate in the Key Vault.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the certificate
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
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     client.delete("NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn delete<N>(&self, name: N) -> azure_core::Result<()>
    where
        N: Into<String>,
    {
        // let mut uri = self.vault_url.clone();
        // uri.set_path(&format!("certificates/{}", certificate_name));

        // self.delete_authed(uri.to_string()).await?;

        // Ok(())

        let _name = name.into();

        todo!("See issue #174 at: https://github.com/Azure/azure-sdk-for-rust/issues/174.")
    }

    /// Lists all the certificates in the Key Vault.
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
    ///     let mut client = KeyvaultClient::new(
    ///          &"KEYVAULT_URL",
    ///          Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificates = client.list_certificates().into_stream().next().await.unwrap();
    ///     dbg!(&certificates);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn list_certificates(&self) -> ListCertificatesBuilder {
        ListCertificatesBuilder::new(self.clone())
    }

    /// Restores a backed up certificate and all its versions.
    /// This operation requires the certificates/restore permission.
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
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     client.restore_certificate("KUF6dXJlS2V5VmF1bHRTZWNyZXRCYWNrdXBWMS5taW").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn restore_certificate<S>(&self, backup_blob: S) -> RestoreCertificateBuilder
    where
        S: Into<String>,
    {
        RestoreCertificateBuilder::new(self.clone(), backup_blob.into())
    }
}
