use async_lock::RwLock;
use azure_core::auth::{TokenCredential, TokenResponse};
use azure_core::error::{Error, ErrorKind};
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;

fn is_expired(token: &TokenResponse) -> bool {
    token.expires_on < OffsetDateTime::now_utc() + Duration::from_secs(20)
}

#[derive(Clone)]
/// Wraps a TokenCredential and handles token refresh on token expiry
pub struct AutoRefreshingTokenCredential {
    credential: Arc<dyn TokenCredential>,
    current_token: Arc<RwLock<Option<azure_core::Result<TokenResponse>>>>,
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

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AutoRefreshingTokenCredential {
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
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
                    return Err(Error::with_message(ErrorKind::Credential, || {
                        err.to_string()
                    }));
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
