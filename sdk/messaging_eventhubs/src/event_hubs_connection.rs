use async_trait::async_trait;
use const_format::concatcp;
use std::sync::{atomic::AtomicBool, Arc};
use url::Url;

use crate::{
    amqp::{amqp_client::AmqpClient, error::{AmqpConnectionScopeError, AmqpClientError}, amqp_producer::AmqpProducer, amqp_consumer::AmqpConsumer},
    authorization::{
        event_hub_token_credential::EventHubTokenCredential,
        shared_access_credential::SharedAccessCredential,
        shared_access_signature::SharedAccessSignature, AzureNamedKeyCredential, AzureSasCredential,
    },
    consumer::EventPosition,
    core::{RecoverableTransport, TransportClient, TransportProducerFeatures},
    event_hubs_connection_option::EventHubConnectionOptions,
    event_hubs_connection_string_properties::EventHubsConnectionStringProperties,
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    event_hubs_transport_type::EventHubsTransportType,
    producer::PartitionPublishingOptions,
    util::sharable::Sharable,
    PartitionProperties,
};

/// Error with the `EventHubConnection`.
#[derive(Debug, thiserror::Error)]
pub enum EventHubConnectionError {
    /// The event hub name is not specified.
    #[error("The EventHub name is not specified")]
    EventHubNameIsNotSpecified,
}

impl From<EventHubConnectionError> for azure_core::error::Error {
    fn from(error: EventHubConnectionError) -> Self {
        use azure_core::error::ErrorKind;

        azure_core::Error::new(ErrorKind::Other, error)
    }
}

/// A connection to the Azure Event Hubs service, enabling client communications with a specific
/// Event Hub instance within an Event Hubs namespace.  A single connection may be shared among multiple
/// Event Hub producers and/or consumers, or may be used as a dedicated connection for a single
/// producer or consumer client.
#[derive(Debug)]
pub struct EventHubConnection {
    fully_qualified_namespace: String,
    event_hub_name: Arc<String>,
    is_closed: Arc<AtomicBool>,
    pub(crate) inner: Sharable<AmqpClient>,
}

impl EventHubConnection {
    /// Creates a new [`EventHubConnection`] from a connection string.
    pub async fn new_from_connection_string(
        connection_string: impl AsRef<str>,
        event_hub_name: impl Into<Option<String>>,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        let connection_string_properties =
            EventHubsConnectionStringProperties::parse(connection_string.as_ref())?;

        let event_hub_name =
            match event_hub_name
                .into()
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
            {
                None => connection_string_properties
                    .event_hub_name
                    .map(|s| s.to_string())
                    .ok_or(EventHubConnectionError::EventHubNameIsNotSpecified)?,
                Some(s) => s,
            };

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
            .ok_or_else(|| {
                azure_core::Error::new(
                    azure_core::error::ErrorKind::Credential,
                    "fully_qualified_namespace cannot be None",
                )
            })?;

        let shared_access_signature = if let Some(shared_access_signature) =
            connection_string_properties.shared_access_signature
        {
            SharedAccessSignature::try_from_signature(shared_access_signature)?
        } else {
            let resource = build_connection_signature_authorization_resource(
                options.transport_type,
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
            )?
        };

        let shared_access_credential =
            SharedAccessCredential::from_signature(shared_access_signature);

        let token_credential =
            EventHubTokenCredential::SharedAccessCredential(shared_access_credential);

        Self::new_from_credential(
            fully_qualified_namespace.to_string(),
            event_hub_name,
            token_credential,
            options,
        )
        .await
    }

