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

mod proxy_configuration;

/// Deferred, threshold-gated diagnostics capture — the driver's diagnostics **engine**.
///
/// This module **owns** the canonical diagnostics model ([`DiagnosticsContext`] and its builder)
/// and provides a cheap, append-only, lock-free hot-path recorder plus an operation-end gate
/// (`Off` / `Threshold` / `Always`). The driver collects diagnostics by feeding the capture-owned
/// `DiagnosticsContextBuilder`; the gate decides whether the resulting
/// [`DiagnosticsContext`](capture::DiagnosticsContext) is
/// surfaced. The model is re-exported below so the public boundary (`diagnostics::DiagnosticsContext`,
/// consumed by the `azure_data_cosmos` SDK) is unchanged. See `DIAGNOSTICS-CAPTURE.md`.
pub mod capture;

pub(crate) use capture::DiagnosticsContextBuilder;
pub use capture::{CompactedRun, CompactionInfo};
pub use capture::{
    DiagnosticsContext, DiagnosticsSummary, ExecutionContext, FailedTransportShardDiagnostics,
    PipelineType, RequestDiagnostics, RequestEvent, RequestEventType, RequestHandle,
    RequestSentStatus, TransportHttpVersion, TransportKind, TransportSecurity,
    TransportShardDiagnostics,
};
pub use proxy_configuration::ProxyConfiguration;

pub use crate::driver::pipeline::hedging_diagnostics::{
    HedgeDiagnostics, HedgeTerminalState, HedgingStrategyConfig,
};
