// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ECS-inspired operation pipeline for Cosmos DB operations.
//!
//! This module implements the data-oriented programming (DOP) pipeline described
//! in the Transport Pipeline Spec. State is decomposed into focused component types
//! and pipeline stages are pure functions over those components.

pub(crate) mod components;
pub(crate) mod operation_pipeline;
pub(crate) mod retry_evaluation;