    /// Creates a new [`EventHubConnection`] from a connection string.
    #[deprecated(
        since = "0.14.1",
        note = "Please use `EventHubConnection::new_from_connection_string` instead"
    )]
    pub async fn from_connection_string(
        connection_string: impl AsRef<str>,
        event_hub_name: impl Into<Option<String>>,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_connection_string(connection_string, event_hub_name, options).await
    }

    /// Creates a new [`EventHubConnection`] from a namespace and a credential.
    pub async fn new_from_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        let fully_qualified_namespace = fully_qualified_namespace.into();
        let event_hub_name = event_hub_name.into();
        let token_credential = credential.into();
        let event_hub_name = Arc::new(event_hub_name);

        let inner_client = AmqpClient::new(
            &fully_qualified_namespace,
            event_hub_name.clone(),
            token_credential,
            options,
        )
        .await
        .map_err(<AmqpClientError as Into<azure_core::Error>>::into)?;
        let is_closed = inner_client.connection_scope.is_disposed.clone();
        let inner = Sharable::Owned(inner_client);

        Ok(Self {
            fully_qualified_namespace,
            event_hub_name,
            is_closed,
            inner,
        })
    }

    /// Creates a new [`EventHubConnection`] from a namespace and a credential.
    #[deprecated(
        since = "0.14.1",
        note = "Please use `EventHubConnection::new_from_credential` instead"
    )]
    pub async fn from_namespace_and_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_credential(fully_qualified_namespace, event_hub_name, credential, options).await
    }

    /// Creates a new [`EventHubConnection`] from a namespace and a [`AzureNamedKeyCredential`].
    pub async fn new_from_named_key_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureNamedKeyCredential,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        let fully_qualified_namespace = fully_qualified_namespace.into();
        let event_hub_name = event_hub_name.into();
        let resource = build_connection_signature_authorization_resource(options.transport_type, &fully_qualified_namespace, &event_hub_name)?;
        let shared_access_credential = SharedAccessCredential::try_from_named_key_credential(credential, resource)?;

        Self::new_from_credential(
            fully_qualified_namespace,
            event_hub_name,
            shared_access_credential,
            options,
        ).await
    }

    /// Creates a new [`EventHubConnection`] from a namespace and a [`AzureNamedKeyCredential`].
    #[deprecated(
        since = "0.14.1",
        note = "Please use `EventHubConnection::new_from_named_key_credential` instead"
    )]
    pub async fn from_namespace_and_named_key_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureNamedKeyCredential,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_named_key_credential(fully_qualified_namespace, event_hub_name, credential, options).await
    }

    /// Creates a new [`EventHubConnection`] from a namespace and a [`AzureSasCredential`].
    pub async fn new_from_sas_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureSasCredential,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        let shared_access_credential = SharedAccessCredential::try_from_sas_credential(credential)?;
        Self::new_from_credential(
            fully_qualified_namespace,
            event_hub_name,
            shared_access_credential,
            options,
        ).await
    }

    /// Creates a new [`EventHubConnection`] from a namespace and a [`AzureSasCredential`].
    #[deprecated(
        since = "0.14.1",
        note = "Please use `EventHubConnection::new_from_sas_credential` instead"
    )]
    pub async fn from_namespace_and_sas_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureSasCredential,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_sas_credential(fully_qualified_namespace, event_hub_name, credential, options).await
    }
}

