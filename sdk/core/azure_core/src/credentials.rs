// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure authentication and authorization.

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Debug};
use typespec_client_core::{fmt::SafeDebug, time::OffsetDateTime};

/// Default Azure authorization scope.
pub static DEFAULT_SCOPE_SUFFIX: &str = "/.default";

/// Represents a secret.
///
/// The [`Debug`] implementation will not print the secret.
#[derive(Clone, Deserialize, Serialize, Eq)]
pub struct Secret(Cow<'static, str>);

impl Secret {
    /// Create a new `Secret`.
    pub fn new<T>(access_token: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self(access_token.into())
    }

    /// Get the secret value.
    pub fn secret(&self) -> &str {
        &self.0
    }
}

// NOTE: this is a constant time compare, however LLVM may (and probably will)
// optimize this in unexpected ways.
impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        let a = self.secret();
        let b = other.secret();

        if a.len() != b.len() {
            return false;
        }

        a.bytes()
            .zip(b.bytes())
            .fold(0, |acc, (a, b)| acc | (a ^ b))
            == 0
    }
}

impl From<String> for Secret {
    fn from(access_token: String) -> Self {
        Self::new(access_token)
    }
}

impl From<&'static str> for Secret {
    fn from(access_token: &'static str) -> Self {
        Self::new(access_token)
    }
}

impl Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secret")
    }
}

/// Represents an Azure service bearer access token with expiry information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    /// Get the access token value.
    pub token: Secret,
    /// Gets the time when the provided token expires.
    pub expires_on: OffsetDateTime,
}

impl AccessToken {
    /// Create a new `AccessToken`.
    pub fn new<T>(token: T, expires_on: OffsetDateTime) -> Self
    where
        T: Into<Secret>,
    {
        Self {
            token: token.into(),
            expires_on,
        }
    }
}

/// Options for getting a token from a [`TokenCredential`]
#[derive(Clone, Default, SafeDebug)]
pub struct TokenRequestOptions;

/// Represents a credential capable of providing an OAuth token.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait TokenCredential: Send + Sync + Debug {
    /// Gets an [`AccessToken`] for the specified scopes
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions>,
    ) -> crate::Result<AccessToken>;
}
