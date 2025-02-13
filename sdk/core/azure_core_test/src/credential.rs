// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    date::OffsetDateTime,
    error::ErrorKind,
};
use std::time::Duration;

/// A mock [`TokenCredential`] useful for testing.
#[derive(Clone, Debug, Default)]
pub struct MockCredential;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for MockCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let token: Secret = format!("TEST TOKEN {}", scopes.join(" ")).into();
        let expires_on = OffsetDateTime::now_utc().saturating_add(
            Duration::from_secs(60 * 5).try_into().map_err(|err| {
                azure_core::Error::full(ErrorKind::Other, err, "failed to compute expiration")
            })?,
        );
        Ok(AccessToken { token, expires_on })
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        Ok(())
    }
}
