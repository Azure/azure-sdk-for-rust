// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ARM-specific HTTP pipeline policies.
//!
//! This module provides policies specific to Azure Resource Manager (ARM) operations.

mod retry;
mod rp_registration;

pub use retry::*;
pub use rp_registration::*;
