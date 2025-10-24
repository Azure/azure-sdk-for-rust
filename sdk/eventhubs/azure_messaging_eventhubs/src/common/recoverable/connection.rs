// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell:ignore geodr georeplication

use super::{
    claims_based_security::RecoverableClaimsBasedSecurity, management::RecoverableManagementClient,
    receiver::RecoverableReceiver, sender::RecoverableSender,
};
use crate::{
    common::{
        authorizer::Authorizer,
        retry::ErrorRecoveryAction,
        user_agent::{get_package_name, get_package_version, get_platform_info, get_user_agent},
    },
    error::Result,
    models::AmqpValue,
    producer::DEFAULT_EVENTHUBS_APPLICATION,
    RetryOptions,
};
use async_lock::Mutex as AsyncMutex;
use azure_core::{
    credentials::TokenCredential, error::ErrorKind as AzureErrorKind, http::Url, time::Duration,
    Uuid,
};
use azure_core_amqp::{
    error::{AmqpErrorCondition, AmqpErrorKind},
    AmqpClaimsBasedSecurity, AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions, AmqpError,
    AmqpManagement, AmqpManagementApis, AmqpReceiver, AmqpReceiverApis, AmqpReceiverOptions,
    AmqpSender, AmqpSenderApis, AmqpSession, AmqpSessionApis, AmqpSessionOptions, AmqpSource,
    AmqpSymbol,
};
#[cfg(test)]
use std::sync::Mutex;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
use tracing::{debug, span, trace, warn};

