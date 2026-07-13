// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cross-crate dual-backend integration tests that drive the in-memory
//! emulator (from azure_data_cosmos_driver) through the public
//! azure_data_cosmos client surface and (optionally) compare against a
//! real Cosmos DB account.

use std::time::Duration;

pub mod driver_end_to_end;
#[cfg(feature = "preview_dtx")]
pub mod dtx_live_comparison;
#[cfg(feature = "preview_dtx")]
pub mod dtx_sdk_validation;
pub mod dual_backend;
pub mod end_to_end;
pub mod hpk;
pub mod query_comparison;
pub mod session_token;
pub mod user_agent;
pub mod validation;

/// Environment variable controlling the bounded live-account setup readiness window.
const SETUP_TIMEOUT_SECONDS_ENV_VAR: &str = "AZURE_COSMOS_TEST_SETUP_TIMEOUT_SECONDS";

/// Default live-account setup readiness window.
const DEFAULT_SETUP_TIMEOUT_SECONDS: u64 = 180;

/// Returns the bounded live-account setup readiness window shared by the
/// dual-backend suites, overridable via the
/// `AZURE_COSMOS_TEST_SETUP_TIMEOUT_SECONDS` environment variable.
fn setup_timeout() -> Duration {
    std::env::var(SETUP_TIMEOUT_SECONDS_ENV_VAR)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|seconds| *seconds > 0)
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(DEFAULT_SETUP_TIMEOUT_SECONDS))
}
