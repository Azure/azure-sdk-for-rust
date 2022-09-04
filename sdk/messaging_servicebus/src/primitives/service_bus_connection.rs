use crate::{amqp::amqp_client::AmqpClient, core::TransportClient};

use super::{
    service_bus_retry_options::ServiceBusRetryOptions,
    service_bus_transport_type::ServiceBusTransportType,
};

/// A connection to the Azure Service Bus service, enabling client communications with a specific
/// Service Bus entity instance within a Service Bus namespace. There is a one-to-one correspondence
/// between [`ServiceBusClient`] and [`ServiceBusConnection`] instances.
#[derive(Debug)]
pub(crate) struct ServiceBusConnection<C>
where
// C: TransportClient,
{
    /// <summary>
    ///   The fully qualified Service Bus namespace that the connection is associated with.
    ///   This is likely to be similar to <c>{yournamespace}.servicebus.windows.net</c>.
    /// </summary>
    // public string FullyQualifiedNamespace { get; }
    pub fully_qualified_namespace: String,

    /// <summary>
    ///   Indicates whether or not this <see cref="ServiceBusConnection"/> has been closed.
    /// </summary>
    ///
    /// <value>
    ///   <c>true</c> if the connection is closed; otherwise, <c>false</c>.
    /// </value>
    // public bool IsClosed => InnerClient.IsClosed;

    /// <summary>
    /// The entity path that the connection is bound to.
    /// </summary>
    // public string EntityPath { get; }
    pub entity_path: String,

    /// <summary>
    ///   The endpoint for the Service Bus service to which the connection is associated.
    ///   This is essentially the <see cref="FullyQualifiedNamespace"/> but with
    ///   the scheme included.
    /// </summary>
    // internal Uri ServiceEndpoint => InnerClient.ServiceEndpoint;

    /// <summary>
    /// The transport type used for this connection.
    /// </summary>
    // public ServiceBusTransportType TransportType { get; }
    pub transport_type: ServiceBusTransportType,

    /// <summary>
    /// The retry options associated with this connection.
    /// </summary>
    // public virtual ServiceBusRetryOptions RetryOptions { get; }
    pub retry_options: ServiceBusRetryOptions,

    pub(crate) inner_client: C,
}
