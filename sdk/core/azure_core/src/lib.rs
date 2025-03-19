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

// Re-export modules in typespec_client_core such that azure_core-based crates don't need to reference it directly.
pub use typespec_client_core::{
    base64, date,
    error::{self, Error, Result},
    fmt, json, parsing, sleep, stream, Bytes, Uuid,
};

#[cfg(feature = "xml")]
pub use typespec_client_core::xml;
