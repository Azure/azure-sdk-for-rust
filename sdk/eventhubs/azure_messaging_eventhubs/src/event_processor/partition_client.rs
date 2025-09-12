// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use super::processor::ProcessorConsumersMap;
use crate::{
    models::{Checkpoint, ConsumerClientDetails, ReceivedEventData},
    processor::CheckpointStore,
    EventReceiver,
};
use azure_core::Result;
use futures::Stream;
use std::pin::Pin;
use std::sync::{Arc, OnceLock, Weak};
use tracing::warn;

/// Represents a client for interacting with a specific partition in Event Hubs.
///
/// The `PartitionClient` provides methods for receiving events, updating checkpoints,
/// and managing the lifecycle of the client for a specific partition.
pub struct PartitionClient {
    partition_id: String,
    checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
    client_details: ConsumerClientDetails,
    event_receiver: OnceLock<EventReceiver>,
    consumers: Weak<ProcessorConsumersMap>,
}

// It's safe to use the PartitionClient from multiple threads simultaneously.
unsafe impl Send for PartitionClient {}
unsafe impl Sync for PartitionClient {}

impl PartitionClient {
    pub(crate) fn new(
        partition_id: String,
        checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
        client_details: ConsumerClientDetails,
        consumers: Weak<ProcessorConsumersMap>,
    ) -> Self {
        Self {
            partition_id,
            checkpoint_store,
            client_details,
            event_receiver: OnceLock::new(),
            consumers,
        }
    }

    /// Returns the partition ID of the `PartitionClient`.
    ///
    /// # Returns
    /// A reference to the partition ID as a String slice.
    pub fn get_partition_id(&self) -> &str {
        &self.partition_id
    }

    /// Receives events from the partition.
    ///
    /// This method returns a stream of `ReceivedEventData` wrapped in a `Result`.
    /// The stream yields events as they are received from the partition.
    ///
    /// # Returns
    /// A stream of `Result<ReceivedEventData>` representing the received events.
    pub fn stream_events(&self) -> impl Stream<Item = azure_core::Result<ReceivedEventData>> + '_ {
        let event_receiver = self.event_receiver.get();
        if let Some(event_receiver) = event_receiver {
            Box::pin(event_receiver.stream_events())
                as Pin<Box<dyn Stream<Item = Result<ReceivedEventData>> + '_>>
        } else {
            // Return a stream with a single error indicating that the event receiver is not available.
            Box::pin(futures::stream::once(async {
                Err(azure_core::error::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Event receiver is not set for this partition.",
                ))
            }))
        }
    }

    /// Closes the `PartitionClient` by detaching the event receiver and removing the partition client
    /// from the processor's consumers map.
    ///
    /// This method performs the following steps:
    /// 1. Detaches the event receiver if it is set, ensuring no further events are received.
    /// 2. Attempts to remove the partition client from the processor's consumers map.
    ///    - If the consumers map has already been dropped, a warning is logged.
    ///
    /// # Errors
    /// Returns an error if detaching the event receiver fails or if removing the partition client
    /// from the consumers map encounters an issue.
    ///
    /// # Example
    /// ```
    /// # use azure_messaging_eventhubs::processor::PartitionClient;
    /// # async fn example(partition_client: PartitionClient) -> azure_core::Result<()> {
    /// partition_client.close().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn close(mut self) -> Result<()> {
        // Detach the event receiver
        if let Some(event_receiver) = self.event_receiver.take() {
            event_receiver.close().await?;
        }
        // Remove the partition client from the processor.
        let consumers = self.consumers.upgrade();
        if let Some(consumers) = consumers {
            consumers.remove_partition_client(&self.partition_id)?;
        } else {
            warn!("Consumers have been dropped, unable to remove partition client.");
        }
        Ok(())
    }

    /// Updates the checkpoint for the current partition.
    ///
    /// This method extracts the sequence number and offset from the provided `ReceivedEventData`
    /// and updates the checkpoint in the `CheckpointStore`.
    ///
    /// # Arguments
    /// * `event_data` - The event data containing the sequence number and offset to update the checkpoint.
    ///
    /// # Errors
    /// Returns an error if the sequence number or offset is invalid, or if updating the checkpoint fails.
    pub async fn update_checkpoint(&self, event_data: &ReceivedEventData) -> Result<()> {
        let mut sequence_number: Option<i64> = None;
        let mut offset: Option<String> = None;

        let amqp_message = event_data.raw_amqp_message();
        if let Some(message_annotations) = &amqp_message.message_annotations {
            for (key, value) in message_annotations.0.iter() {
                if *key == crate::consumer::SEQUENCE_NUMBER_ANNOTATION {
                    match value {
                        azure_core_amqp::AmqpValue::UInt(value) => {
                            sequence_number = Some(*value as i64);
                        }
                        azure_core_amqp::AmqpValue::ULong(value) => {
                            sequence_number = Some(*value as i64);
                        }
                        azure_core_amqp::AmqpValue::Long(value) => {
                            sequence_number = Some(*value);
                        }
                        azure_core_amqp::AmqpValue::Int(value) => {
                            sequence_number = Some(*value as i64);
                        }
                        _ => {
                            return Err(azure_core::error::Error::message(
                                azure_core::error::ErrorKind::Other,
                                "Invalid sequence number",
                            ));
                        }
                    }
                } else if *key == crate::consumer::OFFSET_ANNOTATION {
                    match value {
                        azure_core_amqp::AmqpValue::String(value) => {
                            offset = Some(value.to_string());
                        }
                        _ => {
                            return Err(azure_core::error::Error::message(
                                azure_core::error::ErrorKind::Other,
                                "Invalid offset",
                            ));
                        }
                    }
                }
            }
            let checkpoint = Checkpoint {
                fully_qualified_namespace: self.client_details.fully_qualified_namespace.clone(),
                event_hub_name: self.client_details.eventhub_name.clone(),
                consumer_group: self.client_details.consumer_group.clone(),
                partition_id: self.partition_id.clone(),
                offset,
                sequence_number,
            };
            self.checkpoint_store.update_checkpoint(checkpoint).await?;
        }
        Ok(())
    }

    pub(crate) fn set_event_receiver(&self, event_receiver: EventReceiver) -> Result<()> {
        // Set the event receiver
        self.event_receiver.set(event_receiver).map_err(|_| {
            warn!(
                "Event receiver already set for partition {}",
                self.partition_id
            );
            // If the event receiver is already set, return an error
            azure_core::error::Error::message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "Event receiver already set for partition {}",
                    self.partition_id
                ),
            )
        })?;
        Ok(())
    }
}
