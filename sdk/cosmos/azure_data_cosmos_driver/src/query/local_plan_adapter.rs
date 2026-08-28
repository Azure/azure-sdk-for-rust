// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Production adapter that converts a local [`QueryPlan`](super::plan::QueryPlan) into the
//! [`dataflow::QueryPlan`](crate::driver::dataflow::query_plan::QueryPlan) the pipeline consumes.
//!
//! This module provides:
//!
//! - **Shared PK-filter → EPK-range conversion** used by both the production
//!   driver path and the in-memory emulator.
//! - **Production eligibility checks** that reject query shapes requiring
//!   Gateway-only metadata (ORDER BY rewrites, OFFSET/LIMIT, DISTINCT,
//!   aggregates, GROUP BY, DCOUNT, etc.).
//! - **Emulator-only rewrite helpers** (gated by `cfg(any(test, feature =
//!   "__internal_in_memory_emulator"))`) that synthesize `rewrittenQuery` for
//!   advanced query shapes the emulator needs but production must never
//!   fabricate.

use crate::driver::dataflow::query_plan::{self as dataflow, QueryRange};
use crate::error::{CosmosError, CosmosStatus};
use crate::models::{EffectivePartitionKey, PartitionKeyDefinition, PartitionKeyValue};
use crate::query::plan::{self, PartitionKeyFilter};

// ─── PK filter → dataflow QueryRange conversion (shared with emulator) ──────

