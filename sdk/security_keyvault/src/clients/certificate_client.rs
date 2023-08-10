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
    ///     let certificate = client.get("NAME").await.unwrap();
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

    /// Gets the creation operation of a certificate.
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
    ///     let certificate = client.get_operation("NAME").await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn get_operation<N>(&self, name: N) -> GetCertificateOperationBuilder
    where
        N: Into<String>,
    {
        GetCertificateOperationBuilder::new(self.clone(), name.into())
    }

    /// Creates a new certificate.
    /// If this is the first version, the certificate resource is created.
    /// This operation requires the certificates/create permission.
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
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificate = client.create("NAME", "SUBJECT", "ISSUER").await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn create<N, S, I>(&self, name: N, subject: S, issuer_name: I) -> CreateCertificateBuilder
    where
        N: Into<String>,
        S: Into<String>,
        I: Into<String>,
    {
        CreateCertificateBuilder::new(
            self.clone(),
            name.into(),
            subject.into(),
            issuer_name.into(),
        )
    }

    /// Merges a certificate or a certificate chain with a key pair existing on the server.
    /// The `MergeCertificate` operation performs the merging of a certificate or certificate chain with a key
    /// pair currently available in the service. This operation requires the certificates/create permission.
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
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificate = client.merge("NAME", vec![String::from("X5C")]).await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn merge<N, V>(&self, name: N, x5c: V) -> MergeCertificateBuilder
    where
        N: Into<String>,
        V: Into<Vec<String>>,
    {
        MergeCertificateBuilder::new(self.clone(), name.into(), x5c.into())
    }

    /// Imports a certificate into a specified key vault.
    /// This operation requires the certificates/import permission. The certificate to be imported can be in either PFX
    /// or PEM format. If the certificate is in PEM format the PEM file must contain the key as well as x509 certificates.
    /// Key Vault will only accept a key in PKCS#8 format.
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
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificate = client.import("NAME", "VALUE").pwd("pwd").await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn import<N, V>(&self, name: N, value: V) -> ImportCertificateBuilder
    where
        N: Into<String>,
        V: Into<String>,
    {
        ImportCertificateBuilder::new(self.clone(), name.into(), value.into())
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
    ///     client.backup("NAME").await.unwrap();
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
    pub fn delete<N>(&self, name: N) -> DeleteCertificateBuilder
    where
        N: Into<String>,
    {
        DeleteCertificateBuilder::new(self.clone(), name.into())
    }

    /// Deletes the creation operation for a specific certificate.
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
    ///     client.delete_operation("NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn delete_operation<N>(&self, name: N) -> DeleteCertificateOperationBuilder
    where
        N: Into<String>,
    {
        DeleteCertificateOperationBuilder::new(self.clone(), name.into())
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
    ///     client.restore_certificate("KUF6dXJlS2V5VmF1bHRTZWNyZXRCYWNrdXBWMS5taW").await.unwrap();
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
