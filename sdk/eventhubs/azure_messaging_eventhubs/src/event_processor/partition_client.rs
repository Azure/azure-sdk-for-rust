// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use super::processor::ProcessorConsumersMap;
use crate::{
    error::Result,
    models::{Checkpoint, ConsumerClientDetails, ReceivedEventData},
    processor::CheckpointStore,
    EventHubsError, EventReceiver,
};
use azure_core_amqp::{message::AmqpAnnotationKey, AmqpValue};
use futures::Stream;
use std::{
    pin::Pin,
    sync::{Arc, OnceLock, Weak},
};
use tracing::{debug, trace, warn};

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
    pub fn stream_events(&self) -> impl Stream<Item = Result<ReceivedEventData>> + '_ {
        if let Some(event_receiver) = self.event_receiver.get() {
            Box::pin(event_receiver.stream_events())
                as Pin<Box<dyn Stream<Item = Result<ReceivedEventData>> + '_>>
        } else {
            Box::pin(futures::stream::once(std::future::ready(Err(
                EventHubsError::with_message("Event receiver is not set for this partition."),
            ))))
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
    /// # async fn example(partition_client: PartitionClient) -> Result<(), Box<dyn std::error::Error>> {
    /// partition_client.close().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn close(mut self) -> Result<()> {
        // Detach the event receiver
        if let Some(event_receiver) = self.event_receiver.take() {
            debug!("Closing event receiver for partition {}", self.partition_id);
            event_receiver.close().await?;
        } else {
            debug!("Event receiver not set for partition {}", self.partition_id);
        }
        // Remove the partition client from the processor.
        let consumers = self.consumers.upgrade();
        if let Some(consumers) = consumers {
            debug!(
                "Removing client for partition {} from the consumers map.",
                self.partition_id
            );
            consumers.remove_partition_client(&self.partition_id)?;
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
        let mut offset_option = None;
        let mut sequence_number_option = None;

        let event_data_message = event_data.raw_amqp_message();
        let Some(message_annotations) = event_data_message.message_annotations.as_ref() else {
            // No message annotations. Nothing to do.
            return Ok(());
        };
        for (key, value) in message_annotations.0.iter() {
            let AmqpAnnotationKey::Symbol(symbol) = key else {
                continue;
            };

            if *symbol == "x-opt-offset" {
                let AmqpValue::String(offset_value) = value else {
                    continue;
                };
                offset_option = Some(offset_value.clone());
            } else if *symbol == "x-opt-sequence-number" {
                let AmqpValue::Long(sequence_number_value) = value else {
                    continue;
                };
                sequence_number_option = Some(*sequence_number_value);
            }
        }

        let checkpoint = Checkpoint {
            fully_qualified_namespace: self.client_details.fully_qualified_namespace.clone(),
            event_hub_name: self.client_details.eventhub_name.clone(),
            consumer_group: self.client_details.consumer_group.clone(),
            partition_id: self.partition_id.clone(),
            offset: offset_option,
            sequence_number: sequence_number_option,
        };
        self.checkpoint_store
            .update_checkpoint(checkpoint)
            .await
            .map_err(|e| {
                e.with_context(format!(
                    "Failed to update checkpoint for partition {}",
                    self.partition_id
                ))
                .into()
            })
    }

    pub(crate) fn set_event_receiver(&self, event_receiver: EventReceiver) -> Result<()> {
        // Set the event receiver
        self.event_receiver.set(event_receiver).map_err(|_| {
            warn!(
                "Event receiver already set for partition {}",
                self.partition_id
            );
            // If the event receiver is already set, return an error
            EventHubsError::with_message(format!(
                "Event receiver already set for partition {}",
                self.partition_id
            ))
        })?;
        Ok(())
    }
}

impl Drop for PartitionClient {
    fn drop(&mut self) {
        trace!(
            "Dropping PartitionClient for partition {}",
            self.partition_id
        );
    }
}
