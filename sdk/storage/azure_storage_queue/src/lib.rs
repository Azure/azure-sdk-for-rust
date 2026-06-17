// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

#[allow(unused_imports)]
mod generated;

mod logging;

pub mod clients;
pub mod models;

pub use clients::{QueueClient, QueueClientOptions, QueueServiceClient, QueueServiceClientOptions};