impl EventHubConnection {
    pub(crate) async fn get_properties<RP>(
        &mut self,
        retry_policy: RP,
    ) -> Result<EventHubProperties, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        match &mut self.inner {
            Sharable::Owned(c) => c.get_properties(retry_policy).await,
            Sharable::Shared(c) => c.lock().await.get_properties(retry_policy).await,
            Sharable::None => Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Io,
                AmqpConnectionScopeError::ScopeDisposed,
            )),
        }
    }

    pub(crate) async fn get_partition_ids<RP>(
        &mut self,
        retry_policy: RP,
    ) -> Result<Vec<String>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let properties = self.get_properties(retry_policy).await?;
        Ok(properties.partition_ids)
    }

    pub(crate) async fn get_partition_properties<RP>(
        &mut self,
        partition_id: &str,
        retry_policy: RP,
    ) -> Result<PartitionProperties, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        match &mut self.inner {
            Sharable::Owned(c) => c.get_partition_properties(partition_id, retry_policy).await,
            Sharable::Shared(c) => {
                c.lock()
                    .await
                    .get_partition_properties(partition_id, retry_policy)
                    .await
            }
            Sharable::None => Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Io,
                AmqpConnectionScopeError::ScopeDisposed,
            )),
        }
    }

    pub(crate) async fn create_transport_producer<RP>(
        &mut self,
        partition_id: Option<String>,
        producer_identifier: Option<String>,
        requested_features: TransportProducerFeatures,
        partition_options: PartitionPublishingOptions,
        retry_policy: RP,
    ) -> Result<AmqpProducer<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        match &mut self.inner {
            Sharable::Owned(c) => c
                .create_producer(
                    partition_id,
                    producer_identifier,
                    requested_features,
                    partition_options,
                    retry_policy,
                )
                .await
                .map_err(Into::into),
            Sharable::Shared(c) => c
                .lock()
                .await
                .create_producer(
                    partition_id,
                    producer_identifier,
                    requested_features,
                    partition_options,
                    retry_policy,
                )
                .await
                .map_err(Into::into),
            Sharable::None => Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Io,
                AmqpConnectionScopeError::ScopeDisposed,
            )),
        }
    }

    #[allow(clippy::too_many_arguments)] // TODO: how to reduce the number of arguments?
    pub(crate) async fn create_transport_consumer<RP>(
        &mut self,
        consumer_group: &str,
        partition_id: &str,
        consumer_identifier: Option<String>,
        event_position: EventPosition,
        retry_policy: RP,
        track_last_enqueued_event_properties: bool,
        owner_level: Option<i64>,
        prefetch_count: Option<u32>,
    ) -> Result<AmqpConsumer<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        match &mut self.inner {
            Sharable::Owned(c) => c
                .create_consumer(
                    consumer_group,
                    partition_id,
                    consumer_identifier,
                    event_position,
                    retry_policy,
                    track_last_enqueued_event_properties,
                    owner_level,
                    prefetch_count,
                )
                .await
                .map_err(Into::into),
            Sharable::Shared(c) => c
                .lock()
                .await
                .create_consumer(
                    consumer_group,
                    partition_id,
                    consumer_identifier,
                    event_position,
                    retry_policy,
                    track_last_enqueued_event_properties,
                    owner_level,
                    prefetch_count,
                )
                .await
                .map_err(Into::into),
            Sharable::None => Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Io,
                AmqpConnectionScopeError::ScopeDisposed,
            )),
        }
    }

    /// Closes the inner client regardless of whether it is owned or shared.
    pub async fn close(self) -> Result<(), azure_core::Error> {
        match self.inner {
            Sharable::Owned(mut c) => c
                .close()
                .await
                .map_err(Into::into),
            Sharable::Shared(c) => c
                .lock()
                .await
                .close()
                .await
                .map_err(Into::into),
            Sharable::None => Ok(()),
        }
    }

    /// Closes the inner client if it is owned or if it is shared and this is the last reference to
    /// it.
    pub async fn close_if_owned(self) -> Result<(), azure_core::Error> {
        match self.inner {
            Sharable::Owned(mut client) => client
                .close()
                .await
                .map_err(Into::into),
            Sharable::Shared(client) => match Arc::try_unwrap(client) {
                Ok(mut client) => {
                    // This is the last reference to the client, so we can dispose it.
                    client
                        .get_mut()
                        .close()
                        .await
                        .map_err(Into::into)
                }
                Err(_) => {
                    // This is not the last reference to the client, so we cannot dispose it.
                    Ok(())
                }
            },
            Sharable::None => Ok(()),
        }
    }
}

impl EventHubConnection {
    pub(crate) fn clone_as_shared(&mut self) -> Self {
        let shared = self.inner.clone_as_shared();
        let inner = match shared {
            Some(shared) => Sharable::Shared(shared),
            None => Sharable::None,
        };

        Self {
            fully_qualified_namespace: self.fully_qualified_namespace.clone(),
            event_hub_name: self.event_hub_name.clone(),
            is_closed: self.is_closed.clone(),
            inner,
        }
    }

    /// The fully qualified namespace that the connection is associated with.
    pub fn fully_qualified_namespace(&self) -> &str {
        &self.fully_qualified_namespace
    }

    /// The name of the event hub that the connection is associated with.
    pub fn event_hub_name(&self) -> &str {
        &self.event_hub_name
    }

    /// Returns true if the connection is closed.
    pub fn is_closed(&self) -> bool {
        matches!(self.inner, Sharable::None)
            | self.is_closed.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns true if the connection is owned.
    ///
    /// This will return false even if it is the last reference to the shared connection.
    pub fn is_owned(&self) -> bool {
        matches!(self.inner, Sharable::Owned(_))
    }

    /// Returns true if the connection is shared.
    ///
    /// This will return true even if it is the last reference to the shared connection.
    pub fn is_shared(&self) -> bool {
        matches!(self.inner, Sharable::Shared(_))
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
        return Err(FormatError::ConnectionStringIsEmpty.into());
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

#[async_trait]
impl RecoverableTransport for EventHubConnection {
    type RecoverError = azure_core::Error;

    async fn recover(&mut self) -> Result<(), Self::RecoverError> {
        self.inner
            .recover()
            .await
            .map_err(Into::into)
    }
}
