use crate::{
    auth::TokenCredential,
    headers::AUTHORIZATION,
    policies::{Policy, PolicyResult},
    Context, Request,
};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct BearerTokenCredentialPolicy {
    credential: Arc<dyn TokenCredential>,
    scopes: Vec<String>,
    access_token: Arc<Mutex<String>>,
    last_refresh_time: Arc<Mutex<i64>>,
}

/// Default timeout in seconds before refreshing a new token.
const DEFAULT_REFRESH_TIME: i64 = 120;

impl BearerTokenCredentialPolicy {
    pub fn new<A, B>(credential: Arc<dyn TokenCredential>, scopes: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        Self {
            credential,
            scopes: scopes.into_iter().map(|s| s.into()).collect(),
            access_token: Arc::new(Mutex::new("UNSET".to_string())), // Need a better default here or use Option
            last_refresh_time: Arc::new(Mutex::new(0)), // Need a better default here aswell
        }
    }

    fn update_data(&self, new_access_token: &str) {
        let mut my_access_token = self.access_token.lock().unwrap();
        *my_access_token = new_access_token.to_string();
        let mut my_last_refresh_time = self.last_refresh_time.lock().unwrap();
        *my_last_refresh_time = OffsetDateTime::now_utc().unix_timestamp();
    }

    fn should_refresh(&self) -> bool {
        let current_time = OffsetDateTime::now_utc().unix_timestamp();
        let my_last_refresh_time = self.last_refresh_time.lock().unwrap();
        if current_time - *my_last_refresh_time > DEFAULT_REFRESH_TIME {
            return true;
        }
        return false;
    }

    fn access_token(&self) -> String {
        let my_access_token = self.access_token.lock().unwrap();
        (*my_access_token.clone()).to_string()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for BearerTokenCredentialPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let scopes = self
            .scopes
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        // let access_token = self.credential.get_token(&scopes).await?;
        // let token = access_token.token.secret();
        // request.insert_header(AUTHORIZATION, format!("Bearer {token}"));

        // Test Stuff
        let should_update = self.should_refresh();
        if should_update {
            let access_token = self.credential.get_token(&scopes).await?;
            let token = access_token.token.secret();
            self.update_data(token)
        }
        let token = String::from(self.access_token());
        request.insert_header(AUTHORIZATION, format!("Bearer {token}"));

        next[0].send(ctx, request, &next[1..]).await
    }
}
