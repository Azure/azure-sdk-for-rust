//! Azure authentication and authorization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessToken(String);

impl AccessToken {
    #[must_use]
    pub fn new(access_token: String) -> Self {
        Self(access_token)
    }
    #[must_use]
    pub fn secret(&self) -> &str {
        self.0.as_str()
    }
}

/// Represents an Azure service bearer access token with expiry information.
#[derive(Debug, Clone)]
pub struct TokenResponse {
    /// Get the access token value.
    pub token: AccessToken,
    /// Gets the time when the provided token expires.
    pub expires_on: DateTime<Utc>,
}

impl TokenResponse {
    /// Create a new `TokenResponse`.
    #[must_use]
    pub fn new(token: AccessToken, expires_on: DateTime<Utc>) -> Self {
        Self { token, expires_on }
    }
}

/// Represents a credential capable of providing an OAuth token.
#[async_trait::async_trait]
pub trait TokenCredential: Send + Sync {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> crate::Result<TokenResponse>;
}
