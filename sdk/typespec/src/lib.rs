// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

pub mod error;

pub use error::{Error, Result};

#[cfg(feature = "http")]
mod method;
#[cfg(feature = "http")]
pub use crate::method::Method;

#[cfg(feature = "http")]
mod status_code;
#[cfg(feature = "http")]
pub use crate::status_code::StatusCode;
