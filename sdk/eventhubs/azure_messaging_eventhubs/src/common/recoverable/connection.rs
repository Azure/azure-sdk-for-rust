// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell:ignore geodr georeplication sastoken

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
use async_lock::{Mutex as AsyncMutex, OnceCell, RwLock};
use azure_core::{credentials::TokenCredential, http::Url, time::Duration, Uuid};
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
use tracing::{debug, info, instrument, trace, warn};

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
    // The sender, session, and receiver caches are keyed by path. Each entry is
    // an independently-initialized `OnceCell`, so concurrent operations on
    // *different* partitions never serialize on a shared lock, and the expensive
    // attach (authorize + session begin + link attach) happens without holding
    // the map-wide lock. See issues #2243 and #4563.
    sender_instances: RwLock<HashMap<Url, Arc<OnceCell<Arc<AmqpSender>>>>>,
    session_instances: RwLock<HashMap<Url, Arc<OnceCell<Arc<AmqpSession>>>>>,
    receiver_instances: RwLock<HashMap<Url, Arc<OnceCell<Arc<AmqpReceiver>>>>>,
    pub(super) authorizer: Arc<Authorizer>,
    connections: AsyncMutex<Option<Arc<AmqpConnection>>>,
    connection_name: String,
    pub(super) retry_options: RetryOptions,

    #[cfg(test)]
    forced_error: Mutex<Option<AmqpError>>,

    // Separate from `forced_error`, which the per-operation wrappers
    // (receive, send, management call) consume. This slot is consumed only
    // by `ensure_receiver`, so a test can fail an attach without changing
    // how the operation wrappers behave.
    #[cfg(test)]
    forced_attach_error: Mutex<Option<AmqpError>>,
}

unsafe impl Send for RecoverableConnection {}
unsafe impl Sync for RecoverableConnection {}

/// Returns the per-path `OnceCell` for `key`, inserting an uninitialized one if
/// absent. The read path is taken first so steady-state lookups share a read
/// lock; only the first insert for a key takes the write lock. The attach then
/// runs inside the returned `OnceCell`, so the map lock is never held across it
/// and different paths set up concurrently. Shared by the sender, session, and
/// receiver caches so all three keep identical concurrency semantics.
async fn or_init_cell<T>(
    map: &RwLock<HashMap<Url, Arc<OnceCell<Arc<T>>>>>,
    key: &Url,
) -> Arc<OnceCell<Arc<T>>> {
    if let Some(cell) = map.read().await.get(key) {
        return cell.clone();
    }
    map.write()
        .await
        .entry(key.clone())
        .or_insert_with(|| Arc::new(OnceCell::new()))
        .clone()
}

/// Describes which per-connection caches an [`ErrorRecoveryAction`] must invalidate.
///
/// Splitting "which caches" from "actually clearing them" lets the cache-clearing happen
/// inside async lock acquisitions while the policy stays a pure value that's easy to
/// unit-test for regressions (e.g. forgetting to drop the management client when the
/// entire connection is being reset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RecoveryPlan {
    drop_connection: bool,
    clear_authorizer: bool,
    clear_sessions: bool,
    clear_senders: bool,
    clear_receivers: bool,
    drop_mgmt_client: bool,
}

impl RecoveryPlan {
    /// Returns the recovery plan for an action, or `None` if the action does not
    /// require any cache invalidation (i.e. `RetryAction` / `ReturnError`, which
    /// should never reach `recover_from_error`).
    fn for_action(action: &ErrorRecoveryAction) -> Option<Self> {
        match action {
            ErrorRecoveryAction::ReconnectConnection => Some(Self {
                drop_connection: true,
                clear_authorizer: true,
                clear_sessions: true,
                clear_senders: true,
                clear_receivers: true,
                drop_mgmt_client: true,
            }),
            ErrorRecoveryAction::ReconnectSession => Some(Self {
                drop_connection: false,
                clear_authorizer: false,
                clear_sessions: true,
                clear_senders: true,
                clear_receivers: true,
                drop_mgmt_client: false,
            }),
            ErrorRecoveryAction::ReconnectLink => Some(Self {
                drop_connection: false,
                clear_authorizer: false,
                clear_sessions: true,
                clear_senders: true,
                clear_receivers: true,
                drop_mgmt_client: true,
            }),
            ErrorRecoveryAction::RetryAction | ErrorRecoveryAction::ReturnError => None,
        }
    }
}

