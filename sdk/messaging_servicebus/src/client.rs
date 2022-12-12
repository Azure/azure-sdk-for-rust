//! Implements the ServiceBusClient

use std::borrow::Cow;

use azure_core::Url;

use crate::{
    amqp::{
        amqp_client::AmqpClient,
        error::{OpenSenderError},
    },
    authorization::service_bus_token_credential::ServiceBusTokenCredential,
    core::{BasicRetryPolicy, TransportClient},
    diagnostics,
    entity_name_formatter::{self, format_entity_path},
    primitives::{
        service_bus_connection::ServiceBusConnection,
        service_bus_transport_type::ServiceBusTransportType, error::Error, service_bus_retry_options::ServiceBusRetryOptions,
    },
    receiver::service_bus_session_receiver::{
        ServiceBusSessionReceiver, ServiceBusSessionReceiverOptions,
    },
    ServiceBusReceiver, ServiceBusReceiverOptions, ServiceBusSender, ServiceBusSenderOptions,
};

/// The set of options that can be specified when creating an [`ServiceBusConnection`]
/// to configure its behavior.
#[derive(Debug, Clone, Default)]
pub struct ServiceBusClientOptions {
    /// The type of protocol and transport that will be used for communicating with the Service
    /// Bus service.
    pub transport_type: ServiceBusTransportType,

    /// A property used to set the [`ServiceBusClient`] ID to identify the client. This can be used
    /// to correlate logs and exceptions. If `None` or empty, a random unique value will be
    /// used.
    pub identifier: Option<String>,

    /// A custom endpoint address that can be used when establishing the connection to the Service
    /// Bus service.
    ///
    /// # Remarks
    ///
    /// The custom endpoint address will be used in place of the default endpoint provided by the
    /// Service Bus namespace when establishing the connection. The connection string or fully
    /// qualified namespace will still be needed in order to validate the connection with the
    /// service.
    pub custom_endpoint_address: Option<Url>,

    /// The set of options to use for determining whether a failed operation should be retried and,
    /// if so, the amount of time to wait between retry attempts.  These options also control the
    /// amount of time allowed for receiving messages and other interactions with the Service Bus
    /// service.
    pub retry_options: ServiceBusRetryOptions,

    /// Gets or sets a flag that indicates whether or not transactions may span multiple
    /// Service Bus entities.
    ///
    /// # Value
    ///
    /// `true`, when cross-entity transactions are enabled; `false` when transactions are not being
    /// used or should be limited to a single entity.
    pub enable_cross_entity_transactions: bool,
}

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
    /// The name used to identify this [`ServiceBusClient`]
    identifier: String,

    /// The connection that is used for the client.
    connection: ServiceBusConnection<C>, // TODO: use trait objects?
}

impl ServiceBusClient<AmqpClient<BasicRetryPolicy>> {
    pub async fn new<'a>(
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
            // closed: false,
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

    /// The name used to identify this <see cref="ServiceBusClient"/>.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Indicates whether or not this [`ServiceBusClient`] has been closed.
    ///
    /// # Value
    ///
    /// `true` if the client is closed; otherwise, `false`.
    pub fn is_closed(&self) -> bool {
        self.connection.is_closed()
    }
}

impl ServiceBusClient<()> {
    pub async fn new_with_credential_and_options<C>(
        fully_qualified_namespace: impl Into<String>,
        credential: impl Into<ServiceBusTokenCredential>,
        options: ServiceBusClientOptions,
    ) -> Result<ServiceBusClient<C>, Error>
    where
        C: TransportClient,
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
        let connection = ServiceBusConnection::new_with_credential(
            fully_qualified_namespace,
            credential,
            options,
        )
        .await?;
        Ok(ServiceBusClient {
            // closed: false,
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
        // self.closed = true;

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
            .filter(|id| !id.is_empty())
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
{
    pub fn transport_type(&self) -> ServiceBusTransportType {
        self.connection.transport_type()
    }

    pub async fn create_receiver_for_queue(
        &mut self,
        queue_name: impl Into<String>,
        options: ServiceBusReceiverOptions,
    ) -> Result<ServiceBusReceiver<C::Receiver>, C::CreateReceiverError> {
        let entity_path = queue_name.into();
        self.create_receiver(entity_path, options).await
    }

    pub async fn create_receiver_for_subscription(
        &mut self,
        topic_name: impl AsRef<str>,
        subscription_name: impl AsRef<str>,
        options: ServiceBusReceiverOptions,
    ) -> Result<ServiceBusReceiver<C::Receiver>, C::CreateReceiverError> {
        let entity_path = entity_name_formatter::format_subscription_path(
            topic_name.as_ref(),
            subscription_name.as_ref(),
        );
        self.create_receiver(entity_path, options).await
    }

    // This cannot be used to create a session receiver or proces
    async fn create_receiver(
        &mut self,
        entity_path: String,
        options: ServiceBusReceiverOptions,
    ) -> Result<ServiceBusReceiver<C::Receiver>, C::CreateReceiverError> {
        let identifier = options
            .identifier
            .filter(|id| !id.is_empty())
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

    pub async fn accept_next_session_for_queue(
        &mut self,
        queue_name: impl Into<String>,
        session_id: impl Into<String>,
        options: ServiceBusSessionReceiverOptions,
    ) -> Result<ServiceBusSessionReceiver<C::SessionReceiver>, C::CreateReceiverError> {
        let entity_path = queue_name.into();
        let session_id = session_id.into();
        self.accept_next_session(entity_path, session_id, options)
            .await
    }

    pub async fn accept_next_session_for_subscription(
        &mut self,
        topic_name: impl AsRef<str>,
        subscription_name: impl AsRef<str>,
        session_id: impl Into<String>,
        options: ServiceBusSessionReceiverOptions,
    ) -> Result<ServiceBusSessionReceiver<C::SessionReceiver>, C::CreateReceiverError> {
        let entity_path = entity_name_formatter::format_subscription_path(
            topic_name.as_ref(),
            subscription_name.as_ref(),
        );
        let session_id = session_id.into();
        self.accept_next_session(entity_path, session_id, options)
            .await
    }

    async fn accept_next_session(
        &mut self,
        entity_path: String,
        session_id: String,
        options: ServiceBusSessionReceiverOptions,
    ) -> Result<ServiceBusSessionReceiver<C::SessionReceiver>, C::CreateReceiverError> {
        let identifier = options
            .identifier
            .unwrap_or(diagnostics::utilities::generate_identifier(&entity_path));
        let retry_options = self.connection.retry_options().clone();
        let receive_mode = options.receive_mode;
        let prefetch_count = options.prefetch_count;

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

        Ok(ServiceBusSessionReceiver {
            inner,
            entity_path,
            identifier,
            session_id,
        })
    }
}
