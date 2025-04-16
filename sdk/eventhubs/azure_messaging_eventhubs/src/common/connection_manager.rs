// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::common::user_agent::get_package_version;
use crate::{
    common::user_agent::{get_package_name, get_platform_info, get_user_agent},
    error::{ErrorKind, EventHubsError},
    models::AmqpValue,
};
use async_lock::{Mutex as AsyncMutex, OnceCell};
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    http::Url,
    task::{new_task_spawner, SpawnHandle},
    Result, Uuid,
};
use azure_core_amqp::{
    AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis as _, AmqpConnection, AmqpConnectionApis,
    AmqpConnectionOptions, AmqpSession, AmqpSessionApis as _, AmqpSymbol,
};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::{Arc, Mutex as SyncMutex, OnceLock};
use time::{Duration, OffsetDateTime};
use tracing::{debug, error, trace};

// The number of seconds before token expiration that we wake up to refresh the token.
const TOKEN_REFRESH_BIAS: Duration = Duration::minutes(6); // By default, we refresh tokens 6 minutes before they expire.
const TOKEN_REFRESH_JITTER_MIN: i64 = -5; // Minimum jitter in seconds
const TOKEN_REFRESH_JITTER_MAX: i64 = 5; // Maximum jitter in seconds

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

/// The connection manager is responsible for managing the connection to the Event Hubs service.
/// It also handles authorization and connection recovery.
///
/// Currently the connection manager only handles a *single* connection, eventually it will manage
/// a pool of connections to the service.
pub(crate) struct ConnectionManager {
    url: Url,
    application_id: Option<String>,
    custom_endpoint: Option<Url>,
    credential: Arc<dyn TokenCredential>,
    connections: OnceCell<Arc<AmqpConnection>>,
    authorization_scopes: AsyncMutex<HashMap<Url, AccessToken>>,
    authorization_refresher: OnceLock<SpawnHandle>,
    connection_name: String,
    /// Bias to apply to token refresh time. This determines how much time we will refresh the token before it expires.
    token_refresh_bias: SyncMutex<TokenRefreshTimes>,
    /// This is used to disable authorization for testing purposes.
    #[cfg(test)]
    disable_authorization: SyncMutex<bool>,
}

unsafe impl Send for ConnectionManager {}
unsafe impl Sync for ConnectionManager {}

const EVENTHUBS_AUTHORIZATION_SCOPE: &str = "https://eventhubs.azure.net/.default";

impl ConnectionManager {
    pub fn new(
        url: Url,
        application_id: Option<String>,
        custom_endpoint: Option<Url>,
        credential: Arc<dyn TokenCredential>,
    ) -> Arc<Self> {
        let connection_name = application_id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        Arc::new(Self {
            url,
            application_id,
            connection_name,
            custom_endpoint,
            credential,
            authorization_refresher: OnceLock::new(),
            connections: OnceCell::new(),
            authorization_scopes: AsyncMutex::new(HashMap::new()),
            token_refresh_bias: SyncMutex::new(TokenRefreshTimes::default()),
            #[cfg(test)]
            disable_authorization: SyncMutex::new(false),
        })
    }

    #[cfg(test)]
    fn disable_authorization(&self) -> Result<()> {
        let mut disable_authorization = self.disable_authorization.lock().map_err(|e| {
            azure_core::Error::message(azure_core::error::ErrorKind::Other, e.to_string())
        })?;
        *disable_authorization = true;
        Ok(())
    }

    #[cfg(test)]
    fn set_token_refresh_times(&self, refresh_times: TokenRefreshTimes) -> Result<()> {
        let mut token_refresh_bias = self.token_refresh_bias.lock().map_err(|e| {
            azure_core::Error::message(azure_core::error::ErrorKind::Other, e.to_string())
        })?;
        *token_refresh_bias = refresh_times;
        Ok(())
    }

    async fn create_connection(&self) -> Result<Arc<AmqpConnection>> {
        trace!("Creating connection for {}.", self.url);
        let connection = Arc::new(AmqpConnection::new());

        connection
            .open(
                self.connection_name.clone(),
                self.url.clone(),
                Some(AmqpConnectionOptions {
                    properties: Some(
                        vec![
                            ("user-agent", get_user_agent(&self.application_id)),
                            ("version", get_package_version()),
                            ("platform", get_platform_info()),
                            ("product", get_package_name()),
                        ]
                        .into_iter()
                        .map(|(k, v)| (AmqpSymbol::from(k), AmqpValue::from(v)))
                        .collect(),
                    ),
                    custom_endpoint: self.custom_endpoint.clone(),
                    ..Default::default()
                }),
            )
            .await?;
        Ok(connection)
    }

