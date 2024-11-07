// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//cspell: words amqp mgmt amqps

#![doc = include_str!("README.md")]
use super::{
    common::{
        user_agent::{get_package_name, get_package_version, get_platform_info, get_user_agent},
        ManagementInstance,
    },
    error::ErrorKind,
    models::{EventHubPartitionProperties, EventHubProperties, ReceivedEventData},
};

use async_std::sync::Mutex;
use async_stream::try_stream;
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::{Error, Result},
    RetryOptions,
};
use azure_core_amqp::{
    cbs::{AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis},
    connection::{AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions},
    management::{AmqpManagement, AmqpManagementApis},
    messaging::{AmqpSource, AmqpSourceFilter},
    receiver::{AmqpReceiver, AmqpReceiverApis, AmqpReceiverOptions, ReceiverCreditMode},
    session::{AmqpSession, AmqpSessionApis},
    value::AmqpDescribed,
};
use futures::stream::Stream;
use std::{
    collections::HashMap,
    default::Default,
    fmt::Debug,
    sync::{Arc, OnceLock},
};
use tracing::{debug, trace};
use url::Url;

/// A client that can be used to receive events from an Event Hub.
pub struct ConsumerClient {
    options: ConsumerClientOptions,
    session_instances: Mutex<HashMap<String, Arc<AmqpSession>>>,
    mgmt_client: Mutex<OnceLock<ManagementInstance>>,
    connection: OnceLock<AmqpConnection>,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    eventhub: String,
    url: String,
    authorization_scopes: Mutex<HashMap<String, AccessToken>>,
}
impl ConsumerClient {
    /// Creates a new `ConsumerClient` instance.
    ///
    /// This function creates a new `ConsumerClient` instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `fully_qualified_namespace` - The fully qualified namespace of the Event Hubs instance.
    /// * `eventhub_name` - The name of the Event Hub.
    /// * `consumer_group` - Optional consumer group name. If not provided, the default consumer group will be used.
    /// * `credential` - The token credential used to authenticate with the Event Hubs service.
    /// * `options` - Optional `ConsumerClientOptions` to configure the behavior of the consumer client.
    ///
    /// # Returns
    ///
    /// A new `ConsumerClient` instance.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use azure_messaging_eventhubs::consumer::ConsumerClient;
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    ///
    ///     let my_credential = DefaultAzureCredential::new()?;
    /// let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
    /// # Ok(())}
    /// ```
    pub fn new(
        fully_qualified_namespace: String,
        eventhub_name: String,
        consumer_group: Option<String>,
        credential: Arc<dyn TokenCredential>,
        options: Option<ConsumerClientOptions>,
    ) -> Self {
        let consumer_group = consumer_group.unwrap_or("$Default".into());
        let url = format!(
            "amqps://{}/{}/ConsumerGroups/{}",
            fully_qualified_namespace, eventhub_name, consumer_group
        );
        Self {
            options: options.unwrap_or_default(),
            session_instances: Mutex::new(HashMap::new()),
            mgmt_client: Mutex::new(OnceLock::new()),
            connection: OnceLock::new(),
            credential,
            eventhub: eventhub_name,
            url,
            authorization_scopes: Mutex::new(HashMap::new()),
        }
    }

    /// Opens a connection to the Event Hub.
    ///
    /// This method establishes a connection to the Event Hub associated with the `ConsumerClient`.
    /// It returns a `Result` indicating whether the operation was successful or not.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the operation was successful or not.
    ///
    /// # Example
    ///
    /// ```
    /// use azure_messaging_eventhubs::consumer::ConsumerClient;
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let my_credential = DefaultAzureCredential::new().unwrap();
    ///     let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
    ///
    ///     let result = consumer.open().await;
    ///
    ///     match result {
    ///         Ok(()) => {
    ///             // Connection opened successfully
    ///             println!("Connection opened successfully");
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error opening connection: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn open(&self) -> Result<()> {
        self.ensure_connection(&self.url).await?;
        Ok(())
    }

