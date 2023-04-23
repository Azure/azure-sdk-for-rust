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
            owner_level: None,
            track_last_enqueued_event_properties: true,
        }
    }
}

impl ReadEventOptions {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum amount of time to wait to for an event to be available when reading before
    /// reading an empty event.
    pub fn with_maximum_wait_time(mut self, maximum_wait_time: Duration) -> Self {
        self.maximum_wait_time = Some(maximum_wait_time);
        self
    }

    /// Sets the cache event count.
    pub fn with_cache_event_count(mut self, cache_event_count: u32) -> Self {
        self.cache_event_count = cache_event_count;
        self
    }

    /// Sets the prefetch count.
    pub fn with_prefetch_count(mut self, prefetch_count: u32) -> Self {
        self.prefetch_count = prefetch_count;
        self
    }

    /// Sets the owner level.
    pub fn with_owner_level(mut self, owner_level: i64) -> Self {
        self.owner_level = Some(owner_level);
        self
    }

    /// Sets the track last enqueued event properties.
    pub fn with_track_last_enqueued_event_properties(
        mut self,
        track_last_enqueued_event_properties: bool,
    ) -> Self {
        self.track_last_enqueued_event_properties = track_last_enqueued_event_properties;
        self
    }
}
