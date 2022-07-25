use crate::prelude::*;
use azure_core::auth::{TokenCredential, TokenResponse};
use azure_core::error::{Error, ErrorKind, ResultExt};
use const_format::formatcp;
use std::sync::Arc;
use url::Url;

pub const API_VERSION: &str = "7.0";
pub(crate) const API_VERSION_PARAM: &str = formatcp!("api-version={}", API_VERSION);

/// Client for Key Vault operations - getting a secret, listing secrets, etc.
///
/// # Example
///
/// ```no_run
/// use azure_security_keyvault::KeyvaultClient;
/// use azure_identity::DefaultAzureCredential;
/// let creds = DefaultAzureCredential::default();
/// let client = KeyvaultClient::new(&"https://test-key-vault.vault.azure.net", std::sync::Arc::new(creds)).unwrap();
/// ```
#[derive(Clone)]
pub struct KeyvaultClient {
    pub(crate) vault_url: Url,
    pub(crate) endpoint: String,
    pub(crate) token_credential: Arc<dyn TokenCredential>,
    pub(crate) token: Option<TokenResponse>,
}

impl std::fmt::Debug for KeyvaultClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyvaultClient")
            .field("vault_url", &self.vault_url)
            .field("endpoint", &self.endpoint)
            .finish_non_exhaustive()
    }
}

impl KeyvaultClient {
    /// Creates a new `KeyClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use std::sync::Arc;
    /// let creds = Arc::new(DefaultAzureCredential::default());
    /// let client = KeyvaultClient::new("test-key-vault.vault.azure.net", creds).unwrap();
    /// ```
    pub fn new(
        vault_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let vault_url = Url::parse(vault_url)?;
        let endpoint = extract_endpoint(&vault_url)?;
        let client = Self {
            vault_url,
            endpoint,
            token_credential,
            token: None,
        };
        Ok(client)
    }

    pub(crate) async fn request(
        &mut self,
        method: reqwest::Method,
        uri: String,
        body: Option<String>,
    ) -> azure_core::Result<String> {
        self.get_token().await?;

        let mut req = reqwest::Client::new()
            .request(method, &uri)
            .bearer_auth(self.token.as_ref().unwrap().token.secret());

        if let Some(body) = body {
            req = req.header("content-type", "application/json").body(body);
        } else {
            req = req.header("content-length", 0);
        }

        let resp = req
            .send()
            .await
            .with_context(ErrorKind::Io, || {
                format!("failed to send request. uri: {uri}")
            })?
            .error_for_status()
            .with_context(ErrorKind::Io, || {
                format!("failed to read response body text. uri: {uri}")
            })?;

        let body = resp.text().await.with_context(ErrorKind::Io, || {
            format!("failed to read response body text. uri: {uri}")
        })?;
        Ok(body)
    }

    pub(crate) async fn get_token(&mut self) -> azure_core::Result<&str> {
        if self.token.is_none()
            || matches!(&self.token, Some(token) if token.expires_on < chrono::Utc::now())
        {
            let token = self
                .token_credential
                .get_token(&self.endpoint)
                .await
                .context(ErrorKind::Credential, "get token failed")?;

            self.token = Some(token);
        }
        Ok(self.token.as_ref().unwrap().token.secret())
    }

    pub fn secret_client<S>(&self, name: S) -> SecretClient
    where
        S: Into<String>,
    {
        SecretClient::new(self.clone(), name.into())
    }

    pub fn certificate_client<S>(&self, name: S) -> CertificateClient
    where
        S: Into<String>,
    {
        CertificateClient::new(self.clone(), name.into())
    }

    pub fn key_client<S>(&self, name: S) -> KeyClient
    where
        S: Into<String>,
    {
        KeyClient::new(self.clone(), name.into())
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
    ///     ).unwrap();
    ///     let secrets = client.list_secrets().into_future().await.unwrap();
    ///     dbg!(&secrets);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn list_secrets(&self) -> ListSecretsBuilder {
        ListSecretsBuilder::new(self.clone())
    }

    /// Lists all the certificates in the Key Vault.
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
    ///          &"KEYVAULT_URL",
    ///          Arc::new(creds),
    ///     ).unwrap();
    ///     let certificates = client.list_certificates().into_future().await.unwrap();
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
    ///     ).unwrap();
    ///     client.restore_certificate("KUF6dXJlS2V5VmF1bHRTZWNyZXRCYWNrdXBWMS5taW").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn restore_certificate<S>(&mut self, backup_blob: S) -> RestoreCertificateBuilder
    where
        S: Into<String>,
    {
        RestoreCertificateBuilder::new(self.clone(), backup_blob.into())
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
    ///     ).unwrap();
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

/// Helper to get vault endpoint with a scheme and a trailing slash
/// ex. `https://vault.azure.net/` where the full client url is `https://myvault.vault.azure.net`
pub(crate) fn extract_endpoint(url: &Url) -> azure_core::Result<String> {
    let endpoint = url
        .host_str()
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("failed to parse host from url. url: {url}")
            })
        })?
        .splitn(2, '.') // FIXME: replace with split_once() when it is in stable
        .last()
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("failed to extract endpoint from url. url: {url}")
            })
        })?;
    Ok(format!("{}://{}", url.scheme(), endpoint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_extract_endpoint() {
        let suffix =
            extract_endpoint(&Url::parse("https://myvault.vault.azure.net").unwrap()).unwrap();
        assert_eq!(suffix, "https://vault.azure.net");

        let suffix =
            extract_endpoint(&Url::parse("https://myvault.mycustom.vault.server.net").unwrap())
                .unwrap();
        assert_eq!(suffix, "https://mycustom.vault.server.net");

        let suffix = extract_endpoint(&Url::parse("https://myvault.internal").unwrap()).unwrap();
        assert_eq!(suffix, "https://internal");

        let suffix =
            extract_endpoint(&Url::parse("some-scheme://myvault.vault.azure.net").unwrap())
                .unwrap();
        assert_eq!(suffix, "some-scheme://vault.azure.net");
    }
}
