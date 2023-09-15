use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration as StdDuration,
};

use async_trait::async_trait;
use azure_core::Url;
use fe2o3_amqp::{
    connection::ConnectionHandle, link::receiver::CreditMode, sasl_profile::SaslProfile,
    session::SessionHandle, Connection, Session,
};

#[cfg(feature = "transaction")]
use fe2o3_amqp::transaction::Controller;

use fe2o3_amqp_cbs::client::CbsClient;
use fe2o3_amqp_management::client::MgmtClient;
use fe2o3_amqp_types::{
    definitions::{ReceiverSettleMode, SenderSettleMode},
    messaging::{FilterSet, Source},
};
use fe2o3_amqp_ws::WebSocketStream;
use time::Duration as TimeSpan;
use tokio::sync::mpsc;

use crate::{
    authorization::{service_bus_claim, service_bus_token_credential::ServiceBusTokenCredential},
    core::{RecoverableTransport, TransportConnectionScope},
    entity_name_formatter,
    primitives::service_bus_transport_type::ServiceBusTransportType,
    sealed::Sealed,
    ServiceBusReceiveMode,
};

use super::{
    amqp_cbs_link::{self, AmqpCbsLink, AmqpCbsLinkHandle},
    amqp_client_constants::{self, MANAGEMENT_ADDRESS},
    amqp_connection::AmqpConnection,
    amqp_constants,
    amqp_management_link::AmqpManagementLink,
    amqp_session::AmqpSession,
    cbs_token_provider::CbsTokenProvider,
    error::{
        AmqpConnectionScopeError, CbsAuthError, DisposeError, OpenMgmtLinkError, OpenReceiverError,
        OpenSenderError,
    },
    session_filter::SessionFilter,
    LINK_IDENTIFIER,
};

const AUTHORIZATION_REFRESH_BUFFER_SECONDS: u64 = 7 * 60;

pub(crate) enum ReceiverType {
    NonSession,
    Session { session_id: Option<String> },
}

pub(crate) struct AmqpConnectionScope {
    /// Indicates whether or not this instance has been disposed.
    is_disposed: bool,

    /// The unique identifier of the scope.
    id: String,

    /// The endpoint for the Service Bus service to be used when establishing the connection.
    connection_endpoint: Url,

    /// The type of transport to use for communication.
    transport_type: ServiceBusTransportType,

    /// A handle to the AMQP connection that is active for the current scope.
    connection: AmqpConnection,

    /// A handle to the AMQP session that is active for the current connection
    pub(crate) session: AmqpSession,

    /// CBS client
    pub(crate) cbs_link: AmqpCbsLinkHandle,

    recover_operation_timeout: StdDuration,

    // Keep a copy for recovering
    credential: Arc<ServiceBusTokenCredential>,

    /// The controller responsible for managing transactions.
    ///
    /// TODO: transactions?
    #[cfg(feature = "transaction")]
    transaction_controller: Controller,
}

impl std::fmt::Debug for AmqpConnectionScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AmqpConnectionScope").finish()
    }
}

impl AmqpConnectionScope {
    /// The suffix to attach to the resource path when using web sockets for service communication.
    pub(crate) const WEB_SOCKETS_PATH_SUFFIX: &'static str = "/$servicebus/websocket/";

    /// The amount of time to allow an AMQP connection to be idle before considering
    /// it to be timed out.
    const CONNECTION_IDLE_TIMEOUT: StdDuration = StdDuration::from_secs(60);

    /// The amount of buffer to apply when considering an authorization token
    /// to be expired.  The token's actual expiration will be decreased by this
    /// amount, ensuring that it is renewed before it has expired.
    const AUTHORIZATION_TOKEN_EXPIRATION_BUFFER: TimeSpan =
        TimeSpan::seconds(AUTHORIZATION_REFRESH_BUFFER_SECONDS as i64 + 2 * 60);
}

