use std::borrow::Cow;

use azure_core::Url;
use fe2o3_amqp::{connection::OpenError, link::SenderAttachError, session::BeginError};
use tokio::time::error::Elapsed;

use crate::{
    amqp::{error::{DisposeError, AmqpClientError}},
    authorization::{
        service_bus_token_credential::ServiceBusTokenCredential,
        shared_access_credential::SharedAccessCredential,
        shared_access_signature::{SasSignatureError, SharedAccessSignature},
    },
    client::service_bus_client_options::ServiceBusClientOptions,
    core::TransportClient,
    ServiceBusReceiveMode,
};

use super::{
    service_bus_connection_string_properties::{FormatError, ServiceBusConnectionStringProperties},
    service_bus_retry_options::ServiceBusRetryOptions,
    service_bus_transport_type::ServiceBusTransportType,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Argument error: {}", .0)]
    ArgumentError(String),

    #[error(transparent)]
    FormatError(#[from] FormatError),

    #[error(transparent)]
    SasSignatureError(#[from] SasSignatureError),

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
    ReceiverAttach(#[from] fe2o3_amqp::link::ReceiverAttachError),

    #[error(transparent)]
    Rng(#[from] rand::Error),

    #[error("Cancelled")]
    Cancelled,

    #[error(transparent)]
    Dispose(#[from] DisposeError),
}

impl From<AmqpClientError> for Error {
    fn from(err: AmqpClientError) -> Self {
        match err {
            AmqpClientError::UrlParseError(err) => Self::UrlParseError(err),
            AmqpClientError::Open(err) => Self::Open(err),
            AmqpClientError::WebSocket(err) => Self::WebSocket(err),
            AmqpClientError::TimeoutElapsed(err) => Self::TimeoutElapsed(err),
            AmqpClientError::Begin(err) => Self::Begin(err),
            AmqpClientError::SenderAttach(err) => Self::SenderAttach(err),
            AmqpClientError::Rng(err) => Self::Rng(err),
            AmqpClientError::Cancelled => Self::Cancelled,
            AmqpClientError::Dispose(err) => Self::Dispose(err),
            AmqpClientError::ReceiverAttach(err) => Self::ReceiverAttach(err),
        }
    }
}

macro_rules! ok_if_not_none_or_empty {
    ($id:expr, $type_name:literal) => {
        match $id {
            Some(s) if s.is_empty() => Err(Error::ArgumentError(format!(
                "{} cannot be empty",
                $type_name
            ))),
            None => Err(Error::ArgumentError(format!(
                "{} cannot be None",
                $type_name
            ))),
            Some(s) => Ok(s),
        }
    };
}

/// A connection to the Azure Service Bus service, enabling client communications with a specific
/// Service Bus entity instance within a Service Bus namespace. There is a one-to-one correspondence
/// between [`ServiceBusClient`] and [`ServiceBusConnection`] instances.
#[derive(Debug)]
pub(crate) struct ServiceBusConnection<C> {
    fully_qualified_namespace: String,
    retry_options: ServiceBusRetryOptions,

    pub(crate) inner_client: C,
}

impl<C> ServiceBusConnection<C>
where
    C: TransportClient,
{
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

    /// The retry options associated with this connection.
    pub fn retry_options(&self) -> &ServiceBusRetryOptions {
        &self.retry_options
    }

    /// <summary>
    ///   Builds the audience of the connection for use in the signature.
    /// </summary>
    ///
    /// <param name="transportType">The type of protocol and transport that will be used for communicating with the Service Bus service.</param>
    /// <param name="fullyQualifiedNamespace">The fully qualified Service Bus namespace.  This is likely to be similar to <c>{yournamespace}.servicebus.windows.net</c>.</param>
    /// <param name="entityName">The name of the specific entity to connect the client to.</param>
    ///
    /// <returns>The value to use as the audience of the signature.</returns>
    ///
    fn build_connection_resource(
        transport_type: &ServiceBusTransportType,
        fully_qualified_namespace: Option<&str>,
        entity_name: Option<&str>,
    ) -> Result<String, Error> {
        match fully_qualified_namespace {
            Some(fqn) => {
                let mut builder =
                    Url::parse(&format!("{}://{}", transport_type.url_scheme(), fqn))?;
                builder.set_path(&entity_name.unwrap_or_default());
                builder
                    .set_port(None)
                    .map_err(|_| Error::ArgumentError("Unable to set port to None".to_string()))?;
                builder.set_fragment(None);
                builder.set_password(None).map_err(|_| {
                    Error::ArgumentError("Unable to set password to None".to_string())
                })?;
                builder.set_username("").map_err(|_| {
                    Error::ArgumentError("Unable to set username to empty string".to_string())
                })?;

                // Removes the trailing slash if and only if there is one and it is not the first
                // character
                builder
                    .path_segments_mut()
                    .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
                    .pop_if_empty();

                Ok(builder.to_string().to_lowercase())
            }
            None => Ok(String::new()),
        }
    }
}

impl<C> ServiceBusConnection<C>
where
    C: TransportClient,
{
    /// The transport type used for this connection.
    pub fn transport_type(&self) -> ServiceBusTransportType {
        self.inner_client.transport_type()
    }

    pub(crate) async fn create_transport_sender(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
    ) -> Result<C::Sender, C::CreateSenderError> {
        let sender = self
            .inner_client
            .create_sender(entity_path, identifier, retry_options)
            .await?;

        Ok(sender)
    }

    pub(crate) async fn create_transport_receiver(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
        is_processor: bool,
    ) -> Result<C::Receiver, C::CreateReceiverError> {
        let receiver = self
            .inner_client
            .create_receiver(
                entity_path,
                identifier,
                retry_options,
                receive_mode,
                prefetch_count,
                is_processor,
            )
            .await?;

        Ok(receiver)
    }

    pub(crate) async fn create_transport_session_receiver(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
        session_id: String,
        is_processor: bool,
    ) -> Result<C::SessionReceiver, C::CreateReceiverError> {
        let receiver = self
            .inner_client
            .create_session_receiver(
                entity_path,
                identifier,
                retry_options,
                receive_mode,
                session_id,
                prefetch_count,
                is_processor,
            )
            .await?;

        Ok(receiver)
    }
}

impl<C> ServiceBusConnection<C>
where
    C: TransportClient,
    Error: From<C::CreateClientError>,
{
    pub(crate) async fn new<'a>(
        connection_string: Cow<'a, str>,
        options: ServiceBusClientOptions,
    ) -> Result<Self, Error> {
        let connection_string_properties =
            ServiceBusConnectionStringProperties::parse(connection_string.as_ref())?;
        validate_connection_string_properties(&connection_string_properties, "connection_string")?;

        let fully_qualified_namespace = connection_string_properties
            .endpoint()
            .and_then(|url| url.host_str());
        let entity_path = connection_string_properties.entity_path();

        let shared_access_signature = match connection_string_properties.shared_access_signature() {
            Some(shared_access_signature) => {
                SharedAccessSignature::try_from_signature(shared_access_signature)?
            }
            None => {
                let resource = Self::build_connection_resource(
                    &options.transport_type,
                    fully_qualified_namespace,
                    entity_path,
                )?;
                let shared_access_key_name = ok_if_not_none_or_empty!(
                    connection_string_properties.shared_access_key_name(),
                    "shared_access_key_name"
                )?;
                let shared_access_key = ok_if_not_none_or_empty!(
                    connection_string_properties.shared_access_key(),
                    "shared_access_key"
                )?;
                SharedAccessSignature::try_from_parts(
                    resource,
                    shared_access_key_name,
                    shared_access_key,
                    None,
                )?
            }
        };

        let shared_access_credential =
            SharedAccessCredential::from_signature(shared_access_signature);
        let token_credential: ServiceBusTokenCredential =
            ServiceBusTokenCredential::SharedAccessCredential(shared_access_credential);

        let host = fully_qualified_namespace.unwrap_or("");
        let inner_client = C::create_transport_client(
            host,
            token_credential,
            options.transport_type,
            options.custom_endpoint_address,
            options.retry_options.try_timeout,
        )
        .await?;

        Ok(Self {
            fully_qualified_namespace: host.to_string(),
            retry_options: options.retry_options,
            inner_client,
        })
    }
}

impl<C> ServiceBusConnection<C> {
    pub(crate) async fn new_with_credential(
        fully_qualified_namespace: String,
        credential: impl Into<ServiceBusTokenCredential>,
        options: ServiceBusClientOptions,
    ) -> Result<ServiceBusConnection<C>, Error>
    where
        C: TransportClient,
        Error: From<C::CreateClientError>,
    {
        let token_credential: ServiceBusTokenCredential = credential.into();
        let inner_client = C::create_transport_client(
            &fully_qualified_namespace,
            token_credential,
            options.transport_type,
            options.custom_endpoint_address,
            options.retry_options.try_timeout,
        )
        .await?;

        Ok(ServiceBusConnection {
            fully_qualified_namespace,
            retry_options: options.retry_options,
            inner_client,
        })
    }
}

impl<C> ServiceBusConnection<C>
where
    C: TransportClient + Send,
    Error: From<C::DisposeError>,
{
    // // TODO: expose methods with cancellation token?
    // pub async fn close(&mut self, cancellation_token: CancellationToken) -> Result<(), Error> {
    //     self.inner_client
    //         .close(Some(cancellation_token))
    //         .await
    //         .map_err(Into::into)
    // }

    pub async fn dispose(&mut self) -> Result<(), Error> {
        self.inner_client.dispose().await.map_err(Into::into)
    }
}

fn is_none_or_empty(s: Option<&str>) -> bool {
    match s {
        Some(s) => s.is_empty(),
        None => true,
    }
}

fn validate_connection_string_properties(
    connection_string_properties: &ServiceBusConnectionStringProperties,
    connection_string_argument_name: &str,
) -> Result<(), Error> {
    let has_shared_key = !is_none_or_empty(connection_string_properties.shared_access_key_name())
        && !is_none_or_empty(connection_string_properties.shared_access_key());
    let has_shared_access_signature =
        !is_none_or_empty(connection_string_properties.shared_access_signature());

    // Ensure that each of the needed components are present for connecting
    if is_none_or_empty(
        connection_string_properties
            .endpoint()
            .and_then(|e| e.host_str()),
    ) || (!has_shared_key && !has_shared_access_signature)
    {
        return Err(Error::ArgumentError(format!(
            "Missing connection information {}",
            connection_string_argument_name
        )));
    }

    // The connection string may contain a precomputed shared access signture OR a shared key name and value
    // but not both.
    if has_shared_key && has_shared_access_signature {
        return Err(Error::ArgumentError(format!("Connection string contains both a shared access signature and a shared key. Only one of these should be present {}", connection_string_argument_name)));
    }

    Ok(())
}
