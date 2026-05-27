// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// The whole test binary is gated behind `__internal_in_memory_emulator`.
// Cargo does not gate dev-dependencies by feature, so dev-deps that are only
// used by the emulator tests (e.g. `uuid`) would otherwise be pulled into the
// dependency graph for every `cargo test` invocation. Gating the entire
// translation unit keeps those deps unused when the feature is off.
#![cfg(feature = "__internal_in_memory_emulator")]

mod in_memory_emulator_tests;
