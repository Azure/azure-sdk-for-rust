// Copyright (c) Microsoft Corp. All Rights Reserved.

// cspell: words amqp eventhubs

use std::sync::Mutex;

use super::ProducerClient;

use crate::models::EventData;
use azure_core::error::Result;
use azure_core_amqp::{
    messaging::{AmqpAnnotations, AmqpMessage, AmqpMessageBody, AmqpMessageProperties},
    sender::AmqpSenderTrait,
    value::{AmqpSymbol, AmqpValue},
};
use tracing::debug;
use uuid::Uuid;

/// Represents the options that can be set when adding event data to an `EventDataBatch`.
pub struct AddEventDataOptions {}

struct EventDataBatchState {
    serialized_messages: Vec<Vec<u8>>,
    size_in_bytes: u64,
    batch_envelope: Option<AmqpMessage>,
}

/// Represents a batch of event data that can be sent to an Event Hub.
///
/// The `EventDataBatch` struct is used to create and manage a batch of event data that can be sent to an Event Hub using the `ProducerClient`. It provides methods to add event data to the batch, calculate the size of the batch, and check if the batch is empty. The batch can be attached to a sender and the messages can be retrieved as an `AmqpMessage` to be sent to the Event Hub.
///
/// # Examples
///
/// ``` no_run
/// # use azure_messaging_eventhubs::producer::ProducerClient;
/// # use azure_messaging_eventhubs::producer::ProducerClientOptions;
/// # use azure_messaging_eventhubs::producer::batch::EventDataBatch;
/// # use azure_identity::TokenCredentialOptions;
///
/// # async fn send_event_batch() -> Result<(), Box<dyn std::error::Error>> {
/// # let credentials = azure_identity::DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap();
/// # let producer_client = ProducerClient::new("fully_qualified_domain_name", "event_hub_name", credentials, ProducerClientOptions::builder().build());
///
/// let mut batch = producer_client.create_batch(None).await?;
///
/// batch.try_add_event_data("Hello, Event Hub!", None)?;
/// batch.try_add_event_data("This is another event.", None)?;
///
/// producer_client.submit_batch(&batch).await?;
///
/// # Ok(())
/// # }
/// ```
pub struct EventDataBatch<'a> {
    producer: &'a ProducerClient,

    batch_state: Mutex<EventDataBatchState>,
    max_size_in_bytes: u64,
    partition_key: Option<String>,
    partition_id: Option<String>,
}

impl<'a> EventDataBatch<'a> {
    pub(crate) fn new(
        producer: &'a ProducerClient,
        options: Option<EventDataBatchOptions>,
    ) -> Self {
        Self {
            producer,
            batch_state: Mutex::new(EventDataBatchState {
                serialized_messages: Vec::new(),
                size_in_bytes: 0,
                batch_envelope: None,
            }),
            max_size_in_bytes: options
                .as_ref()
                .map_or(u64::MAX, |o| o.max_size_in_bytes.unwrap_or(u64::MAX)),
            partition_key: options.as_ref().and_then(|o| o.partition_key.clone()),
            partition_id: options.and_then(|o| o.partition_id),
        }
    }

    pub(crate) async fn attach(&mut self) -> Result<()> {
        let sender = self.producer.ensure_sender(self.get_batch_path()).await?;
        self.max_size_in_bytes = sender.lock().await.max_message_size().await.unwrap();
        Ok(())
    }

    /// Gets the size of the batch in bytes.
    ///
    /// The size of the batch is the sum of the size of the messages in the batch.
    ///
    /// # Returns
    /// The size of the batch in bytes.
    ///
    pub fn size(&self) -> u64 {
        self.batch_state.lock().unwrap().size_in_bytes
    }

    /// Gets the number of messages in the batch.
    ///
    /// # Returns
    ///
    /// The number of messages in the batch.
    ///
    pub fn len(&self) -> usize {
        self.batch_state.lock().unwrap().serialized_messages.len()
    }

