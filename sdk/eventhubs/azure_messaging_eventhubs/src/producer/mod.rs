// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: ignore retryable

use crate::{
    common::{
        recoverable::{RecoverableConnection, RecoverableSender},
        ManagementInstance,
    },
    error::Result,
    models::{AmqpMessage, EventData, EventHubPartitionProperties, EventHubProperties},
    EventHubsError, RetryOptions,
};
use azure_core::{
    error::{Error, ErrorKind as AzureErrorKind},
    http::Url,
    Uuid,
};
use azure_core_amqp::{
    error::AmqpErrorKind, AmqpError, AmqpSendOptions, AmqpSendOutcome, AmqpSenderApis,
    AmqpTransport,
};
use batch::{EventDataBatch, EventDataBatchOptions};
use std::{fmt::Debug, sync::Arc};
use tracing::{trace, warn};

/// Types used to collect messages into a "batch" before submitting them to an Event Hub.
pub(crate) mod batch;

/// A producer client that buffers events and publishes them in the background.
pub(crate) mod buffered;

pub(crate) const DEFAULT_EVENTHUBS_APPLICATION: &str = "DefaultApplicationName";

#[derive(Default, Debug, Clone)]
/// Represents the options that can be set when submitting a batch of event data.
pub struct SendBatchOptions {}

/// A client that can be used to send events to an Event Hubs instance.
///
/// The [`ProducerClient`] is used to send events to an Event Hub. It can be used to send events to a specific partition
/// or to allow the Event Hubs instance to automatically select the partition.
///
/// The [`ProducerClient`] can be created with the fully qualified namespace of the Event
/// Hubs instance, the name of the Event Hub, and a `TokenCredential` implementation.
///
/// # Examples
///
/// ```no_run
/// use azure_messaging_eventhubs::ProducerClient;
/// use azure_identity::DeveloperToolsCredential;
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
///    let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
///    let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
///    let my_credentials = DeveloperToolsCredential::new(None)?;
///   let producer = ProducerClient::builder()
///    .with_application_id("your_application_id".to_string())
///    .open(&fully_qualified_namespace, &eventhub_name, my_credentials.clone()).await?;
///   Ok(())
/// }
/// ```
pub struct ProducerClient {
    connection: Arc<RecoverableConnection>,
    eventhub: String,
    endpoint: Url,
}

/// Options used when sending an event to an Event Hub.
///
/// The `SendEventOptions` can be used to specify the partition to which the message should be sent.
/// If the partition is not specified, the Event Hub will automatically select a partition.
///
#[derive(Default, Debug)]
pub struct SendEventOptions {
    /// The id of the partition to which the event should be sent.
    pub partition_id: Option<String>,
}

/// Options used when sending an AMQP message to an Event Hub.
/// The `SendMessageOptions` can be used to specify the partition to which the message should be sent.
/// If the partition is not specified, the Event Hub will automatically select a partition.
#[derive(Default, Debug)]
pub struct SendMessageOptions {
    /// The id of the partition to which the message should be sent.
    pub partition_id: Option<String>,
}

impl From<SendEventOptions> for SendMessageOptions {
    fn from(options: SendEventOptions) -> Self {
        Self {
            partition_id: options.partition_id,
        }
    }
}

impl ProducerClient {
    #[allow(clippy::too_many_arguments, reason = "private API")]
    pub(crate) fn new(
        endpoint: Url,
        eventhub: String,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
        application_id: Option<String>,
        retry_options: RetryOptions,
        custom_endpoint: Option<Url>,
        cbs_token_type: Option<&'static str>,
        transport: AmqpTransport,
    ) -> Self {
        Self {
            connection: RecoverableConnection::new(
                endpoint.clone(),
                application_id,
                custom_endpoint,
                transport,
                credential,
                retry_options,
                cbs_token_type,
            ),
            eventhub,
            endpoint,
        }
    }

    /// Returns a builder which can be used to create a new instance of [`ProducerClient`].
    ///
    /// # Arguments
    ///
    /// * `fully_qualified_namespace` - The fully qualified namespace of the Event Hubs instance.
    /// * `eventhub` - The name of the Event Hub.
    /// * `credential` - The token credential used for authorization.
    /// * `options` - The options for configuring the [`ProducerClient`].
    ///
    /// # Returns
    ///
    /// A new instance of [`ProducerClient`].
    pub fn builder() -> builders::ProducerClientBuilder {
        builders::ProducerClientBuilder::new()
    }

