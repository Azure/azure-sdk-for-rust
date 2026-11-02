// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Service-style rewrites for the emulator, never the production local provider.

use crate::{
    driver::dataflow::query_plan::{DCountInfo, QueryInfo, QueryPlan},
    error::{CosmosError, CosmosStatus, Result},
    models::PartitionKeyDefinition,
    query::{
        ast::{
            SqlCollection, SqlCollectionExpression, SqlLiteral, SqlQuery, SqlScalarExpression,
            SqlSelectSpec,
        },
        common::{infer_property_name, Params},
        local_plan_adapter::{emulator_query_info_to_dataflow, query_ranges_from_pk_filter},
        plan::generate_query_plan_with_parameters,
    },
};
use serde_json::Value;

pub(crate) fn generate(
    query: &SqlQuery,
    original: &str,
    parameters: &Params,
    definition: &PartitionKeyDefinition,
) -> Result<QueryPlan> {
    let paths: Vec<_> = definition
        .paths()
        .iter()
        .map(|path| path.as_ref())
        .collect();
    if let Some((inner, alias)) = distinct_count(query)? {
        let text = render_source_query(inner)?;
        let local = generate_query_plan_with_parameters(inner, &paths, parameters)?;
        let ranges = query_ranges_from_pk_filter(&local.pk_filters, definition)?;
        let mut info = emulator_query_info_to_dataflow(local.query_info, &text);
        info.d_count_info = Some(DCountInfo {
            d_count_alias: Some(alias),
        });
        info.rewritten_query = Some(if inner.order_by.is_some() {
            info.rewritten_query
                .filter(|rewrite| !rewrite.is_empty())
                .ok_or_else(|| invalid("ordered distinct-count rewrite is unavailable"))?
        } else {
            text
        });
        return Ok(QueryPlan {
            partitioned_query_execution_info_version: 2,
            query_info: Some(info),
            query_ranges: ranges,
            hybrid_search_query_info: None,
        });
    }

    // Structural local analysis is path-only for GROUP BY. The emulator can
    // interpret scalar keys, without broadening production eligibility.
    let mut structural = query.clone();
    structural.group_by = None;
    let local = generate_query_plan_with_parameters(&structural, &paths, parameters)?;
    let advanced = query.group_by.is_some() || !local.query_info.aggregates.is_empty();
    let ranges = query_ranges_from_pk_filter(&local.pk_filters, definition)?;
    let mut info = emulator_query_info_to_dataflow(local.query_info, original);
    if advanced {
        rewrite_aggregates(query, &mut info)?;
    }
    Ok(QueryPlan {
        partitioned_query_execution_info_version: 2,
        query_info: Some(info),
        query_ranges: ranges,
        hybrid_search_query_info: None,
    })
}

fn distinct_count(query: &SqlQuery) -> Result<Option<(&SqlQuery, String)>> {
    let Some(from) = &query.from else {
        return Ok(None);
    };
    let SqlCollectionExpression::Aliased {
        collection: SqlCollection::Subquery(inner),
        ..
    } = &from.collection
    else {
        return Ok(None);
    };
    let (expression, alias) = match &query.select.spec {
        SqlSelectSpec::Value(expression) => (expression.as_ref(), String::new()),
        SqlSelectSpec::List(items) if items.len() == 1 => (
            &items[0].expression,
            items[0].alias.clone().unwrap_or_else(|| "$1".into()),
        ),
        _ => {
            return Err(invalid(
                "only canonical COUNT-over-DISTINCT subqueries are supported",
            ))
        }
    };
    let canonical_count = matches!(
        expression,
        SqlScalarExpression::FunctionCall { name, args, is_udf: false }
        if name.eq_ignore_ascii_case("COUNT")
            && matches!(args.as_slice(), [SqlScalarExpression::Literal(SqlLiteral::Integer(1))])
    );
    if !canonical_count
        || query.select.distinct
        || query.select.top.is_some()
        || query.where_clause.is_some()
        || query.group_by.is_some()
        || query.order_by.is_some()
        || query.offset_limit.is_some()
        || !inner.select.distinct
        || !matches!(inner.select.spec, SqlSelectSpec::Value(_))
        || inner.select.top.is_some()
        || inner.group_by.is_some()
        || inner.offset_limit.is_some()
    {
        return Err(invalid(
            "only canonical COUNT-over-DISTINCT subqueries are supported",
        ));
    }
    root_source(inner)?;
    Ok(Some((inner, alias)))
}

