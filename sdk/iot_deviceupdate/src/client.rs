use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
    from_json, Url,
};
use const_format::formatcp;
use serde::de::DeserializeOwned;
use std::sync::Arc;

pub(crate) const API_VERSION: &str = "2022-10-01";
pub(crate) const API_VERSION_PARAM: &str = formatcp!("api-version={}", API_VERSION);

/// Client for Device Update operations - import, list and delete updates
///
/// # Example
///
/// ```no_run
/// use azure_iot_deviceupdate::DeviceUpdateClient;
/// let credential = azure_identity::create_credential().unwrap();
/// let client = DeviceUpdateClient::new("contoso.api.adu.microsoft.com", credential).unwrap();
/// ```

#[derive(Clone)]
pub struct DeviceUpdateClient {
    pub(crate) device_update_url: Url,
    pub(crate) token_credential: Arc<dyn TokenCredential>,
}

impl DeviceUpdateClient {
    /// Creates a new `DeviceUpdateClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_iot_deviceupdate::DeviceUpdateClient;
    /// let credential = azure_identity::create_credential().unwrap();
    /// let client = DeviceUpdateClient::new("contoso.api.adu.microsoft.com", credential).unwrap();
    /// ```
    pub fn new(
        device_update_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let device_update_url = Url::parse(device_update_url)
            .with_context(ErrorKind::DataConversion, || {
                format!("failed to parse update url: {device_update_url}")
            })?;

        let client = DeviceUpdateClient {
            device_update_url,
            token_credential,
        };
        Ok(client)
    }

    async fn get_token(&self) -> azure_core::Result<AccessToken> {
        self.token_credential
            .get_token(&["https://api.adu.microsoft.com/.default"])
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
        from_json(&body)
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
                Some(location) => location.to_str().map(ToString::to_string).context(
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
