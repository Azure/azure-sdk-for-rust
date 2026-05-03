// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multi-write live-account tests for `azure_data_cosmos_driver`.
//!
//! Gated by `test_category = "multi_write"` — requires a live multi-master
//! (multiple write regions enabled) Cosmos DB account, not the local emulator.

#![cfg(test_category = "multi_write")]
#![allow(dead_code, unused_imports)]

mod framework;
mod multi_write_tests;
