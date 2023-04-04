use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration as StdDuration,
};

use fe2o3_amqp::{
    connection::ConnectionHandle, link::receiver::CreditMode, sasl_profile::SaslProfile,
    session::SessionHandle, Connection, Receiver, Sender, Session,
};
use fe2o3_amqp_cbs::client::CbsClient;
use fe2o3_amqp_management::MgmtClient;
use fe2o3_amqp_types::{
    definitions::{Milliseconds, ReceiverSettleMode},
    messaging::Source,
    primitives::{OrderedMap, Symbol},
};
use fe2o3_amqp_ws::WebSocketStream;
use serde_amqp::Value;
use time::Duration as TimeSpan;
use url::Url;

use crate::{
    amqp::{
        amqp_cbs_link::AmqpCbsLink,
        amqp_constants,
        amqp_filter::{self, ConsumerFilter},
        LINK_IDENTIFIER, SESSION_IDENTIFIER,
    },
    authorization::{event_hub_claim, event_hub_token_credential::EventHubTokenCredential},
    consumer::EventPosition,
    core::transport_producer_features::TransportProducerFeatures,
    event_hubs_transport_type::EventHubsTransportType,
    producer::PartitionPublishingOptions,
};

use super::{
    amqp_cbs_link::AmqpCbsLinkHandle,
    amqp_connection::AmqpConnection,
    amqp_consumer::AmqpConsumer,
    amqp_management_link::AmqpManagementLink,
    amqp_producer::AmqpProducer,
    amqp_property,
    cbs_token_provider::CbsTokenProvider,
    error::{
        AmqpConnectionScopeError, CbsAuthError, DisposeError, OpenConsumerError, OpenMgmtLinkError,
        OpenProducerError,
    },
};

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

    // Keep a copy of credential for recovery
    pub(crate) credential: Arc<EventHubTokenCredential>,

    // /// The size of the buffer used for sending information via the active transport.
    // pub(crate) send_buffer_size_in_bytes: usize,

    // /// The size of the buffer used for receiving information via the active transport.
    // pub(crate) receive_buffer_size_in_bytes: usize,
    pub(crate) connection: AmqpConnection,

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
        identifier: Option<String>,
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

        let fut = Self::open_connection(
            &service_endpoint,
            &connection_endpoint,
            transport_type,
            &id,
            idle_timeout.as_millis() as u32,
        );
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
            credential,
            connection,
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
            }
        }
    }

    pub(crate) async fn request_refreshable_authorization_using_cbs(
        &mut self,
        link_identifier: u32,
        endpoint: String,
        resource: String,
        required_claims: Vec<String>,
    ) -> Result<(), CbsAuthError> {
        use fe2o3_amqp::link::LinkStateError;
        use fe2o3_amqp_management::error::Error as ManagementError;

        self.cbs_link_handle
            .request_refreshable_authorization(link_identifier, endpoint, resource, required_claims)
            .await
            // TODO: The CBS event loop should never spontaneously stop
            .map_err(|_| ManagementError::Send(LinkStateError::IllegalSessionState.into()))??;
        Ok(())
    }

    pub(crate) async fn open_management_link(
        &mut self,
    ) -> Result<AmqpManagementLink, OpenMgmtLinkError> {
        self.create_management_link()
            .await
            .map(|(session_handle, client)| AmqpManagementLink {
                session_handle,
                client,
            })
    }

    async fn create_management_link(
        &mut self,
    ) -> Result<(SessionHandle<()>, MgmtClient), OpenMgmtLinkError> {
        if self.is_disposed {
            return Err(OpenMgmtLinkError::ConnectionScopeDisposed);
        }

        let mut session_handle = Session::begin(&mut self.connection.handle).await?;
        let mgmt_link = MgmtClient::attach(&mut session_handle, "").await?;

        Ok((session_handle, mgmt_link))
    }

    pub(crate) async fn open_producer_link(
        &mut self,
        partition_id: Option<String>,
        features: TransportProducerFeatures,
        options: PartitionPublishingOptions,
        identifier: Option<String>,
    ) -> Result<AmqpProducer, OpenProducerError> {
        let path = match partition_id {
            None => self.event_hub_name.clone(),
            Some(partition_id) if partition_id.is_empty() => self.event_hub_name.clone(),
            Some(partition_id) => format!("{}/Partitions/{}", self.event_hub_name, partition_id),
        };
        let producer_endpoint = self.service_endpoint.join(&path)?;

        let identifier = identifier.unwrap_or(uuid::Uuid::new_v4().to_string());
        let session_identifier = SESSION_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        let (session_handle, sender) = self
            .create_sending_session_and_link(
                producer_endpoint,
                features,
                options,
                session_identifier,
                link_identifier,
                identifier,
            )
            .await?;

        let initialized_partition_properties = sender.properties(|properties| {
            let producer_group_id = properties
                .as_ref()
                .and_then(|p| p.get(amqp_property::PRODUCER_GROUP_ID.as_str()))
                .and_then(|value| match value {
                    Value::Long(v) => Some(*v),
                    _ => None,
                });
            let owner_level = properties
                .as_ref()
                .and_then(|p| p.get(amqp_property::PRODUCER_OWNER_LEVEL.as_str()))
                .and_then(|value| match value {
                    Value::Short(v) => Some(*v),
                    _ => None,
                });
            let starting_sequence_number = properties
                .as_ref()
                .and_then(|p| p.get(amqp_property::PRODUCER_SEQUENCE_NUMBER.as_str()))
                .and_then(|value| match value {
                    Value::Int(v) => Some(*v),
                    _ => None,
                });
            PartitionPublishingOptions {
                producer_group_id,
                owner_level,
                starting_sequence_number,
            }
        });

        Ok(AmqpProducer {
            session_handle,
            session_identifier,
            sender,
            link_identifier,
            initialized_partition_properties,
        })
    }

    async fn create_sending_session_and_link(
        &mut self,
        endpoint: Url,
        features: TransportProducerFeatures,
        options: PartitionPublishingOptions,
        session_identifier: u32,
        link_identifier: u32,
        identifier: String, // Used as the source address for the link
    ) -> Result<(SessionHandle<()>, Sender), OpenProducerError> {
        if self.is_disposed {
            return Err(OpenProducerError::ConnectionScopeDisposed);
        }

        // Perform the initial authorization for the link.
        let auth_claims = vec![event_hub_claim::SEND.to_string()];
        let resource = endpoint.to_string();
        self.request_refreshable_authorization_using_cbs(
            link_identifier,
            endpoint.to_string(),
            resource,
            auth_claims,
        )
        .await?;

        // Create and open the AMQP session associated with the link.
        let mut session_handle = Session::begin(&mut self.connection.handle).await?;

        // Create and open the link.

        // linkSettings.LinkName = $"{ Id };{ connection.Identifier }:{ session.Identifier }:{ link.Identifier }";
        let link_name = format!(
            "{};{}:{}:{}",
            self.id, self.connection.identifier, session_identifier, link_identifier
        );
        let mut builder = Sender::builder()
            .name(link_name)
            .source(identifier)
            .target(endpoint);

        if let TransportProducerFeatures::IdempotentPublishing = features {
            builder = builder.add_desired_capabilities(amqp_property::ENABLE_IDEMPOTENT_PUBLISHING);
        }

        // If any of the options have a value, the entire set must be specified for the link
        // settings.  For any options that did not have a value, specifying null will signal the
        // service to generate the value.
        if options.producer_group_id.is_some()
            || options.owner_level.is_some()
            || options.starting_sequence_number.is_some()
        {
            let properties = builder.properties.get_or_insert(Default::default());
            properties.insert(
                Symbol::from(amqp_property::PRODUCER_GROUP_ID),
                options
                    .producer_group_id
                    .map(Value::from)
                    .unwrap_or(Value::Null),
            );
            properties.insert(
                Symbol::from(amqp_property::PRODUCER_OWNER_LEVEL),
                options.owner_level.map(Value::from).unwrap_or(Value::Null),
            );
            properties.insert(
                Symbol::from(amqp_property::PRODUCER_SEQUENCE_NUMBER),
                options
                    .starting_sequence_number
                    .map(Value::from)
                    .unwrap_or(Value::Null),
            );
        }

        let sender = builder.attach(&mut session_handle).await?;
        Ok((session_handle, sender))
    }

    pub(crate) async fn open_consumer_link(
        &mut self,
        consumer_group: String,
        partition_id: String,
        event_position: EventPosition,
        prefetch_count: u32,
        // prefetch_size_in_bytes: Option<usize>, // TODO: what does this do in the c# sdk?
        owner_level: Option<i64>,
        track_last_enqueued_event_properties: bool,
        identifier: Option<String>,
    ) -> Result<AmqpConsumer, OpenConsumerError> {
        let path = format!(
            "{}/ConsumerGroups/{}/Partitions/{}",
            self.event_hub_name, consumer_group, partition_id
        );
        let consumer_endpoint = self.service_endpoint.join(&path)?;
        let identifier = identifier.unwrap_or(uuid::Uuid::new_v4().to_string());
        let session_identifier = SESSION_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);

        let (session_handle, receiver) = self
            .create_receiving_session_and_link(
                consumer_endpoint,
                event_position,
                prefetch_count,
                // prefetch_size_in_bytes,
                owner_level,
                track_last_enqueued_event_properties,
                session_identifier,
                link_identifier,
                identifier,
            )
            .await?;
        Ok(AmqpConsumer {
            session_handle,
            session_identifier,
            receiver,
            link_identifier,
        })
    }

    async fn create_receiving_session_and_link(
        &mut self,
        endpoint: Url,
        event_position: EventPosition,
        prefetch_count: u32,
        // prefetch_size_in_bytes: Option<usize>, // TODO: what does this do in the c# sdk?
        owner_level: Option<i64>,
        track_last_enqueued_event_properties: bool,
        session_identifier: u32,
        link_identifier: u32,
        identifier: String,
    ) -> Result<(SessionHandle<()>, Receiver), OpenConsumerError> {
        if self.is_disposed {
            return Err(OpenConsumerError::ConnectionScopeDisposed);
        }

        // Perform the initial authorization for the link.
        let auth_claims = vec![event_hub_claim::LISTEN.to_string()];
        let resource = endpoint.to_string();
        self.request_refreshable_authorization_using_cbs(
            link_identifier,
            endpoint.to_string(),
            resource,
            auth_claims,
        )
        .await?;

        // Create and open the AMQP session associated with the link.
        let mut session_handle = Session::begin(&mut self.connection.handle).await?;

        // Create and open the link.

        // linkSettings.LinkName = $"{ Id };{ connection.Identifier }:{ session.Identifier }:{ link.Identifier }";
        let link_name = format!(
            "{};{}:{}:{}",
            self.id, self.connection.identifier, session_identifier, link_identifier
        );
        let consumer_filter = ConsumerFilter(amqp_filter::build_filter_expression(event_position)?);
        let source = Source::builder()
            .address(endpoint)
            .add_to_filter(amqp_filter::CONSUMER_FILTER_NAME, consumer_filter)
            .build();

        let mut builder = Receiver::builder()
            .name(link_name)
            .source(source)
            .target(identifier.clone())
            // TODO: Allow user to specify when to automatically re-fill the credit. This needs
            //       upstream support, see
            //       [minghuaw@fe2o3-amqp#199](https://github.com/minghuaw/fe2o3-amqp/issues/199)
            .credit_mode(CreditMode::Auto(prefetch_count));

        // `SettleMode.SettleOnSend` doesn't affect the receiver settle mode. So set it to default.
        //
        // ```csharp
        // case SettleMode.SettleOnSend:
        //     this.SndSettleMode = (byte)SenderSettleMode.Settled;
        //     break;
        // ```
        // https://github.com/Azure/azure-amqp/blob/d32534d2350a3672812928a1886e533c63aae0e3/src/AmqpLinkSettings.cs#L88
        builder = builder.receiver_settle_mode(ReceiverSettleMode::default());

        let mut properties = OrderedMap::new();
        properties.insert(
            Symbol::from(amqp_property::ENTITY_TYPE),
            Value::from(amqp_property::Entity::ConsumerGroup as i32),
        );

        if let Some(owner_level) = owner_level {
            properties.insert(
                Symbol::from(amqp_property::CONSUMER_OWNER_LEVEL),
                Value::from(owner_level),
            );
        }

        if !identifier.is_empty() {
            properties.insert(
                Symbol::from(amqp_property::CONSUMER_IDENTIFIER),
                Value::from(identifier),
            );
        }

        if track_last_enqueued_event_properties {
            builder = builder
                .add_desired_capabilities(amqp_property::TRACK_LAST_ENQUEUED_EVENT_PROPERTIES);
        }

        let receiver = builder
            .properties(properties)
            .attach(&mut session_handle)
            .await?;

        Ok((session_handle, receiver))
    }

    pub(crate) async fn dispose(&mut self) -> Result<(), DisposeError> {
        if self.is_disposed {
            return Ok(());
        }

        self.is_disposed = true;

        let _ = self.cbs_link_handle.stop();
        let _cbs_close_result = self.cbs_link_handle.join_handle_mut().await;

        let session_close_result = self.cbs_session_handle.close().await;
        let connection_close_result = self.connection.handle.close().await;
        match (session_close_result, connection_close_result) {
            (Ok(_), Ok(_)) => Ok(()),
            // Connection error has priority
            (_, Err(e)) => Err(DisposeError::ConnectionCloseError(e)),
            (Err(e), _) => Err(DisposeError::SessionCloseError(e)),
        }
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
