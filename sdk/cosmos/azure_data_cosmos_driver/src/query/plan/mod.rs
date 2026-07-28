// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore asymptotics preorder unioning worklist

//! Query plan generation: partition key extraction and full structural query analysis.
//!
//! This module produces a `QueryPlan` that mirrors the structure returned by the
//! Cosmos DB Gateway query plan REST endpoint, enabling the SDK to make routing
//! and pipeline decisions without a Gateway roundtrip.

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};

use crate::query::ast::{
    SqlBinaryOp, SqlCollectionExpression, SqlLimitSpec, SqlLiteral, SqlOffsetSpec, SqlQuery,
    SqlScalarExpression, SqlSelectClause, SqlSelectSpec, SqlSortOrder, SqlTopSpec,
};
use crate::query::common::get_root_alias;

// ─── Query Plan ──────────────────────────────────────────────────────────────

/// A client-side query plan produced by the local SQL parser.
///
/// Contains partition key targeting information and structural query info.
#[derive(SafeDebug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryPlan {
    /// Partition key filters extracted from the WHERE clause.
    pub(crate) pk_filters: PartitionKeyFilter,

    /// Structural information about the query for pipeline construction.
    pub(crate) query_info: LocalQueryInfo,
}

/// Structural information about a query as produced by the local plan generator.
///
/// split from the previously-unified `LocalQueryInfo`. This struct now contains
/// only the fields the local AST analyzer can populate — the shared structural
/// fields the SDK pipeline needs (TOP / OFFSET / LIMIT / DISTINCT / ORDER BY /
/// GROUP BY / aggregates / SELECT VALUE) plus the local-analysis booleans
/// (`has_join`, `has_subquery`, `has_where`, `has_udf`).
///
/// Gateway-only fields (`rewritten_query`, `group_by_aliases`,
/// `group_by_alias_to_aggregate_type`, `has_non_streaming_order_by`,
/// `d_count_info`) live on
/// [`crate::query::gateway_plan::GatewayQueryInfo`]. To compare a Gateway
/// response against a locally-generated plan use
/// [`crate::query::gateway_plan::GatewayQueryInfo::shared_fields_match`],
/// which compares only the structural core the two types share. There is no
/// `From<GatewayQueryInfo> for LocalQueryInfo` conversion: such a
/// conversion would have to fabricate values for the local-only booleans
/// (`has_join`, `has_subquery`, `has_where`, `has_udf`) and downstream code
/// would have no way to tell those manufactured `false`s apart from values
/// produced by local AST analysis.
#[derive(SafeDebug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LocalQueryInfo {
    /// The kind of DISTINCT, if any.
    #[serde(default)]
    pub(crate) distinct_type: DistinctType,

    /// TOP value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) top: Option<i64>,

    /// OFFSET value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<i64>,

    /// LIMIT value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) limit: Option<i64>,

    /// ORDER BY sort orders (one per ORDER BY item).
    #[serde(default)]
    pub(crate) order_by: Vec<SortOrder>,

    /// ORDER BY expressions as path strings (e.g., `["c.name", "c.age"]`).
    #[serde(default)]
    pub(crate) order_by_expressions: Vec<String>,

    /// GROUP BY expressions as path strings.
    #[serde(default)]
    pub(crate) group_by_expressions: Vec<String>,

    /// Aggregate functions used in the query.
    #[serde(default)]
    pub(crate) aggregates: Vec<AggregateKind>,

    /// Whether the SELECT clause uses `SELECT VALUE`.
    #[serde(default)]
    pub(crate) has_select_value: bool,

    /// Whether the query contains a JOIN (local analysis only).
    #[serde(default)]
    pub(crate) has_join: bool,

    /// Whether the query contains subqueries (local analysis only).
    #[serde(default)]
    pub(crate) has_subquery: bool,

    /// Whether the query contains a WHERE clause (local analysis only).
    #[serde(default)]
    pub(crate) has_where: bool,

    /// Whether the query references UDF functions (local analysis only).
    #[serde(default)]
    pub(crate) has_udf: bool,
}

/// The kind of DISTINCT operator.
#[derive(SafeDebug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub(crate) enum DistinctType {
    /// No DISTINCT.
    #[default]
    None,
    /// Ordered DISTINCT (when ORDER BY is also present).
    Ordered,
    /// Unordered DISTINCT.
    Unordered,
}

/// Sort order for ORDER BY items (mirrors the Gateway's representation).
#[derive(SafeDebug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub(crate) enum SortOrder {
    Ascending,
    Descending,
}

/// Recognized aggregate function kinds.
///
/// `ARRAY_AGG` is intentionally absent: the in-memory evaluator does not
/// implement it and `SUPPORTED_QUERY_FEATURES` does not advertise it, so the
/// planner must not pretend it is structurally an aggregate. Re-add the variant
/// only after both the evaluator and the supported-features advertisement
/// gain support.
#[derive(SafeDebug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub(crate) enum AggregateKind {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

// ─── Partition Key Filter ────────────────────────────────────────────────────

/// Partition key filter extracted from a WHERE clause.
#[derive(SafeDebug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub(crate) enum PartitionKeyFilter {
    /// Exact equality on all PK components: `pk = <value>`.
    Equality(Vec<PartitionKeyValue>),

    /// IN list on first PK component: `pk IN (v1, v2, ...)`.
    InList(Vec<Vec<PartitionKeyValue>>),

    /// PK paths were supplied but the WHERE clause did not constrain them.
    /// The query must be issued as a cross-partition request.
    Unconstrained,

    /// The WHERE clause is logically self-contradictory on the partition key
    /// (e.g., `c.pk = 'a' AND c.pk = 'b'`, or two IN lists with empty
    /// intersection). The result set is provably empty and the routing layer
    /// should short-circuit to an empty feed without issuing any I/O —
    /// otherwise this would fan out a guaranteed-empty query across every
    /// physical partition.
    Contradictory,

    /// PK extraction was not attempted because the caller did not supply any
    /// PK paths. This is distinct from [`PartitionKeyFilter::Unconstrained`]
    /// (which means "caller asked, but query has no usable filter").
    NotEvaluated,
}

/// A single partition key component value.
#[derive(SafeDebug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
#[non_exhaustive]
pub(crate) enum PartitionKeyValue {
    String(String),
    /// All numeric PK values are normalized to `f64`. Integer and floating-point
    /// SQL literals are both stored here so that PK routing comparisons follow the
    /// same canonical semantics the Cosmos backend uses for effective-partition-key
    /// (EPK) hashing — `c.pk = 1` and `c.pk = 1.0` target the same partition.
    ///
    /// **Construct via [`PartitionKeyValue::try_number`]** so that the
    /// finiteness invariant (NaN / ±∞ are not valid PK values and would
    /// silently break the JSON-canonical dedup hash key in
    /// [`normalize_pk_union`]) is enforced. The variant remains directly
    /// constructible inside this crate to keep test fixtures concise, but
    /// production code paths route through `try_number`.
    Number(f64),
    Bool(bool),
    Null,
    Undefined,
    /// A reference to a query parameter that the caller did not bind.
    ///
    /// Produced when the WHERE clause uses `@name` but `parameters` did not
    /// include a value for it. Callers that rely on the extracted PK filter for
    /// routing must either supply a value for the named parameter or treat the
    /// filter as "PK could not be resolved - issue a cross-partition request".
    UnboundParameter(String),

    /// A reference to a parameter whose bound value is not a legal partition
    /// key value (e.g., array, object, or non-finite number).
    ///
    /// Distinct from [`PartitionKeyValue::UnboundParameter`] so callers can
    /// surface a clearer diagnostic - the user *did* bind the parameter; the
    /// binding is just unusable for routing. Callers should still fall back to
    /// a cross-partition request.
    InvalidParameter {
        /// Parameter name (without the leading `@`).
        name: String,
        /// Human-readable reason the bound value cannot be used as a PK value.
        reason: String,
    },
}

impl PartitionKeyValue {
    /// Construct a [`PartitionKeyValue::Number`] enforcing the finiteness
    /// invariant. Returns `None` for `NaN`/`±∞`; callers that receive `None`
    /// should surface an `InvalidParameter` for the offending source so the
    /// diagnostic remains precise. The finiteness invariant also guarantees
    /// that the manual [`Hash`]/[`Eq`] impls below are well-defined for every
    /// `Number` value the planner ever observes.
    pub(crate) fn try_number(n: f64) -> Option<Self> {
        if n.is_finite() {
            Some(PartitionKeyValue::Number(n))
        } else {
            None
        }
    }
}

