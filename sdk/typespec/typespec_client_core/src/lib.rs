// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

pub mod error;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "xml")]
pub mod xml;

pub use typespec::error::*;
