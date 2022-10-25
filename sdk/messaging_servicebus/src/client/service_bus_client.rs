use std::borrow::Cow;

use crate::{
    amqp::amqp_client::AmqpClient,
    authorization::{
        service_bus_token_credential::ServiceBusTokenCredential,
        shared_access_credential::{AzureSasCredential, SharedAccessCredential},
    },
    core::TransportClient,
    diagnostics,
    primitives::{
        inner_client::InnerClient, service_bus_connection::ServiceBusConnection,
        service_bus_transport_type::ServiceBusTransportType,
    },
};

use super::{
    service_bus_client_options::ServiceBusClientOptions,
    service_bus_transport_metrics::ServiceBusTransportMetrics,
};

/// The [`ServiceBusClient`] is the top-level client through which all Service Bus entities can be
/// interacted with. Any lower level types retrieved from here, such as [`ServiceBusSender`] and
/// [`ServiceBusReceiver`] will share the same AMQP connection. Disposing the [`ServiceBusClient`]
/// will cause the AMQP connection to close.
///
/// # Remarks
///
/// The <see cref="ServiceBusClient" /> is safe to cache and use for the lifetime of an application,
/// which is the best practice when the application is making use of Service Bus regularly or
/// semi-regularly.  The client is responsible for ensuring efficient network, CPU, and memory use.
/// Calling <see cref="DisposeAsync" /> as the application is shutting down will ensure that network
/// resources and other unmanaged objects are properly cleaned up.
#[derive(Debug)]
pub struct ServiceBusClient {
    /// Indicates whether or not this instance has been closed.
    ///
    /// TODO: use `ServiceBusConnection::is_closed`?
    closed: bool,

    /// The name used to identify this [`ServiceBusClient`]
    identifier: String,

    /// The connection that is used for the client.
    connection: ServiceBusConnection<AmqpClient<SharedAccessCredential>>, // TODO: use trait objects?
}

impl ServiceBusClient {
    /// The fully qualified Service Bus namespace that the connection is associated with. This is
    /// likely to be similar to `{yournamespace}.servicebus.windows.net`.
    ///
    // public virtual string FullyQualifiedNamespace => Connection.FullyQualifiedNamespace;
    pub fn fully_qualified_namespace(&self) -> &str {
        todo!()
    }

    /// Indicates whether or not this [`ServiceBusClient`] has been closed.
    ///
    /// # Value
    ///
    /// `true` if the client is closed; otherwise, `false`.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// The name used to identify this <see cref="ServiceBusClient"/>.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// The transport type used for this [`ServiceBusClient`].
    pub fn transport_type(&self) -> &ServiceBusTransportType {
        &self.connection.transport_type()
    }

    /// <summary>
    ///   Performs the task needed to clean up resources used by the <see cref="ServiceBusClient" />,
    ///   including ensuring that the client itself has been closed.
    /// </summary>
    ///
    /// <returns>A task to be resolved on when the operation has completed.</returns>
    // [SuppressMessage("Usage", "AZC0002:Ensure all service methods take an optional CancellationToken parameter.", Justification = "This signature must match the IAsyncDisposable interface.")]
    // public virtual async ValueTask DisposeAsync()
    // {
    //     Logger.ClientCloseStart(typeof(ServiceBusClient), Identifier);
    //     IsClosed = true;
    //     try
    //     {
    //         await Connection.CloseAsync(CancellationToken.None).ConfigureAwait(false);
    //         GC.SuppressFinalize(this);
    //     }
    //     catch (Exception ex)
    //     {
    //         Logger.ClientCloseException(typeof(ServiceBusClient), Identifier, ex);
    //         throw;
    //     }
    //     finally
    //     {
    //         Logger.ClientCloseComplete(typeof(ServiceBusClient), Identifier);
    //     }
    // }
    pub async fn dispose(&mut self) {
        todo!()
    }

    /// <summary>
    /// Gets the metrics associated with this <see cref="ServiceBusClient"/> instance. The metrics returned represent a snapshot and will not be updated.
    /// To get updated metrics, this method should be called again.
    /// In order to use this property, <see cref="ServiceBusClientOptions.EnableTransportMetrics"/> must be set to <value>true</value>.
    /// </summary>
    // internal virtual ServiceBusTransportMetrics GetTransportMetrics()
    //     => Connection.InnerClient.TransportMetrics?.Clone() ??
    //         throw new InvalidOperationException("Transport metrics are not enabled. To enable transport metrics, set the EnableTransportMetrics property on the ServiceBusClientOptions.");
    pub(crate) fn transport_metrics(&self) -> Option<ServiceBusTransportMetrics> {
        todo!()
    }

    pub async fn new<'a>(connection_string: impl Into<Cow<'a, str>>) -> Result<Self, super::Error> {
        Self::new_with_options(connection_string, ServiceBusClientOptions::default()).await
    }

    pub async fn new_with_options<'a>(
        connection_string: impl Into<Cow<'a, str>>,
        options: ServiceBusClientOptions,
    ) -> Result<Self, super::Error> {
        let connection_string = connection_string.into();
        let identifier = options.identifier.clone();
        let connection = ServiceBusConnection::new(connection_string, options).await?;
        let identifier = match identifier {
            Some(id) => id,
            None => {
                diagnostics::utilities::generate_identifier(connection.fully_qualified_namespace())
            }
        };
        Ok(Self {
            closed: false,
            identifier,
            connection,
        })
    }

    pub async fn new_with_credential_and_options<C>(
        fully_qualified_namespace: impl Into<String>,
        credential: C,
        options: ServiceBusClientOptions,
    ) -> Result<Self, super::Error> {
        todo!()
    }
}
