// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This module contains types and utilities for working with HTTP status codes.

pub mod headers;
pub mod response;
mod status_code;

pub use headers::DEFAULT_ALLOWED_HEADER_NAMES;
pub use response::RawResponse;
pub use status_code::StatusCode;

/// Default pattern for redacted headers or query parameters.
pub const REDACTED_PATTERN: &str = "REDACTED";
