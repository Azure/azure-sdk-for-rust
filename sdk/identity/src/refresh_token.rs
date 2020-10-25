//! Refresh token utilties

use crate::traits::{BearerToken, ExtExpiresIn, RefreshToken};
use azure_core::errors::AzureError;
use log::debug;
use oauth2::{AccessToken, ClientId, ClientSecret};
use serde::Deserialize;
use std::convert::TryInto;
use url::form_urlencoded;

/// Exchange a refresh token for a new access token and refresh token
pub async fn exchange(
    client: &reqwest::Client,
    tenant_id: &str,
    client_id: &ClientId,
    client_secret: Option<&ClientSecret>,
    refresh_token: &AccessToken,
) -> Result<RefreshTokenResponse, AzureError> {
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

    debug!("encoded ==> {}", encoded);

    let url = url::Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    ))?;

    let ret = client
        .post(url)
        .header("ContentType", "application/x-www-form-urlencoded")
        .body(encoded)
        .send()
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?
        .text()
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?;
    debug!("{}", ret);

    Ok(ret.try_into()?)
}

/// A refresh token
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenResponse {
    token_type: String,
    #[serde(deserialize_with = "deserialize::split")]
    scopes: Vec<String>,
    expires_in: u64,
    ext_expires_in: u64,
    access_token: AccessToken,
    refresh_token: AccessToken,
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

impl TryInto<RefreshTokenResponse> for String {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<RefreshTokenResponse, Self::Error> {
        serde_json::from_str::<RefreshTokenResponse>(&self)
    }
}

impl BearerToken for RefreshTokenResponse {
    fn token_type(&self) -> &str {
        &self.token_type
    }

    fn scopes(&self) -> &[String] {
        &self.scopes
    }

    fn expires_in(&self) -> u64 {
        self.expires_in
    }

    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
}

impl RefreshToken for RefreshTokenResponse {
    fn refresh_token(&self) -> &AccessToken {
        &self.refresh_token
    }
}

impl ExtExpiresIn for RefreshTokenResponse {
    fn ext_expires_in(&self) -> u64 {
        self.ext_expires_in
    }
}
