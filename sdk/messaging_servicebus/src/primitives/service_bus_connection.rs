use std::borrow::Cow;

use azure_core::Url;

use crate::{
    authorization::{
        service_bus_token_credential::ServiceBusTokenCredential,
        shared_access_credential::SharedAccessCredential,
        shared_access_signature::SharedAccessSignature,
    },
    client::service_bus_client::ServiceBusClientOptions,
    core::TransportClient,
    ServiceBusReceiveMode,
};

use super::{
    error::{Error, ArgumentError}, service_bus_connection_string_properties::ServiceBusConnectionStringProperties,
    service_bus_retry_options::ServiceBusRetryOptions,
    service_bus_transport_type::ServiceBusTransportType,
};

macro_rules! ok_if_not_none_or_empty {
    ($id:expr, $type_name:literal) => {
        match $id {
            Some(s) if s.is_empty() => Err(ArgumentError(format!(
                "{} cannot be empty",
                $type_name
            ))),
            None => Err(ArgumentError(format!(
                "{} cannot be None",
                $type_name
            ))),
            Some(s) => Ok(s),
        }
    };
}

/// Builds the audience of the connection for use in the signature.
pub(crate) fn build_connection_resource(
    transport_type: &ServiceBusTransportType,
    fully_qualified_namespace: Option<&str>,
    entity_name: Option<&str>,
) -> Result<String, Error> {
    match fully_qualified_namespace {
        Some(fqn) => {
            let mut builder = Url::parse(&format!("{}://{}", transport_type.url_scheme(), fqn))?;
            builder.set_path(entity_name.unwrap_or_default());
            builder
                .set_port(None)
                .map_err(|_| ArgumentError("Unable to set port to None".to_string()))?;
            builder.set_fragment(None);
            builder
                .set_password(None)
                .map_err(|_| ArgumentError("Unable to set password to None".to_string()))?;
            builder.set_username("").map_err(|_| {
                ArgumentError("Unable to set username to empty string".to_string())
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
    ) -> Result<C::Receiver, C::CreateReceiverError> {
        let receiver = self
            .inner_client
            .create_receiver(
                entity_path,
                identifier,
                retry_options,
                receive_mode,
                prefetch_count,
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
        session_id: Option<String>,
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
            )
            .await?;

        Ok(receiver)
    }

    pub(crate) async fn create_transport_rule_manager(
        &mut self,
        susbcription_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
    ) -> Result<C::RuleManager, C::CreateRuleManagerError> {
        let rule_manager = self
            .inner_client
            .create_rule_manager(susbcription_path, identifier, retry_options)
            .await?;

        Ok(rule_manager)
    }
}

impl<C> ServiceBusConnection<C>
where
    C: TransportClient,
    Error: From<C::CreateClientError>,
{
    pub(crate) async fn new(
        connection_string: Cow<'_, str>,
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
                let resource = build_connection_resource(
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

impl<C> ServiceBusConnection<C>
where
    C: TransportClient + Send,
{
    pub(crate) async fn new_from_credential(
        fully_qualified_namespace: String,
        credential: impl Into<ServiceBusTokenCredential>,
        options: ServiceBusClientOptions,
    ) -> Result<ServiceBusConnection<C>, C::CreateClientError> {
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

    pub async fn dispose(self) -> Result<(), C::DisposeError> {
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
        return Err(ArgumentError(format!(
            "Missing connection information {}",
            connection_string_argument_name
        )).into());
    }

    // The connection string may contain a precomputed shared access signture OR a shared key name and value
    // but not both.
    if has_shared_key && has_shared_access_signature {
        return Err(ArgumentError(format!("Connection string contains both a shared access signature and a shared key. Only one of these should be present {}", connection_string_argument_name)).into());
    }

    Ok(())
}
