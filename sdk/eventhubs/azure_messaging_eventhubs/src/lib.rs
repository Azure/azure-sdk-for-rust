// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![recursion_limit = "128"]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub(crate) mod common;
pub(crate) mod consumer;
pub(crate) mod error;
pub(crate) mod event_processor;
pub(crate) mod producer;

/// Types sent to and received from the Event Hubs service.
pub mod models;

pub use producer::{
    batch::{EventDataBatch, EventDataBatchOptions},
    ProducerClient, SendBatchOptions, SendEventOptions, SendMessageOptions,
};

pub use consumer::{
    ConsumerClient, EventReceiver, OpenReceiverOptions, StartLocation, StartPosition,
};

/// Event Hubs processor related types.
pub mod processor {
    pub use crate::event_processor::processor::{
        ClaimOwnershipOptions, ListCheckpointsOptions, ListOwnershipOptions,
    };
}

pub use event_processor::processor::{CheckpointStore, EventProcessor, ProcessorStrategy};

/// Builders for producer client and consumer client.
pub mod builders {
    pub use crate::consumer::builders::ConsumerClientBuilder;
    pub use crate::producer::builders::ProducerClientBuilder;
}

pub use crate::error::{ErrorKind, EventHubsError};
