// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Some tests don't use all the features of this module.
#![allow(dead_code)]

//! Provides a framework for integration tests for the Azure Cosmos DB service.
//!
//! The framework allows tests to easily run against real Cosmos DB instances, the local emulator, or a mock server using test-proxy.

mod local_recorder;
mod test_account;
pub mod test_data;

#[cfg(feature = "preview_query_engine")]
pub mod query_engine;

pub use local_recorder::{LocalRecorder, Transaction};
pub use test_account::{TestAccount, TestAccountOptions};

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