    pub(crate) async fn ensure_connection(&self) -> Result<()> {
        self.connections
            .get_or_try_init(|| self.create_connection())
            .await?;
        Ok(())
    }

    pub(crate) fn get_connection(&self) -> Result<Arc<AmqpConnection>> {
        let connection = self
            .connections
            .get()
            .ok_or_else(|| EventHubsError::from(ErrorKind::MissingConnection))?;
        Ok(connection.clone())
    }

    pub(crate) fn get_connection_id(&self) -> &str {
        &self.connection_name
    }

    pub(crate) async fn close_connection(&self) -> Result<()> {
        let connection = self.get_connection()?;

        connection.close().await
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
                let spawner = new_task_spawner();
                let self_clone = self.clone();
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
                debug!("No tokens to refresh, sleeping for {TOKEN_REFRESH_BIAS}.");
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
            let most_recent_refresh = expiration_times
                .first()
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?;

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
                    .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?;
                debug!("Token refresh bias with jitter: {token_refresh_bias}");

                refresh_time = most_recent_refresh
                    .checked_sub(token_refresh_bias)
                    .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?;
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
            // Because we cannot mutate the scopes map while we are iterating over it,
            // we create a map of updated tokens and then update the scopes map after we are done.
            let mut updated_tokens = HashMap::new();

            {
                let mut scopes = self.authorization_scopes.lock().await;
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

                    let new_token = self
                        .credential
                        .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE])
                        .await?;

                    // Create an ephemeral session to host the authentication.
                    self.perform_authorization(&self.get_connection()?, url, &new_token)
                        .await?;

