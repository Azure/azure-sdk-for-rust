//! Azure authentication and authorization.

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Debug};
use time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessToken(Cow<'static, str>);

impl AccessToken {
    pub fn new<T>(access_token: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self(access_token.into())
    }

    pub fn secret(&self) -> &str {
        &self.0
    }
}

/// Represents an Azure service bearer access token with expiry information.
#[derive(Debug, Clone)]
pub struct TokenResponse {
    /// Get the access token value.
    pub token: AccessToken,
    /// Gets the time when the provided token expires.
    pub expires_on: OffsetDateTime,
}

impl TokenResponse {
    /// Create a new `TokenResponse`.
    pub fn new(token: AccessToken, expires_on: OffsetDateTime) -> Self {
        Self { token, expires_on }
    }
}

/// Represents a credential capable of providing an OAuth token.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait TokenCredential: Send + Sync {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> crate::Result<TokenResponse>;
}
