// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Emulator-based E2E tests for azure_data_cosmos_driver.
//!
//! These tests require a running Cosmos DB emulator and are ignored when
//! the `test_category = "emulator"` configuration is not set.

// The framework module is shared across test binaries; not all exports are used
// by every binary.
#![allow(dead_code, unused_imports)]

mod emulator_tests;
mod framework;
