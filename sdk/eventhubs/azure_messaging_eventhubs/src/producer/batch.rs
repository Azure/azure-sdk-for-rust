// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::ProducerClient;
use crate::{error::ErrorKind, error::Result, models::EventData, EventHubsError};
use azure_core::{http::Url, Uuid};
use azure_core_amqp::{AmqpMessage, AmqpSymbol};
use std::sync::Mutex;
use tracing::debug;

/// Represents the options that can be set when adding event data to an [`EventDataBatch`].
pub struct AddEventDataOptions {}

/// The owned serialization core of a batch.
///
/// This holds every piece of state needed to accumulate messages and to produce
/// the AMQP batch envelope. It borrows nothing, so a background task can own one
/// for the whole life of the task. [`EventDataBatch`] wraps one of these behind a
/// mutex and keeps the borrow of the [`ProducerClient`] for itself.
pub(crate) struct EventDataBatchInner {
    serialized_messages: Vec<Vec<u8>>,
    size_in_bytes: u64,
    batch_envelope: Option<AmqpMessage>,
    max_size_in_bytes: u64,
    partition_key: Option<String>,
}

impl EventDataBatchInner {
    pub(crate) fn new(max_size_in_bytes: u64, partition_key: Option<String>) -> Self {
        Self {
            serialized_messages: Vec::new(),
            size_in_bytes: 0,
            batch_envelope: None,
            max_size_in_bytes,
            partition_key,
        }
    }

    pub(crate) fn size(&self) -> u64 {
        self.size_in_bytes
    }

