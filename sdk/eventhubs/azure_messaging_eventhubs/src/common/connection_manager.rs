// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::common::user_agent::get_package_version;
use crate::{
    common::user_agent::{get_package_name, get_platform_info, get_user_agent},
    error::{ErrorKind, EventHubsError},
    models::AmqpValue,
};
use async_lock::{Mutex, OnceCell};
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::ErrorKind as AzureErrorKind,
    http::Url,
    sleep::sleep,
    task::{new_task_spawner, SpawnHandle},
    Result, Uuid,
};
use azure_core_amqp::{
    AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis as _, AmqpConnection, AmqpConnectionApis,
    AmqpConnectionOptions, AmqpSession, AmqpSessionApis as _, AmqpSymbol,
};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use time::{Duration, OffsetDateTime};
use tracing::{debug, error, trace};

struct AuthorizationScope {
    access_token: AccessToken,
    credential: Arc<dyn TokenCredential>,
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
    connections: OnceCell<Arc<AmqpConnection>>,
    authorization_scopes: Mutex<HashMap<Url, AuthorizationScope>>,
    authorization_refresher: OnceLock<SpawnHandle>,
    connection_name: String,
    #[cfg(test)]
    mock_connection: OnceLock<Arc<dyn AsRef<AmqpConnection>>>,
}

unsafe impl Send for ConnectionManager {}
unsafe impl Sync for ConnectionManager {}

const TOKEN_REFRESH_EXPIRATION: Duration = Duration::minutes(6); // 6 minutes
const EVENTHUBS_AUTHORIZATION_SCOPE: &str = "https://eventhubs.azure.net/.default";

impl ConnectionManager {
    pub fn new(
        url: Url,
        application_id: Option<String>,
        custom_endpoint: Option<Url>,
    ) -> Arc<Self> {
        let connection_name = application_id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        Arc::new(Self {
            url,
            application_id,
            connection_name,
            custom_endpoint,
            authorization_refresher: OnceLock::new(),
            connections: OnceCell::new(),
            authorization_scopes: Mutex::new(HashMap::new()),
            #[cfg(test)]
            mock_connection: OnceLock::new(),
        })
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

    pub(crate) fn get_connection(&self) -> Result<Arc<dyn AmqpConnectionApis>> {
        #[cfg(test)]
        if let Some(mock_connection) = self.mock_connection.get() {
            return Ok(mock_connection.clone());
        }

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

    #[cfg(test)]
    pub(crate) fn set_mock_connection(
        &self,
        connection: Arc<impl AsRef<AmqpConnection> + 'static>,
    ) -> Result<()> {
        if self
            .mock_connection
            .set(connection as Arc<dyn AsRef<AmqpConnection>>)
            .is_err()
        {
            return Err(EventHubsError::from(ErrorKind::UnableToSetMockConnection).into());
        }
        Ok(())
    }

    pub(crate) async fn authorize_path(
        self: &Arc<Self>,
        connection: &Arc<AmqpConnection>,
        path: &Url,
        credential: Arc<dyn TokenCredential>,
    ) -> Result<AccessToken> {
        debug!("Authorizing path: {path}");
        let mut scopes = self.authorization_scopes.lock().await;

        if !scopes.contains_key(path) {
            // Create an ephemeral session to host the authentication.
            let session = AmqpSession::new();
            session.begin(connection.as_ref(), None).await?;

            let cbs = AmqpClaimsBasedSecurity::new(&session)?;
            cbs.attach().await?;

            debug!("Get Token.");
            let token = credential
                .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE])
                .await?;
            let expires_at = token.expires_on;
            cbs.authorize_path(
                path.to_string(),
                None,
                token.token.secret().to_string(),
                expires_at,
            )
            .await?;

            // insert returns some if it *fails* to insert, None if it succeeded.
            let present = scopes.insert(
                path.clone(),
                AuthorizationScope {
                    credential: credential.clone(),
                    access_token: token,
                },
            );
            if present.is_some() {
                return Err(EventHubsError::from(ErrorKind::UnableToAddAuthenticationToken).into());
            }

            self.authorization_refresher.get_or_init(|| {
                let spawner = new_task_spawner();
                let self_clone = self.clone();
                spawner.spawn_boxed(Box::new(async move {
                    let r = self_clone.refresh_tokens().await;
                    if let Err(e) = r {
                        error!("Error refreshing tokens: {e}");
                    }
                }))
            });
            trace!("Token added.");
        }
        Ok(scopes
            .get(path)
            .ok_or_else(|| EventHubsError::from(ErrorKind::UnableToAddAuthenticationToken))?
            .access_token
            .clone())
    }

