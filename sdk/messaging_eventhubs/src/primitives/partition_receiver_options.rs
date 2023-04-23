use std::time::Duration as StdDuration;

use crate::{EventHubConnectionOptions, EventHubsRetryOptions};

/// The default amount of time to wait for messages when reading
pub const DEFAULT_MAXIMUM_RECEIVE_WAIT_TIME: StdDuration = StdDuration::from_secs(60);

/// The default prefetch count
pub const DEFAULT_PREFETCH_COUNT: u32 = 300;

/// The set of options that can be specified when creating a
/// [`crate::primitives::PartitionReceiver`]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PartitionReceiverOptions {
    /// The set of options that can be specified when creating an [`crate::EventHubConnection`]
    pub connection_options: EventHubConnectionOptions,

    /// The set of options to govern retry behavior and try timeouts.
    pub retry_options: EventHubsRetryOptions,

    /// The amount of time to wait for messages when reading
    pub maximum_receive_wait_time: StdDuration,

    /// The number of events that will be eagerly requested from the Event Hubs service and queued
    /// locally without regard to whether a read operation is currently active, intended to help
    /// maximize throughput by allowing events to be read from from a local cache rather than
    /// waiting on a service request.
    pub prefetch_count: u32,

    /// The identifier of the receiver. If not specified, a UUID will be used.
    pub identifier: Option<String>,

    /// When populated, the owner level indicates that a reading is intended to be performed
    /// exclusively for events in the requested partition and for the associated consumer group.  To
    /// do so, reading will attempt to assert ownership over the partition; in the case where more
    /// than one exclusive reader attempts to assert ownership for the same partition/consumer group
    /// pair, the one having a larger value will "win."
    ///
    /// When an exclusive reader is used, other readers which are non-exclusive or which have a
    /// lower owner level will either not be allowed to be created, if they already exist, will
    /// encounter an exception during the next attempted operation.
    pub owner_level: Option<i64>,

    /// Indicates whether or not the reader should request information on the last enqueued event on
    /// the partition associated with a given event, and track that information as events are read.
    pub track_last_enqueued_event_properties: bool,
}

impl Default for PartitionReceiverOptions {
    fn default() -> Self {
        Self {
            connection_options: Default::default(),
            retry_options: Default::default(),
            maximum_receive_wait_time: DEFAULT_MAXIMUM_RECEIVE_WAIT_TIME,
            prefetch_count: DEFAULT_PREFETCH_COUNT,
            identifier: None,
            owner_level: None,
            track_last_enqueued_event_properties: true,
        }
    }
}
