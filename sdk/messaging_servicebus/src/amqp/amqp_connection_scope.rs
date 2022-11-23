use std::{collections::HashMap, sync::atomic::Ordering, time::Duration as StdDuration};

use async_trait::async_trait;
use azure_core::{auth::TokenCredential, Url};
use fe2o3_amqp::{
    connection::{ConnectionHandle, OpenError},
    link::{receiver::CreditMode, ReceiverAttachError, SenderAttachError},
    sasl_profile::SaslProfile,
    session::{BeginError, SessionHandle},
    transaction::Controller,
    Connection, Session,
};

use fe2o3_amqp_cbs::{client::CbsClient, AsyncCbsTokenProvider};
use fe2o3_amqp_management::client::MgmtClient;
use fe2o3_amqp_types::{
    definitions::{ReceiverSettleMode, SenderSettleMode, MAJOR, MINOR, REVISION},
    messaging::{FilterSet, Source},
    primitives::Symbol,
};
use fe2o3_amqp_ws::WebSocketStream;
use rand::{rngs::StdRng, SeedableRng};
use serde_amqp::Value;
use time::{Duration as TimeSpan, OffsetDateTime};
use tokio::time::{error::Elapsed, timeout, Interval};
use tokio_util::sync::CancellationToken;

use crate::{
    authorization::{service_bus_claim, service_bus_token_credential::ServiceBusTokenCredential},
    core::TransportConnectionScope,
    primitives::service_bus_transport_type::ServiceBusTransportType,
    ServiceBusReceiveMode,
};

use super::{
    amqp_client_constants::{self, MANAGEMENT_ADDRESS},
    amqp_connection::AmqpConnection,
    amqp_constants,
    amqp_session::AmqpSession,
    cbs_token_provider::CbsTokenProvider,
    error::{CbsAuthError, DisposeError, OpenMgmtLinkError, OpenReceiverError, OpenSenderError},
    filters::SessionFilter,
    LINK_IDENTIFIER,
};

