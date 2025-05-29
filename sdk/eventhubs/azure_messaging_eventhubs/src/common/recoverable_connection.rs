// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::common::retry_azure_operation;
use crate::common::user_agent::get_package_version;
use crate::{
    common::user_agent::{get_package_name, get_platform_info, get_user_agent},
    models::AmqpValue,
};
use async_lock::{Mutex as AsyncMutex, OnceCell};
use async_trait::async_trait;
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::ErrorKind as AzureErrorKind,
    http::Url,
    Result, Uuid,
};
use azure_core_amqp::error::{AmqpErrorCondition, AmqpErrorKind};
use azure_core_amqp::{
    AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions, AmqpError, AmqpManagement,
    AmqpManagementApis, AmqpOrderedMap, AmqpSender, AmqpSession, AmqpSessionApis as _,
    AmqpSimpleValue, AmqpSymbol,
};
use std::{collections::HashMap, error::Error, sync::Arc};
use tracing::{debug, trace, warn};

use super::authorizer::Authorizer;
use super::RetryOptions;

struct SenderInstance {
    session: AmqpSession,
    sender: Arc<AmqpSender>,
}

/// The recoverable connection is responsible for managing the connection to the Event Hubs service.
/// It also handles authorization and connection recovery.
///
/// Currently the recoverable connection only handles a *single* connection, eventually it will manage
/// a pool of connections to the service.
pub(crate) struct RecoverableConnection {
    url: Url,
    application_id: Option<String>,
    custom_endpoint: Option<Url>,
    connections: OnceCell<Arc<AmqpConnection>>,
    mgmt_client: OnceCell<Arc<AmqpManagement>>,
    sender_instances: AsyncMutex<HashMap<Url, SenderInstance>>,

    authorizer: Option<Arc<Authorizer>>,

    connection_name: String,
    retry_options: RetryOptions,
}

unsafe impl Send for RecoverableConnection {}
unsafe impl Sync for RecoverableConnection {}

impl RecoverableConnection {
    pub fn new(
        url: Url,
        application_id: Option<String>,
        custom_endpoint: Option<Url>,
        credential: Arc<dyn TokenCredential>,
        retry_options: RetryOptions,
    ) -> Arc<Self> {
        let connection_name = application_id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        Arc::new_cyclic(|weak_rc| {
            let authorizer = Arc::new(Authorizer::new(weak_rc.clone(), credential));

            Self {
                url,
                application_id,
                connection_name,
                custom_endpoint,
                retry_options,
                connections: OnceCell::new(),
                sender_instances: AsyncMutex::new(HashMap::new()),
                mgmt_client: OnceCell::new(),
                authorizer: Some(authorizer),
            }
        })
    }