    /// Closes the connection to the Event Hub.
    ///
    /// This method should be called when the client is no longer needed, it will terminate all outstanding operations on the connection.
    ///
    /// Call this method to close the connection. Dropping the client is not a
    /// substitute. The client has no `Drop` of its own, so a drop releases only
    /// its reference to the connection. When another handle still holds the
    /// connection, such as a handle that an open operation returned, the drop
    /// does not reach the connection at all. When the drop releases the last
    /// reference, the AMQP layer only asks to close, and it neither waits for
    /// the service to answer nor reports a request that it could not send. A
    /// dropped client can therefore leave the connection open.
    pub async fn close(self) -> Result<()> {
        let connection_id = self.connection.get_connection_id().to_string();
        trace!(
            connection_id = %connection_id,
            url = %self.endpoint,
            "Closing producer client."
        );
        // The close does not need exclusive ownership of the connection. See
        // the note on `ConsumerClient::close`.
        self.connection.close_connection().await?;
        trace!(
            connection_id = %connection_id,
            url = %self.endpoint,
            "Closed producer connection."
        );
        Ok(())
    }

    /// Sends an event to the Event Hub.
    ///
    /// # Arguments
    /// * `event` - The event data to send.
    /// * `options` - The options to use when sending the event.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    ///
    /// Note:
    /// - If the event being sent does not have a message ID, a new message ID will be generated.
    /// - If the event options contain a partition ID, the event will be sent to the specified partition.
    ///
    pub async fn send_event(
        &self,
        event: impl Into<EventData>,
        options: Option<SendEventOptions>,
    ) -> Result<()> {
        let event = event.into();
        let mut message = AmqpMessage::from(event);

        if message.properties.is_none() || message.properties.as_ref().unwrap().message_id.is_none()
        {
            message.set_message_id(Uuid::new_v4());
        }

        self.send_message(message, options.map(SendMessageOptions::from))
            .await
    }

