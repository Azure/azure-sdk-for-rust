// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use super::processor::ProcessorConsumersMap;
use crate::{
    error::{ErrorKind, Result},
    models::{Checkpoint, ConsumerClientDetails, ReceivedEventData},
    processor::CheckpointStore,
    EventHubsError, EventReceiver,
};
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
///
/// Stream termination is the only revocation signal: when the partition is
/// reassigned (or the broker disconnects the receiver via epoch), `stream_events()`
/// resolves with `EventHubsError::ConsumerDisconnected`. Re-acquire via
/// [`EventProcessor::next_partition_client`](crate::EventProcessor::next_partition_client).
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

    /// Closes the AMQP receiver so any in-flight `stream_events()` resolves.
    /// Called by load-balancer reconciliation as a backstop for the
    /// broker-initiated disconnect path. Idempotent.
    pub(crate) async fn request_close_receiver(&self) {
        if let Some(receiver) = self.event_receiver.get() {
            if let Err(e) = receiver.request_close().await {
                warn!(
                    partition_id = %self.partition_id,
                    err = ?e,
                    "Failed to close event receiver during revocation for partition."
                );
            }
        }
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
            warn!(
                partition_id = %self.partition_id,
                "stream_events called but event receiver is not set for this partition; \
                 returning an error stream."
            );
            Box::pin(futures::stream::once(std::future::ready(Err(
                EventHubsError::with_message(format!(
                    "Event receiver is not set for partition {}.",
                    self.partition_id
                )),
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
            debug!(partition_id = %self.partition_id, "Closing event receiver for partition.");
            event_receiver.close().await?;
        } else {
            debug!(partition_id = %self.partition_id, "Event receiver not set for partition.");
        }
        // Remove the partition client from the processor.
        let consumers = self.consumers.upgrade();
        if let Some(consumers) = consumers {
            debug!(
                partition_id = %self.partition_id,
                "Removing client for partition from the consumers map."
            );
            consumers.remove_partition_client(&self.partition_id)?;
        }
        Ok(())
    }

    /// Updates the checkpoint for the current partition.
    ///
    /// This method reads the offset and the sequence number from the provided `ReceivedEventData`
    /// and updates the checkpoint in the `CheckpointStore`.
    ///
    /// # Arguments
    /// * `event_data` - The event data that carries the offset and the sequence number to record.
    ///
    /// # Errors
    /// Returns [`ErrorKind::MissingCheckpointMetadata`](crate::error::ErrorKind::MissingCheckpointMetadata)
    /// when the event carries no offset and no sequence number. Such an event names no position in
    /// the partition, and a checkpoint with both fields empty erases the position the store holds.
    /// Returns an error also when the checkpoint store fails to write the checkpoint.
    pub async fn update_checkpoint(&self, event_data: &ReceivedEventData) -> Result<()> {
        let offset = event_data.offset().clone();
        let sequence_number = event_data.sequence_number();
        if offset.is_none() && sequence_number.is_none() {
            return Err(EventHubsError::from(ErrorKind::MissingCheckpointMetadata {
                partition_id: self.partition_id.clone(),
            }));
        }

        debug!(
            partition_id = %self.partition_id,
            sequence_number = ?sequence_number,
            offset = ?offset,
            "Updating checkpoint for partition."
        );
        let checkpoint = Checkpoint {
            fully_qualified_namespace: self.client_details.fully_qualified_namespace.clone(),
            event_hub_name: self.client_details.eventhub_name.clone(),
            consumer_group: self.client_details.consumer_group.clone(),
            partition_id: self.partition_id.clone(),
            offset,
            sequence_number,
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
                partition_id = %self.partition_id,
                "Event receiver already set for partition."
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
            partition_id = %self.partition_id,
            "Dropping PartitionClient for partition."
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorKind;
    use crate::event_processor::Ownership;
    use crate::in_memory_checkpoint_store::InMemoryCheckpointStore;
    // Every AMQP name this module needs is declared here, not inherited from
    // the parent module, so the tests survive a change to the parent imports.
    use azure_core_amqp::{message::AmqpAnnotations, AmqpMessage, AmqpSymbol, AmqpValue};

    const TEST_NAMESPACE: &str = "ns.servicebus.windows.net";
    const TEST_EVENT_HUB: &str = "test-eventhub";
    const TEST_CONSUMER_GROUP: &str = "test-consumer-group";

    fn client_details() -> ConsumerClientDetails {
        ConsumerClientDetails {
            fully_qualified_namespace: TEST_NAMESPACE.to_string(),
            consumer_group: TEST_CONSUMER_GROUP.to_string(),
            eventhub_name: TEST_EVENT_HUB.to_string(),
            client_id: "test-client".to_string(),
        }
    }

    fn client_with_store(partition_id: &str) -> (PartitionClient, Arc<InMemoryCheckpointStore>) {
        let store = Arc::new(InMemoryCheckpointStore::new());
        let client = PartitionClient::new(
            partition_id.to_string(),
            store.clone(),
            client_details(),
            Weak::new(),
        );
        (client, store)
    }

    /// Builds an event whose AMQP message carries message annotations. An
    /// empty slice still sets an empty annotation map, which is not the same
    /// input as an absent map.
    fn event_with(pairs: &[(&str, AmqpValue)]) -> ReceivedEventData {
        let mut annotations = AmqpAnnotations::new();
        for (key, value) in pairs {
            annotations.insert(AmqpSymbol::from(*key), value.clone());
        }
        AmqpMessage::builder()
            .with_message_annotations(annotations)
            .build()
            .into()
    }

    fn event_without_annotations() -> ReceivedEventData {
        AmqpMessage::default().into()
    }

    struct FailingCheckpointStore;

    #[async_trait::async_trait]
    impl CheckpointStore for FailingCheckpointStore {
        async fn claim_ownership(
            &self,
            _ownerships: &[Ownership],
        ) -> azure_core::Result<Vec<Ownership>> {
            unreachable!("update_checkpoint must not claim ownership")
        }

        async fn list_checkpoints(
            &self,
            _namespace: &str,
            _event_hub_name: &str,
            _consumer_group: &str,
        ) -> azure_core::Result<Vec<Checkpoint>> {
            unreachable!("update_checkpoint must not list checkpoints")
        }

        async fn list_ownerships(
            &self,
            _namespace: &str,
            _event_hub_name: &str,
            _consumer_group: &str,
        ) -> azure_core::Result<Vec<Ownership>> {
            unreachable!("update_checkpoint must not list ownerships")
        }

        async fn update_checkpoint(&self, _checkpoint: Checkpoint) -> azure_core::Result<()> {
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "store is down",
            ))
        }
    }

    #[tokio::test]
    async fn update_checkpoint_rejects_an_event_without_message_annotations() {
        let (client, store) = client_with_store("0");
        let event = event_without_annotations();

        let result = client.update_checkpoint(&event).await;
        let stored = store
            .list_checkpoints(TEST_NAMESPACE, TEST_EVENT_HUB, TEST_CONSUMER_GROUP)
            .await
            .expect("the store must list its checkpoints");

        assert!(
            stored.is_empty(),
            "an event without message annotations must write no checkpoint, got: {stored:?}"
        );
        assert!(
            result.is_err(),
            "an event without message annotations must return an error to the caller"
        );
    }

    #[tokio::test]
    async fn update_checkpoint_rejects_an_event_without_offset_or_sequence_number() {
        let (client, store) = client_with_store("1");
        let event = event_with(&[("x-opt-partition-key", AmqpValue::String("pk".into()))]);

        let result = client.update_checkpoint(&event).await;
        let stored = store
            .list_checkpoints(TEST_NAMESPACE, TEST_EVENT_HUB, TEST_CONSUMER_GROUP)
            .await
            .expect("the store must list its checkpoints");

        assert!(
            stored.is_empty(),
            "annotations without an offset and without a sequence number must write no \
             checkpoint, got: {stored:?}"
        );
        assert!(
            result.is_err(),
            "annotations without an offset and without a sequence number must return an error"
        );
    }

    #[tokio::test]
    async fn update_checkpoint_rejects_annotations_with_the_wrong_value_types() {
        let (client, store) = client_with_store("2");
        let event = event_with(&[
            ("x-opt-offset", AmqpValue::Long(42)),
            ("x-opt-sequence-number", AmqpValue::String("17".to_string())),
        ]);

        let result = client.update_checkpoint(&event).await;
        let stored = store
            .list_checkpoints(TEST_NAMESPACE, TEST_EVENT_HUB, TEST_CONSUMER_GROUP)
            .await
            .expect("the store must list its checkpoints");

        assert!(
            stored.is_empty(),
            "annotations with the wrong value types must write no checkpoint, got: {stored:?}"
        );
        assert!(
            result.is_err(),
            "annotations with the wrong value types must return an error"
        );
    }

    #[tokio::test]
    async fn update_checkpoint_error_names_the_partition_and_the_kind() {
        let (client, _store) = client_with_store("7");
        let event = event_with(&[]);

        let error = client
            .update_checkpoint(&event)
            .await
            .expect_err("an event without the two annotations must return an error");

        let ErrorKind::MissingCheckpointMetadata { partition_id } = &error.kind else {
            panic!("the caller must be able to match on the kind, got: {error:?}");
        };
        assert_eq!(
            partition_id.as_str(),
            "7",
            "the error must name the partition, got: {partition_id}"
        );
    }

    #[tokio::test]
    async fn update_checkpoint_writes_an_offset_only_checkpoint() {
        let (client, store) = client_with_store("3");
        let event = event_with(&[("x-opt-offset", AmqpValue::String("1024".to_string()))]);

        client
            .update_checkpoint(&event)
            .await
            .expect("an offset alone must write a checkpoint");

        let stored = store
            .list_checkpoints(TEST_NAMESPACE, TEST_EVENT_HUB, TEST_CONSUMER_GROUP)
            .await
            .expect("the store must list its checkpoints");
        assert_eq!(stored.len(), 1, "the store must hold one checkpoint");
        assert_eq!(stored[0].offset, Some("1024".to_string()));
        assert_eq!(stored[0].sequence_number, None);
    }

    #[tokio::test]
    async fn update_checkpoint_writes_a_sequence_number_only_checkpoint() {
        let (client, store) = client_with_store("4");
        let event = event_with(&[("x-opt-sequence-number", AmqpValue::Long(17))]);

        client
            .update_checkpoint(&event)
            .await
            .expect("a sequence number alone must write a checkpoint");

        let stored = store
            .list_checkpoints(TEST_NAMESPACE, TEST_EVENT_HUB, TEST_CONSUMER_GROUP)
            .await
            .expect("the store must list its checkpoints");
        assert_eq!(stored.len(), 1, "the store must hold one checkpoint");
        assert_eq!(stored[0].offset, None);
        assert_eq!(stored[0].sequence_number, Some(17));
    }

    #[tokio::test]
    async fn update_checkpoint_writes_both_values_and_the_identity_fields() {
        let (client, store) = client_with_store("5");
        let event = event_with(&[
            ("x-opt-offset", AmqpValue::String("2048".to_string())),
            ("x-opt-sequence-number", AmqpValue::Long(99)),
        ]);

        client
            .update_checkpoint(&event)
            .await
            .expect("a complete pair of annotations must write a checkpoint");

        let stored = store
            .list_checkpoints(TEST_NAMESPACE, TEST_EVENT_HUB, TEST_CONSUMER_GROUP)
            .await
            .expect("the store must list its checkpoints");
        assert_eq!(stored.len(), 1, "the store must hold one checkpoint");
        assert_eq!(stored[0].fully_qualified_namespace, TEST_NAMESPACE);
        assert_eq!(stored[0].event_hub_name, TEST_EVENT_HUB);
        assert_eq!(stored[0].consumer_group, TEST_CONSUMER_GROUP);
        assert_eq!(stored[0].partition_id, "5");
        assert_eq!(stored[0].offset, Some("2048".to_string()));
        assert_eq!(stored[0].sequence_number, Some(99));
    }

    #[tokio::test]
    async fn update_checkpoint_reports_a_store_failure_with_its_context() {
        let client = PartitionClient::new(
            "6".to_string(),
            Arc::new(FailingCheckpointStore),
            client_details(),
            Weak::new(),
        );
        let event = event_with(&[
            ("x-opt-offset", AmqpValue::String("4096".to_string())),
            ("x-opt-sequence-number", AmqpValue::Long(5)),
        ]);

        let error = client
            .update_checkpoint(&event)
            .await
            .expect_err("a store failure must reach the caller");

        assert!(
            matches!(error.kind, ErrorKind::AzureCore(_)),
            "a store failure must keep the Azure Core kind, got: {error:?}"
        );
        assert!(
            error
                .to_string()
                .contains("Failed to update checkpoint for partition 6"),
            "the error must name the partition, got: {error}"
        );
    }
}
