use tokio::sync::watch;

use crate::event::Event;

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
    pub(crate) fully_qualified_namespace: String,
    pub(crate) event_hub_name: String,
    pub(crate) consumer_group: String,
    pub(crate) partition_id: String,
    pub(crate) watch_last_received_event: watch::Receiver<Option<PartitionEvent>>,
}

impl PartitionContext {
    /// A set of information about the last enqueued event of a partition, as observed by the associated EventHubs client
    /// associated with this context as events are received from the Event Hubs service.  This is only available if the consumer was
    /// created with [`ReadEventOptions::track_last_enqueued_event_properties`] set.
    fn read_last_enqueued_event_properties(&self) -> LastEnqueuedEventProperties {
        let last_enqueued_event = self.watch_last_received_event.borrow();
        let sequence_number = last_enqueued_event
            .as_ref()
            .and_then(|event| event.last_partition_sequence_number());
        let offset = last_enqueued_event.as_ref().and_then(|event| event.last_partition_offset());
        let enqueued_time = last_enqueued_event.as_ref().and_then(|event| event.last_partition_enqueued_time());
        let last_received_time = last_enqueued_event.as_ref().and_then(|event| event.last_partition_properties_retrieval_time());
        LastEnqueuedEventProperties {
            sequence_number,
            offset,
            enqueued_time,
            last_received_time,
        }
    }
}
