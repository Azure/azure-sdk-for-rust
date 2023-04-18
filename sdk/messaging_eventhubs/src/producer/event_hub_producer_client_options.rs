use std::collections::HashMap;

use crate::{
    core::TransportProducerFeatures, event_hubs_retry_options::EventHubsRetryOptions,
    EventHubConnectionOptions,
};

use super::PartitionPublishingOptions;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EventHubProducerClientOptions {
    pub connection_options: EventHubConnectionOptions,
    pub retry_options: EventHubsRetryOptions,
    pub identifier: Option<String>,
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
