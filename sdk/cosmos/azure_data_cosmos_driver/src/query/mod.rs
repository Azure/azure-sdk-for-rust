// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB SQL query parser, partition key extraction, and in-memory evaluation.
//!
//! This module provides:
//! - A SQL parser for the Cosmos DB SQL dialect
//! - Partition key filter extraction from WHERE clauses (to avoid Gateway query plan calls)
//! - In-memory document matching and projection (for test emulators)

pub(crate) mod ast;
pub(crate) mod common;
#[cfg(any(test, feature = "__internal_in_memory_emulator"))]
pub(crate) mod eval;
pub(crate) mod gateway_plan;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod plan;
#[cfg(any(test, feature = "__internal_in_memory_emulator"))]
mod value;

#[allow(unused_imports)]
// Used by tests, the in-memory evaluator, and the (not-yet-wired) local plan caller.
pub(crate) use parser::parse;

/// Comma-separated list of query features the local plan generator advertises
/// to the Cosmos DB Gateway via `x-ms-cosmos-supported-query-features`.
///
/// Kept in sync with the structural features the local plan generator and
/// pipeline can actually execute. Used both by the internal query-plan request
/// builder (`CosmosOperation::query_plan`) and by gateway-comparison tests so
/// the two stay in lockstep.
///
/// (#8) `MultipleAggregates` and `CompositeAggregate` are intentionally NOT
/// advertised: the in-memory evaluator only handles single-aggregate queries
/// over `COUNT|SUM|AVG|MIN|MAX` inline today and cannot execute the rewritten
/// query that the Gateway returns when those features are enabled. Re-add them
/// once the evaluator gains support, otherwise the Gateway will hand back a
/// plan the local pipeline cannot run.
pub(crate) const SUPPORTED_QUERY_FEATURES: &str =
    "NonValueAggregate,Aggregate,Distinct,MultipleOrderBy,OffsetAndLimit,OrderBy,Top,GroupBy";

/// Re-export of [`SUPPORTED_QUERY_FEATURES`] for cross-crate gateway-comparison
/// tests. Production callers must not depend on this — it shares the
/// `__internal_testing` feature gate and is not covered by SemVer.
#[cfg(any(test, feature = "__internal_testing"))]
#[doc(hidden)]
pub const __TEST_ONLY_SUPPORTED_QUERY_FEATURES: &str = SUPPORTED_QUERY_FEATURES;

#[cfg(any(test, feature = "__internal_testing"))]
pub use plan::__test_only_generate_query_plan_for_pk_paths;
