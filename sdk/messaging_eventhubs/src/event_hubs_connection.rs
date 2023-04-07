use const_format::concatcp;
use std::sync::{atomic::AtomicBool, Arc};
use url::Url;

use tokio::sync::Mutex;

use crate::{
    amqp::amqp_client::AmqpClient,
    authorization::{
        event_hub_token_credential::EventHubTokenCredential,
        shared_access_credential::SharedAccessCredential,
        shared_access_signature::SharedAccessSignature,
    },
    event_hubs_connection_option::EventHubConnectionOptions,
    event_hubs_connection_string_properties::EventHubsConnectionStringProperties,
    event_hubs_transport_type::EventHubsTransportType,
    util::IntoAzureCoreError, core::transport_client::TransportClient,
};

#[derive(Debug, thiserror::Error)]
pub enum EventHubConnectionError {
    #[error("The EventHub name is not specified")]
    EventHubNameIsNotSpecified,
}

impl IntoAzureCoreError for EventHubConnectionError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        azure_core::Error::new(ErrorKind::Other, self)
    }
}

pub struct EventHubConnection<C> {
    fully_qualified_namespace: String,
    event_hub_name: Arc<String>,
    is_closed: Arc<AtomicBool>,
    inner: InnerClient<C>,
}

pub(crate) enum InnerClient<C> {
    Owned(C),
    Shared(Arc<Mutex<C>>),
}

impl EventHubConnection<AmqpClient> {
    pub async fn new(
        connection_string: String,
        mut event_hub_name: String,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        let connection_string_properties =
            EventHubsConnectionStringProperties::parse(&connection_string)
                .map_err(IntoAzureCoreError::into_azure_core_error)?;
        if event_hub_name.is_empty() {
            event_hub_name = connection_string_properties
                .event_hub_name
                .map(|s| s.to_string())
                .ok_or(EventHubConnectionError::EventHubNameIsNotSpecified)
                .map_err(IntoAzureCoreError::into_azure_core_error)?;
        }

        macro_rules! ok_if_not_none_or_empty {
            ($id:expr, $type_name:literal) => {
                match $id {
                    Some(s) if s.is_empty() => Err(azure_core::Error::new(
                        azure_core::error::ErrorKind::Credential,
                        concatcp!("{} cannot be empty", $type_name),
                    )),
                    None => Err(azure_core::Error::new(
                        azure_core::error::ErrorKind::Credential,
                        concatcp!("{} cannot be None", $type_name),
                    )),
                    Some(s) => Ok(s),
                }
            };
        }

        let fully_qualified_namespace = connection_string_properties
            .fully_qualified_namespace()
            .ok_or(azure_core::Error::new(
                azure_core::error::ErrorKind::Credential,
                "fully_qualified_namespace cannot be None",
            ))?;

        let shared_access_signature = if let Some(shared_access_signature) =
            connection_string_properties.shared_access_signature
        {
            SharedAccessSignature::try_from_signature(shared_access_signature)
                .map_err(IntoAzureCoreError::into_azure_core_error)?
        } else {
            let resource = build_connection_signature_authorization_resource(
                options.transport_type(),
                fully_qualified_namespace,
                &event_hub_name,
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
            )
            .map_err(IntoAzureCoreError::into_azure_core_error)?
        };

        let shared_access_credential =
            SharedAccessCredential::from_signature(shared_access_signature);

        let token_credential =
            EventHubTokenCredential::SharedAccessCredential(shared_access_credential);

        Self::new_with_credential(
            fully_qualified_namespace.to_string(),
            event_hub_name,
            token_credential,
            options,
        )
        .await
    }

    pub async fn new_with_credential(
        fully_qualified_namespace: String,
        event_hub_name: String,
        credential: impl Into<EventHubTokenCredential>,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        let token_credential = credential.into();
        let event_hub_name = Arc::new(event_hub_name);

        let inner_client = AmqpClient::new(
            &fully_qualified_namespace,
            event_hub_name.clone(),
            token_credential,
            options,
        )
        .await?;
        let is_closed = inner_client.connection_scope.is_disposed.clone();
        let inner = InnerClient::Owned(inner_client);

        Ok(Self {
            fully_qualified_namespace,
            event_hub_name,
            is_closed,
            inner,
        })
    }
}

impl<C> EventHubConnection<C>
where
    C: TransportClient,
    C::DisposeError: IntoAzureCoreError,
{
    pub async fn close(self) -> Result<(), azure_core::Error> {
        match self.inner {
            InnerClient::Owned(mut client) => client.close().await.map_err(IntoAzureCoreError::into_azure_core_error),
            InnerClient::Shared(client) => match Arc::try_unwrap(client) {
                Ok(mut client) => {
                    // This is the last reference to the client, so we can dispose it.
                    client.get_mut().close().await.map_err(IntoAzureCoreError::into_azure_core_error)
                },
                Err(_) => {
                    // This is not the last reference to the client, so we cannot dispose it.
                    Ok(())
                },
            },
        }
    }
}

impl<C> EventHubConnection<C> {
    pub fn fully_qualified_namespace(&self) -> &str {
        &self.fully_qualified_namespace
    }

    pub fn event_hub_name(&self) -> &str {
        &self.event_hub_name
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.load(std::sync::atomic::Ordering::Relaxed)
    }
}

// internal static string BuildConnectionSignatureAuthorizationResource(EventHubsTransportType transportType,
//     string fullyQualifiedNamespace,
//     string eventHubName)
fn build_connection_signature_authorization_resource(
    transport_type: EventHubsTransportType,
    fully_qualified_namespace: &str,
    event_hub_name: &str,
) -> Result<String, azure_core::Error> {
    use crate::event_hubs_connection_string_properties::FormatError;
    use azure_core::error::ErrorKind;

    // If there is no namespace, there is no basis for a URL and the
    // resource is empty.

    if fully_qualified_namespace.is_empty() {
        return Err(FormatError::ConnectionStringIsEmpty.into_azure_core_error());
    }

    // Form a normalized URI to identify the resource.

    let mut builder = Url::parse(&format!(
        "{}://{}",
        transport_type.url_scheme(),
        fully_qualified_namespace
    ))?;
    builder.set_path(event_hub_name);
    builder
        .set_port(None)
        .map_err(|_| azure_core::Error::new(ErrorKind::Other, "Unable to set port to None"))?;
    builder.set_fragment(None);
    builder.set_password(None).map_err(|_| {
        azure_core::Error::new(
            ErrorKind::Other,
            "Unable to set password to None".to_string(),
        )
    })?;
    builder.set_username("").map_err(|_| {
        azure_core::Error::new(
            ErrorKind::Other,
            "Unable to set username to empty string".to_string(),
        )
    })?;

    // Removes the trailing slash if and only if there is one and it is not the first
    // character
    builder
        .path_segments_mut()
        .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
        .pop_if_empty();

    Ok(builder.to_string().to_lowercase())
}
