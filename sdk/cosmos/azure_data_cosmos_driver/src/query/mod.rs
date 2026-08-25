// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore fulltext vectordistance

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
/// Advertises `Distinct,MultipleOrderBy,NonStreamingOrderBy,OffsetAndLimit,OrderBy,Top`.
/// The production
/// pipeline supports streaming single- and multi-column `ORDER BY` rewrites
/// (`OrderBy,MultipleOrderBy`) and the result-window rewrite shapes
/// `OffsetAndLimit,Top` through [`driver::dataflow::SkipTake`]. Advertising
/// these lets the Gateway return the per-partition rewritten query the
/// client-side pipeline needs, including for combined `ORDER BY … OFFSET/LIMIT`
/// and `ORDER BY … TOP` queries. `NonStreamingOrderBy` enables finite-window
/// cross-partition vector ordering through the buffered merge pipeline.
///
/// Other advanced rewrite shapes (Aggregate, CompositeAggregate, CountIf,
/// DCount, GroupBy, HybridSearch, MultipleAggregates, NonValueAggregate,
/// WeightedRankFusion) remain
/// unadvertised until their corresponding pipeline stages are implemented;
/// advertising one prematurely would cause the Gateway to return a plan we
/// cannot execute.
///
/// The value must be non-empty: the Gateway V2 thin-client proxy rejects
/// QueryPlan requests where the `x-ms-cosmos-supported-query-features` header
/// (and its RNTBD `SupportedQueryFeatures` token) is missing.
///
/// Tests use [`__TEST_ONLY_SUPPORTED_QUERY_FEATURES`] (broad, matches what
/// Java/.NET advertise) so plan-shape parity against the live Gateway is
/// validated end-to-end across the full feature surface.
pub(crate) const SUPPORTED_QUERY_FEATURES: &str =
    "Distinct,MultipleOrderBy,NonStreamingOrderBy,OffsetAndLimit,OrderBy,Top";

/// Returns `true` when `query_spec_json` contains the supported pure-vector
/// ordering shape: exactly one unqualified `ORDER BY VectorDistance(...)`
/// expression with no `ASC` or `DESC` modifier.
///
/// This deliberately classifies the original SQL AST rather than relying on
/// query-plan expression strings, whose formatting is a service detail.
pub(crate) fn is_pure_vector_order_by_query_spec(query_spec_json: &[u8]) -> bool {
    #[derive(serde::Deserialize)]
    struct QuerySpec<'a> {
        #[serde(borrow)]
        query: std::borrow::Cow<'a, str>,
    }

    let Ok(spec) = serde_json::from_slice::<QuerySpec<'_>>(query_spec_json) else {
        return false;
    };
    let Ok(program) = parse(&spec.query) else {
        return false;
    };
    let Some(order_by) = &program.query.order_by else {
        return false;
    };
    let [item] = order_by.items.as_slice() else {
        return false;
    };
    if item.order != ast::SqlSortOrder::Unspecified {
        return false;
    }
    matches!(
        &item.expression,
        ast::SqlScalarExpression::FunctionCall { name, args, is_udf }
            if !is_udf
                && name.eq_ignore_ascii_case("VectorDistance")
                && (2..=3).contains(&args.len())
    ) && !query_contains_full_text_function(&program.query)
}