    /// Sends an AMQP message to the Event Hub.
    ///
    /// # Arguments
    /// * `message` - The event to send.
    /// * `options` - The options to use when sending the event.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    ///
    /// Note:
    /// - The message is sent to the service unmodified.
    ///
    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            connection_id = %self.connection.get_connection_id(),
            eventhub = %self.eventhub,
            partition_id = options
                .as_ref()
                .and_then(|o| o.partition_id.as_deref())
                .unwrap_or("<auto>"),
        ),
        err,
    )]
    pub async fn send_message<M>(
        &self,
        message: M,
        options: Option<SendMessageOptions>,
    ) -> Result<()>
    where
        M: Into<AmqpMessage> + Debug + Send,
    {
        let options = options.unwrap_or_default();
        let mut target = self.endpoint.clone();
        if let Some(partition_id) = options.partition_id {
            let target_url = format!("{}/Partitions/{}", self.base_url(), partition_id);
            target = Url::parse(&target_url).map_err(azure_core::Error::from)?;
        }
        let sender = self.connection.get_sender(target.clone()).await?;

        let outcome = sender
            .send(
                message,
                Some(AmqpSendOptions {
                    message_format: None,
                    ..Default::default()
                }),
            )
            .await?;
        match outcome {
            AmqpSendOutcome::Accepted => Ok(()),
            AmqpSendOutcome::Rejected(reason) => {
                if let Some(reason) = reason {
                    warn!(
                        path = %target,
                        condition = ?reason.condition,
                        description = reason.description.as_deref().unwrap_or_default(),
                        "Send was rejected by the Event Hub."
                    );
                    return Err(AmqpError::from(AmqpErrorKind::AmqpDescribedError(reason)).into());
                }
                warn!(
                    path = %target,
                    "Send was rejected by the Event Hub with no described error."
                );
                Err(EventHubsError::with_message(
                    "Send was rejected by the Event Hub.",
                ))
            }
            AmqpSendOutcome::Modified(reason) => {
                // Modified is treated as success (the return type is unchanged), but the
                // message was not durably accepted as sent, so surface it for diagnosis.
                warn!(
                    path = %target,
                    modification = ?reason,
                    "Send was modified by the Event Hub; not durably accepted."
                );
                Ok(())
            }
            AmqpSendOutcome::Released => {
                // Released is treated as success (the return type is unchanged), but the
                // message was released without being durably accepted; surface it.
                warn!(
                    path = %target,
                    "Send was released by the Event Hub; not durably accepted."
                );
                Ok(())
            }
        }
    }

    const BATCH_MESSAGE_FORMAT: u32 = 0x80013700;

    /// Creates a new batch of events to send to the Event Hub.
    /// # Arguments
    ///
    /// * `batch_options` - The options to use when creating the batch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `EventDataBatch`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::ProducerClient;
    /// use azure_identity::DeveloperToolsCredential;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///   let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
    ///   let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///   let my_credentials = DeveloperToolsCredential::new(None)?;
    ///
    ///   let producer = ProducerClient::builder()
    ///    .with_application_id("your_application_id".to_string())
    ///    .open(&fully_qualified_namespace, &eventhub_name, my_credentials.clone()).await?;
    ///   let mut batch = producer.create_batch(None).await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            partition_id = batch_options
                .as_ref()
                .and_then(|o| o.partition_id.as_deref())
                .unwrap_or("<auto>"),
        ),
        err,
    )]
    pub async fn create_batch(
        &self,
        batch_options: Option<EventDataBatchOptions>,
    ) -> Result<EventDataBatch<'_>> {
        let path = EventDataBatch::batch_path(
            self.base_url(),
            batch_options
                .as_ref()
                .and_then(|o| o.partition_id.as_deref()),
        )?;
        let sender = self.ensure_sender(path.clone()).await?;
        let link_max_size = sender.max_message_size().await?.ok_or_else(|| {
            warn!(
                path = %path,
                "The sender link did not report a maximum message size; cannot size the batch."
            );
            Error::with_message(
                AzureErrorKind::Other,
                "No maximum message size available from the sender link.",
            )
        })?;
        let max_size_in_bytes =
            EventDataBatch::resolve_max_size_in_bytes(batch_options.as_ref(), link_max_size)?;

        Ok(EventDataBatch::new(self, batch_options, max_size_in_bytes))
    }

    /// Submits a batch of events to the Event Hub.
    ///
    /// # Arguments
    ///
    /// * `batch` - The batch of events to submit.
    /// * `options` - The options to use when submitting the batch.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::ProducerClient;
    /// use azure_identity::DeveloperToolsCredential;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///   let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
    ///   let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///   let my_credentials = DeveloperToolsCredential::new(None)?;
    ///
    ///   let producer = ProducerClient::builder()
    ///    .with_application_id("your_application_id".to_string())
    ///    .open(&fully_qualified_namespace, &eventhub_name, my_credentials.clone()).await?;
    ///
    ///   let mut batch = producer.create_batch(None).await?;
    ///   batch.try_add_event_data("Hello, World!", None)?;
    ///   producer.send_batch(batch, None).await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            connection_id = %self.connection.get_connection_id(),
            eventhub = %self.eventhub,
        ),
        err,
    )]
    pub async fn send_batch(
        &self,
        batch: EventDataBatch<'_>,
        #[allow(unused_variables)] options: Option<SendBatchOptions>,
    ) -> Result<()> {
        let path = batch.get_batch_path()?;
        let messages = batch.get_messages();
        let outcome = self.send_batch_envelope(path.clone(), messages).await?;
        match outcome {
            AmqpSendOutcome::Accepted => Ok(()),
            AmqpSendOutcome::Rejected(reason) => {
                if let Some(reason) = reason {
                    warn!(
                        path = %path,
                        condition = ?reason.condition,
                        description = reason.description.as_deref().unwrap_or_default(),
                        "Batch was rejected by the Event Hub."
                    );
                    return Err(EventHubsError::from(AmqpError::from(
                        AmqpErrorKind::AmqpDescribedError(reason),
                    )));
                }
                warn!(
                    path = %path,
                    "Batch was rejected by the Event Hub with no described error."
                );
                Err(EventHubsError::with_message(
                    "Batch was rejected by the Event Hub.",
                ))
            }
            AmqpSendOutcome::Modified(reason) => {
                // Modified is treated as success (the return type is unchanged), but the
                // batch was not durably accepted as sent, so surface it for diagnosis.
                warn!(
                    path = %path,
                    modification = ?reason,
                    "Batch was modified by the Event Hub; not durably accepted."
                );
                Ok(())
            }
            AmqpSendOutcome::Released => {
                // Released is treated as success (the return type is unchanged), but the
                // batch was released without being durably accepted; surface it.
                warn!(
                    path = %path,
                    "Batch was released by the Event Hub; not durably accepted."
                );
                Ok(())
            }
        }
    }

    /// Sends a batch envelope to a path and returns the raw AMQP outcome.
    ///
    /// The caller decides what each outcome means. [`ProducerClient::send_batch`]
    /// treats `Modified` and `Released` as success with a warning, for backward
    /// compatibility. The buffered producer treats them as a delivery failure,
    /// because neither outcome means the service stored the events.
    ///
    /// An `Err` from this method means the retry policy is exhausted, or the
    /// error is not retryable. [`RecoverableSender`] applies the retry policy and
    /// the connection recovery, and it converts a `Rejected` outcome into an
    /// error inside the retry loop.
    pub(crate) async fn send_batch_envelope(
        &self,
        path: Url,
        envelope: AmqpMessage,
    ) -> Result<AmqpSendOutcome> {
        let sender = self.connection.get_sender(path).await?;
        let outcome = sender
            .send(
                envelope,
                Some(AmqpSendOptions {
                    message_format: Some(Self::BATCH_MESSAGE_FORMAT),
                    ..Default::default()
                }),
            )
            .await?;
        Ok(outcome)
    }

    /// Gets the properties of the Event Hub.
    /// # Returns
    /// A `Result` containing the properties of the Event Hub.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::ProducerClient;
    /// use azure_identity::DeveloperToolsCredential;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///   let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
    ///   let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///   let my_credentials = DeveloperToolsCredential::new(None)?;
    ///   let producer = ProducerClient::builder()
    ///     .open(&fully_qualified_namespace, &eventhub_name, my_credentials.clone()).await?;
    ///
    ///   let properties = producer.get_eventhub_properties().await?;
    ///   println!("Event Hub: {:?}", properties);
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_eventhub_properties(&self) -> Result<EventHubProperties> {
        self.get_management_instance()
            .await?
            .get_eventhub_properties(&self.eventhub)
            .await
    }

    async fn get_management_instance(&self) -> Result<Arc<ManagementInstance>> {
        Ok(ManagementInstance::new(self.connection.clone()))
    }

    /// Gets the properties of a partition of the Event Hub.
    /// # Arguments
    /// * `partition_id` - The id of the partition.
    /// # Returns
    /// A `Result` containing the properties of the partition.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::ProducerClient;
    /// use azure_identity::DeveloperToolsCredential;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///  let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
    ///     let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///     let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///     let my_credentials = DeveloperToolsCredential::new(None)?;
    ///     let producer = ProducerClient::builder()
    ///        .open(&fully_qualified_namespace, &eventhub_name, my_credentials.clone()).await?;
    ///     let partition_properties = producer.get_partition_properties("0").await?;
    ///     println!("Event Hub: {:?}", partition_properties);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_partition_properties(
        &self,
        partition_id: &str,
    ) -> Result<EventHubPartitionProperties> {
        self.get_management_instance()
            .await?
            .get_eventhub_partition_properties(&self.eventhub, partition_id)
            .await
    }

    /// Forces an error on the connection.
    #[cfg(test)]
    pub fn force_error(&self, error: AmqpError) -> Result<()> {
        self.connection.force_error(error)
    }

    /// Forces the next sender or receiver attach to fail.
    #[cfg(test)]
    pub(crate) fn force_attach_error(&self, error: AmqpError) -> Result<()> {
        self.connection.force_attach_error(error)
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.endpoint
    }

    async fn ensure_sender(&self, target: Url) -> Result<RecoverableSender> {
        self.connection.get_sender(target).await
    }

    pub(crate) async fn max_message_size(
        &self,
        target: Url,
    ) -> azure_core_amqp::Result<Option<u64>> {
        RecoverableSender::new(Arc::downgrade(&self.connection), target)
            .max_message_size()
            .await
    }

    async fn ensure_connection(&self) -> Result<()> {
        self.connection.ensure_connection().await?;
        Ok(())
    }
}

