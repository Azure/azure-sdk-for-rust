// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

pub mod async_runtime;
pub mod base64;
pub mod error;
pub mod fmt;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "json")]
pub use typespec::json;
pub mod sleep;
pub mod stream;
pub mod time;
#[cfg(feature = "http")]
pub mod tracing;
#[cfg(feature = "xml")]
pub use typespec::xml;

pub use crate::error::{Error, Result};
pub use typespec::Bytes;
pub use uuid::Uuid;

pub use sleep::sleep;

mod private {
    pub trait Sealed {}
    impl Sealed for crate::http::Url {}
}
