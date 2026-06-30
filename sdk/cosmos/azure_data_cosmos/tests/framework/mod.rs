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

pub mod emulator_credential;
pub mod mock_account;
pub mod test_client;
pub mod test_data;

pub use emulator_credential::{CosmosEmulatorCredential, CredentialRecorder};
pub use test_client::{
    assert_local_retry_attempted_on_region, assert_region_contacted_with_retry,
    assert_region_not_contacted, get_effective_hub_endpoint, get_global_endpoint,
    resolve_connection_string, TestClient, TestOptions, TestRunContext, CONNECTION_STRING_ENV_VAR,
    DEFAULT_TEST_TIMEOUT, EMULATOR_CONNECTION_STRING, HUB_REGION, SATELLITE_REGION,
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

/// Error type returned by tests when they cannot conclusively determine pass or fail status, such as when a split doesn't complete within the expected time.
#[derive(PartialEq, Eq)]
pub enum InconclusiveError {
    /// The test was inconclusive because a partition split did not complete within the expected time.
    SplitNotCompleted,
}

// The Debug format is used when a test returns an error, so we need to include some context in the logs to make it clear.
impl std::fmt::Debug for InconclusiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InconclusiveError::SplitNotCompleted => {
                write!(f, "InconclusiveError::SplitNotCompleted")
            }
        }
    }
}

impl std::fmt::Display for InconclusiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InconclusiveError::SplitNotCompleted => write!(
                f,
                "inconclusive: partition split did not complete within the expected time"
            )?,
        }
        write!(
            f,
            " (an inconclusive result does NOT indicate a failure, only that this test couldn't complete because of backend delays or intermittent issues; this failure does NOT need to block PR merges)"
        )
    }
}

impl std::error::Error for InconclusiveError {}