/// The AMQP capability string used to negotiate geographic replication features
/// between client and server. This capability is advertised during AMQP connection setup to indicate
/// support for geographic replication, allowing clients and Event Hubs to coordinate failover and replication
/// scenarios for high availability.
const GEODR_REPLICATION_CAPABILITY: &str = "com.microsoft.georeplication";

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
    mgmt_client: AsyncMutex<Option<Arc<AmqpManagement>>>,
    receiver_instances: AsyncMutex<HashMap<Url, Arc<AmqpReceiver>>>,
    sender_instances: AsyncMutex<HashMap<Url, Arc<AmqpSender>>>,
    session_instances: AsyncMutex<HashMap<Url, Arc<AmqpSession>>>,
    pub(super) authorizer: Arc<Authorizer>,
    connections: AsyncMutex<Option<Arc<AmqpConnection>>>,
    connection_name: String,
    pub(super) retry_options: RetryOptions,

    #[cfg(test)]
    forced_error: Mutex<Option<AmqpError>>,
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
                connections: AsyncMutex::new(None),
                session_instances: AsyncMutex::new(HashMap::new()),
                sender_instances: AsyncMutex::new(HashMap::new()),
                receiver_instances: AsyncMutex::new(HashMap::new()),
                mgmt_client: AsyncMutex::new(None),
                authorizer,
                #[cfg(test)]
                forced_error: Mutex::new(None),
            }
        })
    }

    /// Create a connection that is unconnected
    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) async fn disable_connection(&self) -> Result<()> {
        let mut connection = self.connections.lock().await;
        *connection = Some(Arc::new(AmqpConnection::new()));
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn force_error(&self, error: AmqpError) -> Result<()> {
        use crate::EventHubsError;

        let mut err = self
            .forced_error
            .lock()
            .map_err(|e| EventHubsError::with_message(e.to_string()))?;
        *err = Some(error);
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn get_forced_error(&self) -> azure_core_amqp::error::Result<()> {
        let v = self
            .forced_error
            .lock()
            .expect("Forced error lock is poisoned")
            .take();
        v.map_or(Ok(()), Err)
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
    pub(crate) async fn close_connection(self) -> Result<()> {
        trace!("Closing recoverable connection for {}.", self.url);

        let mut management_client = self.mgmt_client.lock().await;
        if let Some(management_client) = management_client.take() {
            trace!("Closing management client for {}.", self.url);
            if let Ok(management_client) = Arc::try_unwrap(management_client) {
                trace!("Detaching management client for {}.", self.url);
                management_client.detach().await?;
            } else {
                trace!(
                    "Failed to detach management client for {}, references exist.",
                    self.url
                );
            }
        }

        let mut sender_instances = self.sender_instances.lock().await;
        for (path, sender) in sender_instances.drain() {
            trace!("Detaching sender for path {}.", path);
            if let Ok(sender) = Arc::try_unwrap(sender) {
                trace!("Detaching sender for path {}.", path);
                sender.detach().await?;
            } else {
                trace!(
                    "Failed to detach sender for path {}, references exist.",
                    path
                );
            }
        }

        let mut receiver_instances = self.receiver_instances.lock().await;
        for (source_url, receiver) in receiver_instances.drain() {
            trace!("Detaching receiver for source URL {}.", source_url);
            if let Ok(receiver) = Arc::try_unwrap(receiver) {
                trace!("Detaching receiver for source URL {}.", source_url);
                receiver.detach().await?;
            } else {
                trace!(
                    "Failed to detach receiver for source URL {}, references exist.",
                    source_url
                );
            }
        }

        let mut session_instances = self.session_instances.lock().await;
        for (session_id, session) in session_instances.drain() {
            trace!("Detaching session for ID {}.", session_id);
            if let Ok(session) = Arc::try_unwrap(session) {
                session.end().await?;
            } else {
                trace!(
                    "Failed to detach session for ID {}, references exist.",
                    session_id
                );
            }
        }

        if let Some(connection) = self.connections.lock().await.take() {
            trace!("Closing connection for {}.", self.url);
            if let Ok(connection) = Arc::try_unwrap(connection) {
                trace!(
                    "No references, actually closing connection for {}.",
                    self.url
                );
                connection.close().await?;
            } else {
                trace!(
                    "Failed to close connection for {}, references exist.",
                    self.url
                );
            }
        }
        Ok(())
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
    pub(crate) async fn ensure_connection(&self) -> azure_core_amqp::Result<Arc<AmqpConnection>> {
        let mut connection = self.connections.lock().await;
        if connection.is_none() {
            *connection = Some(self.create_connection().await?);
        }
        if let Some(connection) = connection.as_ref() {
            return Ok(connection.clone());
        }
        Err(AmqpError::with_message("Missing Connection."))
    }

    /// Creates a new management client for the Event Hubs service.
    ///
    /// This client is used to perform management operations such as querying the status of the Event Hubs service.
    pub(crate) fn get_management_client(self: &Arc<Self>) -> RecoverableManagementClient {
        RecoverableManagementClient::new(Arc::downgrade(self))
    }

    /// Creates a new Claims-Based Security (CBS) client for the Event Hubs service.
    ///
    /// This client is used to perform authorization operations such as acquiring tokens for accessing Event Hubs resources.
    ///
    /// Note: The Cbs client returned integrates retry operations into the authorization call.
    pub(crate) fn get_cbs_client(self: &Arc<Self>) -> RecoverableClaimsBasedSecurity {
        RecoverableClaimsBasedSecurity::new(Arc::downgrade(self))
    }

    /// Creates a new sender for the Event Hubs service.
    ///
    /// # Notes
    ///
    /// This sender integrates retry operations into the send operation.
    pub(crate) async fn get_sender(self: &Arc<Self>, path: Url) -> Result<RecoverableSender> {
        // Ensure we can create a sender for the Event Hub path.
        self.ensure_sender(&path).await?;

        Ok(RecoverableSender::new(Arc::downgrade(self), path))
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
            Arc::downgrade(self),
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

    async fn get_session(
        self: &Arc<Self>,
        source_url: &Url,
    ) -> azure_core_amqp::Result<Arc<AmqpSession>> {
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
            .ok_or_else(|| {
                AmqpError::from(azure_core::Error::with_message(
                    AzureErrorKind::Other,
                    "Could not find session",
                ))
            })?
            .clone();
        debug!("Cloning session for partition {:?}", source_url);
        Ok(rv)
    }

    async fn create_connection(&self) -> azure_core_amqp::Result<Arc<AmqpConnection>> {
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
                    desired_capabilities: Some(vec![GEODR_REPLICATION_CAPABILITY.into()]),
                    custom_endpoint: self.custom_endpoint.clone(),
                    ..Default::default()
                }),
            )
            .await?;
        Ok(connection)
    }

    pub(super) async fn ensure_amqp_management(
        self: &Arc<Self>,
    ) -> azure_core_amqp::Result<Arc<AmqpManagement>> {
        let mut management_client = self.mgmt_client.lock().await;
        if management_client.is_none() {
            *management_client = Some(
                RecoverableManagementClient::create_management_client(
                    self.clone(),
                    &self.retry_options,
                )
                .await?,
            );
        }
        if let Some(management_client) = management_client.as_ref() {
            return Ok(management_client.clone());
        }

        warn!("Management client is None, cannot ensure management client.");
        Err(AmqpError::with_message("Missing Management Client"))
    }

    /// Ensures that the AMQP Claims-Based Security (CBS) client is created and attached.
    pub(super) async fn ensure_amqp_cbs(
        self: &Arc<Self>,
    ) -> azure_core_amqp::Result<Arc<AmqpClaimsBasedSecurity>> {
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
    ) -> azure_core_amqp::Result<Arc<AmqpReceiver>> {
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
            .ok_or_else(|| AmqpError::with_message("Missing message receiver"))?
            .clone())
    }

    pub(super) async fn ensure_sender(
        self: &Arc<Self>,
        path: &Url,
    ) -> azure_core_amqp::Result<Arc<AmqpSender>> {
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
            .ok_or_else(|| {
                AmqpError::from(azure_core::Error::with_message(
                    AzureErrorKind::Other,
                    "Missing message sender",
                ))
            })?
            .clone())
    }

    pub(super) async fn recover_from_error(
        connection: Weak<RecoverableConnection>,
        reason: ErrorRecoveryAction,
    ) -> azure_core_amqp::error::Result<()> {
        // If the connection is None, we cannot recover.
        let Some(connection) = connection.upgrade() else {
            warn!(
                "Connection is None, cannot recover from error: {:?}",
                reason
            );
            return Err(AmqpError::with_message("Missing Connection"));
        };

        // Log the error and attempt to recover.
        warn!(err=?reason, "Recovering from error: {:?}", reason);
        // Upgrade the weak reference to a strong reference.
        match reason {
            ErrorRecoveryAction::ReconnectConnection => {
                debug!("Recovering from connection error: {:?}", reason);
                connection.connections.lock().await.take();
                connection.authorizer.clear().await;
                connection.session_instances.lock().await.clear();
                connection.sender_instances.lock().await.clear();
                connection.receiver_instances.lock().await.clear();
            }
            ErrorRecoveryAction::ReconnectSession => {
                debug!("Recovering from session error: {:?}", reason);
                // Recreate the session and sender/receiver as needed.
                connection.session_instances.lock().await.clear();
                connection.sender_instances.lock().await.clear();
                connection.receiver_instances.lock().await.clear();
            }
            ErrorRecoveryAction::ReconnectLink => {
                debug!("Recovering from link error: {:?}", reason);
                // Recreate the session and sender/receiver as needed.
                connection.session_instances.lock().await.clear();
                connection.sender_instances.lock().await.clear();
                connection.receiver_instances.lock().await.clear();
                connection.mgmt_client.lock().await.take();
            }
            _ => {
                warn!("Recover action {reason:?} should already have been handled.");
                return Err(AmqpError::with_message(format!(
                    "Unknown error recovery action: {reason:?}"
                )));
            }
        }

        Ok(())
    }

    pub(super) fn should_retry_amqp_error(amqp_error: &AmqpError) -> ErrorRecoveryAction {
        match amqp_error.kind() {
            AmqpErrorKind::ManagementStatusCode(code, _) => {
                debug!("Management operation error: {}", code);
                if matches!(
                    code,
                    azure_core::http::StatusCode::RequestTimeout
                        | azure_core::http::StatusCode::TooManyRequests
                        | azure_core::http::StatusCode::InternalServerError
                        | azure_core::http::StatusCode::BadGateway
                        | azure_core::http::StatusCode::ServiceUnavailable
                        | azure_core::http::StatusCode::GatewayTimeout
                ) {
                    debug!("Management operation error can be retried : {}", code);
                    ErrorRecoveryAction::RetryAction
                } else {
                    debug!("Management operation error cannot be retried: {}", code);
                    ErrorRecoveryAction::ReturnError
                }
            }
            AmqpErrorKind::ConnectionClosedByRemote(_)
            | AmqpErrorKind::ConnectionDetachedByRemote(_) => {
                debug!("Connection dropped error: {}", amqp_error);
                ErrorRecoveryAction::ReconnectConnection
            }
            AmqpErrorKind::SessionClosedByRemote(_) | AmqpErrorKind::SessionDetachedByRemote(_) => {
                debug!("Session dropped error: {}", amqp_error);
                ErrorRecoveryAction::ReconnectSession
            }
            AmqpErrorKind::LinkClosedByRemote(_)
            | AmqpErrorKind::LinkDetachedByRemote(_)
            | AmqpErrorKind::LinkStateError(_) => {
                debug!("Link state error: {}", amqp_error);
                ErrorRecoveryAction::ReconnectLink
            }
            AmqpErrorKind::SendRejected => ErrorRecoveryAction::ReturnError,
            AmqpErrorKind::AmqpDescribedError(described_error) => {
                debug!("AMQP described error: {:?}", described_error);
                if matches!(
                    described_error.condition,
                    AmqpErrorCondition::ResourceLimitExceeded
                        | AmqpErrorCondition::ConnectionFramingError
                        | AmqpErrorCondition::LinkStolen
                        | AmqpErrorCondition::ServerBusyError
                        | AmqpErrorCondition::EntityUpdated
                        | AmqpErrorCondition::EntityDisabledError
                ) {
                    debug!("AMQP described error can be retried: {:?}", described_error);
                    ErrorRecoveryAction::RetryAction
                } else if matches!(
                    described_error.condition,
                    AmqpErrorCondition::EntityDisabledError
                ) {
                    debug!(
                        "AMQP described error triggers a disconnect: {:?}",
                        described_error
                    );
                    ErrorRecoveryAction::RetryAction
                } else {
                    debug!(
                        "AMQP described error cannot be retried: {:?}",
                        described_error
                    );
                    ErrorRecoveryAction::ReturnError
                }
            }
            _ => {
                debug!(err=?amqp_error, "Other AMQP error: {amqp_error}");
                ErrorRecoveryAction::ReturnError
            }
        }
    }
}

impl Drop for RecoverableConnection {
    fn drop(&mut self) {
        trace!("Dropping RecoverableConnection for {}", self.url);
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
        assert!(!connection_manager.connections.lock_blocking().is_some());
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
        assert!(!connection_manager.connections.lock_blocking().is_some());
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

        assert!(!connection_manager.connections.lock_blocking().is_some());
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
