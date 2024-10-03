// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
// Docs.rs build is done with the nightly compiler, so we can enable nightly features in that build.
// In this case we enable two features:
// - `doc_auto_cfg`: Automatically scans `cfg` attributes and uses them to show those required configurations in the generated documentation.
// - `doc_cfg_hide`: Ignore the `doc` configuration for `doc_auto_cfg`.
// See https://doc.rust-lang.org/rustdoc/unstable-features.html#doc_auto_cfg-automatically-generate-doccfg for more details.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]

#[macro_use]
mod macros;
pub mod base64;
pub mod date;
pub mod error;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "json")]
pub mod json;
pub mod parsing;
pub mod sleep;
pub mod stream;
#[cfg(feature = "xml")]
pub mod xml;

pub use crate::error::{Error, Result};
pub use uuid::Uuid;