    #[cfg(test)]
    pub(crate) async fn disable_connection(&self) -> Result<()> {
        self.connections
            .get_or_init(|| async { Arc::new(AmqpConnection::new()) })
            .await;
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

    pub(crate) async fn authorize_path(
        &self,
        connection: &Arc<AmqpConnection>,
        path: &Url,
    ) -> Result<AccessToken> {
        self.authorizer
            .as_ref()
            .unwrap()
            .authorize_path(connection, path)
            .await
    }

    pub(crate) async fn ensure_connection(&self) -> Result<Arc<AmqpConnection>> {
        let connection = self
            .connections
            .get_or_try_init(|| self.create_connection())
            .await?;
        Ok(connection.clone())
    }

    async fn ensure_amqp_management(self: &Arc<Self>) -> Result<Arc<AmqpManagement>> {
        let management_client = self
            .mgmt_client
            .get_or_try_init(|| self.create_management_client())
            .await?;

        Ok(management_client.clone())
    }
    async fn create_management_client(self: &Arc<Self>) -> Result<Arc<AmqpManagement>> {
        // Clients must call ensure_connection before calling ensure_management_client.

        trace!("Create management session.");
        let connection = self.ensure_connection().await?;

        let session = AmqpSession::new();
        session.begin(connection.as_ref(), None).await?;
        trace!("Session created.");

        let management_path = self.url.to_string() + "/$management";
        let management_path = Url::parse(&management_path)?;
        let access_token = self
            .authorizer
            .as_ref()
            .unwrap()
            .authorize_path(&connection, &management_path)
            .await?;

        trace!("Create management client.");
        let management = Arc::new(AmqpManagement::new(
            session,
            "eventhubs_management".to_string(),
            access_token,
        )?);
        management.attach().await?;
        Ok(management)
    }

    fn should_retry_management_response(e: &azure_core::Error) -> bool {
        match e.kind() {
            AzureErrorKind::Amqp => {
                warn!("Amqp operation failed: {}", e.source().unwrap());
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

    pub(crate) async fn get_management_client(
        self: &Arc<Self>,
    ) -> Result<Arc<AmqpManagementClient>> {
        Ok(Arc::new(AmqpManagementClient {
            management_client: self.ensure_amqp_management().await?,
            recoverable_connection: self.clone(),
        }))
    }

    //     pub(crate) async fn get_sender(self: &Arc<Self>, path: &str) -> Result<Arc<AmqpConnection>> {
    //         self.ensure_connection().await?;
    //         let connection = self.ensure_connection().await?;

    //         // Create a sender for the Event Hub.
    //         let sender = self.create_sender(path).await?;
    //         Ok(sender)
    //     }

    pub(crate) fn get_connection_id(&self) -> &str {
        &self.connection_name
    }

    pub(crate) async fn close_connection(&self) -> Result<()> {
        let connection = self.ensure_connection().await?;

        connection.close().await
    }

    // async fn ensure_sender(&self, path: &Url) -> Result<Arc<AmqpSender>> {
    //     let mut sender_instances = self.sender_instances.lock().await;
    //     if !sender_instances.contains_key(path) {
    //         let connection = self.connection.ensure_connection().await?;

    //         self.connection.authorize_path(&connection, path).await?;
    //         let session = AmqpSession::new();
    //         session
    //             .begin(
    //                 connection.as_ref(),
    //                 Some(AmqpSessionOptions {
    //                     incoming_window: Some(u32::MAX),
    //                     outgoing_window: Some(u32::MAX),
    //                     ..Default::default()
    //                 }),
    //             )
    //             .await?;
    //         let sender = AmqpSender::new();
    //         sender
    //             .attach(
    //                 &session,
    //                 format!(
    //                     "{}-rust-sender",
    //                     self.application_id
    //                         .as_ref()
    //                         .unwrap_or(&DEFAULT_EVENTHUBS_APPLICATION.to_string())
    //                 ),
    //                 path.to_string(),
    //                 None,
    //             )
    //             .await?;
    //         sender_instances.insert(
    //             path.clone(),
    //             SenderInstance {
    //                 session,
    //                 sender: Arc::new(sender),
    //             },
    //         );
    //     }
    //     Ok(sender_instances
    //         .get(path)
    //         .ok_or_else(|| EventHubsError::from(ErrorKind::MissingMessageSender))?
    //         .sender
    //         .clone())
    // }

    fn should_retry_cbs_response(e: &azure_core::Error) -> bool {
        match e.kind() {
            AzureErrorKind::Amqp => {
                warn!("Amqp operation failed: {}", e.source().unwrap());
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

pub(crate) struct AmqpManagementClient {
    management_client: Arc<AmqpManagement>,
    recoverable_connection: Arc<RecoverableConnection>,
}

#[async_trait]
impl AmqpManagementApis for AmqpManagementClient {
    async fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
    ) -> azure_core::Result<AmqpOrderedMap<String, AmqpValue>> {
        let result = retry_azure_operation(
            || {
                let management_client = self.management_client.clone();
                let operation_type = operation_type.clone();
                let application_properties = application_properties.clone();

                async move {
                    let result = management_client
                        .call(operation_type, application_properties)
                        .await?;
                    Ok(result)
                }
            },
            &self.recoverable_connection.retry_options,
            Some(RecoverableConnection::should_retry_management_response),
        )
        .await?;
        Ok(result)
    }

    async fn attach(&self) -> azure_core::Result<()> {
        unimplemented!("AmqpManagementClient does not support attach operation");
    }

    async fn detach(self) -> azure_core::Result<()> {
        unimplemented!("AmqpManagementClient does not support detach operation");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{consumer, ErrorKind, EventHubsError};
    use async_trait::async_trait;
    use azure_core::{http::Url, Result};
    use azure_core_amqp::AmqpError;
    use std::sync::{Arc, Mutex as SyncMutex};
    use time::{Duration, OffsetDateTime};

    // Helper struct to mock token credential
    #[derive(Debug)]
    struct MockTokenCredential {
        /// Duration in seconds until the token expires
        token_duration: i64,

        /// The token itself
        /// This is a mock token, so we don't need to worry about the actual value
        token: SyncMutex<AccessToken>,
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
            })
        }
    }

    #[async_trait]
    impl TokenCredential for MockTokenCredential {
        async fn get_token(&self, _scopes: &[&str]) -> Result<AccessToken> {
            // Simulate a token refresh by incrementing the token get count
            // and updating the token expiration time

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

    // The RecoverableConnection implementation uses a UUID to identify connections unless an application ID is provided.
    // This test verifies that a new recoverable connection uses a UUID for its connection ID when no application ID is specified.
    // It also verifies that the connections aren't initialized during construction - they're created on-demand.
    #[tokio::test]
    async fn recoverable_connection() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection_manager = RecoverableConnection::new(
            url,
            None,
            None,
            MockTokenCredential::new(15),
            Default::default(),
        );
        assert!(!connection_manager.connections.is_initialized());
        assert_eq!(connection_manager.get_connection_id().len(), 36); // UUID v4 string length

        // verify that the connection_id can be parsed as a UUID.
        Uuid::parse_str(connection_manager.get_connection_id()).unwrap();
    }

    // When we construct a RecoverableConnection with an application ID, the connection should use that ID
    // instead of generating a UUID. This test verifies that behavior.
    // Note: Using the actual application ID for the connection name helps with telemetry and debugging
    // in production scenarios.
    #[test]
    fn recoverable_connection_with_application_id() {
        let url = Url::parse("amqps://example.com").unwrap();
        let app_id = "test-app-id".to_string();
        let connection_manager = RecoverableConnection::new(
            url,
            Some(app_id.clone()),
            None,
            MockTokenCredential::new(15),
            Default::default(),
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
        let connection_manager = Arc::new(RecoverableConnection::new(
            url.clone(),
            None,
            None,
            MockTokenCredential::new(15),
            Default::default(),
        ));

        assert!(!connection_manager.connections.is_initialized());
    }

    // The RecoverableConnection supports using a custom endpoint for connecting to Event Hubs proxies.
    // This test verifies that the custom endpoint is properly stored in the RecoverableConnection.
    #[test]
    fn constructor_with_custom_endpoint() {
        let url = Url::parse("amqps://example.com").unwrap();
        let custom_endpoint = Url::parse("https://custom-endpoint.com").unwrap();
        let connection_manager = RecoverableConnection::new(
            url,
            None,
            Some(custom_endpoint.clone()),
            MockTokenCredential::new(15),
            Default::default(),
        );

        assert_eq!(connection_manager.custom_endpoint, Some(custom_endpoint));
    }

    #[test]
    fn should_retry_management_response() {
        consumer::tests::setup();

        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::TooManyRequests,
                Some("Too many requests!".into()),
            )
            .into();

            assert!(RecoverableConnection::should_retry_management_response(
                &error
            ));
        }
        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::SwitchingProtocols,
                Some("Switcheroo".into()),
            )
            .into();
            assert!(!RecoverableConnection::should_retry_management_response(
                &error
            ));
        }
        // Verify that an explicitly boxed error is handled correctly
        {
            let error = azure_core::Error::new(
                AzureErrorKind::Amqp,
                Box::new(AmqpError::new_management_error(
                    azure_core::http::StatusCode::TooManyRequests,
                    Some("Too many requests!".into()),
                )),
            );
            assert!(RecoverableConnection::should_retry_management_response(
                &error
            ));
        }

        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::BadGateway,
                Some("Bad Gateway".into()),
            )
            .into();
            assert!(RecoverableConnection::should_retry_management_response(
                &error
            ));
        }
        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::RequestTimeout,
                Some("Request Timeout".into()),
            )
            .into();
            assert!(RecoverableConnection::should_retry_management_response(
                &error
            ));
        }
        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::InternalServerError,
                Some("Internal Server Error".into()),
            )
            .into();
            assert!(RecoverableConnection::should_retry_management_response(
                &error
            ));
            {
                let error: azure_core::Error =
                    EventHubsError::from(ErrorKind::InvalidManagementResponse).into();
                assert!(!RecoverableConnection::should_retry_management_response(
                    &error
                ));
            }

            {
                let error: azure_core::Error = AmqpError::new_described_error(
                    AmqpErrorCondition::ResourceLimitExceeded,
                    Some("Resource Limit Exceeded".into()),
                    Default::default(),
                )
                .into();

                assert!(RecoverableConnection::should_retry_management_response(
                    &error
                ));
            }
            {
                let error: azure_core::Error = AmqpError::new_described_error(
                    AmqpErrorCondition::IllegalState,
                    Some("Illegal State".into()),
                    Default::default(),
                )
                .into();

                assert!(!RecoverableConnection::should_retry_management_response(
                    &error
                ));
            }
        }
    }
}
