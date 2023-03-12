use time::OffsetDateTime;

use super::error::OffsetIsEmpty;

const START_OF_STREAM_OFFSET: &str = "-1";
const END_OF_STREAM_OFFSET: &str = "@latest";

/// The position of events in an Event Hub partition, typically used in the creation of
/// an [`EventHubConsumerClient`].
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum EventPosition {
    /// The offset of the event identified by this position.
    Offset {
        /// The offset of the event identified by this position.
        offset: String,
        /// Indicates if the specified offset is inclusive of the event which it identifies.  This
        /// information is only relevant if the event position was identified by an offset or sequence number.
        is_inclusive: bool,
    },
    /// The sequence number of the event identified by this position.
    SequenceNumber {
        /// The sequence number of the event identified by this position.
        sequence_number: i64,
        /// Indicates if the specified offset is inclusive of the event which it identifies.  This
        /// information is only relevant if the event position was identified by an offset or sequence number.
        is_inclusive: bool,
    },
    /// The enqueue time of the event identified by this position.
    EnqueuedTime(OffsetDateTime),
}

impl EventPosition {
    /// Corresponds to the location of the first event present in the partition.  Use this
    /// position to begin receiving from the first event that was enqueued in the partition
    /// which has not expired due to the retention policy.
    pub fn earliest() -> Self {
        Self::Offset {
            offset: START_OF_STREAM_OFFSET.to_string(),
            is_inclusive: false,
        }
    }

    /// Corresponds to the end of the partition, where no more events are currently enqueued.  Use this
    /// position to begin receiving from the next event to be enqueued in the partition after an event
    /// consumer begins reading with this position.
    pub fn latest() -> Self {
        Self::Offset {
            offset: END_OF_STREAM_OFFSET.to_string(),
            is_inclusive: false,
        }
    }

    /// Corresponds to the event in the partition at the provided offset. Returns an error if the offset is empty.
    ///
    /// # Parameters
    ///
    /// - `offset` - The offset of an event with respect to its relative position in the partition.
    /// - `is_inclusive` - >If true, the event at the `offset` is included; otherwise the next event in sequence will be received.
    pub fn try_from_offset(offset: impl Into<String>, is_inclusive: bool) -> Result<Self, OffsetIsEmpty> {
        let offset = offset.into();
        if offset.is_empty() {
            return Err(OffsetIsEmpty);
        }
        Ok(Self::Offset {
            offset,
            is_inclusive,
        })
    }

    /// Corresponds to an event with the specified sequence number in the partition.  By default, the event
    /// with this `sequence_number` will be read.  Setting `is_inclusive` to
    /// `false` will skip the event with that sequence number and begin reading at the next available event.
    ///
    /// # Parameters
    ///
    /// - sequence_number - The sequence number assigned to an event when it was enqueued in the partition
    /// - is_inclusive - When `true`, the event with the sequence_number is included; otherwise the next event in sequence will be read
    pub fn from_sequence_number(sequence_number: i64, is_inclusive: bool) -> Self {
        Self::SequenceNumber {
            sequence_number,
            is_inclusive,
        }
    }

    /// Corresponds to a specific date and time within the partition to begin seeking an event; the event enqueued on or after
    /// the specified `enqueued_time` will be read.
    ///
    /// # Parameters
    ///
    /// - enqueued_time - The date and time, in UTC, from which the next available event should be chosen
    pub fn from_enqueued_time(enqueued_time: OffsetDateTime) -> Self {
        Self::EnqueuedTime(enqueued_time)
    }
}

