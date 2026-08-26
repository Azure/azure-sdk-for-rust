// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ECS-inspired operation pipeline for Cosmos DB operations.
//!
//! This module implements the data-oriented programming (DOP) pipeline described
//! in the Transport Pipeline Spec. State is decomposed into focused component types
//! and pipeline stages are pure functions over those components.

pub(crate) mod components;
// Exists only to synthesize the PATCH handler's response from a local body.
#[cfg(feature = "preview_patch")]
pub(crate) mod from_local_body;
pub(crate) mod hedge_budget;
pub(crate) mod hedging_diagnostics;
pub(crate) mod hedging_eligibility;
pub(crate) mod operation_pipeline;
// Shared by the preview PATCH handler and the in-memory emulator's DTX patch
// handling, which is the only DTX consumer of local patch evaluation.
#[cfg(any(
    feature = "preview_patch",
    all(feature = "preview_dtx", feature = "__internal_in_memory_emulator")
))]
pub(crate) mod patch_eval;
#[cfg(feature = "preview_patch")]
pub(crate) mod patch_handler;
pub(crate) mod retry_evaluation;
