use std::{time::Duration as StdDuration, sync::Arc};

use fe2o3_amqp::{Connection, sasl_profile::SaslProfile, connection::ConnectionHandle};
use fe2o3_amqp_ws::WebSocketStream;
use url::Url;

use crate::{event_hubs_transport_type::EventHubsTransportType, amqp::amqp_constants, authorization::event_hub_token_credential::EventHubTokenCredential};

use super::{amqp_connection::AmqpConnection, error::AmqpConnectionScopeError};

pub(crate) struct AmqpConnectionScope {
        /// The recommended timeout to associate with an AMQP session.  It is recommended that this
        /// interval be used when creating or opening AMQP links and related constructs.
        pub(crate) session_timeout: StdDuration,

        /// The amount of time to allow a connection to have no observed traffic before considering it idle.
        pub(crate) connection_idle_timeout: StdDuration,

        /// Indicates whether this <see cref="AmqpConnectionScope"/> has been disposed.
        pub(crate) is_disposed: bool,

        // /// <summary>
        // ///   The cancellation token to use with operations initiated by the scope.
        // /// </summary>
        // ///
        // private CancellationTokenSource OperationCancellationSource { get; } = new CancellationTokenSource();

        // /// <summary>
        // ///   The set of active AMQP links associated with the connection scope.  These are considered children
        // ///   of the active connection and should be managed as such.
        // /// </summary>
        // ///
        // private ConcurrentDictionary<AmqpObject, Timer> ActiveLinks { get; } = new ConcurrentDictionary<AmqpObject, Timer>();

        /// The unique identifier of the scope.
        pub(crate) id: String,

        /// The endpoint for the Event Hubs service to which the scope is associated.
        pub(crate) service_endpoint: Url,

        /// The endpoint to used establishing a connection to the Event Hubs service to which the scope is associated.
        pub(crate) connection_endpoint: Url,

        /// The name of the Event Hub to which the scope is associated.
        pub(crate) event_hub_name: String,

        // ///   The provider to use for obtaining a token for authorization with the Event Hubs service.
        // private CbsTokenProvider TokenProvider { get; }

        /// The type of transport to use for communication.
        pub(crate) transport: EventHubsTransportType,

        // /// <summary>
        // ///   The proxy, if any, which should be used for communication.
        // /// </summary>
        // ///
        // private IWebProxy Proxy { get; }

        /// The size of the buffer used for sending information via the active transport.
        pub(crate) send_buffer_size_in_bytes: usize,

        /// The size of the buffer used for receiving information via the active transport.
        pub(crate) receive_buffer_size_in_bytes: usize,

        // /// <summary>
        // ///   A <see cref="RemoteCertificateValidationCallback" /> delegate allowing custom logic to be considered for
        // ///   validation of the remote certificate responsible for encrypting communication.
        // /// </summary>
        // ///
        // private RemoteCertificateValidationCallback CertificateValidationCallback { get; }

        /// <summary>
        ///   The AMQP connection that is active for the current scope.
        /// </summary>
        ///
        // private FaultTolerantAmqpObject<AmqpConnection> ActiveConnection { get; }\
        pub(crate) active_connection: AmqpConnection,
}

impl AmqpConnectionScope {
    const CONNECTION_IDLE_TIMEOUT: StdDuration = StdDuration::from_secs(60);
    const AUTHORIZATION_REFRESH_TIMEOUT: StdDuration = StdDuration::from_secs(60 * 7);

    pub(crate) async fn new(
        service_endpoint: &Url,
        connection_endpoint: Url,
        event_hub_name: String,
        credential: EventHubTokenCredential,
        transport_type: EventHubsTransportType,
        operation_timeout: StdDuration,
        identifier: Option<String>
    ) -> Result<Self, AmqpConnectionScopeError> {
        // sendBufferSizeInBytes and receiveBufferSizeInBytes are not used for now. They probably
        // translate to `tokio::net::TcpSocket::set_send_buffer_size` and
        // `tokio::net::TcpSocket::set_recv_buffer_size` for the TCP transport, and to
        // `tungstenite::WebSocketConfig::max_message_size` or `max_frame_size` for the WebSocket
        // transport.
        let id = identifier.unwrap_or_else(|| {
            let uuid = uuid::Uuid::new_v4();
            format!("{}-{}", service_endpoint, &uuid.to_string()[0..8])
        });
        let credential = Arc::new(credential);

        let fut = Self::open_connection(service_endpoint, &connection_endpoint, transport_type, &id);
        let connection_handle = crate::util::time::timeout(operation_timeout, fut).await??;
        let mut connection = AmqpConnection::new(connection_handle);

        todo!()
    }

    async fn open_connection(
        service_endpoint: &Url,
        connection_endpoint: &Url,
        transport_type: EventHubsTransportType,
        id: &str,
    ) -> Result<ConnectionHandle<()>, AmqpConnectionScopeError> {
        let idle_time_out = Self::CONNECTION_IDLE_TIMEOUT.as_millis() as u32;
        let max_frame_size = amqp_constants::DEFAULT_MAX_FRAME_SIZE;
        let container_id = id;

        let connection_builder = Connection::builder()
            .container_id(container_id)
            .hostname(service_endpoint.host_str())
            .alt_tls_establishment(true)
            .sasl_profile(SaslProfile::Anonymous)
            .max_frame_size(max_frame_size)
            .idle_time_out(idle_time_out);

        match transport_type {
            #[cfg(not(target_arch = "wasm32"))]
            EventHubsTransportType::AmqpTcp => connection_builder
                .open(connection_endpoint.clone())
                .await
                .map_err(Into::into),
            EventHubsTransportType::AmqpWebSockets => {
                let ws_stream = WebSocketStream::connect(connection_endpoint).await?;

                #[cfg(not(target_arch = "wasm32"))]
                let result = connection_builder
                    .open_with_stream(ws_stream)
                    .await
                    .map_err(Into::into);
                #[cfg(target_arch = "wasm32")]
                let result = connection_builder
                    .open_with_stream_on_current_local_set(ws_stream)
                    .await
                    .map_err(Into::into);

                result
            },
        }
    }
}
