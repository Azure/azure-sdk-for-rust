use async_lock::RwLock;
use azure_core::{
    auth::{TokenCredential, TokenResponse},
    Error,
};
use chrono::{Duration, Utc};
use std::sync::Arc;

fn is_expired(token: &TokenResponse) -> bool {
    token.expires_on < Utc::now() + Duration::seconds(20)
}

#[derive(Clone)]
/// Wraps a TokenCredential and handles token refresh on token expiry
pub struct AutoRefreshingTokenCredential {
    credential: Arc<dyn TokenCredential>,
    current_token: Arc<RwLock<Option<std::result::Result<TokenResponse, Error>>>>,
}

impl std::fmt::Debug for AutoRefreshingTokenCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AutoRefreshingTokenCredential")
            .field("credential", &"TokenCredential")
            .finish()
    }
}

impl AutoRefreshingTokenCredential {
    /// Create a new `AutoRefreshingTokenCredential` around the provided base provider.
    pub fn new(provider: Arc<dyn TokenCredential>) -> Self {
        Self {
            credential: provider,
            current_token: Arc::new(RwLock::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for AutoRefreshingTokenCredential {
    async fn get_token(&self, resource: &str) -> std::result::Result<TokenResponse, Error> {
        if let Some(Ok(token)) = self.current_token.read().await.as_ref() {
            if !is_expired(token) {
                return Ok(token.clone());
            }
        }
        loop {
            let mut guard = self.current_token.write().await;
            match guard.as_ref() {
                None => {
                    let res = self.credential.get_token(resource).await;
                    *guard = Some(res);
                }
                Some(Err(err)) => {
                    return Err(Error::AuthorizationPolicy(err.to_string()));
                }
                Some(Ok(token)) => {
                    if is_expired(token) {
                        *guard = None;
                    } else {
                        return Ok(token.clone());
                    };
                }
            }
        }
    }
}
