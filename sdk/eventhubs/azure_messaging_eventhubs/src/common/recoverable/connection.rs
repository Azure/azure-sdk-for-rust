// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell:ignore geodr georeplication sastoken

use super::{
    claims_based_security::RecoverableClaimsBasedSecurity, management::RecoverableManagementClient,
    receiver::RecoverableReceiver, sender::RecoverableSender, MAX_GENERATION_RETRIES,
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
use async_lock::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard, OnceCell, RwLock};
use azure_core::{credentials::TokenCredential, http::Url, time::Duration, Uuid};
use azure_core_amqp::{
    error::{AmqpErrorCondition, AmqpErrorKind},
    AmqpClaimsBasedSecurity, AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions, AmqpError,
    AmqpManagement, AmqpManagementApis, AmqpReceiver, AmqpReceiverApis, AmqpReceiverOptions,
    AmqpSender, AmqpSenderApis, AmqpSession, AmqpSessionApis, AmqpSessionOptions, AmqpSource,
    AmqpSymbol, AmqpTransport,
};
#[cfg(test)]
use std::sync::Mutex;
use std::{
    collections::HashMap,
    future::Future,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Weak,
    },
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
    transport: AmqpTransport,
    // The management client is a single cached instance, held in a `OnceCell`
    // for the same reason the per-path caches are: the expensive build (connect
    // + session begin + CBS authorize + link attach) must not run while a lock
    // is held. The build authorizes the `$management` path, and a CBS failure
    // there runs the recovery hook, which invalidates this cache. Holding a
    // guard across the build made that a same-task self-deadlock. The `RwLock`
    // only guards the *cell pointer*, so recovery can swap in a fresh cell
    // without waiting for a build in flight.
    mgmt_client: RwLock<Arc<OnceCell<Arc<AmqpManagement>>>>,
    // The sender, session, and receiver caches are keyed by path. Each entry is
    // an independently-initialized `OnceCell`, so concurrent operations on
    // *different* partitions never serialize on a shared lock, and the expensive
    // attach (authorize + session begin + link attach) happens without holding
    // the map-wide lock. See issues #2243 and #4563.
    //
    // Each cell is tagged with the recovery `generation` it was created under (see
    // the `generation` field). A slow-path attach that races a recovery completes
    // against a now-dead connection; comparing the cell's generation against the
    // current one after the attach lets that path discard its stale result instead
    // of caching and handing out a resource bound to the old connection. See #4454.
    sender_instances: RwLock<HashMap<Url, GenerationalCell<AmqpSender>>>,
    session_instances: RwLock<HashMap<Url, GenerationalCell<AmqpSession>>>,
    receiver_instances: RwLock<HashMap<Url, GenerationalCell<AmqpReceiver>>>,
    pub(super) authorizer: Arc<Authorizer>,
    // The service permits one `$cbs` link for each connection. Every
    // authorization attaches a link, uses it, and then drops it, so two
    // authorizations that overlap make the service reject the second one with
    // `NotAllowed`. This lock keeps them in sequence. See `lock_claims_based_security`.
    cbs_lock: AsyncMutex<()>,
    connections: AsyncMutex<Option<Arc<AmqpConnection>>>,

    // Set by `close_connection` and never cleared. The client that owns this
    // object is not the only holder: a public handle such as `EventReceiver`
    // keeps a reference and can outlive the client. Without this flag such a
    // handle reaches `ensure_connection`, finds no connection, and opens a new
    // one after the application closed the client. Recovery is different and
    // must still work, so only `close_connection` sets this.
    closed: std::sync::atomic::AtomicBool,

    connection_name: String,
    pub(super) retry_options: RetryOptions,

    // Recovery generation counter (#4454), used as a sequence lock.
    // `apply_recovery_plan` bumps it once before it invalidates anything and once
    // after, so the value is odd for exactly as long as a recovery is tearing state
    // down and even at rest. Two properties follow, and `generation_is_current`
    // tests both:
    //
    // * The value changes across every recovery, so a capture taken before one does
    //   not match afterwards.
    // * The value is odd for a capture taken during one, so such a capture is
    //   rejected even when the recovery has not finished by the time it is tested.
    //
    // Invariant: a cached resource is only valid if the generation it was created
    // under is even and still equals the current generation. The four slow paths
    // (authorize_path, get_session, ensure_sender, ensure_receiver) do their AMQP IO
    // with no map lock held; a recovery that overlaps that window leaves the
    // captured generation odd, changed, or both, so the slow path discards its
    // result rather than caching a resource bound to the dead connection. A single
    // counter is used for all resource types: session-level recovery is rare, so the
    // occasional extra re-init of an unaffected type after a narrower recovery is
    // cheaper than the bookkeeping of per-type counters.
    generation: AtomicU64,

    #[cfg(test)]
    forced_error: Mutex<Option<AmqpError>>,

    // Separate from `forced_error`, which the per-operation wrappers
    // (receive, send, management call) consume. This slot is consumed by
    // `ensure_receiver` and `ensure_sender`, so a test can fail an attach
    // without changing how the operation wrappers behave.
    #[cfg(test)]
    forced_attach_error: Mutex<Option<AmqpError>>,

    // Test seam for the caller side of the #4454 supersession race. When armed,
    // `run_peer_supersession_hook` fires once on the next generational init. It
    // plays a peer task that drove a recovery in the window between a caller's
    // generation capture and its cell resolution. See the hook for details.
    #[cfg(test)]
    peer_supersession_pending: std::sync::atomic::AtomicBool,
}

/// A per-path cache cell tagged with the recovery [`RecoverableConnection::generation`]
/// it was created under. See #4454.
struct GenerationalCell<T> {
    generation: u64,
    cell: Arc<OnceCell<Arc<T>>>,
}

impl<T> Clone for GenerationalCell<T> {
    fn clone(&self) -> Self {
        Self {
            generation: self.generation,
            cell: self.cell.clone(),
        }
    }
}

unsafe impl Send for RecoverableConnection {}
unsafe impl Sync for RecoverableConnection {}

