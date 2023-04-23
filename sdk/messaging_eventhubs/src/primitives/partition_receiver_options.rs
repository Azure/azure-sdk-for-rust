use std::time::Duration as StdDuration;

use crate::{EventHubConnectionOptions, EventHubsRetryOptions};

/// The default amount of time to wait for messages when reading
pub const DEFAULT_MAXIMUM_RECEIVE_WAIT_TIME: StdDuration = StdDuration::from_secs(60);

/// The default prefetch count
pub const DEFAULT_PREFETCH_COUNT: u32 = 300;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PartitionReceiverOptions {
    pub connection_options: EventHubConnectionOptions,
    pub retry_options: EventHubsRetryOptions,
    pub maximum_receive_wait_time: StdDuration,
    pub prefetch_count: u32,
    pub identifier: Option<String>,
    pub owner_level: Option<i64>,
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
