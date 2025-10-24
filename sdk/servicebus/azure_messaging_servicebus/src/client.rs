// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    common::authorizer::Authorizer, ErrorKind, ReceiveMode, Receiver, Result, Sender,
    ServiceBusError,
};
use azure_core::{credentials::TokenCredential, fmt::SafeDebug, http::Url};
use azure_core_amqp::{
    AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions, AmqpOrderedMap, AmqpSymbol,
    AmqpValue,
};
use std::sync::Arc;

/// SubQueue allows you to target a subqueue of a queue or subscription.
///
/// For example, the dead letter queue (SubQueueDeadLetter) or transfer dead letter queue (SubQueueTransfer).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubQueue {
    /// Targets the dead letter queue for a queue or subscription.
    DeadLetter,
    /// Targets the transfer dead letter queue for a queue or subscription.
    Transfer,
}

impl SubQueue {
    /// Returns the path suffix for the sub-queue.
    pub(crate) fn as_path_suffix(&self) -> &'static str {
        match self {
            SubQueue::DeadLetter => "/$DeadLetterQueue",
            SubQueue::Transfer => "/$Transfer/$DeadLetterQueue",
        }
    }
}

/// Options for configuring a Service Bus client.
#[derive(Clone, SafeDebug)]
pub struct ServiceBusClientOptions {
    /// The API version to use when communicating with the Service Bus service.
    pub api_version: String,

    /// Application ID that will be passed to the namespace.
    ///
    /// This optional identifier is passed to the Service Bus namespace during connection establishment
    /// and can be used for diagnostic purposes. It follows the same pattern as the Go SDK's ApplicationID.
    pub application_id: Option<String>,
}

impl Default for ServiceBusClientOptions {
    fn default() -> Self {
        Self {
            api_version: "2021-05".to_string(), // Default Service Bus API version
            application_id: None,
        }
    }
}

/// Options for creating a sender.
#[derive(Clone, Default)]
pub struct CreateSenderOptions; // Place holder for future options

/// Options for creating a receiver.
#[derive(Clone)]
pub struct CreateReceiverOptions {
    /// The receive mode for the receiver.
    pub receive_mode: ReceiveMode,
    /// The sub-queue to target (e.g., dead letter queue).
    pub sub_queue: Option<SubQueue>,
}

impl Default for CreateReceiverOptions {
    fn default() -> Self {
        Self {
            receive_mode: ReceiveMode::PeekLock,
            sub_queue: None,
        }
    }
}

/// A client for interacting with Azure Service Bus.
pub struct ServiceBusClient {
    connection: Arc<AmqpConnection>,
    namespace: String,
    options: ServiceBusClientOptions,
    authorizer: Option<Arc<Authorizer>>,
}

