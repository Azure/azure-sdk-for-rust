// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

pub mod error;
#[cfg(feature = "http")]
pub mod http;

pub use typespec::error::*;
