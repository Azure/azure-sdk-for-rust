use std::{time::Duration as StdDuration, sync::Arc};

use fe2o3_amqp::{Connection, sasl_profile::SaslProfile, connection::ConnectionHandle, session::SessionHandle, Sender, Session};
use fe2o3_amqp_cbs::client::CbsClient;
use fe2o3_amqp_types::definitions::Milliseconds;
use fe2o3_amqp_ws::WebSocketStream;
use url::Url;
use time::Duration as TimeSpan;

use crate::{event_hubs_transport_type::EventHubsTransportType, amqp::{amqp_constants, amqp_cbs_link::AmqpCbsLink}, authorization::event_hub_token_credential::EventHubTokenCredential, core::transport_producer_features::TransportProducerFeatures, producer::PartitionPublishingOptions};

use super::{amqp_connection::AmqpConnection, error::{AmqpConnectionScopeError, OpenProducerError}, amqp_cbs_link::AmqpCbsLinkHandle, cbs_token_provider::CbsTokenProvider};

const AUTHORIZATION_REFRESH_BUFFER_SECONDS: u64 = 7 * 60;

pub(crate) struct AmqpConnectionScope {
    // /// The recommended timeout to associate with an AMQP session.  It is recommended that this
    // /// interval be used when creating or opening AMQP links and related constructs.
    // pub(crate) session_timeout: StdDuration,

    // /// The amount of time to allow a connection to have no observed traffic before considering it idle.
    // pub(crate) connection_idle_timeout_millis: Milliseconds,

    /// Indicates whether this <see cref="AmqpConnectionScope"/> has been disposed.
    pub(crate) is_disposed: bool,

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

    // /// The size of the buffer used for sending information via the active transport.
    // pub(crate) send_buffer_size_in_bytes: usize,

    // /// The size of the buffer used for receiving information via the active transport.
    // pub(crate) receive_buffer_size_in_bytes: usize,

    /// The session dedicated for cbs auth
    pub(crate) cbs_session_handle: SessionHandle<()>,

    /// CBS link for auth
    pub(crate) cbs_link_handle: AmqpCbsLinkHandle,
}

impl AmqpConnectionScope {
    const CONNECTION_IDLE_TIMEOUT: StdDuration = StdDuration::from_secs(60);
    const AUTHORIZATION_REFRESH_TIMEOUT: StdDuration = StdDuration::from_secs(60 * 7);
    /// The amount of buffer to apply when considering an authorization token
    /// to be expired.  The token's actual expiration will be decreased by this
    /// amount, ensuring that it is renewed before it has expired.
    const AUTHORIZATION_TOKEN_EXPIRATION_BUFFER: TimeSpan =
        TimeSpan::seconds(AUTHORIZATION_REFRESH_BUFFER_SECONDS as i64 + 2 * 60);

    pub(crate) async fn new(
        service_endpoint: Url,
        connection_endpoint: Url,
        event_hub_name: String,
        credential: EventHubTokenCredential,
        transport_type: EventHubsTransportType,
        idle_timeout: StdDuration,
        identifier: Option<String>
    ) -> Result<Self, AmqpConnectionScopeError> {
        // sendBufferSizeInBytes and receiveBufferSizeInBytes are not used for now. They probably
        // translate to `tokio::net::TcpSocket::set_send_buffer_size` and
        // `tokio::net::TcpSocket::set_recv_buffer_size` for the TCP transport, and to
        // `tungstenite::WebSocketConfig::max_message_size` or `max_frame_size` for the WebSocket
        // transport.

        // Id = identifier ?? $"{ eventHubName }-{ Guid.NewGuid().ToString("D", CultureInfo.InvariantCulture).Substring(0, 8) }";
        let id = identifier.unwrap_or_else(|| {
            let uuid = uuid::Uuid::new_v4();
            format!("{}-{}", event_hub_name, &uuid.to_string()[0..8])
        });
        let credential = Arc::new(credential);

        let fut = Self::open_connection(&service_endpoint, &connection_endpoint, transport_type, &id, idle_timeout.as_millis() as u32);
        let connection_handle = crate::util::time::timeout(idle_timeout, fut).await??;
        let mut connection = AmqpConnection::new(connection_handle);

        let mut cbs_session_handle = Session::begin(&mut connection.handle).await?;

        let cbs_client = attach_cbs_client(&mut cbs_session_handle).await?;
        let cbs_token_provider = CbsTokenProvider::new(
            credential.clone(),
            Self::AUTHORIZATION_TOKEN_EXPIRATION_BUFFER,
        );
        let cbs_link_handle = AmqpCbsLink::spawn(cbs_token_provider, cbs_client);

        Ok(Self {
            is_disposed: false,
            id,
            service_endpoint,
            connection_endpoint,
            event_hub_name,
            transport: transport_type,
            cbs_session_handle,
            cbs_link_handle,
        })
    }

    async fn open_connection(
        service_endpoint: &Url,
        connection_endpoint: &Url,
        transport_type: EventHubsTransportType,
        id: &str,
        idle_timeout_millis: Milliseconds,
    ) -> Result<ConnectionHandle<()>, AmqpConnectionScopeError> {
        let max_frame_size = amqp_constants::DEFAULT_MAX_FRAME_SIZE;
        let container_id = id;

        let connection_builder = Connection::builder()
            .container_id(container_id)
            .hostname(service_endpoint.host_str())
            .alt_tls_establishment(true)
            .sasl_profile(SaslProfile::Anonymous)
            .max_frame_size(max_frame_size)
            .idle_time_out(idle_timeout_millis);

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

    async fn create_sender_session_and_link(
        &mut self,
        endpoint: Url,
        features: TransportProducerFeatures,
        options: PartitionPublishingOptions,
        link_identifier: String,
    ) -> Result<(SessionHandle<()>, Sender), OpenProducerError> {
        todo!()
    }
}

async fn attach_cbs_client(
    session: &mut SessionHandle<()>,
) -> Result<CbsClient, AmqpConnectionScopeError> {
    CbsClient::attach(session).await.map_err(|err| match err {
        fe2o3_amqp_management::error::AttachError::Sender(err) => {
            AmqpConnectionScopeError::SenderAttach(err)
        }
        fe2o3_amqp_management::error::AttachError::Receiver(err) => {
            AmqpConnectionScopeError::ReceiverAttach(err)
        }
    })
}
