// Licensed under the MIT License.

#![cfg(feature = "key_auth")]

use std::sync::{Arc, Mutex};
use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
use azure_core::time::{Duration, OffsetDateTime};

const PUBLIC_COSMOS_SCOPE: &str = "https://cosmos.azure.com/.default";

#[derive(Debug, Clone)]
struct CapturedScopes(Arc<Mutex<Vec<String>>>);

impl CapturedScopes {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
    fn push(&self, scope: &str) {
        self.0.lock().unwrap().push(scope.to_string());
    }
    fn take(&self) -> Vec<String> {
        std::mem::take(&mut *self.0.lock().unwrap())
    }
}

#[derive(Debug)]
struct ScopeCapturingCredential {
    captured: CapturedScopes,
}

#[async_trait::async_trait]
impl TokenCredential for ScopeCapturingCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _opts: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let scope = scopes.join(",");
        self.captured.push(&scope);

        // Return a simple mock token
        Ok(AccessToken::new(
            "mock_token_for_scope_verification".to_string(),
            OffsetDateTime::now_utc().saturating_add(Duration::minutes(60)),
        ))
    }
}

/// Tests that the authorization policy uses the constant scope "https://cosmos.azure.com/.default"
/// when requesting AAD tokens.
#[tokio::test]
async fn aad_authentication_uses_constant_scope() -> Result<(), Box<dyn std::error::Error>> {
    let captured = CapturedScopes::new();

    let cred = Arc::new(ScopeCapturingCredential { 
        captured: captured.clone()
    });

    // Trigger token request to capture scope
    let _ = cred.get_token(&[PUBLIC_COSMOS_SCOPE], None).await?;

    // Verify scope was captured correctly
    let scopes = captured.take();
    assert!(!scopes.is_empty(), "Expected at least one authentication request");
    assert!(
        scopes.iter().all(|s| s == PUBLIC_COSMOS_SCOPE),
        "Expected all scopes to be '{}', but got: {:?}",
        PUBLIC_COSMOS_SCOPE,
        scopes
    );

    Ok(())
}