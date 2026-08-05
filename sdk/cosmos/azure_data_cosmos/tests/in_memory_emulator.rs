// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// The `__internal_in_memory_emulator` feature is required for this test
// binary; it is enforced via `required-features` in `Cargo.toml`. That keeps
// the entire translation unit (and its dev-only deps such as `uuid`) out of
// the build when the feature is off.

// Test futures compose many operations and near clippy's `large_futures`
// threshold on tokio's large test stacks; allowed here (mirrors `tests/split.rs`).
#![allow(clippy::large_futures)]

mod in_memory_emulator_tests;
