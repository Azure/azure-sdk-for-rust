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

// Docs.rs build is done with the nightly compiler, so we can enable nightly features in that build.
// In this case we enable two features:
// - `doc_auto_cfg`: Automatically scans `cfg` attributes and uses them to show those required configurations in the generated documentation.
// - `doc_cfg_hide`: Ignore the `doc` configuration for `doc_auto_cfg`.
// See https://doc.rust-lang.org/rustdoc/unstable-features.html#doc_auto_cfg-automatically-generate-doccfg for more details.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]

#[macro_use]
mod macros;

mod constants;
pub mod hmac;
mod models;
mod options;
mod pipeline;
mod policies;

pub mod credentials;
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
pub use typespec_client_core::http::{
    LazyResponse, PinnedStream, Response, ResponseBody, ResponseFuture,
};

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
    Uuid,
};

/// A unique identifier for a request.
// NOTE: Only used for Storage?
pub type RequestId = Uuid;

/// A unique session token.
// NOTE: Only used for Cosmos?
pub type SessionToken = String;

/// An empty HTTP body.
#[allow(clippy::declare_interior_mutable_const)]
pub const EMPTY_BODY: bytes::Bytes = bytes::Bytes::new();

#[doc(hidden)]
/// Used by macros as an implementation detail
pub mod __private {
    pub use paste::paste;
}