    /// Determines whether the batch is empty.
    ///
    /// # Returns
    /// `true` if the batch is empty; otherwise, `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn calculate_actual_size_for_payload(length: usize) -> u64 {
        const MESSAGE_HEADER_SIZE_32: usize = 8;
        const MESSAGE_HEADER_SIZE_8: usize = 5;
        if length < 256 {
            length.checked_add(MESSAGE_HEADER_SIZE_8).unwrap() as u64
        } else {
            length.checked_add(MESSAGE_HEADER_SIZE_32).unwrap() as u64
        }
    }

    /// Tries to add an event data to the batch.
    ///
    /// If the event data is successfully added to the batch, the method returns `true`. If the event data cannot be added to the batch because the batch is full, the method returns `false`.
    ///
    /// # Parameters
    /// `event_data` - The event data to add to the batch.
    /// `options` - The options to set when adding the event data to the batch.
    ///
    /// # Returns
    ///
    /// `true` if the event data was added to the batch; otherwise, `false`.
    ///
    /// # Remarks
    /// If the event data does not have a message ID, a new message ID is generated for the event data.
    /// If the batch has a partition key, the event data is assigned the partition key.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///
    /// # use azure_messaging_eventhubs::producer::{ProducerClient, ProducerClientOptions};
    /// # use azure_messaging_eventhubs::producer::batch::EventDataBatch;
    /// # use azure_messaging_eventhubs::producer::batch::EventDataBatchOptions;
    /// # use azure_messaging_eventhubs::producer::batch::AddEventDataOptions;
    /// # use azure_messaging_eventhubs::models::EventData;
    ///
    /// # async fn send_event_batch() -> Result<(), Box<dyn std::error::Error>> {
    /// # let my_credential = azure_identity::DefaultAzureCredential::create(azure_identity::TokenCredentialOptions::default()).unwrap();
    /// # let producer_client = ProducerClient::new("fully_qualified_domain_name", "event_hub_name", my_credential, ProducerClientOptions::builder().build());
    /// let mut batch = producer_client.create_batch(None).await?;
    ///
    /// let event_data = EventData::builder().build();
    /// batch.try_add_event_data(event_data, None)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// # use azure_messaging_eventhubs::producer::batch::EventDataBatch;
    ///
    pub fn try_add_event_data(
        &mut self,
        event_data: impl Into<EventData>,
        #[allow(unused_variables)] options: Option<AddEventDataOptions>,
    ) -> Result<bool> {
        let event_data = event_data.into();
        self.try_add_amqp_message(event_data, options)
    }

    /// Tries to add an AMQP Message to the batch.
    ///
    /// If the message is successfully added to the batch, the method returns `true`. If the message cannot be added to the batch because the batch is full, the method returns `false`.
    ///
    /// # Parameters
    /// `message` - The message to add to the batch.
    /// `options` - The options to set when adding the message to the batch.
    ///
    /// # Returns
    /// `true` if the message was added to the batch; otherwise, `false`.
    ///
    /// # Remarks
    /// If the message does not have a message ID, a new message ID is generated for the message.
    /// If the batch has a partition key, the message is assigned the partition key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use azure_messaging_eventhubs::producer::{ProducerClient, ProducerClientOptions};
    /// # use azure_messaging_eventhubs::producer::batch::EventDataBatch;
    /// # use azure_messaging_eventhubs::producer::batch::EventDataBatchOptions;
    /// # use azure_messaging_eventhubs::producer::batch::AddEventDataOptions;
    /// # use azure_messaging_eventhubs::models::EventData;
    /// # use azure_messaging_eventhubs::models::AmqpMessage;
    ///
    /// # async fn send_event_batch() -> Result<(), Box<dyn std::error::Error>> {
    /// # let my_credential = azure_identity::DefaultAzureCredential::create(azure_identity::TokenCredentialOptions::default()).unwrap();
    /// # let producer_client = ProducerClient::new("fully_qualified_domain_name", "event_hub_name", my_credential, ProducerClientOptions::builder().build());
    /// let mut batch = producer_client.create_batch(None).await?;
    ///
    /// let amqp_message = AmqpMessage::builder().build();
    /// batch.try_add_amqp_message(amqp_message, None)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn try_add_amqp_message(
        &self,
        message: impl Into<AmqpMessage>,
        #[allow(unused_variables)] options: Option<AddEventDataOptions>,
    ) -> Result<bool> {
        let mut message = message.into();
        if message.properties().is_none() || message.properties().unwrap().message_id().is_none() {
            let mut message_properties = AmqpMessageProperties::default();
            if let Some(properties) = message.properties() {
                message_properties = properties.clone()
            }
            message_properties.set_message_id(Uuid::new_v4().to_string());
            message.set_properties(message_properties);
        }
        if self.partition_key.is_some() {
            let mut message_annotations = AmqpAnnotations::default();
            if let Some(annotations) = message.message_annotations() {
                message_annotations = annotations.clone()
            }
            message_annotations.insert(
                Into::<AmqpSymbol>::into("x-opt-partition-key"),
                Into::<AmqpValue>::into(self.partition_key.as_ref().unwrap().clone()),
            );
            message.set_message_annotations(message_annotations);
        }

        let mut batch_state = self.batch_state.lock().unwrap();
        let message_len = AmqpMessage::serialize(message.clone())?.len();
        if batch_state.serialized_messages.is_empty() {
            // The first message serialized is the batch envelope - we capture the parameters from the first message to use for the batch
            batch_state.size_in_bytes = batch_state
                .size_in_bytes
                .checked_add(message_len as u64)
                .unwrap();
            batch_state.batch_envelope = Some(self.create_batch_envelope(&message));
        }
        let serialized_message = AmqpMessage::serialize(message)?;
        let actual_message_size = Self::calculate_actual_size_for_payload(serialized_message.len());
        if batch_state
            .size_in_bytes
            .checked_add(actual_message_size)
            .unwrap()
            > self.max_size_in_bytes
        {
            debug!("Batch is full. Cannot add more messages.");
            debug!("Message size: {actual_message_size}");
            debug!("Current batch size: {:?}", batch_state.size_in_bytes);
            debug!("Max batch size: {:?}", self.max_size_in_bytes);
            if batch_state.serialized_messages.is_empty() {
                batch_state.batch_envelope = None;
                batch_state.size_in_bytes = 0;
            }
            return Ok(false);
        }
        batch_state.size_in_bytes += actual_message_size;
        batch_state.serialized_messages.push(serialized_message);

        Ok(true)
    }

    pub(crate) fn get_messages(&self) -> AmqpMessage {
        let mut batch_state = self.batch_state.lock().unwrap();

        let mut batch_envelope = batch_state.batch_envelope.clone().unwrap();

        // Move the messages out of the batch state into a local variable so we
        // can subsequently move it to the message body.
        let mut serialized_messages = Vec::<Vec<u8>>::new();
        serialized_messages.append(&mut batch_state.serialized_messages);

        batch_envelope.set_message_body(AmqpMessageBody::Binary(serialized_messages));

        // Reset the batch state for the next batch
        batch_state.batch_envelope = None;
        batch_state.size_in_bytes = 0;
        batch_state.serialized_messages.clear();

        batch_envelope
    }

    pub(crate) fn get_batch_path(&self) -> String {
        if self.partition_id.is_none() {
            self.producer.base_url()
        } else {
            format!(
                "{}/Partitions/{}",
                self.producer.base_url(),
                self.partition_id.as_ref().unwrap()
            )
        }
    }

    fn create_batch_envelope(&self, message: &AmqpMessage) -> AmqpMessage {
        // Transfer all the message options from the original message to the batch envelope
        // Do NOT transfer the body, that will be handled later.
        let mut batch_builder = AmqpMessage::builder();

        if message.header().is_some() {
            batch_builder = batch_builder.with_header(message.header().unwrap().clone());
        }
        if message.properties().is_some() {
            batch_builder = batch_builder.with_properties(message.properties().unwrap().clone());
        }
        if message.application_properties().is_some() {
            batch_builder = batch_builder
                .with_application_properties(message.application_properties().unwrap().clone());
        }
        if message.delivery_annotations().is_some() {
            batch_builder = batch_builder
                .with_delivery_annotations(message.delivery_annotations().unwrap().clone());
        }
        if message.message_annotations().is_some() {
            batch_builder = batch_builder
                .with_message_annotations(message.message_annotations().unwrap().clone());
        }
        if message.footer().is_some() {
            batch_builder = batch_builder.with_footer(message.footer().unwrap().clone());
        }

        batch_builder.build()
    }
}

