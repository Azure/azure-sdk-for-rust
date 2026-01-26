// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(feature = "fault_injection")]
mod fault_injection_utils;

#[cfg(feature = "fault_injection")]
mod fault_http_client;

#[cfg(feature = "fault_injection")]
pub use crate::fault_injection::fault_injection_utils::*;