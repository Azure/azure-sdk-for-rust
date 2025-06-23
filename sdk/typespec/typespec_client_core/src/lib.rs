// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
mod macros;
pub mod async_runtime;
pub mod base64;
pub mod error;
pub mod fmt;
pub mod fs;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "json")]
pub mod json;
pub mod sleep;
pub mod stream;
pub mod time;
pub mod tracing;
#[cfg(feature = "xml")]
pub mod xml;

pub use crate::error::{Error, Result};
pub use bytes::Bytes;
pub use uuid::Uuid;

pub use sleep::sleep;
