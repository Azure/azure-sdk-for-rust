use std::time::Duration;

use azure_core::{
    auth::{AccessToken, TokenCredential, TokenResponse},
    Url,
};

use crate::{
    authorization::service_bus_token_credential::ServiceBusTokenCredential,
    client::{
        service_bus_client_options::ServiceBusClientOptions,
        service_bus_transport_metrics::ServiceBusTransportMetrics,
    },
    core::TransportClient,
};

use super::{
    amqp_connection_scope::AmqpConnectionScope, amqp_receiver::AmqpReceiver,
    amqp_rule_manager::AmqpRuleManager, amqp_sender::AmqpSender,
};

const DEFAULT_CREDENTIAL_REFRESH_BUFFER: Duration = Duration::from_secs(5 * 60);

#[derive(Debug, thiserror::Error)]
pub enum AmqpClientError {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
}

/// A transport client abstraction responsible for brokering operations for AMQP-based connections.
/// It is intended that the public <see cref="ServiceBusConnection" /> make use of an instance via containment
/// and delegate operations to it.
///
/// See also [`TransportClient`]
#[derive(Debug)]
pub(crate) struct AmqpClient<C>
where
    C: TokenCredential,
{
    /// <summary>
    ///   The buffer to apply when considering refreshing; credentials that expire less than this duration will be refreshed.
    /// </summary>
    ///
    credential_refresh_buffer: Duration,

    /// <summary>Indicates whether or not this instance has been closed.</summary>
    closed: bool,

    /// <summary>The currently active token to use for authorization with the Service Bus service.</summary>
    access_token: AccessToken,

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

    /// <summary>
    ///   Gets the credential to use for authorization with the Service Bus service.
    /// </summary>
    ///
    credential: C,

    /// <summary>
    ///   The AMQP connection scope responsible for managing transport constructs for this instance.
    /// </summary>
    ///
    // private AmqpConnectionScope ConnectionScope { get; }
    connection_scope: AmqpConnectionScope,

    // public override ServiceBusTransportMetrics TransportMetrics { get; }
    transport_metrics: Option<ServiceBusTransportMetrics>,
}

impl<C: TokenCredential> AmqpClient<ServiceBusTokenCredential<C>> {
    pub async fn create(
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

        let transport_metrics = match options.enable_transport_metrics {
            true => Some(ServiceBusTransportMetrics::new()),
            false => None,
        };

        // Create AmqpConnectionScope
        todo!()
    }
}

impl<C: TokenCredential> TransportClient for AmqpClient<C> {
    type Error = ();

    type Sender = AmqpSender;

    type Receiver = AmqpReceiver;

    type RuleManager = AmqpRuleManager;

    fn is_closed(&self) -> bool {
        todo!()
    }

    fn service_endpoint(&self) -> &Url {
        todo!()
    }

    fn create_sender(
        &mut self,
        entity_path: impl Into<String>,
        retry_policy: impl crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicy,
        identifier: impl Into<String>,
    ) -> Result<Self::Sender, Self::Error> {
        todo!()
    }

    fn create_receiver(
        &mut self,
        entity_path: impl Into<String>,
        retry_policy: impl crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicy,
        receive_mode: crate::receiver::service_bus_receive_mode::ServiceBusReceiveMode,
        prefetch_count: u32,
        identifier: impl Into<String>,
        session_id: impl Into<String>,
        is_session_receiver: bool,
        is_processor: bool,
    ) -> Result<Self::Receiver, Self::Error> {
        todo!()
    }

    fn create_rule_manager(
        &mut self,
        subscription_path: impl Into<String>,
        retry_policy: impl crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicy,
        identifier: impl Into<String>,
    ) -> Result<Self::RuleManager, Self::Error> {
        todo!()
    }

    fn close<'life0, 'async_trait>(
        &'life0 mut self,
        cancellation_token: impl 'async_trait + Into<Option<tokio_util::sync::CancellationToken>>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<(), Self::Error>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
