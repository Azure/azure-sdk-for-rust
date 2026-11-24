// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Executable aggregate projections over backend-generated partial results.

use crate::{
    driver::dataflow::query_plan::{QueryInfo, QueryPlan},
    models::PartitionKeyDefinition,
    query::{
        ast::{SqlQuery, SqlScalarExpression, SqlSelectSpec},
        common::Params,
        lexer::{Lexer, Token, TokenKind},
        local_plan_adapter::{
            full_query_range, has_unresolvable_pk_values, local_distinct_type_to_dataflow,
            local_sort_order_to_dataflow, query_ranges_from_pk_filter,
            synthesize_order_by_rewritten_query, LocalPlanFallbackReason,
        },
        parser::{parse_with_spans, QuerySpans},
        plan::{self, PartitionKeyFilter},
    },
};

type Result<T> = std::result::Result<T, LocalPlanFallbackReason>;

/// Returns an executable plan when the outer query requires aggregate reduction.
pub(crate) fn try_generate(
    query: &SqlQuery,
    source: &str,
    pk_definition: &PartitionKeyDefinition,
    parameters: &Params,
) -> Result<Option<QueryPlan>> {
    let has_aggregate = match &query.select.spec {
        SqlSelectSpec::Value(expression) => contains_aggregate(expression),
        SqlSelectSpec::List(items) => items
            .iter()
            .any(|item| contains_aggregate(&item.expression)),
        SqlSelectSpec::Star => false,
    };
    if !has_aggregate && query.group_by.is_none() {
        return Ok(None);
    }
    if query.from.is_none() {
        return Err(LocalPlanFallbackReason::Aggregates);
    }
    if query
        .where_clause
        .as_ref()
        .is_some_and(|clause| contains_aggregate(&clause.expression))
        || query
            .group_by
            .as_ref()
            .is_some_and(|clause| clause.expressions.iter().any(contains_aggregate))
    {
        return Err(LocalPlanFallbackReason::Aggregates);
    }

    let text = QueryText::new(source)?;

    let pk_paths: Vec<_> = pk_definition
        .paths()
        .iter()
        .map(|path| path.as_ref())
        .collect();
    let local = plan::generate_query_plan_with_parameters(query, &pk_paths, parameters)
        .map_err(|_| LocalPlanFallbackReason::PlanFailed)?;
    let mut info = QueryInfo {
        distinct_type: local_distinct_type_to_dataflow(local.query_info.distinct_type),
        top: local.query_info.top.map(|value| value as u64),
        offset: local.query_info.offset.map(|value| value as u64),
        limit: local.query_info.limit.map(|value| value as u64),
        group_by_expressions: local.query_info.group_by_expressions.clone(),
        has_select_value: local.query_info.has_select_value,
        ..Default::default()
    };
    let expressions = text.projection_expressions(query)?;
    let payload = match &query.select.spec {
        SqlSelectSpec::Value(expression) => {
            if let Some(operator) = aggregate_operator(expression) {
                info.aggregates.push(operator.to_owned());
                format!("[{}]", aggregate_cell(expression, &expressions[0])?)
            } else if contains_aggregate(expression) {
                return Err(LocalPlanFallbackReason::Aggregates);
            } else {
                expressions[0].to_owned()
            }
        }
        SqlSelectSpec::List(items) => {
            let mut fields = Vec::with_capacity(items.len());
            let mut unnamed = 0_u64;
            for (item, expression) in items.iter().zip(&expressions) {
                let mut generated_alias = || {
                    unnamed += 1;
                    format!("${unnamed}")
                };
                let alias = item
                    .alias
                    .clone()
                    .unwrap_or_else(|| match &item.expression {
                        SqlScalarExpression::PropertyRef(name) => name.clone(),
                        SqlScalarExpression::MemberRef { member, .. } => member.clone(),
                        SqlScalarExpression::MemberIndexer { index, .. } => match index.as_ref() {
                            SqlScalarExpression::Literal(
                                crate::query::ast::SqlLiteral::String(name),
                            ) => name.clone(),
                            _ => generated_alias(),
                        },
                        _ => generated_alias(),
                    });
                if info.group_by_alias_to_aggregate_type.contains_key(&alias) {
                    return Err(LocalPlanFallbackReason::Aggregates);
                }
                let (operator, value) = match aggregate_operator(&item.expression) {
                    Some(operator) => (
                        serde_json::Value::String(operator.to_owned()),
                        aggregate_cell(&item.expression, expression)?,
                    ),
                    None if contains_aggregate(&item.expression) => {
                        return Err(LocalPlanFallbackReason::Aggregates);
                    }
                    None => (serde_json::Value::Null, expression.clone()),
                };
                fields.push(format!("{}: {value}", quoted(&alias)));
                info.group_by_aliases.push(alias.clone());
                info.group_by_alias_to_aggregate_type
                    .insert(alias, operator);
            }
            format!("{{{}}}", fields.join(", "))
        }
        SqlSelectSpec::Star => return Err(LocalPlanFallbackReason::GroupBy),
    };
    let ordered_groups =
        info.has_group_by() && !has_aggregate && !local.query_info.order_by.is_empty();
    let distinct = if query.select.distinct && !ordered_groups {
        "DISTINCT "
    } else {
        ""
    };
    let projection = if info.has_group_by() {
        let keys = info
            .group_by_expressions
            .iter()
            .map(|key| format!(r#"{{"item": {key}}}"#))
            .collect::<Vec<_>>()
            .join(", ");
        format!("SELECT {distinct}[{keys}] AS groupByItems, {payload} AS payload")
    } else if info.has_select_value {
        format!("SELECT {distinct}VALUE {payload}")
    } else {
        format!("SELECT {distinct}{payload} AS payload")
    };
    let mut rewrite = format!("{projection}{}", text.source_tail()?);
    if !info.has_group_by() && !info.has_select_value {
        if let Some(limit) = info.limit {
            rewrite.push_str(&format!(
                " OFFSET 0 LIMIT {}",
                info.offset.unwrap_or(0).saturating_add(limit)
            ));
        }
    }
    info.rewritten_query = Some(if ordered_groups {
        info.order_by = local
            .query_info
            .order_by
            .iter()
            .copied()
            .map(local_sort_order_to_dataflow)
            .collect();
        info.order_by_expressions = local.query_info.order_by_expressions;
        synthesize_order_by_rewritten_query(&rewrite, &info.order_by_expressions)
            .ok_or(LocalPlanFallbackReason::OrderByRewriteUnavailable)?
    } else {
        rewrite
    });
    finish(info, &local.pk_filters, pk_definition).map(Some)
}

fn finish(
    info: QueryInfo,
    filter: &PartitionKeyFilter,
    pk_definition: &PartitionKeyDefinition,
) -> Result<QueryPlan> {
    if has_unresolvable_pk_values(filter) {
        return Err(LocalPlanFallbackReason::UnresolvablePkFilter);
    }
    // An empty source still produces a COUNT/SUM result. Keep the predicate
    // on a normal backend request rather than returning an empty public feed.
    let query_ranges = if matches!(filter, PartitionKeyFilter::Contradictory) {
        vec![full_query_range()]
    } else {
        query_ranges_from_pk_filter(filter, pk_definition)
            .map_err(|_| LocalPlanFallbackReason::EpkConversionFailed)?
    };
    Ok(QueryPlan {
        partitioned_query_execution_info_version: 2,
        query_info: Some(info),
        query_ranges,
        hybrid_search_query_info: None,
    })
}

fn aggregate_operator(expression: &SqlScalarExpression) -> Option<&'static str> {
    let SqlScalarExpression::FunctionCall {
        name,
        is_udf: false,
        ..
    } = expression
    else {
        return None;
    };
    match name.to_ascii_uppercase().as_str() {
        "COUNT" => Some("Count"),
        "SUM" => Some("Sum"),
        "AVG" => Some("Average"),
        "MIN" => Some("Min"),
        "MAX" => Some("Max"),
        _ => None,
    }
}

fn contains_aggregate(expression: &SqlScalarExpression) -> bool {
    let mut stack = vec![expression];
    while let Some(expression) = stack.pop() {
        if aggregate_operator(expression).is_some() {
            return true;
        }
        match expression {
            SqlScalarExpression::FunctionCall { args, .. }
            | SqlScalarExpression::ArrayCreate(args) => stack.extend(args),
            SqlScalarExpression::ObjectCreate(properties) => {
                stack.extend(properties.iter().map(|property| &property.expression));
            }
            SqlScalarExpression::Binary { left, right, .. }
            | SqlScalarExpression::Coalesce { left, right } => {
                stack.extend([left.as_ref(), right.as_ref()]);
            }
            SqlScalarExpression::Unary { operand, .. } => stack.push(operand),
            SqlScalarExpression::IsNull { expression, .. } => stack.push(expression),
            SqlScalarExpression::MemberRef { source, .. } => stack.push(source),
            SqlScalarExpression::MemberIndexer { source, index } => {
                stack.extend([source.as_ref(), index.as_ref()]);
            }
            SqlScalarExpression::In {
                expression, items, ..
            } => {
                stack.push(expression);
                stack.extend(items);
            }
            SqlScalarExpression::Between {
                expression,
                low,
                high,
                ..
            } => {
                stack.extend([expression.as_ref(), low.as_ref(), high.as_ref()]);
            }
            SqlScalarExpression::Like {
                expression,
                pattern,
                ..
            } => {
                stack.extend([expression.as_ref(), pattern.as_ref()]);
            }
            SqlScalarExpression::Conditional {
                condition,
                if_true,
                if_false,
            } => {
                stack.extend([condition.as_ref(), if_true.as_ref(), if_false.as_ref()]);
            }
            _ => {}
        }
    }
    false
}

fn aggregate_cell(expression: &SqlScalarExpression, source: &str) -> Result<String> {
    let SqlScalarExpression::FunctionCall { args, .. } = expression else {
        return Err(LocalPlanFallbackReason::Aggregates);
    };
    let [argument] = args.as_slice() else {
        return Err(LocalPlanFallbackReason::Aggregates);
    };
    if contains_aggregate(argument) {
        return Err(LocalPlanFallbackReason::Aggregates);
    }
    let operator = aggregate_operator(expression).ok_or(LocalPlanFallbackReason::Aggregates)?;
    let tokens = Lexer::tokenize(source);
    let mut tokens = tokens.as_slice();
    while tokens
        .last()
        .is_some_and(|token| token.kind == TokenKind::Eof)
    {
        tokens = &tokens[..tokens.len() - 1];
    }
    while tokens
        .first()
        .is_some_and(|token| token.kind == TokenKind::LParen)
        && tokens
            .last()
            .is_some_and(|token| token.kind == TokenKind::RParen)
    {
        tokens = &tokens[1..tokens.len() - 1];
    }
    if tokens.len() < 4
        || tokens[1].kind != TokenKind::LParen
        || tokens
            .last()
            .is_none_or(|token| token.kind != TokenKind::RParen)
    {
        return Err(LocalPlanFallbackReason::Aggregates);
    }
    let argument = source[tokens[1].span.end..tokens[tokens.len() - 1].span.start].trim();
    Ok(match operator {
        "Average" => {
            format!(r#"{{"item": {{"sum": SUM({argument}), "count": COUNT({argument})}}}}"#)
        }
        "Min" | "Max" => {
            let function = operator.to_ascii_uppercase();
            let property = operator.to_ascii_lowercase();
            format!(
                r#"{{"item": {function}({argument}), "item2": {{"{property}": {function}({argument}), "count": COUNT({argument})}}}}"#
            )
        }
        "Count" | "Sum" => format!(
            r#"{{"item": {}({argument})}}"#,
            operator.to_ascii_uppercase()
        ),
        _ => return Err(LocalPlanFallbackReason::Aggregates),
    })
}

fn quoted(value: &str) -> String {
    serde_json::Value::String(value.to_owned()).to_string()
}

struct QueryText<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    top_level: Vec<usize>,
    spans: QuerySpans,
}

impl<'a> QueryText<'a> {
    fn new(source: &'a str) -> Result<Self> {
        let (_, spans) =
            parse_with_spans(source).map_err(|_| LocalPlanFallbackReason::ParseFailed)?;
        let tokens = Lexer::tokenize(source);
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
        Ok(Self {
            source,
            tokens,
            top_level,
            spans,
        })
    }

    fn clause(&self, kind: TokenKind) -> Option<usize> {
        self.top_level.iter().copied().find(|index| {
            self.tokens[*index].kind == kind
                && (*index == 0 || self.tokens[*index - 1].kind != TokenKind::Dot)
        })
    }

    fn projection_expressions(&self, query: &SqlQuery) -> Result<Vec<String>> {
        let mut start = self
            .clause(TokenKind::Select)
            .ok_or(LocalPlanFallbackReason::Aggregates)?
            + 1;
        if query.select.distinct {
            start += 1;
        }
        if query.select.top.is_some() {
            start += 2;
        }
        if matches!(query.select.spec, SqlSelectSpec::Value(_)) {
            start += 1;
        }
        let end = self
            .spans
            .from
            .and_then(|span| {
                self.tokens
                    .iter()
                    .position(|token| token.span.start == span.start)
            })
            .or_else(|| self.clause(TokenKind::Eof))
            .ok_or(LocalPlanFallbackReason::Aggregates)?;
        let mut boundaries: Vec<_> = self
            .top_level
            .iter()
            .copied()
            .filter(|index| {
                *index >= start && *index < end && self.tokens[*index].kind == TokenKind::Comma
            })
            .collect();
        boundaries.push(end);
        let aliases: Vec<_> = match &query.select.spec {
            SqlSelectSpec::Value(_) => vec![false],
            SqlSelectSpec::List(items) => items.iter().map(|item| item.alias.is_some()).collect(),
            SqlSelectSpec::Star => return Err(LocalPlanFallbackReason::GroupBy),
        };
        if aliases.len() != boundaries.len() {
            return Err(LocalPlanFallbackReason::Aggregates);
        }
        let mut expressions = Vec::with_capacity(aliases.len());
        for (end, has_alias) in boundaries.into_iter().zip(aliases) {
            let mut expression_end = end;
            if has_alias {
                expression_end = expression_end
                    .checked_sub(1)
                    .ok_or(LocalPlanFallbackReason::Aggregates)?;
                if expression_end > start && self.tokens[expression_end - 1].kind == TokenKind::As {
                    expression_end -= 1;
                }
            }
            if expression_end <= start {
                return Err(LocalPlanFallbackReason::Aggregates);
            }
            expressions.push(
                self.source
                    [self.tokens[start].span.start..self.tokens[expression_end - 1].span.end]
                    .to_owned(),
            );
            start = end + 1;
        }
        Ok(expressions)
    }

    fn source_tail(&self) -> Result<String> {
        let from = self.spans.from.ok_or(LocalPlanFallbackReason::Aggregates)?;
        let end = self.spans.window_prefix_end;
        let tail = self
            .source
            .get(from.start..end)
            .ok_or(LocalPlanFallbackReason::Aggregates)?;
        Ok(format!(" {}", tail.trim()))
    }
}

#[cfg(test)]
mod tests {
    use super::{full_query_range, PartitionKeyDefinition, QueryInfo, QueryPlan};
    use crate::query::local_plan_adapter::{try_local_plan, ProviderResolution};

    fn generate(sql: &str) -> QueryPlan {
        let body = serde_json::to_vec(&serde_json::json!({"query": sql})).unwrap();
        let ProviderResolution::Plan(plan) =
            try_local_plan(Some(&body), &PartitionKeyDefinition::from("/pk")).unwrap()
        else {
            panic!("aggregate must produce a plan");
        };
        *plan
    }

    #[test]
    fn value_partials_match_gateway_contract() {
        for (function, operator, partial) in [
            ("COUNT", "Count", r#"{"item": COUNT(c.x)}"#),
            ("SUM", "Sum", r#"{"item": SUM(c.x)}"#),
            (
                "AVG",
                "Average",
                r#"{"item": {"sum": SUM(c.x), "count": COUNT(c.x)}}"#,
            ),
            (
                "MIN",
                "Min",
                r#"{"item": MIN(c.x), "item2": {"min": MIN(c.x), "count": COUNT(c.x)}}"#,
            ),
            (
                "MAX",
                "Max",
                r#"{"item": MAX(c.x), "item2": {"max": MAX(c.x), "count": COUNT(c.x)}}"#,
            ),
        ] {
            let plan = generate(&format!("SELECT VALUE {function}(c.x) FROM c"));
            let mut expected = QueryInfo {
                aggregates: vec![operator.into()],
                has_select_value: true,
                rewritten_query: Some(format!("SELECT VALUE [{partial}] FROM c")),
                ..Default::default()
            };
            assert_eq!(plan.query_info, Some(expected.clone()));
            let grouped = generate(&format!(
                "SELECT VALUE {function}(c.x) FROM c GROUP BY c.key"
            ));
            expected.group_by_expressions = vec!["c.key".into()];
            expected.rewritten_query = Some(format!(
                r#"SELECT [{{"item": c.key}}] AS groupByItems, [{partial}] AS payload FROM c GROUP BY c.key"#
            ));
            assert_eq!(grouped.query_info, Some(expected));
        }
    }

    #[test]
    fn unnamed_aliases_do_not_count_named_properties() {
        let info =
            generate("SELECT c.key, COUNT(1), AVG(c.x) AS mean, SUM(c.x) FROM c GROUP BY c.key")
                .query_info
                .unwrap();
        let expected: QueryInfo = serde_json::from_value(serde_json::json!({
            "groupByExpressions":["c.key"],
            "groupByAliases":["key","$1","mean","$2"],
            "groupByAliasToAggregateType":{"key":null,"$1":"Count","mean":"Average","$2":"Sum"},
            "rewrittenQuery":"SELECT [{\"item\": c.key}] AS groupByItems, {\"key\": c.key, \"$1\": {\"item\": COUNT(1)}, \"mean\": {\"item\": {\"sum\": SUM(c.x), \"count\": COUNT(c.x)}}, \"$2\": {\"item\": SUM(c.x)}} AS payload FROM c GROUP BY c.key"
        })).unwrap();
        assert_eq!(info, expected);
    }

    #[test]
    fn quoted_keyword_alias_preserves_real_window() {
        let info = generate("SELECT COUNT(1) AS \"OFFSET\" FROM c WHERE c.note = 'OFFSET 9 LIMIT 9' -- OFFSET 7 LIMIT 8\nOFFSET 1 LIMIT 2")
            .query_info.unwrap();
        assert_eq!((info.offset, info.limit), (Some(1), Some(2)));
        assert_eq!(info.group_by_aliases, ["OFFSET"]);
        assert_eq!(
            info.rewritten_query.as_deref(),
            Some(
                r#"SELECT {"OFFSET": {"item": COUNT(1)}} AS payload FROM c WHERE c.note = 'OFFSET 9 LIMIT 9' OFFSET 0 LIMIT 3"#
            )
        );
    }

    #[test]
    fn unsupported_aggregate_shapes_keep_fallback() {
        for sql in [
            "SELECT VALUE COUNT(1)",
            "SELECT VALUE SUM(1)",
            "SELECT VALUE AVG(1)",
            "SELECT VALUE MIN(1)",
            "SELECT VALUE MAX(1)",
            "SELECT VALUE SUM(c.x) + 1 FROM c",
            "SELECT VALUE {n: COUNT(1)} FROM c",
            "SELECT VALUE SUM(COUNT(1)) FROM c",
            "SELECT VALUE COUNT(c.x, c.y) FROM c",
            "SELECT VALUE COUNT(1) FROM c WHERE COUNT(1) > 0",
            "SELECT c.key, COUNT(1) FROM c GROUP BY c.key + 1",
            "SELECT c.key, COUNT(1) FROM c GROUP BY c['key']",
            // cspell:disable-next-line
            "SELECT VALUE COUNTIF(c.x) FROM c",
            "SELECT VALUE ARRAY_AGG(c.x) FROM c",
            "SELECT VALUE COUNT(t.x) FROM (SELECT DISTINCT VALUE c.x FROM c) AS t",
            "SELECT VALUE COUNT(1) FROM (SELECT DISTINCT TOP 2 VALUE c.x FROM c) AS t",
            "SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE c.x FROM c OFFSET 1 LIMIT 2) AS t",
            "SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE c.x FROM c ORDER BY c.x) AS t",
        ] {
            let body = serde_json::to_vec(&serde_json::json!({"query":sql})).unwrap();
            assert!(
                try_local_plan(Some(&body), &PartitionKeyDefinition::from("/pk")).is_err(),
                "{sql}"
            );
        }
    }

    #[test]
    fn rewrites_preserve_source_expressions_and_global_windows() {
        let info = generate("SELECT TOP 2 c.key name, AVG(/* sum */ (c.x + 1)) mean FROM c WHERE c.kind = 'FROM, AS' GROUP BY c.key OFFSET 1 LIMIT 2")
            .query_info.unwrap();
        assert_eq!(
            (info.top, info.offset, info.limit),
            (Some(2), Some(1), Some(2))
        );
        assert_eq!(info.rewritten_query.as_deref(), Some("SELECT [{\"item\": c.key}] AS groupByItems, {\"name\": c.key, \"mean\": {\"item\": {\"sum\": SUM(/* sum */ (c.x + 1)), \"count\": COUNT(/* sum */ (c.x + 1))}}} AS payload FROM c WHERE c.kind = 'FROM, AS' GROUP BY c.key"));
    }

    #[test]
    fn contradictory_aggregate_input_does_not_drain_public_output() {
        let plan = generate("SELECT VALUE COUNT(1) FROM c WHERE c.pk = 'a' AND c.pk = 'b'");
        assert_eq!(plan.query_ranges, [full_query_range()]);
        assert!(plan.query_info.unwrap().has_aggregates());
    }
}