// `Eq` is sound because the only floating-point variant (`Number`) is
// constructed exclusively through `try_number`, which rejects NaN/±∞ — so
// `==` is reflexive on every value the planner ever produces. The matching
// `Hash` impl hashes the discriminant plus a per-variant payload so that
// `a == b` implies `hash(a) == hash(b)`, which is what `HashSet` requires.
impl Eq for PartitionKeyValue {}
impl std::hash::Hash for PartitionKeyValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            PartitionKeyValue::String(s) => s.hash(state),
            PartitionKeyValue::Number(n) => n.to_bits().hash(state),
            PartitionKeyValue::Bool(b) => b.hash(state),
            PartitionKeyValue::Null | PartitionKeyValue::Undefined => {}
            PartitionKeyValue::UnboundParameter(s) => s.hash(state),
            PartitionKeyValue::InvalidParameter { name, reason } => {
                name.hash(state);
                reason.hash(state);
            }
        }
    }
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Generate a complete query plan from parsed SQL and partition key paths.
///
/// `pk_paths` is a list of partition key paths (e.g., `["/pk"]` or `["/tenant", "/userId"]`).
///
/// # Examples
///
/// ```ignore
/// use azure_data_cosmos_driver::query::{parse, plan};
/// let program = parse("SELECT * FROM c WHERE c.pk = 'hello'").unwrap();
/// let qp = plan::generate_query_plan(&program.query, &["/pk"]);
/// assert!(matches!(qp.pk_filters, plan::PartitionKeyFilter::Equality(_)));
/// assert_eq!(qp.query_info.distinct_type, plan::DistinctType::None);
/// ```
pub(crate) fn generate_query_plan(
    query: &SqlQuery,
    pk_paths: &[&str],
) -> crate::error::Result<QueryPlan> {
    // Convenience wrapper for callers that do not need parameter substitution
    // for `TOP` / `OFFSET` / `LIMIT`. If the query references a parameter in
    // any of those clauses this returns an error — use
    // `generate_query_plan_with_parameters` to supply the values up front.
    generate_query_plan_with_parameters(query, pk_paths, &[])
}

/// Type alias for query parameters used during plan generation.
///
/// Each entry is a `(name, value)` pair. Names are stored *without* the leading `@`.
/// Values are arbitrary JSON values; only integer values are accepted as substitutions
/// for parameterized `TOP` / `OFFSET` / `LIMIT` clauses.
pub(crate) use crate::query::common::Params;

/// Generate a complete query plan, substituting query parameters into parameterized
/// `TOP`, `OFFSET`, and `LIMIT` clauses.
///
/// Returns an error if the query references a parameter (in `TOP`, `OFFSET`, or `LIMIT`)
/// that is not present in `parameters`, or whose value is not a non-negative integer.
///
/// The Cosmos DB Gateway rejects query-plan requests for queries with parameterized
/// `TOP` / `OFFSET` / `LIMIT` (HTTP 400). Unlike the Gateway, this function can produce
/// a valid plan when the caller supplies the parameter values up-front.
pub(crate) fn generate_query_plan_with_parameters(
    query: &SqlQuery,
    pk_paths: &[&str],
    parameters: &Params,
) -> crate::error::Result<QueryPlan> {
    let query_info = analyze_query(query, parameters)?;
    let root_alias = get_root_alias(query);

    let pk_filters = if pk_paths.is_empty() {
        PartitionKeyFilter::NotEvaluated
    } else {
        let pk_segments: Vec<Vec<&str>> = pk_paths
            .iter()
            .map(|p| p.strip_prefix('/').unwrap_or(p).split('/').collect())
            .collect();

        if let Some(where_clause) = &query.where_clause {
            extract_pk_from_expression(
                &where_clause.expression,
                &pk_segments,
                root_alias.as_deref(),
                parameters,
            )
        } else {
            PartitionKeyFilter::Unconstrained
        }
    };

    Ok(QueryPlan {
        pk_filters,
        query_info,
    })
}

/// Look up a parameter value by name and return it as a non-negative `i64`.
///
/// Used to substitute parameterized `TOP` / `OFFSET` / `LIMIT` values. Thin
/// `crate::error::Result`-flavored wrapper around the shared
/// [`crate::query::common::resolve_non_negative_integer_parameter`] helper so
/// the plan and eval pipelines validate parameters identically. Adds a
/// `TOP/OFFSET/LIMIT` clause-context tag to the error message so callers can
/// distinguish it from other parameter-resolution failures.
fn resolve_integer_parameter(name: &str, parameters: &Params) -> crate::error::Result<i64> {
    crate::query::common::resolve_non_negative_integer_parameter(parameters, name).map_err(|msg| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT)
            .with_message(format!("{msg} (TOP/OFFSET/LIMIT clause)"))
            .build()
    })
}

// ─── Query Analysis ──────────────────────────────────────────────────────────

/// Returns true if the expression is a constant (literal) that doesn't reference
/// any collection variable. Used to detect cases where DISTINCT is a no-op.
fn is_constant_expression(expr: &SqlScalarExpression) -> bool {
    match expr {
        SqlScalarExpression::Literal(_) => true,
        SqlScalarExpression::ArrayCreate(items) => items.iter().all(is_constant_expression),
        SqlScalarExpression::ObjectCreate(props) => {
            props.iter().all(|p| is_constant_expression(&p.expression))
        }
        SqlScalarExpression::Unary { operand, .. } => is_constant_expression(operand),
        SqlScalarExpression::Binary { left, right, .. } => {
            is_constant_expression(left) && is_constant_expression(right)
        }
        _ => false,
    }
}

fn analyze_query(query: &SqlQuery, parameters: &Params) -> crate::error::Result<LocalQueryInfo> {
    let mut info = LocalQueryInfo {
        has_select_value: matches!(query.select.spec, SqlSelectSpec::Value(_)),
        has_where: query.where_clause.is_some(),
        ..Default::default()
    };

    // DISTINCT — Gateway optimizes away DISTINCT when the SELECT expression is a
    // constant (literal) that doesn't reference any collection variable, because
    // a single constant value is always distinct by definition.
    if query.select.distinct {
        // Gateway only collapses DISTINCT-on-constant for the `SELECT DISTINCT VALUE <expr>`
        // form. The list form (`SELECT DISTINCT 1, 2 FROM c`) is treated as ordinary DISTINCT
        // by the Gateway because the result rows are JSON objects (with synthesized property
        // names) and are therefore not all guaranteed to be identical. We mirror that
        // asymmetry intentionally — do not extend this to `SqlSelectSpec::List` without
        // verifying behavior against the Gateway.
        let is_constant_select = match &query.select.spec {
            SqlSelectSpec::Value(expr) => is_constant_expression(expr),
            _ => false,
        };
        if is_constant_select {
            // Gateway reports distinctType: "None" for constant expressions
            info.distinct_type = DistinctType::None;
        } else if query.order_by.is_some() {
            info.distinct_type = DistinctType::Ordered;
        } else {
            info.distinct_type = DistinctType::Unordered;
        }
    }

    // TOP — substitute parameterized values; error if unresolvable.
    info.top = match &query.select.top {
        Some(SqlTopSpec::Literal(n)) => Some(*n),
        Some(SqlTopSpec::Parameter(name)) => Some(resolve_integer_parameter(name, parameters)?),
        None => None,
    };

    // OFFSET / LIMIT — same substitution rules as TOP.
    if let Some(ol) = &query.offset_limit {
        info.offset = match &ol.offset {
            SqlOffsetSpec::Literal(n) => Some(*n),
            SqlOffsetSpec::Parameter(name) => Some(resolve_integer_parameter(name, parameters)?),
        };
        info.limit = match &ol.limit {
            SqlLimitSpec::Literal(n) => Some(*n),
            SqlLimitSpec::Parameter(name) => Some(resolve_integer_parameter(name, parameters)?),
        };
    }

    // ORDER BY
    if let Some(order_by) = &query.order_by {
        for item in &order_by.items {
            let sort = match item.order {
                SqlSortOrder::Descending => SortOrder::Descending,
                _ => SortOrder::Ascending,
            };
            info.order_by.push(sort);
            info.order_by_expressions
                .push(expr_to_path_string(&item.expression)?);
        }
    }

    // GROUP BY
    if let Some(group_by) = &query.group_by {
        for expr in &group_by.expressions {
            info.group_by_expressions.push(expr_to_path_string(expr)?);
        }
    }

    // JOIN
    if let Some(from) = &query.from {
        info.has_join = has_join(&from.collection);
    }

    // Aggregates, subqueries, UDFs — scan all expressions
    visit_select_for_info(&query.select, &mut info);
    if let Some(w) = &query.where_clause {
        visit_expr_for_info(&w.expression, &mut info);
    }
    if let Some(ob) = &query.order_by {
        for item in &ob.items {
            visit_expr_for_info(&item.expression, &mut info);
        }
    }
    if let Some(gb) = &query.group_by {
        for expr in &gb.expressions {
            visit_expr_for_info(expr, &mut info);
        }
    }

    Ok(info)
}