fn rewrite_aggregates(query: &SqlQuery, info: &mut QueryInfo) -> Result<()> {
    if query.order_by.is_some() {
        return Err(invalid(
            "emulator aggregate ORDER BY combinations are unsupported",
        ));
    }
    if let Some(group) = &query.group_by {
        if query.select.distinct {
            let complete_keys = match &query.select.spec {
                SqlSelectSpec::Value(value) => {
                    group.expressions.first() == Some(value.as_ref())
                        && group.expressions.len() == 1
                }
                SqlSelectSpec::List(items) => group
                    .expressions
                    .iter()
                    .all(|key| items.iter().any(|item| &item.expression == key)),
                SqlSelectSpec::Star => false,
            };
            // The inherited DISTINCT stage precedes grouping. Only a redundant
            // DISTINCT over complete scalar keys is safe in that composition.
            if info.has_aggregates() || !complete_keys {
                return Err(invalid(
                    "emulator DISTINCT with GROUP BY requires complete keys and no aggregates",
                ));
            }
        }
    }
    let source = root_source(query)?;
    let mut suffix = format!(" FROM {source}");
    if let Some(filter) = &query.where_clause {
        suffix.push_str(&format!(" WHERE {}", scalar(&filter.expression)?));
    }
    info.aggregates.clear();
    let projection = match &query.select.spec {
        SqlSelectSpec::Value(expression) => {
            if let Some((operator, argument)) = aggregate(expression)? {
                info.aggregates.push(operator.into());
                format!("[{}]", cell(operator, &argument))
            } else if query.group_by.is_some() {
                validate_group_scalar(expression, query)?;
                scalar(expression)?
            } else {
                return Err(invalid(
                    "aggregate VALUE projection must be a single aggregate",
                ));
            }
        }
        SqlSelectSpec::List(items) => {
            let mut properties = Vec::new();
            for (index, item) in items.iter().enumerate() {
                let operator = aggregate(&item.expression)?;
                let alias = item.alias.clone().unwrap_or_else(|| {
                    if operator.is_some() {
                        format!("${}", index + 1)
                    } else {
                        infer_property_name(&item.expression, index + 1)
                    }
                });
                if info.group_by_alias_to_aggregate_type.contains_key(&alias) {
                    return Err(invalid("duplicate aggregate projection alias"));
                }
                let (value, metadata) = match operator {
                    Some((operator, argument)) => {
                        (cell(operator, &argument), Value::from(operator))
                    }
                    None if query.group_by.is_some() => {
                        validate_group_scalar(&item.expression, query)?;
                        (scalar(&item.expression)?, Value::Null)
                    }
                    None => {
                        return Err(invalid(
                            "non-grouped aggregate projection contains a scalar",
                        ))
                    }
                };
                info.group_by_aliases.push(alias.clone());
                info.group_by_alias_to_aggregate_type
                    .insert(alias.clone(), metadata);
                properties.push(format!("{}: {value}", quoted(&alias)));
            }
            format!("{{{}}}", properties.join(", "))
        }
        SqlSelectSpec::Star => {
            return Err(invalid(
                "SELECT * is unsupported with aggregates or GROUP BY",
            ))
        }
    };
    info.rewritten_query = Some(if let Some(group) = &query.group_by {
        info.group_by_expressions = group
            .expressions
            .iter()
            .map(scalar)
            .collect::<Result<_>>()?;
        let keys = info
            .group_by_expressions
            .iter()
            .map(|key| format!("{{\"item\": {key}}}"))
            .collect::<Vec<_>>();
        format!(
            "SELECT [{}] AS groupByItems, {projection} AS payload{suffix} GROUP BY {}",
            keys.join(", "),
            info.group_by_expressions.join(", ")
        )
    } else if info.has_select_value {
        format!("SELECT VALUE {projection}{suffix}")
    } else {
        format!("SELECT {projection} AS payload{suffix}")
    });
    Ok(())
}

