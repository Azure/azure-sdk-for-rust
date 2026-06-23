// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(unused_imports)]
mod generated;

mod logging;

pub mod clients;
pub mod models;

#[cfg(feature = "sas_builder")]
mod sas;

pub use clients::{QueueClient, QueueClientOptions, QueueServiceClient, QueueServiceClientOptions};
