// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![recursion_limit = "128"]
#![warn(missing_docs)]
// cspell: words amqp eventdata
#![doc = include_str!("../README.md")]

pub(crate) mod common;

/// Types related to consuming events from an Event Hub.
pub mod consumer;

/// Types related to errors processing events.
pub mod error;

/// Types to create and send events to an Event Hub.
pub mod producer;

/// Types sent to and received from the EventHubs service.
pub mod models;

pub use producer::batch::*;
pub use producer::ProducerClient;
pub use producer::ProducerClientOptions;
pub use producer::SendEventOptions;
pub use producer::SendMessageOptions;
pub use producer::SubmitBatchOptions;

pub use consumer::ConsumerClient;
pub use consumer::ConsumerClientOptions;
pub use consumer::OpenReceiverOptions;
pub use consumer::StartLocation;
pub use consumer::StartPosition;
