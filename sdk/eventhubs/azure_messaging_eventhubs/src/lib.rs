// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![recursion_limit = "128"]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "in_memory_checkpoint_store")]
mod in_memory_checkpoint_store;

mod common;
mod consumer;
/// Error types for the Event Hubs service.
pub mod error;
mod event_processor;
mod producer;

/// Types sent to and received from the Event Hubs service.
pub mod models;
pub use consumer::{
    ConsumerClient, EventReceiver, OpenReceiverOptions, StartLocation, StartPosition,
};
pub use producer::{
    batch::{EventDataBatch, EventDataBatchOptions},
    ProducerClient, SendBatchOptions, SendEventOptions, SendMessageOptions,
};

/// Event Hubs processor related types.
pub mod processor {
    pub use crate::event_processor::partition_client::PartitionClient;
    pub use crate::event_processor::CheckpointStore;
}
pub use event_processor::{processor::EventProcessor, CheckpointStore, ProcessorStrategy};
/// Builders for producer client and consumer client.
pub mod builders {
    pub use crate::consumer::builders::ConsumerClientBuilder;
    pub use crate::event_processor::processor::builders::EventProcessorBuilder;
    pub use crate::producer::builders::ProducerClientBuilder;
}
pub use common::retry::RetryOptions;
pub use error::{EventHubsError, Result};

#[cfg(feature = "in_memory_checkpoint_store")]
pub use in_memory_checkpoint_store::InMemoryCheckpointStore;
