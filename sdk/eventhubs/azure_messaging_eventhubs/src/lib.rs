// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![recursion_limit = "128"]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub(crate) mod common;
pub(crate) mod consumer;

/// Types related to errors processing events.
pub(crate) mod error;

/// Types to create and send events to an Event Hubs instance.
pub(crate) mod producer;

pub(crate) mod processor;

/// Types sent to and received from the Event Hubs service.
pub mod models;

pub use producer::{
    batch::{EventDataBatch, EventDataBatchOptions},
    ProducerClient, SendBatchOptions, SendEventOptions, SendMessageOptions,
};

pub use consumer::{
    ConsumerClient, EventReceiver, OpenReceiverOptions, StartLocation, StartPosition,
};

/// Builders for producer client and consumer client.
pub mod builders {
    pub use crate::consumer::builders::ConsumerClientBuilder;
    pub use crate::producer::builders::ProducerClientBuilder;
}

pub use crate::error::{ErrorKind, EventHubsError};