/// Converts local-plan PK values to model-layer PK values for EPK computation.
///
/// `UnboundParameter` and `InvalidParameter` variants produce errors — callers
/// that want a fallback should check eligibility *before* calling this.
pub(crate) fn model_partition_key_values(
    values: &[plan::PartitionKeyValue],
) -> crate::error::Result<Vec<PartitionKeyValue>> {
    values
        .iter()
        .map(|value| match value {
            plan::PartitionKeyValue::String(s) => Ok(PartitionKeyValue::from(s.clone())),
            plan::PartitionKeyValue::Number(n) => Ok(PartitionKeyValue::from(*n)),
            plan::PartitionKeyValue::Bool(b) => Ok(PartitionKeyValue::from(*b)),
            plan::PartitionKeyValue::Null => Ok(PartitionKeyValue::NULL),
            plan::PartitionKeyValue::Undefined => Ok(PartitionKeyValue::UNDEFINED),
            plan::PartitionKeyValue::UnboundParameter(name) => Err(CosmosError::builder()
                .with_status(CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
                .with_message(format!(
                    "query plan partition key filter references unbound parameter @{name}"
                ))
                .build()),
            plan::PartitionKeyValue::InvalidParameter { name, reason } => {
                Err(CosmosError::builder()
                    .with_status(CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
                    .with_message(format!(
                        "query plan partition key filter parameter @{name} is invalid: {reason}"
                    ))
                    .build())
            }
        })
        .collect()
}

/// Full [MIN, MAX) query range — the entire partition key space.
pub(crate) fn full_query_range() -> QueryRange {
    QueryRange {
        min: EffectivePartitionKey::MIN.to_hex(),
        max: EffectivePartitionKey::MAX.to_hex(),
        is_min_inclusive: true,
        is_max_inclusive: false,
    }
}

/// Converts an EPK `Range<EffectivePartitionKey>` to a `QueryRange`.
pub(crate) fn epk_range_to_query_range(
    range: std::ops::Range<EffectivePartitionKey>,
) -> QueryRange {
    QueryRange {
        min: range.start.to_hex(),
        max: range.end.to_hex(),
        is_min_inclusive: true,
        is_max_inclusive: true,
    }
}

/// Converts a [`PartitionKeyFilter`] to the `Vec<QueryRange>` the dataflow
/// pipeline consumes.
///
/// - `Equality` → single EPK point range.
/// - `InList` → one range per value set, sorted by min EPK and deduplicated.
/// - `Contradictory` → empty vec (short-circuits to empty feed).
/// - `Unconstrained` / `NotEvaluated` → full [MIN, MAX) range.
pub(crate) fn query_ranges_from_pk_filter(
    filter: &PartitionKeyFilter,
    pk_definition: &PartitionKeyDefinition,
) -> crate::error::Result<Vec<QueryRange>> {
    match filter {
        PartitionKeyFilter::Equality(values) => {
            let values = model_partition_key_values(values)?;
            let range = EffectivePartitionKey::compute_range(&values, pk_definition)?;
            Ok(vec![epk_range_to_query_range(range)])
        }
        PartitionKeyFilter::InList(value_sets) => {
            let mut ranges: Vec<QueryRange> = value_sets
                .iter()
                .map(|values| {
                    let values = model_partition_key_values(values)?;
                    EffectivePartitionKey::compute_range(&values, pk_definition)
                        .map(epk_range_to_query_range)
                })
                .collect::<crate::error::Result<Vec<_>>>()?;
            // Sort by min EPK and deduplicate identical ranges.
            ranges.sort_by(|a, b| a.min.cmp(&b.min).then_with(|| a.max.cmp(&b.max)));
            ranges.dedup_by(|a, b| a.min == b.min && a.max == b.max);
            Ok(ranges)
        }
        PartitionKeyFilter::Contradictory => Ok(Vec::new()),
        PartitionKeyFilter::Unconstrained | PartitionKeyFilter::NotEvaluated => {
            Ok(vec![full_query_range()])
        }
    }
}

// ─── LocalQueryInfo → dataflow QueryInfo conversion ─────────────────────────

/// Converts a local [`plan::DistinctType`] to the dataflow
/// [`DistinctType`](dataflow::DistinctType).
pub(crate) fn local_distinct_type_to_dataflow(dt: plan::DistinctType) -> dataflow::DistinctType {
    match dt {
        plan::DistinctType::None => dataflow::DistinctType::None,
        plan::DistinctType::Ordered => dataflow::DistinctType::Ordered,
        plan::DistinctType::Unordered => dataflow::DistinctType::Unordered,
    }
}

/// Converts a local [`plan::SortOrder`] to the dataflow
/// [`SortOrder`](dataflow::SortOrder).
pub(crate) fn local_sort_order_to_dataflow(so: plan::SortOrder) -> dataflow::SortOrder {
    match so {
        plan::SortOrder::Ascending => dataflow::SortOrder::Ascending,
        plan::SortOrder::Descending => dataflow::SortOrder::Descending,
    }
}

/// Converts a [`plan::LocalQueryInfo`] into a production-safe dataflow
/// [`QueryInfo`](dataflow::QueryInfo).
///
/// This is the **production** conversion: it must never synthesize
/// `rewrittenQuery` or any Gateway-only metadata. Callers must verify
/// eligibility via [`check_production_eligibility`] *before* calling this.
fn production_query_info_to_dataflow(info: &plan::LocalQueryInfo) -> dataflow::QueryInfo {
    dataflow::QueryInfo {
        distinct_type: local_distinct_type_to_dataflow(info.distinct_type),
        top: info.top.map(|v| v as u64),
        offset: None,
        limit: None,
        order_by: Vec::new(),
        order_by_expressions: Vec::new(),
        group_by_expressions: Vec::new(),
        group_by_aliases: Vec::new(),
        aggregates: Vec::new(),
        group_by_alias_to_aggregate_type: std::collections::HashMap::new(),
        rewritten_query: None,
        has_select_value: info.has_select_value,
        has_non_streaming_order_by: false,
    }
}

/// Converts a [`plan::LocalQueryInfo`] into a dataflow [`QueryInfo`] for the
/// **emulator**, synthesizing `rewrittenQuery` and advanced metadata the real
/// Gateway would produce.
///
/// This must only be used by the emulator, never by production.
#[cfg(any(test, feature = "__internal_in_memory_emulator"))]
pub(crate) fn emulator_query_info_to_dataflow(
    info: plan::LocalQueryInfo,
    original_query: &str,
) -> dataflow::QueryInfo {
    let rewritten_query = if !info.order_by.is_empty() {
        synthesize_order_by_rewritten_query(original_query, &info.order_by_expressions)
    } else if let Some(limit) = info.limit {
        synthesize_offset_limit_rewritten_query(
            original_query,
            info.offset.unwrap_or(0) as u64,
            limit as u64,
        )
    } else {
        Some(String::new())
    };
    dataflow::QueryInfo {
        distinct_type: local_distinct_type_to_dataflow(info.distinct_type),
        top: info.top.map(|v| v as u64),
        offset: info.offset.map(|v| v as u64),
        limit: info.limit.map(|v| v as u64),
        order_by: info
            .order_by
            .into_iter()
            .map(local_sort_order_to_dataflow)
            .collect(),
        order_by_expressions: info.order_by_expressions,
        group_by_expressions: info.group_by_expressions,
        group_by_aliases: Vec::new(),
        aggregates: info
            .aggregates
            .into_iter()
            .map(|a| format!("{a:?}"))
            .collect(),
        group_by_alias_to_aggregate_type: std::collections::HashMap::new(),
        rewritten_query,
        has_select_value: info.has_select_value,
        has_non_streaming_order_by: false,
    }
}

// ─── Rewritten-query synthesis (emulator-only) ──────────────────────────────

/// Synthesizes the per-partition `rewrittenQuery` envelope the real Gateway
/// returns for `ORDER BY` queries.
#[cfg(any(test, feature = "__internal_in_memory_emulator"))]
pub(crate) fn synthesize_order_by_rewritten_query(
    original_query: &str,
    order_by_expressions: &[String],
) -> Option<String> {
    use crate::query::lexer::{Lexer, TokenKind};

    const ORDER_BY_FILTER_PLACEHOLDER: &str = "{documentdb-formattableorderbyquery-filter}";

    let tokens = Lexer::tokenize(original_query);
    let mut depth = 0_usize;
    let mut top_level = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if matches!(
            token.kind,
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace
        ) {
            depth = depth.saturating_sub(1);
        }
        if depth == 0 {
            top_level.push(index);
        }
        if matches!(
            token.kind,
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace
        ) {
            depth += 1;
        }
    }
    let is_clause_keyword = |index: usize| index == 0 || tokens[index - 1].kind != TokenKind::Dot;
    let select_idx = top_level
        .iter()
        .copied()
        .find(|&i| tokens[i].kind == TokenKind::Select)?;
    let from_idx = top_level
        .iter()
        .copied()
        .find(|&i| tokens[i].kind == TokenKind::From && is_clause_keyword(i))?;
    let collection_token = tokens.get(from_idx + 1)?;
    let alias = match tokens.get(from_idx + 2) {
        Some(t) if t.kind == TokenKind::As => tokens.get(from_idx + 3)?.text,
        Some(t) if t.kind == TokenKind::Identifier => t.text,
        _ => collection_token.text,
    };
    let mut payload_idx = select_idx + 1;
    if tokens
        .get(payload_idx)
        .is_some_and(|t| t.kind == TokenKind::Top)
    {
        payload_idx += 2;
    }
    let payload = match tokens.get(payload_idx)? {
        token if token.kind == TokenKind::Star => alias.to_owned(),
        token if token.kind == TokenKind::Value => original_query
            [token.span.end..tokens[from_idx].span.start]
            .trim()
            .to_owned(),
        _ => return None,
    };

    let order_idx = top_level.iter().copied().find(|&i| {
        tokens[i].kind == TokenKind::Order
            && tokens
                .get(i + 1)
                .is_some_and(|token| token.kind == TokenKind::By)
    });
    let clause_end = order_idx
        .map(|i| tokens[i].span.start)
        .unwrap_or(original_query.len());
    let order_by_end = order_idx.and_then(|oi| {
        top_level
            .iter()
            .copied()
            .find(|&i| i > oi && tokens[i].kind == TokenKind::Offset && is_clause_keyword(i))
            .map(|i| tokens[i].span.start)
    });
    let order_by_text = order_idx.map(|i| {
        let end = order_by_end.unwrap_or(original_query.len());
        original_query[tokens[i].span.start..end].trim()
    })?;

    let where_bound = order_idx.unwrap_or(tokens.len());
    let where_idx = top_level.iter().copied().find(|&i| {
        i > from_idx
            && i < where_bound
            && tokens[i].kind == TokenKind::Where
            && is_clause_keyword(i)
    });
    let from_end = where_idx
        .map(|i| tokens[i].span.start)
        .unwrap_or(clause_end);
    let from_text = original_query[tokens[from_idx].span.start..from_end].trim();
    let where_clause = match where_idx {
        Some(i) => {
            let predicate = original_query[tokens[i].span.end..clause_end].trim();
            format!("WHERE ({predicate}) AND {ORDER_BY_FILTER_PLACEHOLDER}")
        }
        None => format!("WHERE {ORDER_BY_FILTER_PLACEHOLDER}"),
    };

    let order_by_items: Vec<String> = order_by_expressions
        .iter()
        .map(|expr| format!(r#"{{"item": {expr}}}"#))
        .collect();

    Some(format!(
        r#"SELECT VALUE {{"_rid": {alias}._rid, "orderByItems": [{items}], "payload": {payload}}} {from_text} {where_clause} {order_by}"#,
        items = order_by_items.join(", "),
        payload = payload,
        from_text = from_text,
        where_clause = where_clause,
        order_by = order_by_text,
    ))
}

/// Synthesizes the per-partition `rewrittenQuery` for `OFFSET`/`LIMIT`
/// without `ORDER BY`.
#[cfg(any(test, feature = "__internal_in_memory_emulator"))]
pub(crate) fn synthesize_offset_limit_rewritten_query(
    original_query: &str,
    offset: u64,
    limit: u64,
) -> Option<String> {
    use crate::query::lexer::{Lexer, TokenKind};

    let tokens = Lexer::tokenize(original_query);
    let mut depth: u32 = 0;
    let mut outer_offset_idx: Option<usize> = None;
    for (idx, token) in tokens.iter().enumerate() {
        match token.kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => depth += 1,
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                depth = depth.saturating_sub(1);
            }
            TokenKind::Offset if depth == 0 => {
                let is_property_access = idx > 0 && tokens[idx - 1].kind == TokenKind::Dot;
                if !is_property_access {
                    outer_offset_idx = Some(idx);
                }
            }
            _ => {}
        }
    }
    let Some(offset_idx) = outer_offset_idx else {
        return Some(String::new());
    };
    let prefix = original_query[..tokens[offset_idx].span.start].trim_end();
    let combined = offset.saturating_add(limit);
    Some(format!("{prefix} OFFSET 0 LIMIT {combined}"))
}

// ─── Production local-plan eligibility and conversion ───────────────────────

/// Bounded, static fallback-reason codes. Display never includes query text,
/// parameter values, PK values, or unbounded error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalPlanFallbackReason {
    /// The operation body was missing or not valid UTF-8.
    NoBody,
    /// Failed to parse the query spec JSON.
    InvalidQuerySpec,
    /// Query text was blank.
    BlankQuery,
    /// The SQL parser rejected the query.
    ParseFailed,
    /// The local planner could not produce a plan.
    PlanFailed,
    /// Query uses ORDER BY (requires rewrittenQuery).
    OrderBy,
    /// Query uses OFFSET/LIMIT (requires rewrittenQuery).
    OffsetLimit,
    /// Query uses DISTINCT (requires authoritative metadata).
    Distinct,
    /// Query uses aggregates (COUNT, SUM, AVG, MIN, MAX).
    Aggregates,
    /// Query uses GROUP BY (requires aliases/aggregate mapping).
    GroupBy,
    /// Query uses JOIN (conservative safety — declined).
    Join,
    /// Query uses subqueries (conservative safety — declined).
    Subquery,
    /// Query references UDFs (conservative safety — declined).
    Udf,
    /// Query uses DCOUNT metadata that only the Gateway/native providers expose.
    DCount,
    /// Query uses hybrid/vector ranking metadata.
    HybridSearch,
    /// PK filter contains unresolvable parameters.
    UnresolvablePkFilter,
    /// PK-to-EPK conversion failed.
    EpkConversionFailed,
}

