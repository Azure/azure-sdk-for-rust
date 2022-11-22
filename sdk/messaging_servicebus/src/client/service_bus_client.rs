use std::borrow::Cow;

use azure_core::auth::TokenCredential;

use crate::{
    amqp::{
        amqp_client::AmqpClient,
        error::{OpenReceiverError, OpenSenderError},
    },
    authorization::{
        service_bus_token_credential::ServiceBusTokenCredential,
        shared_access_credential::SharedAccessCredential,
    },
    core::{BasicRetryPolicy, TransportClient},
    diagnostics,
    entity_name_formatter::format_entity_path,
    primitives::{
        service_bus_connection::ServiceBusConnection,
        service_bus_transport_type::ServiceBusTransportType,
    },
    receiver::service_bus_session_receiver::{
        ServiceBusSessionReceiver, ServiceBusSessionReceiverOptions,
    },
    ServiceBusReceiver, ServiceBusReceiverOptions, ServiceBusSender, ServiceBusSenderOptions,
};

use super::{service_bus_client_options::ServiceBusClientOptions, Error};

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
pub struct ServiceBusClient<C> {
    /// Indicates whether or not this instance has been closed.
    ///
    /// TODO: use `ServiceBusConnection::is_closed`?
    closed: bool,

    /// The name used to identify this [`ServiceBusClient`]
    identifier: String,

    /// The connection that is used for the client.
    connection: ServiceBusConnection<C>, // TODO: use trait objects?
}

impl ServiceBusClient<AmqpClient<SharedAccessCredential, BasicRetryPolicy>> {
    pub async fn new<'a>(connection_string: impl Into<Cow<'a, str>>) -> Result<Self, super::Error> {
        Self::new_with_options(connection_string, ServiceBusClientOptions::default()).await
    }

    pub async fn new_with_options<'a>(
        connection_string: impl Into<Cow<'a, str>>,
        options: ServiceBusClientOptions,
    ) -> Result<Self, Error> {
        let connection_string = connection_string.into();
        let identifier = options.identifier.clone();
        let connection = ServiceBusConnection::new(connection_string, options).await?;
        let identifier = identifier.unwrap_or(diagnostics::utilities::generate_identifier(
            connection.fully_qualified_namespace(),
        ));
        Ok(Self {
            closed: false,
            identifier,
            connection,
        })
    }
}

