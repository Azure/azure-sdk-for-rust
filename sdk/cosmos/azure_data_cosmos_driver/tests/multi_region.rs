// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multi-region live-account tests for azure_data_cosmos_driver.
//!
//! These tests require a multi-region Cosmos DB account and are ignored when
//! the `test_category = "multi_region"` configuration is not set.

#![cfg(test_category = "multi_region")]

mod framework;
mod multi_region_tests;