const AUTHORIZATION_REFRESH_BUFFER_SECONDS: u64 = 7 * 60;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpConnectionScopeError {
    #[error(transparent)]
    Open(#[from] OpenError),

    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    #[error(transparent)]
    TimeoutElapsed(#[from] Elapsed),

    #[error(transparent)]
    Begin(#[from] BeginError),

    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    #[error(transparent)]
    Rng(#[from] rand::Error),
}

pub(crate) struct AmqpConnectionScope<TC: TokenCredential> {
    /// <summary>The seed to use for initializing random number generated for a given thread-specific instance.</summary>
    // private static int s_randomSeed = Environment.TickCount;

    /// <summary>The random number generator to use for a specific thread.</summary>
    // private static readonly ThreadLocal<Random> RandomNumberGenerator = new ThreadLocal<Random>(() => new Random(Interlocked.Increment(ref s_randomSeed)), false);
    random_number_generator: StdRng,

    /// <summary>Indicates whether or not this instance has been disposed.</summary>
    is_disposed: bool,

    /// The cancellation token to use with operations initiated by the scope.
    operation_cancellation_source: CancellationToken,

    /// The set of active AMQP links associated with the connection scope.  These are considered
    /// children of the active connection and should be managed as such.
    active_links: HashMap<Value, Interval>,

    /// The unique identifier of the scope.
    id: String,

    /// The endpoint for the Service Bus service to which the scope is associated.
    service_endpoint: Url,

    /// <summary>
    ///   The endpoint for the Service Bus service to be used when establishing the connection.
    /// </summary>
    ///
    connection_endpoint: Url,

    /// The provider to use for obtaining a token for authorization with the Service Bus service.
    cbs_token_provider: CbsTokenProvider<TC>,

    /// The type of transport to use for communication.
    transport_type: ServiceBusTransportType,

    // /// <summary>
    // ///   The proxy, if any, which should be used for communication.
    // /// </summary>
    // private IWebProxy Proxy { get; }

    //
    /// A handle to the AMQP connection that is active for the current scope.
    connection: AmqpConnection,

    /// A handle to the AMQP session that is active for the current connection
    ///
    /// TODO: a single session?
    session: AmqpSession,

    /// The controller responsible for managing transactions.
    transaction_controller: Controller,

    /// CBS client
    cbs_client: CbsClient,
}

impl<TC: TokenCredential + std::fmt::Debug> std::fmt::Debug for AmqpConnectionScope<TC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AmqpConnectionScope")
            // .field("random_number_generator", &self.random_number_generator)
            // .field("is_disposed", &self.is_disposed)
            // .field(
            //     "operation_cancellation_source",
            //     &self.operation_cancellation_source,
            // )
            // .field("active_links", &self.active_links)
            // .field("id", &self.id)
            // .field("service_endpoint", &self.service_endpoint)
            // .field("connection_endpoint", &self.connection_endpoint)
            // .field("cbs_token_provider", &self.cbs_token_provider)
            // .field("transport_type", &self.transport_type)
            // .field("connection", &self.connection_handle)
            // .field("session_handle", &self.session_handle)
            // .field("transaction_controller", &self.transaction_controller)
            // .field("cbs_client", &"CbsClient")
            .finish()
    }
}

impl<TC: TokenCredential> AmqpConnectionScope<TC> {
    /// The name to assign to the SASL handler to specify that CBS tokens are in use.
    const CBS_SASL_HANDLER_NAME: &'static str = "MSSBCBS";

    /// The suffix to attach to the resource path when using web sockets for service communication.
    const WEB_SOCKETS_PATH_SUFFIX: &'static str = "/$servicebus/websocket/";

    /// The URI scheme to apply when using web sockets for service communication.
    const WEB_SOCKETS_URI_SCHEME: &'static str = "wss";

    /// The version of AMQP to use within the scope.
    const AMQP_VERSION_MAJOR: u8 = MAJOR;
    const AMQP_VERSION_MINOR: u8 = MINOR;
    const AMQP_VERSION_REVISION: u8 = REVISION;

    /// The amount of time to allow an AMQP connection to be idle before considering
    /// it to be timed out.
    const CONNECTION_IDLE_TIMEOUT: StdDuration = StdDuration::from_secs(1 * 60);

    /// The amount of buffer to apply to account for clock skew when
    /// refreshing authorization.  Authorization will be refreshed earlier
    /// than the expected expiration by this amount.
    const AUTHORIZATION_REFRESH_BUFFER: StdDuration =
        StdDuration::from_secs(AUTHORIZATION_REFRESH_BUFFER_SECONDS); // 7 mins

    /// The amount of seconds to use as the basis for calculating a random jitter amount
    /// when refreshing token authorization.  This is intended to ensure that multiple
    /// resources using the authorization do not all attempt to refresh at the same moment.
    const AUTHORIZATION_BASE_JITTER_SECONDS: u64 = 30;

    /// The minimum amount of time for authorization to be refreshed; any calculations that
    /// call for refreshing more frequently will be substituted with this value.
    const MINIMUM_AUTHORIZATION_REFRESH: StdDuration = StdDuration::from_secs(3 * 60);

    /// The maximum amount of time to allow before authorization is refreshed; any calculations
    /// that call for refreshing less frequently will be substituted with this value.
    ///
    /// # Remarks
    ///
    /// This value must be less than 49 days, 17 hours, 2 minutes, 47 seconds, 294 milliseconds
    /// in order to not overflow the Timer used to track authorization refresh.
    const MAXIMUM_AUTHORIZATION_REFRESH: StdDuration = StdDuration::from_secs(49 * 24 * 60 * 60); // 49 days

    /// The amount time to allow to refresh authorization of an AMQP link.
    const AUTHORIZATION_REFRESH_TIMEOUT: StdDuration = StdDuration::from_secs(3 * 60); // 3 mins

    /// The amount of buffer to apply when considering an authorization token
    /// to be expired.  The token's actual expiration will be decreased by this
    /// amount, ensuring that it is renewed before it has expired.
    const AUTHORIZATION_TOKEN_EXPIRATION_BUFFER: TimeSpan =
        TimeSpan::seconds(AUTHORIZATION_REFRESH_BUFFER_SECONDS as i64 + 2 * 60);

    pub fn service_endpoint(&self) -> &Url {
        &self.service_endpoint
    }
}

impl<TC> AmqpConnectionScope<TC>
where
    TC: TokenCredential + 'static,
{
    pub(crate) fn transport_type(&self) -> &ServiceBusTransportType {
        &self.transport_type
    }

    async fn negotiate_claim(&mut self) -> Result<(), ()> {
        todo!()
    }

    async fn begin_amqp_session(&mut self) -> Result<SessionHandle<()>, ()> {
        todo!()
    }

    /// Initializes a new instance of the <see cref="AmqpConnectionScope"/> class.
    ///
    /// # Arguments
    ///
    /// * `service_endpoint` - Endpoint for the Service Bus service to which the scope is
    ///   associated.
    /// * `connection_endpoint` - The endpoint to use for the initial connection to the Service Bus
    ///   service.
    /// * `credential` - The credential to use for authorization with the Service Bus service.
    /// * `transport_type` - The transport to use for communication.
    /// * `use_single_session` - If true, all links will use a single session.
    /// * `operation_timeout` - The timeout for operations associated with the connection.
    /// * `metrics` - The metrics instance to populate transport metrics. May be null.
    pub async fn new(
        service_endpoint: Url,
        connection_endpoint: Url, // FIXME: this will be the same as service_endpoint if a custom endpoint is not supplied
        credential: ServiceBusTokenCredential<TC>,
        transport_type: ServiceBusTransportType,
        // use_single_session: bool,
        operation_timeout: StdDuration,
        // metrics: Option<ServiceBusTransportMetrics>, // TODO: implement metrics
    ) -> Result<Self, AmqpConnectionScopeError> {
        // `Guid` from dotnet:
        // This is a convenient static method that you can call to get a new Guid. The method
        // creates a Version 4 Universally Unique Identifier (UUID) as described in RFC 4122, Sec.
        // 4.4. The returned Guid is guaranteed to not equal Guid.Empty.
        let uuid = uuid::Uuid::new_v4();
        let id = format!("{}-{}", service_endpoint, &uuid.to_string()[0..8]);
        let operation_cancellation_source = CancellationToken::new();
        let cbs_token_provider =
            CbsTokenProvider::new(credential, Self::AUTHORIZATION_TOKEN_EXPIRATION_BUFFER);

        let fut = Self::open_connection(&connection_endpoint, &transport_type, &id);
        let connection_handle = timeout(operation_timeout, fut).await??;
        let mut connection = AmqpConnection::new(connection_handle);

        // TODO: should timeout account for time used previously?
        let session_handle =
            timeout(operation_timeout, Session::begin(&mut connection.handle)).await??;
        let mut session = AmqpSession::new(session_handle);

        let transaction_controller = timeout(
            operation_timeout,
            Self::attach_txn_controller(&mut session.handle, &id),
        )
        .await??;

        let cbs_client = CbsClient::attach(&mut session.handle)
            .await
            .map_err(|err| match err {
                fe2o3_amqp_management::error::AttachError::Sender(err) => {
                    AmqpConnectionScopeError::SenderAttach(err)
                }
                fe2o3_amqp_management::error::AttachError::Receiver(err) => {
                    AmqpConnectionScopeError::ReceiverAttach(err)
                }
            })?;

        let rng = rand::thread_rng();
        let rng = StdRng::from_rng(rng)?;

        Ok(Self {
            random_number_generator: rng,
            is_disposed: false,
            operation_cancellation_source,
            active_links: HashMap::new(),
            id,
            service_endpoint,
            connection_endpoint,
            cbs_token_provider,
            transport_type,
            connection,
            session,
            transaction_controller,
            cbs_client,
        })
    }

    async fn open_connection(
        // service_endpoint: Url,
        connection_endpoint: &Url,
        transport_type: &ServiceBusTransportType,
        scope_identifier: &str,
        // timeout: TimeSpan, // FIXME: do timeout outside?
    ) -> Result<ConnectionHandle<()>, AmqpConnectionScopeError> {
        // This is the `hostname` field in the `Open` frame
        // let service_host_name = service_endpoint.host_str();
        // This is what will be used for Tcp/Tls or Ws/Wss connection
        // let connection_host_name = connection_endpoint.host_str();

        let idle_time_out = Self::CONNECTION_IDLE_TIMEOUT.as_millis() as u32; // FIXME: bound check?
        let max_frame_size = amqp_constants::DEFAULT_MAX_FRAME_SIZE;
        let container_id = scope_identifier;

        let connection_builder = Connection::builder()
            .container_id(container_id)
            .alt_tls_establishment(true)
            .sasl_profile(SaslProfile::Anonymous)
            .max_frame_size(max_frame_size)
            .idle_time_out(idle_time_out);
        let connection = match transport_type {
            ServiceBusTransportType::AmqpTcp => {
                connection_builder.open(connection_endpoint.clone()).await?
            }
            ServiceBusTransportType::AmqpWebSockets => {
                let (ws_stream, _) = WebSocketStream::connect(connection_endpoint).await?;
                connection_builder.open_with_stream(ws_stream).await?
            }
        };
        Ok(connection)
    }

    async fn attach_txn_controller(
        session: &mut SessionHandle<()>,
        scope_identifier: &str,
    ) -> Result<Controller, AmqpConnectionScopeError> {
        let controller_id = format!("{}-txn-controller", scope_identifier);
        Controller::attach(session, controller_id)
            .await
            .map_err(Into::into)
    }

    async fn request_authorization_using_cbs(
        &mut self,
        endpoint: &str,
        audience: &[&str],
        required_claims: &[&str],
    ) -> Result<Option<OffsetDateTime>, CbsAuthError> {
        let mut cbs_token_expires_at_utc = None;

        for resource in audience {
            let token = self
                .cbs_token_provider
                .get_token_async(endpoint, resource, required_claims)
                .await?;

            // find the smallest timeout
            let expires_at = match token.expires_at_utc() {
                Some(timestamp) => Some(OffsetDateTime::from(timestamp.clone())),
                None => None,
            };

            match (cbs_token_expires_at_utc, expires_at) {
                (Some(existing), Some(new)) => {
                    if new < existing {
                        cbs_token_expires_at_utc = Some(new);
                    }
                }
                (None, Some(new)) => {
                    cbs_token_expires_at_utc = Some(new);
                }
                _ => {}
            }

            self.cbs_client.put_token(*resource, token).await?;
        }

        Ok(cbs_token_expires_at_utc)
    }

    pub(crate) async fn open_management_link(
        &mut self,
        entity_path: &str,
        _identifier: &str,
    ) -> Result<MgmtClient, OpenMgmtLinkError> {
        if self.is_disposed {
            return Err(OpenMgmtLinkError::ScopeIsDisposed);
        }

        // TODO: customize mgmt-link properties?
        let entity_path = format!("{}/{}", entity_path, MANAGEMENT_ADDRESS);

        let required_claims = vec![
            service_bus_claim::MANAGE,
            service_bus_claim::LISTEN,
            service_bus_claim::SEND,
        ];
        let endpoint = format!("{}/{}", self.service_endpoint, entity_path);
        let audience = vec![&endpoint[..]];
        let _auth_expiration_utc = self
            .request_authorization_using_cbs(&endpoint, &audience, &required_claims)
            .await?;

        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        let link_name = format!(
            "{};{}:{}:{}",
            self.id, self.connection.identifier, self.session.identifier, link_identifier
        );
        let mgmt_link = MgmtClient::builder()
            .client_node_addr(link_name)
            .management_node_address(entity_path)
            .attach(&mut self.session.handle)
            .await?;
        Ok(mgmt_link)
    }

    pub(crate) async fn open_sender_link(
        &mut self,
        entity_path: &str,
        identifier: &str,
    ) -> Result<(u32, String, fe2o3_amqp::Sender), OpenSenderError> {
        if self.is_disposed {
            return Err(OpenSenderError::ScopeIsDisposed);
        }

        let endpoint = format!("{}/{}", self.service_endpoint, entity_path);
        let audience = vec![&endpoint[..]];
        let required_claims = vec![service_bus_claim::SEND];

        // TODO: what to do about auto-renewal?
        let _auth_expiration_utc = self
            .request_authorization_using_cbs(&endpoint, &audience, &required_claims)
            .await?;
        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        let link_name = format!(
            "{};{}:{}:{}",
            self.id, self.connection.identifier, self.session.identifier, link_identifier
        );
        let sender = fe2o3_amqp::Sender::builder()
            .name(&link_name)
            .source(identifier)
            .target(endpoint)
            .attach(&mut self.session.handle)
            .await?;
        Ok((link_identifier, link_name, sender))
    }

    pub(crate) async fn open_receiver_link(
        &mut self,
        entity_path: &str,
        identifier: &str,
        receive_mode: &ServiceBusReceiveMode,
        session_id: Option<String>,
        prefetch_count: u32,
    ) -> Result<(u32, String, fe2o3_amqp::Receiver), OpenReceiverError> {
        if self.is_disposed {
            return Err(OpenReceiverError::ScopeIsDisposed);
        }

        let endpoint = format!("{}/{}", self.service_endpoint, entity_path);
        let audience = vec![&endpoint[..]];
        let required_claims = vec![service_bus_claim::SEND];

        let _auth_expiration_utc = self
            .request_authorization_using_cbs(&endpoint, &audience, &required_claims)
            .await?;

        let filter_set = match session_id {
            Some(session_id) => {
                let mut filter_set = FilterSet::with_capacity(1);
                filter_set.insert(
                    Symbol::from(amqp_client_constants::SESSION_FILTER_NAME),
                    SessionFilter(session_id).into(),
                );
                filter_set
            }
            None => FilterSet::with_capacity(0),
        };

        // linkSettings.LinkName = $"{connection.Settings.ContainerId};{connection.Identifier}:{session.Identifier}:{link.Identifier}:{linkSettings.Source.ToString()}";
        // connection container id is the scope identifier
        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        let source = Source::builder()
            .address(endpoint)
            .filter(filter_set) // TODO: regular receiver link has an empty filter
            .build();
        let (snd_settle_mode, rcv_settle_mode) = service_bus_receive_mode_to_amqp(receive_mode);
        let link_name = format!(
            "{};{}:{}:{}:{:?}",
            self.id, self.connection.identifier, self.session.identifier, link_identifier, source
        );

        let mut builder = fe2o3_amqp::Receiver::builder()
            .name(&link_name)
            .source(source)
            .target(identifier);

        if let Some(snd_settle_mode) = snd_settle_mode {
            builder = builder.sender_settle_mode(snd_settle_mode);
        }
        if let Some(rcv_settle_mode) = rcv_settle_mode {
            builder = builder.receiver_settle_mode(rcv_settle_mode);
        }
        if prefetch_count > 0 {
            builder = builder.credit_mode(CreditMode::Auto(prefetch_count));
        } else {
            builder = builder.credit_mode(CreditMode::Manual);
        }

        let receiver = builder.attach(&mut self.session.handle).await?;
        Ok((link_identifier, link_name, receiver))
    }
}

// Reference:
// https://github.com/Azure/azure-amqp/blob/c6242a5dad1a1638dfee53282e08c8440913e8f7/src/AmqpLinkSettings.cs#L88
fn service_bus_receive_mode_to_amqp(
    mode: &ServiceBusReceiveMode,
) -> (Option<SenderSettleMode>, Option<ReceiverSettleMode>) {
    // switch (value)
    // {
    //     case SettleMode.SettleOnSend:
    //         this.SndSettleMode = (byte)SenderSettleMode.Settled;
    //         break;
    //     case SettleMode.SettleOnReceive:
    //         break;
    //     case SettleMode.SettleOnDispose:
    //         this.RcvSettleMode = (byte)ReceiverSettleMode.Second;
    //         break;
    // }
    match mode {
        // SettleOnDispose
        ServiceBusReceiveMode::PeekLock => (None, Some(ReceiverSettleMode::Second)),
        // SettleOnSend
        ServiceBusReceiveMode::ReceiveAndDelete => (Some(SenderSettleMode::Settled), None),
    }
}

#[async_trait]
impl<TC: TokenCredential> TransportConnectionScope for AmqpConnectionScope<TC> {
    type Error = DisposeError;

    fn is_disposed(&self) -> bool {
        self.is_disposed
    }

    fn set_is_disposed(&mut self, value: bool) {
        self.is_disposed = value;
    }

    async fn dispose(&mut self) -> Result<(), Self::Error> {
        // TODO: handle link close?

        self.session.handle.close().await?;
        self.connection.handle.close().await?;
        Ok(())
    }
}
