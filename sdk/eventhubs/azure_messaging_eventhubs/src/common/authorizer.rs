// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{common::recoverable::RecoverableConnection, error::Result};
use async_lock::Mutex as AsyncMutex;
use azure_core::{
    async_runtime::{get_async_runtime, SpawnedTask},
    credentials::{AccessToken, TokenCredential},
    http::Url,
    time::{Duration, OffsetDateTime},
};
use azure_core_amqp::{AmqpClaimsBasedSecurityApis as _, AmqpError};
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

const EVENTHUBS_AUTHORIZATION_SCOPE: &str = "https://eventhubs.azure.net/.default";

#[derive(Debug)]
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
    recoverable_connection: Weak<RecoverableConnection>,
    /// This is used to disable authorization for testing purposes.
    #[cfg(test)]
    disable_authorization: SyncMutex<bool>,
}

unsafe impl Send for Authorizer {}
unsafe impl Sync for Authorizer {}

impl Authorizer {
    pub(crate) fn new(
        recoverable_connection: Weak<RecoverableConnection>,
        credential: Arc<dyn TokenCredential>,
    ) -> Self {
        Self {
            authorization_refresher: OnceLock::new(),
            authorization_scopes: AsyncMutex::new(HashMap::new()),
            token_refresh_bias: SyncMutex::new(TokenRefreshTimes::default()),
            credential,
            recoverable_connection,
            #[cfg(test)]
            disable_authorization: SyncMutex::new(false),
        }
    }

    pub(crate) async fn clear(&self) {
        debug!("Clearing authorization scopes.");
        let mut scopes = self.authorization_scopes.lock().await;
        scopes.clear();
    }

    #[cfg(test)]
    fn disable_authorization(&self) -> Result<()> {
        use crate::EventHubsError;

        let mut disable_authorization = self
            .disable_authorization
            .lock()
            .map_err(|e| EventHubsError::with_message(e.to_string()))?;
        *disable_authorization = true;
        Ok(())
    }

    pub(crate) async fn authorize_path(
        self: &Arc<Self>,
        connection: &Arc<RecoverableConnection>,
        path: &Url,
    ) -> azure_core_amqp::Result<AccessToken> {
        debug!("Authorizing path: {path}");
        let mut scopes = self.authorization_scopes.lock().await;

        if !scopes.contains_key(path) {
            debug!("Creating new authorization scope for path: {path}");

            debug!("Get Token.");
            let token = self
                .credential
                .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE], None)
                .await
                .map_err(AmqpError::from)?;

            debug!("Token for path {path} expires at {}", token.expires_on);

            self.perform_authorization(connection, path, &token).await?;

