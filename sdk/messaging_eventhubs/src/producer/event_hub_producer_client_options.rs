use std::collections::HashMap;

use crate::{
    core::TransportProducerFeatures, event_hubs_retry_options::EventHubsRetryOptions,
    EventHubConnectionOptions,
};

use super::PartitionPublishingOptions;

/// The set of options that can be specified when creating an Event Hub producer.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EventHubProducerClientOptions {
    /// The set of options that can be specified when creating an Event Hub connection.
    pub connection_options: EventHubConnectionOptions,

    /// The set of options that can be specified when retrying operations.
    pub retry_options: EventHubsRetryOptions,

    /// The identifier of the producer. If not specified, a UUID will be generated.
    pub identifier: Option<String>,

    /// The set of options that can be specified when publishing events to a specific partition.
    pub partition_options: HashMap<String, PartitionPublishingOptions>,
}

impl EventHubProducerClientOptions {
    pub(crate) fn create_features(&self) -> TransportProducerFeatures {
        TransportProducerFeatures::None
    }

    pub(crate) fn get_publishing_options_or_default_for_partition(
        &self,
        partition_id: Option<&str>,
    ) -> PartitionPublishingOptions {
        partition_id
            .and_then(|id| self.partition_options.get(id))
            .cloned()
            .unwrap_or_default()
    }
}