    /// Closes the connection to the Event Hub.
    ///
    /// This method closes the connection to the Event Hub associated with the `ConsumerClient`.
    /// It returns a `Result` indicating whether the operation was successful or not.
    ///
    /// Note that closing a consumer will cancel all outstanding receive requests.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the operation was successful or not.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use azure_messaging_eventhubs::consumer::ConsumerClient;
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let my_credential = DefaultAzureCredential::new().unwrap();
    ///     let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
    ///
    ///     consumer.open().await.unwrap();
    ///
    ///     let result = consumer.close().await;
    ///
    ///     match result {
    ///         Ok(()) => {
    ///             // Connection closed successfully
    ///             println!("Connection closed successfully");
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error closing connection: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn close(self) -> Result<()> {
        self.connection
            .get()
            .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingConnection))?
            .close()
            .await?;
        Ok(())
    }

    /// Receives events from a specific partition of the Event Hub.
    ///
    /// This function establishes a connection to the specified partition of the Event Hub and starts receiving events from it.
    /// It returns a stream of `ReceivedEventData` items, representing the events received from the partition.
    ///
    /// # Arguments
    ///
    /// * `partition_id` - The ID of the partition to receive events from.
    /// * `options` - Optional `ReceiveOptions` to configure the behavior of the receiver.
    ///
    /// # Returns
    ///
    /// A stream of `Result<ReceivedEventData>`, where each item represents an event received from the partition.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::consumer::ConsumerClient;
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    /// use async_std::stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let my_credential = DefaultAzureCredential::new().unwrap();
    ///     let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
    ///     let partition_id = "0";
    ///     let options = None;
    ///
    ///     consumer.open().await.unwrap();
    ///
    ///     let event_stream = consumer.receive_events_on_partition(partition_id.to_string(), options).await;
    ///
    ///     tokio::pin!(event_stream);
    ///     while let Some(event_result) = event_stream.next().await {
    ///         match event_result {
    ///             Ok(event) => {
    ///                 // Process the received event
    ///                 println!("Received event: {:?}", event);
    ///             }
    ///             Err(err) => {
    ///                 // Handle the error
    ///                 eprintln!("Error receiving event: {:?}", err);
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn receive_events_on_partition(
        &self,
        partition_id: String,
        options: Option<ReceiveOptions>,
    ) -> impl Stream<Item = Result<ReceivedEventData>> + '_ {
        let options = options.unwrap_or_default();

        let receiver_name = self
            .options
            .instance_id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let start_expression = StartPosition::start_expression(&options.start_position);
        let source_url = format!("{}/Partitions/{}", self.url, partition_id);

        try_stream! {
            // Authorize access to the source address.
        self.authorize_path(source_url.clone()).await?;

        let session = self.get_session(&partition_id).await?;
        let message_source = AmqpSource::builder()
            .with_address(source_url)
            .add_to_filter(
                AmqpSourceFilter::selector_filter().description().into(),
                Box::new(AmqpDescribed::new(
                    AmqpSourceFilter::selector_filter().code(),
                    start_expression,
                )),
            )
            .build();

        let mut receiver_options_builder = AmqpReceiverOptions::builder()
            .with_name(receiver_name.clone())
            .add_property("com.microsoft.com:receiver-name".into(), receiver_name)
            .with_credit_mode(ReceiverCreditMode::Auto(options.prefetch.unwrap_or(300)))
            .with_auto_accept(true);

        if let Some(owner_level) = options.owner_level {
            receiver_options_builder = receiver_options_builder.add_property("com.microsoft:epoch".into(), owner_level);
        }

        let receiver = AmqpReceiver::new();
        receiver
            .attach(
                &session,
                message_source,
                Some(receiver_options_builder.build()),
            )
            .await?;

            loop{
                let message = receiver.receive().await?;
                let event: ReceivedEventData= message.into();
                yield event;
            }
        }
    }

    /// Retrieves the properties of the Event Hub.
    ///
    /// This function retrieves the properties of the Event Hub associated with the `ConsumerClient`.
    /// It returns a `Result` containing the `EventHubProperties` if the operation is successful.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `EventHubProperties` if the operation is successful.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use azure_messaging_eventhubs::consumer::ConsumerClient;
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let my_credential = DefaultAzureCredential::new().unwrap();
    ///     let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
    ///
    ///     let eventhub_properties = consumer.get_eventhub_properties().await;
    ///
    ///     match eventhub_properties {
    ///         Ok(properties) => {
    ///             // Process the Event Hub properties
    ///             println!("Event Hub properties: {:?}", properties);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error retrieving Event Hub properties: {:?}", err);
    ///         }
    ///     }
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

    /// Retrieves the properties of a specific partition in the Event Hub.
    ///
    /// This function retrieves the properties of the specified partition in the Event Hub.
    /// It returns a `Result` containing the `EventHubPartitionProperties` if the operation is successful.
    ///
    /// # Arguments
    ///
    /// * `partition_id` - The ID of the partition to retrieve properties for.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `EventHubPartitionProperties` if the operation is successful.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use azure_messaging_eventhubs::consumer::ConsumerClient;
    /// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let my_credential = DefaultAzureCredential::new().unwrap();
    ///     let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
    ///     let partition_id = "0";
    ///
    ///     let partition_properties = consumer.get_partition_properties(partition_id.to_string()).await;
    ///
    ///     match partition_properties {
    ///         Ok(properties) => {
    ///             // Process the partition properties
    ///             println!("Partition properties: {:?}", properties);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error retrieving partition properties: {:?}", err);
    ///         }
    ///     }
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
            .ok_or(azure_core::Error::from(ErrorKind::MissingConnection))?;
        let session = AmqpSession::new();
        session.begin(connection, None).await?;
        trace!("Session created.");

        let management_path = self.url.clone() + "/$management";
        let access_token = self.authorize_path(management_path).await?;

        trace!("Create management client.");
        let management = AmqpManagement::new(
            session,
            "eventhubs_consumer_management".to_string(),
            access_token,
        )?;
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
                    self.options
                        .application_id
                        .clone()
                        .unwrap_or(uuid::Uuid::new_v4().to_string()),
                    Url::parse(url).map_err(Error::from)?,
                    Some(
                        AmqpConnectionOptions::builder()
                            .with_properties(vec![
                                ("user-agent", get_user_agent(&self.options.application_id)),
                                ("version", get_package_version()),
                                ("platform", get_platform_info()),
                                ("product", get_package_name()),
                            ])
                            .build(),
                    ),
                )
                .await?;
            self.connection
                .set(connection)
                .map_err(|_| azure_core::Error::from(ErrorKind::MissingManagementClient))?
        }
        Ok(())
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

            // insert returns some if it *fails* to insert, None if it succeeded.
            let present = scopes.insert(url.clone(), token);
            if present.is_some() {
                return Err(azure_core::Error::from(
                    ErrorKind::UnableToAddAuthenticationToken,
                ));
            }
            trace!("Token added.");
        }
        Ok(scopes
            .get(url.as_str())
            .ok_or_else(|| azure_core::Error::from(ErrorKind::UnableToAddAuthenticationToken))?
            .clone())
    }

    async fn get_session(&self, partition_id: &String) -> Result<Arc<AmqpSession>> {
        let mut session_instances = self.session_instances.lock().await;
        if !session_instances.contains_key(partition_id) {
            debug!("Creating session for partition: {:?}", partition_id);
            let connection = self
                .connection
                .get()
                .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingConnection))?;
            let session = AmqpSession::new();
            session.begin(connection, None).await?;
            session_instances.insert(partition_id.clone(), Arc::new(session));
        }
        let rv = session_instances
            .get(partition_id)
            .ok_or_else(|| azure_core::Error::from(ErrorKind::MissingSession))?
            .clone();
        debug!("Cloning session for partition {:?}", partition_id);
        Ok(rv)
    }
}

