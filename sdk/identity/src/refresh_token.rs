//! Refresh token utilities

use crate::errors::ErrorToken;
use azure_core::error::{ErrorKind, Result, ResultExt};
use oauth2::{AccessToken, ClientId, ClientSecret};
use serde::Deserialize;
use url::form_urlencoded;

/// Exchange a refresh token for a new access token and refresh token
pub async fn exchange(
    client: &reqwest::Client,
    tenant_id: &str,
    client_id: &ClientId,
    client_secret: Option<&ClientSecret>,
    refresh_token: &AccessToken,
) -> Result<RefreshTokenResponse> {
    let mut encoded = form_urlencoded::Serializer::new(String::new());
    let encoded = encoded.append_pair("grant_type", "refresh_token");
    let encoded = encoded.append_pair("client_id", client_id.as_str());
    // optionally add the client secret
    let encoded = if let Some(client_secret) = client_secret {
        encoded.append_pair("client_secret", client_secret.secret())
    } else {
        encoded
    };
    let encoded = encoded.append_pair("refresh_token", refresh_token.secret());
    let encoded = encoded.finish();

    let url = url::Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    ))?;

    let rsp = client
        .post(url)
        .header("ContentType", "application/x-www-form-urlencoded")
        .body(encoded)
        .send()
        .await
        .map_kind(ErrorKind::Credential)?;

    let rsp_body = rsp.bytes().await.map_kind(ErrorKind::Credential)?;

    match serde_json::from_slice::<RefreshTokenResponse>(&rsp_body) {
        Ok(r) => Ok(r),
        Err(e) => {
            if let Ok(token_error) = serde_json::from_slice::<ErrorToken>(&rsp_body) {
                Err(crate::errors::Error::Token(token_error).into())
            } else {
                Err(e.into())
            }
        }
    }
}

/// A refresh token
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenResponse {
    token_type: String,
    #[serde(rename = "scope", deserialize_with = "deserialize::split")]
    scopes: Vec<String>,
    expires_in: u64,
    ext_expires_in: u64,
    access_token: AccessToken,
    refresh_token: AccessToken,
}

impl RefreshTokenResponse {
    /// Returns the token_type. Always `Bearer` for Azure AD.
    pub fn token_type(&self) -> &str {
        &self.token_type
    }
    /// The scopes that the `access_token` is valid for.
    pub fn scopes(&self) -> &[String] {
        &self.scopes
    }
    /// Number of seconds the `access_token` is valid for.
    pub fn expires_in(&self) -> u64 {
        self.expires_in
    }
    /// Issued for the scopes that were requested.
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
    /// The new refresh token and should replace old refresh token.
    pub fn refresh_token(&self) -> &AccessToken {
        &self.refresh_token
    }
    /// Indicates the extended lifetime of an `access_token`.
    pub fn ext_expires_in(&self) -> u64 {
        self.ext_expires_in
    }
}

mod deserialize {
    use serde::Deserializer;
    pub fn split<'de, D>(scope: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: String = serde::Deserialize::deserialize(scope)?;
        Ok(string.split(' ').map(|s| s.to_owned()).collect())
    }
}