    pub(crate) fn len(&self) -> usize {
        self.serialized_messages.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.serialized_messages.is_empty()
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

    /// Tries to add an AMQP message to the batch.
    ///
    /// Returns `true` when the message was added. Returns `false` when the
    /// message does not fit; in that case the batch is left unchanged.
    pub(crate) fn try_add(&mut self, message: impl Into<AmqpMessage>) -> Result<bool> {
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

        let message_len = AmqpMessage::serialize(&message)?.len();
        if self.serialized_messages.is_empty() {
            // The first message serialized is the batch envelope - we capture the parameters from the first message to use for the batch
            self.size_in_bytes = self
                .size_in_bytes
                .checked_add(message_len as u64)
                .ok_or_else(Self::arithmetic_error)?;
            self.batch_envelope = Some(Self::create_batch_envelope(&message));
        }
        let serialized_message = AmqpMessage::serialize(&message)?;
        let actual_message_size =
            Self::calculate_actual_size_for_payload(serialized_message.len())?;
        if self
            .size_in_bytes
            .checked_add(actual_message_size)
            .ok_or_else(Self::arithmetic_error)?
            > self.max_size_in_bytes
        {
            debug!("Batch is full. Cannot add more messages.");
            debug!("Message size: {actual_message_size}");
            debug!("Current batch size: {:?}", self.size_in_bytes);
            debug!("Max batch size: {:?}", self.max_size_in_bytes);
            if self.serialized_messages.is_empty() {
                self.batch_envelope = None;
                self.size_in_bytes = 0;
            }
            return Ok(false);
        }
        self.size_in_bytes += actual_message_size;
        self.serialized_messages.push(serialized_message);

        Ok(true)
    }

    /// Takes the accumulated messages as a single AMQP batch envelope and resets
    /// the batch so that it can accumulate again.
    ///
    /// # Panics
    ///
    /// Panics when the batch is empty. Callers must not send an empty batch.
    pub(crate) fn take_envelope(&mut self) -> AmqpMessage {
        let mut batch_envelope = self.batch_envelope.clone().expect(
            "Batch envelope is missing when getting messages; \
             send_batch was called on an empty batch (add at least one event before sending).",
        );

        // Move the messages out of the batch state into a local variable so we
        // can subsequently move it to the message body.
        let mut serialized_messages = Vec::<Vec<u8>>::new();
        serialized_messages.append(&mut self.serialized_messages);

        batch_envelope.set_message_body(serialized_messages);

        // Reset the batch state for the next batch
        self.batch_envelope = None;
        self.size_in_bytes = 0;
        self.serialized_messages.clear();

        batch_envelope
    }

    fn create_batch_envelope(message: &AmqpMessage) -> AmqpMessage {
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
    inner: Mutex<EventDataBatchInner>,
    partition_id: Option<String>,
}

impl<'a> EventDataBatch<'a> {
    /// Creates a batch with the maximum size already decided.
    ///
    /// The caller must get `max_size_in_bytes` from
    /// [`EventDataBatch::resolve_max_size_in_bytes`]. The batch does not read
    /// the size again from `options`, so there is only one place that can get
    /// it wrong.
    pub(crate) fn new(
        producer: &'a ProducerClient,
        options: Option<EventDataBatchOptions>,
        max_size_in_bytes: u64,
    ) -> Self {
        let partition_key = options.as_ref().and_then(|o| o.partition_key.clone());
        Self {
            producer,
            inner: Mutex::new(EventDataBatchInner::new(max_size_in_bytes, partition_key)),
            partition_id: options.and_then(|o| o.partition_id),
        }
    }

    /// Decides the batch size from the options the caller supplied and the
    /// maximum the link reports.
    ///
    /// A request larger than the link allows is rejected, it is not reduced.
    /// The broker refuses the oversized transfer anyway, and a silent reduction
    /// hides that the requested size was impossible. The other Azure SDKs for
    /// Event Hubs (.NET, Go and Java) all report an error in this case.
    ///
    /// A request of zero is also rejected. It is too small to hold the batch
    /// envelope, so every event is refused and the batch can never be sent.
    /// The Rust client uses `None` for "no request", so zero has no other
    /// meaning here.
    ///
    /// With no request, the link maximum applies.
    pub(crate) fn resolve_max_size_in_bytes(
        options: Option<&EventDataBatchOptions>,
        link_max_size: u64,
    ) -> Result<u64> {
        let invalid = |requested| {
            Err(EventHubsError::from(ErrorKind::InvalidBatchSize {
                requested,
                max_allowed: link_max_size,
            }))
        };
        match options.and_then(|o| o.max_size_in_bytes) {
            Some(0) => invalid(0),
            Some(requested) if requested > link_max_size => invalid(requested),
            Some(requested) => Ok(requested),
            None => Ok(link_max_size),
        }
    }

    /// Returns the AMQP path of the batch: the partition when the caller named
    /// one, and the Event Hub itself when the caller did not.
    pub(crate) fn batch_path(base_url: &Url, partition_id: Option<&str>) -> Result<Url> {
        match partition_id {
            Some(partition_id) => {
                let batch_path = format!("{base_url}/Partitions/{partition_id}");
                Url::parse(&batch_path).map_err(|e| azure_core::Error::from(e).into())
            }
            None => Ok(base_url.clone()),
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
        self.inner.lock().unwrap().size()
    }

    /// Gets the number of messages in the batch.
    ///
    /// # Returns
    ///
    /// The number of messages in the batch.
    ///
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }

    /// Determines whether the batch is empty.
    ///
    /// # Returns
    /// `true` if the batch is empty; otherwise, `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.inner.lock().unwrap().is_empty()
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
        self.inner.lock().unwrap().try_add(message)
    }

    pub(crate) fn get_messages(&self) -> AmqpMessage {
        self.inner.lock().unwrap().take_envelope()
    }

    pub(crate) fn get_batch_path(&self) -> Result<Url> {
        Self::batch_path(self.producer.base_url(), self.partition_id.as_deref())
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
    use crate::RetryOptions;
    use azure_core_amqp::AmqpTransport;
    use azure_core_test::credentials::MockCredential;
    use std::sync::Arc;

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

    const LINK_MAX_SIZE: u64 = 1_048_576;

    fn options_with_max_size(max_size_in_bytes: u64) -> EventDataBatchOptions {
        EventDataBatchOptions {
            max_size_in_bytes: Some(max_size_in_bytes),
            ..Default::default()
        }
    }

    // The cap the batch actually enforces, read out of the serialization core.
    fn effective_max_size(batch: &EventDataBatch<'_>) -> u64 {
        batch.inner.lock().unwrap().max_size_in_bytes
    }

    // A client that never opens a connection. `try_add_event_data` only
    // serializes and measures, so a batch can be driven without a broker.
    fn offline_producer() -> ProducerClient {
        ProducerClient::new(
            Url::parse("amqps://test.servicebus.windows.net").unwrap(),
            "eventhub".to_string(),
            Arc::new(MockCredential),
            None,
            RetryOptions::default(),
            None,
            None,
            AmqpTransport::default(),
        )
    }

    // The size the caller asks for must reach the batch. Before this was fixed,
    // `attach` replaced it with the link maximum, so a batch capped at 1 KiB
    // accepted far more than 1 KiB.
    #[test]
    fn caller_size_is_kept_when_it_fits() {
        let size = EventDataBatch::resolve_max_size_in_bytes(
            Some(&options_with_max_size(1024)),
            LINK_MAX_SIZE,
        )
        .expect("a size below the link maximum is allowed");
        assert_eq!(size, 1024);
    }

    // With no request, the link maximum applies. This is the historical
    // behavior and the other Azure SDKs agree on it.
    #[test]
    fn link_size_applies_when_the_caller_asks_for_nothing() {
        let size = EventDataBatch::resolve_max_size_in_bytes(None, LINK_MAX_SIZE)
            .expect("the link maximum is always allowed");
        assert_eq!(size, LINK_MAX_SIZE);

        let size = EventDataBatch::resolve_max_size_in_bytes(
            Some(&EventDataBatchOptions::default()),
            LINK_MAX_SIZE,
        )
        .expect("options that ask for no size are the same as no options");
        assert_eq!(size, LINK_MAX_SIZE);
    }

    // A request the link cannot satisfy is an error, not a smaller batch. The
    // message must name both sizes, so the caller can see the limit.
    #[test]
    fn caller_size_above_the_link_maximum_is_rejected() {
        let error = EventDataBatch::resolve_max_size_in_bytes(
            Some(&options_with_max_size(2_097_152)),
            LINK_MAX_SIZE,
        )
        .expect_err("a size above the link maximum must be rejected");
        assert!(
            matches!(
                error.kind,
                ErrorKind::InvalidBatchSize {
                    requested: 2_097_152,
                    max_allowed: LINK_MAX_SIZE,
                }
            ),
            "the caller must be able to match on the kind, got: {error:?}"
        );
        let message = error.to_string();
        assert!(
            message.contains("2097152") && message.contains("1048576"),
            "the error must name the requested and the allowed size, got: {message}"
        );
    }

    // Zero cannot hold the batch envelope, so a batch capped at zero refuses
    // every event and can never be sent. Report it instead. `None` already
    // means "no request", so zero is not a sentinel here.
    #[test]
    fn caller_size_of_zero_is_rejected() {
        let error = EventDataBatch::resolve_max_size_in_bytes(
            Some(&options_with_max_size(0)),
            LINK_MAX_SIZE,
        )
        .expect_err("a size of zero must be rejected");
        assert!(
            matches!(
                error.kind,
                ErrorKind::InvalidBatchSize {
                    requested: 0,
                    max_allowed: LINK_MAX_SIZE,
                }
            ),
            "a size of zero must report the batch size kind, got: {error:?}"
        );
    }

    // The boundary is inclusive: a request equal to the link maximum fits.
    #[test]
    fn caller_size_equal_to_the_link_maximum_is_allowed() {
        let size = EventDataBatch::resolve_max_size_in_bytes(
            Some(&options_with_max_size(LINK_MAX_SIZE)),
            LINK_MAX_SIZE,
        )
        .expect("a size equal to the link maximum is allowed");
        assert_eq!(size, LINK_MAX_SIZE);
    }

    // The whole chain the bug broke, without a broker: the option the caller
    // supplies decides the effective size, the effective size reaches the
    // batch, and the batch stops accepting events at it. A regression in any
    // one of the three fails this test.
    #[test]
    fn the_resolved_size_stops_the_batch() {
        const MAX_SIZE: u64 = 1024;
        let producer = offline_producer();
        let options = options_with_max_size(MAX_SIZE);

        let max_size_in_bytes =
            EventDataBatch::resolve_max_size_in_bytes(Some(&options), LINK_MAX_SIZE)
                .expect("1024 bytes is below the link maximum");
        let batch = EventDataBatch::new(&producer, Some(options), max_size_in_bytes);
        assert_eq!(effective_max_size(&batch), MAX_SIZE);

        let body = "x".repeat(128);
        let mut accepted = 0;
        for _ in 0..64 {
            if !batch
                .try_add_event_data(body.clone(), None)
                .expect("adding an event of a known size cannot fail")
            {
                break;
            }
            accepted += 1;
        }

        assert!(
            accepted > 0,
            "a batch capped at {MAX_SIZE} bytes must accept at least one 128 byte event"
        );
        assert!(
            batch.size() <= MAX_SIZE,
            "batch grew to {} bytes, past its {MAX_SIZE} byte cap",
            batch.size()
        );
        assert_eq!(
            accepted,
            batch.len(),
            "every accepted event must be in the batch"
        );
        // A 128 byte body serializes to well under 256 bytes, so a 1024 byte
        // cap holds a handful of them. The exact count depends on the AMQP
        // encoding; the bound only has to be tight enough to fail if the cap
        // reverts to the link maximum.
        assert!(
            accepted < 16,
            "a {MAX_SIZE} byte batch accepted {accepted} events of 128 bytes, so the cap was ignored"
        );
    }

    // With no request the batch takes the link maximum, and it is the link
    // maximum that reaches the field.
    #[test]
    fn the_link_size_stops_the_batch_when_the_caller_asks_for_nothing() {
        let producer = offline_producer();
        let max_size_in_bytes = EventDataBatch::resolve_max_size_in_bytes(None, LINK_MAX_SIZE)
            .expect("the link maximum is always allowed");
        let batch = EventDataBatch::new(&producer, None, max_size_in_bytes);

        assert_eq!(effective_max_size(&batch), LINK_MAX_SIZE);
        assert!(batch
            .try_add_event_data("x".repeat(128), None)
            .expect("adding an event of a known size cannot fail"));
    }
}
