// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    common::{
        user_agent::{get_package_name, get_package_version, get_platform_info, get_user_agent},
        ManagementInstance,
    },
    error::ErrorKind,
    models::{AmqpMessage, EventData, EventHubPartitionProperties, EventHubProperties},
};
use async_std::sync::Mutex;
use azure_core::{
    credentials::AccessToken,
    error::{Error, Result},
    RetryOptions, Uuid,
};
use azure_core_amqp::{
    AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis, AmqpConnection, AmqpConnectionApis,
    AmqpConnectionOptions, AmqpManagement, AmqpManagementApis, AmqpSendOptions, AmqpSender,
    AmqpSenderApis, AmqpSenderOptions, AmqpSession, AmqpSessionApis, AmqpSessionOptions,
    AmqpSymbol, AmqpValue,
};
use batch::{EventDataBatch, EventDataBatchOptions};
use std::sync::{Arc, OnceLock};
use std::{collections::HashMap, fmt::Debug};
use tracing::{debug, trace};
use url::Url;

/// Types used to collect messages into a "batch" before submitting them to an Event Hub.
pub(crate) mod batch;

const DEFAULT_EVENTHUBS_APPLICATION: &str = "DefaultApplicationName";

struct SenderInstance {
    #[allow(dead_code)]
    session: AmqpSession,
    sender: Arc<Mutex<AmqpSender>>,
}

#[derive(Default, Debug, Clone)]
/// Represents the options that can be set when submitting a batch of event data.
pub struct SubmitBatchOptions {}

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
/// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
///    let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
///    let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
///    let my_credentials = DefaultAzureCredential::new()?;
///   let producer = ProducerClient::builder(fully_qualified_namespace, eventhub_name, my_credentials.clone())
///    .with_application_id("your_application_id".to_string())
///    .open().await?;
///   Ok(())
/// }
/// ```
pub struct ProducerClient {
    sender_instances: Mutex<HashMap<String, SenderInstance>>,
    mgmt_client: Mutex<OnceLock<ManagementInstance>>,
    connection: OnceLock<AmqpConnection>,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    eventhub: String,
    url: String,
    authorization_scopes: Mutex<HashMap<String, AccessToken>>,
    /// The application id that will be used to identify the client.
    application_id: Option<String>,

    /// The options used to configure retry operations.
    #[allow(dead_code)]
    retry_options: Option<RetryOptions>,

    /// The maximum size of a message that can be sent to the Event Hub.
    max_message_size: Option<u64>,
}

/// Options used when sending a message to an Event Hub.
///
/// The `SendMessageOptions` can be used to specify the partition to which the message should be sent.
/// If the partition is not specified, the Event Hub will automatically select a partition.
///
#[derive(Default, Debug)]
pub struct SendEventOptions {
    /// The id of the partition to which the message should be sent.
    pub partition_id: Option<String>,
}

/// Options used when sending an AMQP message to an Event Hub.
#[derive(Default, Debug)]
pub struct SendMessageOptions {}

