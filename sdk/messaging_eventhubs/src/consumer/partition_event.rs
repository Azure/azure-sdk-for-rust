use crate::event::Event;

use super::PartitionContext;

/// The Event Hub partition that the <see cref="PartitionEvent.Data" /> is associated with
#[derive(Debug)]
pub struct PartitionEvent {
    /// The Event Hub partition that the <see cref="PartitionEvent.Data" /> is associated with.
    pub(crate) partition_context: PartitionContext,

    /// An event that was read from the associated <see cref="PartitionEvent.Partition" />.
    pub(crate) data: Event
}

impl PartitionEvent {
    /// Initializes a new instance of the <see cref="PartitionEvent"/> structure.
    pub fn new(partition_context: PartitionContext, data: Event) -> Self {
        Self {
            partition_context,
            data,
        }
    }

    /// The Event Hub partition that the <see cref="PartitionEvent.Data" /> is associated with.
    pub fn partition_context(&self) -> &PartitionContext {
        &self.partition_context
    }

    /// An event that was read from the associated <see cref="PartitionEvent.Partition" />.
    pub fn data(&self) -> &Event {
        &self.data
    }
}
