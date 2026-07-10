// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cross-crate dual-backend integration tests that drive the in-memory
//! emulator (from azure_data_cosmos_driver) through the public
//! azure_data_cosmos client surface and (optionally) compare against a
//! real Cosmos DB account.

pub mod driver_end_to_end;
#[cfg(feature = "preview_dtx")]
pub mod dtx_live_comparison;
#[cfg(feature = "preview_dtx")]
pub mod dtx_sdk_validation;
pub mod dual_backend;
pub mod end_to_end;
pub mod hpk;
pub mod session_token;
pub mod user_agent;
pub mod validation;
