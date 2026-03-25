// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory Cosmos DB emulator for deterministic testing.
//!
//! Provides a fully in-memory implementation of the Cosmos DB HTTP transport layer,
//! enabling tests to run without network access, external accounts, or Docker containers.
//!
//! The emulator intercepts requests at the [`azure_core::http::HttpClient`] boundary,
//! so the entire operation pipeline (routing, session management, retry, failover)
//! executes normally — only the final HTTP I/O is replaced.
//!
//! # Feature Gate
//!
//! All code in this module is gated behind the `in_memory_emulator` feature flag.

mod client;
mod config;
mod dispatch;
mod epk;
mod operations;
mod response;
mod rid;
mod ru_model;
mod session;
mod store;
mod system_properties;

pub use client::InMemoryEmulatorHttpClient;
pub use config::{
    ConsistencyLevel, ContainerConfig, ReplicationConfig, VirtualAccountConfig, VirtualRegion,
    WriteMode,
};
pub use epk::Epk;
pub use ru_model::RuChargingModel;
pub use store::EmulatorStore;