impl ProducerClient {
    pub(crate) fn new(
        url: String,
        eventhub: String,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
        application_id: Option<String>,
        retry_options: Option<RetryOptions>,
        max_message_size: Option<u64>,
    ) -> Self {
        Self {
            sender_instances: Mutex::new(HashMap::new()),
            mgmt_client: Mutex::new(OnceLock::new()),
            connection: OnceLock::new(),
            credential: credential.clone(),
            eventhub,
            url,
            authorization_scopes: Mutex::new(HashMap::new()),
            application_id,
            retry_options,
            max_message_size,
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
    pub fn builder(
        fully_qualified_namespace: String,
        eventhub: String,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
    ) -> builders::ProducerClientBuilder {
        builders::ProducerClientBuilder::new(fully_qualified_namespace, eventhub, credential)
    }

    /// Closes the connection to the Event Hub.
    ///
    /// This method should be called when the client is no longer needed, it will terminate all outstanding operations on the connection.
    ///
    /// Note that dropping the ProducerClient will also close the connection.
    pub async fn close(self) -> Result<()> {
        self.connection
            .get()
            .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingConnection))?
            .close()
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

        if message.properties().is_none() || message.properties().unwrap().message_id.is_none() {
            message.set_message_id(Uuid::new_v4());
        }
        if let Some(options) = options {
            if let Some(partition_id) = options.partition_id {
                message.add_message_annotation(
                    AmqpSymbol::from("x-opt-partition-id"),
                    partition_id.clone(),
                );
            }
        }

        self.send_message(message, None).await
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
    pub async fn send_message(
        &self,
        message: impl Into<AmqpMessage> + Debug,
        #[allow(unused_variables)] options: Option<SendMessageOptions>,
    ) -> Result<()> {
        let sender = self.ensure_sender(self.url.clone()).await.unwrap();

        sender
            .lock()
            .await
            .send(
                message,
                Some(AmqpSendOptions {
                    message_format: None,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
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
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///   let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
    ///   let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///   let my_credentials = DefaultAzureCredential::new()?;
    ///
    ///   let producer = ProducerClient::builder(fully_qualified_namespace, eventhub_name, my_credentials.clone())
    ///    .with_application_id("your_application_id".to_string())
    ///    .open().await?;
    ///   let mut batch = producer.create_batch(None).await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn create_batch(
        &self,
        batch_options: Option<EventDataBatchOptions>,
    ) -> Result<EventDataBatch> {
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
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///   let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
    ///   let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///   let my_credentials = DefaultAzureCredential::new()?;
    ///
    ///   let producer = ProducerClient::builder(fully_qualified_namespace, eventhub_name, my_credentials.clone())
    ///    .with_application_id("your_application_id".to_string())
    ///    .open().await?;
    ///
    ///   let mut batch = producer.create_batch(None).await?;
    ///   batch.try_add_event_data("Hello, World!", None)?;
    ///   producer.submit_batch(&batch, None).await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn submit_batch(
        &self,
        batch: &EventDataBatch<'_>,
        #[allow(unused_variables)] options: Option<SubmitBatchOptions>,
    ) -> Result<()> {
        let sender = self.ensure_sender(batch.get_batch_path()).await?;
        let messages = batch.get_messages();

        sender
            .lock()
            .await
            .send(
                messages,
                Some(AmqpSendOptions {
                    message_format: Some(Self::BATCH_MESSAGE_FORMAT),
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }

    /// Gets the properties of the Event Hub.
    /// # Returns
    /// A `Result` containing the properties of the Event Hub.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::ProducerClient;
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///   let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
    ///   let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///   let my_credentials = DefaultAzureCredential::new()?;
    ///   let producer = ProducerClient::builder(fully_qualified_namespace, eventhub_name, my_credentials.clone())
    ///     .open().await?;
    ///
    ///   let properties = producer.get_eventhub_properties().await?;
    ///   println!("Event Hub: {:?}", properties);
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_eventhub_properties(&self) -> Result<EventHubProperties> {
        self.ensure_management_client().await?;

        self.mgmt_client
            .lock()
            .await
            .get()
            .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingManagementClient))?
            .get_eventhub_properties(self.eventhub.clone())
            .await
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
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///  let fully_qualified_namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
    ///     let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///     let eventhub_name = std::env::var("EVENT_HUB_NAME")?;
    ///     let my_credentials = DefaultAzureCredential::new()?;
    ///     let producer = ProducerClient::builder(fully_qualified_namespace, eventhub_name, my_credentials.clone())
    ///        .open().await?;
    ///     let partition_properties = producer.get_partition_properties("0".to_string()).await?;
    ///     println!("Event Hub: {:?}", partition_properties);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_partition_properties(
        &self,
        partition_id: String,
    ) -> Result<EventHubPartitionProperties> {
        self.ensure_management_client().await?;

        self.mgmt_client
            .lock()
            .await
            .get()
            .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingManagementClient))?
            .get_eventhub_partition_properties(self.eventhub.clone(), partition_id)
            .await
    }

    pub(crate) fn base_url(&self) -> String {
        self.url.clone()
    }

    async fn ensure_management_client(&self) -> Result<()> {
        trace!("Ensure management client.");

        let mgmt_client = self.mgmt_client.lock().await;

        if mgmt_client.get().is_some() {
            trace!("Management client already exists.");
            return Ok(());
        }

        // Clients must call ensure_connection before calling ensure_management_client.
        if self.connection.get().is_none() {
            return Err(ErrorKind::MissingConnection.into());
        }

        trace!("Create management session.");
        let connection = self
            .connection
            .get()
            .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingConnection))?;

        let session = AmqpSession::new();
        session.begin(connection, None).await?;
        trace!("Session created.");

        let management_path = self.url.clone() + "/$management";
        let access_token = self.authorize_path(management_path).await?;

        trace!("Create management client.");
        let management =
            AmqpManagement::new(session, "eventhubs_management".to_string(), access_token)?;
        management.attach().await?;
        mgmt_client
            .set(ManagementInstance::new(management))
            .map_err(|_| azure_core::Error::from(ErrorKind::MissingManagementClient))?;
        trace!("Management client created.");
        Ok(())
    }

    async fn ensure_connection(&self, url: &str) -> Result<()> {
        if self.connection.get().is_none() {
            let connection = AmqpConnection::new();
            connection
                .open(
                    self.application_id
                        .clone()
                        .unwrap_or(Uuid::new_v4().to_string()),
                    Url::parse(url).map_err(Error::from)?,
                    Some(AmqpConnectionOptions {
                        properties: Some(
                            vec![
                                ("user-agent", get_user_agent(&self.application_id)),
                                ("version", get_package_version()),
                                ("platform", get_platform_info()),
                                ("product", get_package_name()),
                            ]
                            .into_iter()
                            .map(|(k, v)| (AmqpSymbol::from(k), AmqpValue::from(v)))
                            .collect(),
                        ),
                        ..Default::default()
                    }),
                )
                .await?;
            self.connection
                .set(connection)
                .map_err(|_| azure_core::Error::from(ErrorKind::MissingConnection))?;
        }
        Ok(())
    }

    async fn ensure_sender(&self, path: String) -> Result<Arc<Mutex<AmqpSender>>> {
        let mut sender_instances = self.sender_instances.lock().await;
        if !sender_instances.contains_key(&path) {
            self.ensure_connection(&path).await?;
            let connection = self
                .connection
                .get()
                .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingConnection))?;

            self.authorize_path(path.clone()).await?;
            let session = AmqpSession::new();
            session
                .begin(
                    connection,
                    Some(AmqpSessionOptions {
                        incoming_window: Some(u32::MAX),
                        outgoing_window: Some(u32::MAX),
                        ..Default::default()
                    }),
                )
                .await?;
            let sender = AmqpSender::new();
            sender
                .attach(
                    &session,
                    format!(
                        "{}-rust-sender",
                        self.application_id
                            .as_ref()
                            .unwrap_or(&DEFAULT_EVENTHUBS_APPLICATION.to_string())
                    ),
                    path.clone(),
                    Some(AmqpSenderOptions {
                        max_message_size: Some(self.max_message_size.unwrap_or(u64::MAX)),
                        ..Default::default()
                    }),
                )
                .await?;
            sender_instances.insert(
                path.clone(),
                SenderInstance {
                    session,
                    sender: Arc::new(Mutex::new(sender)),
                },
            );
        }
        Ok(sender_instances
            .get(&path)
            .ok_or_else(|| Error::from(ErrorKind::MissingMessageSender))?
            .sender
            .clone())
    }

    async fn authorize_path(&self, url: String) -> Result<AccessToken> {
        debug!("Authorizing path: {:?}", url);
        let mut scopes = self.authorization_scopes.lock().await;
        if self.connection.get().is_none() {
            return Err(ErrorKind::MissingConnection.into());
        }
        if !scopes.contains_key(url.as_str()) {
            let connection = self
                .connection
                .get()
                .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingConnection))?;

            // Create an ephemeral session to host the authentication.
            let session = AmqpSession::new();
            session.begin(connection, None).await?;

            let cbs = AmqpClaimsBasedSecurity::new(&session)?;
            cbs.attach().await?;

            debug!("Get Token.");
            let token = self
                .credential
                .get_token(&["https://eventhubs.azure.net/.default"])
                .await?;
            debug!("Got token: {:?}", token.token.secret());
            let expires_at = token.expires_on;
            cbs.authorize_path(
                url.clone(),
                None,
                token.token.secret().to_string(),
                expires_at,
            )
            .await?;
            let present = scopes.insert(url.clone(), token);
            // insert returns some if it *fails* to insert, None if it succeeded.
            if present.is_some() {
                return Err(Error::from(ErrorKind::UnableToAddAuthenticationToken));
            }
        }
        Ok(scopes
            .get(url.as_str())
            .ok_or_else(|| Error::from(ErrorKind::UnableToAddAuthenticationToken))?
            .clone())
    }
}

