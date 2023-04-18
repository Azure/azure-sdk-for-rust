use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration as StdDuration,
};

use async_trait::async_trait;
use fe2o3_amqp::{
    connection::ConnectionHandle, link::receiver::CreditMode, sasl_profile::SaslProfile,
    session::SessionHandle, Connection, Receiver, Sender, Session,
};
use fe2o3_amqp_cbs::client::CbsClient;
use fe2o3_amqp_management::MgmtClient;
use fe2o3_amqp_types::{
    definitions::ReceiverSettleMode,
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
    core::{RecoverableTransport, TransportProducerFeatures},
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
        OpenProducerError, RecoverManagementLinkError,
    },
};

const AUTHORIZATION_REFRESH_BUFFER_SECONDS: u64 = 7 * 60;
const WEBSOCKETS_PATH_SUFFIX: &str = "/$servicebus/websocket/";

pub(crate) struct AmqpConnectionScope {
    /// Indicates whether this <see cref="AmqpConnectionScope"/> has been disposed.
    pub(crate) is_disposed: Arc<AtomicBool>,

    /// The unique identifier of the scope.
    pub(crate) id: String,

    /// The endpoint for the Event Hubs service to which the scope is associated.
    pub(crate) service_endpoint: Url,

    /// The endpoint to used establishing a connection to the Event Hubs service to which the scope is associated.
    pub(crate) connection_endpoint: Url,

    /// The name of the Event Hub to which the scope is associated.
    pub(crate) event_hub_name: Arc<String>,

    /// The type of transport to use for communication.
    pub(crate) transport: EventHubsTransportType,

    // Keep a copy of credential for recovery
    pub(crate) credential: Arc<EventHubTokenCredential>,

    // Keep a copy of connection_idle_timeout for recovery
    pub(crate) connection_idle_timeout: StdDuration,

    /// The AMQP connection to the Event Hubs service.
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
        event_hub_name: Arc<String>,
        credential: EventHubTokenCredential,
        transport_type: EventHubsTransportType,
        connection_idle_timeout: StdDuration,
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

        let connection_handle = Self::open_connection(
            &service_endpoint,
            &connection_endpoint,
            transport_type,
            &id,
            connection_idle_timeout,
        )
        .await?;
        let mut connection = AmqpConnection::new(connection_handle);

        let mut cbs_session_handle = Session::begin(&mut connection.handle).await?;

        let cbs_client = attach_cbs_client(&mut cbs_session_handle).await?;
        let cbs_token_provider = CbsTokenProvider::new(
            credential.clone(),
            Self::AUTHORIZATION_TOKEN_EXPIRATION_BUFFER,
        );
        let cbs_link_handle = AmqpCbsLink::spawn(cbs_token_provider, cbs_client);

