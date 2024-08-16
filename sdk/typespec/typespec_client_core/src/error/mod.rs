// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Client-specific error functions.

#[cfg(feature = "http")]
mod http_error;

#[cfg(feature = "http")]
pub use http_error::*;
pub use typespec::error::*;
