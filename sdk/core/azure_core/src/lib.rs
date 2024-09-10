// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Core types and traits for the Rust Azure SDK.
//!
//! This crate is part of the unofficial Azure SDK effort in Rust. For more
//! information on the project and an overview of other crates, please refer to
//! [our GitHub repository](https://github.com/azure/azure-sdk-for-rust).
//!
//! It is a library that provides cross-cutting services to other client
//! libraries.  Please see the [general
//! guidelines](https://azure.github.io/azure-sdk/general_azurecore.html).

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
// #![warn(missing_docs, future_incompatible, unreachable_pub)]

#[macro_use]
mod macros;

mod constants;
pub mod hmac;
mod models;
mod options;
mod pipeline;
mod policies;

pub mod auth;
pub mod headers;
pub mod lro;
pub mod request_options;

pub mod tokio;

pub use constants::*;
#[doc(inline)]
pub use models::*;
pub use options::*;
pub use pipeline::*;
pub use policies::*;
pub use typespec_client_core::http::response::{Model, PinnedStream, Response, ResponseBody};

// Re-export typespec types that are not specific to Azure.
pub use typespec::{Error, Result};
pub mod error {
    pub use typespec::error::*;
    pub use typespec_client_core::error::*;
}
#[cfg(feature = "xml")]
pub use typespec_client_core::xml;
pub use typespec_client_core::{
    base64, date,
    http::{
        headers::Header, new_http_client, AppendToUrlQuery, Body, Context, Continuable, HttpClient,
        Method, Pageable, Request, RequestContent, StatusCode, Url,
    },
    json, parsing,
    sleep::{self, sleep},
    stream::{BytesStream, SeekableStream},
};

/// A unique identifier for a request.
// NOTE: only used for Storage?
pub type RequestId = typespec_client_core::Uuid;

/// A unique session token.
// NOTE: only used for Cosmos?
pub type SessionToken = String;

/// An empty HTTP body.
#[allow(clippy::declare_interior_mutable_const)]
pub const EMPTY_BODY: bytes::Bytes = bytes::Bytes::new();

#[doc(hidden)]
/// Used by macros as an implementation detail
pub mod __private {
    pub use paste::paste;
}