        Ok(Self {
            is_disposed: Arc::new(AtomicBool::new(false)),
            id,
            service_endpoint,
            connection_endpoint,
            event_hub_name,
            transport: transport_type,
            credential,
            connection_idle_timeout,
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
        idle_timeout: StdDuration,
    ) -> Result<ConnectionHandle<()>, AmqpConnectionScopeError> {
        let max_frame_size = amqp_constants::DEFAULT_MAX_FRAME_SIZE;
        let container_id = id;

        let connection_builder = Connection::builder()
            .container_id(container_id)
            .hostname(service_endpoint.host_str())
            .alt_tls_establishment(true)
            .sasl_profile(SaslProfile::Anonymous)
            .max_frame_size(max_frame_size)
            .idle_time_out(idle_timeout.as_millis() as u32);

        match transport_type {
            #[cfg(not(target_arch = "wasm32"))]
            EventHubsTransportType::AmqpTcp => connection_builder
                .open(connection_endpoint.clone())
                .await
                .map_err(Into::into),
            EventHubsTransportType::AmqpWebSockets => {
                let addr = connection_endpoint.join(WEBSOCKETS_PATH_SUFFIX)?;
                let ws_stream = WebSocketStream::connect(addr).await?;

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
        if self.is_disposed.load(Ordering::Relaxed) {
            return Err(OpenMgmtLinkError::ConnectionScopeDisposed);
        }

        let mut session_handle = Session::begin(&mut self.connection.handle).await?;
        let mgmt_link = MgmtClient::attach(&mut session_handle, "").await?;

        Ok((session_handle, mgmt_link))
    }

    pub(crate) async fn recover_management_link(
        &mut self,
        management_link: &mut AmqpManagementLink,
    ) -> Result<(), RecoverManagementLinkError> {
        if management_link.session_handle.is_ended() {
            let new_management_session = Session::begin(&mut self.connection.handle).await?;
            management_link
                .client
                .detach_then_resume_on_session(&new_management_session)
                .await?;
            let mut old_session =
                std::mem::replace(&mut management_link.session_handle, new_management_session);
            let _ = old_session.try_end();
        }
        Ok(())
    }

    pub(crate) async fn open_producer_link<RP>(
        &mut self,
        partition_id: Option<String>,
        features: TransportProducerFeatures,
        options: PartitionPublishingOptions,
        identifier: Option<String>,
        retry_policy: RP,
    ) -> Result<AmqpProducer<RP>, OpenProducerError> {
        use std::borrow::Cow;

        let path: Cow<str> = match &partition_id {
            None => Cow::Borrowed(&self.event_hub_name),
            Some(partition_id) if partition_id.is_empty() => Cow::Borrowed(&self.event_hub_name),
            Some(partition_id) => Cow::Owned(format!(
                "{}/Partitions/{}",
                self.event_hub_name, partition_id
            )),
        };
        let producer_endpoint = self.service_endpoint.join(&path)?;

        let identifier = identifier.unwrap_or(uuid::Uuid::new_v4().to_string());
        let session_identifier = SESSION_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        let (session_handle, sender) = self
            .create_sending_session_and_link(
                &producer_endpoint,
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
            retry_policy,
            endpoint: producer_endpoint,
            cbs_command_sender: self.cbs_link_handle.command_sender().clone(),
        })
    }

    async fn create_sending_session_and_link(
        &mut self,
        endpoint: &Url,
        features: TransportProducerFeatures,
        options: PartitionPublishingOptions,
        session_identifier: u32,
        link_identifier: u32,
        identifier: String, // Used as the source address for the link
    ) -> Result<(SessionHandle<()>, Sender), OpenProducerError> {
        if self.is_disposed.load(Ordering::Relaxed) {
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
            .target(endpoint.to_string());

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

    pub(crate) async fn open_consumer_link<RP>(
        &mut self,
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        prefetch_count: u32,
        // prefetch_size_in_bytes: Option<usize>, // TODO: what does this do in the c# sdk?
        owner_level: Option<i64>,
        track_last_enqueued_event_properties: bool,
        invalidate_consumer_when_partition_stolen: bool,
        identifier: Option<String>,
        retry_policy: RP,
    ) -> Result<AmqpConsumer<RP>, OpenConsumerError> {
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
                &consumer_endpoint,
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
            invalidate_consumer_when_partition_stolen,
            track_last_enqueued_event_properties,
            current_event_position: None,
            last_received_event: None,
            retry_policy,
            prefetch_count,
            cbs_command_sender: self.cbs_link_handle.command_sender().clone(),
            endpoint: consumer_endpoint,
        })
    }

    async fn create_receiving_session_and_link(
        &mut self,
        endpoint: &Url,
        event_position: EventPosition,
        prefetch_count: u32,
        // prefetch_size_in_bytes: Option<usize>, // TODO: what does this do in the c# sdk?
        owner_level: Option<i64>,
        track_last_enqueued_event_properties: bool,
        session_identifier: u32,
        link_identifier: u32,
        identifier: String,
    ) -> Result<(SessionHandle<()>, Receiver), OpenConsumerError> {
        if self.is_disposed.load(Ordering::Relaxed) {
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
            .address(endpoint.to_string())
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

    pub(crate) async fn close(&mut self) -> Result<(), DisposeError> {
        let mut is_disposed = self.is_disposed.load(Ordering::Relaxed);
        if is_disposed || self.connection.handle.is_closed() {
            return Ok(());
        }

        loop {
            match self.is_disposed.compare_exchange_weak(
                is_disposed,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => is_disposed = x,
            }
        }

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

#[async_trait]
impl RecoverableTransport for AmqpConnectionScope {
    type RecoverError = AmqpConnectionScopeError;

    async fn recover(&mut self) -> Result<(), Self::RecoverError> {
        let is_disposed = self.is_disposed.load(Ordering::Relaxed);

        // A connection can only be disposed by the user, so a disposed
        // connection should not be auto-recovered.
        if is_disposed {
            return Err(AmqpConnectionScopeError::ScopeDisposed);
        }

        // Recover connection if it is closed
        if self.connection.handle.is_closed() {
            if let Err(err) = self.connection.handle.close().await {
                log::error!("Error closing connection during recovering: {:?}", err);
            }

            let connection_handle = Self::open_connection(
                &self.service_endpoint,
                &self.connection_endpoint,
                self.transport,
                &self.id,
                self.connection_idle_timeout,
            )
            .await?;
            self.connection = AmqpConnection::new(connection_handle);
        }

        // Recover CBS session and link if it is closed
        if self.cbs_session_handle.is_ended() {
            if let Err(err) = self.cbs_session_handle.end().await {
                log::error!("Error ending CBS session during recovering: {:?}", err);
            }
            self.cbs_session_handle = Session::begin(&mut self.connection.handle).await?;

            let cbs_client = attach_cbs_client(&mut self.cbs_session_handle).await?;
            let cbs_token_provider = CbsTokenProvider::new(
                self.credential.clone(),
                Self::AUTHORIZATION_TOKEN_EXPIRATION_BUFFER,
            );
            self.cbs_link_handle = AmqpCbsLink::spawn(cbs_token_provider, cbs_client);
        }

        Ok(())
    }
}
