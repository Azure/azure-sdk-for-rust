// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multi-region live-account tests for `azure_data_cosmos_driver`.
//!
//! Requires a live multi-region (multiple read regions) Cosmos DB account,
//! not the local emulator. Individual tests self-gate with
//! `#[cfg_attr(not(test_category = "multi_region"), ignore = ...)]` rather
//! than gating this root on the cfg, so the bodies keep compiling on ordinary
//! builds and bit-rot surfaces as a build break instead of silent skipping.

#[allow(dead_code, unused_imports)]
mod framework;

mod multi_region_tests;
