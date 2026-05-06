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
/// Kept in lockstep with the supported-features list the Java and .NET SDKs
/// advertise so the Gateway returns the same query plan shape across SDKs.
/// This enables exhaustive cross-SDK plan-parity testing today via
/// [`tests/gateway_query_plan_comparison.rs`](../../tests/gateway_query_plan_comparison.rs).
///
/// **Plan generation vs. query execution.** Advertising a feature here only
/// affects what the Gateway is willing to *plan*. Whether the SDK can actually
/// *execute* the resulting plan is a separate concern — the local query
/// pipeline does not yet support the more advanced rewrite shapes (multiple
/// aggregates, composite aggregates, DCount, CountIf, non-streaming ORDER BY,
/// hybrid search, weighted rank fusion). The integration PR that wires the
/// local plan generator into the production query path is expected to gate
/// the production header on a stricter, pipeline-aware subset of this list,
/// while leaving the testing-side advertisement broad so plan-shape parity is
/// validated end-to-end against the live Gateway.
pub(crate) const SUPPORTED_QUERY_FEATURES: &str = "Aggregate,CompositeAggregate,CountIf,DCount,Distinct,GroupBy,HybridSearch,MultipleAggregates,MultipleOrderBy,NonStreamingOrderBy,NonValueAggregate,OffsetAndLimit,OrderBy,Top,WeightedRankFusion";

/// Re-export of [`SUPPORTED_QUERY_FEATURES`] for cross-crate gateway-comparison
/// tests. Production callers must not depend on this — it shares the
/// `__internal_testing` feature gate and is not covered by SemVer.
#[cfg(any(test, feature = "__internal_testing"))]
#[doc(hidden)]
pub const __TEST_ONLY_SUPPORTED_QUERY_FEATURES: &str = SUPPORTED_QUERY_FEATURES;

#[cfg(any(test, feature = "__internal_testing"))]
pub use plan::__test_only_generate_query_plan_for_pk_paths;
