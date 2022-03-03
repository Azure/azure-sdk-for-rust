use azure_core::{
    auth::{TokenCredential, TokenResponse},
    Error,
};
use chrono::{Duration, Utc};
use futures::lock::Mutex;
use std::sync::Arc;

fn is_expired(token: TokenResponse) -> bool {
    token.expires_on < Utc::now() + Duration::seconds(20)
}

#[derive(Clone)]
pub struct AutoRefreshingTokenCredential {
    credential: Arc<dyn TokenCredential>,
    current_token: Arc<Mutex<Option<std::result::Result<TokenResponse, Error>>>>,
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
            current_token: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for AutoRefreshingTokenCredential {
    async fn get_token(&self, resource: &str) -> std::result::Result<TokenResponse, Error> {
        loop {
            let mut guard = self.current_token.lock().await;
            match guard.as_ref() {
                None => {
                    let res = self.credential.get_token(resource).await;
                    *guard = Some(res);
                }
                Some(Err(err)) => {
                    // TODO return a meaningful error here, once we decide how to proceed, introduce a new variant,
                    // or migrate token credentials to return new errors with error kind.
                    return Err(Error::HeaderNotFound(err.to_string()));
                }
                Some(Ok(token)) => {
                    if is_expired(token.clone()) {
                        *guard = None;
                    } else {
                        return Ok(token.clone());
                    };
                }
            }
        }
    }
}
