use crate::KeyVaultError;
use anyhow::Context;
use anyhow::Result;
use azure_auth_aad::{TokenCredential, TokenResponse};
use chrono::{DateTime, Utc};
use oauth2::{AccessToken, ClientId, ClientSecret};
use std::sync::Arc;

pub(crate) const PUBLIC_ENDPOINT_SUFFIX: &str = "vault.azure.net";
pub(crate) const API_VERSION: &str = "7.0";

/// Client for Key Vault operations - getting a secret, listing secrets, etc.
///
/// # Example
///
/// ```no_run
/// use azure_keyvault::KeyVaultClient;
/// let client = KeyVaultClient::new(&"{client_id}", &"{client_secret}", &"{tenant_id}", &"test-keyvault");
/// ```
#[derive(Debug)]
pub struct KeyVaultClient<'a, T> {
    pub(crate) token_credential: &'a T,
    pub(crate) keyvault_name: &'a str,
    pub(crate) endpoint_suffix: String,
    pub(crate) keyvault_endpoint: String,
    pub(crate) token: Option<TokenResponse>,
}

impl<'a, T:TokenCredential> KeyVaultClient<'a, T> {
    /// Creates a new `KeyVaultClient` with an endpoint suffix. Useful for non-public Azure clouds.
    /// For the default public environment, use `KeyVaultClient::new`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use azure_auth_aad::DefaultCredential;
    /// let client = KeyVaultClient::with_endpoint_suffix(&DefaultCredential::default(), &"test-keyvault", "vault.azure.net".to_owned());
    /// ```
    pub fn with_endpoint_suffix(
        token_credential: &'a T,
        keyvault_name: &'a str,
        endpoint_suffix: String,
    ) -> Self {
        let endpoint = format!("https://{}.{}", keyvault_name, endpoint_suffix);
        Self {
            token_credential,
            keyvault_name,
            endpoint_suffix,
            keyvault_endpoint: endpoint,
            token: None,
        }
    }

    /// Creates a new `KeyVaultClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use azure_auth_aad::DefaultCredential;
    /// let client = KeyVaultClient::new(&DefaultCredential::default(), &"test-keyvault");
    /// ```
    pub fn new(
        token_credential: &'a T,
        keyvault_name: &'a str,
    ) -> Self {
        KeyVaultClient::with_endpoint_suffix(
            token_credential,
            keyvault_name,
            PUBLIC_ENDPOINT_SUFFIX.to_owned(),
        )
    }

    pub(crate) async fn refresh_token(&mut self) -> Result<(), KeyVaultError> {
        if matches!(&self.token, Some(token) if token.expires_on > chrono::Utc::now()) {
            // Token is valid, return it.
            return Ok(());
        }
        let resource = format!("https://{}", &self.endpoint_suffix);
        let token = self.token_credential.get_token(&resource)
        .await
        .with_context(|| "Failed to authenticate to Azure Active Directory")
        .map_err(|e| KeyVaultError::AuthorizationError(e))?;
        self.token = Some(token);
        Ok(())
    }

    pub(crate) async fn get_authed(&mut self, uri: String) -> Result<String, KeyVaultError> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .get(&uri)
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap().token.secret()),
            )
            .send()
            .await
            .unwrap();
        let body = resp.text().await.unwrap();
        Ok(body)
    }

    pub(crate) async fn put_authed(
        &mut self,
        uri: String,
        body: String,
    ) -> Result<String, KeyVaultError> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .put(&uri)
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap().token.secret()),
            )
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();
        let body = resp.text().await.unwrap();
        Ok(body)
    }

    pub(crate) async fn post_authed(
        &mut self,
        uri: String,
        json_body: Option<String>,
    ) -> Result<String, KeyVaultError> {
        self.refresh_token().await?;

        let mut req = reqwest::Client::new().post(&uri).header(
            "Authorization",
            format!("Bearer {}", self.token.as_ref().unwrap().token.secret()),
        );

        if let Some(body) = json_body {
            req = req.header("Content-Type", "application/json").body(body);
        } else {
            req = req.header("Content-Length", 0);
        }

        let resp = req.send().await.unwrap();

        let body = resp.text().await.unwrap();
        Ok(body)
    }

    pub(crate) async fn patch_authed(
        &mut self,
        uri: String,
        body: String,
    ) -> Result<String, KeyVaultError> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .patch(&uri)
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap().token.secret()),
            )
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        let body = resp.text().await.unwrap();

        let body_serialized = serde_json::from_str::<serde_json::Value>(&body).unwrap();
        if let Some(err) = body_serialized.get("error") {
            return Err(KeyVaultError::GeneralError(
                err.get("message")
                    .expect("Received an error accessing the Key Vault, which could not be parsed as expected.")
                    .to_string(),
            ));
        }

        Ok(body)
    }

    pub(crate) async fn delete_authed(&mut self, uri: String) -> Result<String, KeyVaultError> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .delete(&uri)
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap().token.secret()),
            )
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();
        let body = resp.text().await.unwrap();
        Ok(body)
    }
}
