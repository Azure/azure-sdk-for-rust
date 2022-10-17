use std::time::Duration;

use async_trait::async_trait;
use azure_core::{
    auth::{AccessToken, TokenCredential, TokenResponse},
    Url,
};
use fe2o3_amqp::{connection::OpenError, link::SenderAttachError, session::BeginError};
use tokio::time::error::Elapsed;
use tokio_util::sync::CancellationToken;

use crate::{
    authorization::service_bus_token_credential::ServiceBusTokenCredential,
    client::{
        service_bus_client_options::ServiceBusClientOptions,
        service_bus_transport_metrics::ServiceBusTransportMetrics,
    },
    core::TransportClient,
    primitives::service_bus_retry_policy::ServiceBusRetryPolicy,
    receiver::service_bus_receive_mode::ServiceBusReceiveMode,
};

use super::{
    amqp_connection_scope::{AmqpConnectionScope, AmqpConnectionScopeError},
    amqp_receiver::AmqpReceiver,
    amqp_rule_manager::AmqpRuleManager,
    amqp_sender::AmqpSender,
};

const DEFAULT_CREDENTIAL_REFRESH_BUFFER: Duration = Duration::from_secs(5 * 60);

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
}

impl From<AmqpConnectionScopeError> for AmqpClientError {
    fn from(err: AmqpConnectionScopeError) -> Self {
        match err {
            AmqpConnectionScopeError::Open(err) => Self::Open(err),
            AmqpConnectionScopeError::WebSocket(err) => Self::WebSocket(err),
            AmqpConnectionScopeError::TimeoutElapsed(err) => Self::TimeoutElapsed(err),
            AmqpConnectionScopeError::Begin(err) => Self::Begin(err),
            AmqpConnectionScopeError::SenderAttach(err) => Self::SenderAttach(err),
        }
    }
}

/// A transport client abstraction responsible for brokering operations for AMQP-based connections.
/// It is intended that the public <see cref="ServiceBusConnection" /> make use of an instance via containment
/// and delegate operations to it.
///
/// See also [`TransportClient`]
#[derive(Debug)]
pub(crate) struct AmqpClient<TC>
where
    TC: TokenCredential,
{
    /// <summary>
    ///   The buffer to apply when considering refreshing; credentials that expire less than this duration will be refreshed.
    /// </summary>
    ///
    credential_refresh_buffer: Duration,

    /// <summary>Indicates whether or not this instance has been closed.</summary>
    closed: bool,

    /// <summary>The currently active token to use for authorization with the Service Bus service.</summary>
    access_token: Option<AccessToken>,

    /// <summary>
    ///   The endpoint for the Service Bus service to which the client is associated.
    /// </summary>
    ///
    service_endpoint: Url,

    /// <summary>
    ///   The endpoint for the Service Bus service to be used when establishing the connection.
    /// </summary>
    ///
    connection_endpoint: Url,

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
    connection_scope: AmqpConnectionScope<TC>,
    // TODO: implement metrics
    // // public override ServiceBusTransportMetrics TransportMetrics { get; }
    // transport_metrics: Option<ServiceBusTransportMetrics>,
}

impl<C: TokenCredential> AmqpClient<C> {
    pub async fn new(
        host: &str,
        credential: ServiceBusTokenCredential<C>,
        options: ServiceBusClientOptions,
    ) -> Result<Self, AmqpClientError> {
        let service_endpoint = {
            let scheme = options.transport_type.url_scheme();
            let addr = format!("{scheme}://{host}");
            Url::parse(&addr)?
        };

        let connection_endpoint = match options
            .custom_endpoint_address
            .as_ref()
            .and_then(|url| url.host_str())
        {
            Some(custom_host) => {
                let addr = format!("{}://{}", service_endpoint.scheme(), custom_host);
                Url::parse(&addr)?
            }
            None => service_endpoint.clone(),
        };

        // let transport_metrics = match options.enable_transport_metrics {
        //     true => Some(ServiceBusTransportMetrics::new()),
        //     false => None,
        // };

        // Create AmqpConnectionScope
        let connection_scope = AmqpConnectionScope::new(
            service_endpoint.clone(),
            connection_endpoint.clone(),
            credential,
            options.transport_type,
            *options.retry_options.try_timeout(),
        )
        .await?;
        Ok(Self {
            credential_refresh_buffer: Duration::from_secs(5 * 60), // 5 mins
            closed: false,
            access_token: None,
            service_endpoint,
            connection_endpoint,
            connection_scope,
            // transport_metrics,
        })
    }
}

#[async_trait]
impl<C: TokenCredential> TransportClient for AmqpClient<C> {
    type Error = ();

    type Sender = AmqpSender;

    type Receiver = AmqpReceiver;

    type RuleManager = AmqpRuleManager;

    /// Indicates whether or not this client has been closed.
    ///
    /// Returnss `true` if the client is closed, otherwise `false`
    fn is_closed(&self) -> bool {
        todo!()
    }

    /// The endpoint for the Service Bus service to which the client is associated.
    fn service_endpoint(&self) -> &Url {
        todo!()
    }

    /// Creates a sender strongly aligned with the active protocol and transport,
    /// responsible for sending <see cref="ServiceBusMessage" /> to the entity.
    ///
    /// # Arguments
    ///
    /// * `entity_path` - The entity path to send the message to.
    /// * `retry_policy` - The policy which governs retry behavior and try timeouts
    /// * `identifier` - The identifier for the sender.
    ///
    /// # Returns
    ///
    /// A [TransportSender] configured in the requested manner.
    fn create_sender(
        &mut self,
        entity_path: impl Into<String>, // TODO: AsRef<str> or AsRef<Path>?
        retry_policy: impl ServiceBusRetryPolicy,
        identifier: impl Into<String>,
    ) -> Result<Self::Sender, Self::Error> {
        todo!()
    }

    fn create_receiver(
        &mut self,
        entity_path: impl Into<String>,
        retry_policy: impl ServiceBusRetryPolicy,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
        identifier: impl Into<String>,
        session_id: impl Into<String>,
        is_session_receiver: bool,
        is_processor: bool,
    ) -> Result<Self::Receiver, Self::Error> {
        todo!()
    }

    /// Creates a rule manager strongly aligned with the active protocol and transport, responsible
    /// for adding, removing and getting rules from the Service Bus subscription.
    ///
    /// # Arguments
    ///
    /// * `subscription_path` - The path of the Service Bus subscription to which the rule manager
    ///   is bound.
    /// * `retry_policy` - The policy which governs retry behavior and try timeouts.
    /// * `identifier` - The identifier for the rule manager.
    ///
    /// # Returns
    ///
    /// A [TransportRuleManager] configured in the requested manner.
    fn create_rule_manager(
        &mut self,
        subscription_path: impl Into<String>,
        retry_policy: impl ServiceBusRetryPolicy,
        identifier: impl Into<String>,
    ) -> Result<Self::RuleManager, Self::Error> {
        todo!()
    }

    /// Closes the connection to the transport client instance.
    ///
    /// # Arguments
    ///
    /// An optional [CancellationToken] instance to signal the request to cancel the operation.
    async fn close(
        &mut self,
        cancellation_token: impl Into<Option<CancellationToken>> + Send,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