pub mod builders {
    use super::ProducerClient;
    use crate::{
        common::{
            connection_string::{resolve_eventhub, ConnectionString},
            sas_credential::SasCredential,
            SAS_TOKEN_TYPE,
        },
        Result, RetryOptions,
    };
    use azure_core::{http::Url, Error};
    use azure_core_amqp::AmqpTransport;
    use std::sync::Arc;

    /// A builder for creating a [`ProducerClient`].
    ///
    /// This builder is used to create a new [`ProducerClient`] with the specified parameters.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::ProducerClient;
    /// use azure_identity::DeveloperToolsCredential;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let my_credential = DeveloperToolsCredential::new(None).unwrap();
    ///   let producer = ProducerClient::builder()
    ///      .open("my_namespace", "my_eventhub", my_credential).await.unwrap();
    /// }
    /// ```
    #[derive(Default)]
    pub struct ProducerClientBuilder {
        /// The application id that will be used to identify the client.
        application_id: Option<String>,

        /// The options used to configure retry operations.
        retry_options: Option<RetryOptions>,

        /// The custom endpoint for the Event Hub.
        custom_endpoint: Option<String>,

        /// The transport used to communicate with the Event Hub.
        transport: Option<AmqpTransport>,
    }

    impl ProducerClientBuilder {
        ///
        /// # Arguments
        ///
        /// * `fully_qualified_namespace` - The fully qualified namespace of the Event Hubs instance.
        /// * `eventhub` - The name of the Event Hub.
        /// * `credential` - The token credential used for authorization.
        ///
        /// # Returns
        ///
        /// A new instance of [`ProducerClientBuilder`].
        pub(super) fn new() -> Self {
            Self {
                ..Default::default()
            }
        }

