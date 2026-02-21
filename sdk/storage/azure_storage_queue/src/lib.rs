// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

#[allow(unused_imports)]
mod generated;

mod logging;

/// Data models and types used by the Azure Storage Queue service.
///
/// This module contains all the request and response models, enums, and other data types
/// used when interacting with Azure Storage Queues, including queue messages, metadata,
/// and service properties.
pub mod models {
    pub use crate::generated::models::*;
}

/// Client implementations for interacting with Azure Storage Queue service.
///
/// This module provides high-level client APIs for managing queues and queue messages,
/// including operations like creating queues, sending/receiving messages, and managing
/// queue metadata.
pub mod clients;

pub use clients::{QueueClient, QueueClientOptions, QueueServiceClient, QueueServiceClientOptions};
