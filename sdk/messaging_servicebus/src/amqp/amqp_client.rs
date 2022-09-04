use std::time::Duration;

use azure_core::auth::{AccessToken, TokenCredential, TokenResponse};

use super::amqp_connection_scope::AmqpConnectionScope;

const DEFAULT_CREDENTIAL_REFRESH_BUFFER: Duration = Duration::from_secs(5 * 60);

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
    service_endpoint: String, // TODO: Uri?

    /// <summary>
    ///   The endpoint for the Service Bus service to be used when establishing the connection.
    /// </summary>
    ///
    connection_endpoint: String, // TODO: Uri?

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
    transport_metrics: (),
}