                    debug!(
                        "Token refreshed for {url}, new expiration time: {}",
                        new_token.expires_on
                    );
                    updated_tokens.insert(url.clone(), new_token);
                }

                // Now that we have the set of refreshed tokens, we can update the map.
                for (url, token) in updated_tokens.into_iter() {
                    scopes.insert(url.clone(), token);
                }
                debug!("Updated tokens.");
            }
        }
        //        Ok(())
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
        let session = AmqpSession::new();
        session.begin(connection.as_ref(), None).await?;
        let cbs = AmqpClaimsBasedSecurity::new(&session)?;
        cbs.attach().await?;

        let expires_at = new_token.expires_on;
        cbs.authorize_path(
            url.to_string(),
            None,
            new_token.token.secret().to_string(),
            expires_at,
        )
        .await?;
        Ok(())
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

    // The ConnectionManager implementation uses a UUID to identify connections unless an application ID is provided.
    // This test verifies that a new connection manager uses a UUID for its connection ID when no application ID is specified.
    // It also verifies that the connections aren't initialized during construction - they're created on-demand.
    #[test]
    fn connection_manager() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection_manager =
            ConnectionManager::new(url, None, None, MockTokenCredential::new(15));
        assert!(!connection_manager.connections.is_initialized());
        assert!(connection_manager.get_connection().is_err()); // Should return an error since connection is not established yet
        assert_eq!(connection_manager.get_connection_id().len(), 36); // UUID v4 string length

        // verify that the connection_id can be parsed as a UUID.
        Uuid::parse_str(connection_manager.get_connection_id()).unwrap();
    }

    // When we construct a ConnectionManager with an application ID, the connection should use that ID
    // instead of generating a UUID. This test verifies that behavior.
    // Note: Using the actual application ID for the connection name helps with telemetry and debugging
    // in production scenarios.
    #[test]
    fn connection_manager_with_application_id() {
        let url = Url::parse("amqps://example.com").unwrap();
        let app_id = "test-app-id".to_string();
        let connection_manager = ConnectionManager::new(
            url,
            Some(app_id.clone()),
            None,
            MockTokenCredential::new(15),
        );
        assert!(!connection_manager.connections.is_initialized());
        assert_eq!(connection_manager.get_connection_id(), app_id);
    }

    /// Verifies that a new connection is not open by default.
    ///
    /// # Panics
    ///
    /// Panics if the connection is open.
    #[tokio::test]
    async fn connection_is_not_open_by_default() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection_manager = Arc::new(ConnectionManager::new(
            url.clone(),
            None,
            None,
            MockTokenCredential::new(15),
        ));

        // This should fail because we don't have a connection established
        let connection = connection_manager.get_connection();
        assert!(connection.is_err());
    }

    // The ConnectionManager supports using a custom endpoint for connecting to Event Hubs proxies.
    // This test verifies that the custom endpoint is properly stored in the ConnectionManager.
    #[test]
    fn constructor_with_custom_endpoint() {
        let url = Url::parse("amqps://example.com").unwrap();
        let custom_endpoint = Url::parse("https://custom-endpoint.com").unwrap();
        let connection_manager = ConnectionManager::new(
            url,
            None,
            Some(custom_endpoint.clone()),
            MockTokenCredential::new(15),
        );

        assert_eq!(connection_manager.custom_endpoint, Some(custom_endpoint));
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

        let connection_manager = Arc::new(ConnectionManager::new(url, None, None, mock_credential));

        // Override default connection creation for testing
        connection_manager
            .connections
            .get_or_init(|| async { Arc::new(AmqpConnection::new()) })
            .await;

        // Disable actual authorization for testing
        connection_manager.disable_authorization().unwrap();

        // Expire tokens 10 seconds before they would normally expire.
        // The token in question expires in 15 seconds, so we want to refresh it before then.
        connection_manager
            .set_token_refresh_times(TokenRefreshTimes {
                before_expiration_refresh_time: Duration::seconds(10),
                ..Default::default()
            })
            .unwrap();

        // Get access to the connection
        let connection = connection_manager.get_connection().unwrap();

        // This should succeed and store the token in the authorization scopes
        let result = connection_manager.authorize_path(&connection, &path).await;
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        // Verify token is stored
        let scopes = connection_manager.authorization_scopes.lock().await;
        assert!(scopes.contains_key(&path));

        // Verify expiration time
        let stored_token = scopes.get(&path).unwrap();
        let now = OffsetDateTime::now_utc();
        assert!(stored_token.expires_on > now);
        assert!(stored_token.expires_on < now + Duration::seconds(15)); // Should be less than now + 15 seconds
    }

    // The ConnectionManager automatically refreshes tokens before they expire.
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
        let connection_manager = ConnectionManager::new(url, None, None, mock_credential.clone());

        // Get initial token get count
        let initial_count = mock_credential.get_token_get_count();
        assert_eq!(initial_count, 0);

        // Override default connection creation for testing
        connection_manager
            .connections
            .get_or_init(|| async { Arc::new(AmqpConnection::new()) })
            .await;

        // Disable actual authorization for testing
        connection_manager.disable_authorization().unwrap();

        // Set token refresh times to 6 seconds before expiration with default jitter.
        // This means we will refresh the token somewhere between 1 and 11 seconds before it expires.
        // The token in question expires in 20 seconds, so we want to refresh it before then.
        connection_manager
            .set_token_refresh_times(TokenRefreshTimes {
                before_expiration_refresh_time: Duration::seconds(6),
                ..Default::default()
            })
            .unwrap();

        // Get access to the connection
        let connection = connection_manager.get_connection().unwrap();

        // Authorize the path, which will store the token
        connection_manager
            .authorize_path(&connection, &path)
            .await
            .unwrap();

        // Verify initial token retrieval count - we will only have authorized the token once.
        let current_count = mock_credential.get_token_get_count();
        assert_eq!(current_count, 1);

        debug!("Sleeping for 14 seconds to allow token to expire and be refreshed. Current token count: {current_count}");

        // Sleep a bit to ensure we will have refreshed the token - since the token expires in 20 seconds,
        // we will refresh it between 1 and 11 seconds before the expiration time. If we wait for 15 seconds,
        // we should have refreshed the token.
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;

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
        let connection_manager = ConnectionManager::new(host, None, None, mock_credential.clone());

        // Get initial token get count
        let initial_count = mock_credential.get_token_get_count();
        assert_eq!(initial_count, 0);

        // Override default connection creation for testing
        connection_manager
            .connections
            .get_or_init(|| async { Arc::new(AmqpConnection::new()) })
            .await;

        // Disable actual authorization for testing
        connection_manager.disable_authorization().unwrap();

        // We will refresh the token 5 seconds before it expires (with jitter).
        // The token in question expires in 15 seconds, so we want to refresh it before then.
        // In practice, this means that we can guarantee that the token will be refreshed
        // between 4 and 6 seconds before it expires.
        connection_manager
            .set_token_refresh_times(TokenRefreshTimes {
                before_expiration_refresh_time: Duration::seconds(5),
                jitter_min: -1,
                jitter_max: 1,
            })
            .unwrap();

        // Authorize the path, which will store the token
        let path1 = Url::parse("amqps://example.com/test_token_refresh_1").unwrap();
        // Get access to the connection
        let connection = connection_manager.get_connection().unwrap();
        connection_manager
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
        connection_manager
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
