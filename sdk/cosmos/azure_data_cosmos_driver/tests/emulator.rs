// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Emulator-based E2E tests for azure_data_cosmos_driver.
//!
//! These tests require a running Cosmos DB emulator and are gated by
//! the `test_category = "emulator"` configuration.

#![cfg(test_category = "emulator")]

mod emulator_tests;
mod framework;