impl AmqpConnectionScope {
    /// Initializes a new instance of the [`AmqpConnectionScope`] class.
    pub(crate) async fn new(
        service_endpoint: &Url,
        connection_endpoint: Url, // FIXME: this will be the same as service_endpoint if a custom endpoint is not supplied
        credential: ServiceBusTokenCredential,
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
        let credential = Arc::new(credential);

        let fut = Self::open_connection(&connection_endpoint, &transport_type, &id);
        let connection_handle = crate::util::time::timeout(operation_timeout, fut).await??;
        let mut connection = AmqpConnection::new(connection_handle);

        // TODO: should timeout account for time used previously?
        let fut = Self::begin_session(&mut connection.handle);
        let session_handle = crate::util::time::timeout(operation_timeout, fut).await??;
        let mut session = AmqpSession::new(session_handle);

        #[cfg(feature = "transaction")]
        let transaction_controller = crate::util::time::timeout(
            operation_timeout,
            Self::attach_txn_controller(&mut session.handle, &id),
        )
        .await??;

        let cbs_client = attach_cbs_client(&mut session.handle).await?;
        let cbs_token_provider = CbsTokenProvider::new(
            credential.clone(),
            Self::AUTHORIZATION_TOKEN_EXPIRATION_BUFFER,
        );
        #[cfg(not(target_arch = "wasm32"))]
        let cbs_link = AmqpCbsLink::spawn(cbs_token_provider, cbs_client);
        #[cfg(target_arch = "wasm32")]
        let cbs_link = AmqpCbsLink::spawn_local(cbs_token_provider, cbs_client);

        Ok(Self {
            is_disposed: false,
            id,
            connection_endpoint,
            transport_type,
            connection,
            session,
            cbs_link,
            recover_operation_timeout: operation_timeout,
            credential,
            #[cfg(feature = "transaction")]
            transaction_controller,
        })
    }

    async fn begin_session(
        connection_handle: &mut ConnectionHandle<()>,
    ) -> Result<SessionHandle<()>, AmqpConnectionScopeError> {
        let builder = Session::builder();

        #[cfg(not(target_arch = "wasm32"))]
        let session_handle = builder.begin(connection_handle).await?;
        #[cfg(target_arch = "wasm32")]
        let session_handle = builder
            .begin_on_current_local_set(connection_handle)
            .await?;

        Ok(session_handle)
    }