fn validate_group_scalar(expression: &SqlScalarExpression, query: &SqlQuery) -> Result<()> {
    fn covered(expression: &SqlScalarExpression, keys: &[SqlScalarExpression]) -> bool {
        use SqlScalarExpression as E;
        if keys.contains(expression) {
            return true;
        }
        match expression {
            E::Literal(_) | E::ParameterRef(_) => true,
            E::FunctionCall { args, .. } | E::ArrayCreate(args) => {
                args.iter().all(|arg| covered(arg, keys))
            }
            E::ObjectCreate(properties) => properties
                .iter()
                .all(|property| covered(&property.expression, keys)),
            E::MemberRef { source, .. }
            | E::Unary {
                operand: source, ..
            }
            | E::IsNull {
                expression: source, ..
            } => covered(source, keys),
            E::Binary { left, right, .. }
            | E::Coalesce { left, right }
            | E::MemberIndexer {
                source: left,
                index: right,
            } => covered(left, keys) && covered(right, keys),
            E::Conditional {
                condition,
                if_true,
                if_false,
            } => covered(condition, keys) && covered(if_true, keys) && covered(if_false, keys),
            E::Between {
                expression,
                low,
                high,
                ..
            } => covered(expression, keys) && covered(low, keys) && covered(high, keys),
            E::In {
                expression, items, ..
            } => covered(expression, keys) && items.iter().all(|item| covered(item, keys)),
            E::Like {
                expression,
                pattern,
                ..
            } => covered(expression, keys) && covered(pattern, keys),
            _ => false,
        }
    }
    if query
        .group_by
        .as_ref()
        .is_some_and(|group| covered(expression, &group.expressions))
    {
        Ok(())
    } else {
        Err(invalid(
            "scalar projection must depend only on GROUP BY keys",
        ))
    }
}

fn aggregate(expression: &SqlScalarExpression) -> Result<Option<(&'static str, String)>> {
    let SqlScalarExpression::FunctionCall {
        name,
        args,
        is_udf: false,
    } = expression
    else {
        // scalar() rejects aggregates nested in user projections.
        scalar(expression)?;
        return Ok(None);
    };
    let operator = match name.to_ascii_uppercase().as_str() {
        "COUNT" => "Count",
        "SUM" => "Sum",
        "AVG" => "Average",
        "MIN" => "Min",
        "MAX" => "Max",
        _ => {
            scalar(expression)?;
            return Ok(None);
        }
    };
    if args.len() != 1 {
        return Err(invalid("aggregate requires exactly one argument"));
    }
    Ok(Some((operator, scalar(&args[0])?)))
}

fn cell(operator: &str, argument: &str) -> String {
    match operator {
        "Average" => format!("{{\"item\": {{\"sum\": SUM({argument}), \"count\": COUNT({argument})}}}}"),
        "Min" | "Max" => format!(
            "{{\"item\": {}({argument}), \"item2\": {{\"{}\": {}({argument}), \"count\": COUNT({argument})}}}}",
            operator.to_ascii_uppercase(), operator.to_ascii_lowercase(), operator.to_ascii_uppercase()
        ),
        _ => format!("{{\"item\": {}({argument})}}", operator.to_ascii_uppercase()),
    }
}

