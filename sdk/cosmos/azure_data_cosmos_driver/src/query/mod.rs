// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB SQL query parser, partition key extraction, and in-memory evaluation.
//!
//! This module provides:
//! - A SQL parser for the Cosmos DB SQL dialect
//! - Partition key filter extraction from WHERE clauses (to avoid Gateway query plan calls)
//! - In-memory document matching and projection (for test emulators)

mod aggregate_plan;
pub(crate) mod ast;
pub(crate) mod common;
#[cfg(any(test, feature = "__internal_in_memory_emulator"))]
pub(crate) mod emulator_plan;
#[cfg(any(test, feature = "__internal_in_memory_emulator"))]
pub(crate) mod eval;
pub(crate) mod gateway_plan;
pub(crate) mod lexer;
pub(crate) mod local_plan_adapter;
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
/// The production pipeline supports ordinary aggregate and GROUP BY partials,
/// globally deduplicated distinct counts, streaming single- and multi-column `ORDER BY` rewrites
/// (`OrderBy,MultipleOrderBy`) and the result-window rewrite shapes
/// `OffsetAndLimit,Top` through [`driver::dataflow::SkipTake`]. Advertising
/// these lets the Gateway return the per-partition rewritten query the
/// client-side pipeline needs, including for combined `ORDER BY … OFFSET/LIMIT`
/// and `ORDER BY … TOP` queries. `NonStreamingOrderBy` enables finite-window
/// plans that require the fully buffered ordered merge pipeline.
/// `HybridSearch` and `WeightedRankFusion` enable the bounded ranked full-text
/// and hybrid search pipeline.
///
/// Other advanced rewrite shapes (CompositeAggregate, CountIf) remain
/// unadvertised until their corresponding pipeline stages are implemented;
/// advertising one prematurely would cause the Gateway to return a plan we
/// cannot execute.
///
/// The value must be non-empty: the Gateway V2 thin-client proxy rejects
/// QueryPlan requests where the `x-ms-cosmos-supported-query-features` header
/// (and its RNTBD `SupportedQueryFeatures` token) is missing.
///
/// Production-equivalence tests use this list. Broader plan-shape tests use
/// [`__TEST_ONLY_SUPPORTED_QUERY_FEATURES`] to cover additional features.
#[doc(hidden)]
pub const SUPPORTED_QUERY_FEATURES: &str =
    "Aggregate,DCount,Distinct,GroupBy,HybridSearch,MultipleAggregates,MultipleOrderBy,NonStreamingOrderBy,NonValueAggregate,OffsetAndLimit,OrderBy,Top,WeightedRankFusion";

/// Whether a query requires hybrid planning even when scoped to one logical partition.
pub(crate) fn uses_ranked_search(body: Option<&[u8]>) -> bool {
    let Some(query) = body
        .and_then(|body| serde_json::from_slice::<serde_json::Value>(body).ok())
        .and_then(|spec| {
            spec.get("query")
                .and_then(|query| query.as_str())
                .map(str::to_owned)
        })
    else {
        return false;
    };
    local_plan_adapter::uses_ranked_search(&query)
}

/// Broad supported-features list used by cross-crate gateway-comparison
/// tests. Matches what the Java and .NET SDKs send today so the Gateway
/// returns the same plan shape across SDKs and plan-parity tests stay
/// meaningful. Production callers must not depend on this — it shares the
/// `__internal_testing` feature gate and is not covered by SemVer.
#[cfg(any(test, feature = "__internal_testing"))]
#[doc(hidden)]
pub const __TEST_ONLY_SUPPORTED_QUERY_FEATURES: &str = "Aggregate,CompositeAggregate,CountIf,DCount,Distinct,GroupBy,HybridSearch,MultipleAggregates,MultipleOrderBy,NonStreamingOrderBy,NonValueAggregate,OffsetAndLimit,OrderBy,Top,WeightedRankFusion";

#[cfg(any(test, feature = "__internal_testing"))]
pub use local_plan_adapter::__test_only_generate_production_query_plan;
#[cfg(any(test, feature = "__internal_testing"))]
pub use plan::__test_only_generate_query_plan_for_pk_paths;

/// Compares rewritten SQL, normalizing equivalent object projections and formatting.
///
/// # Errors
///
/// Returns an error if either query cannot be parsed.
#[cfg(any(test, feature = "__internal_testing"))]
#[doc(hidden)]
pub fn __test_only_query_rewrites_match(left: &str, right: &str) -> Result<bool, String> {
    fn normalize(source: &str) -> Result<ast::SqlProgram, String> {
        let source = source.replace("{documentdb-formattableorderbyquery-filter}", "true");
        let mut program = parse(&source).map_err(|error| error.to_string())?;
        if let ast::SqlSelectSpec::List(items) = &program.query.select.spec {
            let properties = items
                .iter()
                .enumerate()
                .map(|(index, item)| ast::SqlObjectProperty {
                    name: item.alias.clone().unwrap_or_else(|| {
                        common::infer_property_name(&item.expression, index + 1)
                    }),
                    expression: item.expression.clone(),
                })
                .collect();
            program.query.select.spec = ast::SqlSelectSpec::Value(Box::new(
                ast::SqlScalarExpression::ObjectCreate(properties),
            ));
        }
        Ok(program)
    }
    let left = normalize(left)?;
    let right = normalize(right)?;
    Ok(left == right)
}