        /// Sets the application id that will be used to identify the client.
        pub fn with_application_id(mut self, application_id: String) -> Self {
            self.application_id = Some(application_id);
            self
        }

        /// Sets the options used to configure retry operations.
        ///
        /// # Arguments
        ///
        /// * `retry_options` - The options used to configure retry operations.
        ///
        /// # Returns
        ///
        /// The updated [`ProducerClientBuilder`].
        pub fn with_retry_options(mut self, retry_options: RetryOptions) -> Self {
            self.retry_options = Some(retry_options);
            self
        }

        /// Sets a custom endpoint for the Event Hub.
        ///
        /// # Arguments
        /// * `endpoint` - The custom endpoint for the Event Hub.
        ///
        /// # Returns
        /// The updated [`ProducerClientBuilder`].
        ///
        /// Note: The custom endpoint option allows a customer to specify an AMQP proxy
        /// which will be used to forward requests to the actual Event Hub instance.
        ///
        /// An explicit port on the endpoint carries into the address that the client
        /// dials. Under [`AmqpTransport::WebSocket`] that is the `wss://` address, so
        /// name the port that the proxy accepts WebSockets on, and leave the port out
        /// to dial the default port 443.
        ///
        pub fn with_custom_endpoint(mut self, endpoint: String) -> Self {
            self.custom_endpoint = Some(endpoint);
            self
        }

        /// Sets the transport used to communicate with the Event Hub.
        ///
        /// # Arguments
        /// * `transport` - The transport to use. Defaults to
        ///   [`AmqpTransport::Tcp`]. Use [`AmqpTransport::WebSocket`] to
        ///   tunnel AMQP over WebSockets (port 443) when the native AMQP
        ///   ports are blocked.
        ///
        /// # Returns
        /// The updated [`ProducerClientBuilder`].
        pub fn with_transport(mut self, transport: AmqpTransport) -> Self {
            self.transport = Some(transport);
            self
        }

        /// Returns the AMQP transport this builder opens the connection with.
        /// Shared by every `open` path so they cannot drift apart.
        pub(crate) fn transport(&self) -> AmqpTransport {
            self.transport.unwrap_or_default()
        }

        /// Opens the connection to the Event Hub.
        ///
        /// # Arguments
        /// * `fully_qualified_namespace` - The fully qualified namespace of the Event Hubs instance.
        /// * `eventhub` - The name of the Event Hub.
        /// * `credential` - The token credential to be used for authorization.
        ///
        /// # Returns
        /// A new instance of [`ProducerClient`].
        ///
        pub async fn open(
            self,
            fully_qualified_namespace: &str,
            eventhub: &str,
            credential: Arc<dyn azure_core::credentials::TokenCredential>,
        ) -> Result<ProducerClient> {
            let transport = self.transport();
            let url = format!("amqps://{}/{}", fully_qualified_namespace, eventhub);
            let url = Url::parse(&url).map_err(azure_core::Error::from)?;

            let custom_endpoint = match self.custom_endpoint {
                Some(endpoint) => Some(Url::parse(&endpoint).map_err(Error::from)?),
                None => None,
            };

            let client = ProducerClient::new(
                url.clone(),
                eventhub.to_string(),
                credential,
                self.application_id,
                self.retry_options.unwrap_or_default(),
                custom_endpoint,
                None,
                transport,
            );

            // Open a connection to the Event Hub to ensure that the client is ready to send messages.
            client.ensure_connection().await?;
            Ok(client)
        }