fn root_source(query: &SqlQuery) -> Result<String> {
    match query.from.as_ref().map(|from| &from.collection) {
        Some(SqlCollectionExpression::Aliased {
            collection: SqlCollection::Path { root, path },
            alias,
        }) if path.is_empty() => Ok(match alias {
            Some(alias) => format!("{root} AS {alias}"),
            None => root.clone(),
        }),
        _ => Err(invalid(
            "emulator advanced queries require a root collection source",
        )),
    }
}

fn render_source_query(query: &SqlQuery) -> Result<String> {
    let SqlSelectSpec::Value(value) = &query.select.spec else {
        return Err(invalid("distinct-count source must use SELECT VALUE"));
    };
    let value = scalar(value)?;
    let mut text = format!(
        "SELECT DISTINCT VALUE {value} FROM {} WHERE IS_DEFINED({value})",
        root_source(query)?
    );
    if let Some(filter) = &query.where_clause {
        text.push_str(&format!(" AND ({})", scalar(&filter.expression)?));
    }
    if let Some(order) = &query.order_by {
        let items = order
            .items
            .iter()
            .map(|item| {
                let direction = if matches!(item.order, crate::query::ast::SqlSortOrder::Descending)
                {
                    "DESC"
                } else {
                    "ASC"
                };
                Ok(format!("{} {direction}", scalar(&item.expression)?))
            })
            .collect::<Result<Vec<_>>>()?;
        text.push_str(&format!(" ORDER BY {}", items.join(", ")));
    }
    Ok(text)
}

fn quoted(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn scalar(expression: &SqlScalarExpression) -> Result<String> {
    use SqlScalarExpression as E;
    Ok(match expression {
        E::Literal(value) => match value {
            SqlLiteral::String(value) => quoted(value),
            SqlLiteral::Integer(value) => value.to_string(),
            SqlLiteral::Number(value) if value.is_finite() => value.to_string(),
            SqlLiteral::Boolean(value) => value.to_string(),
            SqlLiteral::Null => "null".into(),
            SqlLiteral::Undefined => "undefined".into(),
            _ => return Err(invalid("non-finite query literal")),
        },
        E::PropertyRef(name) => name.clone(),
        E::ParameterRef(name) => format!("@{name}"),
        E::MemberRef { source, member } => {
            if member.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                && member
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                format!("{}.{member}", scalar(source)?)
            } else {
                format!("{}[{}]", scalar(source)?, quoted(member))
            }
        }
        E::MemberIndexer { source, index } => format!("{}[{}]", scalar(source)?, scalar(index)?),
        E::Binary { op, left, right } => format!("({} {op} {})", scalar(left)?, scalar(right)?),
        E::Unary { op, operand } => format!("({op} {})", scalar(operand)?),
        E::FunctionCall {
            name,
            args,
            is_udf: false,
        } => {
            if matches!(
                name.to_ascii_uppercase().as_str(),
                "COUNT" | "SUM" | "AVG" | "MIN" | "MAX"
            ) {
                return Err(invalid("composite aggregates are unsupported"));
            }
            format!(
                "{name}({})",
                args.iter()
                    .map(scalar)
                    .collect::<Result<Vec<_>>>()?
                    .join(", ")
            )
        }
        E::ArrayCreate(items) => format!(
            "[{}]",
            items
                .iter()
                .map(scalar)
                .collect::<Result<Vec<_>>>()?
                .join(", ")
        ),
        E::ObjectCreate(items) => format!(
            "{{{}}}",
            items
                .iter()
                .map(|item| Ok(format!(
                    "{}: {}",
                    quoted(&item.name),
                    scalar(&item.expression)?
                )))
                .collect::<Result<Vec<_>>>()?
                .join(", ")
        ),
        E::Conditional {
            condition,
            if_true,
            if_false,
        } => format!(
            "({} ? {} : {})",
            scalar(condition)?,
            scalar(if_true)?,
            scalar(if_false)?
        ),
        E::Coalesce { left, right } => format!("({} ?? {})", scalar(left)?, scalar(right)?),
        E::In {
            expression,
            items,
            not,
        } => format!(
            "({} {}IN ({}))",
            scalar(expression)?,
            if *not { "NOT " } else { "" },
            items
                .iter()
                .map(scalar)
                .collect::<Result<Vec<_>>>()?
                .join(", ")
        ),
        E::Between {
            expression,
            low,
            high,
            not,
        } => format!(
            "({} {}BETWEEN {} AND {})",
            scalar(expression)?,
            if *not { "NOT " } else { "" },
            scalar(low)?,
            scalar(high)?
        ),
        E::IsNull { expression, not } => format!(
            "({} IS {}NULL)",
            scalar(expression)?,
            if *not { "NOT " } else { "" }
        ),
        E::Like {
            expression,
            pattern,
            escape,
            not,
        } => format!(
            "({} {}LIKE {}{})",
            scalar(expression)?,
            if *not { "NOT " } else { "" },
            scalar(pattern)?,
            match escape {
                Some(value) => format!(" ESCAPE {}", quoted(value)),
                None => String::new(),
            }
        ),
        _ => return Err(invalid("unsupported emulator advanced query expression")),
    })
}

