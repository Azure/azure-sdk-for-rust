// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure Core OpenTelemetry tracing integration.
//!
//! This crate provides OpenTelemetry distributed tracing support for the Azure SDK for Rust.
//! It bridges the standardized typespec_client_core tracing traits with OpenTelemetry implementation,
//! enabling automatic span creation, context propagation, and telemetry collection for Azure services.

mod attributes;
mod conventions;
mod span;
pub mod telemetry;
mod tracer;

// Re-export the main types for convenience
pub use telemetry::OpenTelemetryTracerProvider;