impl ServiceBusClient {
    /// Creates a helper function to build AmqpConnectionOptions from ServiceBusClientOptions.
    fn build_connection_options(
        options: Option<ServiceBusClientOptions>,
    ) -> Option<AmqpConnectionOptions> {
        if let Some(options) = options {
            if let Some(application_id) = &options.application_id {
                let mut properties = AmqpOrderedMap::new();
                properties.insert(
                    AmqpSymbol::from("user-agent"),
                    AmqpValue::from(application_id.clone()),
                );
                Some(AmqpConnectionOptions {
                    properties: Some(properties),
                    ..Default::default()
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    /// Returns a builder which can be used to create a new instance of [`ServiceBusClient`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_servicebus::ServiceBusClient;
    /// use azure_identity::DeveloperToolsCredential;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let credential = DeveloperToolsCredential::new(None)?;
    ///     let client = ServiceBusClient::builder()
    ///         .open("my-servicebus.servicebus.windows.net", credential.clone()).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn builder() -> ServiceBusClientBuilder {
        ServiceBusClientBuilder::new()
    }

    /// Creates a new client internally using the provided options.
    pub(crate) async fn new_internal(
        fully_qualified_namespace: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<ServiceBusClientOptions>,
    ) -> Result<Self> {
        let endpoint = format!("amqps://{}:5671", fully_qualified_namespace);
        let endpoint_url = Url::parse(&endpoint).map_err(|e| {
            ServiceBusError::new(
                ErrorKind::InvalidRequest,
                format!("Invalid endpoint URL: {}", e),
            )
        })?;
        let namespace = fully_qualified_namespace.to_string();

        let connection = Arc::new(AmqpConnection::new());

        // Create authorizer with the credential for token-based authentication
        let authorizer = Arc::new(Authorizer::new(Arc::downgrade(&connection), credential));

        let connection_options = Self::build_connection_options(options.clone());
        connection
            .open(
                "servicebus-client".to_string(),
                endpoint_url,
                connection_options,
            )
            .await?;

        Ok(Self {
            connection,
            namespace,
            options: options.unwrap_or_default(),
            authorizer: Some(authorizer),
        })
    }

    /// Creates a sender for the specified queue or topic.
    pub async fn create_sender(
        &self,
        queue_or_topic_name: &str,
        _options: Option<CreateSenderOptions>,
    ) -> Result<Sender> {
        // Authorize the path if we have a credential-based client
        self.authorize_path(queue_or_topic_name).await?;

        Sender::new(
            self.connection.clone(),
            queue_or_topic_name.to_string(),
            self.options.clone(),
        )
        .await
    }

    /// Creates a receiver for the specified queue with options.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_identity::DeveloperToolsCredential;
    /// use azure_messaging_servicebus::{ServiceBusClient, CreateReceiverOptions, ReceiveMode, SubQueue};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // ... create client ...
    ///     let credential = DeveloperToolsCredential::new(None)?;
    ///     let client = ServiceBusClient::builder()
    ///         .open("my-servicebus.servicebus.windows.net", credential.clone()).await?;
    ///
    ///     // Create receiver with default PeekLock mode
    ///     let receiver = client.create_receiver("my-queue", None).await?;
    ///
    ///     // Create receiver with ReceiveAndDelete mode
    ///     let receiver = client.create_receiver("my-queue", Some(CreateReceiverOptions {
    ///         receive_mode: ReceiveMode::ReceiveAndDelete,
    ///         sub_queue: None,
    ///     })).await?;
    ///
    ///     // Create receiver for dead letter queue
    ///     let receiver = client.create_receiver("my-queue", Some(CreateReceiverOptions {
    ///         receive_mode: ReceiveMode::PeekLock,
    ///         sub_queue: Some(SubQueue::DeadLetter),
    ///     })).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_receiver(
        &self,
        queue_name: &str,
        options: Option<CreateReceiverOptions>,
    ) -> Result<Receiver> {
        // Build the entity path based on sub_queue option
        let options = options.unwrap_or_default();
        let entity_path = if let Some(ref sub_queue) = options.sub_queue {
            format!("{}{}", queue_name, sub_queue.as_path_suffix())
        } else {
            queue_name.to_string()
        };

        // Authorize the path if we have a credential-based client
        self.authorize_path(&entity_path).await?;

        Receiver::new(
            self.connection.clone(),
            entity_path,
            None,
            options.receive_mode,
            self.options.clone(),
        )
        .await
    }

    /// Creates a receiver for the specified topic and subscription with options.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_identity::DeveloperToolsCredential;
    /// use azure_messaging_servicebus::{ServiceBusClient, CreateReceiverOptions, SubQueue};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // ... create client ...
    ///     let credential = DeveloperToolsCredential::new(None)?;
    ///     let client = ServiceBusClient::builder()
    ///         .open("my-servicebus.servicebus.windows.net", credential.clone()).await?;
    ///
    ///     // Create regular subscription receiver
    ///     let receiver = client.create_receiver_for_subscription(
    ///         "my-topic",
    ///         "my-subscription",
    ///         None
    ///     ).await?;
    ///
    ///     // Create receiver for subscription's dead letter queue
    ///     let receiver = client.create_receiver_for_subscription(
    ///         "my-topic",
    ///         "my-subscription",
    ///         Some(CreateReceiverOptions {
    ///             sub_queue: Some(SubQueue::DeadLetter),
    ///             ..Default::default()
    ///         })
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_receiver_for_subscription(
        &self,
        topic_name: &str,
        subscription_name: &str,
        options: Option<CreateReceiverOptions>,
    ) -> Result<Receiver> {
        // For topic subscriptions, the base path is topic/subscriptions/subscription
        let base_path = format!("{}/subscriptions/{}", topic_name, subscription_name);

        // Build the entity path based on sub_queue option
        let options = options.unwrap_or_default();
        let entity_path = if let Some(ref sub_queue) = options.sub_queue {
            format!("{}{}", base_path, sub_queue.as_path_suffix())
        } else {
            base_path
        };

        self.authorize_path(&entity_path).await?;

        Receiver::new(
            self.connection.clone(),
            entity_path,
            None,
            options.receive_mode,
            self.options.clone(),
        )
        .await
    }

    /// Gets the fully qualified namespace.
    pub fn fully_qualified_namespace(&self) -> &str {
        &self.namespace
    }

    /// Authorizes access to a Service Bus entity path using the configured credential.
    /// This method is used internally by senders and receivers when authentication is required.
    pub(crate) async fn authorize_path(&self, entity_path: &str) -> Result<()> {
        if let Some(ref authorizer) = self.authorizer {
            let entity_url = azure_core::http::Url::parse(&format!(
                "amqps://{}:5671/{}",
                self.namespace, entity_path
            ))
            .map_err(azure_core::Error::from)?;

            authorizer
                .authorize_path(&self.connection, &entity_url)
                .await?;
        }
        // If no authorizer is configured (e.g., connection string auth), no additional authorization is needed
        Ok(())
    }

    /// Closes the client and all associated senders and receivers.
    pub async fn close(&self) -> Result<()> {
        self.connection.close().await?;
        Ok(())
    }
}

/// A builder for creating a [`ServiceBusClient`].
///
/// This builder is used to create a new [`ServiceBusClient`] with the specified parameters.
/// It follows the same pattern as other Azure SDK client builders.
///
/// # Examples
///
/// ```no_run
/// use azure_messaging_servicebus::ServiceBusClient;
/// use azure_identity::DeveloperToolsCredential;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let credential = DeveloperToolsCredential::new(None)?;
///     let client = ServiceBusClient::builder()
///         .open("my-servicebus.servicebus.windows.net", credential.clone()).await?;
///     Ok(())
/// }
/// ```
#[derive(Default)]
pub struct ServiceBusClientBuilder {
    /// Application ID for diagnostic purposes.
    application_id: Option<String>,
}

impl ServiceBusClientBuilder {
    /// Creates a new [`ServiceBusClientBuilder`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the application ID for the client.
    ///
    /// This identifier is passed to the Service Bus namespace during connection establishment
    /// and can be used for diagnostic purposes.
    ///
    /// # Arguments
    ///
    /// * `application_id` - A string identifier for the application.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_servicebus::ServiceBusClient;
    /// use azure_identity::DeveloperToolsCredential;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let credential = DeveloperToolsCredential::new(None)?;
    ///     let client = ServiceBusClient::builder()
    ///         .with_application_id("my-application".to_string())
    ///         .open("my-servicebus.servicebus.windows.net", credential.clone()).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn with_application_id(mut self, application_id: String) -> Self {
        self.application_id = Some(application_id);
        self
    }

    /// Opens a connection to the Service Bus namespace.
    ///
    /// # Arguments
    ///
    /// * `fully_qualified_namespace` - The fully qualified namespace of the Service Bus (e.g., "my-servicebus.servicebus.windows.net").
    /// * `credential` - The token credential to be used for authorization.
    ///
    /// # Returns
    ///
    /// A new instance of [`ServiceBusClient`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, CreateSenderOptions, CreateReceiverOptions};
    /// use azure_identity::DeveloperToolsCredential;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let credential = DeveloperToolsCredential::new(None)?;
    ///     let client = ServiceBusClient::builder()
    ///         .open("my-servicebus.servicebus.windows.net", credential.clone()).await?;
    ///
    ///     // Use the client to create senders and receivers
    ///     let sender = client.create_sender("my-queue", None).await?;
    ///     let receiver = client.create_receiver("my-queue", None).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn open(
        self,
        fully_qualified_namespace: &str,
        credential: Arc<dyn TokenCredential>,
    ) -> Result<ServiceBusClient> {
        let options = ServiceBusClientOptions {
            application_id: self.application_id,
            ..Default::default()
        };

        ServiceBusClient::new_internal(fully_qualified_namespace, credential, Some(options)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subqueue_path_suffixes() {
        assert_eq!(SubQueue::DeadLetter.as_path_suffix(), "/$DeadLetterQueue");
        assert_eq!(
            SubQueue::Transfer.as_path_suffix(),
            "/$Transfer/$DeadLetterQueue"
        );
    }

    #[test]
    fn test_create_receiver_options_with_subqueue() {
        // Test default options
        let default_options = CreateReceiverOptions::default();
        assert_eq!(default_options.receive_mode, ReceiveMode::PeekLock);
        assert_eq!(default_options.sub_queue, None);

        // Test options with dead letter queue
        let dlq_options = CreateReceiverOptions {
            receive_mode: ReceiveMode::ReceiveAndDelete,
            sub_queue: Some(SubQueue::DeadLetter),
        };
        assert_eq!(dlq_options.receive_mode, ReceiveMode::ReceiveAndDelete);
        assert_eq!(dlq_options.sub_queue, Some(SubQueue::DeadLetter));

        // Test options with transfer queue
        let transfer_options = CreateReceiverOptions {
            receive_mode: ReceiveMode::PeekLock,
            sub_queue: Some(SubQueue::Transfer),
        };
        assert_eq!(transfer_options.receive_mode, ReceiveMode::PeekLock);
        assert_eq!(transfer_options.sub_queue, Some(SubQueue::Transfer));
    }

    #[test]
    fn test_entity_path_with_subqueue() {
        // Test queue path with dead letter queue
        let queue_name = "my-queue";
        let dlq_path = format!("{}{}", queue_name, SubQueue::DeadLetter.as_path_suffix());
        assert_eq!(dlq_path, "my-queue/$DeadLetterQueue");

        // Test subscription path with dead letter queue
        let topic_name = "my-topic";
        let subscription_name = "my-subscription";
        let base_path = format!("{}/subscriptions/{}", topic_name, subscription_name);
        let sub_dlq_path = format!("{}{}", base_path, SubQueue::DeadLetter.as_path_suffix());
        assert_eq!(
            sub_dlq_path,
            "my-topic/subscriptions/my-subscription/$DeadLetterQueue"
        );

        // Test transfer queue path
        let transfer_path = format!("{}{}", queue_name, SubQueue::Transfer.as_path_suffix());
        assert_eq!(transfer_path, "my-queue/$Transfer/$DeadLetterQueue");
    }

    #[test]
    fn test_servicebus_client_options_with_application_id() {
        let options = ServiceBusClientOptions {
            application_id: Some("test-application".to_string()),
            ..Default::default()
        };

        assert_eq!(options.application_id, Some("test-application".to_string()));
    }

    #[test]
    fn test_servicebus_client_builder_with_application_id() {
        let client_builder =
            ServiceBusClientBuilder::new().with_application_id("my-rust-app".to_string());

        assert_eq!(
            client_builder.application_id,
            Some("my-rust-app".to_string())
        );
    }

    #[test]
    fn test_build_connection_options_with_application_id() {
        let options = ServiceBusClientOptions {
            application_id: Some("test-app-id".to_string()),
            ..Default::default()
        };

        let connection_options = ServiceBusClient::build_connection_options(Some(options));
        assert!(connection_options.is_some());

        let conn_opts = connection_options.unwrap();
        assert!(conn_opts.properties.is_some());

        let properties = conn_opts.properties.unwrap();
        assert_eq!(
            properties.get(&AmqpSymbol::from("user-agent")),
            Some(&AmqpValue::from("test-app-id"))
        );
    }

    #[test]
    fn test_build_connection_options_without_application_id() {
        let options = ServiceBusClientOptions {
            application_id: None,
            ..Default::default()
        };

        let connection_options = ServiceBusClient::build_connection_options(Some(options));
        assert!(connection_options.is_none());
    }

    #[test]
    fn test_build_connection_options_with_none() {
        let connection_options = ServiceBusClient::build_connection_options(None);
        assert!(connection_options.is_none());
    }
}