/// Convert an expression to a dot-separated path string for the plan output.
///
/// Returns an error for non-path expressions (e.g., `c.a + c.b`, function calls).
/// The Gateway query-plan endpoint accepts such expressions and rewrites the query,
/// but the local plan generator cannot fully reproduce that rewrite — emitting a
/// debug-formatted placeholder would silently produce a JSON plan that does not
/// match the Gateway's. Callers receiving this error should fall back to fetching
/// the plan from the Gateway (#2).
///
/// errors from this helper carry the [`LocalPlanFallbackError::NEEDS_GATEWAY_FALLBACK`]
/// sentinel string in their message, so the integration layer that wires the
/// local plan generator into the SDK can distinguish a "please fall back to
/// Gateway" outcome from a generic conversion failure without parsing free-form
/// text fragments.
fn expr_to_path_string(expr: &SqlScalarExpression) -> crate::error::Result<String> {
    let mut parts = Vec::new();
    if collect_path_parts(expr, &mut parts) {
        Ok(parts.join("."))
    } else {
        Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::CLIENT_QUERY_PLAN_COMPLEX_PROJECTION_UNSUPPORTED).with_message(format!(
                "{} GROUP BY / ORDER BY expression is not a property path; local plan generation cannot reproduce the Gateway's rewrite. Fall back to the Gateway query-plan endpoint. expression: {expr:?}",
                LocalPlanFallbackError::NEEDS_GATEWAY_FALLBACK
            )).build())
    }
}

/// Sentinel marker carried in error messages that the local plan generator
/// emits when the integration layer should fall back to the Gateway query-plan
/// endpoint instead of failing the operation.
///
/// The local plan generator is intentionally not yet wired into the SDK
/// production path; once it is, the wiring layer can match on this sentinel to
/// distinguish a recoverable "plan this on the server" outcome from a hard
/// error. Kept as a constant rather than a typed error variant because the
/// fragment the error model just for an internal fallback signal.
pub(crate) struct LocalPlanFallbackError;

impl LocalPlanFallbackError {
    /// Sentinel substring callers can search for to detect a fallback request.
    /// Stable across patch releases of the driver crate.
    pub(crate) const NEEDS_GATEWAY_FALLBACK: &'static str = "[NEEDS_GATEWAY_FALLBACK]";
}

/// Returns `true` when this PK value is a parameter reference that could not be
/// resolved to a concrete literal (unbound or bound to an unusable JSON type).
/// Used by `intersect_pk_filters` to avoid producing a bogus `Contradictory`
/// when one conjunct contains an unresolved parameter and the other contains a
/// real literal.
fn is_unresolved_pk_value(v: &PartitionKeyValue) -> bool {
    matches!(
        v,
        PartitionKeyValue::UnboundParameter(_) | PartitionKeyValue::InvalidParameter { .. }
    )
}

#[allow(clippy::collapsible_match)] // clippy suggests a match guard, but that won't compile with &mut
fn collect_path_parts(expr: &SqlScalarExpression, parts: &mut Vec<String>) -> bool {
    match expr {
        SqlScalarExpression::PropertyRef(name) => {
            parts.push(name.clone());
            true
        }
        SqlScalarExpression::MemberRef { source, member } => {
            if collect_path_parts(source, parts) {
                parts.push(member.clone());
                true
            } else {
                false
            }
        }
        // Bracket access with a literal string index is semantically equivalent
        // to dotted property access (`c["foo"]` ≡ `c.foo`), and the Gateway
        // query-plan endpoint emits the dotted form in `orderByExpressions` /
        // `groupByExpressions`, so we can produce a local plan that matches.
        // Integer subscripts (`c.a[0]`) are *not* property paths — they index
        // into arrays and the Gateway emits them with the bracket syntax
        // preserved; flattening to `"c.a.0"` would silently diverge from the
        // Gateway, so those fall through to `false` and trigger the
        // `NEEDS_GATEWAY_FALLBACK` sentinel. Non-literal indices (e.g.
        // `c[@param]`) are likewise not paths.
        // Bracket access (`c["name"]`, `c['name']`, `c.scores[0]`) is *not*
        // treated as a property path here. Empirically (see the
        // `gw_local_parity_*_bracket_path*` tests in
        // `tests/gateway_query_plan_comparison.rs`), the Cosmos Gateway
        // preserves the source bracket syntax verbatim in
        // `orderByExpressions` / `groupByExpressions` (`"c[\"name\"]"`) rather
        // than flattening to `"c.name"`. Producing the dotted form locally
        // would silently diverge from the Gateway, breaking plan-shape parity
        // with other SDKs. Surface the fallback sentinel instead so the
        // integration layer defers to the Gateway query-plan endpoint.
        _ => false,
    }
}

fn has_join(coll: &SqlCollectionExpression) -> bool {
    matches!(coll, SqlCollectionExpression::Join { .. })
}

fn visit_select_for_info(select: &SqlSelectClause, info: &mut LocalQueryInfo) {
    match &select.spec {
        SqlSelectSpec::List(items) => {
            for item in items {
                visit_expr_for_info(&item.expression, info);
            }
        }
        SqlSelectSpec::Value(expr) => visit_expr_for_info(expr.as_ref(), info),
        SqlSelectSpec::Star => {}
    }
}

fn visit_expr_for_info(expr: &SqlScalarExpression, info: &mut LocalQueryInfo) {
    walk_expr_for_info(expr, info, /* no_aggregates */ false);
}

/// Walk an expression tree without recording aggregates. Used inside UDF
/// argument lists where any apparent aggregate must be ignored —
/// Cosmos disallows aggregates inside UDF args, and the Gateway never
/// reports them on `queryInfo.aggregates`. Other state (`has_subquery`,
/// `has_udf` for nested UDFs) is still recorded.
fn visit_expr_for_info_no_aggregates(expr: &SqlScalarExpression, info: &mut LocalQueryInfo) {
    walk_expr_for_info(expr, info, /* no_aggregates */ true);
}

/// Iterative worklist walk shared by both `visit_expr_for_info` and its
/// `_no_aggregates` variant. Each work-stack entry carries a
/// `no_aggregates` flag so the original semantics are preserved: descending
/// into a UDF's argument list (or being called from the no-aggregates entry
/// point) suppresses aggregate detection for every nested function call.
///
/// Iterative on purpose: `analyze_query` runs this on the WHERE clause, and
/// generated workloads commonly produce left-deep `AND`/`OR` chains with
/// thousands of conjuncts. Recursive descent would overflow the worker
/// thread's default stack long before reaching the PK extractor.
///
/// Children are pushed in reverse so the LIFO `pop` order matches a
/// left-to-right preorder traversal — important because `info.aggregates`
/// records the order in which aggregate calls appear in the source.
fn walk_expr_for_info(
    root: &SqlScalarExpression,
    info: &mut LocalQueryInfo,
    no_aggregates_root: bool,
) {
    let mut stack: Vec<(&SqlScalarExpression, bool)> = vec![(root, no_aggregates_root)];
    while let Some((expr, no_aggregates)) = stack.pop() {
        match expr {
            SqlScalarExpression::FunctionCall {
                name, args, is_udf, ..
            } => {
                if *is_udf {
                    info.has_udf = true;
                    // Cosmos disallows aggregates inside UDF arg lists; the
                    // Gateway never emits them on `queryInfo.aggregates`. Walk
                    // arguments with aggregate detection suppressed.
                    for arg in args.iter().rev() {
                        stack.push((arg, true));
                    }
                } else {
                    if !no_aggregates {
                        let upper = name.to_ascii_uppercase();
                        match upper.as_str() {
                            "COUNT" => info.aggregates.push(AggregateKind::Count),
                            "SUM" => info.aggregates.push(AggregateKind::Sum),
                            "AVG" => info.aggregates.push(AggregateKind::Avg),
                            "MIN" => info.aggregates.push(AggregateKind::Min),
                            "MAX" => info.aggregates.push(AggregateKind::Max),
                            // ARRAY_AGG is intentionally NOT advertised as a
                            // local-plan aggregate — the in-memory evaluator
                            // does not implement it, and the supported-query-
                            // features list does not include it. A query
                            // containing ARRAY_AGG falls into the generic
                            // non-aggregate path; routing/aggregation will
                            // surface the correct error from the evaluator.
                            _ => {}
                        }
                    }
                    for arg in args.iter().rev() {
                        stack.push((arg, no_aggregates));
                    }
                }
            }
            SqlScalarExpression::Exists(_)
            | SqlScalarExpression::Subquery(_)
            | SqlScalarExpression::Array(_) => {
                info.has_subquery = true;
            }
            SqlScalarExpression::Binary { left, right, .. } => {
                stack.push((right, no_aggregates));
                stack.push((left, no_aggregates));
            }
            SqlScalarExpression::Unary { operand, .. } => {
                stack.push((operand, no_aggregates));
            }
            SqlScalarExpression::Conditional {
                condition,
                if_true,
                if_false,
            } => {
                stack.push((if_false, no_aggregates));
                stack.push((if_true, no_aggregates));
                stack.push((condition, no_aggregates));
            }
            SqlScalarExpression::Coalesce { left, right } => {
                stack.push((right, no_aggregates));
                stack.push((left, no_aggregates));
            }
            SqlScalarExpression::In {
                expression, items, ..
            } => {
                for item in items.iter().rev() {
                    stack.push((item, no_aggregates));
                }
                stack.push((expression, no_aggregates));
            }
            SqlScalarExpression::Between {
                expression,
                low,
                high,
                ..
            } => {
                stack.push((high, no_aggregates));
                stack.push((low, no_aggregates));
                stack.push((expression, no_aggregates));
            }
            SqlScalarExpression::Like {
                expression,
                pattern,
                ..
            } => {
                stack.push((pattern, no_aggregates));
                stack.push((expression, no_aggregates));
            }
            SqlScalarExpression::ArrayCreate(items) => {
                for item in items.iter().rev() {
                    stack.push((item, no_aggregates));
                }
            }
            SqlScalarExpression::ObjectCreate(props) => {
                for prop in props.iter().rev() {
                    stack.push((&prop.expression, no_aggregates));
                }
            }
            _ => {}
        }
    }
}

