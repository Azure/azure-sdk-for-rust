// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multi-region live-account tests for azure_data_cosmos_driver.
//!
//! These tests require a multi-region Cosmos DB account and are ignored when
//! the `test_category = "multi_region"` configuration is not set.

#![cfg(test_category = "multi_region")]
// The framework module is shared across test binaries; not all exports are used
// by every binary.
#![allow(dead_code, unused_imports)]

mod framework;
mod multi_region_tests;
