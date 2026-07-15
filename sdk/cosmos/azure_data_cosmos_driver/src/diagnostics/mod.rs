// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation diagnostics for the Cosmos DB driver.
//!
//! The driver produces one canonical [`DiagnosticsContext`] per operation, always collected and
//! surfaced via [`CosmosResponse::diagnostics`](crate::models::CosmosResponse::diagnostics). The
//! context carries the operation-level outcome plus the per-attempt [`RequestDiagnostics`], and
//! exposes the predicates and accessors a higher-level SDK uses to decide whether to emit telemetry.
//!
//! This module is the plain foundation only: it collects diagnostics into a context but never emits
//! metrics, traces, or logs itself.

mod context;
mod request;

pub use context::DiagnosticsContext;
pub use request::{ExecutionContext, RequestDiagnostics};

pub(crate) use context::DiagnosticsContextBuilder;
