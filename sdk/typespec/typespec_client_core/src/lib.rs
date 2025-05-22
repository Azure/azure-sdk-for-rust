// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
mod macros;
pub mod base64;
pub mod date;
pub mod error;
pub mod fmt;
pub mod fs;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "json")]
pub mod json;
pub mod serde;
pub mod sleep;
pub mod stream;
#[cfg(feature = "xml")]
pub mod xml;

pub use crate::error::{Error, Result};
pub use bytes::Bytes;
pub use uuid::Uuid;