/// Returns the per-path cell for `key` valid at `generation`, inserting an
/// uninitialized one if absent. The read path is taken first so steady-state
/// lookups share a read lock; only the first insert for a key (or replacing a cell
/// left over from a previous generation) takes the write lock. The attach then
/// runs inside the returned `OnceCell`, so the map lock is never held across it and
/// different paths set up concurrently. Shared by the sender, session, and receiver
/// caches so all three keep identical concurrency semantics.
///
/// A cached cell whose generation predates `generation` is stale: a recovery
/// cleared and re-stamped the connection after it was created. Such a cell is
/// replaced with a fresh one so the caller re-attaches against the live
/// connection. A cell at a *newer* generation than `generation` is returned
/// as-is rather than overwritten: a recovery already advanced past the
/// generation the caller captured, and a peer task may have attached a valid
/// resource into that newer cell. Clobbering it with a fresh cell stamped at the
/// older `generation` would discard that peer's work and force a redundant
/// re-attach, the exact wasted recovery cycle #4454 set out to remove. The
/// caller's post-`init` generation check sorts out the captured-then-superseded
/// case instead (see [`RecoverableConnection::get_or_init_generational`]). Only
/// a strictly-older or absent entry is replaced. See #4454.
async fn or_init_cell<T>(
    map: &RwLock<HashMap<Url, GenerationalCell<T>>>,
    key: &Url,
    generation: u64,
) -> GenerationalCell<T> {
    if let Some(entry) = map.read().await.get(key) {
        if entry.generation >= generation {
            return entry.clone();
        }
    }
    let mut guard = map.write().await;
    match guard.get(key) {
        Some(entry) if entry.generation >= generation => entry.clone(),
        _ => {
            let fresh = GenerationalCell {
                generation,
                cell: Arc::new(OnceCell::new()),
            };
            guard.insert(key.clone(), fresh.clone());
            fresh
        }
    }
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
        transport: AmqpTransport,
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
                transport,
                retry_options,
                cbs_lock: AsyncMutex::new(()),
                connections: AsyncMutex::new(None),
                session_instances: RwLock::new(HashMap::new()),
                sender_instances: RwLock::new(HashMap::new()),
                receiver_instances: RwLock::new(HashMap::new()),
                mgmt_client: RwLock::new(Arc::new(OnceCell::new())),
                authorizer,
                generation: AtomicU64::new(0),
                #[cfg(test)]
                forced_error: Mutex::new(None),
                #[cfg(test)]
                forced_attach_error: Mutex::new(None),
                #[cfg(test)]
                peer_supersession_pending: std::sync::atomic::AtomicBool::new(false),
                closed: std::sync::atomic::AtomicBool::new(false),
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

    /// Reports whether `close_connection` has run on this object.
    #[cfg(test)]
    pub(crate) fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
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

    /// Makes the next `ensure_receiver` or `ensure_sender` call fail with `error`.
    ///
    /// The injected error takes the same return path as a rejected link
    /// attach: out of the `get_or_try_init` closure, through
    /// `ensure_receiver` or `ensure_sender`, and into the caller's error
    /// handling. Tests use this to drive attach-failure branches without a
    /// live broker.
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
    /// The method is idempotent. It takes `&self`, so the compiler no longer
    /// limits it to one call. A second call finds the flag already set, the
    /// caches already drained, and the connection slot already empty, and it
    /// reports success.
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
    pub(crate) async fn close_connection(&self) -> Result<()> {
        debug!(
            connection_id = %self.get_connection_id(),
            url = %self.url,
            "Closing recoverable connection."
        );

        // Record the close before the teardown starts. A handle that outlives
        // the client, for example an `EventReceiver` that the caller still
        // holds, shares this object, and `ensure_connection` would otherwise
        // open a second connection to the service after the application closed
        // the client.
        self.closed.store(true, Ordering::Release);

        self.authorizer.stop_refresh_task().await;

        // Swap the cell out under the write lock, then detach without holding
        // it. The guard is a separate binding so the lock scope is visible and
        // a debugger can read it.
        let mut cell_slot = self.mgmt_client.write().await;
        let management_cell = std::mem::replace(&mut *cell_slot, Arc::new(OnceCell::new()));
        drop(cell_slot);
        if let Some(Some(management_client)) = Arc::try_unwrap(management_cell)
            .ok()
            .map(OnceCell::into_inner)
        {
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
        for (path, GenerationalCell { cell, .. }) in sender_instances.drain() {
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
        for (source_url, GenerationalCell { cell, .. }) in receiver_instances.drain() {
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
        for (session_id, GenerationalCell { cell, .. }) in session_instances.drain() {
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
        // Read the flag under the lock. `close_connection` sets it before it
        // takes this lock, so a caller that gets here first has its connection
        // closed by the close that waits behind it, and a caller that gets here
        // after the close sees the flag. A check before the lock would leave a
        // window where a caller reads the flag, loses its thread for the whole
        // close, and then opens a connection that nothing closes.
        if self.closed.load(Ordering::Acquire) {
            return Err(AmqpError::with_message(
                "The client that owns this connection is closed.",
            ));
        }
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
        let Some(GenerationalCell { cell, .. }) =
            self.receiver_instances.write().await.remove(source_url)
        else {
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

    /// The recovery generation the caches are currently stamped at. See the
    /// `generation` field and #4454.
    fn current_generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }

    /// Whether a resource created under `captured` is still bound to live state
    /// (#4454). Every slow-path guard tests this, and it holds only when both
    /// halves of the sequence-lock rule do:
    ///
    /// * `captured` is even, so no recovery was in flight when it was taken. An odd
    ///   capture came from inside `apply_recovery_plan`, where the connection may
    ///   already be gone (or about to be taken) and the caches are being cleared.
    /// * `captured` still equals the current generation, so no recovery has started
    ///   since.
    ///
    /// Testing the parity matters on its own. `apply_recovery_plan` releases every
    /// lock it takes, so it can stall between its two bumps under contention; a slow
    /// path that captured an odd generation there has time to finish a whole attach
    /// and test its capture while the value is still unchanged. Equality alone would
    /// accept that attach.
    pub(crate) fn generation_is_current(&self, captured: u64) -> bool {
        captured.is_multiple_of(2) && self.current_generation() == captured
    }

    /// Resolves the per-path cell for `key`, runs `init` to attach the resource
    /// without holding the map lock, and guards the result against a racing
    /// recovery via the generation counter (#4454).
    ///
    /// The attach (`init`) does its AMQP IO with no map lock held, so a recovery
    /// can clear the caches and bump the generation while it is in flight. After
    /// `init` completes this re-reads the generation: if it still matches the one
    /// the cell was created under, the result is fresh and is returned. If it
    /// changed, the just-attached resource is bound to a now-dead connection;
    /// rather than caching and handing out that stale resource (the old behavior
    /// that cost an extra recovery cycle on the next operation), the stale cell is
    /// evicted and the whole attach retries against the new generation.
    ///
    /// `init` is therefore an `FnMut`: it may run more than once if recovery keeps
    /// racing. [`MAX_GENERATION_RETRIES`] bounds the loop, so a pathological storm
    /// of back-to-back recoveries surfaces an error instead of spinning forever.
    async fn get_or_init_generational<T, F, Fut>(
        &self,
        map: &RwLock<HashMap<Url, GenerationalCell<T>>>,
        key: &Url,
        mut init: F,
    ) -> azure_core_amqp::Result<Arc<T>>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = azure_core_amqp::Result<Arc<T>>>,
    {
        for _ in 0..MAX_GENERATION_RETRIES {
            let generation = self.current_generation();

            // Test seam (#4454): reproduce a peer task that drives a recovery and
            // installs a newer cell in the window between this capture and the
            // resolve below. In production this compiles away.
            #[cfg(test)]
            self.run_peer_supersession_hook(map, key).await;

            let entry = or_init_cell(map, key, generation).await;
            let value = entry.cell.get_or_try_init(&mut init).await?;

            // If no recovery raced the attach above, the cell is valid; return it.
            // Test the cell's *own* generation, not the value captured at the top of
            // the loop: `or_init_cell` may have handed back a newer cell that a
            // racing task installed, and such a cell is valid as long as its
            // generation is still current. Using the captured `generation` here
            // would wrongly discard (and evict) that peer's freshly-attached
            // resource. See #4454.
            if self.generation_is_current(entry.generation) {
                return Ok(value.clone());
            }

            // A recovery cleared the caches mid-attach. The value we just produced
            // (or read from a cell another racing task initialized) is bound to the
            // old connection. Evict this cell if it is still the one mapped for
            // `key` so the next pass re-inits against the new generation, then loop.
            debug!(
                %key,
                "Discarding stale resource produced during recovery (#4454); re-initializing."
            );
            let mut guard = map.write().await;
            if let Some(current) = guard.get(key) {
                if Arc::ptr_eq(&current.cell, &entry.cell) {
                    guard.remove(key);
                }
            }
        }

        // Intentionally a plain `AmqpError::with_message`: `should_retry_amqp_error`
        // classifies this unrecognized kind as `ReturnError`, so exhausting the
        // budget surfaces to the caller instead of looping. Do not "fix" this into a
        // retryable kind, that would let a recovery storm spin here forever (#4454).
        Err(AmqpError::with_message(format!(
            "Exceeded retry budget ({MAX_GENERATION_RETRIES}) re-initializing resource '{key}' across recoveries"
        )))
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
        // that sessions for other partitions can be created concurrently. The
        // generation guard discards a session begun against a connection that a
        // racing recovery has since replaced (#4454).
        let session = self
            .get_or_init_generational(&self.session_instances, source_url, || async {
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
        Ok(session)
    }

    /// Returns the `OnceCell` that owns the session for `source_url` at the current
    /// generation, inserting an uninitialized one if absent. See [`or_init_cell`]
    /// for the locking strategy. Used in tests to assert cell identity.
    #[cfg(test)]
    async fn session_cell(&self, source_url: &Url) -> Arc<OnceCell<Arc<AmqpSession>>> {
        or_init_cell(
            &self.session_instances,
            source_url,
            self.current_generation(),
        )
        .await
        .cell
    }

    /// Builds the options handed to [`AmqpConnection::open`]. Kept separate from
    /// `create_connection` so the wiring can be asserted without a broker.
    fn connection_options(&self) -> AmqpConnectionOptions {
        AmqpConnectionOptions {
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
            transport: Some(self.transport),
            ..Default::default()
        }
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
                Some(self.connection_options()),
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
        // Take the cell pointer under a brief read lock, then build without any
        // lock held. The build reaches the CBS retry loop, whose recovery hook
        // can invalidate this cache on the same task; holding a guard here would
        // deadlock that task.
        let cell = self.mgmt_client.read().await.clone();
        let management_client = cell
            .get_or_try_init(|| async {
                RecoverableManagementClient::create_management_client(
                    self.clone(),
                    &self.retry_options,
                )
                .await
            })
            .await?;
        Ok(management_client.clone())
    }

    /// Takes the lock that keeps the claims-based-security round trips of this
    /// connection in sequence.
    ///
    /// The service permits one `$cbs` link for each connection, and it rejects a
    /// second attach with `NotAllowed`. [`Self::ensure_amqp_cbs`] attaches a new
    /// link for each authorization, so the caller must hold this lock for the
    /// full round trip.
    ///
    /// Without this lock, the authorizations for different paths overlap when a
    /// client sets up more than one link at once, for example a buffered
    /// producer that starts one sender for each partition. The lock covers only
    /// the authorization. The link attach that follows and the session begin
    /// stay concurrent.
    pub(super) async fn lock_claims_based_security(&self) -> AsyncMutexGuard<'_, ()> {
        self.cbs_lock.lock().await
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
        let receiver = self
            .get_or_init_generational(&self.receiver_instances, source_url, || async {
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

        Ok(receiver)
    }

    /// Returns the `OnceCell` that owns the receiver for `source_url` at the current
    /// generation, inserting an uninitialized one if absent. See [`or_init_cell`]
    /// for the locking strategy. Used in tests to assert cell identity.
    #[cfg(test)]
    async fn receiver_cell(&self, source_url: &Url) -> Arc<OnceCell<Arc<AmqpReceiver>>> {
        or_init_cell(
            &self.receiver_instances,
            source_url,
            self.current_generation(),
        )
        .await
        .cell
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
        let sender = self
            .get_or_init_generational(&self.sender_instances, path, || async {
                // Test seam: fail the attach with an injected error before any
                // network activity. The error takes the same path as a
                // rejected sender attach below it.
                #[cfg(test)]
                self.get_forced_attach_error()?;

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

        Ok(sender)
    }

    /// Returns the `OnceCell` that owns the sender for `path` at the current
    /// generation, inserting an uninitialized one if absent. See [`or_init_cell`]
    /// for the locking strategy. Used in tests to assert cell identity.
    #[cfg(test)]
    async fn sender_cell(&self, path: &Url) -> Arc<OnceCell<Arc<AmqpSender>>> {
        or_init_cell(&self.sender_instances, path, self.current_generation())
            .await
            .cell
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
    ///
    /// #4454 stale-resource window. Any plan that invalidates something brackets
    /// that invalidation with a bump of the recovery `generation`: one before it
    /// and one after it, which leaves the counter odd for the whole span. A slow
    /// path (authorize_path / get_session / ensure_sender / ensure_receiver) that is
    /// mid-attach captured a generation from inside or before that bracket, so
    /// `generation_is_current` rejects it on completion and the slow path discards
    /// its result instead of caching a resource bound to the connection this
    /// recovery just tore down. The body explains why one bump on either side alone
    /// is not enough.
    async fn apply_recovery_plan(&self, plan: RecoveryPlan) {
        let connection_id = self.get_connection_id();

        // A plan that invalidates anything brackets the invalidation with a
        // generation bump: one before it touches the connection or any cache, and
        // one after the last of them. The generation is therefore odd for exactly
        // the span in which this connection's state is inconsistent, which is the
        // sequence-lock rule `generation_is_current` tests (#4454).
        //
        // Both bumps are needed, and so is the parity test:
        //
        // * Without the closing bump, a slow path that captured the old generation
        //   can clone the connection, attach, and test its capture before the single
        //   bump lands. The generation still matches, so it caches and returns a
        //   resource bound to the connection this recovery drops a moment later.
        // * Without the opening bump, a slow path can capture the new generation and
        //   *then* clone the old connection, which `connections` still holds. Its
        //   post-init test matches too, so the same stale resource reaches the
        //   caller. The token cache has the same shape: a reader that runs after the
        //   bump and before `clear()` gets a token that was authorized on the CBS
        //   link of the connection being dropped.
        // * Without the parity test, a slow path that captured a generation between
        //   the two bumps is accepted for as long as this function has not reached
        //   the closing one. Every lock below is released before the next is taken,
        //   so a contended recovery can stall here long enough for that slow path to
        //   finish a whole attach against the connection being dropped.
        //
        // A task that captures the final, even generation started after the last
        // invalidation, so it finds an empty cache and builds against the new
        // connection.
        let invalidates = plan.drop_connection
            || plan.clear_authorizer
            || plan.clear_sessions
            || plan.clear_senders
            || plan.clear_receivers;

        if invalidates {
            self.generation.fetch_add(1, Ordering::AcqRel);
        }

        if plan.drop_connection {
            self.connections.lock().await.take();
            debug!(connection_id = %connection_id, "Recovery: dropped AMQP connection.");
        }

        if plan.clear_authorizer {
            self.authorizer.clear().await;
            debug!(connection_id = %connection_id, "Recovery: cleared authorizer tokens.");
        }
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
            // Swap in a fresh cell instead of clearing the old one in place. The
            // write lock is held only for the pointer swap, never across a build,
            // so this never waits for a management-client build in flight (which,
            // on the CBS failure path, runs on this very task).
            *self.mgmt_client.write().await = Arc::new(OnceCell::new());
            debug!(connection_id = %connection_id, "Recovery: dropped management client.");
        }

        // Closing bump. See the comment above the opening one.
        if invalidates {
            self.generation.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// The recovery generation, exposed for the authorizer's slow-path guard
    /// (#4454) and for tests. See `current_generation`.
    pub(crate) fn generation(&self) -> u64 {
        self.current_generation()
    }

    /// Test hook: simulate the cache-clearing half of a `ReconnectConnection`
    /// recovery (bump the generation and clear the per-path caches) without needing
    /// a live broker connection. Used to drive the #4454 stale-resource race
    /// deterministically.
    #[cfg(test)]
    pub(crate) async fn simulate_reconnect(&self) {
        self.apply_recovery_plan(
            RecoveryPlan::for_action(&ErrorRecoveryAction::ReconnectConnection)
                .expect("ReconnectConnection has a recovery plan"),
        )
        .await;
    }

    /// Test hook: advance the recovery generation past one whole recovery without
    /// taking the cache locks. Used to exercise the #4454 generation guard in
    /// isolation. The step is two, the same as a completed `apply_recovery_plan`,
    /// so the counter is left even and a capture taken after this hook is current.
    #[cfg(test)]
    pub(crate) fn bump_generation_for_test(&self) {
        self.generation.fetch_add(2, Ordering::AcqRel);
    }

    /// Test hook: park the generation mid-recovery, as `apply_recovery_plan` does
    /// between its two bumps, and leave it there. Used to assert that a capture
    /// taken during a recovery is rejected even when the recovery has not finished.
    #[cfg(test)]
    pub(crate) fn enter_recovery_generation_for_test(&self) {
        self.generation.fetch_add(1, Ordering::AcqRel);
    }

    /// Arms `run_peer_supersession_hook` to fire on the next generational init.
    /// See that hook and the `get_or_init_generational_returns_superseding_peer_cell`
    /// test.
    #[cfg(test)]
    pub(crate) fn arm_peer_supersession_for_test(&self) {
        self.peer_supersession_pending
            .store(true, Ordering::Release);
    }

    /// Test seam for the caller side of the #4454 supersession property. When
    /// armed by `arm_peer_supersession_for_test`, this fires once, in the window
    /// between a caller capturing its generation and resolving its cell. It plays
    /// a peer task that drove a recovery in that window: it bumps the generation,
    /// so the caller's captured value is now stale, and installs a fresh, empty
    /// cell for `key` at the new generation. `or_init_cell` then hands that newer
    /// cell back to the caller.
    ///
    /// The correct guard in `get_or_init_generational` compares the cell's own
    /// generation, so it returns the resource the caller attaches into that cell:
    /// the cell is current, the resource is valid, and no eviction happens. A
    /// guard that compared the captured local generation instead would see a
    /// mismatch, evict the valid cell, and re-attach, the wasted recovery cycle
    /// #4454 removes. The empty cell makes that difference observable: a correct
    /// return keeps the caller's single `init`, a wrong eviction forces a second.
    /// This compiles away in production; the field is `cfg(test)` only.
    #[cfg(test)]
    async fn run_peer_supersession_hook<T>(
        &self,
        map: &RwLock<HashMap<Url, GenerationalCell<T>>>,
        key: &Url,
    ) {
        if self.peer_supersession_pending.swap(false, Ordering::AcqRel) {
            // Drive the peer's recovery to completion, so the caller's captured
            // generation is now behind and the new one is settled (even).
            self.generation.fetch_add(2, Ordering::AcqRel);
            // Install the peer's fresh, higher-generation cell. It is empty on
            // purpose, so the caller's own `init` fills it.
            let generation = self.current_generation();
            map.write().await.insert(
                key.clone(),
                GenerationalCell {
                    generation,
                    cell: Arc::new(OnceCell::new()),
                },
            );
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
    use azure_core::{
        credentials::{AccessToken, TokenCredential, TokenRequestOptions},
        http::Url,
        time::{Duration, OffsetDateTime},
    };
    use azure_core_test::credentials::MockCredential;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tokio::sync::Notify;

    // A close does not need exclusive ownership of the connection.
    //
    // `close_connection` used to take `self`, so every caller first had to take
    // the value out of its `Arc`. A handle that the application still held, for
    // example an `EventReceiver`, made that fail, and the client reported an
    // error and left the connection open. `Drop` only writes a trace message,
    // so nothing closed the connection after that.
    #[tokio::test]
    async fn close_works_while_another_reference_exists() {
        let connection = RecoverableConnection::new(
            Url::parse("amqps://example.com").unwrap(),
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );

        // Stand in for a public handle that outlives the client.
        let handle = connection.clone();
        assert_eq!(Arc::strong_count(&connection), 2);

        connection
            .close_connection()
            .await
            .expect("a close must not need exclusive ownership");

        // The connection records the close, so the surviving handle cannot open
        // a second connection to the service.
        let Err(error) = handle.ensure_connection().await else {
            panic!("a closed connection must not open a new one");
        };
        assert!(
            error.to_string().contains("closed"),
            "the error must say that the client is closed, got: {error}"
        );
    }

    #[tokio::test]
    async fn close_stops_owned_authorization_refresh_task() {
        #[derive(Debug)]
        struct GatedCredential {
            requests: AtomicUsize,
            entered_refresh: Notify,
            release_refresh: Notify,
        }

        #[async_trait::async_trait]
        impl TokenCredential for GatedCredential {
            async fn get_token(
                &self,
                _scopes: &[&str],
                _options: Option<TokenRequestOptions<'_>>,
            ) -> azure_core::Result<AccessToken> {
                match self.requests.fetch_add(1, Ordering::SeqCst) {
                    0 => Ok(AccessToken::new(
                        azure_core::credentials::Secret::new("initial_token"),
                        OffsetDateTime::now_utc() + Duration::hours(1),
                    )),
                    1 => {
                        self.entered_refresh.notify_one();
                        self.release_refresh.notified().await;
                        Ok(AccessToken::new(
                            azure_core::credentials::Secret::new("refreshed_token"),
                            OffsetDateTime::now_utc() + Duration::hours(1),
                        ))
                    }
                    request => unreachable!("unexpected token request {request}"),
                }
            }
        }

        let credential = Arc::new(GatedCredential {
            requests: AtomicUsize::new(0),
            entered_refresh: Notify::new(),
            release_refresh: Notify::new(),
        });
        let connection = RecoverableConnection::new(
            Url::parse("amqps://example.com").unwrap(),
            None,
            None,
            AmqpTransport::default(),
            credential.clone(),
            Default::default(),
            None,
        );
        let authorizer = connection.authorizer.clone();
        authorizer.disable_authorization().unwrap();
        authorizer
            .set_token_refresh_bias_for_test(Duration::hours(2))
            .unwrap();

        let path = Url::parse("amqps://example.com/close_refresh_task").unwrap();
        authorizer.authorize_path(&connection, &path).await.unwrap();

        // The second request proves that the refresher holds an Arc to the authorizer.
        credential.entered_refresh.notified().await;

        connection.close_connection().await.unwrap();
        assert_eq!(
            Arc::strong_count(&authorizer),
            2,
            "the connection and test authorizer references must remain after close"
        );
        drop(connection);
        assert_eq!(
            Arc::strong_count(&authorizer),
            1,
            "authorization refresh task remained alive after close; strong_count={}",
            Arc::strong_count(&authorizer)
        );
    }

    #[tokio::test]
    async fn close_racing_first_authorization_does_not_start_refresher() {
        #[derive(Debug)]
        struct GatedCredential {
            requests: AtomicUsize,
            entered: Notify,
            release: Notify,
        }

        #[async_trait::async_trait]
        impl TokenCredential for GatedCredential {
            async fn get_token(
                &self,
                _scopes: &[&str],
                _options: Option<TokenRequestOptions<'_>>,
            ) -> azure_core::Result<AccessToken> {
                if self.requests.fetch_add(1, Ordering::SeqCst) == 0 {
                    self.entered.notify_one();
                    self.release.notified().await;
                }
                Ok(AccessToken::new(
                    azure_core::credentials::Secret::new("initial_token"),
                    OffsetDateTime::now_utc() + Duration::hours(1),
                ))
            }
        }

        let credential = Arc::new(GatedCredential {
            requests: AtomicUsize::new(0),
            entered: Notify::new(),
            release: Notify::new(),
        });
        let connection = RecoverableConnection::new(
            Url::parse("amqps://example.com").unwrap(),
            None,
            None,
            AmqpTransport::default(),
            credential.clone(),
            Default::default(),
            None,
        );
        let authorizer = connection.authorizer.clone();
        authorizer.disable_authorization().unwrap();

        let path = Url::parse("amqps://example.com/close_first_authorization").unwrap();
        let authorization = {
            let authorizer = authorizer.clone();
            let connection = connection.clone();
            tokio::spawn(async move { authorizer.authorize_path(&connection, &path).await })
        };

        credential.entered.notified().await;
        connection.close_connection().await.unwrap();
        credential.release.notify_one();
        authorization
            .await
            .expect("authorize_path task panicked")
            .expect("authorize_path returned an error");

        assert_eq!(
            credential.requests.load(Ordering::SeqCst),
            1,
            "the first authorization must make one token request"
        );
        drop(connection);
        assert_eq!(
            Arc::strong_count(&authorizer),
            1,
            "authorization refresh task must not start after close"
        );
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
            AmqpTransport::default(),
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
            AmqpTransport::default(),
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
            AmqpTransport::default(),
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
            AmqpTransport::default(),
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

    // #4454: a recovery that clears the per-path caches must bump the recovery
    // generation so racing slow-path attaches can detect it. A simulated
    // ReconnectConnection must advance `generation()`.
    //
    // The step is two, not one: `apply_recovery_plan` brackets its invalidation
    // with a bump on each side, so a task that captures the generation part way
    // through the recovery also sees a mismatch when it completes. The exact value
    // is asserted here to pin that bracketing; nothing else compares generations by
    // anything other than equality.
    #[tokio::test]
    async fn simulate_reconnect_bumps_generation() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );

        assert_eq!(connection.generation(), 0);
        connection.simulate_reconnect().await;
        assert_eq!(connection.generation(), 2);
        connection.simulate_reconnect().await;
        assert_eq!(connection.generation(), 4);
    }

    // #4454: a generation captured *part way through* a recovery must also end up
    // stale. This is the edge a single leading bump leaves open: a slow path that
    // starts after the bump can still clone the connection that `apply_recovery_plan`
    // has not taken yet, or read a token that `clear()` has not removed yet, and its
    // post-init check would then match and hand the caller a resource bound to the
    // connection this recovery is dropping.
    //
    // The token cache's write lock is the seam. Holding it stops the recovery inside
    // `authorizer.clear()`, which is after the opening bump and before the closing
    // one, so the test can capture the generation a racing slow path would see.
    #[tokio::test]
    async fn recovery_generation_differs_for_a_mid_recovery_capture() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );
        connection.disable_connection().await.unwrap();

        let scopes = connection.authorizer.lock_scopes_for_test().await;

        let recovery = {
            let connection = connection.clone();
            tokio::spawn(async move { connection.simulate_reconnect().await })
        };

        // Wait for the opening bump. The recovery then blocks on the guard above.
        while connection.generation() == 0 {
            tokio::task::yield_now().await;
        }
        let captured_mid_recovery = connection.generation();

        drop(scopes);
        recovery.await.expect("recovery task panicked");

        assert_ne!(
            connection.generation(),
            captured_mid_recovery,
            "a generation captured during a recovery must not survive it, or a slow \
             path that started mid-recovery would pass its post-init check and cache \
             a resource bound to the dropped connection"
        );
    }

    // #4454: the core of the fix. A cell resolved under generation N must be
    // replaced by a fresh, distinct cell once the generation advances to N+1,
    // because the recovery that bumped the generation tore down the connection the
    // old cell's resource was attached to. Resolving at the same generation must
    // keep returning the same cell (so we don't lose single-init within a
    // generation).
    #[tokio::test]
    async fn stale_generation_cell_is_replaced() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );
        let path = Url::parse("amqps://example.com/eh/Partitions/0").unwrap();

        // Two resolutions at the same generation share a cell.
        let cell_gen0 = connection.sender_cell(&path).await;
        assert!(Arc::ptr_eq(
            &cell_gen0,
            &connection.sender_cell(&path).await
        ));

        // After a recovery, the next resolution returns a brand-new cell.
        connection.simulate_reconnect().await;
        let cell_gen1 = connection.sender_cell(&path).await;
        assert!(
            !Arc::ptr_eq(&cell_gen0, &cell_gen1),
            "cell from the previous generation must be discarded after recovery"
        );

        // And that new cell is itself stable within its generation.
        assert!(Arc::ptr_eq(
            &cell_gen1,
            &connection.sender_cell(&path).await
        ));
    }

    // #4454: an attach that both starts and finishes inside a recovery must be
    // discarded too. The two bumps make the generation odd for the span in which
    // `apply_recovery_plan` is invalidating state, so a capture taken there is
    // rejected on parity alone, without waiting for the recovery to end.
    //
    // Equality against the captured value cannot catch this case: the generation
    // has not moved since the capture. `apply_recovery_plan` releases each lock
    // before it takes the next, so a contended recovery can stall between its bumps
    // long enough for a slow path to finish attaching to the connection it is
    // dropping. The test parks the generation mid-recovery to hold that state open.
    #[tokio::test]
    async fn generation_captured_mid_recovery_is_never_current() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );
        let path = Url::parse("amqps://example.com/eh/Partitions/0").unwrap();

        assert!(connection.generation_is_current(connection.generation()));

        // Park the counter where `apply_recovery_plan` holds it between its bumps.
        connection.enter_recovery_generation_for_test();
        let captured_mid_recovery = connection.generation();
        assert!(
            !connection.generation_is_current(captured_mid_recovery),
            "a generation captured during a recovery must never be current, even \
             while the recovery is still in flight and the value is unchanged"
        );

        // An attach that runs entirely inside the recovery is therefore never
        // cached. It retries to the budget and surfaces an error instead.
        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let map: RwLock<HashMap<Url, GenerationalCell<u64>>> = RwLock::new(HashMap::new());
        let result =
            connection
                .get_or_init_generational(&map, &path, || {
                    let calls = calls.clone();
                    async move {
                        Ok::<_, AmqpError>(Arc::new(calls.fetch_add(1, Ordering::SeqCst) as u64))
                    }
                })
                .await;

        assert!(
            result.is_err(),
            "a resource attached during a recovery must not be handed to the caller"
        );
        assert_eq!(calls.load(Ordering::SeqCst), MAX_GENERATION_RETRIES);
        assert!(
            map.read().await.get(&path).is_none(),
            "no cell attached during a recovery may stay cached"
        );
    }

    // #4454: `get_or_init_generational` must discard a value produced during a
    // racing recovery and re-init against the new generation. Here the init closure
    // fires a simulated reconnect on its first call (the in-flight-slow-path
    // window), so the first attempt's value is stale and must be thrown away; the
    // second attempt runs at a stable generation and its value is the one returned
    // and cached.
    #[tokio::test]
    async fn get_or_init_generational_discards_value_produced_during_recovery() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );
        let path = Url::parse("amqps://example.com/eh/Partitions/0").unwrap();

        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let map: RwLock<HashMap<Url, GenerationalCell<u64>>> = RwLock::new(HashMap::new());

        let result = connection
            .get_or_init_generational(&map, &path, || {
                let calls = calls.clone();
                let connection = &connection;
                async move {
                    let attempt = calls.fetch_add(1, Ordering::SeqCst);
                    // On the first attempt only, simulate a recovery firing during
                    // the lock-free init window. This bumps the generation, so the
                    // value produced here is stale and must be discarded.
                    if attempt == 0 {
                        connection.simulate_reconnect().await;
                    }
                    Ok::<_, AmqpError>(Arc::new(attempt as u64))
                }
            })
            .await
            .expect("init should succeed on the second, stable-generation attempt");

        // The closure ran twice: once racing the recovery (discarded), once clean.
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        // The returned value is the second attempt's (index 1), not the stale first.
        assert_eq!(*result, 1);
        // The cached cell holds the fresh value, stamped at the post-recovery generation.
        let cached = map.read().await.get(&path).cloned().unwrap();
        assert_eq!(cached.generation, connection.generation());
        assert_eq!(**cached.cell.get().unwrap(), 1);
    }

    // #4454 regression: `or_init_cell` must never overwrite a cell at a *newer*
    // generation than the one the caller captured. A slow task that captured
    // generation N can reach the lookup only after a recovery advanced to N+1 and a
    // peer task already cached a valid resource there; clobbering it with a fresh
    // gen-N cell would discard the peer's freshly-attached resource and force a
    // redundant re-attach, the exact wasted recovery cycle the fix removes. A
    // strictly-older cached cell must still be replaced so the caller re-attaches
    // against the live connection.
    #[tokio::test]
    async fn or_init_cell_reuses_newer_cell_and_replaces_older() {
        let path = Url::parse("amqps://example.com/eh/Partitions/0").unwrap();
        let map: RwLock<HashMap<Url, GenerationalCell<u64>>> = RwLock::new(HashMap::new());

        // A peer task at generation 1 attached a resource and cached it.
        let newer = or_init_cell(&map, &path, 1).await;
        newer.cell.set(Arc::new(42)).await.unwrap();
        assert_eq!(newer.generation, 1);

        // A slow task that captured the stale generation 0 resolves the same key. It
        // must get the gen-1 cell back, value intact, not a fresh gen-0 cell that
        // throws the peer's work away.
        let stale = or_init_cell(&map, &path, 0).await;
        assert!(
            Arc::ptr_eq(&stale.cell, &newer.cell),
            "a cell newer than the captured generation must be reused, not clobbered"
        );
        assert_eq!(stale.generation, 1);
        assert_eq!(**stale.cell.get().unwrap(), 42);

        // Resolving at a generation strictly newer than the cached cell replaces it
        // with a fresh, empty cell so the caller re-attaches against the live
        // connection.
        let replaced = or_init_cell(&map, &path, 2).await;
        assert!(
            !Arc::ptr_eq(&replaced.cell, &newer.cell),
            "a cell older than the captured generation must be replaced"
        );
        assert_eq!(replaced.generation, 2);
        assert!(replaced.cell.get().is_none());
    }

    // #4454 regression, the caller side of the supersession property. `or_init_cell`
    // owns one half: it never overwrites a newer cell (see
    // `or_init_cell_reuses_newer_cell_and_replaces_older`). `get_or_init_generational`
    // owns the other half, exercised here: when `or_init_cell` hands back a cell that
    // is newer than the caller's captured generation but still current, the caller
    // must attach into it and return the result, not evict it against the stale
    // captured generation and re-attach.
    //
    // The scenario is a slow caller that captures generation N, then a peer task
    // drives a recovery to N+1 and installs a fresh cell there before the caller
    // resolves its own cell. The `run_peer_supersession_hook` seam reproduces that
    // peer exactly, in the capture-to-resolve window, so the race is deterministic.
    //
    // With the correct guard (compare the cell's own generation) the init closure
    // runs once and its value is returned. If the guard wrongly compared the captured
    // local generation, the caller would evict the valid cell and run init a second
    // time; this test then fails on the call count and the returned value. That is the
    // mutation the earlier proof left uncaught, so this test closes the gap.
    #[tokio::test]
    async fn get_or_init_generational_returns_superseding_peer_cell() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );
        let path = Url::parse("amqps://example.com/eh/Partitions/0").unwrap();

        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let map: RwLock<HashMap<Url, GenerationalCell<u64>>> = RwLock::new(HashMap::new());

        // Arm the peer: on the next generational init it bumps the generation and
        // installs a fresh, higher-generation cell between the capture and the resolve.
        connection.arm_peer_supersession_for_test();

        let result = connection
            .get_or_init_generational(&map, &path, || {
                let calls = calls.clone();
                async move {
                    let attempt = calls.fetch_add(1, Ordering::SeqCst);
                    Ok::<_, AmqpError>(Arc::new(attempt as u64))
                }
            })
            .await
            .expect("init should succeed against the peer's current-generation cell");

        // The init closure ran exactly once: the caller attached into the peer's
        // newer-but-current cell and returned it, with no eviction and no retry.
        assert_eq!(
            calls.load(Ordering::SeqCst),
            1,
            "a newer-but-current cell must be returned, not evicted and re-initialized"
        );
        // The returned value is that single init's value (attempt index 0).
        assert_eq!(*result, 0);
        // The cached cell is stamped at the post-recovery generation and holds the value.
        let cached = map.read().await.get(&path).cloned().unwrap();
        assert_eq!(cached.generation, connection.generation());
        assert_eq!(**cached.cell.get().unwrap(), 0);
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
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );

        assert_eq!(connection_manager.custom_endpoint, Some(custom_endpoint));
    }

    // The transport selected on a client builder (and, transitively, on an
    // EventProcessor's injected ConsumerClient) must reach the connection so it
    // is applied when the AMQP connection is opened. This verifies the field is
    // stored on the RecoverableConnection.
    #[test]
    fn constructor_with_websocket_transport() {
        let url = Url::parse("amqps://example.com").unwrap();
        let connection_manager = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::WebSocket,
            Arc::new(MockCredential),
            Default::default(),
            None,
        );

        assert_eq!(connection_manager.transport, AmqpTransport::WebSocket);
    }

    // The stored transport must also reach the options handed to
    // `AmqpConnection::open`. Asserting on the constructor alone would still
    // pass if `create_connection` dropped the `with_transport` call.
    #[test]
    fn connection_options_carry_the_transport() {
        let url = Url::parse("amqps://example.com").unwrap();
        let custom_endpoint = Url::parse("amqps://proxy.example.com:8081").unwrap();
        for transport in [AmqpTransport::Tcp, AmqpTransport::WebSocket] {
            let connection_manager = RecoverableConnection::new(
                url.clone(),
                None,
                Some(custom_endpoint.clone()),
                transport,
                Arc::new(MockCredential),
                Default::default(),
                None,
            );

            let options = connection_manager.connection_options();
            assert_eq!(options.transport, Some(transport));
            assert_eq!(options.custom_endpoint, Some(custom_endpoint.clone()));
            assert!(options.properties.is_some());
        }
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

        // Test SimpleMessage (the kind `AmqpError::with_message` produces) ->
        // ReturnError. The retry-budget-exhausted errors in `get_or_init_generational`
        // and `authorize_path` are `with_message` errors that intentionally rely on
        // this classification to surface instead of spinning across a recovery storm
        // (#4454). This pins that contract: adding an explicit `SimpleMessage` arm, or
        // flipping the `_` default to a retryable action, must fail here and force a
        // deliberate decision rather than silently turning those backstops into an
        // infinite retry loop.
        let err = AmqpError::with_message("retry budget exhausted");
        assert_eq!(
            RecoverableConnection::should_retry_amqp_error(&err),
            ErrorRecoveryAction::ReturnError
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

    // The management-client build must not hold any `mgmt_client` lock.
    //
    // This points the connection at a local TCP peer that accepts the socket and
    // never sends the AMQP protocol header, so `create_connection` stays inside
    // `ensure_amqp_management` for the whole test. The cache lock must still be
    // free: it only guards the cell pointer, so recovery and `close_connection`
    // can take it while a build is in flight.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn management_build_does_not_hold_mgmt_lock() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind stalled AMQP peer");
        let port = listener.local_addr().expect("listener address").port();
        // Hold every accepted socket open and answer nothing. The first accept
        // signals the test, which is the synchronization point that makes this
        // test deterministic. A fixed sleep would not do: on a loaded runner the
        // build task can still be pending, and the assertions below would then
        // pass against the old implementation.
        let (accepted_tx, accepted_rx) = tokio::sync::oneshot::channel();
        std::thread::spawn(move || {
            let mut accepted = Vec::new();
            let mut accepted_tx = Some(accepted_tx);
            while let Ok((stream, _)) = listener.accept() {
                if let Some(tx) = accepted_tx.take() {
                    let _ = tx.send(());
                }
                accepted.push(stream);
            }
        });

        let url = Url::parse(&format!("amqp://127.0.0.1:{port}")).expect("stalled peer URL");
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );

        let build = tokio::spawn({
            let connection = connection.clone();
            async move {
                let _ = connection.ensure_amqp_management().await;
            }
        });

        // The old code took the `mgmt_client` guard before it opened the
        // connection, so a completed accept proves the build is past that point
        // and inside the region that used to be locked.
        tokio::time::timeout(std::time::Duration::from_secs(30), accepted_rx)
            .await
            .expect("the build did not connect to the stalled peer within 30s")
            .expect("the listener thread dropped the accept signal");

        let build_is_running = !build.is_finished();
        let lock_is_free = connection.mgmt_client.try_write().is_some();
        build.abort();

        assert!(
            build_is_running,
            "The management-client build finished instead of blocking on the stalled peer."
        );
        assert!(
            lock_is_free,
            "`ensure_amqp_management` held the `mgmt_client` lock across the build. That is \
             the self-deadlock: the build authorizes the management path, and a CBS failure \
             there re-enters the same lock through `apply_recovery_plan` on the same task."
        );
    }

    // Recovery must never wait for an in-flight management-client build.
    //
    // This test uses production entry points only: one task calls
    // `ensure_amqp_management` against a TCP peer that accepts the socket and
    // then answers nothing, so the build stays in flight. A second task then
    // runs `recover_from_error` for `ReconnectLink`, the action a detached or
    // stolen CBS link produces. That plan sets `drop_mgmt_client` and leaves the
    // connection alone.
    //
    // While the management client lived behind a single `AsyncMutex` that
    // `ensure_amqp_management` held across the whole build, the recovery task
    // waited on that guard and never returned. The same wait happens on one
    // task in production (build -> CBS authorize -> retry loop -> recovery
    // hook), where it is a self-deadlock instead of contention.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn recovery_does_not_wait_for_in_flight_management_build() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind stalled AMQP peer");
        let port = listener.local_addr().expect("listener address").port();
        // Hold every accepted socket open and answer nothing. The first accept
        // signals the test, which is the synchronization point that makes this
        // test deterministic. A fixed sleep would not do: on a loaded runner the
        // build task can still be pending, and the assertions below would then
        // pass against the old implementation.
        let (accepted_tx, accepted_rx) = tokio::sync::oneshot::channel();
        std::thread::spawn(move || {
            let mut accepted = Vec::new();
            let mut accepted_tx = Some(accepted_tx);
            while let Ok((stream, _)) = listener.accept() {
                if let Some(tx) = accepted_tx.take() {
                    let _ = tx.send(());
                }
                accepted.push(stream);
            }
        });

        let url = Url::parse(&format!("amqp://127.0.0.1:{port}")).expect("stalled peer URL");
        let connection = RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        );

        let build = tokio::spawn({
            let connection = connection.clone();
            async move {
                let _ = connection.ensure_amqp_management().await;
            }
        });

        // Wait for the build to reach the stalled peer. The old code took the
        // `mgmt_client` guard before it opened the connection, so a completed
        // accept proves the build holds whatever lock the implementation takes.
        tokio::time::timeout(std::time::Duration::from_secs(30), accepted_rx)
            .await
            .expect("the build did not connect to the stalled peer within 30s")
            .expect("the listener thread dropped the accept signal");

        assert!(
            !build.is_finished(),
            "The management-client build finished instead of blocking on the stalled peer."
        );

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            RecoverableConnection::recover_from_error(
                Arc::downgrade(&connection),
                ErrorRecoveryAction::ReconnectLink,
            ),
        )
        .await;
        build.abort();

        assert!(
            result.is_ok(),
            "Recovery did not complete in 10s: it waited for the in-flight management-client \
             build. On the production path the same wait happens on a single task and hangs \
             forever."
        );
    }

    fn cbs_lock_test_connection() -> Arc<RecoverableConnection> {
        let url = Url::parse("amqps://example.com").unwrap();
        RecoverableConnection::new(
            url,
            None,
            None,
            AmqpTransport::default(),
            Arc::new(MockCredential),
            Default::default(),
            None,
        )
    }

    // The service permits one `$cbs` link for each connection, so an
    // authorization must not start while another one holds the link. A second
    // caller must wait until the first guard drops. This test needs no network,
    // because it exercises the lock that `authorize_path` takes.
    #[tokio::test]
    async fn cbs_lock_blocks_a_second_caller_until_the_guard_drops() {
        let connection = cbs_lock_test_connection();

        let guard = connection.lock_claims_based_security().await;
        assert!(
            connection.cbs_lock.try_lock().is_none(),
            "a second caller must not take the lock while the first one holds it"
        );

        drop(guard);
        assert!(
            connection.cbs_lock.try_lock().is_some(),
            "the lock must be free after the guard drops"
        );
    }

    // Regression guard for the `NotAllowed` failure: if a later change moves or
    // narrows the guard in `authorize_path`, two round trips can overlap again.
    // Count the callers that hold the lock at the same time, and make sure the
    // count never goes above one.
    #[tokio::test]
    async fn cbs_lock_never_lets_two_callers_overlap() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let connection = cbs_lock_test_connection();
        let in_flight = Arc::new(AtomicUsize::new(0));
        let most_seen = Arc::new(AtomicUsize::new(0));

        let mut tasks = Vec::new();
        for _ in 0..8 {
            let connection = connection.clone();
            let in_flight = in_flight.clone();
            let most_seen = most_seen.clone();
            tasks.push(tokio::spawn(async move {
                let _guard = connection.lock_claims_based_security().await;
                let now = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
                most_seen.fetch_max(now, Ordering::SeqCst);
                // Give the other tasks a chance to run while this one holds the
                // lock, which is what a real round trip does at its await points.
                for _ in 0..4 {
                    tokio::task::yield_now().await;
                }
                in_flight.fetch_sub(1, Ordering::SeqCst);
            }));
        }

        for task in tasks {
            task.await.unwrap();
        }

        assert_eq!(
            most_seen.load(Ordering::SeqCst),
            1,
            "the claims-based-security round trips of one connection must not overlap"
        );
    }
}
