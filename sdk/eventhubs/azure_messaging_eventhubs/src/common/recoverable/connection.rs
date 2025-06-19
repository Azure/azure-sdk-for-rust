// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{
    claims_based_security::RecoverableClaimsBasedSecurity, management::RecoverableManagementClient,
    receiver::RecoverableReceiver, sender::RecoverableSender,
};
use crate::{
    common::{
        authorizer::Authorizer,
        user_agent::{get_package_name, get_package_version, get_platform_info, get_user_agent},
    },
    models::AmqpValue,
    producer::DEFAULT_EVENTHUBS_APPLICATION,
    ErrorKind, EventHubsError, RetryOptions,
};
use async_lock::{Mutex as AsyncMutex, OnceCell};
use azure_core::{credentials::TokenCredential, http::Url, time::Duration, Result, Uuid};
use azure_core_amqp::{
    error::{AmqpErrorCondition, AmqpErrorKind},
    AmqpClaimsBasedSecurity, AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions, AmqpError,
    AmqpManagement, AmqpReceiver, AmqpReceiverApis, AmqpReceiverOptions, AmqpSender,
    AmqpSenderApis, AmqpSession, AmqpSessionApis, AmqpSessionOptions, AmqpSource, AmqpSymbol,
};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, span, trace, warn};

/// The recoverable connection is responsible for managing the connection to the Event Hubs service.
/// It also handles authorization and connection recovery.
///
/// * Notes
///
/// The way a client uses a `RecoverableConnection` is as follows:
///   1. Create a new instance of the `RecoverableConnection`.
///   2. Retrieve an interim object from the `RecoverableConnection`. Supported
///      interim objects are:
///      - `AmqpManagement`: Used for management operations.
///      - `AmqpSender`: Used for sending messages to the Event Hubs service.
///      - `AmqpReceiver`: Used for receiving messages from the Event Hubs service.
///      - `AmqpClaimsBasedSecurity`: Used for authorization operations (should not be used directly)
///   3. Use the interim object to perform operations on the Event Hubs service.
///
/// Under the covers, the interim objects contain a reference back to the [`RecoverableConnection`],
/// and enough information to recreate the underlying AMQP connection, session, management, cbs, or sender/receiver
/// objects as needed.
///
/// The various interim objects implement the appropriate AMQP APIs, but wrap the underlying APIs with
/// a retry loop `Recoverable<Type>::should_retry_<type>_error()`], so that the actual client does not have to worry about retrying or recovering operations.
///
/// There is a taxonomy of methods in this struct:
///   - `ensure_*` methods: These methods are used to ensure that the underlying connection, session, management client, cbs client, sender, or receiver is created and available.
///   - `get_*` methods: These methods are used to retrieve a wrapper around the underlying session, management client, cbs client, sender, or receiver.
///   - `create_*` methods: These methods are used to create a new underlying connection, session, management client, cbs client, sender, or receiver.
///
/// In general, the `ensure_*` and `create_*` methods are private to the `RecoverableConnection`
/// struct, while the `get_*` methods are public(crate) to allow clients to retrieve the underlying objects.
///
pub(crate) struct RecoverableConnection {
    pub(super) url: Url,
    application_id: Option<String>,
    custom_endpoint: Option<Url>,
    connections: OnceCell<Arc<AmqpConnection>>,
    mgmt_client: OnceCell<Arc<AmqpManagement>>,
    session_instances: AsyncMutex<HashMap<Url, Arc<AmqpSession>>>,
    receiver_instances: AsyncMutex<HashMap<Url, Arc<AmqpReceiver>>>,
    sender_instances: AsyncMutex<HashMap<Url, Arc<AmqpSender>>>,
    pub(super) authorizer: Arc<Authorizer>,
    connection_name: String,
    pub(super) retry_options: RetryOptions,
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
                session_instances: AsyncMutex::new(HashMap::new()),
                sender_instances: AsyncMutex::new(HashMap::new()),
                receiver_instances: AsyncMutex::new(HashMap::new()),
                mgmt_client: OnceCell::new(),
                authorizer,
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

