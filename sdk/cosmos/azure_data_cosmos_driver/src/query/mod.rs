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

/// Production-safe list of query features the local plan generator
/// advertises to the Cosmos DB Gateway via
/// `x-ms-cosmos-supported-query-features`.
///
/// Advertised as `"None"`: the cross-partition
/// query pipeline does not yet support any of the advanced rewrite shapes the
/// Gateway can plan (Aggregate, CompositeAggregate, CountIf, DCount, Distinct,
/// GroupBy, HybridSearch, MultipleAggregates, MultipleOrderBy,
/// NonStreamingOrderBy, NonValueAggregate, OffsetAndLimit, OrderBy, Top,
/// WeightedRankFusion); advertising any of them in production would cause the
/// Gateway to return a plan we cannot execute. Add a feature here only after
/// the local pipeline gains support for the corresponding rewrite shape.
///
/// The value must be non-empty: the Gateway V2 thin-client proxy rejects
/// QueryPlan requests where the `x-ms-cosmos-supported-query-features` header
/// (and its RNTBD `SupportedQueryFeatures` token) is missing.
///
/// Tests use [`__TEST_ONLY_SUPPORTED_QUERY_FEATURES`] (broad, matches what
/// Java/.NET advertise) so plan-shape parity against the live Gateway is
/// validated end-to-end across the full feature surface.
pub(crate) const SUPPORTED_QUERY_FEATURES: &str = "None";

/// Broad supported-features list used by cross-crate gateway-comparison
/// tests. Matches what the Java and .NET SDKs send today so the Gateway
/// returns the same plan shape across SDKs and plan-parity tests stay
/// meaningful. Production callers must not depend on this — it shares the
/// `__internal_testing` feature gate and is not covered by SemVer.
#[cfg(any(test, feature = "__internal_testing"))]
#[doc(hidden)]
pub const __TEST_ONLY_SUPPORTED_QUERY_FEATURES: &str = "Aggregate,CompositeAggregate,CountIf,DCount,Distinct,GroupBy,HybridSearch,MultipleAggregates,MultipleOrderBy,NonStreamingOrderBy,NonValueAggregate,OffsetAndLimit,OrderBy,Top,WeightedRankFusion";

#[cfg(any(test, feature = "__internal_testing"))]
pub use plan::__test_only_generate_query_plan_for_pk_paths;
