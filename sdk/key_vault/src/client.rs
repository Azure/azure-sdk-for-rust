use crate::Error;
use azure_core::{TokenCredential, TokenResponse};
use const_format::formatcp;
use url::Url;

pub(crate) const API_VERSION: &str = "7.0";
pub(crate) const API_VERSION_PARAM: &str = formatcp!("api-version={}", API_VERSION);

/// Client for Key Vault operations - getting a secret, listing secrets, etc.
///
/// # Example
///
/// ```no_run
/// use azure_key_vault::KeyClient;
/// use azure_identity::token_credentials::DefaultCredential;
/// let creds = DefaultCredential::default();
/// let client = KeyClient::new(&"https://test-key-vault.vault.azure.net", &creds).unwrap();
/// ```
#[derive(Debug)]
pub struct KeyClient<'a, T> {
    pub(crate) vault_url: Url,
    pub(crate) endpoint: String,
    pub(crate) token_credential: &'a T,
    pub(crate) token: Option<TokenResponse>,
}

impl<'a, T: TokenCredential> KeyClient<'a, T> {
    /// Creates a new `KeyClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_key_vault::KeyClient;
    /// use azure_identity::token_credentials::DefaultCredential;
    /// let creds = DefaultCredential::default();
    /// let client = KeyClient::new("test-key-vault.vault.azure.net", &creds).unwrap();
    /// ```
    pub fn new(vault_url: &str, token_credential: &'a T) -> Result<Self, Error> {
        let vault_url = Url::parse(vault_url)?;
        let endpoint = extract_endpoint(&vault_url)?;
        let client = KeyClient {
            vault_url,
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
            .await
            .unwrap();
        let body = resp.text().await.unwrap();
        Ok(body)
    }

    pub(crate) async fn put_authed(&mut self, uri: String, body: String) -> Result<String, Error> {
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

        let body = resp.text().await?;

        let body_serialized = serde_json::from_str::<serde_json::Value>(&body).unwrap();

        if let Some(err) = body_serialized.get("error") {
            let msg = err.get("message").ok_or(Error::UnparsableError)?;
            Err(Error::General(msg.to_string()))
        } else {
            Ok(body)
        }
    }

    pub(crate) async fn patch_authed(
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
    Ok(format!("{}://{}/", url.scheme(), endpoint))
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
