// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Emulator-based E2E tests for azure_data_cosmos_driver.
//!
//! These tests require a running Cosmos DB emulator. Most tests are ignored
//! unless either `test_category = "emulator"` (the legacy Windows emulator)
//! or `test_category = "emulator_vnext"` (the new Linux vnext emulator) is
//! set. A subset of tests that depend on legacy-only behavior (multi-endpoint
//! topology, partition-failover semantics) are gated on `"emulator"` only and
//! remain ignored under `"emulator_vnext"`; those files document the reason
//! in their module headers.

// The framework module is shared across test binaries; not all exports are used
// by every binary.
#![allow(dead_code, unused_imports)]

mod emulator_tests;
mod framework;