impl std::fmt::Display for LocalPlanFallbackReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBody => f.write_str("no_body"),
            Self::InvalidQuerySpec => f.write_str("invalid_query_spec"),
            Self::BlankQuery => f.write_str("blank_query"),
            Self::ParseFailed => f.write_str("parse_failed"),
            Self::PlanFailed => f.write_str("plan_failed"),
            Self::OrderBy => f.write_str("order_by"),
            Self::OffsetLimit => f.write_str("offset_limit"),
            Self::Distinct => f.write_str("distinct"),
            Self::Aggregates => f.write_str("aggregates"),
            Self::GroupBy => f.write_str("group_by"),
            Self::Join => f.write_str("join"),
            Self::Subquery => f.write_str("subquery"),
            Self::Udf => f.write_str("udf"),
            Self::DCount => f.write_str("dcount"),
            Self::HybridSearch => f.write_str("hybrid_search"),
            Self::UnresolvablePkFilter => f.write_str("unresolvable_pk_filter"),
            Self::EpkConversionFailed => f.write_str("epk_conversion_failed"),
        }
    }
}

/// Result of local provider resolution.
#[derive(Debug)]
pub(crate) enum ProviderResolution {
    /// A locally produced plan ready for topology/pipeline construction.
    Plan(Box<dataflow::QueryPlan>),
    /// Contradictory PK filters — provably empty result set. The driver
    /// must build a `DrainedLeaf` plan without topology or backend I/O.
    Empty,
}

