// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{recoverable_connection::RecoverableConnection, RetryOptions};
use crate::{
    common::retry_azure_operation,
    error::{ErrorKind, EventHubsError},
};
use async_lock::Mutex as AsyncMutex;
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::ErrorKind as AzureErrorKind,
    http::Url,
    task::{new_task_spawner, SpawnedTask},
    Result,
};
use azure_core_amqp::{
    error::{AmqpErrorCondition, AmqpErrorKind},
    AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis as _, AmqpConnection, AmqpError,
    AmqpSession, AmqpSessionApis,
};
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex as SyncMutex, OnceLock, Weak};
use std::{collections::HashMap, error::Error as _};
use time::{Duration, OffsetDateTime};
use tracing::{debug, error, trace, warn};

// The number of seconds before token expiration that we wake up to refresh the token.
const TOKEN_REFRESH_BIAS: Duration = Duration::minutes(6); // By default, we refresh tokens 6 minutes before they expire.
const TOKEN_REFRESH_JITTER_MIN: i64 = -5; // Minimum jitter in seconds
const TOKEN_REFRESH_JITTER_MAX: i64 = 5; // Maximum jitter in seconds

const EVENTHUBS_AUTHORIZATION_SCOPE: &str = "https://eventhubs.azure.net/.default";

#[derive(Debug)]
struct TokenRefreshTimes {
    before_expiration_refresh_time: Duration,
    jitter_min: i64,
    jitter_max: i64,
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
    retry_options: RetryOptions,
    recoverable_connection: Weak<RecoverableConnection>,
    /// This is used to disable authorization for testing purposes.
    #[cfg(test)]
    disable_authorization: SyncMutex<bool>,
}

unsafe impl Send for Authorizer {}
unsafe impl Sync for Authorizer {}

