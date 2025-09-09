// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use typespec_client_core::error::{Error, ErrorKind, Result, ResultExt};
mod error_response;

pub mod http {
    pub use super::error_response::*;
}
