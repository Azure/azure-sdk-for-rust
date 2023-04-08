use std::collections::HashMap;

use crate::{EventHubConnectionOptions, event_hubs_retry_options::EventHubsRetryOptions, core::transport_producer_features::TransportProducerFeatures};

use super::PartitionPublishingOptions;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EventHubProducerClientOptions {
    pub connection_options: EventHubConnectionOptions,
    pub retry_options: EventHubsRetryOptions,
    pub identifier: Option<String>,
    pub enable_idempotent_partitions: bool,
    pub partition_options: HashMap<String, PartitionPublishingOptions>
}

impl EventHubProducerClientOptions {
    pub(crate) fn create_features(&self) -> TransportProducerFeatures {
        match self.enable_idempotent_partitions {
            true => TransportProducerFeatures::IdempotentPublishing,
            false => TransportProducerFeatures::None
        }
    }

    pub(crate) fn get_publishing_options_or_default_for_partition(&self, partition_id: Option<&str>) -> PartitionPublishingOptions {
        partition_id.and_then(|id| self.partition_options.get(id)).cloned().unwrap_or_default()
    }
}