    async fn open_connection(
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
            .hostname(connection_endpoint.host_str())
            .alt_tls_establishment(true)
            .sasl_profile(SaslProfile::Anonymous)
            .max_frame_size(max_frame_size)
            .idle_time_out(idle_time_out);
        match transport_type {
            #[cfg(not(target_arch = "wasm32"))]
            ServiceBusTransportType::AmqpTcp => connection_builder
                .open(connection_endpoint.clone())
                .await
                .map_err(Into::into),
            ServiceBusTransportType::AmqpWebSocket => {
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

    #[cfg(feature = "transaction")]
    async fn attach_txn_controller(
        session: &mut SessionHandle<()>,
        scope_identifier: &str,
    ) -> Result<Controller, AmqpConnectionScopeError> {
        let controller_id = format!("{}-txn-controller", scope_identifier);
        Controller::attach(session, controller_id)
            .await
            .map_err(Into::into)
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

        self.cbs_link
            .request_refreshable_authorization(link_identifier, endpoint, resource, required_claims)
            .await
            // TODO: The CBS event loop should never spontaneously stop
            .map_err(|_| ManagementError::Send(LinkStateError::IllegalSessionState.into()))??;
        Ok(())
    }

    pub(crate) async fn open_management_link(
        &mut self,
        service_endpoint: &Url,
        entity_path: &str,
        _identifier: &str, // TODO: logging using the identifier
    ) -> Result<AmqpManagementLink, OpenMgmtLinkError> {
        if self.is_disposed {
            return Err(OpenMgmtLinkError::ConnectionScopeDisposed);
        }

        // TODO: customize mgmt-link properties?
        let entity_path = format!("{}/{}", entity_path, MANAGEMENT_ADDRESS);

        let required_claims = vec![
            service_bus_claim::MANAGE.to_string(),
            service_bus_claim::LISTEN.to_string(),
            service_bus_claim::SEND.to_string(),
        ];
        let endpoint = entity_name_formatter::format_endpoint(service_endpoint, &entity_path);
        let resource = endpoint.clone();
        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        self.request_refreshable_authorization_using_cbs(
            link_identifier,
            endpoint,
            resource,
            required_claims,
        )
        .await?;

        let link_name = format!(
            "{};{}:{}:{}",
            self.id, self.connection.identifier, self.session.identifier, link_identifier
        );
        let mgmt_link = MgmtClient::builder()
            .client_node_addr(link_name)
            .management_node_address(entity_path)
            .attach(&mut self.session.handle)
            .await?;
        let mgmt_link = AmqpManagementLink::new(
            link_identifier,
            mgmt_link,
            self.cbs_link.command_sender().clone(),
        );
        Ok(mgmt_link)
    }

    pub(crate) async fn open_sender_link(
        &mut self,
        service_endpoint: &Url,
        entity_path: &str,
        identifier: &str,
    ) -> Result<
        (
            u32,
            fe2o3_amqp::Sender,
            mpsc::Sender<amqp_cbs_link::Command>,
        ),
        OpenSenderError,
    > {
        if self.is_disposed {
            return Err(OpenSenderError::ConnectionScopeDisposed);
        }

        let endpoint = entity_name_formatter::format_endpoint(service_endpoint, entity_path);
        let resource = endpoint.clone();
        let required_claims = vec![service_bus_claim::SEND.to_string()];

        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        self.request_refreshable_authorization_using_cbs(
            link_identifier,
            endpoint.clone(),
            resource,
            required_claims,
        )
        .await?;
        let link_name = format!(
            "{};{}:{}:{}",
            self.id, self.connection.identifier, self.session.identifier, link_identifier
        );
        let sender = fe2o3_amqp::Sender::builder()
            .name(link_name)
            .source(identifier)
            .target(endpoint)
            .attach(&mut self.session.handle)
            .await?;
        Ok((
            link_identifier,
            sender,
            self.cbs_link.command_sender().clone(),
        ))
    }

    /// Open a receiver link to the service bus.
    ///
    /// `session_id` takes a String because it will be encoded as a `Value::String` in the
    /// filter set.
    pub(crate) async fn open_receiver_link(
        &mut self,
        service_endpoint: &Url,
        entity_path: &str,
        identifier: &str,
        receive_mode: &ServiceBusReceiveMode,
        receiver_type: ReceiverType,
        prefetch_count: u32,
    ) -> Result<
        (
            u32,
            fe2o3_amqp::Receiver,
            mpsc::Sender<amqp_cbs_link::Command>,
        ),
        OpenReceiverError,
    > {
        if self.is_disposed {
            return Err(OpenReceiverError::ConnectionScopeDisposed);
        }

        let endpoint = entity_name_formatter::format_endpoint(service_endpoint, entity_path);
        let resource = endpoint.clone();
        let required_claims = vec![service_bus_claim::SEND.to_string()];

        let mut source_builder = Source::builder().address(endpoint.clone());

        source_builder = match receiver_type {
            ReceiverType::NonSession => source_builder.filter(FilterSet::with_capacity(0)),
            ReceiverType::Session { session_id } => source_builder.add_to_filter(
                amqp_client_constants::SESSION_FILTER_NAME,
                session_id.map(SessionFilter).map(|filter| filter.into()),
            ),
        };

        let link_identifier = LINK_IDENTIFIER.fetch_add(1, Ordering::Relaxed);
        self.request_refreshable_authorization_using_cbs(
            link_identifier,
            endpoint,
            resource,
            required_claims,
        )
        .await?;

        // linkSettings.LinkName = $"{connection.Settings.ContainerId};{connection.Identifier}:{session.Identifier}:{link.Identifier}:{linkSettings.Source.ToString()}";
        // connection container id is the scope identifier
        let source = source_builder.build();
        let (snd_settle_mode, rcv_settle_mode) = service_bus_receive_mode_to_amqp(receive_mode);
        let link_name = format!(
            "{};{}:{}:{}:{:?}",
            self.id, self.connection.identifier, self.session.identifier, link_identifier, source
        );

        let mut builder = fe2o3_amqp::Receiver::builder()
            .name(link_name)
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
        Ok((
            link_identifier,
            receiver,
            self.cbs_link.command_sender().clone(),
        ))
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

impl Sealed for AmqpConnectionScope {}

#[async_trait]
impl TransportConnectionScope for AmqpConnectionScope {
    type Error = DisposeError;

    fn transport_type(&self) -> ServiceBusTransportType {
        // This is a simply enum, cloning should be cheaper than or equivalent to a reference
        self.transport_type
    }

    fn is_disposed(&self) -> bool {
        self.is_disposed
    }

    async fn dispose(&mut self) -> Result<(), Self::Error> {
        cfg_wasm32! {
            use fe2o3_amqp::session::error::TryEndError;
            use fe2o3_amqp::connection::TryCloseError;
        }

        if self.is_disposed {
            return Ok(());
        }

        // TODO: close active links? Is this necessary?
        self.is_disposed = true;

        let _ = self.cbs_link.stop();
        let _cbs_close_result = self.cbs_link.join_handle_mut().await;

        #[cfg(not(target_arch = "wasm32"))]
        let session_close_result = self.session.handle.close().await;
        #[cfg(target_arch = "wasm32")]
        let session_close_result = match self.session.handle.try_end() {
            Ok(res) => res,
            Err(TryEndError::AlreadyEnded) => Err(fe2o3_amqp::session::Error::IllegalState),
            Err(TryEndError::RemoteEndNotReceived) => Ok(()), // FIXME: somehow wasm fails to receive the remote end
        };

        #[cfg(not(target_arch = "wasm32"))]
        let connection_close_result = self.connection.handle.close().await;
        #[cfg(target_arch = "wasm32")]
        let connection_close_result = match self.connection.handle.try_close() {
            Ok(res) => res,
            Err(TryCloseError::AlreadyClosed) => Err(fe2o3_amqp::connection::Error::IllegalState),
            Err(TryCloseError::RemoteCloseNotReceived) => Ok(()), // FIXME: somehow wasm fails to receive the remote end
        };

        match (session_close_result, connection_close_result) {
            (Ok(_), Ok(_)) => Ok(()),
            // Connection error has priority
            (_, Err(e)) => Err(DisposeError::ConnectionCloseError(e)),
            (Err(e), _) => Err(DisposeError::SessionCloseError(e)),
        }
    }
}

cfg_not_wasm32! {
    /// Only attemp recovery on non-wasm32 platforms
    #[async_trait]
    impl RecoverableTransport for AmqpConnectionScope {
        type RecoverError = AmqpConnectionScopeError;

        async fn recover(&mut self) -> Result<(), Self::RecoverError> {
            // Perform some state checks before attempting to recover
            if self.is_disposed {
                return Err(AmqpConnectionScopeError::ScopeDisposed);
            }

            // Session and connection event loops are still running
            if !self.session.handle.is_ended() && !self.connection.handle.is_closed() {
                return Ok(());
            }

            // Recover connection first
            if self.connection.handle.is_closed() {
                let result = self.connection.handle.close().await;
                if let Err(err) = result {
                    log::error!("Error closing connection during recovering: {:?}", err);
                }

                // recover
                let fut =
                    Self::open_connection(&self.connection_endpoint, &self.transport_type, &self.id);
                let connection_handle = crate::util::time::timeout(self.recover_operation_timeout, fut).await??;
                self.connection.handle = connection_handle;
            }

            // Recover session
            if self.session.handle.is_ended() {
                if let Err(err) = self.session.handle.end().await {
                    log::error!("Error ending session during recovering: {:?}", err);
                }

                let session_handle = crate::util::time::timeout(
                    self.recover_operation_timeout,
                    Session::begin(&mut self.connection.handle),
                )
                .await??;
                self.session.handle = session_handle;

                // Transaction controller link must be re-created
                // TODO: can txn controller be re-attached?
                #[cfg(feature = "transaction")]
                {
                    let txn_controller = crate::util::time::timeout(
                        self.recover_operation_timeout,
                        Self::attach_txn_controller(&mut self.session.handle, &self.id),
                    )
                    .await??;
                    let prev_txn_controller =
                        std::mem::replace(&mut self.transaction_controller, txn_controller);

                    // TODO: Is it necessary to close the old txn controller?
                    if let Err(err) = prev_txn_controller.close().await {
                        log::error!("Error closing transaction controller: {:?}", err);
                    }
                }

                // recover CBS link only if session was recovered
                let _ = self.cbs_link.stop();
                let _cbs_close_result = self.cbs_link.join_handle_mut().await;
                let cbs_client = attach_cbs_client(&mut self.session.handle).await?;
                let cbs_token_provider = CbsTokenProvider::new(
                    self.credential.clone(),
                    Self::AUTHORIZATION_TOKEN_EXPIRATION_BUFFER,
                );
                let cbs_link = AmqpCbsLink::spawn(cbs_token_provider, cbs_client);
                self.cbs_link = cbs_link;
            }

            Ok(())
        }
    }
}

cfg_wasm32! {
    #[async_trait(?Send)]
    impl RecoverableTransport for AmqpConnectionScope {
        type RecoverError = AmqpConnectionScopeError;

        async fn recover(&mut self) -> Result<(), Self::RecoverError> {
            use fe2o3_amqp::connection::TryCloseError;
            use fe2o3_amqp::session::TryEndError;

            // Perform some state checks before attempting to recover
            if self.is_disposed {
                return Err(AmqpConnectionScopeError::ScopeDisposed);
            }

            // Session and connection event loops are still running
            if !self.session.handle.is_ended() && !self.connection.handle.is_closed() {
                return Ok(());
            }

            // Recover connection first
            if self.connection.handle.is_closed() {
                match self.connection.handle.try_close() {
                    Ok(Ok(_)) => {},
                    Ok(Err(err)) => {
                        log::error!("Error closing connection during recovering: {:?}", err);
                    },
                    Err(TryCloseError::AlreadyClosed) => {
                        log::error!("Error closing connection during recovering: {:?}", "AlreadyClosed");
                    },
                    Err(TryCloseError::RemoteCloseNotReceived) => {
                        log::error!("Error closing connection during recovering: {:?}", "RemoteCloseNotReceived");
                    },
                }

                // recover
                let fut =
                    Self::open_connection(&self.connection_endpoint, &self.transport_type, &self.id);
                let connection_handle = crate::util::time::timeout(self.recover_operation_timeout, fut).await??;
                self.connection.handle = connection_handle;
            }

            // Recover session
            if self.session.handle.is_ended() {
                match self.session.handle.try_end() {
                    Ok(Ok(_)) => {},
                    Ok(Err(err)) => {
                        log::error!("Error ending session during recovering: {:?}", err);
                    },
                    Err(TryEndError::AlreadyEnded) => {
                        log::error!("Error ending session during recovering: {:?}", "AlreadyClosed");
                    },
                    Err(TryEndError::RemoteEndNotReceived) => {
                        log::error!("Error ending session during recovering: {:?}", "RemoteCloseNotReceived");
                    },
                }

                let session_handle = crate::util::time::timeout(
                    self.recover_operation_timeout,
                    Session::builder().begin_on_current_local_set(&mut self.connection.handle)
                )
                .await??;
                self.session.handle = session_handle;

                // Transaction controller link must be re-created
                // TODO: can txn controller be re-attached?
                #[cfg(feature = "transaction")]
                {
                    let txn_controller = crate::util::time::timeout(
                        self.recover_operation_timeout,
                        Self::attach_txn_controller(&mut self.session.handle, &self.id),
                    )
                    .await??;
                    let prev_txn_controller =
                        std::mem::replace(&mut self.transaction_controller, txn_controller);

                    // TODO: Is it necessary to close the old txn controller?
                    if let Err(err) = prev_txn_controller.close().await {
                        log::error!("Error closing transaction controller: {:?}", err);
                    }
                }

                // recover CBS link only if session was recovered
                let _ = self.cbs_link.stop();
                let _cbs_close_result = self.cbs_link.join_handle_mut().await;
                let cbs_client = attach_cbs_client(&mut self.session.handle).await?;
                let cbs_token_provider = CbsTokenProvider::new(
                    self.credential.clone(),
                    Self::AUTHORIZATION_TOKEN_EXPIRATION_BUFFER,
                );
                let cbs_link = AmqpCbsLink::spawn_local(cbs_token_provider, cbs_client);
                self.cbs_link = cbs_link;
            }

            Ok(())
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
