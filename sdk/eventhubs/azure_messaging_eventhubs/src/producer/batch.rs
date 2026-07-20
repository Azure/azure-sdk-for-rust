// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::ProducerClient;
use crate::{error::Result, models::EventData, EventHubsError};
use azure_core::{http::Url, Error, Uuid};
use azure_core_amqp::{AmqpMessage, AmqpSenderApis, AmqpSymbol};
use std::sync::Mutex;
use tracing::{debug, warn};

/// Represents the options that can be set when adding event data to an [`EventDataBatch`].
pub struct AddEventDataOptions {}

struct EventDataBatchState {
    serialized_messages: Vec<Vec<u8>>,
    size_in_bytes: u64,
    batch_envelope: Option<AmqpMessage>,
}

/// Represents a collections of event data that can be sent to an Event Hubs instance in one operation.
///
/// The [`EventDataBatch`] struct is used to create and manage a batch of event data
/// that can be sent to an Event Hubs instance using the [`ProducerClient`]. It provides
/// methods to add event data to the batch, calculate the size of the batch, and
/// check if the batch is empty.
///
/// # Examples
///
/// ``` no_run
/// # use azure_messaging_eventhubs::ProducerClient;
///
/// # async fn send_event_batch() -> Result<(), Box<dyn std::error::Error>> {
/// # let credentials = azure_identity::DeveloperToolsCredential::new(None)?;
/// # let producer_client = ProducerClient::builder()
/// #     .open("fully_qualified_domain_name", "event_hub_name", credentials.clone()).await?;
/// #
///
/// let mut batch = producer_client.create_batch(None).await?;
///
/// batch.try_add_event_data("Hello, Event Hub!", None)?;
/// batch.try_add_event_data("This is another event.", None)?;
///
/// producer_client.send_batch(batch, None).await?;
///
/// # Ok(())
/// # }
/// ```
pub struct EventDataBatch<'a> {
    producer: &'a ProducerClient,
    batch_state: Mutex<EventDataBatchState>,
    /// The size the caller asked for, if any. `attach` compares it against the
    /// maximum the link reports and keeps it when it fits. It stays separate
    /// from `max_size_in_bytes` so that `attach` can tell "the caller asked for
    /// this" apart from "nobody asked, use the link maximum".
    requested_max_size_in_bytes: Option<u64>,
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
            requested_max_size_in_bytes: options.as_ref().and_then(|o| o.max_size_in_bytes),
            max_size_in_bytes: options
                .as_ref()
                .map_or(u64::MAX, |o| o.max_size_in_bytes.unwrap_or(u64::MAX)),
            partition_key: options.as_ref().and_then(|o| o.partition_key.clone()),
            partition_id: options.and_then(|o| o.partition_id),
        }
    }

    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            partition_id = self.partition_id.as_deref().unwrap_or("<auto>"),
        ),
        err,
    )]
    pub(crate) async fn attach(&mut self) -> Result<()> {
        let path = self.get_batch_path()?;
        let sender = self.producer.ensure_sender(path.clone()).await?;
        let link_max_size = sender.max_message_size().await?.ok_or_else(|| {
            warn!(
                path = %path,
                "The sender link did not report a maximum message size; cannot size the batch."
            );
            Error::with_message(
                azure_core::error::ErrorKind::Other,
                "No maximum message size available from the sender link.",
            )
        })?;

        if let Some(requested) = self.requested_max_size_in_bytes {
            if requested > link_max_size {
                warn!(
                    path = %path,
                    requested,
                    link_max_size,
                    "The requested batch size is larger than the link allows."
                );
            }
        }
        self.max_size_in_bytes =
            Self::resolve_max_size_in_bytes(self.requested_max_size_in_bytes, link_max_size)?;
        Ok(())
    }

    /// Decides the batch size from the size the caller asked for and the
    /// maximum the link reports.
    ///
    /// A request larger than the link allows is rejected, it is not reduced.
    /// The broker refuses the oversized transfer anyway, and a silent reduction
    /// hides that the requested size was impossible. The other Azure SDKs for
    /// Event Hubs (.NET, Go and Java) all report an error in this case. With no
    /// request, the link maximum applies.
    fn resolve_max_size_in_bytes(requested: Option<u64>, link_max_size: u64) -> Result<u64> {
        match requested {
            Some(requested) if requested > link_max_size => Err(Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "The requested maximum batch size ({requested} bytes) is larger than the \
                     maximum the link allows ({link_max_size} bytes)."
                ),
            )
            .into()),
            Some(requested) => Ok(requested),
            None => Ok(link_max_size),
        }
    }

    /// Gets the size of the batch in bytes.
    ///
    /// The size of the batch is the sum of the size of the messages in the batch.
    ///
    /// # Returns
    /// The size of the batch in bytes.
    ///
    pub fn size(&self) -> u64 {
        // Note that lock() returns an infallible result.
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

    fn arithmetic_error() -> EventHubsError {
        EventHubsError::with_message("Arithmetic error calculating Batch size.")
    }

    fn calculate_actual_size_for_payload(length: usize) -> Result<u64> {
        const MESSAGE_HEADER_SIZE_32: usize = 8;
        const MESSAGE_HEADER_SIZE_8: usize = 5;
        if length < 256 {
            Ok(length
                .checked_add(MESSAGE_HEADER_SIZE_8)
                .ok_or_else(Self::arithmetic_error)? as u64)
        } else {
            Ok(length
                .checked_add(MESSAGE_HEADER_SIZE_32)
                .ok_or_else(Self::arithmetic_error)? as u64)
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
    /// # use azure_messaging_eventhubs::ProducerClient;
    /// # use azure_messaging_eventhubs::models::EventData;
    ///
    /// # async fn send_event_batch() -> Result<(), Box<dyn std::error::Error>> {
    /// # let my_credential = azure_identity::DeveloperToolsCredential::new(None)?;
    /// # let producer_client = ProducerClient::builder().open("fully_qualified_domain_name", "event_hub_name", my_credential.clone()).await?;
    /// let mut batch = producer_client.create_batch(None).await?;
    ///
    /// let event_data = EventData::builder().build();
    /// batch.try_add_event_data(event_data, None)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// # use azure_messaging_eventhubs::EventDataBatch;
    ///
    pub fn try_add_event_data(
        &self,
        event_data: impl Into<EventData>,
        options: Option<AddEventDataOptions>,
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
    /// # use azure_messaging_eventhubs::ProducerClient;
    /// # use azure_messaging_eventhubs::models::EventData;
    /// # use azure_messaging_eventhubs::models::AmqpMessage;
    ///
    /// # async fn send_event_batch() -> Result<(), Box<dyn std::error::Error>> {
    /// # let my_credential = azure_identity::DeveloperToolsCredential::new(None)?;
    /// # let producer_client = ProducerClient::builder().open("fully_qualified_domain_name", "event_hub_name", my_credential.clone()).await?;
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
        if message.properties.is_none() || message.properties.as_ref().unwrap().message_id.is_none()
        {
            message.set_message_id(Uuid::new_v4());
        }
        if let Some(partition_key) = self.partition_key.as_ref() {
            message.add_message_annotation(
                AmqpSymbol::from("x-opt-partition-key"),
                partition_key.clone(),
            );
        }

        let mut batch_state = self.batch_state.lock().unwrap();
        let message_len = AmqpMessage::serialize(&message)?.len();
        if batch_state.serialized_messages.is_empty() {
            // The first message serialized is the batch envelope - we capture the parameters from the first message to use for the batch
            batch_state.size_in_bytes = batch_state
                .size_in_bytes
                .checked_add(message_len as u64)
                .ok_or_else(Self::arithmetic_error)?;
            batch_state.batch_envelope = Some(self.create_batch_envelope(&message));
        }
        let serialized_message = AmqpMessage::serialize(&message)?;
        let actual_message_size =
            Self::calculate_actual_size_for_payload(serialized_message.len())?;
        if batch_state
            .size_in_bytes
            .checked_add(actual_message_size)
            .ok_or_else(Self::arithmetic_error)?
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

        let mut batch_envelope = batch_state.batch_envelope.clone().expect(
            "Batch envelope is missing when getting messages; \
             send_batch was called on an empty batch (add at least one event before sending).",
        );

        // Move the messages out of the batch state into a local variable so we
        // can subsequently move it to the message body.
        let mut serialized_messages = Vec::<Vec<u8>>::new();
        serialized_messages.append(&mut batch_state.serialized_messages);

        batch_envelope.set_message_body(serialized_messages);

        // Reset the batch state for the next batch
        batch_state.batch_envelope = None;
        batch_state.size_in_bytes = 0;
        batch_state.serialized_messages.clear();

        batch_envelope
    }

    pub(crate) fn get_batch_path(&self) -> Result<Url> {
        if let Some(partition_id) = self.partition_id.as_ref() {
            let batch_path = format!("{}/Partitions/{}", self.producer.base_url(), partition_id);

            Url::parse(&batch_path).map_err(|e| azure_core::Error::from(e).into())
        } else {
            Ok(self.producer.base_url().clone())
        }
    }

    fn create_batch_envelope(&self, message: &AmqpMessage) -> AmqpMessage {
        // Transfer all the message options from the original message to the batch envelope
        // Do NOT transfer the body, that will be handled later.
        let mut batch_builder = AmqpMessage::builder();

        if let Some(message_header) = message.header.as_ref() {
            batch_builder = batch_builder.with_header(message_header.clone());
        }
        if let Some(message_properties) = message.properties.as_ref() {
            batch_builder = batch_builder.with_properties(message_properties.clone());
        }
        if let Some(application_properties) = message.application_properties.as_ref() {
            batch_builder =
                batch_builder.with_application_properties(application_properties.clone());
        }
        if let Some(delivery_annotations) = message.delivery_annotations.as_ref() {
            batch_builder = batch_builder.with_delivery_annotations(delivery_annotations.clone());
        }
        if let Some(message_annotations) = message.message_annotations.as_ref() {
            batch_builder = batch_builder.with_message_annotations(message_annotations.clone());
        }
        if let Some(footer) = message.footer.as_ref() {
            batch_builder = batch_builder.with_footer(footer.clone());
        }

        batch_builder.build()
    }
}

/// Represents the options that can be set when creating an [`EventDataBatch`].
/// The options include the maximum size of the batch, the partition key, and the partition ID.
///
/// # Examples
///
/// ```
/// use azure_messaging_eventhubs::EventDataBatchOptions;
///
/// let options = EventDataBatchOptions{
///    max_size_in_bytes: Some(1024),
///    partition_key: Some("pk".to_string()),
///    partition_id: Some("12".to_string()),
///    ..Default::default()};
/// ```
///
#[derive(Default)]
pub struct EventDataBatchOptions {
    /// The maximum size of the batch in bytes.
    pub max_size_in_bytes: Option<u64>,

    /// The partition key to use when writing messages.
    pub partition_key: Option<String>,

    /// The partition ID to use as the target partition for the messages being written.
    pub partition_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_builder() {
        let options = EventDataBatchOptions {
            max_size_in_bytes: Some(1024),
            partition_key: Some("pk".to_string()),
            partition_id: Some("pid".to_string()),
        };

        assert_eq!(options.max_size_in_bytes, Some(1024));
        assert_eq!(options.partition_key, Some("pk".to_string()));
        assert_eq!(options.partition_id, Some("pid".to_string()));
    }

    // The size the caller asks for must reach the batch. Before this was fixed,
    // `attach` replaced it with the link maximum, so a batch capped at 1 KiB
    // accepted far more than 1 KiB.
    #[test]
    fn caller_size_is_kept_when_it_fits() {
        let size = EventDataBatch::resolve_max_size_in_bytes(Some(1024), 1_048_576)
            .expect("a size below the link maximum is allowed");
        assert_eq!(size, 1024);
    }

    // With no request, the link maximum applies. This is the historical
    // behavior and the other Azure SDKs agree on it.
    #[test]
    fn link_size_applies_when_the_caller_asks_for_nothing() {
        let size = EventDataBatch::resolve_max_size_in_bytes(None, 1_048_576)
            .expect("the link maximum is always allowed");
        assert_eq!(size, 1_048_576);
    }

    // A request the link cannot satisfy is an error, not a smaller batch. The
    // message must name both sizes, so the caller can see the limit.
    #[test]
    fn caller_size_above_the_link_maximum_is_rejected() {
        let error = EventDataBatch::resolve_max_size_in_bytes(Some(2_097_152), 1_048_576)
            .expect_err("a size above the link maximum must be rejected");
        let message = error.to_string();
        assert!(
            message.contains("2097152") && message.contains("1048576"),
            "the error must name the requested and the allowed size, got: {message}"
        );
    }

    // The boundary is inclusive: a request equal to the link maximum fits.
    #[test]
    fn caller_size_equal_to_the_link_maximum_is_allowed() {
        let size = EventDataBatch::resolve_max_size_in_bytes(Some(1_048_576), 1_048_576)
            .expect("a size equal to the link maximum is allowed");
        assert_eq!(size, 1_048_576);
    }
}
