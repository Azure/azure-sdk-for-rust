use std::time::Duration;

/// The default number of events that will be read from the Event Hubs service and held in a local memory
pub const DEFAULT_CACHE_EVENT_COUNT: u32 = 100;

/// The default number of events that will be eagerly requested from the Event Hubs service when reading is active and
pub const DEFAULT_PREFETCH_COUNT: u32 = 300;

/// The set of options that can be specified to configure behavior when reading events from an
/// `EventHubConsumerClient`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadEventOptions {
    /// The maximum amount of time to wait to for an event to be available when reading before
    /// reading an empty event.
    ///
    /// If specified, should there be no events available before this waiting period expires, an
    /// empty event will be returned, allowing control to return to the reader that was waiting.
    ///
    /// If `None`, the reader will wait forever for items to be available unless reading is
    /// canceled. Empty items will not be returned.
    pub maximum_wait_time: Option<Duration>,

    /// The maximum number of events that will be read from the Event Hubs service and held in a
    /// local memory cache when reading is active and events are being emitted to an enumerator for
    /// processing.
    ///
    /// Default to [`Some(DEFAULT_CACHE_EVENT_COUNT)`]
    pub cache_event_count: u32,

    /// The number of events that will be eagerly requested from the Event Hubs service and queued
    /// locally without regard to whether a read operation is currently active, intended to help
    /// maximize throughput by allowing events to be read from from a local cache rather than
    /// waiting on a service request.
    ///
    /// Default to [`DEFAULT_PREFETCH_COUNT`]
    pub prefetch_count: u32,

    /// The desired number of bytes to attempt to eagerly request from the Event Hubs service and
    /// queued locally without regard to whether a read operation is currently active, intended to
    /// help maximize throughput by allowing events to be read from from a local cache rather than
    /// waiting on a service request.
    ///
    /// When set to `None`, the option is considered disabled; otherwise, it will be considered
    /// enabled and take precedence over any value specified for the `prefetch_count` The
    /// `prefetch_size_in_bytes` is an advanced control that developers can use to help tune
    /// performance in some scenarios; it is recommended to prefer using the `prefetch_count` over
    /// this option where possible for more accurate control and more predictable throughput.
    ///
    /// This size should be considered a statement of intent rather than a guaranteed limit; the
    /// local cache may be larger or smaller than the number of bytes specified, and will always
    /// contain at least one event when the `prefetch_size_in_bytes` is specified.  A heuristic is
    /// used to predict the average event size to use for size calculations, which should be
    /// expected to fluctuate as traffic passes through the system.  Consequently, the resulting
    /// resource use will fluctuate as well.
    pub prefetch_size_in_bytes: Option<usize>,

    /// When populated, the owner level indicates that a reading is intended to be performed
    /// exclusively for events in the requested partition and for the associated consumer group.  To
    /// do so, reading will attempt to assert ownership over the partition; in the case where more
    /// than one exclusive reader in the consumer group attempts to assert ownership for the same
    /// partition, the one having a larger `owner_level` value will "win".
    ///
    /// When an exclusive reader is used, other readers which are non-exclusive or which have a
    /// lower owner level will either not be allowed to be created, if they already exist, will
    /// encounter an exception during the next attempted operation.
    pub owner_level: Option<i64>,

    /// Indicates whether or not the reader should request information on the last enqueued event on
    /// the partition associated with a given event, and track that information as events are read.
    ///
    /// When information about a partition's last enqueued event is being tracked, each event
    /// received from the Event Hubs service will carry metadata about the partition that it
    /// otherwise would not. This results in a small amount of additional network bandwidth
    /// consumption that is generally a favorable trade-off when considered against periodically
    /// making requests for partition properties using one of the Event Hub clients.
    pub track_last_enqueued_event_properties: bool,
}

impl Default for ReadEventOptions {
    fn default() -> Self {
        Self {
            maximum_wait_time: None,
            cache_event_count: DEFAULT_CACHE_EVENT_COUNT,
            prefetch_count: DEFAULT_PREFETCH_COUNT,
            prefetch_size_in_bytes: None,
            owner_level: None,
            track_last_enqueued_event_properties: true,
        }
    }
}
