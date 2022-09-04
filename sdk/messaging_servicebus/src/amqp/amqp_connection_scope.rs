use std::time::Duration;

use async_trait::async_trait;
use azure_core::Url;
use fe2o3_amqp::{
    connection::ConnectionHandle,
    session::SessionHandle,
    transaction::Controller,
    transport::protocol_header::{ProtocolHeader, ProtocolId},
};
use fe2o3_amqp_types::definitions::{MAJOR, MINOR, REVISION};

use crate::{
    core::TransportConnectionScope, primitives::service_bus_transport_type::ServiceBusTransportType,
};

use super::cbs_token_provider::CbsTokenProvider;

const AUTHORIZATION_REFRESH_BUFFER_SECONDS: u64 = 7 * 60;

#[derive(Debug)]
pub(crate) struct AmqpConnectionScope {
    /// <summary>The seed to use for initializing random number generated for a given thread-specific instance.</summary>
    // private static int s_randomSeed = Environment.TickCount;

    /// <summary>The random number generator to use for a specific thread.</summary>
    // private static readonly ThreadLocal<Random> RandomNumberGenerator = new ThreadLocal<Random>(() => new Random(Interlocked.Increment(ref s_randomSeed)), false);

    /// <summary>Indicates whether or not this instance has been disposed.</summary>
    disposed: bool,

    // /// <summary>
    // ///   The cancellation token to use with operations initiated by the scope.
    // /// </summary>
    // private CancellationTokenSource OperationCancellationSource { get; } = new();

    //
    /// The unique identifier of the scope.
    id: String,

    /// The endpoint for the Service Bus service to which the scope is associated.
    service_endpoint: Url,

    /// The provider to use for obtaining a token for authorization with the Service Bus service.
    cbs_token_provider: CbsTokenProvider,

    /// The type of transport to use for communication.
    transport: ServiceBusTransportType,

    // /// <summary>
    // ///   The proxy, if any, which should be used for communication.
    // /// </summary>
    // private IWebProxy Proxy { get; }

    //
    /// A handle to the AMQP connection that is active for the current scope.
    connection_handle: ConnectionHandle<()>,

    /// A handle to the AMQP session that is active for the current connection
    session_handle: SessionHandle<()>,

    /// The controller responsible for managing transactions.
    transaction_controller: Controller,
}

impl AmqpConnectionScope {
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
    const CONNECTION_IDLE_TIMEOUT: Duration = Duration::from_secs(1 * 60);

    /// The amount of buffer to apply to account for clock skew when
    /// refreshing authorization.  Authorization will be refreshed earlier
    /// than the expected expiration by this amount.
    const AUTHORIZATION_REFRESH_BUFFER: Duration =
        Duration::from_secs(AUTHORIZATION_REFRESH_BUFFER_SECONDS); // 7 mins

    /// The amount of seconds to use as the basis for calculating a random jitter amount
    /// when refreshing token authorization.  This is intended to ensure that multiple
    /// resources using the authorization do not all attempt to refresh at the same moment.
    const AUTHORIZATION_BASE_JITTER_SECONDS: u64 = 30;

    /// The minimum amount of time for authorization to be refreshed; any calculations that
    /// call for refreshing more frequently will be substituted with this value.
    const MINIMUM_AUTHORIZATION_REFRESH: Duration = Duration::from_secs(3 * 60);

    /// The maximum amount of time to allow before authorization is refreshed; any calculations
    /// that call for refreshing less frequently will be substituted with this value.
    ///
    /// # Remarks
    ///
    /// This value must be less than 49 days, 17 hours, 2 minutes, 47 seconds, 294 milliseconds
    /// in order to not overflow the Timer used to track authorization refresh.
    const MAXIMUM_AUTHORIZATION_REFRESH: Duration = Duration::from_secs(49 * 24 * 60 * 60); // 49 days

    /// The amount time to allow to refresh authorization of an AMQP link.
    const AUTHORIZATION_REFRESH_TIMEOUT: Duration = Duration::from_secs(3 * 60); // 3 mins

    /// The amount of buffer to apply when considering an authorization token
    /// to be expired.  The token's actual expiration will be decreased by this
    /// amount, ensuring that it is renewed before it has expired.
    const AUTHORIZATION_TOKEN_EXPIRATION_BUFFER: Duration =
        Duration::from_secs(AUTHORIZATION_REFRESH_BUFFER_SECONDS + 2 * 60);
}

impl AmqpConnectionScope {
    async fn negotiate_claim(&mut self) -> Result<(), ()> {
        todo!()
    }

    async fn begin_amqp_session(&mut self) -> Result<SessionHandle<()>, ()> {
        todo!()
    }
}

#[async_trait]
impl TransportConnectionScope for AmqpConnectionScope {
    fn is_disposed(&self) -> bool {
        self.disposed
    }

    fn set_is_disposed(&mut self, value: bool) {
        self.disposed = value;
    }

    async fn dispose(&mut self) {
        todo!()
    }
}