impl RecoverableConnection {
    /// Creates a recoverable connection. `cbs_token_type` is `None` for
    /// JWT/Entra credentials and `Some("servicebus.windows.net:sastoken")` for
    /// SAS (connection-string) credentials.
    pub fn new(
        url: Url,
        application_id: Option<String>,
        custom_endpoint: Option<Url>,
        credential: Arc<dyn TokenCredential>,
        retry_options: RetryOptions,
        cbs_token_type: Option<&'static str>,
    ) -> Arc<Self> {
        let connection_name = application_id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        Arc::new_cyclic(|weak_rc| {
            let authorizer = Arc::new(Authorizer::new(weak_rc.clone(), credential, cbs_token_type));

            Self {
                url,
                application_id,
                connection_name,
                custom_endpoint,
                retry_options,
                connections: AsyncMutex::new(None),
                session_instances: RwLock::new(HashMap::new()),
                sender_instances: RwLock::new(HashMap::new()),
                receiver_instances: RwLock::new(HashMap::new()),
                mgmt_client: AsyncMutex::new(None),
                authorizer,
                #[cfg(test)]
                forced_error: Mutex::new(None),
                #[cfg(test)]
                forced_attach_error: Mutex::new(None),
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

    /// Makes the next `ensure_receiver` call fail with `error`.
    ///
    /// The injected error takes the same return path as a rejected link
    /// attach: out of the `get_or_try_init` closure, through
    /// `ensure_receiver` and `get_receiver`, and into the caller's error
    /// handling. Tests use this to drive the attach-failure branch of
    /// `EventReceiver::stream_events` without a live broker.
    #[cfg(test)]
    pub(crate) fn force_attach_error(&self, error: AmqpError) -> Result<()> {
        use crate::EventHubsError;

        let mut err = self
            .forced_attach_error
            .lock()
            .map_err(|e| EventHubsError::with_message(e.to_string()))?;
        *err = Some(error);
        Ok(())
    }

    #[cfg(test)]
    fn get_forced_attach_error(&self) -> azure_core_amqp::error::Result<()> {
        let v = self
            .forced_attach_error
            .lock()
            .expect("Forced attach error lock is poisoned")
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
    #[instrument(
        level = "debug",
        skip_all,
        fields(
            connection_id = %self.get_connection_id(),
            url = %self.url,
        ),
        err,
    )]
    pub(crate) async fn close_connection(self) -> Result<()> {
        debug!(
            connection_id = %self.get_connection_id(),
            url = %self.url,
            "Closing recoverable connection."
        );

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

        let mut sender_instances = self.sender_instances.write().await;
        for (path, cell) in sender_instances.drain() {
            trace!("Detaching sender for path {}.", path);
            let Some(sender) = Arc::try_unwrap(cell).ok().and_then(OnceCell::into_inner) else {
                trace!(
                    "Failed to detach sender for path {}, references exist.",
                    path
                );
                continue;
            };
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

        let mut receiver_instances = self.receiver_instances.write().await;
        for (source_url, cell) in receiver_instances.drain() {
            trace!("Detaching receiver for source URL {}.", source_url);
            let Some(receiver) = Arc::try_unwrap(cell).ok().and_then(OnceCell::into_inner) else {
                trace!(
                    "Failed to detach receiver for source URL {}, references exist.",
                    source_url
                );
                continue;
            };
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

        let mut session_instances = self.session_instances.write().await;
        for (session_id, cell) in session_instances.drain() {
            trace!("Detaching session for ID {}.", session_id);
            let Some(session) = Arc::try_unwrap(cell).ok().and_then(OnceCell::into_inner) else {
                trace!(
                    "Failed to detach session for ID {}, references exist.",
                    session_id
                );
                continue;
            };
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
        info!(
            connection_id = %self.get_connection_id(),
            url = %self.url,
            "Closed recoverable connection."
        );
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
        // Drop the map's write lock as soon as the cell is removed so the detach
        // (network I/O) doesn't hold it.
        let Some(cell) = self.receiver_instances.write().await.remove(source_url) else {
            // No entry for this path; nothing to detach.
            return Ok(());
        };
        let receiver = match Arc::try_unwrap(cell) {
            Ok(cell) => cell.into_inner(),
            Err(_) => {
                // A concurrent `ensure_receiver` is mid-attach and still holds a
                // clone of the cell. The map entry is already removed and
                // `EventReceiver::closed` stops the stream from reattaching, so
                // the in-flight receiver is dropped once its operation completes;
                // we just can't detach it by value here.
                trace!(
                    source = %source_url,
                    "close_receiver skipped detach; attach in flight"
                );
                return Ok(());
            }
        };
        let Some(receiver) = receiver else {
            // Cell was removed before any attach completed; nothing to detach.
            return Ok(());
        };
        let strong_count = Arc::strong_count(&receiver);
        if let Ok(receiver) = Arc::try_unwrap(receiver) {
            trace!("Detaching receiver: {:?}", source_url);
            receiver.detach().await?;
        } else {
            // In-flight `receive_delivery` holds a clone of the Arc.
            // Map entry is already removed; `EventReceiver::closed`
            // (set before this call by `request_close`) stops the
            // stream from reattaching on its next poll.
            warn!(
                source = %source_url,
                strong_count,
                "close_receiver could not detach by-value"
            );
        }
        Ok(())
    }

    #[instrument(
        level = "debug",
        skip_all,
        fields(
            connection_id = %self.get_connection_id(),
            source_url = %source_url,
        ),
        err(level = "warn"),
    )]
    async fn get_session(
        self: &Arc<Self>,
        source_url: &Url,
    ) -> azure_core_amqp::Result<Arc<AmqpSession>> {
        // Resolve the per-path cell while holding the map lock only briefly, then
        // initialize (which may begin a new AMQP session) without holding it, so
        // that sessions for other partitions can be created concurrently.
        let cell = self.session_cell(source_url).await;
        let session = cell
            .get_or_try_init(|| async {
                debug!(source_url = %source_url, "Creating session for partition.");
                let connection = self.ensure_connection().await?;

                let session = AmqpSession::new();
                session
                    .begin(
                        connection.as_ref(),
                        Some(AmqpSessionOptions::with_unbounded_windows()),
                    )
                    .await?;
                Ok::<_, AmqpError>(Arc::new(session))
            })
            .await?;
        debug!(source_url = %source_url, "Cloning session for partition.");
        Ok(session.clone())
    }

    /// Returns the `OnceCell` that owns the session for `source_url`, inserting an
    /// uninitialized one if absent. See [`or_init_cell`] for the locking strategy.
    async fn session_cell(&self, source_url: &Url) -> Arc<OnceCell<Arc<AmqpSession>>> {
        or_init_cell(&self.session_instances, source_url).await
    }

    #[instrument(
        level = "debug",
        skip_all,
        fields(
            connection_id = %self.get_connection_id(),
            url = %self.url,
        ),
        err(level = "warn"),
    )]
    async fn create_connection(&self) -> azure_core_amqp::Result<Arc<AmqpConnection>> {
        debug!(
            connection_id = %self.connection_name,
            url = %self.url,
            "Opening AMQP connection."
        );
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
        info!(
            connection_id = %self.connection_name,
            url = %self.url,
            "Opened AMQP connection."
        );
        Ok(connection)
    }

    #[instrument(
        level = "debug",
        skip_all,
        fields(connection_id = %self.get_connection_id()),
        err(level = "warn"),
    )]
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
    #[instrument(
        level = "debug",
        skip_all,
        fields(connection_id = %self.get_connection_id()),
        err(level = "warn"),
    )]
    pub(super) async fn ensure_amqp_cbs(
        self: &Arc<Self>,
    ) -> azure_core_amqp::Result<Arc<AmqpClaimsBasedSecurity>> {
        let connection = self.ensure_connection().await?;
        let cbs_client = RecoverableClaimsBasedSecurity::create_claims_based_security(
            connection.clone(),
            &self.retry_options,
        )
        .await?;
        Ok(cbs_client)
    }

    #[instrument(
        level = "debug",
        skip_all,
        fields(
            connection_id = %self.get_connection_id(),
            source_url = %source_url,
        ),
    )]
    pub(super) async fn ensure_receiver(
        self: &Arc<Self>,
        source_url: &Url,
        message_source: &AmqpSource,
        receiver_options: &AmqpReceiverOptions,
    ) -> azure_core_amqp::Result<Arc<AmqpReceiver>> {
        // Resolve the per-path cell while holding the map lock only briefly, then
        // attach (ensure connection + authorize + session begin + link attach)
        // without holding it, so receivers for other partitions can be created
        // concurrently and steady-state receives never serialize on a shared
        // lock. See issues #2243 and #4563.
        let cell = self.receiver_cell(source_url).await;
        let receiver = cell
            .get_or_try_init(|| async {
                // Test seam: fail the attach with an injected error before
                // any network activity. The error leaves this closure on the
                // same path a rejected `receiver.attach` below takes.
                #[cfg(test)]
                self.get_forced_attach_error()?;

                self.ensure_connection().await?;
                self.authorizer.authorize_path(self, source_url).await?;

                let session = self.get_session(source_url).await?;

                debug!(source_url = %source_url, "Creating receiver on partition.");
                let receiver = AmqpReceiver::new();
                if let Err(e) = receiver
                    .attach(
                        &session,
                        message_source.clone(),
                        Some(receiver_options.clone()),
                    )
                    .await
                {
                    warn!(
                        connection_id = %self.get_connection_id(),
                        source_url = %source_url,
                        err = %e,
                        "Failed to attach receiver on partition."
                    );
                    return Err(e);
                }
                info!(
                    connection_id = %self.get_connection_id(),
                    source_url = %source_url,
                    "Attached receiver on partition."
                );
                Ok::<_, AmqpError>(Arc::new(receiver))
            })
            .await?;

        Ok(receiver.clone())
    }

    /// Returns the `OnceCell` that owns the receiver for `source_url`, inserting
    /// an uninitialized one if absent. See [`or_init_cell`] for the locking strategy.
    async fn receiver_cell(&self, source_url: &Url) -> Arc<OnceCell<Arc<AmqpReceiver>>> {
        or_init_cell(&self.receiver_instances, source_url).await
    }

    #[instrument(
        level = "debug",
        skip_all,
        fields(
            connection_id = %self.get_connection_id(),
            path = %path,
        ),
    )]
    pub(super) async fn ensure_sender(
        self: &Arc<Self>,
        path: &Url,
    ) -> azure_core_amqp::Result<Arc<AmqpSender>> {
        // Resolve the per-path cell while holding the map lock only briefly, then
        // attach (authorize + session begin + link attach) without holding it, so
        // that senders for other partitions can be created concurrently and
        // steady-state sends never serialize on a shared lock. See issue #2243.
        let cell = self.sender_cell(path).await;
        let sender = cell
            .get_or_try_init(|| async {
                // Ensure that we are authorized to access the senders path.
                self.authorizer.authorize_path(self, path).await?;

                // Retrieve a session for the sender from the session cache.
                let session = self.get_session(path).await?;
                debug!(path = %path, "Creating sender on path.");
                let sender = AmqpSender::new();
                if let Err(e) = sender
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
                    .await
                {
                    warn!(
                        connection_id = %self.get_connection_id(),
                        path = %path,
                        err = %e,
                        "Failed to attach sender on path."
                    );
                    return Err(e);
                }
                info!(
                    connection_id = %self.get_connection_id(),
                    path = %path,
                    "Attached sender on path."
                );
                Ok::<_, AmqpError>(Arc::new(sender))
            })
            .await?;

        Ok(sender.clone())
    }

    /// Returns the `OnceCell` that owns the sender for `path`, inserting an
    /// uninitialized one if absent. See [`or_init_cell`] for the locking strategy.
    async fn sender_cell(&self, path: &Url) -> Arc<OnceCell<Arc<AmqpSender>>> {
        or_init_cell(&self.sender_instances, path).await
    }

    #[instrument(
        level = "debug",
        skip_all,
        fields(reason = ?reason),
        err,
    )]
    pub(super) async fn recover_from_error(
        connection: Weak<RecoverableConnection>,
        reason: ErrorRecoveryAction,
    ) -> azure_core_amqp::error::Result<()> {
        let Some(connection) = connection.upgrade() else {
            warn!(
                reason = ?reason,
                "Connection is None, cannot recover from error."
            );
            return Err(AmqpError::with_message("Missing Connection"));
        };

        let connection_id = connection.get_connection_id();

        // Log the error and attempt to recover.
        warn!(
            connection_id = %connection_id,
            reason = ?reason,
            "Recovering from error."
        );

        let Some(plan) = RecoveryPlan::for_action(&reason) else {
            warn!(
                connection_id = %connection_id,
                reason = ?reason,
                "Recover action should already have been handled."
            );
            return Err(AmqpError::with_message(format!(
                "Unknown error recovery action: {reason:?}"
            )));
        };

        debug!(
            connection_id = %connection_id,
            reason = ?reason,
            "Applying recovery plan {plan:?}."
        );
        connection.apply_recovery_plan(plan).await;
        info!(
            connection_id = %connection_id,
            reason = ?reason,
            "Recovery complete."
        );
        Ok(())
    }

    /// Side-effecting half of `recover_from_error`: takes the locks and clears
    /// whichever caches the [`RecoveryPlan`] flagged.
    async fn apply_recovery_plan(&self, plan: RecoveryPlan) {
        let connection_id = self.get_connection_id();
        if plan.drop_connection {
            self.connections.lock().await.take();
            debug!(connection_id = %connection_id, "Recovery: dropped AMQP connection.");
        }
        if plan.clear_authorizer {
            self.authorizer.clear().await;
            debug!(connection_id = %connection_id, "Recovery: cleared authorizer tokens.");
        }
        // Clearing a cache drops its per-path `OnceCell` entries, so the next
        // `ensure_*` for a path re-inits a fresh cell against the new connection.
        // A task already mid-`get_or_try_init` (or holding a value it just read)
        // keeps using the old cell until it next re-enters `ensure_*`, so there's
        // a brief window where stale, pre-recovery links can still be handed out.
        // Closing that window (invalidating in-flight work immediately via a
        // recovery generation counter) is tracked in #4454.
        if plan.clear_sessions {
            let mut sessions = self.session_instances.write().await;
            let count = sessions.len();
            sessions.clear();
            debug!(connection_id = %connection_id, count, "Recovery: cleared cached sessions.");
        }
        if plan.clear_senders {
            let mut senders = self.sender_instances.write().await;
            let count = senders.len();
            senders.clear();
            debug!(connection_id = %connection_id, count, "Recovery: cleared cached senders.");
        }
        if plan.clear_receivers {
            let mut receivers = self.receiver_instances.write().await;
            let count = receivers.len();
            receivers.clear();
            debug!(connection_id = %connection_id, count, "Recovery: cleared cached receivers.");
        }
        if plan.drop_mgmt_client {
            self.mgmt_client.lock().await.take();
            debug!(connection_id = %connection_id, "Recovery: dropped management client.");
        }
    }

    /// Classifies an [`AmqpError`] into the recovery action the retry loop should take.
    ///
    /// Connection-level transport failures (dropped, framing, idle timeout) require a
    /// full reconnect. Link/session-level failures only require reattach. Described
    /// errors are bucketed by their `AmqpErrorCondition`. `TransportImplementationError`
    /// is intentionally left to fall through to `ReturnError`: it covers errors local
    /// to the AMQP backend with no defined recovery semantics, and blind retries risk
    /// hammering a deterministic bug. Anything else not recognized likewise falls
    /// through to `ReturnError`.
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
            | AmqpErrorKind::ConnectionDetachedByRemote(_)
            | AmqpErrorKind::ConnectionDropped(_)
            | AmqpErrorKind::FramingError(_)
            | AmqpErrorKind::IdleTimeoutElapsed(_) => {
                debug!(err = %amqp_error, "Connection dropped error, will reconnect connection.");
                ErrorRecoveryAction::ReconnectConnection
            }
            AmqpErrorKind::SessionClosedByRemote(_) | AmqpErrorKind::SessionDetachedByRemote(_) => {
                debug!(
                    "Session dropped error, will reconnect session: {}",
                    amqp_error
                );
                ErrorRecoveryAction::ReconnectSession
            }
            AmqpErrorKind::LinkClosedByRemote(_)
            | AmqpErrorKind::LinkDetachedByRemote(_)
            | AmqpErrorKind::LinkStateError(_)
            | AmqpErrorKind::DetachError(_)
            | AmqpErrorKind::TransferLimitExceeded(_) => {
                // TransferLimitExceeded means more transfers were sent than the
                // link's credit allowed. Reattaching resets link credit; a full
                // session/connection reconnect is unnecessary.
                debug!(err = %amqp_error, "Link state error, will reconnect link.");
                ErrorRecoveryAction::ReconnectLink
            }
            AmqpErrorKind::SendRejected => ErrorRecoveryAction::ReturnError,
            AmqpErrorKind::AmqpDescribedError(described_error) => {
                debug!(
                    condition = ?described_error.condition,
                    "AMQP described error."
                );
                if matches!(
                    described_error.condition,
                    AmqpErrorCondition::ResourceLimitExceeded
                        | AmqpErrorCondition::ServerBusyError
                        | AmqpErrorCondition::EntityUpdated
                        | AmqpErrorCondition::EntityDisabledError
                        | AmqpErrorCondition::TimeoutError
                        | AmqpErrorCondition::InternalError
                        | AmqpErrorCondition::OperationCancelled
                ) {
                    debug!(
                        condition = ?described_error.condition,
                        "AMQP described error can be retried."
                    );
                    ErrorRecoveryAction::RetryAction
                } else if matches!(
                    described_error.condition,
                    AmqpErrorCondition::ConnectionForced
                        | AmqpErrorCondition::ConnectionFramingError
                ) {
                    debug!(
                        condition = ?described_error.condition,
                        "AMQP described error requires reconnect."
                    );
                    ErrorRecoveryAction::ReconnectConnection
                } else if matches!(
                    described_error.condition,
                    AmqpErrorCondition::UnauthorizedAccess
                ) {
                    // Fail fast on auth failures, matching the .NET and Java Event Hubs
                    // SDKs, which both classify `amqp:unauthorized-access` as non-transient
                    // / non-retryable. A runtime unauthorized-access almost always means a
                    // bad or revoked credential or missing RBAC, not a momentarily expired
                    // token; keeping tokens fresh is the CBS refresher's job (proactive
                    // pre-expiry renewal), not something to recover by reconnecting on a 401.
                    // Routing this to ReconnectConnection would turn a fast failure into N
                    // full reconnect + re-auth cycles before the error finally surfaces.
                    // Authorization failures are terminal fast-fails the caller cannot
                    // recover from; surface them at warn! with the condition.
                    warn!(
                        condition = ?described_error.condition,
                        "AMQP unauthorized-access error, will not retry."
                    );
                    ErrorRecoveryAction::ReturnError
                } else if matches!(
                    described_error.condition,
                    AmqpErrorCondition::LinkStolen | AmqpErrorCondition::LinkDetachForced
                ) {
                    // The link is gone; retrying the same operation against it will keep
                    // failing. Reattach. (LinkStolen was previously classified as a retry,
                    // which guaranteed N spins through the backoff before bailing.)
                    debug!(
                        condition = ?described_error.condition,
                        "AMQP described error requires link reattach."
                    );
                    ErrorRecoveryAction::ReconnectLink
                } else {
                    debug!(
                        condition = ?described_error.condition,
                        "AMQP described error cannot be retried."
                    );
                    ErrorRecoveryAction::ReturnError
                }
            }
            AmqpErrorKind::AzureCore(_) => {
                // The ensure_* callsites in the per-operation wrappers (sender, CBS,
                // management) re-wrap inner AmqpError values through
                // `azure_core::Error::with_error`, producing
                // `AzureCore(azure_core::Error { source: original AmqpError })`.
                // If we don't walk the source chain we'd classify those as
                // ReturnError and lose the ability to recover transport-level failures
                // that round-trip through this wrapping pattern.
                Self::classify_azure_core_chain(amqp_error)
            }
            _ => {
                debug!(err=?amqp_error, "Other AMQP error: {amqp_error}");
                ErrorRecoveryAction::ReturnError
            }
        }
    }

    /// Like `should_retry_amqp_error` but returns `ReturnError` on
    /// `LinkStolen` so a displaced receiver surfaces the steal instead of
    /// silently re-attaching. .NET parallel: `InvalidateConsumerWhenPartitionIsStolen`.
    pub(super) fn should_retry_receive_error(amqp_error: &AmqpError) -> ErrorRecoveryAction {
        // A `LinkStolen` means the partition was claimed by another consumer; reattaching
        // would silently resurrect a displaced receiver, so surface it instead. The
        // condition can arrive at the top level or wrapped through `azure_core::Error` (the
        // same wrapping `classify_azure_core_chain` unwraps), and the chain walk would
        // otherwise reclassify a wrapped `LinkStolen` to `ReconnectLink`, defeating this
        // guard. So check the whole source chain, not just the top-level kind.
        if Self::is_link_stolen(amqp_error) {
            debug!("Receive operation will not retry link-stolen: {amqp_error}");
            return ErrorRecoveryAction::ReturnError;
        }
        Self::should_retry_amqp_error(amqp_error)
    }

    /// Returns true if `amqp_error` is, or wraps via its [`std::error::Error::source`]
    /// chain, an [`AmqpErrorKind::AmqpDescribedError`] whose condition is `LinkStolen`.
    /// The stream translation uses the same walk, so both agree on what counts
    /// as a stolen link.
    fn is_link_stolen(amqp_error: &AmqpError) -> bool {
        crate::error::find_link_stolen(amqp_error).is_some()
    }

    /// Walks the [`std::error::Error::source`] chain looking for a wrapped [`AmqpError`]
    /// whose kind is something other than [`AmqpErrorKind::AzureCore`], and classifies
    /// that. Falls back to `ReturnError` if no recoverable inner kind is found.
    ///
    /// A bounded loop guards against pathological self-referential chains.
    fn classify_azure_core_chain(amqp_error: &AmqpError) -> ErrorRecoveryAction {
        use std::error::Error as _;
        const MAX_DEPTH: usize = 16;
        let mut cause: Option<&(dyn std::error::Error + 'static)> = amqp_error.source();
        for _ in 0..MAX_DEPTH {
            let Some(c) = cause else { break };
            if let Some(amqp) = c.downcast_ref::<AmqpError>() {
                if !matches!(amqp.kind(), AmqpErrorKind::AzureCore(_)) {
                    debug!(
                        err=?amqp_error,
                        "Unwrapped AzureCore chain to inner AmqpError: {amqp}"
                    );
                    return Self::should_retry_amqp_error(amqp);
                }
            }
            cause = c.source();
        }
        debug!(
            err=?amqp_error,
            "AzureCore-wrapped error with no recoverable inner kind: {amqp_error}"
        );
        ErrorRecoveryAction::ReturnError
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
            None,
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
            None,
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
            None,
        ));

        assert!(!connection_manager.connections.lock_blocking().is_some());
    }

    // The per-path sender/session/receiver caches must hand out one shared
    // `OnceCell` per path (so concurrent first-operations on a partition attach
    // exactly once) and distinct cells for distinct paths (so operations on
    // different partitions never share a cell and can initialize concurrently).
    // See issues #2243 and #4563.
    #[tokio::test]
    async fn sender_session_and_receiver_cells_are_keyed_by_path() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            Arc::new(MockCredential),
            Default::default(),
            None,
        );

        let path_a = Url::parse("amqps://example.com/eh/Partitions/0").unwrap();
        let path_b = Url::parse("amqps://example.com/eh/Partitions/1").unwrap();

        // Same path resolves to the same cell for senders, sessions, and receivers.
        assert!(Arc::ptr_eq(
            &connection.sender_cell(&path_a).await,
            &connection.sender_cell(&path_a).await
        ));
        assert!(Arc::ptr_eq(
            &connection.session_cell(&path_a).await,
            &connection.session_cell(&path_a).await
        ));
        assert!(Arc::ptr_eq(
            &connection.receiver_cell(&path_a).await,
            &connection.receiver_cell(&path_a).await
        ));

        // Different paths resolve to different cells.
        assert!(!Arc::ptr_eq(
            &connection.sender_cell(&path_a).await,
            &connection.sender_cell(&path_b).await
        ));
        assert!(!Arc::ptr_eq(
            &connection.receiver_cell(&path_a).await,
            &connection.receiver_cell(&path_b).await
        ));

        // Cells are uninitialized until an attach succeeds.
        assert!(connection.sender_cell(&path_a).await.get().is_none());
        assert!(connection.session_cell(&path_a).await.get().is_none());
        assert!(connection.receiver_cell(&path_a).await.get().is_none());
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
            None,
        );

        assert_eq!(connection_manager.custom_endpoint, Some(custom_endpoint));
    }

    #[test]
    fn test_should_retry_amqp_error() {
        use azure_core_amqp::AmqpDescribedError;

        // Test ConnectionDropped -> ReconnectConnection
        let err = AmqpError::from(AmqpErrorKind::ConnectionDropped(Box::new(
            std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "dropped"),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReconnectConnection
        );

        // Test TimeoutError -> RetryAction
        let err = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::TimeoutError,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::RetryAction
        );

        // Test ConnectionForced -> ReconnectConnection
        let err = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::ConnectionForced,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReconnectConnection
        );

        // Test UnauthorizedAccess -> ReturnError. Auth failures are non-transient
        // (matches the .NET / Java SDKs); reconnecting on a 401 would only burn the
        // retry budget against a bad credential before the error surfaces.
        let err = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::UnauthorizedAccess,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReturnError
        );

        // Test EntityDisabledError -> RetryAction (matched by the first arm of the
        // described-error branch; a removed-but-unreachable elif previously also
        // listed it).
        let err = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::EntityDisabledError,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::RetryAction
        );

        // Test IdleTimeoutElapsed -> ReconnectConnection. Idle-timeout means the peer
        // hasn't sent a frame inside the negotiated heartbeat window, so the transport
        // is effectively dead.
        let err = AmqpError::from(AmqpErrorKind::IdleTimeoutElapsed(Box::new(
            std::io::Error::new(std::io::ErrorKind::TimedOut, "idle timeout"),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReconnectConnection
        );

        // Test FramingError -> ReconnectConnection. The wire protocol is corrupted;
        // there is no recovery short of a fresh connection.
        let err = AmqpError::from(AmqpErrorKind::FramingError(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "framing error",
        ))));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReconnectConnection
        );

        // Test DetachError -> ReconnectLink. The link's detach handshake failed;
        // reattach is required to make any further use of it.
        let err = AmqpError::from(AmqpErrorKind::DetachError(Box::new(std::io::Error::other(
            "detach error",
        ))));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReconnectLink
        );

        // Test LinkStolen -> ReconnectLink. Behavior change: previously classified as
        // RetryAction, which burned the entire backoff against a link that is gone.
        let err = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::LinkStolen,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReconnectLink
        );

        // Test LinkDetachForced -> ReconnectLink. The peer force-detached the link;
        // reattach is required.
        let err = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::LinkDetachForced,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReconnectLink
        );
    }

    #[test]
    fn receive_error_link_stolen_returns_error_top_level_and_wrapped() {
        use azure_core::error::ErrorKind as AzureErrorKind;
        use azure_core_amqp::AmqpDescribedError;

        // Top-level LinkStolen must surface as ReturnError so the stolen partition is
        // reported, not silently reattached (.NET: InvalidateConsumerWhenPartitionIsStolen).
        let top = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::LinkStolen,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_receive_error(&top),
            ErrorRecoveryAction::ReturnError
        );

        // LinkStolen wrapped through azure_core::Error (as the ensure_* wrappers produce)
        // would slip past a top-level-only guard and be reclassified to ReconnectLink by
        // the source-chain walk, resurrecting a stolen receiver. It must still ReturnError.
        let inner = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::LinkStolen,
            None,
            Default::default(),
        )));
        let wrapped = AmqpError::from(azure_core::Error::with_error(
            AzureErrorKind::Other,
            inner,
            "ensure_receiver failed",
        ));
        assert_eq!(
            RecoverableConnection::should_retry_receive_error(&wrapped),
            ErrorRecoveryAction::ReturnError
        );
    }

    #[test]
    fn azure_core_wrapped_errors_unwrap_to_inner_kind() {
        // The per-operation wrappers in sender.rs / claims_based_security.rs /
        // management.rs all wrap an inner AmqpError via
        // `AmqpError::from(azure_core::Error::with_error(AzureErrorKind::Other, e, "..."))`.
        // Before this test we'd classify the outer error as ReturnError via the
        // catch-all, silently turning a recoverable transport failure into a
        // non-retryable one. should_retry_amqp_error must walk the source chain
        // and honor the inner kind's classification.
        use azure_core::error::ErrorKind as AzureErrorKind;

        // AzureCore(... ConnectionDropped ...) -> ReconnectConnection
        let inner = AmqpError::from(AmqpErrorKind::ConnectionDropped(Box::new(
            std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "dropped"),
        )));
        let wrapped = AmqpError::from(azure_core::Error::with_error(
            AzureErrorKind::Other,
            inner,
            "ensure_sender failed",
        ));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&wrapped),
            ErrorRecoveryAction::ReconnectConnection
        );

        // AzureCore(... LinkClosedByRemote ...) -> ReconnectLink
        let inner = AmqpError::from(AmqpErrorKind::LinkClosedByRemote(Box::new(
            std::io::Error::other("link closed"),
        )));
        let wrapped = AmqpError::from(azure_core::Error::with_error(
            AzureErrorKind::Other,
            inner,
            "ensure_amqp_cbs failed",
        ));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&wrapped),
            ErrorRecoveryAction::ReconnectLink
        );

        // Nested wrapping: AzureCore(... AzureCore(... ConnectionDropped ...) ...).
        // The recovery path can re-wrap (e.g. ensure_connection inside
        // ensure_management_client). The chain walk must keep descending.
        let innermost = AmqpError::from(AmqpErrorKind::ConnectionDropped(Box::new(
            std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "dropped"),
        )));
        let mid = AmqpError::from(azure_core::Error::with_error(
            AzureErrorKind::Other,
            innermost,
            "ensure_connection failed",
        ));
        let outer = AmqpError::from(azure_core::Error::with_error(
            AzureErrorKind::Other,
            mid,
            "create_management_client failed",
        ));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&outer),
            ErrorRecoveryAction::ReconnectConnection
        );

        // AzureCore wrapping something that isn't an AmqpError -> ReturnError.
        // (No recoverable inner kind to honor; preserve the catch-all default.)
        let wrapped = AmqpError::from(azure_core::Error::with_error(
            AzureErrorKind::Other,
            std::io::Error::other("non-AMQP failure"),
            "unrelated error path",
        ));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&wrapped),
            ErrorRecoveryAction::ReturnError
        );
    }

    #[test]
    fn transfer_limit_exceeded_reattaches_link() {
        // amqp:link:transfer-limit-exceeded: peer sent more transfers than the
        // link's credit allowed. Reattaching resets link credit; ReturnError
        // would have surfaced a recoverable condition to the caller.
        let err = AmqpError::from(AmqpErrorKind::TransferLimitExceeded(Box::new(
            std::io::Error::other("transfer limit exceeded"),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReconnectLink
        );
    }

    #[test]
    fn internal_error_and_operation_cancelled_are_retried() {
        use azure_core_amqp::AmqpDescribedError;

        // amqp:internal-error is conventionally transient (consistent with the
        // .NET / Java Service Bus + Event Hubs SDKs). The link and connection
        // are unaffected.
        let err = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::InternalError,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::RetryAction
        );

        // com.microsoft:operation-cancelled: service-side cancel of a single
        // operation. The link is alive, the op can be retried.
        let err = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::OperationCancelled,
            None,
            Default::default(),
        )));
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::RetryAction
        );
    }

    #[test]
    fn recovery_plan_reconnect_connection_clears_everything() {
        let plan = RecoveryPlan::for_action(&ErrorRecoveryAction::ReconnectConnection)
            .expect("ReconnectConnection has a recovery plan");
        // Regression guard: a full reconnect must drop the management client too,
        // otherwise it would be left holding a session attached to the just-dropped
        // connection and the next management call would fail and re-trigger recovery.
        assert!(plan.drop_mgmt_client);
        assert!(plan.drop_connection);
        assert!(plan.clear_authorizer);
        assert!(plan.clear_sessions);
        assert!(plan.clear_senders);
        assert!(plan.clear_receivers);
    }

    #[test]
    fn recovery_plan_reconnect_link_drops_mgmt_client_but_keeps_connection() {
        let plan = RecoveryPlan::for_action(&ErrorRecoveryAction::ReconnectLink)
            .expect("ReconnectLink has a recovery plan");
        assert!(!plan.drop_connection);
        assert!(!plan.clear_authorizer);
        assert!(plan.clear_sessions);
        assert!(plan.clear_senders);
        assert!(plan.clear_receivers);
        assert!(plan.drop_mgmt_client);
    }

    #[test]
    fn recovery_plan_reconnect_session_keeps_mgmt_client_and_connection() {
        let plan = RecoveryPlan::for_action(&ErrorRecoveryAction::ReconnectSession)
            .expect("ReconnectSession has a recovery plan");
        assert!(!plan.drop_connection);
        assert!(!plan.clear_authorizer);
        assert!(plan.clear_sessions);
        assert!(plan.clear_senders);
        assert!(plan.clear_receivers);
        assert!(!plan.drop_mgmt_client);
    }

    #[test]
    fn recovery_plan_none_for_non_reconnect_actions() {
        assert!(RecoveryPlan::for_action(&ErrorRecoveryAction::RetryAction).is_none());
        assert!(RecoveryPlan::for_action(&ErrorRecoveryAction::ReturnError).is_none());
    }
}