/// Represents the options for configuring a ConsumerClient.
#[derive(Debug, Default)]
pub struct ConsumerClientOptions {
    /// The application ID to set.
    pub application_id: Option<String>,
    /// The instance ID to set.
    pub instance_id: Option<String>,
    /// The retry options to set.
    pub retry_options: Option<RetryOptions>,
}

impl ConsumerClientOptions {}

/// Represents the options for receiving events from an Event Hub.
#[derive(Debug, Clone, Default)]
pub struct ReceiveOptions {
    /// The owner level for messages being retrieved.
    pub owner_level: Option<i64>,
    /// The prefetch count for messages being retrieved.
    pub prefetch: Option<u32>,
    /// The starting position for messages being retrieved.
    pub start_position: Option<StartPosition>,
}
/// Represents the options for receiving events from an Event Hub.
impl ReceiveOptions {}

/// Represents the starting position of a consumer when receiving events from an Event Hub.
#[derive(Debug, Default, PartialEq, Clone)]
pub enum StartLocation {
    /// The starting position is specified by an offset.
    Offset(String),
    /// The starting position is specified by a sequence number.
    SequenceNumber(i64),
    /// The starting position is specified by an enqueued time.
    EnqueuedTime(std::time::SystemTime),
    /// The starting position is the earliest event in the partition.
    Earliest,
    #[default]
    /// The starting position is the latest event in the partition.
    Latest,
}