/// Returns `true` if the PK filter contains any unresolvable values.
fn has_unresolvable_pk_values(filter: &PartitionKeyFilter) -> bool {
    fn any_unresolvable(values: &[plan::PartitionKeyValue]) -> bool {
        values.iter().any(|v| {
            matches!(
                v,
                plan::PartitionKeyValue::UnboundParameter(_)
                    | plan::PartitionKeyValue::InvalidParameter { .. }
            )
        })
    }

    match filter {
        PartitionKeyFilter::Equality(values) => any_unresolvable(values),
        PartitionKeyFilter::InList(value_sets) => value_sets.iter().any(|vs| any_unresolvable(vs)),
        _ => false,
    }
}

/// Checks whether the local plan's query info is eligible for production
/// execution without Gateway-only metadata.
///
/// Rejects ORDER BY, OFFSET/LIMIT, DISTINCT, aggregates, GROUP BY, JOIN,
/// subqueries, and UDFs. Accepts TOP-only and plain SELECT/WHERE queries.
fn check_production_eligibility(
    info: &plan::LocalQueryInfo,
) -> Result<(), LocalPlanFallbackReason> {
    if !info.order_by.is_empty() {
        return Err(LocalPlanFallbackReason::OrderBy);
    }
    if info.offset.is_some() || info.limit.is_some() {
        return Err(LocalPlanFallbackReason::OffsetLimit);
    }
    if info.distinct_type != plan::DistinctType::None {
        return Err(LocalPlanFallbackReason::Distinct);
    }
    if !info.aggregates.is_empty() {
        return Err(LocalPlanFallbackReason::Aggregates);
    }
    if !info.group_by_expressions.is_empty() {
        return Err(LocalPlanFallbackReason::GroupBy);
    }
    if info.has_join {
        return Err(LocalPlanFallbackReason::Join);
    }
    if info.has_subquery {
        return Err(LocalPlanFallbackReason::Subquery);
    }
    if info.has_udf {
        return Err(LocalPlanFallbackReason::Udf);
    }
    Ok(())
}

fn check_gateway_only_functions(query_text: &str) -> Result<(), LocalPlanFallbackReason> {
    use crate::query::lexer::{Lexer, TokenKind};

    let tokens = Lexer::tokenize(query_text);
    for (index, token) in tokens.iter().enumerate() {
        if token.kind != TokenKind::Identifier
            || tokens.get(index + 1).map(|next| next.kind) != Some(TokenKind::LParen)
            || index > 0 && tokens[index - 1].kind == TokenKind::Dot
        {
            continue;
        }

        match token.text.to_ascii_uppercase().as_str() {
            "DCOUNT" => return Err(LocalPlanFallbackReason::DCount),
            "RRF" | "FULLTEXTSCORE" | "VECTORDISTANCE" => {
                return Err(LocalPlanFallbackReason::HybridSearch);
            }
            _ => {}
        }
    }
    Ok(())
}

