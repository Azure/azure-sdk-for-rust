// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![deny(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

#[macro_use]
mod macros;

pub mod cloud;
pub mod credentials;
pub mod error;
pub mod hmac;
pub mod http;
#[cfg(feature = "resourcemanager")]
pub mod resourcemanager;
#[cfg(feature = "test")]
pub mod test;

// Re-export modules in typespec_client_core such that azure_core-based crates don't need to reference it directly.
pub use typespec_client_core::{
    async_runtime, base64, fmt, json, sleep, stream, time, Bytes, Error, Result, Uuid, Value,
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

#[cfg(not(target_arch = "wasm32"))]
mod conditional_send {
    /// Conditionally implements [`Send`] based on the `target_arch`.
    ///
    /// This implementation requires `Send`.
    pub trait ConditionalSend: Send {}

    impl<T> ConditionalSend for T where T: Send {}
}

#[cfg(target_arch = "wasm32")]
mod conditional_send {
    /// Conditionally implements [`Send`] based on the `target_arch`.
    ///
    /// This implementation does not require `Send`.
    pub trait ConditionalSend {}

    impl<T> ConditionalSend for T {}
}

mod private {
    pub trait Sealed {}
}