const ENQUEUED_TIME_ANNOTATION: &str = "amqp.annotation.x-opt-enqueued-time";
const OFFSET_ANNOTATION: &str = "amqp.annotation.x-opt-offset";
const SEQUENCE_NUMBER_ANNOTATION: &str = "amqp.annotation.x-opt-sequence-number";

/// Represents the starting position of a consumer when receiving events from an Event Hub.
///
/// This enum provides different ways to specify the starting position of a consumer when receiving events from an Event Hub.
/// The starting position can be specified using an offset, a sequence number, an enqueued time, or the earliest or latest event in the partition.
///
/// The default starting position is the latest event in the partition (always receive new events).
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use azure_messaging_eventhubs::consumer::{StartPosition, StartLocation};
///
/// let start_position = StartPosition{
///   location: StartLocation::SequenceNumber(12345),
///    ..Default::default()};;
/// ```
///
/// ```
/// use azure_messaging_eventhubs::consumer::{StartPosition, StartLocation};
///
/// let start_position = StartPosition{
///  location: StartLocation::EnqueuedTime(std::time::SystemTime::now()),
///  ..Default::default()
/// };
/// ```
///
/// ```
/// use azure_messaging_eventhubs::consumer::{StartPosition, StartLocation};
///
/// let start_position = StartPosition{
///   location: StartLocation::Offset("12345".to_string()),
///   ..Default::default()
/// };
/// ```
///
/// ```
/// use azure_messaging_eventhubs::consumer::{StartPosition, StartLocation};
///
/// let start_position = StartPosition{
///   location: StartLocation::Earliest,
///   ..Default::default()
/// };
/// ```
///
/// ```
/// use azure_messaging_eventhubs::consumer::{StartPosition, StartLocation};
///
/// let start_position = StartPosition{
///   location: StartLocation::Latest,
///   ..Default::default()
/// };
/// ```
///
/// ```
/// use azure_messaging_eventhubs::consumer::StartPosition;
///
/// let start_position = StartPosition::default();
/// ```
///
#[derive(Debug, PartialEq, Clone, Default)]
pub struct StartPosition {
    /// The location of the starting position.
    pub location: StartLocation,

    /// Whether the starting position is inclusive (includes the event at StartLocation).
    pub inclusive: bool,
}

