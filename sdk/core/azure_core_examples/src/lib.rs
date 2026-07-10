// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example client implementations for `azure_core` documentation and tests.
//!
//! This crate provides minimal example service clients to use in `azure_core` examples and tests,
//! avoiding circular dependencies on production service client crates.

pub mod certificates;
pub mod client;
pub mod identity;
pub mod secrets;

pub use azure_core::{credentials, error, http, Error, Result};