    async fn refresh_tokens(self: &Arc<Self>) -> Result<()> {
        debug!("Refreshing tokens.");
        let mut scopes = self.authorization_scopes.lock().await;
        let mut expiration_times = vec![];
        for (_, token) in scopes.iter() {
            expiration_times.push(token.access_token.expires_on);
        }
        expiration_times.sort();

        let expiration_jitter = Duration::seconds(thread_rng().gen_range(0..=10));

        loop {
            // Calculate duration until we should refresh (6 minutes before expiration,
            // with added random jitter)
            let now = OffsetDateTime::now_utc();
            let refresh_time = expiration_times[0]
                .checked_sub(
                    TOKEN_REFRESH_EXPIRATION
                        .checked_add(expiration_jitter)
                        .unwrap(),
                )
                .unwrap();

            // Convert to a duration if refresh time is in the future and sleep until it's time
            // to refresh the token.
            if refresh_time > now {
                let sleep_duration = refresh_time - now;
                let std_duration =
                    std::time::Duration::from_secs_f64(sleep_duration.as_seconds_f64());
                sleep(std_duration).await;
            }

            // Refresh the tokens.
            // Because we cannot mutate the scopes map while we are iterating over it,
            // we create a map of updated tokens and then update the scopes map after we are done.
            let mut updated_tokens = HashMap::new();

            for (url, token) in scopes.iter() {
                if token.access_token.expires_on >= now + TOKEN_REFRESH_EXPIRATION {
                    debug!("Token not expired for {url}");
                }

                let credential = token.credential.clone();
                let new_token = credential
                    .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE])
                    .await?;

                // Create an ephemeral session to host the authentication.
                let session = AmqpSession::new();
                session
                    .begin(
                        self.connections
                            .get()
                            .ok_or_else(|| {
                                azure_core::Error::message(
                                    AzureErrorKind::Other,
                                    "Could not retrieve connection?",
                                )
                            })?
                            .as_ref(),
                        None,
                    )
                    .await?;

                let cbs = AmqpClaimsBasedSecurity::new(&session)?;
                cbs.attach().await?;

                debug!("Refreshing Token.");

                let expires_at = new_token.expires_on;
                cbs.authorize_path(
                    url.to_string(),
                    None,
                    new_token.token.secret().to_string(),
                    expires_at,
                )
                .await?;

                updated_tokens.insert(
                    url.clone(),
                    AuthorizationScope {
                        access_token: new_token,
                        credential,
                    },
                );
                debug!("Token refreshed for {url}");
            }

