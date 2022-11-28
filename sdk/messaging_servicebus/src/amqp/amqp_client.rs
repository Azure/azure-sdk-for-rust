use std::{marker::PhantomData, time::Duration};

use async_trait::async_trait;
use azure_core::{auth::AccessToken, Url};
use fe2o3_amqp::{
    connection::OpenError,
    link::{ReceiverAttachError, SenderAttachError},
    session::BeginError,
};
use tokio::time::error::Elapsed;
use tokio_util::sync::CancellationToken;

use crate::{
    authorization::service_bus_token_credential::ServiceBusTokenCredential,
    constants::DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER,
    core::{TransportClient, TransportConnectionScope},
    primitives::{
        service_bus_retry_options::ServiceBusRetryOptions,
        service_bus_retry_policy::ServiceBusRetryPolicy,
        service_bus_transport_type::ServiceBusTransportType,
    },
    receiver::service_bus_receive_mode::ServiceBusReceiveMode,
};

use super::{
    amqp_connection_scope::{AmqpConnectionScope, AmqpConnectionScopeError},
    amqp_receiver::AmqpReceiver,
    amqp_rule_manager::AmqpRuleManager,
    amqp_sender::AmqpSender,
    amqp_session_receiver::AmqpSessionReceiver,
    error::{DisposeError, OpenReceiverError, OpenSenderError},
};

// const DEFAULT_CREDENTIAL_REFRESH_BUFFER: Duration = Duration::from_secs(5 * 60);

#[derive(Debug, thiserror::Error)]
pub enum AmqpClientError {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

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

    #[error("Cancelled")]
    Cancelled,

    #[error(transparent)]
    Dispose(#[from] DisposeError),
}

impl From<AmqpConnectionScopeError> for AmqpClientError {
    fn from(err: AmqpConnectionScopeError) -> Self {
        match err {
            AmqpConnectionScopeError::Open(err) => Self::Open(err),
            AmqpConnectionScopeError::WebSocket(err) => Self::WebSocket(err),
            AmqpConnectionScopeError::TimeoutElapsed(err) => Self::TimeoutElapsed(err),
            AmqpConnectionScopeError::Begin(err) => Self::Begin(err),
            AmqpConnectionScopeError::SenderAttach(err) => Self::SenderAttach(err),
            AmqpConnectionScopeError::Rng(err) => Self::Rng(err),
            AmqpConnectionScopeError::ReceiverAttach(err) => Self::ReceiverAttach(err),
        }
    }
}

/// A transport client abstraction responsible for brokering operations for AMQP-based connections.
/// It is intended that the public <see cref="ServiceBusConnection" /> make use of an instance via containment
/// and delegate operations to it.
///
/// See also [`TransportClient`]
#[derive(Debug)]
pub struct AmqpClient<RP> {
    /// <summary>
    ///   The buffer to apply when considering refreshing; credentials that expire less than this duration will be refreshed.
    /// </summary>
    ///
    credential_refresh_buffer: Duration,

    /// <summary>Indicates whether or not this instance has been closed.</summary>
    closed: bool,

    /// <summary>The currently active token to use for authorization with the Service Bus service.</summary>
    access_token: Option<AccessToken>,

    // /// <summary>
    // ///   Gets the credential to use for authorization with the Service Bus service.
    // /// </summary>
    // ///
    // credential: TC, // TODO: is this the same credential used in `connection_scope`?

    //
    /// <summary>
    ///   The AMQP connection scope responsible for managing transport constructs for this instance.
    /// </summary>
    ///
    // private AmqpConnectionScope ConnectionScope { get; }
    connection_scope: AmqpConnectionScope,

    transport_type: ServiceBusTransportType,

    /// Retry policy phantom
    retry_policy: PhantomData<RP>,
}

impl<RP> AmqpClient<RP>
where
    RP: ServiceBusRetryPolicy,
{
    pub(crate) fn set_retry_policy<RP2>(self) -> AmqpClient<RP2> {
        AmqpClient {
            credential_refresh_buffer: self.credential_refresh_buffer,
            closed: self.closed,
            access_token: self.access_token,
            connection_scope: self.connection_scope,
            transport_type: self.transport_type,
            retry_policy: PhantomData,
        }
    }
}

#[async_trait]
impl<RP> TransportClient for AmqpClient<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
{
    type CreateClientError = AmqpClientError;
    type CreateSenderError = OpenSenderError;
    type CreateReceiverError = OpenReceiverError;
    type CreateRuleManagerError = AmqpClientError;
    type DisposeError = AmqpClientError;

    type Sender = AmqpSender<RP>;
    type Receiver = AmqpReceiver<RP>;
    type SessionReceiver = AmqpSessionReceiver<RP>;
    type RuleManager = AmqpRuleManager;

    async fn create_transport_client(
        host: &str,
        credential: ServiceBusTokenCredential,
        transport_type: ServiceBusTransportType,
        custom_endpoint: Option<Url>,
        retry_timeout: Duration,
    ) -> Result<Self, Self::CreateClientError> {
        let service_endpoint = match transport_type {
            ServiceBusTransportType::AmqpTcp => {
                let addr = format!("{}://{}", transport_type.url_scheme(), host);
                Url::parse(&addr)?
            }
            ServiceBusTransportType::AmqpWebSockets => {
                let addr = format!(
                    "{}://{}{}",
                    transport_type.url_scheme(),
                    host,
                    AmqpConnectionScope::WEB_SOCKETS_PATH_SUFFIX
                );
                Url::parse(&addr)?
            }
        };

        let connection_endpoint = match custom_endpoint.as_ref().and_then(|url| url.host_str()) {
            Some(custom_host) => match transport_type {
                ServiceBusTransportType::AmqpTcp => {
                    let addr = format!("{}://{}", service_endpoint.scheme(), custom_host);
                    Url::parse(&addr)?
                }
                ServiceBusTransportType::AmqpWebSockets => {
                    let addr = format!(
                        "{}://{}{}",
                        service_endpoint.scheme(),
                        custom_host,
                        AmqpConnectionScope::WEB_SOCKETS_PATH_SUFFIX
                    );
                    Url::parse(&addr)?
                }
            },
            None => service_endpoint.clone(),
        };

        // Create AmqpConnectionScope
        let connection_scope = AmqpConnectionScope::new(
            service_endpoint,
            connection_endpoint,
            credential,
            transport_type.clone(),
            retry_timeout,
        )
        .await?;

        Ok(Self {
            credential_refresh_buffer: Duration::from_secs(5 * 60), // 5 mins
            closed: false,
            access_token: None,
            connection_scope,
            retry_policy: PhantomData,
            transport_type,
        })
    }