fn invalid(message: &str) -> CosmosError {
    CosmosError::builder()
        .with_status(CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
        .with_message(message.to_owned())
        .build()
}

#[cfg(test)]
mod tests {
    use super::{generate, PartitionKeyDefinition, Value};
    use serde_json::json;

    #[test]
    fn service_derived_partial_fixtures() {
        let catalog: Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/aggregate_scenarios.json"
        ))
        .unwrap();
        for scenario in catalog["scenarios"].as_array().unwrap() {
            let query = scenario["query"].as_str().unwrap();
            let ast = crate::query::parse(query).unwrap();
            let plan =
                generate(&ast.query, query, &[], &PartitionKeyDefinition::from("/pk")).unwrap();
            let info = plan.query_info.unwrap();
            let rewrite = info.rewritten_query.unwrap();
            let rows = crate::query::eval::query_documents(
                &rewrite,
                &[],
                scenario["documents"].as_array().unwrap(),
            )
            .unwrap();
            assert_eq!(json!(rows), scenario["rows"], "{query}: {rewrite}");
        }
    }

    #[test]
    fn distinct_count_metadata_and_inner_ranges() {
        for (projection, alias) in [
            ("VALUE COUNT(1)", ""),
            ("COUNT(1)", "$1"),
            ("COUNT(1) AS total", "total"),
        ] {
            let query = format!("SELECT {projection} FROM (SELECT DISTINCT VALUE c.category FROM c WHERE c.pk = @pk) AS t");
            let ast = crate::query::parse(&query).unwrap();
            let plan = generate(
                &ast.query,
                &query,
                &[("pk".into(), json!("a"))],
                &PartitionKeyDefinition::from("/pk"),
            )
            .unwrap();
            let info = plan.query_info.unwrap();
            assert!(!info.has_aggregates());
            assert_eq!(
                info.d_count_info.unwrap().d_count_alias.as_deref(),
                Some(alias)
            );
            assert_ne!(plan.query_ranges[0].min, "");
            assert!(info.rewritten_query.unwrap().contains("@pk"));
        }
    }

    #[test]
    fn rejects_composite_and_general_subqueries() {
        for query in [
            "SELECT VALUE SUM(c.v) + 1 FROM c",
            "SELECT c.key, ABS(AVG(c.v)) FROM c GROUP BY c.key",
            "SELECT VALUE COUNT(1) FROM (SELECT VALUE c.v FROM c) AS t",
            "SELECT DISTINCT c.key, COUNT(1) AS n FROM c GROUP BY c.key",
            "SELECT DISTINCT VALUE c.key FROM c GROUP BY c.key, c.other",
            "SELECT c.other FROM c GROUP BY c.key",
        ] {
            let ast = crate::query::parse(query).unwrap();
            assert!(
                generate(&ast.query, query, &[], &PartitionKeyDefinition::from("/pk")).is_err(),
                "{query}"
            );
        }
    }
}
