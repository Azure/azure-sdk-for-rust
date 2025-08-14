// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
mod macros;

mod constants;
pub mod credentials;
pub mod hmac;
pub mod http;

#[cfg(feature = "test")]
pub mod test;

pub use constants::*;

// Re-export modules in typespec_client_core such that azure_core-based crates don't need to reference it directly.
pub use typespec_client_core::{
    async_runtime, base64, create_enum, create_extensible_enum,
    error::{self, Error, Result},
    fmt, json, sleep, stream, time, Bytes, Uuid,
};

/// Abstractions for distributed tracing and telemetry.
pub mod tracing {
    pub use crate::http::policies::PublicApiInstrumentationInformation;
    pub use azure_core_macros::{client, function, new, subclient};
    pub use typespec_client_core::tracing::{
        AsAny, Attribute, AttributeArray, AttributeValue, Span, SpanGuard, SpanKind, SpanStatus,
        Tracer, TracerProvider,
    };
}

#[cfg(feature = "xml")]
pub use typespec_client_core::xml;
