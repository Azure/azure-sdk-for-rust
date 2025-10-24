// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use async_lock::Mutex as AsyncMutex;
use azure_core::{
    async_runtime::{get_async_runtime, SpawnedTask},
    credentials::{AccessToken, TokenCredential},
    error::ErrorKind as AzureErrorKind,
    fmt::SafeDebug,
    http::Url,
    time::{Duration, OffsetDateTime},
};
use azure_core_amqp::{
    error::Result, AmqpClaimsBasedSecurityApis as _, AmqpConnection, AmqpSessionApis as _,
};
use rand::{rng, Rng};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex as SyncMutex, OnceLock, Weak},
};
use tracing::{debug, trace, warn};

// The number of seconds before token expiration that we wake up to refresh the token.
const TOKEN_REFRESH_BIAS: Duration = Duration::minutes(6); // By default, we refresh tokens 6 minutes before they expire.
const TOKEN_REFRESH_JITTER_MIN: Duration = Duration::seconds(-5); // Minimum jitter (added from the bias, so a negative number means we refresh before the bias)
const TOKEN_REFRESH_JITTER_MAX: Duration = Duration::seconds(5); // Maximum jitter (added to the bias)

const SERVICEBUS_AUTHORIZATION_SCOPE: &str = "https://servicebus.azure.net/.default";

#[derive(SafeDebug)]
struct TokenRefreshTimes {
    before_expiration_refresh_time: Duration,
    jitter_min: Duration,
    jitter_max: Duration,
}

impl Default for TokenRefreshTimes {
    fn default() -> Self {
        Self {
            before_expiration_refresh_time: TOKEN_REFRESH_BIAS,
            jitter_min: TOKEN_REFRESH_JITTER_MIN,
            jitter_max: TOKEN_REFRESH_JITTER_MAX,
        }
    }
}

pub(crate) struct Authorizer {
    authorization_scopes: AsyncMutex<HashMap<Url, AccessToken>>,
    authorization_refresher: OnceLock<SpawnedTask>,
    /// Bias to apply to token refresh time. This determines how much time we will refresh the token before it expires.
    token_refresh_bias: SyncMutex<TokenRefreshTimes>,
    credential: Arc<dyn TokenCredential>,
    connection: Weak<AmqpConnection>,
    /// This is used to disable authorization for testing purposes.
    #[cfg(test)]
    disable_authorization: SyncMutex<bool>,
}

unsafe impl Send for Authorizer {}
unsafe impl Sync for Authorizer {}

impl Authorizer {
    pub fn new(connection: Weak<AmqpConnection>, credential: Arc<dyn TokenCredential>) -> Self {
        Self {
            authorization_refresher: OnceLock::new(),
            authorization_scopes: AsyncMutex::new(HashMap::new()),
            token_refresh_bias: SyncMutex::new(TokenRefreshTimes::default()),
            credential,
            connection,
            #[cfg(test)]
            disable_authorization: SyncMutex::new(false),
        }
    }

    #[cfg(test)]
    fn disable_authorization(&self) -> Result<()> {
        let mut disable_authorization = self.disable_authorization.lock().map_err(|e| {
            azure_core::Error::with_message(azure_core::error::ErrorKind::Other, e.to_string())
        })?;
        *disable_authorization = true;
        Ok(())
    }