impl<C> ServiceBusClient<C>
where
    C: TransportClient + Send + Sync,
{
    /// The fully qualified Service Bus namespace that the connection is associated with. This is
    /// likely to be similar to `{yournamespace}.servicebus.windows.net`.
    ///
    // public virtual string FullyQualifiedNamespace => Connection.FullyQualifiedNamespace;
    pub fn fully_qualified_namespace(&self) -> &str {
        self.connection.fully_qualified_namespace()
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

    // /// <summary>
    // /// Gets the metrics associated with this <see cref="ServiceBusClient"/> instance. The metrics returned represent a snapshot and will not be updated.
    // /// To get updated metrics, this method should be called again.
    // /// In order to use this property, <see cref="ServiceBusClientOptions.EnableTransportMetrics"/> must be set to <value>true</value>.
    // /// </summary>
    // // internal virtual ServiceBusTransportMetrics GetTransportMetrics()
    // //     => Connection.InnerClient.TransportMetrics?.Clone() ??
    // //         throw new InvalidOperationException("Transport metrics are not enabled. To enable transport metrics, set the EnableTransportMetrics property on the ServiceBusClientOptions.");
    // pub(crate) fn transport_metrics(&self) -> Option<ServiceBusTransportMetrics> {
    //     todo!()
    // }
}

impl ServiceBusClient<()> {
    pub async fn new_with_credential_and_options<TC, C>(
        fully_qualified_namespace: impl Into<String>,
        credential: TC,
        options: ServiceBusClientOptions,
    ) -> Result<ServiceBusClient<C>, Error>
    where
        TC: TokenCredential,
        ServiceBusTokenCredential<TC>: From<TC>,
        C: TransportClient<TokenCredential = TC>,
        Error: From<C::CreateClientError>,
    {
        let fully_qualified_namespace = fully_qualified_namespace.into();
        let identifier =
            options
                .identifier
                .clone()
                .unwrap_or(diagnostics::utilities::generate_identifier(
                    &fully_qualified_namespace,
                ));
        let connection = ServiceBusConnection::new_with_credential::<TC, C>(
            fully_qualified_namespace,
            credential,
            options,
        )
        .await?;
        Ok(ServiceBusClient {
            closed: false,
            identifier,
            connection,
        })
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Dispose                                  */
/* -------------------------------------------------------------------------- */

impl<C> ServiceBusClient<C>
where
    C: TransportClient + Send + Sync + 'static,
    Error: From<C::DisposeError>,
{
    /// <summary>
    ///   Performs the task needed to clean up resources used by the <see cref="ServiceBusClient" />,
    ///   including ensuring that the client itself has been closed.
    /// </summary>
    ///
    /// <returns>A task to be resolved on when the operation has completed.</returns>
    pub async fn dispose(&mut self) -> Result<(), Error> {
        self.closed = true;

        self.connection.dispose().await?;
        Ok(())
    }
}

/* -------------------------------------------------------------------------- */
/*                                Create Sender                               */
/* -------------------------------------------------------------------------- */

impl<C> ServiceBusClient<C>
where
    C: TransportClient + Send + Sync + 'static,
    OpenSenderError: From<C::CreateSenderError>,
{
    pub async fn create_sender(
        &mut self,
        queue_or_topic_name: impl Into<String>,
        options: ServiceBusSenderOptions,
    ) -> Result<ServiceBusSender<C::Sender>, OpenSenderError> {
        let entity_path = queue_or_topic_name.into();
        let identifier = options
            .identifier
            .unwrap_or(diagnostics::utilities::generate_identifier(&entity_path));
        let retry_options = self.connection.retry_options().clone();
        let inner = self
            .connection
            .create_transport_sender(entity_path.clone(), identifier.clone(), retry_options) // TODO: remove clone once GAT is stablized
            .await?;

        Ok(ServiceBusSender {
            inner,
            entity_path,
            identifier,
        })
    }
}

/* -------------------------------------------------------------------------- */
/*                               Create Receiver                              */
/* -------------------------------------------------------------------------- */

impl<C> ServiceBusClient<C>
where
    C: TransportClient + Send + Sync + 'static,
    OpenReceiverError: From<C::CreateReceiverError>,
{
    // This cannot be used to create a session receiver or proces
    pub async fn create_receiver(
        &mut self,
        queue_or_topic_name: impl Into<String>,
        options: ServiceBusReceiverOptions,
    ) -> Result<ServiceBusReceiver<C::Receiver>, C::CreateReceiverError> {
        let entity_path = queue_or_topic_name.into();
        let identifier = options
            .identifier
            .unwrap_or(diagnostics::utilities::generate_identifier(&entity_path));
        let retry_options = self.connection.retry_options().clone();
        let receive_mode = options.receive_mode;
        let prefetch_count = options.prefetch_count;
        let entity_path = format_entity_path(entity_path, options.sub_queue);

        let inner = self
            .connection
            .create_transport_receiver(
                entity_path.clone(),
                identifier.clone(),
                retry_options,
                receive_mode,
                prefetch_count,
                false,
            )
            .await?;
        Ok(ServiceBusReceiver {
            inner,
            entity_path,
            identifier,
        })
    }

    pub async fn accept_session(
        &mut self,
        queue_or_topic_name: impl Into<String>,
        session_id: impl Into<String>,
        options: ServiceBusSessionReceiverOptions,
    ) -> Result<ServiceBusSessionReceiver<C::Receiver>, C::CreateReceiverError> {
        let entity_path = queue_or_topic_name.into();
        let identifier = options
            .identifier
            .unwrap_or(diagnostics::utilities::generate_identifier(&entity_path));
        let retry_options = self.connection.retry_options().clone();
        let receive_mode = options.receive_mode;
        let prefetch_count = options.prefetch_count;
        let session_id = session_id.into();

        let inner = self
            .connection
            .create_transport_session_receiver(
                entity_path.clone(),
                identifier.clone(),
                retry_options,
                receive_mode,
                prefetch_count,
                session_id.clone(),
                false,
            )
            .await?;

        let inner = ServiceBusReceiver {
            inner,
            entity_path,
            identifier,
        };
        Ok(ServiceBusSessionReceiver { inner, session_id })
    }
}