// ─── PK Extraction (unchanged logic) ────────────────────────────────────────

fn extract_pk_from_expression(
    expr: &SqlScalarExpression,
    pk_segments: &[Vec<&str>],
    root_alias: Option<&str>,
    parameters: &Params,
) -> PartitionKeyFilter {
    if pk_segments.len() == 1 {
        return extract_single_pk(expr, &pk_segments[0], root_alias, parameters);
    }
    extract_hierarchical_pk(expr, pk_segments, root_alias, parameters)
}

fn extract_single_pk(
    expr: &SqlScalarExpression,
    pk_path: &[&str],
    root_alias: Option<&str>,
    parameters: &Params,
) -> PartitionKeyFilter {
    match expr {
        SqlScalarExpression::Binary {
            op: SqlBinaryOp::Equal,
            left,
            right,
        } => {
            if is_pk_reference(left, pk_path, root_alias) {
                if let Some(val) = extract_literal_value(right, parameters) {
                    return PartitionKeyFilter::Equality(vec![val]);
                }
            }
            if is_pk_reference(right, pk_path, root_alias) {
                if let Some(val) = extract_literal_value(left, parameters) {
                    return PartitionKeyFilter::Equality(vec![val]);
                }
            }
            PartitionKeyFilter::Unconstrained
        }
        SqlScalarExpression::In {
            expression,
            items,
            not: false,
        } => {
            if is_pk_reference(expression, pk_path, root_alias) {
                let values: Vec<Vec<PartitionKeyValue>> = items
                    .iter()
                    .filter_map(|item| extract_literal_value(item, parameters).map(|v| vec![v]))
                    .collect();
                if values.len() == items.len() {
                    return PartitionKeyFilter::InList(values);
                }
            }
            PartitionKeyFilter::Unconstrained
        }
        // Flatten left-deep AND/OR chains iteratively to avoid blowing the
        // worker thread's stack on generated queries with 1000s of conjuncts/
        // disjuncts (the common case for tooling-generated SQL). Each leaf is
        // analyzed independently and the per-leaf filters are folded with
        // `intersect` (AND) or `union` (OR) — semantics identical to the
        // previous recursive descent.
        SqlScalarExpression::Binary {
            op: SqlBinaryOp::And,
            ..
        } => {
            let mut conjuncts = Vec::new();
            flatten_and(expr, &mut conjuncts);
            conjuncts
                .into_iter()
                .map(|c| extract_single_pk(c, pk_path, root_alias, parameters))
                .reduce(intersect_pk_filters)
                .unwrap_or(PartitionKeyFilter::Unconstrained)
        }
        SqlScalarExpression::Binary {
            op: SqlBinaryOp::Or,
            ..
        } => {
            let mut disjuncts = Vec::new();
            flatten_or(expr, &mut disjuncts);
            disjuncts
                .into_iter()
                .map(|d| extract_single_pk(d, pk_path, root_alias, parameters))
                .reduce(union_pk_filters)
                .unwrap_or(PartitionKeyFilter::Unconstrained)
        }
        _ => PartitionKeyFilter::Unconstrained,
    }
}

fn union_pk_filters(a: PartitionKeyFilter, b: PartitionKeyFilter) -> PartitionKeyFilter {
    match (a, b) {
        (PartitionKeyFilter::Equality(a), PartitionKeyFilter::Equality(b)) => {
            normalize_pk_union(vec![a, b])
        }
        (PartitionKeyFilter::Equality(a), PartitionKeyFilter::InList(mut list))
        | (PartitionKeyFilter::InList(mut list), PartitionKeyFilter::Equality(a)) => {
            list.push(a);
            normalize_pk_union(list)
        }
        (PartitionKeyFilter::InList(mut a), PartitionKeyFilter::InList(b)) => {
            a.extend(b);
            normalize_pk_union(a)
        }
        // `Contradictory ∪ X = X`. The contradictory side contributes no
        // values to the union; preserving the other side avoids forcing a
        // cross-partition fan-out for queries like
        // `(c.pk='a' AND c.pk='b') OR c.pk='c'`.
        (PartitionKeyFilter::Contradictory, other) | (other, PartitionKeyFilter::Contradictory) => {
            other
        }
        _ => PartitionKeyFilter::Unconstrained,
    }
}

fn normalize_pk_union(values: Vec<Vec<PartitionKeyValue>>) -> PartitionKeyFilter {
    // Dedup directly through the `Hash + Eq` impls on `PartitionKeyValue`.
    // The previous `Vec::contains` lookup was O(n^2) for long IN-lists (e.g.
    // `c.pk IN (...1000 values...) OR ...`); the prior fix routed through
    // `serde_json::to_string` per entry, which kept the asymptotics linear
    // but allocated a JSON string per value. Hashing the value tuples
    // directly avoids both the quadratic blowup and the per-value allocation.
    let mut seen: std::collections::HashSet<Vec<PartitionKeyValue>> =
        std::collections::HashSet::with_capacity(values.len());
    let mut deduped: Vec<Vec<PartitionKeyValue>> = Vec::with_capacity(values.len());
    for value in values {
        if !seen.contains(&value) {
            seen.insert(value.clone());
            deduped.push(value);
        }
    }

    match deduped.len() {
        0 => PartitionKeyFilter::Unconstrained,
        1 => PartitionKeyFilter::Equality(deduped.into_iter().next().unwrap()),
        _ => PartitionKeyFilter::InList(deduped),
    }
}

/// Intersect two PK filters from the two sides of an AND expression.
///
/// - `None AND X` → `X` (no constraint on one side, keep the other)
/// - `Equality(a) AND Equality(b)` → `Equality(a)` if a == b, else `None` (contradiction)
/// - `Equality(a) AND InList(list)` → `Equality(a)` if a is in list, else `None`
/// - `InList(a) AND InList(b)` → `InList(intersection)`, or `None` if empty
fn intersect_pk_filters(a: PartitionKeyFilter, b: PartitionKeyFilter) -> PartitionKeyFilter {
    match (a, b) {
        // One side has no PK constraint — the other side's constraint stands.
        (PartitionKeyFilter::Unconstrained, other) | (other, PartitionKeyFilter::Unconstrained) => {
            other
        }

        // Contradiction is absorbing — `Contradictory AND anything` stays
        // contradictory because no value can satisfy both sides.
        (PartitionKeyFilter::Contradictory, _) | (_, PartitionKeyFilter::Contradictory) => {
            PartitionKeyFilter::Contradictory
        }

        // Both sides have equality — they must agree, otherwise the
        // conjunction is provably empty.
        //
        // an `UnboundParameter` / `InvalidParameter` value is not a real
        // PK literal — the `==` check between e.g. `String("a")` and
        // `UnboundParameter("x")` would always be `false` and produce a
        // bogus `Contradictory`. Defer to the side that has a usable
        // literal so the routing layer can still narrow the request.
        (PartitionKeyFilter::Equality(a), PartitionKeyFilter::Equality(b)) => {
            let a_unresolved = a.iter().any(is_unresolved_pk_value);
            let b_unresolved = b.iter().any(is_unresolved_pk_value);
            match (a_unresolved, b_unresolved) {
                (true, true) => PartitionKeyFilter::Unconstrained,
                (true, false) => PartitionKeyFilter::Equality(b),
                (false, true) => PartitionKeyFilter::Equality(a),
                (false, false) => {
                    if a == b {
                        PartitionKeyFilter::Equality(a)
                    } else {
                        PartitionKeyFilter::Contradictory
                    }
                }
            }
        }

        // Equality AND InList — narrow the IN list to just the equality value if present.
        (PartitionKeyFilter::Equality(eq), PartitionKeyFilter::InList(list))
        | (PartitionKeyFilter::InList(list), PartitionKeyFilter::Equality(eq)) => {
            if eq.iter().any(is_unresolved_pk_value) {
                // the equality side carries an unbound/invalid parameter;
                // it cannot prune the IN list. Keep the IN list as-is.
                normalize_pk_union(list)
            } else if list.contains(&eq) {
                PartitionKeyFilter::Equality(eq)
            } else {
                PartitionKeyFilter::Contradictory
            }
        }

        // InList AND InList — compute intersection.
        (PartitionKeyFilter::InList(a), PartitionKeyFilter::InList(b)) => {
            let intersection: Vec<Vec<PartitionKeyValue>> =
                a.into_iter().filter(|item| b.contains(item)).collect();
            match intersection.len() {
                0 => PartitionKeyFilter::Contradictory,
                1 => PartitionKeyFilter::Equality(intersection.into_iter().next().unwrap()),
                _ => PartitionKeyFilter::InList(intersection),
            }
        }
        // `NotEvaluated` is only ever set at the top level (when no PK paths were
        // supplied) and is never produced by the recursive extractors. Coerce to
        // `Unconstrained` defensively in case the variant ever leaks here so the
        // intersection logic can't silently mis-route a query.
        (PartitionKeyFilter::NotEvaluated, other) | (other, PartitionKeyFilter::NotEvaluated) => {
            other
        }
    }
}

