use crate::prelude::*;
use azure_core::auth::{TokenCredential, TokenResponse};
use azure_core::error::{Error, ErrorKind, ResultExt};
use const_format::formatcp;
use std::sync::Arc;
use time::OffsetDateTime;
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
            || matches!(&self.token, Some(token) if token.expires_on < OffsetDateTime::now_utc())
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

    pub fn secret_client(&self) -> SecretClient {
        SecretClient::new_with_client(self.clone())
    }

    pub fn certificate_client(&self) -> CertificateClient {
        CertificateClient::new_with_client(self.clone())
    }

    pub fn key_client(&self) -> KeyClient {
        KeyClient::new_with_client(self.clone())
    }
}

/// Helper to get vault endpoint with a scheme and a trailing slash
/// ex. `https://vault.azure.net/` where the full client url is `https://myvault.vault.azure.net`
fn extract_endpoint(url: &Url) -> azure_core::Result<String> {
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