/// Represents the options that can be set when creating an `EventDataBatch`.
/// The options include the maximum size of the batch, the partition key, and the partition ID.
///
/// # Examples
///
/// ```
/// use azure_messaging_eventhubs::producer::batch::EventDataBatchOptions;
///
/// let options = EventDataBatchOptions::builder()
///    .with_max_size_in_bytes(1024)
///    .with_partition_key("pk")
///    .with_partition_id("pid")
///    .build();
/// ```
///
pub struct EventDataBatchOptions {
    max_size_in_bytes: Option<u64>,
    partition_key: Option<String>,
    partition_id: Option<String>,
}

impl EventDataBatchOptions {
    /// Creates a new `EventDataBatchOptionsBuilder` to build an `EventDataBatchOptions`.
    ///
    /// # Returns
    ///
    /// An `EventDataBatchOptionsBuilder`.
    ///
    pub fn builder() -> builders::EventDataBatchOptionsBuilder {
        builders::EventDataBatchOptionsBuilder::new()
    }
}

mod builders {
    use super::*;

    pub struct EventDataBatchOptionsBuilder {
        options: EventDataBatchOptions,
    }

    impl EventDataBatchOptionsBuilder {
        pub(super) fn new() -> Self {
            Self {
                options: EventDataBatchOptions {
                    max_size_in_bytes: None,
                    partition_key: None,
                    partition_id: None,
                },
            }
        }

