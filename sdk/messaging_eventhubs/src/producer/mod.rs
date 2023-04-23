//! Producer client

// TODO: mod enqueue_event_options;
// TODO: mod event_hub_buffered_producer_client;
// TODO: mod event_hub_buffered_producer_client_options;

mod create_batch_options;
mod event_data_batch;
mod event_hub_producer_client;
mod event_hub_producer_client_options;
mod partition_publishing_options;
mod send_event_options;

pub(crate) use partition_publishing_options::PartitionPublishingOptions;

pub use create_batch_options::CreateBatchOptions;
pub use event_data_batch::EventDataBatch;
pub use event_hub_producer_client::{EventHubProducerClient, MINIMUM_BATCH_SIZE_LIMIT};
pub use event_hub_producer_client_options::EventHubProducerClientOptions;
pub use send_event_options::SendEventOptions;