        /// Opens a connection to the Event Hub using a connection string.
        ///
        /// This is an alternative to [`open`](Self::open) for development and
        /// test scenarios that authenticate with a Shared Access Signature
        /// instead of Microsoft Entra ID. For production, prefer
        /// [`open`](Self::open) with a `TokenCredential`.
        ///
        /// When the connection string carries a `SharedAccessKeyName` /
        /// `SharedAccessKey`, the client signs and refreshes SAS tokens itself.
        /// When it carries a pre-formed `SharedAccessSignature`, that token is
        /// used as-is and *cannot* be refreshed (there is no key to re-sign
        /// with); the broker drops the link once the token's own expiry elapses.
        ///
        /// # Arguments
        /// * `connection_string` - An Event Hubs connection string, e.g.
        ///   `Endpoint=sb://<ns>.servicebus.windows.net/;SharedAccessKeyName=<policy>;SharedAccessKey=<key>`.
        ///   It may include an `EntityPath` naming the Event Hub.
        /// * `eventhub` - The Event Hub name. Required unless the connection
        ///   string includes an `EntityPath`; if both are given they must agree.
        ///
        /// # Returns
        /// A new instance of [`ProducerClient`].
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use azure_messaging_eventhubs::ProducerClient;
        ///
        /// #[tokio::main]
        /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
        ///     let connection_string = std::env::var("EVENTHUBS_CONNECTION_STRING")?;
        ///     let producer = ProducerClient::builder()
        ///         .open_with_connection_string(&connection_string, Some("my_eventhub"))
        ///         .await?;
        ///     Ok(())
        /// }
        /// ```
        pub async fn open_with_connection_string(
            self,
            connection_string: &str,
            eventhub: Option<&str>,
        ) -> Result<ProducerClient> {
            let transport = self.transport();
            let connection_string: ConnectionString = connection_string.parse()?;
            let eventhub = resolve_eventhub(&connection_string, eventhub)?;
            let credential = Arc::new(SasCredential::from_connection_string(
                &connection_string,
                &eventhub,
            )?);

            let url = format!(
                "amqps://{}/{}",
                connection_string.fully_qualified_namespace, eventhub
            );
            let url = Url::parse(&url).map_err(Error::from)?;

            let custom_endpoint = match self.custom_endpoint {
                Some(endpoint) => Some(Url::parse(&endpoint).map_err(Error::from)?),
                None => None,
            };

            let client = ProducerClient::new(
                url.clone(),
                eventhub,
                credential,
                self.application_id,
                self.retry_options.unwrap_or_default(),
                custom_endpoint,
                Some(SAS_TOKEN_TYPE),
                transport,
            );

            client.ensure_connection().await?;
            Ok(client)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::tests::force_errors;
    use crate::{models::EventData, EventDataBatchOptions, ProducerClient, Result};
    use azure_core::time::Duration;
    use azure_core_amqp::{error::AmqpErrorKind, AmqpTransport};
    use azure_core_test::{recorded, TestContext};
    use std::sync::Arc;

    // Every `open` path on the builder reads the transport through one helper,
    // so this covers the plumbing that the connection-string path shares.
    #[test]
    fn builder_reads_the_transport_through_one_helper() {
        assert_eq!(
            ProducerClient::builder()
                .with_transport(AmqpTransport::WebSocket)
                .transport(),
            AmqpTransport::WebSocket
        );
        assert_eq!(
            ProducerClient::builder()
                .with_transport(AmqpTransport::Tcp)
                .transport(),
            AmqpTransport::Tcp
        );
        // An unset transport keeps the TCP default.
        assert_eq!(ProducerClient::builder().transport(), AmqpTransport::Tcp);
    }

    #[recorded::test(live)]
    async fn force_errors_send_batch_link_error(ctx: TestContext) -> Result<()> {
        const EVENTHUB_PARTITION: &str = "1";
        const TEST_NAME: &str = "force_errors_send_batch_link_error";
        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();
        let producer = Arc::new(
            ProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );

        force_errors(
            producer.clone(),
            |producer: Arc<ProducerClient>| {
                let producer = producer.clone();
                async move {
                    loop {
                        let batch = producer
                            .create_batch(Some(EventDataBatchOptions {
                                partition_id: Some(EVENTHUB_PARTITION.to_string()),
                                partition_key: Some("My Partition Key.".to_string()),
                                ..Default::default()
                            }))
                            .await
                            .unwrap();

                        for i in 1..200 {
                            assert!(batch
                                .try_add_event_data(
                                    EventData::builder()
                                        .with_body(b"Hello, World!")
                                        .add_property("Message#".to_string(), i)
                                        .with_message_id(i)
                                        .build(),
                                    None
                                )
                                .unwrap());
                        }
                        producer.send_batch(batch, None).await.unwrap()
                    }
                }
            },
            |producer| {
                producer
                    .force_error(azure_core_amqp::AmqpError::from(
                        AmqpErrorKind::LinkClosedByRemote(Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        ))),
                    ))
                    .unwrap();
            },
            Duration::seconds(10), // Seconds until stable state.
            Duration::seconds(30), // Seconds until test timeout.
        )
        .await?;

        Ok(())
    }

