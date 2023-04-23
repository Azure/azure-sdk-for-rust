use std::sync::Arc;

use tokio::sync::watch;

use super::{LastEnqueuedEventProperties, PartitionEvent};

/// Represents an Event Hub partition and its relative state, as scoped to an associated
/// operation performed against it.
///
/// TODO: this seems like only associated with a processor. Implement this later.
///
/// There was only one use of source consumer which is to obtain the `LastReceivedEvent`, which is an `Event`. This
/// can be achieved by using a `tokio::sync::watch` channel.
#[derive(Debug)]
pub struct PartitionContext {
    pub(crate) fully_qualified_namespace: Arc<String>,
    pub(crate) event_hub_name: Arc<String>,
    pub(crate) consumer_group: Arc<String>,
    pub(crate) partition_id: Arc<String>,
    pub(crate) watch_last_received_event: watch::Receiver<Option<PartitionEvent>>,
}

impl PartitionContext {
    /// The fully qualified Event Hubs namespace that this context is associated with.
    pub fn fully_qualified_namespace(&self) -> &str {
        &self.fully_qualified_namespace
    }

    pub fn event_hub_name(&self) -> &str {
        &self.event_hub_name
    }

    pub fn consumer_group(&self) -> &str {
        &self.consumer_group
    }

    pub fn partition_id(&self) -> &str {
        &self.partition_id
    }

    /// A set of information about the last enqueued event of a partition, as observed by the associated EventHubs client
    /// associated with this context as events are received from the Event Hubs service.  This is only available if the consumer was
    /// created with [`ReadEventOptions::track_last_enqueued_event_properties`] set.
    pub fn read_last_enqueued_event_properties(&self) -> LastEnqueuedEventProperties {
        let last_enqueued_event = self.watch_last_received_event.borrow();
        let sequence_number = last_enqueued_event
            .as_ref()
            .and_then(|event| event.last_partition_sequence_number());
        let offset = last_enqueued_event
            .as_ref()
            .and_then(|event| event.last_partition_offset());
        let enqueued_time = last_enqueued_event
            .as_ref()
            .and_then(|event| event.last_partition_enqueued_time());
        let last_received_time = last_enqueued_event
            .as_ref()
            .and_then(|event| event.last_partition_properties_retrieval_time());
        LastEnqueuedEventProperties {
            sequence_number,
            offset,
            enqueued_time,
            last_received_time,
        }
    }
}