            // Now that we have the set of refreshed tokens, we can update the map.
            for (url, token) in updated_tokens.into_iter() {
                scopes.insert(url.clone(), token);
            }
        }
        //        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::testing::{
        MockAmqpClaimsBasedSecurity, MockAmqpConnection, MockAmqpSession,
    };
    use async_trait::async_trait;
    use azure_core::{http::Url, Result};
    use azure_core_test::{recorded, TestContext};
    use std::{sync::Arc, time::Duration as StdDuration};
    use time::OffsetDateTime;

    // Helper struct to mock token credential
    #[derive(Debug)]
    struct MockTokenCredential {
        token: AccessToken,
    }

    impl MockTokenCredential {
        fn new(expires_in_seconds: i64) -> Self {
            let expires_on = OffsetDateTime::now_utc() + Duration::seconds(expires_in_seconds);
            Self {
                token: AccessToken::new(
                    azure_core::credentials::Secret::new("mock_token"),
                    expires_on,
                ),
            }
        }
    }

    #[async_trait]
    impl TokenCredential for MockTokenCredential {
        async fn get_token(&self, _scopes: &[&str]) -> Result<AccessToken> {
            Ok(self.token.clone())
        }
    }

    #[test]
    fn test_connection_manager() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection_manager = ConnectionManager::new(url, None, None);
        assert!(!connection_manager.connections.is_initialized());
        assert!(connection_manager.get_connection().is_err()); // Should return an error since connection is not established yet
        assert_eq!(connection_manager.get_connection_id().len(), 36); // UUID v4 string length
    }

    #[test]
    fn test_connection_manager_with_application_id() {
        let url = Url::parse("amqps://example.com").unwrap();
        let app_id = "test-app-id".to_string();
        let connection_manager = ConnectionManager::new(url, Some(app_id.clone()), None);
        assert!(!connection_manager.connections.is_initialized());
        assert_eq!(connection_manager.get_connection_id(), app_id);
    }

    #[tokio::test]
    async fn test_authorize_path_errors_without_connection() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection_manager = Arc::new(ConnectionManager::new(url.clone(), None, None));
        let credential = Arc::new(MockTokenCredential::new(300)) as Arc<dyn TokenCredential>;
        let path = Url::parse("amqps://example.com/path").unwrap();

        // This should fail because we don't have a connection established
        let connection = connection_manager.get_connection();
        assert!(connection.is_err());
    }

    #[test]
    fn test_constructor_with_custom_endpoint() {
        let url = Url::parse("amqps://example.com").unwrap();
        let custom_endpoint = Url::parse("https://custom-endpoint.com").unwrap();
        let connection_manager = ConnectionManager::new(url, None, Some(custom_endpoint.clone()));

        assert_eq!(connection_manager.custom_endpoint, Some(custom_endpoint));
    }

    #[tokio::test]
    async fn test_create_connection_with_mocks() {
        // Arrange
        let url = Url::parse("amqps://example.com").unwrap();
        let app_id = "test-connection-id".to_string();

        // Create a ConnectionManager with mocked connection capabilities
        let connection_manager = ConnectionManager::new(url, Some(app_id.clone()), None);

        // Create a mock connection and set it in the manager
        let mock_connection = Arc::new(MockAmqpConnection::new());
        connection_manager
            .set_mock_connection(mock_connection)
            .unwrap();

        // Act
        let connection = connection_manager.get_connection();

        // Assert
        assert!(connection.is_ok());
    }

    #[tokio::test]
    async fn test_close_connection() {
        // Arrange
        let url = Url::parse("amqps://example.com").unwrap();
        let connection_manager = ConnectionManager::new(url, None, None);

        // Create and set a mock connection
        let mock_connection = Arc::new(MockAmqpConnection::new());
        connection_manager
            .set_mock_connection(mock_connection)
            .unwrap();

        // Act & Assert
        let connection_result = connection_manager.get_connection();
        assert!(connection_result.is_ok());

        let close_result = connection_manager.close_connection().await;
        assert!(close_result.is_ok());
    }

    #[test]
    fn test_set_mock_connection() {
        // Arrange
        let url = Url::parse("amqps://example.com").unwrap();
        let connection_manager = ConnectionManager::new(url, None, None);
        let mock_connection = Arc::new(MockAmqpConnection::new());

        // Act
        let result = connection_manager.set_mock_connection(mock_connection);

        // Assert
        assert!(result.is_ok());

        // Setting it again should fail
        let second_mock = Arc::new(MockAmqpConnection::new());
        let second_result = connection_manager.set_mock_connection(second_mock);
        assert!(second_result.is_err());
    }
}