impl Authorizer {
    pub fn new(
        recoverable_connection: Weak<RecoverableConnection>,
        credential: Arc<dyn TokenCredential>,
    ) -> Self {
        Self {
            authorization_refresher: OnceLock::new(),
            authorization_scopes: AsyncMutex::new(HashMap::new()),
            token_refresh_bias: SyncMutex::new(TokenRefreshTimes::default()),
            credential,
            retry_options: RetryOptions::default(),
            recoverable_connection,
            #[cfg(test)]
            disable_authorization: SyncMutex::new(false),
        }
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
                .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE])
                .await?;

            debug!("Token for path {path} expires at {}", token.expires_on);

            self.perform_authorization(connection, path, &token).await?;

            // insert returns some if it *fails* to insert, None if it succeeded.
            let present = scopes.insert(path.clone(), token);
            if present.is_some() {
                return Err(EventHubsError::from(ErrorKind::UnableToAddAuthenticationToken).into());
            }

            debug!("Token verified.");
            self.authorization_refresher.get_or_init(|| {
                debug!("Starting authorization refresh task.");
                let self_clone = self.clone();
                let spawner = new_task_spawner();
                spawner.spawn(Box::pin(self_clone.refresh_tokens_task()))
            });
        } else {
            debug!("Token already exists for path: {path}");
        }
        Ok(scopes
            .get(path)
            .ok_or_else(|| EventHubsError::from(ErrorKind::UnableToAddAuthenticationToken))?
            .clone())
    }

    #[cfg(test)]
    fn disable_authorization(&self) -> Result<()> {
        let mut disable_authorization = self.disable_authorization.lock().map_err(|e| {
            azure_core::Error::message(azure_core::error::ErrorKind::Other, e.to_string())
        })?;
        *disable_authorization = true;
        Ok(())
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
        connection: &Arc<AmqpConnection>,
        url: &Url,
        new_token: &AccessToken,
    ) -> Result<()> {
        // Test Hook: Disable interacting with Event Hubs service if the test doesn't want it.
        #[cfg(test)]
        {
            let disable_authorization = self.disable_authorization.lock().map_err(|e| {
                azure_core::Error::message(
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

        retry_azure_operation(
            || async {
                let connection = connection.clone();
                let path = url.to_string();
                let token = new_token.token.secret().to_string();
                let expires_at = new_token.expires_on;

                let session = AmqpSession::new();
                session.begin(connection.as_ref(), None).await?;
                let cbs = AmqpClaimsBasedSecurity::new(&session)?;
                cbs.attach().await?;

                cbs.authorize_path(path, None, token, expires_at).await?;
                Ok(())
            },
            &self.retry_options,
            Some(Self::should_retry_cbs_response),
        )
        .await?;
        Ok(())
    }

    async fn refresh_tokens_task(self: Arc<Self>) {
        let result = self.refresh_tokens().await;
        if let Err(e) = result {
            error!("Error refreshing tokens: {e}");
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
                debug!("No tokens to refresh. Sleeping for {TOKEN_REFRESH_BIAS}.");
                azure_core::sleep::sleep(
                    std::time::Duration::try_from(TOKEN_REFRESH_BIAS).map_err(|e| {
                        azure_core::Error::new(azure_core::error::ErrorKind::Other, e)
                    })?,
                )
                .await;
                continue;
            }

            // Calculate duration until we should refresh (6 minutes before expiration,
            // with added random jitter)

            let mut now = OffsetDateTime::now_utc();
            trace!("refresh_tokens: Start pass for: {now}");
            let most_recent_refresh = expiration_times.first().ok_or_else(|| {
                azure_core::Error::message(AzureErrorKind::Other, "No tokens to refresh?")
            })?;

            debug!(
                "Nearest token refresh time: {most_recent_refresh}, in {}",
                *most_recent_refresh - now
            );

            let refresh_time: OffsetDateTime;
            let token_refresh_bias: Duration;
            {
                let token_refresh_times = self.token_refresh_bias.lock().map_err(|e| {
                    azure_core::Error::message(
                        azure_core::error::ErrorKind::Other,
                        format!("Unable to grab token refresh bias mutex: {}", e),
                    )
                })?;

                debug!("Token refresh times: {token_refresh_times:?}");

                let expiration_jitter = Duration::seconds(
                    thread_rng()
                        .gen_range(token_refresh_times.jitter_min..token_refresh_times.jitter_max),
                );
                debug!("Expiration jitter: {expiration_jitter}");

                token_refresh_bias = token_refresh_times
                    .before_expiration_refresh_time
                    .checked_add(expiration_jitter)
                    .ok_or_else(|| {
                        azure_core::Error::message(
                            AzureErrorKind::Other,
                            "Unable to calculate token refresh bias - overflow",
                        )
                    })?;
                debug!("Token refresh bias with jitter: {token_refresh_bias}");

                refresh_time = most_recent_refresh
                    .checked_sub(token_refresh_bias)
                    .ok_or_else(|| {
                        azure_core::Error::message(
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
                let std_duration = std::time::Duration::try_from(sleep_duration)
                    .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;
                debug!(
                    "refresh_tokens: Sleeping for {std_duration:?} until {:?}",
                    now + std_duration
                );
                azure_core::sleep::sleep(std_duration).await;
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
                            "Token not expired for {url}: ExpiresOn: {}, Now: {}, Bias: {}",
                            token.expires_on, now, token_refresh_bias
                        );
                        continue;
                    }

                    debug!(
                        "Token about to be expired for {url}: ExpiresOn: {}, Now: {}, Bias: {}",
                        token.expires_on, now, token_refresh_bias
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
                    .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE])
                    .await?;

                // Create an ephemeral connection to host the authentication.
                let connection = self
                    .recoverable_connection
                    .upgrade()
                    .ok_or_else(|| {
                        azure_core::Error::message(
                            AzureErrorKind::Other,
                            "Recoverable connection has been dropped",
                        )
                    })?
                    .ensure_connection()
                    .await?;
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
            azure_core::Error::message(azure_core::error::ErrorKind::Other, e.to_string())
        })?;
        *token_refresh_bias = refresh_times;
        Ok(())
    }

    fn should_retry_cbs_response(e: &azure_core::Error) -> bool {
        match e.kind() {
            AzureErrorKind::Amqp => {
                warn!("Amqp operation failed: {:?}", e.source());
                if let Some(e) = e.source() {
                    debug!("Error: {}", e);

                    if let Some(amqp_error) = e.downcast_ref::<Box<AmqpError>>() {
                        Self::should_retry_amqp_error(amqp_error)
                    } else if let Some(amqp_error) = e.downcast_ref::<AmqpError>() {
                        Self::should_retry_amqp_error(amqp_error)
                    } else {
                        debug!("Non AMQP error: {}", e);
                        false
                    }
                } else {
                    debug!("No source error found");
                    false
                }
            }
            _ => {
                debug!("Non AMQP error: {}", e);
                false
            }
        }
    }

    fn should_retry_amqp_error(amqp_error: &AmqpError) -> bool {
        match amqp_error.kind() {
            AmqpErrorKind::ManagementStatusCode(code, _) => {
                debug!("Management operation error: {}", code);
                matches!(
                    code,
                    azure_core::http::StatusCode::RequestTimeout
                        | azure_core::http::StatusCode::TooManyRequests
                        | azure_core::http::StatusCode::InternalServerError
                        | azure_core::http::StatusCode::BadGateway
                        | azure_core::http::StatusCode::ServiceUnavailable
                        | azure_core::http::StatusCode::GatewayTimeout
                )
            }
            AmqpErrorKind::AmqpDescribedError(described_error) => {
                debug!("AMQP described error: {:?}", described_error);
                matches!(
                    described_error.condition(),
                    AmqpErrorCondition::ResourceLimitExceeded
                        | AmqpErrorCondition::ConnectionFramingError
                        | AmqpErrorCondition::LinkStolen
                )
            }
            _ => {
                debug!("Other AMQP error: {}", amqp_error);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use azure_core::{http::Url, Result};
    use std::sync::Arc;
    use time::OffsetDateTime;
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

    #[async_trait]
    impl TokenCredential for MockTokenCredential {
        async fn get_token(&self, _scopes: &[&str]) -> Result<AccessToken> {
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
    #[tokio::test]
    async fn token_credential_expiration() {
        crate::consumer::tests::setup();

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

        // Get access to the connection
        let connection = connection_manager.ensure_connection().await.unwrap();

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
    #[tokio::test]
    async fn token_refresh() {
        crate::consumer::tests::setup();

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
                jitter_min: -2,
                jitter_max: 2,
            })
            .unwrap();

        // Get access to the connection
        let connection = connection_manager.ensure_connection().await.unwrap();

        // Authorize the path, which will store the token
        authorizer.authorize_path(&connection, &path).await.unwrap();

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

        assert_eq!(
            final_count, 2,
            "Expected token get count to be 2, but got {final_count}"
        );
        info!("Final token get count: {final_count}");
    }

    #[tokio::test]
    async fn multiple_token_refresh() -> Result<()> {
        crate::consumer::tests::setup();

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
                jitter_min: -1,
                jitter_max: 1,
            })
            .unwrap();

        let connection = recoverable_connection.ensure_connection().await.unwrap();

        // Authorize the path, which will store the token
        let path1 = Url::parse("amqps://example.com/test_token_refresh_1").unwrap();
        // Get access to the connection
        //let connection = connection_manager.ensure_connection().await.unwrap();
        authorizer
            .authorize_path(&connection, &path1)
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
            .authorize_path(&connection, &path2)
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
        assert_eq!(
            final_count, 3,
            "Expected first get token count to be 3, but got {}",
            final_count
        );

        info!("First token expiration get count: {}", final_count);
        // Token_refresh_1 will be refreshed between 13 and 15 seconds from now.
        // Token_refresh_2 will be refreshed between 7 and 9 seconds from now.

        // Sleep for 10 seconds to allow the second token to expire and be refreshed.
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        // Verify that the token get count has increased, indicating a single refresh was attempted - we refreshed token_refresh_2.
        let final_count = mock_credential.get_token_get_count();
        debug!("Getting second token count: {final_count}");
        assert_eq!(
            final_count, 4,
            "Expected second get token count to be 4, but got {final_count}"
        );
        info!("Second token expiration get count: {}", final_count);

        Ok(())
    }
}