    pub(crate) async fn authorize_path(
        self: &Arc<Self>,
        connection: &Arc<AmqpConnection>,
        path: &Url,
    ) -> Result<AccessToken> {
        debug!("Authorizing path: {path}");
        let mut scopes = self.authorization_scopes.lock().await;

        if !scopes.contains_key(path) {
            debug!("Creating new authorization scope for path: {path}");

            debug!("Get Token.");
            let token = self
                .credential
                .get_token(&[SERVICEBUS_AUTHORIZATION_SCOPE], None)
                .await?;

            debug!("Token for path {path} expires at {}", token.expires_on);

            self.perform_authorization(connection, path, &token).await?;

            // insert returns some if it *fails* to insert, None if it succeeded.
            let present = scopes.insert(path.clone(), token);
            if present.is_some() {
                return Err(azure_core::Error::with_message(
                    AzureErrorKind::Other,
                    "Unable to add authentication token",
                )
                .into());
            }

            debug!("Token verified.");
            self.authorization_refresher.get_or_init(|| {
                debug!("Starting authorization refresh task.");
                let self_clone = self.clone();
                let async_runtime = get_async_runtime();
                async_runtime.spawn(Box::pin(self_clone.refresh_tokens_task()))
            });
        } else {
            debug!("Token already exists for path: {path}");
        }
        Ok(scopes
            .get(path)
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    AzureErrorKind::Other,
                    "Unable to add authentication token",
                )
            })?
            .clone())
    }

    /// Actually perform an authorization against the Service Bus service.
    ///
    /// This method establishes a connection to the Service Bus service and
    /// performs the necessary authorization steps using the provided token.
    ///
    /// # Arguments
    ///
    /// * `connection` - The AMQP connection to use for the authorization.
    /// * `url` - The URL of the resource being authorized.
    /// * `new_token` - The new access token to use for authorization.
    ///
    async fn perform_authorization(
        self: &Arc<Self>,
        connection: &Arc<AmqpConnection>,
        url: &Url,
        new_token: &AccessToken,
    ) -> Result<()> {
        // Test Hook: Disable interacting with Service Bus service if the test doesn't want it.
        #[cfg(test)]
        {
            let disable_authorization = self.disable_authorization.lock().map_err(|e| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("Unable to grab disable mutex: {}", e),
                )
            })?;
            if *disable_authorization {
                debug!("Authorization disabled for testing.");
                return Ok(());
            }
        }

        debug!("Performing authorization for {url}");

        // Create a session for CBS operations
        let session = azure_core_amqp::AmqpSession::new();
        session.begin(connection.as_ref(), None).await?;

        // Create CBS client and authorize the path
        let cbs_client = azure_core_amqp::AmqpClaimsBasedSecurity::new(session)?;
        cbs_client.attach().await?;

        cbs_client
            .authorize_path(
                url.to_string(),
                None,
                &new_token.token,
                new_token.expires_on,
            )
            .await?;

        cbs_client.detach().await?;

        Ok(())
    }

    async fn refresh_tokens_task(self: Arc<Self>) {
        let result = self.refresh_tokens().await;
        if let Err(e) = result {
            warn!(err=?e, "Error refreshing tokens: {e}");
        }
        debug!("Token refresher task completed.");
    }

    /// Refresh the authorization tokens associated with this connection manager.
    ///
    /// Each connection manager maintains an authorization token for each
    /// resource it accesses, and this method ensures that all tokens are
    /// refreshed before their expiration.
    ///
    /// This method is designed to be called periodically to ensure that
    /// tokens are kept up to date.
    ///
    /// The first step in the refresh process is to gather the expiration times
    /// of all tokens. This allows us to determine when to refresh each token
    /// based on its expiration time.
    ///
    /// We calculate the first token to expire and sleep until it expires (with a bit of
    /// jitter in the sleep).
    ///
    /// After we wake up, we iterate over all the authorized paths and refresh their tokens with
    /// the Service Bus service.
    async fn refresh_tokens(self: &Arc<Self>) -> Result<()> {
        debug!("Refreshing tokens.");
        loop {
            let mut expiration_times = vec![];
            {
                let scopes = self.authorization_scopes.lock().await;
                for (path, token) in scopes.iter() {
                    debug!(
                        "Token expiration time for path {}: {}",
                        path, token.expires_on
                    );
                    expiration_times.push(token.expires_on);
                }
            }
            expiration_times.sort();
            debug!("Found expiration times: {:?}", expiration_times);
            if expiration_times.is_empty() {
                debug!("No tokens to refresh. Sleeping for {TOKEN_REFRESH_BIAS:?}.");
                azure_core::sleep::sleep(TOKEN_REFRESH_BIAS).await;
                continue;
            }

            // Calculate duration until we should refresh (6 minutes before expiration,
            // with added random jitter)

            let mut now = OffsetDateTime::now_utc();
            trace!("refresh_tokens: Start pass for: {now}");
            let most_recent_refresh = expiration_times.first().ok_or_else(|| {
                azure_core::Error::with_message(AzureErrorKind::Other, "No tokens to refresh?")
            })?;

            debug!(
                "Nearest token refresh time: {most_recent_refresh}, in {}",
                *most_recent_refresh - now
            );

            let refresh_time: OffsetDateTime;
            let token_refresh_bias: Duration;
            {
                let token_refresh_times = self.token_refresh_bias.lock().map_err(|e| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        format!("Unable to grab token refresh bias mutex: {}", e),
                    )
                })?;

                debug!("Token refresh times: {token_refresh_times:?}");

                let jitter_min = token_refresh_times.jitter_min.whole_milliseconds() as i64;
                let jitter_max = token_refresh_times.jitter_max.whole_milliseconds() as i64;
                let expiration_jitter =
                    Duration::milliseconds(rng().random_range(jitter_min..jitter_max));
                debug!("Expiration jitter: {expiration_jitter:?}");

                token_refresh_bias = token_refresh_times
                    .before_expiration_refresh_time
                    .checked_add(expiration_jitter)
                    .ok_or_else(|| {
                        azure_core::Error::with_message(
                            AzureErrorKind::Other,
                            "Unable to calculate token refresh bias - overflow",
                        )
                    })?;
                debug!("Token refresh bias with jitter: {token_refresh_bias:?}");

                refresh_time = most_recent_refresh
                    .checked_sub(token_refresh_bias)
                    .ok_or_else(|| {
                        azure_core::Error::with_message(
                            AzureErrorKind::Other,
                            "Unable to calculate token refresh bias - underflow",
                        )
                    })?;
            }
            debug!("refresh_tokens: Refresh time: {refresh_time}");

            // Convert to a duration if refresh time is in the future and sleep until it's time
            // to refresh the token.
            if refresh_time > now {
                let sleep_duration = refresh_time - now;
                debug!(
                    "refresh_tokens: Sleeping for {sleep_duration:?} until {:?}",
                    now + sleep_duration
                );
                azure_core::sleep::sleep(sleep_duration).await;
                now = OffsetDateTime::now_utc();
            } else {
                debug!("Not sleeping because refresh time ({refresh_time}) is in the past (now = {now}).");
            }

            // Refresh the tokens.
            // First, collect the tokens that need refreshing while holding the lock briefly
            let tokens_to_refresh = {
                let scopes = self.authorization_scopes.lock().await;
                let mut to_refresh = Vec::new();
                for (url, token) in scopes.iter() {
                    if token.expires_on >= now + (token_refresh_bias) {
                        debug!(
                            "Token not expired for {url}: ExpiresOn: {}, Now: {now}, Bias: {token_refresh_bias:?}",
                            token.expires_on
                        );
                        continue;
                    }

                    debug!(
                        "Token about to be expired for {url}: ExpiresOn: {}, Now: {now}, Bias: {token_refresh_bias:?}",
                        token.expires_on
                    );
                    to_refresh.push(url.clone());
                }
                to_refresh
            };

            // Now refresh tokens without holding the lock to avoid deadlocks
            let mut updated_tokens = HashMap::new();
            for url in tokens_to_refresh {
                let new_token = self
                    .credential
                    .get_token(&[SERVICEBUS_AUTHORIZATION_SCOPE], None)
                    .await?;

                // Create an ephemeral connection to host the authentication.
                let connection = self.connection.upgrade().ok_or_else(|| {
                    azure_core::Error::with_message(
                        AzureErrorKind::Other,
                        "Connection has been dropped",
                    )
                })?;
                self.perform_authorization(&connection, &url, &new_token)
                    .await?;

                debug!(
                    "Token refreshed for {url}, new expiration time: {}",
                    new_token.expires_on
                );
                updated_tokens.insert(url.clone(), new_token);
            }

            // Finally, update the scopes map with the new tokens
            if !updated_tokens.is_empty() {
                let mut scopes = self.authorization_scopes.lock().await;
                for (url, token) in updated_tokens.into_iter() {
                    scopes.insert(url.clone(), token);
                }
                debug!("Updated tokens.");
            }
        }
    }

    #[cfg(test)]
    fn set_token_refresh_times(&self, refresh_times: TokenRefreshTimes) -> Result<()> {
        let mut token_refresh_bias = self.token_refresh_bias.lock().map_err(|e| {
            azure_core::Error::with_message(azure_core::error::ErrorKind::Other, e.to_string())
        })?;
        *token_refresh_bias = refresh_times;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::credentials::TokenRequestOptions;
    use azure_core::{http::Url, time::OffsetDateTime, Result};
    use std::sync::Arc;

    // Helper struct to mock token credential
    #[derive(Debug)]
    struct MockTokenCredential {
        /// Duration in seconds until the token expires
        token_duration: i64,

        /// The token itself
        /// This is a mock token, so we don't need to worry about the actual value
        token: SyncMutex<AccessToken>,

        /// Count of how many times the token has been requested
        /// This is used to verify that the token is being refreshed correctly
        /// in the tests
        get_token_count: SyncMutex<usize>,
    }

    impl MockTokenCredential {
        fn new(expires_in_seconds: i64) -> Arc<Self> {
            let expires_on = OffsetDateTime::now_utc() + Duration::seconds(expires_in_seconds);
            Arc::new(Self {
                token_duration: expires_in_seconds,
                token: SyncMutex::new(AccessToken::new(
                    azure_core::credentials::Secret::new("mock_token"),
                    expires_on,
                )),
                get_token_count: SyncMutex::new(0),
            })
        }

        #[allow(dead_code)]
        fn get_token_get_count(&self) -> usize {
            *self.get_token_count.lock().unwrap()
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl TokenCredential for MockTokenCredential {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _options: Option<TokenRequestOptions<'_>>,
        ) -> Result<AccessToken> {
            // Simulate a token refresh by incrementing the token get count
            // and updating the token expiration time
            {
                let mut count = self.get_token_count.lock().unwrap();
                *count += 1;
            }

            let expires_on = OffsetDateTime::now_utc() + Duration::seconds(self.token_duration);
            {
                let mut token = self.token.lock().unwrap();
                *token = AccessToken::new(
                    azure_core::credentials::Secret::new("mock_token"),
                    expires_on,
                );
                Ok(token.clone())
            }
        }
    }

    // When a token is created, it needs to have a proper expiration time.
    // This test verifies that the expiration time of tokens is set correctly when
    // authorizing a path. It also confirms that tokens are properly stored for reuse
    // and that their expiration times are within the expected range.
    //
    #[tokio::test]
    async fn token_credential_expiration() {
        let _url = Url::parse("amqps://example.com").unwrap();
        let path = Url::parse("amqps://example.com/test_token_credential_expiration").unwrap();

        // Create a mock token credential that expires in 15 seconds
        let mock_credential = MockTokenCredential::new(15);

        let connection = Arc::new(azure_core_amqp::AmqpConnection::new());
        let authorizer = Arc::new(Authorizer::new(
            Arc::downgrade(&connection),
            mock_credential.clone(),
        ));

        // Disable actual authorization for testing
        authorizer.disable_authorization().unwrap();

        // Expire tokens 10 seconds before they would normally expire.
        // The token in question expires in 15 seconds, so we want to refresh it before then.
        authorizer
            .set_token_refresh_times(TokenRefreshTimes {
                before_expiration_refresh_time: Duration::seconds(10),
                ..Default::default()
            })
            .unwrap();

        // This should succeed and store the token in the authorization scopes
        let result = authorizer.authorize_path(&connection, &path).await;
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        // Verify token is stored
        let scopes = authorizer.authorization_scopes.lock().await;
        assert!(scopes.contains_key(&path));

        // Verify expiration time
        let stored_token = scopes.get(&path).unwrap();
        let now = OffsetDateTime::now_utc();
        assert!(stored_token.expires_on > now);
        assert!(stored_token.expires_on < now + Duration::seconds(15)); // Should be less than now + 15 seconds
    }
}
