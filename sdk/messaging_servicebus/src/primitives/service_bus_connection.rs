use azure_core::Url;

use crate::{
    amqp::amqp_client::AmqpClient, client::service_bus_client_options::ServiceBusClientOptions,
    core::TransportClient,
};

use super::{
    inner_client::InnerClient, service_bus_error::ServiceBusError,
    service_bus_retry_options::ServiceBusRetryOptions,
    service_bus_transport_type::ServiceBusTransportType,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Argument error: {}", .0)]
    ArgumentError(String),
}

/// A connection to the Azure Service Bus service, enabling client communications with a specific
/// Service Bus entity instance within a Service Bus namespace. There is a one-to-one correspondence
/// between [`ServiceBusClient`] and [`ServiceBusConnection`] instances.
#[derive(Debug)]
pub(crate) struct ServiceBusConnection {
    fully_qualified_namespace: String,
    entity_path: String,
    transport_type: ServiceBusTransportType,
    retry_options: ServiceBusRetryOptions,

    pub(crate) inner_client: InnerClient,
}

impl ServiceBusConnection {
    /// Indicates whether or not this [`ServiceBusConnection`] has been closed.
    ///
    /// # Value
    ///
    /// `true` if the connection is closed; otherwise, `false`.
    pub fn is_closed(&self) -> bool {
        self.inner_client.is_closed()
    }

    /// The fully qualified Service Bus namespace that the connection is associated with.
    /// This is likely to be similar to `{yournamespace}.servicebus.windows.net`.
    pub fn fully_qualified_namespace(&self) -> &str {
        &self.fully_qualified_namespace
    }

    /// <summary>
    /// The entity path that the connection is bound to.
    /// </summary>
    // public string EntityPath { get; }
    pub fn entity_path(&self) -> &str {
        &self.entity_path
    }

    /// The endpoint for the Service Bus service to which the connection is associated.
    /// This is essentially the <see cref="FullyQualifiedNamespace"/> but with
    /// the scheme included.
    pub(crate) fn service_endpoint(&self) -> &Url {
        self.inner_client.service_endpoint()
    }

    /// The transport type used for this connection.
    pub fn transport_type(&self) -> &ServiceBusTransportType {
        &self.transport_type
    }

    /// The retry options associated with this connection.
    pub fn retry_options(&self) -> &ServiceBusRetryOptions {
        &self.retry_options
    }

    pub(crate) async fn open(
        connection_string: impl Into<String>,
        options: ServiceBusClientOptions,
    ) -> Result<Self, Error> {
        todo!()
    }
}