fn extract_hierarchical_pk(
    expr: &SqlScalarExpression,
    pk_segments: &[Vec<&str>],
    root_alias: Option<&str>,
    parameters: &Params,
) -> PartitionKeyFilter {
    // handle top-level OR by extracting HPK from each disjunct and
    // unioning the results. `(c.tenant='a' AND c.userId='u1') OR
    // (c.tenant='b' AND c.userId='u2')` becomes an `InList` of full HPK
    // tuples instead of falling back to a cross-partition fan-out.
    // Flatten top-level OR chains iteratively. `(A) OR (B) OR (C) ...`
    // would otherwise recurse one frame per disjunct and blow a default
    // worker thread stack on adversarial / generated input. Each disjunct
    // is analyzed independently and the per-disjunct filters are folded
    // with `union_pk_filters` — same semantics as the prior recursive
    // descent.
    if let SqlScalarExpression::Binary {
        op: SqlBinaryOp::Or,
        ..
    } = expr
    {
        let mut disjuncts = Vec::new();
        flatten_or(expr, &mut disjuncts);
        return disjuncts
            .into_iter()
            .map(|d| extract_hierarchical_pk(d, pk_segments, root_alias, parameters))
            .reduce(union_pk_filters)
            .unwrap_or(PartitionKeyFilter::Unconstrained);
    }
    let mut conjuncts = Vec::new();
    flatten_and(expr, &mut conjuncts);

    // per component, accept either `Equal` or a positive `IN (...)` list,
    // then cartesian-product across components. The Gateway recognizes
    // `WHERE c.tenant IN ('a','b') AND c.userId='u1'` for HPK
    // `(/tenant,/userId)` and routes to the two specific tuples; previously
    // the local plan generator dropped to `Unconstrained` and forced a
    // cross-partition fan-out.
    //
    // The cartesian product is bounded by `MAX_HPK_TUPLES` to keep an
    // adversarial query (`IN (...1000 vals) AND IN (...1000 vals)`) from
    // generating a million-tuple `InList`. When the cap is exceeded we fall
    // back to `Unconstrained` rather than emitting an enormous filter.
    const MAX_HPK_TUPLES: usize = 1024;

    // Per-component accepted values. A leading constrained prefix is useful for
    // MultiHash routing (`/tenant` on `(/tenant,/user,/session)`), so stop at
    // the first unconstrained component after at least one constrained one.
    // A gap later in the hierarchy (`/tenant` missing but `/user` present, or
    // `/tenant` + `/session` without `/user`) is not a routable HPK prefix.
    let mut per_component: Vec<Vec<PartitionKeyValue>> = Vec::with_capacity(pk_segments.len());
    for pk_path in pk_segments {
        let mut equal_value: Option<PartitionKeyValue> = None;
        let mut in_values: Option<Vec<PartitionKeyValue>> = None;
        for conjunct in &conjuncts {
            match conjunct {
                SqlScalarExpression::Binary {
                    op: SqlBinaryOp::Equal,
                    left,
                    right,
                } => {
                    let val = if is_pk_reference(left, pk_path, root_alias) {
                        extract_literal_value(right, parameters)
                    } else if is_pk_reference(right, pk_path, root_alias) {
                        extract_literal_value(left, parameters)
                    } else {
                        None
                    };
                    if let Some(v) = val {
                        match &equal_value {
                            None => equal_value = Some(v),
                            Some(existing) if *existing == v => {} // redundant
                            Some(_) => return PartitionKeyFilter::Contradictory,
                        }
                    }
                }
                // positive IN over the component's path.
                SqlScalarExpression::In {
                    expression,
                    items,
                    not: false,
                } if is_pk_reference(expression, pk_path, root_alias) => {
                    let mut vs: Vec<PartitionKeyValue> = Vec::with_capacity(items.len());
                    let mut all_literal = true;
                    for item in items {
                        match extract_literal_value(item, parameters) {
                            Some(v) => vs.push(v),
                            None => {
                                all_literal = false;
                                break;
                            }
                        }
                    }
                    if !all_literal {
                        continue;
                    }
                    // Multiple IN lists for the same component narrow rather
                    // than union (AND semantics).
                    in_values = Some(match in_values {
                        None => vs,
                        Some(existing) => existing.into_iter().filter(|v| vs.contains(v)).collect(),
                    });
                    if matches!(in_values.as_ref(), Some(v) if v.is_empty()) {
                        return PartitionKeyFilter::Contradictory;
                    }
                }
                _ => {}
            }
        }

        let component_values: Vec<PartitionKeyValue> = match (equal_value, in_values) {
            (Some(eq), Some(list)) => {
                if list.contains(&eq) {
                    vec![eq]
                } else {
                    return PartitionKeyFilter::Contradictory;
                }
            }
            (Some(eq), None) => vec![eq],
            (None, Some(list)) => list,
            (None, None) => {
                if per_component.is_empty() {
                    return PartitionKeyFilter::Unconstrained;
                }
                break;
            }
        };
        per_component.push(component_values);
    }

    // Cartesian product across components.
    let total: usize = per_component.iter().map(|v| v.len()).product();
    if total == 0 {
        return PartitionKeyFilter::Contradictory;
    }
    if total > MAX_HPK_TUPLES {
        // Avoid emitting a pathological InList; defer to cross-partition.
        return PartitionKeyFilter::Unconstrained;
    }
    let mut tuples: Vec<Vec<PartitionKeyValue>> = vec![Vec::with_capacity(per_component.len())];
    for component in &per_component {
        let mut next: Vec<Vec<PartitionKeyValue>> =
            Vec::with_capacity(tuples.len() * component.len());
        for prefix in &tuples {
            for v in component {
                let mut t = prefix.clone();
                t.push(v.clone());
                next.push(t);
            }
        }
        tuples = next;
    }
    if tuples.len() == 1 {
        PartitionKeyFilter::Equality(tuples.into_iter().next().unwrap())
    } else {
        PartitionKeyFilter::InList(tuples)
    }
}

fn flatten_and<'a>(expr: &'a SqlScalarExpression, out: &mut Vec<&'a SqlScalarExpression>) {
    flatten_chain(expr, SqlBinaryOp::And, out);
}

fn flatten_or<'a>(expr: &'a SqlScalarExpression, out: &mut Vec<&'a SqlScalarExpression>) {
    flatten_chain(expr, SqlBinaryOp::Or, out);
}

/// Iteratively flatten a left-deep `Binary { op, .. }` chain into its leaf
/// operands, preserving original left-to-right order. Iterative on purpose so
/// generated queries with thousands of conjuncts/disjuncts (real-world
/// workloads regularly exceed 1k AND clauses) cannot overflow the worker
/// thread's stack the way a naive recursive descent would.
fn flatten_chain<'a>(
    root: &'a SqlScalarExpression,
    op: SqlBinaryOp,
    out: &mut Vec<&'a SqlScalarExpression>,
) {
    // LIFO worklist. Push children right-then-left so the pop order matches
    // a left-to-right in-order traversal of the original tree.
    let mut stack: Vec<&'a SqlScalarExpression> = vec![root];
    while let Some(node) = stack.pop() {
        match node {
            SqlScalarExpression::Binary {
                op: node_op,
                left,
                right,
            } if *node_op == op => {
                stack.push(right);
                stack.push(left);
            }
            other => out.push(other),
        }
    }
}

fn is_pk_reference(expr: &SqlScalarExpression, pk_path: &[&str], root_alias: Option<&str>) -> bool {
    let mut resolved_path = Vec::new();
    if !resolve_property_path(expr, &mut resolved_path) {
        return false;
    }
    if let Some(alias) = root_alias {
        if resolved_path.first().map(String::as_str) == Some(alias) {
            return resolved_path[1..]
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                == pk_path;
        }
    }
    resolved_path.iter().map(String::as_str).collect::<Vec<_>>() == pk_path
}

