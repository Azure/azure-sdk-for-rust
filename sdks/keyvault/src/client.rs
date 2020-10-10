use crate::KeyVaultError;
use anyhow::Context;
use anyhow::Result;
use azure_auth_aad::authorize_client_credentials_flow;
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
pub struct KeyVaultClient<'a> {
    pub(crate) aad_client_id: &'a str,
    pub(crate) aad_client_secret: &'a str,
    pub(crate) aad_tenant_id: &'a str,
    pub(crate) keyvault_name: &'a str,
    pub(crate) endpoint_suffix: String,
    pub(crate) keyvault_endpoint: String,
    pub(crate) token: Option<AccessToken>,
    pub(crate) token_expiration: Option<DateTime<Utc>>,
}

impl<'a> KeyVaultClient<'a> {
    /// Creates a new `KeyVaultClient` with an endpoint suffix. Useful for non-public Azure clouds.
    /// For the default public environment, use `KeyVaultClient::new`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// let client = KeyVaultClient::with_endpoint_suffix(&"c1a6d79b-082b-4798-b362-a77e96de50db", &"SUPER_SECRET_KEY", &"bc598e67-03d8-44d5-aa46-8289b9a39a14", &"test-keyvault", "vault.azure.net".to_owned());
    /// ```
    pub fn with_endpoint_suffix(
        aad_client_id: &'a str,
        aad_client_secret: &'a str,
        aad_tenant_id: &'a str,
        keyvault_name: &'a str,
        endpoint_suffix: String,
    ) -> Self {
        let endpoint = format!("https://{}.{}", keyvault_name, endpoint_suffix);
        Self {
            aad_client_id,
            aad_client_secret,
            aad_tenant_id,
            keyvault_name,
            endpoint_suffix,
            keyvault_endpoint: endpoint,
            token: None,
            token_expiration: None,
        }
    }

    /// Creates a new `KeyVaultClient` with a pre-existing AAD token.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use chrono::{Utc, Duration};
    /// use oauth2::AccessToken;
    /// let client = KeyVaultClient::with_aad_token(&"c1a6d79b-082b-4798-b362-a77e96de50db", &"SUPER_SECRET_KEY", &"bc598e67-03d8-44d5-aa46-8289b9a39a14", &"test-keyvault", AccessToken::new(String::new()), Utc::now() + Duration::days(14));
    /// ```
    pub fn with_aad_token(
        aad_client_id: &'a str,
        aad_client_secret: &'a str,
        aad_tenant_id: &'a str,
        keyvault_name: &'a str,
        aad_token: AccessToken,
        aad_token_expiration: DateTime<Utc>,
    ) -> Self {
        let endpoint = format!("https://{}.{}", keyvault_name, PUBLIC_ENDPOINT_SUFFIX);
        Self {
            aad_client_id,
            aad_client_secret,
            aad_tenant_id,
            keyvault_name,
            endpoint_suffix: PUBLIC_ENDPOINT_SUFFIX.to_owned(),
            keyvault_endpoint: endpoint,
            token: Some(aad_token),
            token_expiration: Some(aad_token_expiration),
        }
    }

    /// Creates a new `KeyVaultClient` with a pre-existing AAD token.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// use chrono::{Utc, Duration};
    /// use oauth2::AccessToken;
    /// let client = KeyVaultClient::with_aad_token(&"c1a6d79b-082b-4798-b362-a77e96de50db", &"SUPER_SECRET_KEY", &"bc598e67-03d8-44d5-aa46-8289b9a39a14", &"test-keyvault", AccessToken::new(String::new()), Utc::now() + Duration::days(14));
    /// ```
    pub fn with_aad_token_and_endpoint_suffix(
        aad_client_id: &'a str,
        aad_client_secret: &'a str,
        aad_tenant_id: &'a str,
        keyvault_name: &'a str,
        aad_token: AccessToken,
        aad_token_expiration: DateTime<Utc>,
    ) -> Self {
        let endpoint = format!("https://{}.{}", keyvault_name, PUBLIC_ENDPOINT_SUFFIX);
        Self {
            aad_client_id,
            aad_client_secret,
            aad_tenant_id,
            keyvault_name,
            endpoint_suffix: PUBLIC_ENDPOINT_SUFFIX.to_owned(),
            keyvault_endpoint: endpoint,
            token: Some(aad_token),
            token_expiration: Some(aad_token_expiration),
        }
    }

    /// Creates a new `KeyVaultClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_keyvault::KeyVaultClient;
    /// let client = KeyVaultClient::new(&"c1a6d79b-082b-4798-b362-a77e96de50db", &"SUPER_SECRET_KEY", &"bc598e67-03d8-44d5-aa46-8289b9a39a14", &"test-keyvault");
    /// ```
    pub fn new(
        aad_client_id: &'a str,
        aad_client_secret: &'a str,
        aad_tenant_id: &'a str,
        keyvault_name: &'a str,
    ) -> Self {
        KeyVaultClient::with_endpoint_suffix(
            aad_client_id,
            aad_client_secret,
            aad_tenant_id,
            keyvault_name,
            PUBLIC_ENDPOINT_SUFFIX.to_owned(),
        )
    }

    pub(crate) async fn refresh_token(&mut self) -> Result<(), KeyVaultError> {
        if matches!(self.token_expiration, Some(exp) if exp > chrono::Utc::now()) {
            // Token is valid, return it.
            return Ok(());
        }
        let aad_client_id = ClientId::new(self.aad_client_id.to_owned());
        let aad_client_secret = ClientSecret::new(self.aad_client_secret.to_owned());
        let token = authorize_client_credentials_flow(
            Arc::new(reqwest::Client::new()),
            &aad_client_id,
            &aad_client_secret,
            &format!("https://{}", self.endpoint_suffix),
            self.aad_tenant_id,
        )
        .await
        .with_context(|| "Failed to authenticate to Azure Active Directory")
        .map_err(|e| KeyVaultError::AuthorizationError(e))?;
        self.token = Some(token.access_token().clone());
        self.token_expiration = token.expires_on;
        Ok(())
    }

    pub(crate) async fn get_authed(&mut self, uri: String) -> Result<String, KeyVaultError> {
        self.refresh_token().await?;

        let resp = reqwest::Client::new()
            .get(&uri)
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap().secret()),
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
                format!("Bearer {}", self.token.as_ref().unwrap().secret()),
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
            format!("Bearer {}", self.token.as_ref().unwrap().secret()),
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
                format!("Bearer {}", self.token.as_ref().unwrap().secret()),
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
                format!("Bearer {}", self.token.as_ref().unwrap().secret()),
            )
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();
        let body = resp.text().await.unwrap();
        Ok(body)
    }
}