    /// Returns the name of the connection as specified by the client.
    pub(crate) fn get_connection_id(&self) -> &str {
        &self.connection_name
    }

    /// Closes the connection to the Event Hubs service.
    ///
    /// # Notes
    /// This method will close the underlying AMQP connection, if it exists. It will also cause all outstanding sends and receives
    /// to complete with an error.
    ///
    pub(crate) async fn close_connection(&self) -> Result<()> {
        let connection = self.ensure_connection().await?;

        connection.close().await
    }

    /// Ensures that the connection to the Event Hubs service is established.
    ///
    /// This method will create a new connection if one does not already exist.
    ///
    /// # Note
    ///
    /// This method is public(crate) to allow event producers and event consumers to
    /// verify that the underlying connection is established before finishing the
    /// construction of the underlying client - this avoids the "magic function" problem
    /// where the client is constructed, but the connection is not established until the
    /// first operation is performed.
    ///
    pub(crate) async fn ensure_connection(&self) -> Result<Arc<AmqpConnection>> {
        let connection = self
            .connections
            .get_or_try_init(|| self.create_connection())
            .await?;
        Ok(connection.clone())
    }

    /// Creates a new management client for the Event Hubs service.
    ///
    /// This client is used to perform management operations such as querying the status of the Event Hubs service.
    pub(crate) async fn get_management_client(
        self: &Arc<Self>,
    ) -> Result<RecoverableManagementClient> {
        Ok(RecoverableManagementClient::new(self.clone()))
    }

    /// Creates a new Claims-Based Security (CBS) client for the Event Hubs service.
    ///
    /// This client is used to perform authorization operations such as acquiring tokens for accessing Event Hubs resources.
    ///
    /// Note: The Cbs client returned integrates retry operations into the authorization call.
    pub(crate) async fn get_cbs_client(self: &Arc<Self>) -> Result<RecoverableClaimsBasedSecurity> {
        Ok(RecoverableClaimsBasedSecurity::new(self.clone()))
    }

    /// Creates a new sender for the Event Hubs service.
    ///
    /// # Notes
    ///
    /// This sender integrates retry operations into the send operation.
    pub(crate) async fn get_sender(self: &Arc<Self>, path: Url) -> Result<RecoverableSender> {
        // Ensure we can create a sender for the Event Hub path.
        self.ensure_sender(&path).await?;

        Ok(RecoverableSender::new(self.clone(), path))
    }

    pub(crate) async fn get_receiver(
        self: &Arc<Self>,
        source_url: &Url,
        message_source: AmqpSource,
        receiver_options: AmqpReceiverOptions,
        timeout: Option<Duration>,
    ) -> Result<RecoverableReceiver> {
        self.ensure_receiver(source_url, &message_source, &receiver_options)
            .await?;

        Ok(RecoverableReceiver::new(
            self.clone(),
            receiver_options,
            message_source,
            source_url.clone(),
            timeout,
        ))
    }

    pub(crate) async fn close_receiver(self: &Arc<Self>, source_url: &Url) -> Result<()> {
        let mut receiver_instances = self.receiver_instances.lock().await;
        if let Some(receiver) = receiver_instances.remove(source_url) {
            let r = Arc::try_unwrap(receiver);
            if let Ok(receiver) = r {
                trace!("Detaching receiver: {:?}", source_url);
                receiver.detach().await?;
            } else {
                warn!("Failed to detach receiver: {:?}", source_url);
            }
        }
        Ok(())
    }

