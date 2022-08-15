use azure_core::{
    auth::{TokenCredential, TokenResponse},
    error::{Error, ErrorKind, ResultExt},
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
/// use azure_iot_deviceupdate::DeviceUpdateClient;
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
    /// use azure_iot_deviceupdate::DeviceUpdateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// let creds = std::sync::Arc::new(DefaultAzureCredential::default());
    /// let client = DeviceUpdateClient::new("contoso.api.adu.microsoft.com", creds).unwrap();
    /// ```
    pub fn new(
        device_update_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let device_update_url = Url::parse(device_update_url)
            .with_context(ErrorKind::DataConversion, || {
                format!("failed to parse update url: {device_update_url}")
            })?;
        let endpoint = extract_endpoint(&device_update_url)?;
        let token_credential = AutoRefreshingTokenCredential::new(token_credential);

        let client = DeviceUpdateClient {
            device_update_url,
            endpoint,
            token_credential,
        };
        Ok(client)
    }

    async fn get_token(&self) -> azure_core::Result<TokenResponse> {
        self.token_credential
            .get_token(&self.endpoint)
            .await
            .context(ErrorKind::Credential, "get token failed")
    }

    pub(crate) async fn get<R>(&self, uri: String) -> azure_core::Result<R>
    where
        R: DeserializeOwned,
    {
        let resp = reqwest::Client::new()
            .get(&uri)
            .bearer_auth(self.get_token().await?.token.secret())
            .send()
            .await
            .with_context(ErrorKind::Io, || {
                format!("failed to send request. uri: {uri}")
            })?;

        let body = resp.bytes().await.with_context(ErrorKind::Io, || {
            format!("failed to read response body text. uri: {uri}")
        })?;
        serde_json::from_slice(&body).context(
            ErrorKind::DataConversion,
            "failed to deserialize json response body",
        )
    }

    pub(crate) async fn post(
        &self,
        uri: String,
        json_body: Option<String>,
    ) -> azure_core::Result<String> {
        let mut req = reqwest::Client::new()
            .post(&uri)
            .bearer_auth(self.get_token().await?.token.secret());

        if let Some(body) = json_body {
            req = req.header("content-type", "application/json").body(body);
        } else {
            req = req.header("content-length", 0);
        }

        let resp = req.send().await.with_context(ErrorKind::Io, || {
            format!("failed to send request. uri: {uri}")
        })?;

        if resp.status() == 202u16 {
            let headers = resp.headers();
            return match headers.get("operation-location") {
                Some(location) => location.to_str().map(|x| x.to_string()).context(
                    ErrorKind::Other,
                    "invalid characters in operation-location path",
                ),
                None => Err(Error::message(
                    ErrorKind::Other,
                    "successful import (202 status) but no operation-location header found",
                )),
            };
        }

        Err(Error::with_message(ErrorKind::Other, || {
            format!("import unsuccessful, status: {}", resp.status())
        }))
    }

    pub(crate) async fn delete(&self, uri: String) -> azure_core::Result<String> {
        let resp = reqwest::Client::new()
            .delete(&uri)
            .bearer_auth(self.get_token().await?.token.secret())
            .header("content-type", "application/json")
            .send()
            .await
            .with_context(ErrorKind::Io, || {
                format!("failed to send delete request. uri: {uri}")
            })?;
        let body = resp.text().await.with_context(ErrorKind::Io, || {
            format!("failed to read response body text. uri: {uri}")
        })?;
        Ok(body)
    }
}

/// Helper to get vault endpoint with a scheme and a trailing slash
/// ex. `https://vault.azure.net/` where the full client url is `https://myvault.vault.azure.net`
fn extract_endpoint(url: &Url) -> azure_core::Result<String> {
    let endpoint = url
        .host_str()
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("could not get device update domain. url: {url}")
            })
        })?
        .split_once('.')
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("could not parse device update domain. url: {url}")
            })
        })?
        .1;
    Ok(format!("{}://{}", url.scheme(), endpoint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_extract_endpoint() -> azure_core::Result<()> {
        let url = "https://myadu.api.adu.microsoft.com";
        let suffix = extract_endpoint(&Url::parse(url)?)?;
        assert_eq!(suffix, "https://api.adu.microsoft.com");

        let suffix = extract_endpoint(&Url::parse("https://myadu.mycustom.api.adu.server.net")?)?;
        assert_eq!(suffix, "https://mycustom.api.adu.server.net");

        let suffix = extract_endpoint(&Url::parse("https://myadu.internal")?)?;
        assert_eq!(suffix, "https://internal");

        let suffix = extract_endpoint(&Url::parse("some-scheme://myadu.api.adu.microsoft.com")?)?;
        assert_eq!(suffix, "some-scheme://api.adu.microsoft.com");
        Ok(())
    }

    #[test]
    fn can_not_extract_endpoint() -> azure_core::Result<()> {
        let url = "https://shouldfail";
        let suffix = extract_endpoint(&Url::parse(url)?);
        assert_eq!(suffix.unwrap_err().kind(), &ErrorKind::DataConversion);
        Ok(())
    }
}