fn query_contains_full_text_function(query: &ast::SqlQuery) -> bool {
    use ast::{SqlCollection, SqlCollectionExpression, SqlScalarExpression, SqlSelectSpec};

    let mut queries = vec![query];
    let mut expressions = Vec::new();
    while let Some(query) = queries.pop() {
        match &query.select.spec {
            SqlSelectSpec::Star => {}
            SqlSelectSpec::List(items) => {
                expressions.extend(items.iter().map(|item| &item.expression));
            }
            SqlSelectSpec::Value(expression) => expressions.push(expression),
        }
        if let Some(where_clause) = &query.where_clause {
            expressions.push(&where_clause.expression);
        }
        if let Some(group_by) = &query.group_by {
            expressions.extend(group_by.expressions.iter());
        }
        if let Some(order_by) = &query.order_by {
            expressions.extend(order_by.items.iter().map(|item| &item.expression));
        }

        let mut collections = query
            .from
            .as_ref()
            .map(|from| vec![&from.collection])
            .unwrap_or_default();
        while let Some(collection) = collections.pop() {
            match collection {
                SqlCollectionExpression::Aliased { collection, .. }
                | SqlCollectionExpression::ArrayIterator { collection, .. } => {
                    if let SqlCollection::Subquery(subquery) = collection {
                        queries.push(subquery);
                    }
                }
                SqlCollectionExpression::Join { left, right } => {
                    collections.push(right);
                    collections.push(left);
                }
            }
        }

        while let Some(expression) = expressions.pop() {
            match expression {
                SqlScalarExpression::FunctionCall {
                    name, args, is_udf, ..
                } => {
                    if !is_udf && name.to_ascii_lowercase().starts_with("fulltext") {
                        return true;
                    }
                    expressions.extend(args.iter());
                }
                SqlScalarExpression::MemberRef { source, .. }
                | SqlScalarExpression::Unary {
                    operand: source, ..
                }
                | SqlScalarExpression::IsNull {
                    expression: source, ..
                } => expressions.push(source),
                SqlScalarExpression::MemberIndexer { source, index } => {
                    expressions.push(index);
                    expressions.push(source);
                }
                SqlScalarExpression::Exists(subquery)
                | SqlScalarExpression::Subquery(subquery)
                | SqlScalarExpression::Array(subquery) => {
                    queries.push(subquery);
                }
                SqlScalarExpression::Binary { left, right, .. }
                | SqlScalarExpression::Coalesce { left, right } => {
                    expressions.push(right);
                    expressions.push(left);
                }
                SqlScalarExpression::Between {
                    expression,
                    low,
                    high,
                    ..
                } => {
                    expressions.push(high);
                    expressions.push(low);
                    expressions.push(expression);
                }
                SqlScalarExpression::In {
                    expression, items, ..
                } => {
                    expressions.extend(items.iter());
                    expressions.push(expression);
                }
                SqlScalarExpression::Like {
                    expression,
                    pattern,
                    escape: _,
                    ..
                } => {
                    expressions.push(pattern);
                    expressions.push(expression);
                }
                SqlScalarExpression::Conditional {
                    condition,
                    if_true,
                    if_false,
                } => {
                    expressions.push(if_false);
                    expressions.push(if_true);
                    expressions.push(condition);
                }
                SqlScalarExpression::ArrayCreate(items) => expressions.extend(items.iter()),
                SqlScalarExpression::ObjectCreate(properties) => {
                    expressions.extend(properties.iter().map(|property| &property.expression));
                }
                _ => {}
            }
        }
    }
    false
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
pub use plan::__test_only_generate_query_plan_for_pk_paths;

#[cfg(test)]
mod vector_order_by_tests {
    use super::is_pure_vector_order_by_query_spec;

    fn query_spec(sql: &str) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({"query": sql, "parameters": []})).unwrap()
    }

    #[test]
    fn recognizes_pure_vector_order_by() {
        assert!(is_pure_vector_order_by_query_spec(&query_spec(
            "SELECT TOP 5 * FROM c \
             ORDER BY VectorDistance(c.embedding, @vector, false)"
        )));
        assert!(is_pure_vector_order_by_query_spec(&query_spec(
            "SELECT * FROM c ORDER BY vectordistance(c.embedding, @vector) OFFSET 1 LIMIT 2"
        )));
    }

    #[test]
    fn rejects_non_vector_or_qualified_order_by() {
        for sql in [
            "SELECT TOP 5 * FROM c ORDER BY c.rank",
            "SELECT TOP 5 * FROM c ORDER BY VectorDistance(c.embedding, @vector) ASC",
            "SELECT TOP 5 * FROM c ORDER BY VectorDistance(c.embedding, @vector), c.id",
            "SELECT TOP 5 * FROM c ORDER BY udf.VectorDistance(c.embedding, @vector)",
            "SELECT TOP 5 * FROM c ORDER BY VectorDistance(c.embedding)",
        ] {
            assert!(
                !is_pure_vector_order_by_query_spec(&query_spec(sql)),
                "unexpectedly recognized {sql}"
            );
        }
    }

    #[test]
    fn rejects_vector_ordering_combined_with_full_text() {
        for sql in [
            "SELECT TOP 5 * FROM c WHERE FullTextContains(c.text, 'term') \
             ORDER BY VectorDistance(c.embedding, @vector)",
            "SELECT TOP 5 c.id, FullTextScore(c.text, ['term']) AS textScore FROM c \
             ORDER BY VectorDistance(c.embedding, @vector)",
        ] {
            assert!(
                !is_pure_vector_order_by_query_spec(&query_spec(sql)),
                "{sql}"
            );
        }
    }
}
