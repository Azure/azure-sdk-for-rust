// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! E2E PATCH scaffolds.
//!
//! These tests are stubs that exercise the SDK-level `patch_item` surface
//! against an emulator-backed account. They are currently `#[ignore]`'d
//! because the driver test harness does not yet expose a high-level
//! `patch_item` helper — wiring them up is tracked as follow-up work in
//! `.coding-harness/implementation-state.json`.
//!
//! The test names map to scenarios in the original RMW spec:
//!
//! - **A5**: round-trip a basic `set` PATCH and observe the post-image.
//! - **A6**: `increment` preserves i64 fidelity.
//! - **A7**: 412-loop: the handler retries on a concurrent writer.
//! - **A8**: ops targeting partition-key paths are rejected.
//! - **A9**: large-but-not-pathological PatchSpec round-trips.
//! - **A12**: `move` between two scalar fields.
//! - **A13**: `remove` of a missing leaf surfaces a `PatchEvalError`.

use std::error::Error;

#[tokio::test]
#[ignore = "scaffold: requires patch helper in DriverTestClient (see implementation-state.json)"]
async fn a5_set_patch_returns_post_image() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::test]
#[ignore = "scaffold: requires patch helper in DriverTestClient (see implementation-state.json)"]
async fn a6_increment_preserves_i64_fidelity() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::test]
#[ignore = "scaffold: requires concurrency harness; see implementation-state.json"]
async fn a7_concurrent_writer_triggers_412_retry_and_succeeds() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::test]
#[ignore = "scaffold: requires patch helper in DriverTestClient (see implementation-state.json)"]
async fn a8_patch_on_partition_key_path_is_rejected() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::test]
#[ignore = "scaffold: requires patch helper in DriverTestClient (see implementation-state.json)"]
async fn a9_large_patch_spec_round_trips() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::test]
#[ignore = "scaffold: requires patch helper in DriverTestClient (see implementation-state.json)"]
async fn a12_move_between_scalar_fields() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::test]
#[ignore = "scaffold: requires patch helper in DriverTestClient (see implementation-state.json)"]
async fn a13_remove_missing_leaf_surfaces_eval_error() -> Result<(), Box<dyn Error>> {
    Ok(())
}