/// Attempts to produce a local provider resolution without a Gateway roundtrip.
///
/// Returns `Ok(ProviderResolution::Plan(..))` for eligible queries,
/// `Ok(ProviderResolution::Empty)` for contradictory PK filters, or
/// `Err(reason)` when the local path is ineligible and the caller should
/// fall back to the Gateway.
pub(crate) fn try_local_plan(
    body: Option<&[u8]>,
    pk_definition: &PartitionKeyDefinition,
) -> Result<ProviderResolution, LocalPlanFallbackReason> {
    // 1. Extract query text and parameters from the JSON body.
    let body_str = body
        .and_then(|b| std::str::from_utf8(b).ok())
        .ok_or(LocalPlanFallbackReason::NoBody)?;

    let (query_text, parameters) = parse_query_spec_for_plan(body_str)
        .map_err(|_| LocalPlanFallbackReason::InvalidQuerySpec)?;

    if query_text.trim().is_empty() {
        return Err(LocalPlanFallbackReason::BlankQuery);
    }

    check_gateway_only_functions(&query_text)?;

    // 2. Parse the SQL.
    let program =
        crate::query::parse(&query_text).map_err(|_| LocalPlanFallbackReason::ParseFailed)?;

    // 3. Generate the local plan with parameter substitution.
    let pk_paths: Vec<&str> = pk_definition.paths().iter().map(|p| p.as_ref()).collect();
    let local_plan =
        plan::generate_query_plan_with_parameters(&program.query, &pk_paths, &parameters)
            .map_err(|_| LocalPlanFallbackReason::PlanFailed)?;

    // 4. Production eligibility: reject shapes requiring Gateway-only metadata.
    check_production_eligibility(&local_plan.query_info)?;

    // 5. Pre-acceptance: reject if PK filter has unresolvable parameters.
    if has_unresolvable_pk_values(&local_plan.pk_filters) {
        return Err(LocalPlanFallbackReason::UnresolvablePkFilter);
    }

    // 6. Handle contradictory filters as an explicit empty outcome.
    if matches!(local_plan.pk_filters, PartitionKeyFilter::Contradictory) {
        return Ok(ProviderResolution::Empty);
    }

    // 7. Convert PK filters to EPK query ranges.
    let query_ranges = query_ranges_from_pk_filter(&local_plan.pk_filters, pk_definition)
        .map_err(|_| LocalPlanFallbackReason::EpkConversionFailed)?;

    // 8. Convert LocalQueryInfo to production-safe dataflow QueryInfo.
    let query_info = production_query_info_to_dataflow(&local_plan.query_info);

    Ok(ProviderResolution::Plan(Box::new(dataflow::QueryPlan {
        partitioned_query_execution_info_version: 2,
        query_info: Some(query_info),
        query_ranges,
        hybrid_search_query_info: None,
    })))
}