impl StartPosition {
    pub(crate) fn start_expression(position: &Option<StartPosition>) -> String {
        if let Some(position) = position {
            let mut greater_than: &str = ">";
            if position.inclusive {
                greater_than = ">=";
            }
            match &position.location {
                StartLocation::Offset(offset) => {
                    format!("{} {}'{}'", OFFSET_ANNOTATION, greater_than, offset)
                }
                StartLocation::SequenceNumber(sequence_number) => {
                    format!(
                        "{} {}'{}'",
                        SEQUENCE_NUMBER_ANNOTATION, greater_than, sequence_number
                    )
                }
                StartLocation::EnqueuedTime(enqueued_time) => {
                    let enqueued_time = enqueued_time
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis();
                    format!(
                        "{} {}'{}'",
                        ENQUEUED_TIME_ANNOTATION, greater_than, enqueued_time
                    )
                }
                StartLocation::Earliest => "amqp.annotation.x-opt-offset > '-1'".to_string(),
                StartLocation::Latest => "amqp.annotation.x-opt-offset > '@latest'".to_string(),
            }
        } else {
            "amqp.annotation.x-opt-offset > '@latest'".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

    #[test]
    fn setup() {
        INIT_LOGGING.call_once(|| {
            println!("Setting up test logger...");

            tracing_subscriber::fmt::init();
        });
    }

    #[test]
    fn test_default_consumer_options() {
        {
            let options = ConsumerClientOptions::default();
            assert!(options.application_id.is_none());
            assert!(options.instance_id.is_none());
            assert!(options.retry_options.is_none());
        }

        {
            let options = ConsumerClientOptions::default();
            assert!(options.application_id.is_none());
            assert!(options.instance_id.is_none());
            assert!(options.retry_options.is_none());
        }
    }

    #[test]
    fn test_consumer_client_with_options() {
        let options = ConsumerClientOptions {
            application_id: Some("test_app_id".to_string()),
            instance_id: Some("test_instance_id".to_string()),
            retry_options: Some(RetryOptions::default()),
        };

        assert_eq!(options.application_id, Some("test_app_id".to_string()));
        assert_eq!(options.instance_id, Some("test_instance_id".to_string()));
        assert!(options.retry_options.is_some());
    }
    #[test]
    fn test_start_position_builder_with_sequence_number() {
        setup();
        let sequence_number = 12345i64;
        let start_position = StartPosition {
            location: StartLocation::SequenceNumber(sequence_number),
            ..Default::default()
        };
        assert_eq!(
            start_position.location,
            StartLocation::SequenceNumber(sequence_number)
        );
        assert_eq!(
            StartPosition::start_expression(&Some(start_position)),
            "amqp.annotation.x-opt-sequence-number >'12345'"
        );

        let start_position = StartPosition {
            location: StartLocation::SequenceNumber(sequence_number),
            inclusive: true,
        };
        assert_eq!(
            StartPosition::start_expression(&Some(start_position)),
            "amqp.annotation.x-opt-sequence-number >='12345'"
        );
    }

    #[test]
    fn test_start_position_builder_with_enqueued_time() {
        setup();
        let enqueued_time = std::time::SystemTime::now();
        let start_position = StartPosition {
            location: StartLocation::EnqueuedTime(enqueued_time),
            ..Default::default()
        };
        info!("enqueued_time: {:?}", enqueued_time);
        info!(
            "enqueued_time: {:?}",
            enqueued_time.duration_since(std::time::UNIX_EPOCH)
        );
        info!(
            "enqueued_time: {:?}",
            enqueued_time
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );
        assert_eq!(
            start_position.location,
            StartLocation::EnqueuedTime(enqueued_time)
        );
        assert!(!start_position.inclusive);
        assert_eq!(
            StartPosition::start_expression(&Some(start_position)),
            format!(
                "amqp.annotation.x-opt-enqueued-time >'{}'",
                enqueued_time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            )
        );

        let start_position = StartPosition {
            location: StartLocation::EnqueuedTime(enqueued_time),
            inclusive: true,
        };
        assert_eq!(
            StartPosition::start_expression(&Some(start_position)),
            format!(
                "amqp.annotation.x-opt-enqueued-time >='{}'",
                enqueued_time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            )
        );
    }

    #[test]
    fn test_start_position_builder_with_offset() {
        setup();
        let offset = "12345".to_string();
        let start_position = StartPosition {
            location: StartLocation::Offset(offset.clone()),
            ..Default::default()
        };
        assert_eq!(
            start_position.location,
            StartLocation::Offset(offset.clone())
        );
        assert_eq!(
            "amqp.annotation.x-opt-offset >'12345'",
            StartPosition::start_expression(&Some(start_position)),
        );

        let start_position = StartPosition {
            location: StartLocation::Offset(offset.clone()),
            inclusive: true,
        };
        assert_eq!(
            "amqp.annotation.x-opt-offset >='12345'",
            StartPosition::start_expression(&Some(start_position)),
        );
    }

    #[test]
    fn test_start_position_builder_inclusive() {
        setup();
        let start_position = StartPosition {
            inclusive: true,
            ..Default::default()
        };
        assert!(start_position.inclusive);
        let start_position = StartPosition::default();
        assert!(!start_position.inclusive);
    }
}
