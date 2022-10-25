use std::borrow::Cow;

use azure_core::{auth::TokenCredential, Url};
use fe2o3_amqp::{connection::OpenError, link::SenderAttachError, session::BeginError};
use tokio::time::error::Elapsed;

use crate::{
    amqp::amqp_client::{AmqpClient, AmqpClientError},
    authorization::{
        service_bus_token_credential::ServiceBusTokenCredential,
        shared_access_credential::SharedAccessCredential,
        shared_access_signature::{SasSignatureError, SharedAccessSignature},
    },
    client::service_bus_client_options::ServiceBusClientOptions,
    core::TransportClient,
};

use super::{
    inner_client::InnerClient,
    service_bus_connection_string_properties::{FormatError, ServiceBusConnectionStringProperties},
    service_bus_error::ServiceBusError,
    service_bus_retry_options::ServiceBusRetryOptions,
    service_bus_transport_type::ServiceBusTransportType,
};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
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
    entity_path: Option<String>,
    retry_options: ServiceBusRetryOptions,

    pub(crate) inner_client: C,
}

impl<C: TransportClient> ServiceBusConnection<C> {
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
    pub fn entity_path(&self) -> Option<&str> {
        self.entity_path.as_ref().map(|s| s.as_str())
    }

    /// The endpoint for the Service Bus service to which the connection is associated.
    /// This is essentially the <see cref="FullyQualifiedNamespace"/> but with
    /// the scheme included.
    pub(crate) fn service_endpoint(&self) -> &Url {
        self.inner_client.service_endpoint()
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
        // TODO:
        //
        // if (builder.Path.EndsWith("/", StringComparison.Ordinal))
        // {
        //     builder.Path = builder.Path.TrimEnd('/');
        // }

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

                Ok(builder.to_string().to_lowercase())
            }
            None => Ok(String::new()),
        }
    }
}

impl<TC: TokenCredential> ServiceBusConnection<AmqpClient<TC>> {
    /// The transport type used for this connection.
    pub fn transport_type(&self) -> &ServiceBusTransportType {
        &self.inner_client.transport_type()
    }

    pub(crate) async fn new_with_credential(
        fully_qualified_namespace: String,
        credential: impl Into<ServiceBusTokenCredential<TC>>,
        options: ServiceBusClientOptions,
    ) -> Result<Self, Error> {
        let token_credential: ServiceBusTokenCredential<_> = credential.into();
        let inner_client = AmqpClient::new(
            &fully_qualified_namespace,
            token_credential,
            options.transport_type,
            options.custom_endpoint_address,
            options.retry_options.try_timeout,
        )
        .await?;

        Ok(Self {
            fully_qualified_namespace,
            entity_path: None,
            retry_options: options.retry_options,
            inner_client,
        })
    }
}

impl ServiceBusConnection<AmqpClient<SharedAccessCredential>> {
    pub(crate) async fn new<'a>(
        connection_string: Cow<'a, str>,
        options: ServiceBusClientOptions,
    ) -> Result<Self, Error> {
        let connection_string_properties =
            ServiceBusConnectionStringProperties::parse(connection_string.as_ref())?;

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
        let token_credential: ServiceBusTokenCredential<_> =
            ServiceBusTokenCredential::SharedAccessCredential(shared_access_credential);

        let host = fully_qualified_namespace.unwrap_or("");
        let inner_client = AmqpClient::new(
            host,
            token_credential,
            options.transport_type,
            options.custom_endpoint_address,
            options.retry_options.try_timeout,
        )
        .await?;

        Ok(Self {
            fully_qualified_namespace: host.to_string(),
            entity_path: entity_path.map(|s| s.to_string()),
            retry_options: options.retry_options,
            inner_client,
        })
    }
}
