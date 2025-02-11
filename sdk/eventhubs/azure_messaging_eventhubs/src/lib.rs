// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![recursion_limit = "128"]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub(crate) mod common;

/// Types related to consuming events from an Event Hubs instance.
pub(crate) mod consumer;

/// Types related to errors processing events.
mod error;

/// Types to create and send events to an Event Hubs instance.
pub(crate) mod producer;

/// Types sent to and received from the Event Hubs service.
pub mod models;

pub use producer::{
    batch::{EventDataBatch, EventDataBatchOptions},
    ProducerClient, ProducerClientOptions, SendEventOptions, SendMessageOptions,
    SubmitBatchOptions,
};

pub use consumer::{
    ConsumerClient, ConsumerClientOptions, EventReceiver, OpenReceiverOptions, StartLocation,
    StartPosition,
};