    #[recorded::test(live)]
    async fn force_errors_send_batch_session_error(ctx: TestContext) -> Result<()> {
        const EVENTHUB_PARTITION: &str = "1";
        const TEST_NAME: &str = "force_errors_send_batch_session_error";
        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();
        let producer = Arc::new(
            ProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );

        force_errors(
            producer.clone(),
            |producer: Arc<ProducerClient>| {
                let producer = producer.clone();
                async move {
                    loop {
                        let batch = producer
                            .create_batch(Some(EventDataBatchOptions {
                                partition_id: Some(EVENTHUB_PARTITION.to_string()),
                                partition_key: Some("My Partition Key.".to_string()),
                                ..Default::default()
                            }))
                            .await
                            .unwrap();

                        for i in 1..200 {
                            assert!(batch
                                .try_add_event_data(
                                    EventData::builder()
                                        .with_body(b"Hello, World!")
                                        .add_property("Message#".to_string(), i)
                                        .with_message_id(i)
                                        .build(),
                                    None
                                )
                                .unwrap());
                        }
                        producer.send_batch(batch, None).await.unwrap()
                    }
                }
            },
            |producer| {
                producer
                    .force_error(azure_core_amqp::AmqpError::from(
                        AmqpErrorKind::SessionDetachedByRemote(Box::new(
                            azure_core::error::Error::new(
                                azure_core::error::ErrorKind::Other,
                                "Forced error",
                            ),
                        )),
                    ))
                    .unwrap();
            },
            Duration::seconds(10), // Seconds until stable state.
            Duration::seconds(30), // Seconds until test timeout.
        )
        .await?;

        Ok(())
    }

    #[recorded::test(live)]
    async fn force_errors_send_batch_connection_error(ctx: TestContext) -> Result<()> {
        const EVENTHUB_PARTITION: &str = "1";
        const TEST_NAME: &str = "force_errors_send_batch_connection_error";
        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();
        let producer = Arc::new(
            ProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );

        force_errors(
            producer.clone(),
            |producer: Arc<ProducerClient>| {
                let producer = producer.clone();
                async move {
                    loop {
                        let batch = producer
                            .create_batch(Some(EventDataBatchOptions {
                                partition_id: Some(EVENTHUB_PARTITION.to_string()),
                                partition_key: Some("My Partition Key.".to_string()),
                                ..Default::default()
                            }))
                            .await
                            .unwrap();

                        for i in 1..200 {
                            assert!(batch
                                .try_add_event_data(
                                    EventData::builder()
                                        .with_body(b"Hello, World!")
                                        .add_property("Message#".to_string(), i)
                                        .with_message_id(i)
                                        .build(),
                                    None
                                )
                                .unwrap());
                        }
                        producer.send_batch(batch, None).await.unwrap()
                    }
                }
            },
            |producer| {
                producer
                    .force_error(azure_core_amqp::AmqpError::from(
                        AmqpErrorKind::ConnectionClosedByRemote(Box::new(
                            azure_core::error::Error::new(
                                azure_core::error::ErrorKind::Other,
                                "Forced error",
                            ),
                        )),
                    ))
                    .unwrap();
            },
            Duration::seconds(10), // Seconds until forcing the error.
            Duration::seconds(30), // Seconds until test timeout.
        )
        .await?;

        Ok(())
    }

    // Send to a single partition in a tight loop; any error (including a
    // post-reconnect unauthorized / detached error from a stale token) panics the
    // loop and fails the test.
    async fn send_to_partition(producer: Arc<ProducerClient>, partition: &str) {
        loop {
            let batch = producer
                .create_batch(Some(EventDataBatchOptions {
                    partition_id: Some(partition.to_string()),
                    ..Default::default()
                }))
                .await
                .unwrap();
            assert!(batch
                .try_add_event_data(
                    EventData::builder().with_body(b"Hello, World!").build(),
                    None,
                )
                .unwrap());
            producer.send_batch(batch, None).await.unwrap();
        }
    }

