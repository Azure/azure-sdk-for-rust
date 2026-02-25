// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(
    dead_code,
    reason = "Some tests don't use all the features of this module."
)]
#![allow(
    unused_imports,
    reason = "Some tests don't use all the features of this module."
)]

//! Provides a framework for integration tests for the Azure Cosmos DB service.
//!
//! The framework allows tests to easily run against real Cosmos DB instances, the local emulator, or a mock server using test-proxy.

#[cfg(feature = "fault_injection")]
pub mod mock_account;
pub mod test_client;
pub mod test_data;

pub use test_client::{
    get_effective_hub_endpoint, get_global_endpoint, TestClient, TestOptions, TestRunContext,
    DEFAULT_TEST_TIMEOUT, HUB_REGION, SATELLITE_REGION,
};

use serde::{Deserialize, Serialize};

/// Represents a single item in the mock engine.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MockItem {
    /// The ID of the item.
    pub id: String,
    /// The partition key of the item.
    pub partition_key: String,
    /// The global merge order of the item, which will be used by the mock query pipeline to sort items.
    pub merge_order: usize,
}
