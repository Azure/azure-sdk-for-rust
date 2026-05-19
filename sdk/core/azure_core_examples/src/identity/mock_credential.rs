// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    time::{Duration, OffsetDateTime},
};
use std::sync::Arc;

/// A mock [`TokenCredential`] useful for testing.
#[derive(Clone, Debug, Default)]
pub struct MockCredential;

impl MockCredential {
    /// Create a new `MockCredential`.
    pub fn new() -> azure_core::Result<Arc<Self>> {
        Ok(Arc::new(MockCredential {}))
    }
}

#[async_trait::async_trait]
impl TokenCredential for MockCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let token: Secret = format!("TEST TOKEN {}", scopes.join(" ")).into();
        let expires_on = OffsetDateTime::now_utc().saturating_add(Duration::minutes(5));

        Ok(AccessToken { token, expires_on })
    }
}