    fn transport_type(&self) -> &ServiceBusTransportType {
        &self.transport_type
    }

    /// Indicates whether or not this client has been closed.
    ///
    /// Returnss `true` if the client is closed, otherwise `false`
    fn is_closed(&self) -> bool {
        todo!()
    }

    /// The endpoint for the Service Bus service to which the client is associated.
    fn service_endpoint(&self) -> &Url {
        self.connection_scope.service_endpoint()
    }

    /// Creates a sender strongly aligned with the active protocol and transport,
    /// responsible for sending <see cref="ServiceBusMessage" /> to the entity.
    ///
    /// # Arguments
    ///
    /// * `entity_path` - The entity path to send the message to.
    /// * `retry_options` - The policy which governs retry behavior and try timeouts
    /// * `identifier` - The identifier for the sender.
    ///
    /// # Returns
    ///
    /// A [TransportSender] configured in the requested manner.
    async fn create_sender(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
    ) -> Result<Self::Sender, Self::CreateSenderError> {
        // TODO: this will be updated once GAT is stablized
        let (link_identifier, link_name, sender) = self
            .connection_scope
            .open_sender_link(&entity_path, &identifier)
            .await?;
        let management_client = self
            .connection_scope
            .open_management_link(&entity_path, &identifier)
            .await?;
        let retry_policy = RP::new(retry_options);
        Ok(AmqpSender {
            identifier: link_identifier,
            name: link_name,
            retry_policy,
            sender,
            management_client,
        })
    }

    async fn create_receiver(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
        is_processor: bool,
    ) -> Result<Self::Receiver, Self::CreateReceiverError> {
        let (link_identifier, receiver) = self
            .connection_scope
            .open_receiver_link(
                &entity_path,
                &identifier,
                &receive_mode,
                None,
                prefetch_count,
            )
            .await?;
        let management_client = self
            .connection_scope
            .open_management_link(&entity_path, &identifier)
            .await?;
        let retry_policy = RP::new(retry_options);
        Ok(AmqpReceiver {
            identifier: link_identifier,
            retry_policy,
            receiver,
            receive_mode,
            is_processor,
            prefetch_count,
            management_client,
            request_response_locked_messages: Default::default(),
            last_peeked_sequence_number: DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER,
        })
    }

    async fn create_session_receiver(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        session_id: String,
        prefetch_count: u32,
        is_processor: bool,
    ) -> Result<Self::SessionReceiver, Self::CreateReceiverError> {
        let (link_identifier, receiver) = self
            .connection_scope
            .open_receiver_link(
                &entity_path,
                &identifier,
                &receive_mode,
                Some(session_id),
                prefetch_count,
            )
            .await?;
        let management_client = self
            .connection_scope
            .open_management_link(&entity_path, &identifier)
            .await?;
        let retry_policy = RP::new(retry_options);
        let inner = AmqpReceiver {
            identifier: link_identifier,
            retry_policy,
            receiver,
            receive_mode,
            is_processor,
            prefetch_count,
            management_client,
            request_response_locked_messages: Default::default(),
            last_peeked_sequence_number: DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER,
        };
        Ok(AmqpSessionReceiver { inner })
    }

    /// Creates a rule manager strongly aligned with the active protocol and transport, responsible
    /// for adding, removing and getting rules from the Service Bus subscription.
    ///
    /// # Arguments
    ///
    /// * `subscription_path` - The path of the Service Bus subscription to which the rule manager
    ///   is bound.
    /// * `retry_options` - The policy which governs retry behavior and try timeouts.
    /// * `identifier` - The identifier for the rule manager.
    ///
    /// # Returns
    ///
    /// A [TransportRuleManager] configured in the requested manner.
    async fn create_rule_manager(
        &mut self,
        _subscription_path: String,
        _retry_options: ServiceBusRetryOptions,
        _identifier: String,
    ) -> Result<Self::RuleManager, Self::CreateRuleManagerError> {
        todo!()
    }

    /// Closes the connection to the transport client instance.
    ///
    /// # Arguments
    ///
    /// An optional [CancellationToken] instance to signal the request to cancel the operation.
    async fn close(
        &mut self,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::DisposeError> {
        if self.closed {
            Ok(())
        } else {
            match cancellation_token {
                Some(token) => {
                    tokio::select! {
                        _cancel = token.cancelled() => Err(Self::DisposeError::Cancelled),
                        result = self.connection_scope.dispose() => {
                            self.closed = true;
                            result.map_err(Into::into)
                        }
                    }
                }
                None => self
                    .connection_scope
                    .dispose()
                    .await
                    .and_then(|_| {
                        self.closed = true;
                        Ok(())
                    })
                    .map_err(Into::into),
            }
        }
    }
}

#[cfg(test)]
mod tests {}
