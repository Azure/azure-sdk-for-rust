// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

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
};
use batch::{EventDataBatch, EventDataBatchOptions};
use std::{fmt::Debug, sync::Arc};
use tracing::trace;

/// Types used to collect messages into a "batch" before submitting them to an Event Hub.
pub(crate) mod batch;

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
    pub(crate) fn new(
        endpoint: Url,
        eventhub: String,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
        application_id: Option<String>,
        retry_options: RetryOptions,
        custom_endpoint: Option<Url>,
    ) -> Self {
        Self {
            connection: RecoverableConnection::new(
                endpoint.clone(),
                application_id,
                custom_endpoint,
                credential,
                retry_options,
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
    /// Note that dropping the ProducerClient will also close the connection.
    pub async fn close(self) -> Result<()> {
        trace!("Closing producer client for {}.", self.endpoint);
        Arc::try_unwrap(self.connection)
            .map_err(|_| {
                Error::with_message(
                    AzureErrorKind::Other,
                    "Could not close producer recoverable connection, multiple references exist",
                )
            })?
            .close_connection()
            .await?;
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
        let sender = self.connection.get_sender(target).await?;

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
                trace!("Send was rejected: {:?}", reason);
                if let Some(reason) = reason {
                    return Err(AmqpError::from(AmqpErrorKind::AmqpDescribedError(reason)).into());
                }
                Err(EventHubsError::with_message(
                    "Send was rejected by the Event Hub.",
                ))
            }
            AmqpSendOutcome::Modified(reason) => {
                trace!("Send was modified: {:?}", reason);
                Ok(())
            }
            AmqpSendOutcome::Released => Ok(()),
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
    pub async fn create_batch(
        &self,
        batch_options: Option<EventDataBatchOptions>,
    ) -> Result<EventDataBatch<'_>> {
        let mut batch = EventDataBatch::new(self, batch_options);

        batch.attach().await?;
        Ok(batch)
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
    pub async fn send_batch(
        &self,
        batch: EventDataBatch<'_>,
        #[allow(unused_variables)] options: Option<SendBatchOptions>,
    ) -> Result<()> {
        let sender = self.connection.get_sender(batch.get_batch_path()?).await?;

        let messages = batch.get_messages();
        let outcome = sender
            .send(
                messages,
                Some(AmqpSendOptions {
                    message_format: Some(Self::BATCH_MESSAGE_FORMAT),
                    ..Default::default()
                }),
            )
            .await?;
        match outcome {
            AmqpSendOutcome::Accepted => Ok(()),
            AmqpSendOutcome::Rejected(reason) => {
                trace!("Batch was rejected: {:?}", reason);
                if let Some(reason) = reason {
                    return Err(EventHubsError::from(AmqpError::from(
                        AmqpErrorKind::AmqpDescribedError(reason),
                    )));
                }
                Err(EventHubsError::with_message(
                    "Batch was rejected by the Event Hub.",
                ))
            }
            AmqpSendOutcome::Modified(reason) => {
                trace!("Batch was modified: {:?}", reason);
                Ok(())
            }
            AmqpSendOutcome::Released => Ok(()),
        }
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

    pub(crate) fn base_url(&self) -> &Url {
        &self.endpoint
    }

    async fn ensure_sender(&self, target: Url) -> Result<RecoverableSender> {
        self.connection.get_sender(target).await
    }

    async fn ensure_connection(&self) -> Result<()> {
        self.connection.ensure_connection().await?;
        Ok(())
    }
}

pub mod builders {
    use super::ProducerClient;
    use crate::{Result, RetryOptions};
    use azure_core::{http::Url, Error};
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
        pub fn with_custom_endpoint(mut self, endpoint: String) -> Self {
            self.custom_endpoint = Some(endpoint);
            self
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
            );

            // Open a connection to the Event Hub to ensure that the client is ready to send messages.
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
    use azure_core_amqp::error::AmqpErrorKind;
    use azure_core_test::{recorded, TestContext};
    use std::sync::Arc;

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