#[allow(clippy::collapsible_match)] // clippy suggests a match guard, but that won't compile with &mut
fn resolve_property_path(expr: &SqlScalarExpression, path: &mut Vec<String>) -> bool {
    match expr {
        SqlScalarExpression::PropertyRef(name) => {
            path.push(name.clone());
            true
        }
        SqlScalarExpression::MemberRef { source, member } => {
            if resolve_property_path(source, path) {
                path.push(member.clone());
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn extract_literal_value(
    expr: &SqlScalarExpression,
    parameters: &Params,
) -> Option<PartitionKeyValue> {
    match expr {
        SqlScalarExpression::Literal(lit) => match lit {
            SqlLiteral::String(s) => Some(PartitionKeyValue::String(s.clone())),
            // Both numeric literal forms canonicalize to `Number(f64)` to mirror the
            // backend's EPK-hash equivalence between `1` and `1.0` (#3).
            //
            // Invariant (#13/F17): every `PartitionKeyValue::Number(f)` flowing
            // through `normalize_pk_union`'s JSON-canonical dedup must be
            // finite — NaN/±∞ would silently break dedup. The parser cannot
            // emit NaN (it has no NaN literal) and `serde_json` rejects
            // non-finite numbers, so the invariant holds today; the
            // `PartitionKeyValue::try_number` constructor enforces it at
            // runtime.
            SqlLiteral::Number(n) => PartitionKeyValue::try_number(*n),
            SqlLiteral::Integer(n) => PartitionKeyValue::try_number(*n as f64),
            SqlLiteral::Boolean(b) => Some(PartitionKeyValue::Bool(*b)),
            SqlLiteral::Null => Some(PartitionKeyValue::Null),
            SqlLiteral::Undefined => Some(PartitionKeyValue::Undefined),
        },
        SqlScalarExpression::ParameterRef(name) => {
            // #14: substitute the supplied parameter value if present; otherwise
            // leave the placeholder so the caller can decide whether to fall back to
            // a cross-partition request.
            Some(resolve_pk_parameter(name, parameters))
        }
        _ => None,
    }
}

/// Look up `name` in `parameters` and convert the JSON value to a partition key
/// value, or fall back to [`PartitionKeyValue::Parameter`] if the caller did not
/// supply a value (an unresolved parameter — caller may need to issue a
/// cross-partition request).
fn resolve_pk_parameter(name: &str, parameters: &Params) -> PartitionKeyValue {
    let needle = name.trim_start_matches('@');
    let entry = parameters
        .iter()
        .find(|(n, _)| n.trim_start_matches('@') == needle);
    let value = match entry {
        Some((_, v)) => v,
        None => return PartitionKeyValue::UnboundParameter(needle.to_string()),
    };
    match value {
        serde_json::Value::String(s) => PartitionKeyValue::String(s.clone()),
        serde_json::Value::Number(n) => {
            // Always canonicalize to f64 (#3). `as_f64` returns `None` only for
            // non-finite values that serde_json refuses to round-trip; surface
            // those as InvalidParameter so the diagnostic is precise. Route via
            // `try_number` so any future relaxation of `serde_json`'s
            // round-trip rule still preserves the finiteness invariant.
            n.as_f64()
                .and_then(PartitionKeyValue::try_number)
                .unwrap_or_else(|| PartitionKeyValue::InvalidParameter {
                    name: needle.to_string(),
                    reason: format!("number value `{n}` is not a finite f64"),
                })
        }
        serde_json::Value::Bool(b) => PartitionKeyValue::Bool(*b),
        serde_json::Value::Null => PartitionKeyValue::Null,
        // Arrays / objects are not valid PK values.
        serde_json::Value::Array(_) => PartitionKeyValue::InvalidParameter {
            name: needle.to_string(),
            reason: "array values cannot be used as a partition key".to_string(),
        },
        serde_json::Value::Object(_) => PartitionKeyValue::InvalidParameter {
            name: needle.to_string(),
            reason: "object values cannot be used as a partition key".to_string(),
        },
    }
}
/// Generate a query plan as a JSON value from SQL text, partition key paths, and
/// query parameters.
///
/// Substitutes parameter values into parameterized `TOP` / `OFFSET` / `LIMIT` clauses.
/// Returns an error if the query references a parameter in one of those clauses and
/// no matching integer value is supplied. Pass an empty slice for queries that do not
/// use parameters in those clauses.
///
/// **This function is intentionally not part of the supported public API.** It is
/// gated on the `__internal_testing` feature flag and exists solely so that
/// cross-crate gateway-comparison tests can exercise the local plan generator
/// without taking a dependency on internal types. Production callers must not use it.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "__internal_testing")]
/// # fn main() {
/// use azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths;
///
/// let plan = __test_only_generate_query_plan_for_pk_paths(
///     "SELECT * FROM c WHERE c.pk = 'hello'",
///     &["/pk"],
///     &[],
/// )
/// .unwrap();
/// assert_eq!(plan["queryInfo"]["hasWhere"], serde_json::json!(true));
/// # }
/// # #[cfg(not(feature = "__internal_testing"))]
/// # fn main() {}
/// ```
#[cfg(any(test, feature = "__internal_testing"))]
#[doc(hidden)]
pub fn __test_only_generate_query_plan_for_pk_paths(
    sql: &str,
    pk_paths: &[&str],
    parameters: &[(String, serde_json::Value)],
) -> crate::error::Result<serde_json::Value> {
    let program = crate::query::parse(sql).map_err(|e| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message("failed to parse query")
            .with_source(e)
            .build()
    })?;

    let raw_plan = generate_query_plan_with_parameters(&program.query, pk_paths, parameters)?;

    serde_json::to_value(&raw_plan).map_err(|e| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message("failed to serialize query plan")
            .with_source(e)
            .build()
    })
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::parse;

    fn plan(sql: &str) -> QueryPlan {
        let p = parse(sql).unwrap();
        generate_query_plan(&p.query, &["/pk"]).unwrap()
    }

    // Include the exhaustive comparison tests from the external file.
    // The `#[path]` attribute makes the indirection explicit; without it Rust
    // would resolve `mod query_plan_comparison;` (because it lives inside the
    // inline `mod tests`) to `plan/tests/query_plan_comparison.rs` via the
    // implicit `tests` directory — a non-obvious convention. (#11)
    #[path = "query_plan_comparison.rs"]
    mod query_plan_comparison;

    // ── PK extraction ────────────────────────────────────────────────────

    #[test]
    fn pk_equality() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'hello'").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("hello".into())])
        );
    }

    #[test]
    fn pk_with_and() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'x' AND c.age > 21").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())])
        );
    }

    #[test]
    fn pk_in_list() {
        match plan("SELECT * FROM c WHERE c.pk IN ('a', 'b')").pk_filters {
            PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 2),
            other => panic!("expected InList, got {other:?}"),
        }
    }

    /// `(c.pk='a' AND c.pk='b') OR c.pk='c'` \u2014 the contradictory disjunct
    /// must not absorb the surviving equality.
    #[test]
    fn pk_or_with_contradictory_disjunct_preserves_other_side() {
        let qp = plan("SELECT * FROM c WHERE (c.pk = 'a' AND c.pk = 'b') OR c.pk = 'c'");
        assert_eq!(
            qp.pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("c".into())])
        );
    }

    /// `c.pk = 'a' AND c.pk = @unbound` \u2014 the unbound parameter must not
    /// turn the conjunction into `Contradictory`. The literal side wins.
    #[test]
    fn pk_and_with_unbound_parameter_keeps_literal_side() {
        let p = parse("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = @unbound").unwrap();
        let qp = generate_query_plan_with_parameters(&p.query, &["/pk"], &[]).unwrap();
        assert_eq!(
            qp.pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    /// `PartitionKeyValue::try_number` enforces the finiteness invariant.
    #[test]
    fn try_number_rejects_non_finite() {
        assert!(PartitionKeyValue::try_number(f64::NAN).is_none());
        assert!(PartitionKeyValue::try_number(f64::INFINITY).is_none());
        assert!(PartitionKeyValue::try_number(f64::NEG_INFINITY).is_none());
        assert!(PartitionKeyValue::try_number(0.0).is_some());
        assert!(PartitionKeyValue::try_number(1.5).is_some());
    }

    /// aggregates inside UDF arg lists must not be reflected in
    /// `info.aggregates`.
    #[test]
    fn aggregate_inside_udf_arg_not_advertised() {
        let p = parse("SELECT udf.foo(COUNT(c.x)) FROM c").unwrap();
        let qp = generate_query_plan(&p.query, &["/pk"]).unwrap();
        assert!(qp.query_info.has_udf);
        assert!(
            qp.query_info.aggregates.is_empty(),
            "aggregates inside UDF args must be skipped; got {:?}",
            qp.query_info.aggregates
        );
    }

    /// `c.pk = 1` and `c.pk = 1.0` must hash to the same effective partition
    /// key, so the locally-extracted PK filter must canonicalize both literal
    /// forms to the same `Number(f64)` representation. Both the pkFilters and
    /// the structural queryInfo must be byte-identical between the two forms.
    #[test]
    fn numeric_pk_canonicalization_int_and_float_match() {
        let int_form = generate_query_plan(
            &parse("SELECT * FROM c WHERE c.pk = 1").unwrap().query,
            &["/pk"],
        )
        .unwrap();
        let float_form = generate_query_plan(
            &parse("SELECT * FROM c WHERE c.pk = 1.0").unwrap().query,
            &["/pk"],
        )
        .unwrap();
        assert_eq!(int_form.pk_filters, float_form.pk_filters);
        assert_eq!(int_form.query_info, float_form.query_info);
    }

    /// Bracket access (`c["name"]`, `c['name']`, `c.scores[0]`) must surface
    /// the NEEDS_GATEWAY_FALLBACK sentinel rather than producing a dotted
    /// path. Gateway empirically preserves the source bracket syntax verbatim
    /// in `orderByExpressions` / `groupByExpressions` (see the
    /// `gw_local_parity_*_bracket_path*` tests in
    /// `tests/gateway_query_plan_comparison.rs`); flattening to a dotted path
    /// locally would silently diverge from the Gateway response.
    #[test]
    fn bracket_paths_fall_back_to_gateway() {
        for sql in [
            "SELECT * FROM c ORDER BY c['name'] ASC",
            "SELECT * FROM c ORDER BY c[\"name\"] ASC",
            "SELECT * FROM c ORDER BY c.scores[0] ASC",
        ] {
            let p = parse(sql).unwrap();
            let err = generate_query_plan(&p.query, &["/pk"])
                .expect_err(&format!("bracket path must surface fallback: {sql}"));
            assert!(
                format!("{err}").contains(LocalPlanFallbackError::NEEDS_GATEWAY_FALLBACK),
                "fallback sentinel missing for {sql}: {err}"
            );
        }
    }
    /// Non-path GROUP BY expressions (`GROUP BY c.x & 1`) must surface a
    /// fail-fast error rather than silently emitting a non-Gateway-comparable
    /// plan. The Gateway accepts and rewrites such queries; the local plan
    /// generator cannot reproduce that rewrite, so the integration layer must
    /// fall back to the Gateway query-plan endpoint when this error fires.
    #[test]
    fn non_path_group_by_errors() {
        let p = parse("SELECT c.x & 1 AS parity, COUNT(1) FROM c GROUP BY c.x & 1").unwrap();
        let err = generate_query_plan(&p.query, &["/pk"]).expect_err(
            "non-path GROUP BY must surface an error so callers can fall back to Gateway",
        );
        let msg = format!("{err}");
        assert!(
            msg.contains("GROUP BY / ORDER BY"),
            "unexpected error message: {msg}"
        );
        assert!(
            msg.contains(LocalPlanFallbackError::NEEDS_GATEWAY_FALLBACK),
            "error must carry the fallback sentinel; got: {msg}"
        );
    }

    #[test]
    fn no_pk_filter() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.age > 21").pk_filters,
            PartitionKeyFilter::Unconstrained
        );
    }

    #[test]
    fn no_where_clause() {
        assert_eq!(
            plan("SELECT * FROM c").pk_filters,
            PartitionKeyFilter::Unconstrained
        );
    }

    // ── LocalQueryInfo: DISTINCT ──────────────────────────────────────────────

    #[test]
    fn distinct_unordered() {
        let qp = plan("SELECT DISTINCT c.name FROM c");
        assert_eq!(qp.query_info.distinct_type, DistinctType::Unordered);
    }

    #[test]
    fn distinct_ordered() {
        let qp = plan("SELECT DISTINCT c.name FROM c ORDER BY c.name");
        assert_eq!(qp.query_info.distinct_type, DistinctType::Ordered);
    }

    #[test]
    fn no_distinct() {
        let qp = plan("SELECT c.name FROM c");
        assert_eq!(qp.query_info.distinct_type, DistinctType::None);
    }

    // ── LocalQueryInfo: TOP / OFFSET / LIMIT ──────────────────────────────────

    #[test]
    fn top_value() {
        assert_eq!(plan("SELECT TOP 10 * FROM c").query_info.top, Some(10));
    }

    #[test]
    fn offset_limit() {
        let qp = plan("SELECT * FROM c OFFSET 5 LIMIT 20");
        assert_eq!(qp.query_info.offset, Some(5));
        assert_eq!(qp.query_info.limit, Some(20));
    }

    // ── LocalQueryInfo: ORDER BY ──────────────────────────────────────────────

    #[test]
    fn order_by_single_asc() {
        let qp = plan("SELECT * FROM c ORDER BY c.name ASC");
        assert_eq!(qp.query_info.order_by, vec![SortOrder::Ascending]);
        assert_eq!(qp.query_info.order_by_expressions, vec!["c.name"]);
    }

    #[test]
    fn order_by_single_desc() {
        let qp = plan("SELECT * FROM c ORDER BY c.name DESC");
        assert_eq!(qp.query_info.order_by, vec![SortOrder::Descending]);
    }

    #[test]
    fn order_by_multiple() {
        let qp = plan("SELECT * FROM c ORDER BY c.name ASC, c.age DESC");
        assert_eq!(
            qp.query_info.order_by,
            vec![SortOrder::Ascending, SortOrder::Descending]
        );
        assert_eq!(qp.query_info.order_by_expressions, vec!["c.name", "c.age"]);
    }

    // ── LocalQueryInfo: GROUP BY ──────────────────────────────────────────────

    #[test]
    fn group_by_single() {
        let qp = plan("SELECT c.city, COUNT(1) FROM c GROUP BY c.city");
        assert_eq!(qp.query_info.group_by_expressions, vec!["c.city"]);
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Count));
    }

    #[test]
    fn group_by_multiple() {
        let qp = plan("SELECT c.city, c.state, COUNT(1) FROM c GROUP BY c.city, c.state");
        assert_eq!(
            qp.query_info.group_by_expressions,
            vec!["c.city", "c.state"]
        );
    }

    // ── LocalQueryInfo: Aggregates ────────────────────────────────────────────

    #[test]
    fn aggregate_count() {
        let qp = plan("SELECT COUNT(1) FROM c");
        assert_eq!(qp.query_info.aggregates, vec![AggregateKind::Count]);
    }

    #[test]
    fn aggregate_sum() {
        let qp = plan("SELECT SUM(c.price) FROM c");
        assert_eq!(qp.query_info.aggregates, vec![AggregateKind::Sum]);
    }

    #[test]
    fn aggregate_avg() {
        let qp = plan("SELECT AVG(c.score) FROM c");
        assert_eq!(qp.query_info.aggregates, vec![AggregateKind::Avg]);
    }

    #[test]
    fn aggregate_min_max() {
        let qp = plan("SELECT MIN(c.age), MAX(c.age) FROM c");
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Min));
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Max));
    }

    #[test]
    fn multiple_aggregates() {
        let qp = plan("SELECT COUNT(1), SUM(c.price), AVG(c.score) FROM c");
        assert_eq!(qp.query_info.aggregates.len(), 3);
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Count));
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Sum));
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Avg));
    }

    #[test]
    fn no_aggregates() {
        let qp = plan("SELECT * FROM c");
        assert!(qp.query_info.aggregates.is_empty());
    }

    // ── LocalQueryInfo: SELECT VALUE ──────────────────────────────────────────

    #[test]
    fn select_value_detected() {
        assert!(
            plan("SELECT VALUE c.name FROM c")
                .query_info
                .has_select_value
        );
    }

    #[test]
    fn select_star_not_value() {
        assert!(!plan("SELECT * FROM c").query_info.has_select_value);
    }

    // ── LocalQueryInfo: JOIN ──────────────────────────────────────────────────

    #[test]
    fn join_detected() {
        assert!(plan("SELECT * FROM c JOIN t IN c.tags").query_info.has_join);
    }

    #[test]
    fn no_join() {
        assert!(!plan("SELECT * FROM c").query_info.has_join);
    }

    // ── LocalQueryInfo: Subqueries ────────────────────────────────────────────

    #[test]
    fn exists_subquery_detected() {
        assert!(
            plan("SELECT * FROM c WHERE EXISTS(SELECT VALUE t FROM t IN c.tags)")
                .query_info
                .has_subquery
        );
    }

    #[test]
    fn array_subquery_detected() {
        assert!(
            plan("SELECT ARRAY(SELECT t FROM t IN c.tags) FROM c")
                .query_info
                .has_subquery
        );
    }

    // ── LocalQueryInfo: UDF ───────────────────────────────────────────────────

    #[test]
    fn udf_detected() {
        assert!(
            plan("SELECT * FROM c WHERE udf.myFunc(c.x) > 0")
                .query_info
                .has_udf
        );
    }

    #[test]
    fn builtin_function_not_udf() {
        assert!(
            !plan("SELECT * FROM c WHERE CONTAINS(c.name, 'x')")
                .query_info
                .has_udf
        );
    }

    // ── LocalQueryInfo: WHERE ─────────────────────────────────────────────────

    #[test]
    fn has_where() {
        assert!(plan("SELECT * FROM c WHERE c.x = 1").query_info.has_where);
    }

    #[test]
    fn no_where() {
        assert!(!plan("SELECT * FROM c").query_info.has_where);
    }

    // ── Combined: PK + full query info ───────────────────────────────────

    #[test]
    fn aggregate_with_pk_and_group_by() {
        let qp = plan(
            "SELECT c.city, COUNT(1) AS cnt, SUM(c.revenue) AS total \
             FROM c WHERE c.pk = 'x' GROUP BY c.city",
        );
        assert_eq!(
            qp.pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())])
        );
        assert_eq!(qp.query_info.group_by_expressions, vec!["c.city"]);
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Count));
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Sum));
    }

    #[test]
    fn order_by_with_pk_and_top() {
        let qp = plan("SELECT TOP 5 * FROM c WHERE c.pk = 'x' ORDER BY c.name DESC");
        assert_eq!(
            qp.pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())])
        );
        assert_eq!(qp.query_info.top, Some(5));
        assert_eq!(qp.query_info.order_by, vec![SortOrder::Descending]);
    }

    #[test]
    fn cross_partition_aggregate_with_order_by() {
        let qp = plan("SELECT c.city, COUNT(1) FROM c GROUP BY c.city ORDER BY c.city ASC");
        assert_eq!(qp.pk_filters, PartitionKeyFilter::Unconstrained);
        assert!(!qp.query_info.group_by_expressions.is_empty());
        assert!(!qp.query_info.order_by.is_empty());
        assert!(!qp.query_info.aggregates.is_empty());
    }

    // ── AND intersection logic ───────────────────────────────────────────

    #[test]
    fn and_contradictory_equality_is_contradictory() {
        // c.pk = 'a' AND c.pk = 'b' — contradiction, no partition can match
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b'").pk_filters,
            PartitionKeyFilter::Contradictory
        );
    }

    #[test]
    fn and_redundant_equality_is_ok() {
        // c.pk = 'a' AND c.pk = 'a' — redundant but consistent
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'a'").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_equality_narrows_in_list() {
        // c.pk = 'a' AND c.pk IN ('a', 'b') — narrows to 'a'
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk IN ('a', 'b')").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_equality_not_in_list_is_contradictory() {
        // c.pk = 'c' AND c.pk IN ('a', 'b') — contradiction
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'c' AND c.pk IN ('a', 'b')").pk_filters,
            PartitionKeyFilter::Contradictory
        );
    }

    #[test]
    fn and_in_list_narrows_in_list() {
        // c.pk IN ('a', 'b', 'c') AND c.pk IN ('b', 'c', 'd') — intersection is ('b', 'c')
        let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c') AND c.pk IN ('b', 'c', 'd')");
        match qp.pk_filters {
            PartitionKeyFilter::InList(ref list) => {
                assert_eq!(list.len(), 2);
                assert!(list.contains(&vec![PartitionKeyValue::String("b".into())]));
                assert!(list.contains(&vec![PartitionKeyValue::String("c".into())]));
            }
            _ => panic!("expected InList, got {:?}", qp.pk_filters),
        }
    }

    #[test]
    fn and_in_list_intersection_single_becomes_equality() {
        // c.pk IN ('a', 'b') AND c.pk IN ('b', 'c') — intersection is just 'b'
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('b', 'c')").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("b".into())])
        );
    }

    #[test]
    fn and_in_list_empty_intersection_is_contradictory() {
        // c.pk IN ('a', 'b') AND c.pk IN ('c', 'd') — empty intersection
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('c', 'd')").pk_filters,
            PartitionKeyFilter::Contradictory
        );
    }

    #[test]
    fn and_pk_with_non_pk_keeps_pk() {
        // c.pk = 'a' AND c.other > 5 — non-PK side is None, keep PK side
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.other > 5").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_non_pk_with_pk_keeps_pk() {
        // c.other > 5 AND c.pk = 'a' — reversed order
        assert_eq!(
            plan("SELECT * FROM c WHERE c.other > 5 AND c.pk = 'a'").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_chain_multiple_consistent() {
        // c.pk = 'a' AND c.x > 1 AND c.pk = 'a' AND c.y < 10 — consistent
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.x > 1 AND c.pk = 'a' AND c.y < 10")
                .pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_chain_contradictory() {
        // c.pk = 'a' AND c.x > 1 AND c.pk = 'b' — contradiction deep in chain
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.x > 1 AND c.pk = 'b'").pk_filters,
            PartitionKeyFilter::Contradictory
        );
    }

    #[test]
    fn and_in_list_with_non_pk() {
        // c.pk IN ('a', 'b') AND c.other > 5 — non-PK on one side
        match plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.other > 5").pk_filters {
            PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 2),
            other => panic!("expected InList, got {other:?}"),
        }
    }

    // ── Hierarchical PK AND conflict detection ──────────────────────────

    fn plan_hpk(sql: &str) -> QueryPlan {
        let p = parse(sql).unwrap();
        generate_query_plan(&p.query, &["/tenant", "/userId"]).unwrap()
    }

    #[test]
    fn hpk_contradictory_first_component() {
        assert_eq!(
            plan_hpk("SELECT * FROM c WHERE c.tenant = 'a' AND c.tenant = 'b' AND c.userId = 'u1'")
                .pk_filters,
            PartitionKeyFilter::Contradictory
        );
    }

    #[test]
    fn hpk_contradictory_second_component() {
        assert_eq!(
            plan_hpk(
                "SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.userId = 'u2'"
            )
            .pk_filters,
            PartitionKeyFilter::Contradictory
        );
    }

    #[test]
    fn hpk_redundant_constraints_ok() {
        assert_eq!(
            plan_hpk("SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.tenant = 'a'")
                .pk_filters,
            PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
            ])
        );
    }

    // ── #7: Contradictory short-circuit (regression) ───────────────────────

    /// `c.pk = 'a' AND c.pk = 'b'` is provably empty — surface a distinct
    /// `Contradictory` variant so the routing layer can short-circuit to an
    /// empty feed instead of fanning out across every physical partition.
    #[test]
    fn contradictory_pk_equality_is_distinct_from_unconstrained() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b'").pk_filters,
            PartitionKeyFilter::Contradictory
        );
        // No-WHERE / non-PK WHERE must remain `Unconstrained`, not collapse to
        // `Contradictory`.
        assert_eq!(
            plan("SELECT * FROM c").pk_filters,
            PartitionKeyFilter::Unconstrained
        );
        assert_eq!(
            plan("SELECT * FROM c WHERE c.age > 18").pk_filters,
            PartitionKeyFilter::Unconstrained
        );
    }

    /// `Contradictory` is absorbing under AND-intersection: nesting it inside
    /// a longer chain must not silently degrade back to `Unconstrained`.
    #[test]
    fn contradictory_is_absorbing_under_and() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b' AND c.age > 18").pk_filters,
            PartitionKeyFilter::Contradictory
        );
    }

    // ── #9: PK parameter resolution variants (regression) ──────────────────

    /// An unbound parameter must surface `UnboundParameter`, not collapse to
    /// `Unconstrained` (the routing layer needs to distinguish "user forgot to
    /// bind" from "WHERE has no PK constraint at all").
    #[test]
    fn unbound_pk_parameter_is_distinct_variant() {
        let p = parse("SELECT * FROM c WHERE c.pk = @missing").unwrap();
        let qp = generate_query_plan_with_parameters(&p.query, &["/pk"], &[]).unwrap();
        match qp.pk_filters {
            PartitionKeyFilter::Equality(values) => {
                assert_eq!(values.len(), 1);
                match &values[0] {
                    PartitionKeyValue::UnboundParameter(name) => assert_eq!(name, "missing"),
                    other => panic!("expected UnboundParameter, got {other:?}"),
                }
            }
            other => panic!("expected Equality(UnboundParameter), got {other:?}"),
        }
    }

    /// A parameter bound to an array/object is `InvalidParameter` — the user
    /// did bind it, but the binding is unusable for routing.
    #[test]
    fn invalid_pk_parameter_carries_reason() {
        let p = parse("SELECT * FROM c WHERE c.pk = @bad").unwrap();
        let params = vec![("bad".to_string(), serde_json::json!([1, 2, 3]))];
        let qp = generate_query_plan_with_parameters(&p.query, &["/pk"], &params).unwrap();
        match qp.pk_filters {
            PartitionKeyFilter::Equality(values) => match &values[0] {
                PartitionKeyValue::InvalidParameter { name, reason } => {
                    assert_eq!(name, "bad");
                    assert!(reason.contains("array"), "reason was: {reason}");
                }
                other => panic!("expected InvalidParameter, got {other:?}"),
            },
            other => panic!("expected Equality, got {other:?}"),
        }
    }
}