            // insert returns some if it *fails* to insert, None if it succeeded.
            let present = scopes.insert(path.clone(), token);
            if present.is_some() {
                return Err(AmqpError::with_message(
                    "Unable to add authentication token",
                ));
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
            .ok_or_else(|| AmqpError::with_message("Unable to add authentication token"))?
            .clone())
    }

    /// Actually perform an authorization against the Event Hubs service.
    ///
    /// This method establishes a connection to the Event Hubs service and
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
        connection: &Arc<RecoverableConnection>,
        url: &Url,
        new_token: &AccessToken,
    ) -> azure_core_amqp::Result<()> {
        // Test Hook: Disable interacting with Event Hubs service if the test doesn't want it.
        #[cfg(test)]
        {
            let disable_authorization = self.disable_authorization.lock().map_err(|e| {
                AmqpError::with_message(format!("Unable to grab disable mutex: {}", e))
            })?;
            if *disable_authorization {
                debug!("Authorization disabled for testing.");
                return Ok(());
            }
        }

        debug!("Performing authorization for {url}");

        connection
            .get_cbs_client()
            .authorize_path(
                url.to_string(),
                None,
                &new_token.token,
                new_token.expires_on,
            )
            .await
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
    /// the Event Hubs service.
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
            let most_recent_refresh = expiration_times
                .first()
                .ok_or_else(|| AmqpError::with_message("No tokens to refresh?"))?;

            debug!(
                "Nearest token refresh time: {most_recent_refresh}, in {}",
                *most_recent_refresh - now
            );

            let refresh_time: OffsetDateTime;
            let token_refresh_bias: Duration;
            {
                let token_refresh_times = self.token_refresh_bias.lock().map_err(|e| {
                    AmqpError::with_message(format!(
                        "Unable to grab token refresh bias mutex: {}",
                        e
                    ))
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
                        AmqpError::with_message("Unable to calculate token refresh bias - overflow")
                    })?;
                debug!("Token refresh bias with jitter: {token_refresh_bias:?}");

                refresh_time = most_recent_refresh
                    .checked_sub(token_refresh_bias)
                    .ok_or_else(|| {
                        AmqpError::with_message(
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
                    .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE], None)
                    .await?;

                // Create an ephemeral connection to host the authentication.
                let connection = self.recoverable_connection.upgrade().ok_or_else(|| {
                    AmqpError::with_message("Recoverable connection has been dropped")
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
            AmqpError::with_message(format!("Unable to grab token refresh bias mutex: {}", e))
        })?;
        *token_refresh_bias = refresh_times;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{credentials::TokenRequestOptions, http::Url, time::OffsetDateTime, Result};
    use azure_core_test::{recorded, TestContext};
    use std::sync::Arc;
    use tracing::info;

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
    // In production, incorrect token expiration could lead to authentication failures
    // or excessive token refresh operations, so this verification is critical.
    #[recorded::test]
    async fn token_credential_expiration(_ctx: TestContext) -> Result<()> {
        let url = Url::parse("amqps://example.com").unwrap();
        let path = Url::parse("amqps://example.com/test_token_credential_expiration").unwrap();

        // Create a mock token credential that expires in 15 seconds
        let mock_credential = MockTokenCredential::new(15);

        let connection_manager = RecoverableConnection::new(
            url,
            None,
            None,
            mock_credential.clone(),
            Default::default(),
        );

        let authorizer = Arc::new(Authorizer::new(
            Arc::downgrade(&connection_manager),
            mock_credential.clone(),
        ));

        // Disable actual authorization for testing
        authorizer.disable_authorization().unwrap();

        connection_manager.disable_connection().await.unwrap();

        // Expire tokens 10 seconds before they would normally expire.
        // The token in question expires in 15 seconds, so we want to refresh it before then.
        authorizer
            .set_token_refresh_times(TokenRefreshTimes {
                before_expiration_refresh_time: Duration::seconds(10),
                ..Default::default()
            })
            .unwrap();

        // This should succeed and store the token in the authorization scopes
        let result = authorizer.authorize_path(&connection_manager, &path).await;
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
        Ok(())
    }

    // The RecoverableConnection automatically refreshes tokens before they expire.
    // This is a critical feature for long-running connections, as it prevents
    // authentication failures due to expired tokens.
    //
    // This test verifies that the token refresh mechanism works correctly by:
    // 1. Creating a mock credential with a short expiration time
    // 2. Setting up the token refresh interval to be shorter than the token expiration
    // 3. Waiting long enough for a refresh to happen
    // 4. Verifying that additional token requests were made to the credential
    //
    // If this feature fails in production, clients would disconnect when their tokens expire,
    // which could lead to data loss, application failures, or service degradation.
    #[recorded::test]
    async fn token_refresh(_ctx: TestContext) -> Result<()> {
        let url = Url::parse("amqps://example.com").unwrap();
        let path = Url::parse("amqps://example.com/test_token_refresh").unwrap();

        // Create a mock token credential with a very short expiration (20 seconds)
        let mock_credential = MockTokenCredential::new(20);
        let connection_manager = RecoverableConnection::new(
            url,
            None,
            None,
            mock_credential.clone(),
            Default::default(),
        );

        connection_manager.disable_connection().await.unwrap();

        // Get initial token get count
        let initial_count = mock_credential.get_token_get_count();
        assert_eq!(initial_count, 0);

        let authorizer = Arc::new(Authorizer::new(
            Arc::downgrade(&connection_manager),
            mock_credential.clone(),
        ));

        // Disable actual authorization for testing
        authorizer.disable_authorization().unwrap();
        connection_manager.disable_connection().await.unwrap();

        // Set token refresh times to 10 seconds before expiration with default jitter.
        // This means we will refresh the token somewhere between 8 and 12 seconds before it expires.
        // The token in question expires in 20 seconds, so we want to refresh it before then.
        authorizer
            .set_token_refresh_times(TokenRefreshTimes {
                before_expiration_refresh_time: Duration::seconds(10),
                jitter_min: Duration::seconds(-2), // 2 seconds in milliseconds
                jitter_max: Duration::seconds(2),  // 2 seconds in milliseconds
            })
            .unwrap();

        // Authorize the path, which will store the token
        authorizer
            .authorize_path(&connection_manager, &path)
            .await
            .unwrap();

        // Verify initial token retrieval count - we will only have authorized the token once.
        let current_count = mock_credential.get_token_get_count();
        assert_eq!(current_count, 1);

        debug!("Sleeping for 15 seconds to allow token to expire and be refreshed. Current token count: {current_count}");

        // Sleep a bit to ensure we will have refreshed the token - since the token expires in 20 seconds,
        // we will refresh it between 8 and 12 seconds before the expiration time. If we wait for 13 seconds,
        // we should have refreshed the token.
        tokio::time::sleep(std::time::Duration::from_secs(13)).await;

        // Verify that the token get count has increased, indicating a refresh was attempted
        let final_count = mock_credential.get_token_get_count();
        debug!("After sleeping, token count: {final_count}");

        assert!(
            final_count >= 2,
            "Expected token get count to be greater or equal to 2, but got {final_count}"
        );
        info!("Final token get count: {final_count}");
        Ok(())
    }

    #[recorded::test]
    async fn multiple_token_refresh(_ctx: TestContext) -> Result<()> {
        let host = Url::parse("amqps://example.com").unwrap();
        // Create a mock token credential with a very short expiration (20 seconds) - we choose 20 seconds because we configure the token refresh bias (the time before expiration we will attempt a refresh to 5 seconds and there's a +- 5 second
        let mock_credential = MockTokenCredential::new(20);
        let recoverable_connection = Arc::new(RecoverableConnection::new(
            host.clone(),
            None,
            None,
            mock_credential.clone(),
            Default::default(),
        ));
        let authorizer = Arc::new(Authorizer::new(
            Arc::downgrade(&recoverable_connection),
            mock_credential.clone(),
        ));

        // Get initial token get count
        let initial_count = mock_credential.get_token_get_count();
        assert_eq!(initial_count, 0);

        // Disable actual authorization for testing
        authorizer.disable_authorization().unwrap();

        recoverable_connection.disable_connection().await.unwrap();

        // We will refresh the token 5 seconds before it expires (with jitter).
        // The token in question expires in 15 seconds, so we want to refresh it before then.
        // In practice, this means that we can guarantee that the token will be refreshed
        // between 4 and 6 seconds before it expires.
        authorizer
            .set_token_refresh_times(TokenRefreshTimes {
                before_expiration_refresh_time: Duration::seconds(5),
                jitter_min: Duration::milliseconds(-500),
                jitter_max: Duration::milliseconds(500),
            })
            .unwrap();

        // Authorize the path, which will store the token
        let path1 = Url::parse("amqps://example.com/test_token_refresh_1").unwrap();
        // Get access to the connection
        //let connection = connection_manager.ensure_connection().await.unwrap();
        authorizer
            .authorize_path(&recoverable_connection, &path1)
            .await
            .unwrap();

        // Because the token expires in 20 seconds, token_refresh_1 will be refreshed
        // between 14 and 16 seconds from now.

        // The second token expires after the first token.
        debug!("Sleeping for 10 seconds to establish separation between token_refresh_1 and token_refresh_2.");
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        // Authorize the second path, which will store the token
        let path2 = Url::parse("amqps://example.com/test_token_refresh_2").unwrap();
        authorizer
            .authorize_path(&recoverable_connection, &path2)
            .await
            .unwrap();

        // Verify initial token retrieval count - it should have been refreshed three times -
        let current_count = mock_credential.get_token_get_count();
        // Two paths are authorized, so we called get_token twice.
        assert_eq!(current_count, initial_count + 2);

        // Token_refresh_1 will be refreshed between 4 and 6 seconds from now.
        // Token_refresh_2 will be refreshed between 14 and 16 from now.
        debug!("Sleeping for 7 seconds to allow token_refresh_1 to expire and be refreshed. Current token count: {current_count}");
        tokio::time::sleep(std::time::Duration::from_secs(7)).await;

        // Verify that the token get count has increased, indicating a single refresh was attempted - we refreshed token_refresh_1 but not token_refresh_2.
        let final_count = mock_credential.get_token_get_count();
        debug!("After sleeping the first time, token count: {final_count}");
        assert!(
            final_count >= 3,
            "Expected first get token count to be at least 3, but got {final_count}"
        );

        info!("First token expiration get count: {}", final_count);
        // Token_refresh_1 will be refreshed between 13 and 15 seconds from now.
        // Token_refresh_2 will be refreshed between 7 and 9 seconds from now.

        // Sleep for 10 seconds to allow the second token to expire and be refreshed.
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        // Verify that the token get count has increased, indicating a single refresh was attempted - we refreshed token_refresh_2.
        let final_count = mock_credential.get_token_get_count();
        debug!("Getting second token count: {final_count}");
        assert!(
            final_count >= 4,
            "Expected second get token count to be 4, but got {final_count}"
        );
        info!("Second token expiration get count: {}", final_count);

        Ok(())
    }
}
