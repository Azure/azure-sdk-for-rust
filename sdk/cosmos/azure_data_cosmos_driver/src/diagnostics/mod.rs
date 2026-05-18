// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Diagnostic and telemetry types for Cosmos DB operations.
//!
//! This module provides rich diagnostic information about Cosmos DB operations,
//! similar to [CosmosDiagnosticsContext](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/CosmosDiagnosticsContext.java)
//! in the Java SDK.
//!
//! Diagnostics are **operational metadata** tracked by the SDK, not service resources.
//!
//! # Architecture
//!
//! - `DiagnosticsContextBuilder` (internal): Mutable builder used during operation execution
//! - [`DiagnosticsContext`]: Immutable, finalized diagnostics returned to callers
//!
//! The builder is `pub(crate)` and used internally by the driver to collect diagnostics.
//! When an operation completes, the builder is consumed to create an immutable
//! `DiagnosticsContext` which is safe to share via `Arc` without locking.

mod diagnostics_context;
mod error_diagnostics;
mod proxy_configuration;

pub(crate) use diagnostics_context::DiagnosticsContextBuilder;
pub use diagnostics_context::{
    DiagnosticsContext, ExecutionContext, FailedTransportShardDiagnostics, PipelineType,
    RequestDiagnostics, RequestEvent, RequestEventType, RequestHandle, RequestSentStatus,
    TransportHttpVersion, TransportKind, TransportSecurity, TransportShardDiagnostics,
};
// The error-diagnostics carrier and helpers are wrapper-crate plumbing.
// They are reachable from `azure_data_cosmos` (which wraps them behind
// `CosmosError`) but are not part of this driver crate's intended
// public surface — `#[doc(hidden)]` keeps them out of the published
// rustdoc so they do not become semver-stable surface area that
// external driver-crate consumers can rely on.
#[doc(hidden)]
pub use error_diagnostics::{attach_diagnostics, split_diagnostics_carrier};
pub use proxy_configuration::ProxyConfiguration;
