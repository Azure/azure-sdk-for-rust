//! Refresh token utilities

use azure_core::Method;
use azure_core::{
    auth::AccessToken,
    content_type,
    error::{Error, ErrorKind, ResultExt},
    headers, HttpClient, Request,
};
use oauth2::{ClientId, ClientSecret};
use serde::Deserialize;
use std::fmt;
use std::sync::Arc;
use url::{form_urlencoded, Url};

/// Exchange a refresh token for a new access token and refresh token
#[allow(clippy::manual_async_fn)]
#[fix_hidden_lifetime_bug::fix_hidden_lifetime_bug]
pub async fn exchange(
    http_client: Arc<dyn HttpClient>,
    tenant_id: &str,
    client_id: &ClientId,
    client_secret: Option<&ClientSecret>,
    refresh_token: &AccessToken,
) -> azure_core::Result<RefreshTokenResponse> {
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

    let url = Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    ))?;

    let mut req = Request::new(url, Method::Post);
    req.insert_header(
        headers::CONTENT_TYPE,
        content_type::APPLICATION_X_WWW_FORM_URLENCODED,
    );
    req.set_body(encoded);

    let rsp = http_client.execute_request(&req).await?;
    let rsp_status = rsp.status();
    let rsp_body = rsp.into_body().collect().await?;

    if !rsp_status.is_success() {
        if let Ok(token_error) = serde_json::from_slice::<RefreshTokenError>(&rsp_body) {
            return Err(Error::new(ErrorKind::Credential, token_error));
        } else {
            return Err(ErrorKind::http_response_from_body(rsp_status, &rsp_body).into_error());
        }
    }

    serde_json::from_slice::<RefreshTokenResponse>(&rsp_body).map_kind(ErrorKind::Credential)
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

/// An error response body when there is an error requesting a token
#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct RefreshTokenError {
    error: String,
    error_description: String,
    error_codes: Vec<i64>,
    timestamp: Option<String>,
    trace_id: Option<String>,
    correlation_id: Option<String>,
    suberror: Option<String>,
    claims: Option<String>,
}

impl std::error::Error for RefreshTokenError {}

impl fmt::Display for RefreshTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        writeln!(f, "error: {}", self.error)?;
        if let Some(suberror) = &self.suberror {
            writeln!(f, "suberror: {}", suberror)?;
        }
        writeln!(f, "description: {}", self.error_description)
    }
}
