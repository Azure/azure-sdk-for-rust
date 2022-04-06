use crate::Error;
use azure_core::auth::{TokenCredential, TokenResponse};
use const_format::formatcp;
use url::Url;

pub(crate) const API_VERSION: &str = "2021-06-01-preview";
pub(crate) const API_VERSION_PARAM: &str = formatcp!("api-version={}", API_VERSION);

/// Client for Device Update operations - import, list and delete updates
///
/// # Example
///
/// ```no_run
/// use azure_device_update::DeviceUpdateClient;
/// use azure_identity::token_credentials::DefaultAzureCredential;
/// let creds = DefaultAzureCredential::default();
/// let client = DeviceUpdateClient::new("contoso.api.adu.microsoft.com", &creds).unwrap();
/// ```
#[derive(Debug)]
pub struct DeviceUpdateClient<'a, T> {
    pub(crate) device_update_url: Url,
    pub(crate) endpoint: String,
    pub(crate) token_credential: &'a T,
    pub(crate) token: Option<TokenResponse>,
}

impl<'a, T: TokenCredential> DeviceUpdateClient<'a, T> {
    /// Creates a new `DeviceUpdateClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_device_update::DeviceUpdateClient;
    /// use azure_identity::token_credentials::DefaultAzureCredential;
    /// let creds = DefaultAzureCredential::default();
    /// let client = DeviceUpdateClient::new("contoso.api.adu.microsoft.com", &creds).unwrap();
    /// ```
    pub fn new(device_update_url: &str, token_credential: &'a T) -> Result<Self, Error> {
        let device_update_url = Url::parse(device_update_url)?;
        let endpoint = extract_endpoint(&device_update_url)?;
        let client = DeviceUpdateClient {
            device_update_url,
            endpoint,
            token_credential,
            token: None,
        };
        Ok(client)
    }

    pub(crate) async fn refresh_token(&mut self) -> Result<(), Error> {
        if matches!(&self.token, Some(token) if token.expires_on > chrono::Utc::now()) {
            // Token is valid, return it.
            return Ok(());
        }

        let token = self
            .token_credential
            .get_token(&self.endpoint)
            .await
            .map_err(|_| Error::Authorization)?;
        self.token = Some(token);
        Ok(())
    }

    pub(crate) async fn get_authed(&mut self, uri: String) -> Result<String, Error> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .get(&uri)
            .bearer_auth(self.token.as_ref().unwrap().token.secret())
            .send()
            .await;
        let resp = match resp {
            Ok(r) => r,
            Err(_e) => {
                return Err(Error::InvalidOperationPath());
            }
        };
        let body = resp.text().await.unwrap();
        Ok(body)
    }

    pub(crate) async fn _put_authed(&mut self, uri: String, body: String) -> Result<String, Error> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .put(&uri)
            .bearer_auth(self.token.as_ref().unwrap().token.secret())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();
        let body = resp.text().await?;
        Ok(body)
    }

    pub(crate) async fn post_authed(
        &mut self,
        uri: String,
        json_body: Option<String>,
    ) -> Result<String, Error> {
        self.refresh_token().await?;

        let mut req = reqwest::Client::new()
            .post(&uri)
            .bearer_auth(self.token.as_ref().unwrap().token.secret());

        if let Some(body) = json_body {
            req = req.header("Content-Type", "application/json").body(body);
        } else {
            req = req.header("Content-Length", 0);
        }

        let resp = req.send().await?;
        if resp.status() == 202u16 {
            let headers = resp.headers();
            if headers.contains_key("operation-location") {
                return match headers.get("operation-location").unwrap().to_str() {
                    Ok(p) => Ok(p.to_owned()),
                    Err(_e) => Err(Error::InvalidOperationPath()),
                };
            } else {
                return Err(Error::NoOperationLocation);
            }
        }

        Err(Error::ImportError(resp.status()))
    }

    pub(crate) async fn _patch_authed(
        &mut self,
        uri: String,
        body: String,
    ) -> Result<String, Error> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .patch(&uri)
            .bearer_auth(self.token.as_ref().unwrap().token.secret())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        let body = resp.text().await.unwrap();

        let body_serialized = serde_json::from_str::<serde_json::Value>(&body).unwrap();

        if let Some(err) = body_serialized.get("error") {
            let msg = err.get("message").ok_or(Error::UnparsableError)?;
            Err(Error::General(msg.to_string()))
        } else {
            Ok(body)
        }
    }

    pub(crate) async fn delete_authed(&mut self, uri: String) -> Result<String, Error> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .delete(&uri)
            .bearer_auth(self.token.as_ref().unwrap().token.secret())
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();
        let body = resp.text().await.unwrap();
        Ok(body)
    }
}

/// Helper to get vault endpoint with a scheme and a trailing slash
/// ex. `https://vault.azure.net/` where the full client url is `https://myvault.vault.azure.net`
fn extract_endpoint(url: &Url) -> Result<String, Error> {
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
    fn can_extract_endpoint() {
        let suffix =
            extract_endpoint(&Url::parse("https://myadu.api.adu.microsoft.com").unwrap()).unwrap();
        assert_eq!(suffix, "https://api.adu.microsoft.com");

        let suffix =
            extract_endpoint(&Url::parse("https://myadu.mycustom.api.adu.server.net").unwrap())
                .unwrap();
        assert_eq!(suffix, "https://mycustom.api.adu.server.net");

        let suffix = extract_endpoint(&Url::parse("https://myadu.internal").unwrap()).unwrap();
        assert_eq!(suffix, "https://internal");

        let suffix =
            extract_endpoint(&Url::parse("some-scheme://myadu.api.adu.microsoft.com").unwrap())
                .unwrap();
        assert_eq!(suffix, "some-scheme://api.adu.microsoft.com");
    }
}
