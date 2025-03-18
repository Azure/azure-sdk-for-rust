// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

mod constants;
pub mod credentials;
pub mod fs;
pub mod hmac;
pub mod http;
pub mod process;
#[cfg(feature = "test")]
pub mod test;

pub use constants::*;

// Re-export typespec types that are not specific to Azure.
pub use typespec::{Error, Result};

/// Client-specific error functions.
pub mod error {
    pub use typespec::error::*;
    pub use typespec_client_core::error::*;
}
pub use typespec_client_core::fmt;
pub use typespec_client_core::{
    base64, date, json, parsing, sleep,
    stream::{BytesStream, SeekableStream, DEFAULT_BUFFER_SIZE},
    Bytes, Uuid,
};

#[cfg(feature = "xml")]
pub use typespec_client_core::xml;

/// A unique identifier for a request.
// NOTE: Only used for Storage?
pub type RequestId = Uuid;

/// A unique session token.
// NOTE: Only used for Cosmos?
pub type SessionToken = String;

/// An empty HTTP body.
#[allow(clippy::declare_interior_mutable_const)]
pub const EMPTY_BODY: bytes::Bytes = bytes::Bytes::new();
