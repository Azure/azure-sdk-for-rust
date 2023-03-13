use time::OffsetDateTime;

/// A set of information for a single partition of an Event Hub.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PartitionProperties {
    pub(crate) event_hub_name: String,
    pub(crate) id: String,
    pub(crate) beginning_sequence_number: i64,
    pub(crate) last_enqueued_sequence_number: i64,
    pub(crate) last_enqueued_offset: i64,
    pub(crate) last_enqueued_time: OffsetDateTime,
    pub(crate) is_empty: bool,
}

impl PartitionProperties {
    /// The name of the Event Hub where the partitions reside, specific to the
    /// Event Hubs namespace that contains it.
    pub fn event_hub_name(&self) -> &str {
        &self.event_hub_name
    }

    /// The identifier of the partition, unique to the Event Hub which contains it.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The first sequence number available for events in the partition.
    pub fn beginning_sequence_number(&self) -> i64 {
        self.beginning_sequence_number
    }

    /// The sequence number of the last observed event to be enqueued in the partition.
    pub fn last_enqueued_sequence_number(&self) -> i64 {
        self.last_enqueued_sequence_number
    }

    /// The offset of the last observed event to be enqueued in the partition.
    pub fn last_enqueued_offset(&self) -> i64 {
        self.last_enqueued_offset
    }

    /// The date and time, in UTC, that the last observed event was enqueued in the partition.
    pub fn last_enqueued_time(&self) -> &OffsetDateTime {
        &self.last_enqueued_time
    }

    /// Indicates whether or not the partition is currently empty.
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}