        /// Sets the maximum size of the batch in bytes.
        pub fn with_max_size_in_bytes(mut self, max_size_in_bytes: u64) -> Self {
            self.options.max_size_in_bytes = Some(max_size_in_bytes);
            self
        }

        /// Sets the target partition key for the batch.
        pub fn with_partition_key(mut self, partition_key: impl Into<String>) -> Self {
            self.options.partition_key = Some(partition_key.into());
            self
        }

        /// Sets the target partition ID for the batch.
        pub fn with_partition_id(mut self, partition_id: impl Into<String>) -> Self {
            self.options.partition_id = Some(partition_id.into());
            self
        }

        /// Builds the `EventDataBatchOptions`.
        ///
        /// # Returns
        ///
        /// An `EventDataBatchOptions`.
        ///
        pub fn build(self) -> EventDataBatchOptions {
            self.options
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_builder() {
        let options = EventDataBatchOptions::builder()
            .with_max_size_in_bytes(1024)
            .with_partition_key("pk")
            .with_partition_id("pid")
            .build();

        assert_eq!(options.max_size_in_bytes, Some(1024));
        assert_eq!(options.partition_key, Some("pk".to_string()));
        assert_eq!(options.partition_id, Some("pid".to_string()));
    }

    #[test]
    fn test_clone_array() {
        let mut array = vec![1, 2, 3, 4, 5];
        let mut copy = Vec::<i32>::new();

        // while let Some(val) = array.pop() {
        //     println!("{:?}", val);
        //     copy.push(val);
        // }
        copy.append(&mut array);
        assert_eq!(array.len(), 0);
        assert_eq!(copy, vec![1, 2, 3, 4, 5]);
    }
}
