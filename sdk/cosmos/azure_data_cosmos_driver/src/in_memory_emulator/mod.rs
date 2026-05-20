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
//! # Stability
//!
//! The entire `in_memory_emulator` module — every type, function, constant, and test
//! hook re-exported below (including `#[doc(hidden)]` items such as
//! [`EmulatorStore::pause_replication`], [`EmulatorStore::resume_replication`],
//! `force_session_not_available`, `split_partition`, `merge_partitions`, the
//! `test_headers` re-export, and friends) — is exposed **only** behind the
//! `__internal_in_memory_emulator` Cargo feature and is **not part of the public
//! API contract**. Names, signatures, and observable behavior may change in any
//! release without a deprecation cycle. The module exists to support deterministic
//! integration testing of `azure_data_cosmos` and `azure_data_cosmos_driver`; it is
//! not intended for use by external crates.
//!
//! # Feature Gate
//!
//! All code in this module is gated behind the `__internal_in_memory_emulator` feature flag.

mod client;
mod config;
mod dispatch;
mod epk;
mod observer;
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
    WriteMode, DEFAULT_MAX_BUFFERED_REPLICATIONS,
};
pub use epk::Epk;
pub use observer::RequestObserver;
#[doc(hidden)]
pub use response::headers as test_headers;
pub use ru_model::RuChargingModel;
pub use store::EmulatorStore;