    // #4454: after a connection-level reconnect the per-path authorization tokens
    // must be re-established cleanly on the new connection. Sending to several
    // partitions concurrently keeps multiple `authorize_path` re-authorizations in
    // flight across the forced `ConnectionClosedByRemote`, so a token cached against
    // the torn-down connection (the stale-resource race this issue targets) would
    // surface here as an unauthorized / detached error and panic a send loop's
    // `unwrap`. A clean 30s run means every partition re-authorized against the new
    // connection without a second recovery cycle.
    #[recorded::test(live)]
    async fn force_errors_concurrent_authorize_send_reconnect(ctx: TestContext) -> Result<()> {
        const TEST_NAME: &str = "force_errors_concurrent_authorize_send_reconnect";
        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();
        let producer = Arc::new(
            ProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );

        // Derive the partition IDs from the Event Hub rather than hard-coding
        // "0".."3", which would panic on a hub configured with fewer than four
        // partitions. The race this test targets only needs several
        // `authorize_path` re-authorizations in flight at once, so send to up to
        // four of whatever partitions the hub actually exposes.
        let partition_ids = producer.get_eventhub_properties().await?.partition_ids;
        assert!(
            partition_ids.len() >= 2,
            "this test needs at least 2 partitions to keep concurrent authorizations \
             in flight across the reconnect, but the configured Event Hub has {}",
            partition_ids.len()
        );
        let partition_ids: Vec<String> = partition_ids.into_iter().take(4).collect();

        force_errors(
            producer.clone(),
            move |producer: Arc<ProducerClient>| {
                let partition_ids = partition_ids.clone();
                async move {
                    // Run the send loops via `join_all` (not `tokio::spawn`) so they
                    // are cancelled with the test future when `force_errors`'s
                    // timeout arm fires.
                    futures::future::join_all(
                        partition_ids
                            .iter()
                            .map(|partition| send_to_partition(producer.clone(), partition)),
                    )
                    .await;
                }
            },
            |producer| {
                producer
                    .force_error(azure_core_amqp::AmqpError::from(
                        AmqpErrorKind::ConnectionClosedByRemote(Box::new(
                            azure_core::error::Error::new(
                                azure_core::error::ErrorKind::Other,
                                "Forced error",
                            ),
                        )),
                    ))
                    .unwrap();
            },
            Duration::seconds(10), // Seconds until forcing the error.
            Duration::seconds(30), // Seconds until test timeout.
        )
        .await?;

        Ok(())
    }

    #[recorded::test(live)]
    async fn force_errors_producer_properties_connection(ctx: TestContext) -> Result<()> {
        const TEST_NAME: &str = "force_errors_producer_properties_connection";
        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();
        let producer = Arc::new(
            ProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );

        force_errors(
            producer.clone(),
            |producer: Arc<ProducerClient>| {
                let producer = producer.clone();
                async move {
                    loop {
                        producer.get_eventhub_properties().await.unwrap();
                    }
                }
            },
            |producer| {
                producer
                    .force_error(azure_core_amqp::AmqpError::from(
                        AmqpErrorKind::ConnectionClosedByRemote(Box::new(
                            azure_core::error::Error::new(
                                azure_core::error::ErrorKind::Other,
                                "Forced error",
                            ),
                        )),
                    ))
                    .unwrap();
            },
            Duration::seconds(10), // Seconds until forcing the error.
            Duration::seconds(20), // Seconds until test timeout.
        )
        .await?;

        Ok(())
    }

    #[recorded::test(live)]
    async fn force_errors_producer_properties_session(ctx: TestContext) -> Result<()> {
        const TEST_NAME: &str = "force_errors_producer_properties_session";
        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();
        let producer = Arc::new(
            ProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );

        force_errors(
            producer.clone(),
            |producer: Arc<ProducerClient>| {
                let producer = producer.clone();
                async move {
                    loop {
                        producer.get_eventhub_properties().await.unwrap();
                    }
                }
            },
            |producer| {
                producer
                    .force_error(azure_core_amqp::AmqpError::from(
                        AmqpErrorKind::SessionClosedByRemote(Box::new(
                            azure_core::error::Error::new(
                                azure_core::error::ErrorKind::Other,
                                "Forced error",
                            ),
                        )),
                    ))
                    .unwrap();
            },
            Duration::seconds(10), // Seconds until forcing the error.
            Duration::seconds(20), // Seconds until test timeout.
        )
        .await?;

        Ok(())
    }

    #[recorded::test(live)]
    async fn force_errors_producer_properties_link(ctx: TestContext) -> Result<()> {
        const TEST_NAME: &str = "force_errors_producer_properties_link";
        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();
        let producer = Arc::new(
            ProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );

        force_errors(
            producer.clone(),
            |producer: Arc<ProducerClient>| {
                let producer = producer.clone();
                async move {
                    loop {
                        producer.get_eventhub_properties().await.unwrap();
                    }
                }
            },
            |producer| {
                producer
                    .force_error(azure_core_amqp::AmqpError::from(
                        AmqpErrorKind::LinkClosedByRemote(Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        ))),
                    ))
                    .unwrap();
            },
            Duration::seconds(10), // Seconds until forcing the error.
            Duration::seconds(20), // Seconds until test timeout.
        )
        .await?;

        Ok(())
    }
}