    async fn get_session(self: &Arc<Self>, source_url: &Url) -> Result<Arc<AmqpSession>> {
        let mut session_instances = self.session_instances.lock().await;
        if !session_instances.contains_key(source_url) {
            debug!("Creating session for partition: {:?}", source_url);
            let connection = self.ensure_connection().await?;

            let session = AmqpSession::new();
            session
                .begin(
                    connection.as_ref(),
                    Some(AmqpSessionOptions {
                        incoming_window: Some(u32::MAX),
                        outgoing_window: Some(u32::MAX),
                        ..Default::default()
                    }),
                )
                .await?;
            session_instances.insert(source_url.clone(), Arc::new(session));
        }
        let rv = session_instances
            .get(source_url)
            .ok_or_else(|| EventHubsError::from(ErrorKind::MissingSession))?
            .clone();
        debug!("Cloning session for partition {:?}", source_url);
        Ok(rv)
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

    pub(super) async fn ensure_amqp_management(self: &Arc<Self>) -> Result<Arc<AmqpManagement>> {
        let management_client = self
            .mgmt_client
            .get_or_try_init(|| {
                RecoverableManagementClient::create_management_client(
                    self.clone(),
                    &self.retry_options,
                )
            })
            .await?;

        Ok(management_client.clone())
    }

    /// Ensures that the AMQP Claims-Based Security (CBS) client is created and attached.
    pub(super) async fn ensure_amqp_cbs(self: &Arc<Self>) -> Result<Arc<AmqpClaimsBasedSecurity>> {
        let span = span!(
            tracing::Level::DEBUG,
            "ensure_amqp_cbs",
            connection_id = self.get_connection_id()
        );
        let _enter = span.enter();

        let connection = self.ensure_connection().await?;
        let cbs_client = RecoverableClaimsBasedSecurity::create_claims_based_security(
            connection.clone(),
            &self.retry_options,
        )
        .await?;
        Ok(cbs_client)
    }

    pub(super) async fn ensure_receiver(
        self: &Arc<Self>,
        source_url: &Url,
        message_source: &AmqpSource,
        receiver_options: &AmqpReceiverOptions,
    ) -> Result<Arc<AmqpReceiver>> {
        let mut receiver_instances = self.receiver_instances.lock().await;
        if !receiver_instances.contains_key(source_url) {
            self.ensure_connection().await?;
            self.authorizer.authorize_path(self, source_url).await?;

            let session = self.get_session(source_url).await?;

            debug!("Create receiver on partition {source_url}.");
            let receiver = AmqpReceiver::new();
            receiver
                .attach(
                    &session,
                    message_source.clone(),
                    Some(receiver_options.clone()),
                )
                .await?;

            receiver_instances.insert(source_url.clone(), Arc::new(receiver));
        }

        Ok(receiver_instances
            .get(source_url)
            .ok_or_else(|| EventHubsError::from(ErrorKind::MissingMessageReceiver))?
            .clone())
    }

    pub(super) async fn ensure_sender(self: &Arc<Self>, path: &Url) -> Result<Arc<AmqpSender>> {
        let mut sender_instances = self.sender_instances.lock().await;
        if !sender_instances.contains_key(path) {
            // Ensure that we are authorized to access the senders path.
            self.authorizer.authorize_path(self, path).await?;

            // Retrieve a session for the sender from the session cache.
            let session = self.get_session(path).await?;
            let sender = AmqpSender::new();
            sender
                .attach(
                    &session,
                    format!(
                        "{}-rust-sender",
                        self.application_id
                            .as_ref()
                            .unwrap_or(&DEFAULT_EVENTHUBS_APPLICATION.to_string())
                    ),
                    path.to_string(),
                    None,
                )
                .await?;
            sender_instances.insert(path.clone(), Arc::new(sender));
        }

        Ok(sender_instances
            .get(path)
            .ok_or_else(|| EventHubsError::from(ErrorKind::MissingMessageSender))?
            .clone())
    }

    pub(super) fn should_retry_amqp_error(amqp_error: &AmqpError) -> bool {
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
                        | AmqpErrorCondition::ServerBusyError
                )
            }
            _ => {
                debug!(err=?amqp_error, "Other AMQP error: {amqp_error}");
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::Url;
    use azure_core_test::credentials::MockCredential;
    use std::sync::Arc;

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
            Arc::new(MockCredential),
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
            Arc::new(MockCredential),
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
            Arc::new(MockCredential),
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
            Arc::new(MockCredential),
            Default::default(),
        );

        assert_eq!(connection_manager.custom_endpoint, Some(custom_endpoint));
    }
}