/// Minimal query-spec JSON parser for the production local-plan path.
fn parse_query_spec_for_plan(json: &str) -> Result<(String, Vec<(String, serde_json::Value)>), ()> {
    #[derive(serde::Deserialize)]
    struct QuerySpec {
        query: String,
        #[serde(default)]
        parameters: Vec<QueryParameter>,
    }
    #[derive(serde::Deserialize)]
    struct QueryParameter {
        name: String,
        value: serde_json::Value,
    }

    let spec: QuerySpec = serde_json::from_str(json).map_err(|_| ())?;
    let parameters = spec
        .parameters
        .into_iter()
        .map(|p| (p.name, p.value))
        .collect();
    Ok((spec.query, parameters))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    fn single_pk_def() -> PartitionKeyDefinition {
        PartitionKeyDefinition::new(vec![Cow::Borrowed("/pk")])
    }

    fn hpk_def() -> PartitionKeyDefinition {
        PartitionKeyDefinition::new(vec![Cow::Borrowed("/tenantId"), Cow::Borrowed("/userId")])
    }

    // ── Query body parsing ──────────────────────────────────────────────

    #[test]
    fn no_body_causes_fallback() {
        let result = try_local_plan(None, &single_pk_def());
        assert_eq!(result.unwrap_err(), LocalPlanFallbackReason::NoBody);
    }

    #[test]
    fn invalid_utf8_causes_fallback() {
        let result = try_local_plan(Some(&[0xFF, 0xFE]), &single_pk_def());
        assert_eq!(result.unwrap_err(), LocalPlanFallbackReason::NoBody);
    }

    #[test]
    fn invalid_json_causes_fallback() {
        let result = try_local_plan(Some(b"not json"), &single_pk_def());
        assert_eq!(
            result.unwrap_err(),
            LocalPlanFallbackReason::InvalidQuerySpec
        );
    }

    #[test]
    fn empty_body_causes_fallback() {
        let result = try_local_plan(Some(b""), &single_pk_def());
        assert_eq!(
            result.unwrap_err(),
            LocalPlanFallbackReason::InvalidQuerySpec
        );
    }

    #[test]
    fn non_object_json_causes_fallback() {
        let result = try_local_plan(Some(b"42"), &single_pk_def());
        assert_eq!(
            result.unwrap_err(),
            LocalPlanFallbackReason::InvalidQuerySpec
        );
    }

    #[test]
    fn missing_query_field_causes_fallback() {
        let result = try_local_plan(Some(br#"{"parameters": []}"#), &single_pk_def());
        assert_eq!(
            result.unwrap_err(),
            LocalPlanFallbackReason::InvalidQuerySpec
        );
    }

    #[test]
    fn blank_query_causes_fallback() {
        let result = try_local_plan(Some(br#"{"query": "   "}"#), &single_pk_def());
        assert_eq!(result.unwrap_err(), LocalPlanFallbackReason::BlankQuery);
    }

    #[test]
    fn valid_body_omitted_parameters_accepted() {
        let body = br#"{"query": "SELECT * FROM c"}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        assert!(matches!(result, Ok(ProviderResolution::Plan(_))));
    }

    #[test]
    fn valid_body_empty_parameters_accepted() {
        let body = br#"{"query": "SELECT * FROM c", "parameters": []}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        assert!(matches!(result, Ok(ProviderResolution::Plan(_))));
    }

    #[test]
    fn bound_parameter_accepted() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.pk = @val", "parameters": [{"name": "@val", "value": "hello"}]}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        assert!(matches!(result, Ok(ProviderResolution::Plan(ref p)) if p.query_ranges.len() == 1));
    }

    #[test]
    fn parameterized_top_is_accepted() {
        let body = br#"{"query": "SELECT TOP @count * FROM c", "parameters": [{"name": "@count", "value": 3}]}"#;
        let result = try_local_plan(Some(body), &single_pk_def()).unwrap();
        let ProviderResolution::Plan(plan) = result else {
            panic!("expected local plan");
        };
        assert_eq!(plan.query_info.unwrap().top, Some(3));
    }

    #[test]
    fn invalid_parameterized_top_falls_back() {
        for value in [
            serde_json::json!(-1),
            serde_json::json!(1.5),
            serde_json::json!("3"),
        ] {
            let body = serde_json::to_vec(&serde_json::json!({
                "query": "SELECT TOP @count * FROM c",
                "parameters": [{"name": "@count", "value": value}],
            }))
            .unwrap();
            assert_eq!(
                try_local_plan(Some(&body), &single_pk_def()).unwrap_err(),
                LocalPlanFallbackReason::PlanFailed
            );
        }
    }

    #[test]
    fn invalid_pk_parameter_falls_back() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.pk = @value", "parameters": [{"name": "@value", "value": [1, 2]}]}"#;
        assert_eq!(
            try_local_plan(Some(body), &single_pk_def()).unwrap_err(),
            LocalPlanFallbackReason::UnresolvablePkFilter
        );
    }

    #[test]
    fn unbound_parameter_causes_fallback() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.pk = @val"}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        assert_eq!(
            result.unwrap_err(),
            LocalPlanFallbackReason::UnresolvablePkFilter
        );
    }

    // ── Production eligibility ──────────────────────────────────────────

    #[test]
    fn plain_select_accepted() {
        let body = br#"{"query": "SELECT * FROM c"}"#;
        assert!(matches!(
            try_local_plan(Some(body), &single_pk_def()),
            Ok(ProviderResolution::Plan(_))
        ));
    }

    #[test]
    fn select_value_accepted() {
        let body = br#"{"query": "SELECT VALUE c.name FROM c"}"#;
        assert!(matches!(
            try_local_plan(Some(body), &single_pk_def()),
            Ok(ProviderResolution::Plan(_))
        ));
    }

    #[test]
    fn top_only_accepted() {
        let body = br#"{"query": "SELECT TOP 5 * FROM c"}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        match result {
            Ok(ProviderResolution::Plan(p)) => {
                let qi = p.query_info.as_ref().unwrap();
                assert_eq!(qi.top, Some(5));
                assert!(qi.order_by.is_empty());
                assert!(qi.rewritten_query.is_none());
            }
            other => panic!("expected Plan, got {other:?}"),
        }
    }

    #[test]
    fn order_by_rejected() {
        let body = br#"{"query": "SELECT * FROM c ORDER BY c.name ASC"}"#;
        assert_eq!(
            try_local_plan(Some(body), &single_pk_def()).unwrap_err(),
            LocalPlanFallbackReason::OrderBy
        );
    }

    #[test]
    fn offset_limit_rejected() {
        let body = br#"{"query": "SELECT * FROM c OFFSET 0 LIMIT 10"}"#;
        assert_eq!(
            try_local_plan(Some(body), &single_pk_def()).unwrap_err(),
            LocalPlanFallbackReason::OffsetLimit
        );
    }

    #[test]
    fn distinct_rejected() {
        let body = br#"{"query": "SELECT DISTINCT c.name FROM c"}"#;
        assert_eq!(
            try_local_plan(Some(body), &single_pk_def()).unwrap_err(),
            LocalPlanFallbackReason::Distinct
        );
    }

    #[test]
    fn aggregate_count_rejected() {
        let body = br#"{"query": "SELECT COUNT(1) FROM c"}"#;
        assert_eq!(
            try_local_plan(Some(body), &single_pk_def()).unwrap_err(),
            LocalPlanFallbackReason::Aggregates
        );
    }

    #[test]
    fn group_by_rejected() {
        // Pure GROUP BY without aggregates to isolate the check.
        let body = br#"{"query": "SELECT c.city FROM c GROUP BY c.city"}"#;
        let reason = try_local_plan(Some(body), &single_pk_def()).unwrap_err();
        // The local planner may report GroupBy or ParseFailed depending on
        // parser support; either is a valid rejection.
        assert!(
            reason == LocalPlanFallbackReason::GroupBy
                || reason == LocalPlanFallbackReason::Aggregates
                || reason == LocalPlanFallbackReason::ParseFailed,
            "expected GroupBy/Aggregates/ParseFailed, got {reason}"
        );
    }

    #[test]
    fn join_rejected() {
        let body = br#"{"query": "SELECT * FROM c JOIN t IN c.tags"}"#;
        assert_eq!(
            try_local_plan(Some(body), &single_pk_def()).unwrap_err(),
            LocalPlanFallbackReason::Join
        );
    }

    #[test]
    fn subquery_rejected() {
        let body =
            br#"{"query": "SELECT * FROM c WHERE c.pk IN (SELECT VALUE t FROM t IN c.tags)"}"#;
        let reason = try_local_plan(Some(body), &single_pk_def()).unwrap_err();
        // The local parser may not support subquery syntax and return ParseFailed.
        assert!(
            reason == LocalPlanFallbackReason::Subquery
                || reason == LocalPlanFallbackReason::ParseFailed,
            "expected Subquery or ParseFailed, got {reason}"
        );
    }

    // ── PK filter → EPK range conversion ────────────────────────────────

    #[test]
    fn equality_filter_produces_point_range() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.pk = 'hello'"}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        match result {
            Ok(ProviderResolution::Plan(p)) => {
                assert_eq!(p.query_ranges.len(), 1);
                assert_eq!(p.query_ranges[0].min, p.query_ranges[0].max);
                assert!(p.query_ranges[0].is_min_inclusive);
                assert!(p.query_ranges[0].is_max_inclusive);
            }
            other => panic!("expected Plan, got {other:?}"),
        }
    }

    #[test]
    fn in_list_filter_produces_sorted_deduped_ranges() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.pk IN ('c', 'a', 'b', 'a')"}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        match result {
            Ok(ProviderResolution::Plan(p)) => {
                // 'a' appears twice → deduplicated to 3 ranges.
                assert_eq!(p.query_ranges.len(), 3);
                // Ranges are sorted by min EPK.
                for w in p.query_ranges.windows(2) {
                    assert!(w[0].min <= w[1].min, "ranges must be sorted");
                }
            }
            other => panic!("expected Plan, got {other:?}"),
        }
    }

    #[test]
    fn contradictory_filter_produces_empty() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b'"}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        assert!(
            matches!(result, Ok(ProviderResolution::Empty)),
            "contradictory → Empty"
        );
    }

    #[test]
    fn unconstrained_produces_full_range() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.name = 'x'"}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        match result {
            Ok(ProviderResolution::Plan(p)) => {
                assert_eq!(p.query_ranges.len(), 1);
                assert_eq!(p.query_ranges[0].min, EffectivePartitionKey::MIN.to_hex());
                assert_eq!(p.query_ranges[0].max, EffectivePartitionKey::MAX.to_hex());
            }
            other => panic!("expected Plan, got {other:?}"),
        }
    }

    // ── HPK tests ───────────────────────────────────────────────────────

    #[test]
    fn hpk_full_equality_produces_point_range() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.tenantId = 'T1' AND c.userId = 'U1'"}"#;
        let result = try_local_plan(Some(body), &hpk_def());
        match result {
            Ok(ProviderResolution::Plan(p)) => {
                assert_eq!(p.query_ranges.len(), 1);
                assert_eq!(p.query_ranges[0].min, p.query_ranges[0].max);
            }
            other => panic!("expected Plan, got {other:?}"),
        }
    }

    #[test]
    fn hpk_prefix_produces_prefix_range() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.tenantId = 'T1'"}"#;
        let result = try_local_plan(Some(body), &hpk_def());
        match result {
            Ok(ProviderResolution::Plan(p)) => {
                assert_eq!(p.query_ranges.len(), 1);
                // Prefix range: min != max (covers all suffixes).
                assert_ne!(p.query_ranges[0].min, p.query_ranges[0].max);
            }
            other => panic!("expected Plan, got {other:?}"),
        }
    }

    #[test]
    fn hpk_with_parameter_accepted() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.tenantId = @t AND c.userId = @u", "parameters": [{"name": "@t", "value": "T1"}, {"name": "@u", "value": "U1"}]}"#;
        let result = try_local_plan(Some(body), &hpk_def());
        assert!(matches!(result, Ok(ProviderResolution::Plan(ref p)) if p.query_ranges.len() == 1));
    }

    #[test]
    fn hpk_prefix_in_produces_sorted_distinct_ranges() {
        let body = br#"{"query": "SELECT * FROM c WHERE c.tenantId IN ('T2', 'T1', 'T2')"}"#;
        let result = try_local_plan(Some(body), &hpk_def()).unwrap();
        let ProviderResolution::Plan(plan) = result else {
            panic!("expected local plan");
        };
        assert_eq!(plan.query_ranges.len(), 2);
        assert!(plan.query_ranges[0].min < plan.query_ranges[1].min);
        assert!(plan.query_ranges.iter().all(|range| range.min != range.max));
    }

    // ── Fallback reason Display is bounded/static ───────────────────────

    #[test]
    fn fallback_display_never_leaks_query_text() {
        let sentinel = "SECRET_QUERY_TEXT_12345";
        let body = format!(r#"{{"query": "{sentinel}"}}"#);
        let result = try_local_plan(Some(body.as_bytes()), &single_pk_def());
        if let Err(reason) = result {
            let display = format!("{reason}");
            assert!(
                !display.contains(sentinel),
                "Display must not contain query text: {display}"
            );
        }
    }

    #[test]
    fn fallback_display_never_leaks_parameter_values() {
        let sentinel = "SECRET_PARAM_VALUE_67890";
        let body = format!(
            r#"{{"query": "SELECT * FROM c WHERE c.pk = @p", "parameters": [{{"name": "@p", "value": "{sentinel}"}}]}}"#
        );
        let result = try_local_plan(Some(body.as_bytes()), &single_pk_def());
        // Whether it succeeds or fails, the display must not contain the value.
        let display = match &result {
            Ok(_) => String::new(),
            Err(reason) => format!("{reason}"),
        };
        assert!(
            !display.contains(sentinel),
            "Display must not contain param values: {display}"
        );
    }

    // ── Production plan never manufactures Gateway metadata ─────────────

    #[test]
    fn production_plan_has_no_rewritten_query_metadata() {
        let body = br#"{"query": "SELECT TOP 10 * FROM c WHERE c.pk = 'a'"}"#;
        let result = try_local_plan(Some(body), &single_pk_def());
        match result {
            Ok(ProviderResolution::Plan(p)) => {
                let qi = p.query_info.as_ref().unwrap();
                assert!(qi.rewritten_query.is_none());
                assert!(qi.order_by.is_empty());
                assert!(qi.order_by_expressions.is_empty());
                assert!(qi.group_by_expressions.is_empty());
                assert!(qi.group_by_aliases.is_empty());
                assert!(qi.aggregates.is_empty());
                assert!(qi.group_by_alias_to_aggregate_type.is_empty());
                assert!(!qi.has_non_streaming_order_by);
                assert!(qi.offset.is_none());
                assert!(qi.limit.is_none());
            }
            other => panic!("expected Plan, got {other:?}"),
        }
    }

    // ── Numeric EPK canonicalization ────────────────────────────────────

    #[test]
    fn numeric_1_and_1_0_produce_same_epk() {
        let body_int = br#"{"query": "SELECT * FROM c WHERE c.pk = 1"}"#;
        let body_float = br#"{"query": "SELECT * FROM c WHERE c.pk = 1.0"}"#;
        let pk = PartitionKeyDefinition::new(vec![Cow::Borrowed("/pk")]);
        let r1 = match try_local_plan(Some(body_int), &pk).unwrap() {
            ProviderResolution::Plan(p) => p.query_ranges[0].min.clone(),
            _ => panic!("expected Plan"),
        };
        let r2 = match try_local_plan(Some(body_float), &pk).unwrap() {
            ProviderResolution::Plan(p) => p.query_ranges[0].min.clone(),
            _ => panic!("expected Plan"),
        };
        assert_eq!(r1, r2, "1 and 1.0 must produce the same EPK");
    }

    #[test]
    fn dcount_requires_gateway_metadata() {
        let body = br#"{"query": "SELECT VALUE DCOUNT(c.pk) FROM c"}"#;
        assert_eq!(
            try_local_plan(Some(body), &single_pk_def()).unwrap_err(),
            LocalPlanFallbackReason::DCount
        );
    }

    #[test]
    fn hybrid_functions_require_gateway_metadata() {
        for function in [
            "RRF(c.score)",
            "FullTextScore(c.text, 'term')",
            "VectorDistance(c.v, [1])",
        ] {
            let body = format!(r#"{{"query": "SELECT VALUE {function} FROM c"}}"#);
            assert_eq!(
                try_local_plan(Some(body.as_bytes()), &single_pk_def()).unwrap_err(),
                LocalPlanFallbackReason::HybridSearch
            );
        }
    }

    #[test]
    fn gateway_only_function_names_used_as_properties_are_not_rejected() {
        let body = br#"{"query": "SELECT VALUE c.DCOUNT FROM c"}"#;
        assert!(matches!(
            try_local_plan(Some(body), &single_pk_def()),
            Ok(ProviderResolution::Plan(_))
        ));
    }
}
