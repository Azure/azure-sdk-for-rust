use crate::{Error, Result};
use azure_core::{
    auth::{TokenCredential, TokenResponse},
    Error as CoreError, HttpError,
};
use azure_identity::AutoRefreshingTokenCredential;
use const_format::formatcp;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use url::Url;

pub(crate) const API_VERSION: &str = "2021-06-01-preview";
pub(crate) const API_VERSION_PARAM: &str = formatcp!("api-version={}", API_VERSION);

/// Client for Device Update operations - import, list and delete updates
///
/// # Example
///
/// ```no_run
/// use azure_device_update::DeviceUpdateClient;
/// use azure_identity::DefaultAzureCredential;
/// let creds = std::sync::Arc::new(DefaultAzureCredential::default());
/// let client = DeviceUpdateClient::new("contoso.api.adu.microsoft.com", creds).unwrap();
/// ```

#[derive(Clone)]
pub struct DeviceUpdateClient {
    pub(crate) device_update_url: Url,
    pub(crate) endpoint: String,
    pub(crate) token_credential: AutoRefreshingTokenCredential,
}

impl DeviceUpdateClient {
    /// Creates a new `DeviceUpdateClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_device_update::DeviceUpdateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// let creds = std::sync::Arc::new(DefaultAzureCredential::default());
    /// let client = DeviceUpdateClient::new("contoso.api.adu.microsoft.com", creds).unwrap();
    /// ```
    pub fn new(
        device_update_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> Result<Self> {
        let device_update_url =
            Url::parse(device_update_url).map_err(|e| CoreError::Http(HttpError::Url(e)))?;
        let endpoint = extract_endpoint(&device_update_url)?;
        let token_credential = AutoRefreshingTokenCredential::new(token_credential);

        let client = DeviceUpdateClient {
            device_update_url,
            endpoint,
            token_credential,
        };
        Ok(client)
    }

    async fn get_token(&self) -> Result<TokenResponse> {
        self.token_credential
            .get_token(&self.endpoint)
            .await
            .map_err(Error::Core)
    }

    pub(crate) async fn get<R>(&self, uri: String) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let resp = reqwest::Client::new()
            .get(&uri)
            .bearer_auth(self.get_token().await?.token.secret())
            .send()
            .await
            .map_err(|e| Error::Core(CoreError::Http(HttpError::ExecuteRequest(e.into()))))?;

        let body = resp
            .bytes()
            .await
            .map_err(|e| Error::Core(CoreError::Http(HttpError::ReadBytes(e.into()))))?;
        serde_json::from_slice(&body).map_err(|e| Error::Core(CoreError::Json(e)))
    }

    pub(crate) async fn post(&self, uri: String, json_body: Option<String>) -> Result<String> {
        let mut req = reqwest::Client::new()
            .post(&uri)
            .bearer_auth(self.get_token().await?.token.secret());

        if let Some(body) = json_body {
            req = req.header("Content-Type", "application/json").body(body);
        } else {
            req = req.header("Content-Length", 0);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| Error::Core(CoreError::Http(HttpError::ExecuteRequest(e.into()))))?;

        if resp.status() == 202u16 {
            let headers = resp.headers();
            return match headers.get("operation-location") {
                Some(location) => location
                    .to_str()
                    .map(|x| x.to_string())
                    .map_err(|_| Error::InvalidOperationPath),
                None => Err(Error::NoOperationLocation),
            };
        }

        Err(Error::ImportError(resp.status()))
    }

    pub(crate) async fn delete(&self, uri: String) -> Result<String> {
        let resp = reqwest::Client::new()
            .delete(&uri)
            .bearer_auth(self.get_token().await?.token.secret())
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| Error::Core(CoreError::Http(HttpError::ExecuteRequest(e.into()))))?;
        let body = resp
            .text()
            .await
            .map_err(|e| Error::Core(CoreError::Http(HttpError::ReadBytes(e.into()))))?;
        Ok(body)
    }
}

/// Helper to get vault endpoint with a scheme and a trailing slash
/// ex. `https://vault.azure.net/` where the full client url is `https://myvault.vault.azure.net`
fn extract_endpoint(url: &Url) -> Result<String> {
    let endpoint = url
        .host_str()
        .ok_or(Error::DomainParse)?
        .splitn(2, '.') // FIXME: replace with split_once() when it is in stable
        .last()
        .ok_or(Error::DomainParse)?;
    Ok(format!("{}://{}", url.scheme(), endpoint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_extract_endpoint() -> Result<()> {
        let suffix = extract_endpoint(
            &Url::parse("https://myadu.api.adu.microsoft.com")
                .map_err(|e| CoreError::Http(HttpError::Url(e)))?,
        )?;
        assert_eq!(suffix, "https://api.adu.microsoft.com");

        let suffix = extract_endpoint(
            &Url::parse("https://myadu.mycustom.api.adu.server.net")
                .map_err(|e| CoreError::Http(HttpError::Url(e)))?,
        )?;
        assert_eq!(suffix, "https://mycustom.api.adu.server.net");

        let suffix = extract_endpoint(
            &Url::parse("https://myadu.internal")
                .map_err(|e| CoreError::Http(HttpError::Url(e)))?,
        )?;
        assert_eq!(suffix, "https://internal");

        let suffix = extract_endpoint(
            &Url::parse("some-scheme://myadu.api.adu.microsoft.com")
                .map_err(|e| CoreError::Http(HttpError::Url(e)))?,
        )?;
        assert_eq!(suffix, "some-scheme://api.adu.microsoft.com");
        Ok(())
    }
}
