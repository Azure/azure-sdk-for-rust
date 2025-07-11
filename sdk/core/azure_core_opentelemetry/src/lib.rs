// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod attributes;
mod span;
mod telemetry;
mod tracer;

// Re-export the main types for convenience
pub use telemetry::OpenTelemetryTracerProvider;