pub mod builders {
    use super::ProducerClient;
    use azure_core::RetryOptions;
    use std::sync::Arc;

    pub struct ProducerClientBuilder {
        url: String,
        eventhub: String,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
        /// The application id that will be used to identify the client.
        application_id: Option<String>,

        /// The options used to configure retry operations.
        retry_options: Option<RetryOptions>,

        /// The maximum size of a message that can be sent to the Event Hub.
        max_message_size: Option<u64>,
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
        pub fn new(
            fully_qualified_namespace: String,
            eventhub: String,
            credential: Arc<dyn azure_core::credentials::TokenCredential>,
        ) -> Self {
            Self {
                credential: credential.clone(),
                url: format!("amqps://{}/{}", fully_qualified_namespace, eventhub),
                eventhub,
                application_id: None,
                retry_options: None,
                max_message_size: None,
            }
        }

        /// Sets the application id that will be used to identify the client.
        pub fn with_application_id(mut self, application_id: String) -> Self {
            self.application_id = Some(application_id);
            self
        }

        /// Opens the connection to the Event Hub.
        ///
        /// This method must be called before any other operation on the EventHub producer.
        ///
        pub async fn open(self) -> azure_core::Result<ProducerClient> {
            let client = ProducerClient::new(
                self.url.clone(),
                self.eventhub,
                self.credential,
                self.application_id.clone(),
                self.retry_options.clone(),
                self.max_message_size,
            );

            client.ensure_connection(&self.url).await?;
            Ok(client)
        }
    }
}
#[cfg(test)]
mod tests {}
