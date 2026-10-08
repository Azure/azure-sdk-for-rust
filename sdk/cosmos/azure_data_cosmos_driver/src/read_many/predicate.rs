// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    query::ast::{SqlLiteral, SqlScalarExpression as Expr},
    read_many::invalid,
};

fn quoted(value: &str) -> crate::Result<String> {
    serde_json::to_string(value).map_err(Into::into)
}

/// Fully parenthesize expressions so caller predicates cannot escape selection.
pub(super) fn render(
    expression: &Expr,
    parameter: &mut impl FnMut(&str) -> crate::Result<String>,
) -> crate::Result<String> {
    Ok(match expression {
        Expr::Literal(value) => match value {
            SqlLiteral::String(value) => quoted(value)?,
            SqlLiteral::Number(value) if value.is_finite() => value.to_string(),
            SqlLiteral::Integer(value) => value.to_string(),
            SqlLiteral::Boolean(value) => value.to_string(),
            SqlLiteral::Null => "null".into(),
            SqlLiteral::Undefined => "undefined".into(),
            _ => return Err(invalid("invalid filter literal")),
        },
        Expr::PropertyRef(name) if name == "c" => "c".into(),
        Expr::ParameterRef(name) => parameter(&format!("@{name}"))?,
        Expr::MemberRef { source, member } => {
            format!("{}[{}]", render(source, parameter)?, quoted(member)?)
        }
        Expr::MemberIndexer { source, index } => {
            format!(
                "{}[{}]",
                render(source, parameter)?,
                render(index, parameter)?
            )
        }
        Expr::Binary { op, left, right } => format!(
            "({} {op} {})",
            render(left, parameter)?,
            render(right, parameter)?
        ),
        Expr::Unary { op, operand } => format!("({op} {})", render(operand, parameter)?),
        Expr::FunctionCall {
            name,
            args,
            is_udf: false,
        } if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            && !["COUNT", "SUM", "AVG", "MIN", "MAX"]
                .contains(&name.to_ascii_uppercase().as_str()) =>
        {
            let args = args
                .iter()
                .map(|arg| render(arg, parameter))
                .collect::<crate::Result<Vec<_>>>()?;
            format!("{name}({})", args.join(", "))
        }
        Expr::Between {
            expression,
            low,
            high,
            not,
        } => format!(
            "({} {}BETWEEN {} AND {})",
            render(expression, parameter)?,
            if *not { "NOT " } else { "" },
            render(low, parameter)?,
            render(high, parameter)?
        ),
        Expr::In {
            expression,
            items,
            not,
        } => {
            let items = items
                .iter()
                .map(|item| render(item, parameter))
                .collect::<crate::Result<Vec<_>>>()?;
            format!(
                "({} {}IN ({}))",
                render(expression, parameter)?,
                if *not { "NOT " } else { "" },
                items.join(", ")
            )
        }
        Expr::Like {
            expression,
            pattern,
            escape,
            not,
        } => {
            let escape = escape
                .as_deref()
                .map(quoted)
                .transpose()?
                .map(|value| format!(" ESCAPE {value}"))
                .unwrap_or_default();
            format!(
                "({} {}LIKE {}{escape})",
                render(expression, parameter)?,
                if *not { "NOT " } else { "" },
                render(pattern, parameter)?
            )
        }
        Expr::Conditional {
            condition,
            if_true,
            if_false,
        } => format!(
            "({} ? {} : {})",
            render(condition, parameter)?,
            render(if_true, parameter)?,
            render(if_false, parameter)?
        ),
        Expr::Coalesce { left, right } => format!(
            "({} ?? {})",
            render(left, parameter)?,
            render(right, parameter)?
        ),
        Expr::ArrayCreate(items) => {
            let items = items
                .iter()
                .map(|item| render(item, parameter))
                .collect::<crate::Result<Vec<_>>>()?;
            format!("[{}]", items.join(", "))
        }
        Expr::ObjectCreate(properties) => {
            let properties = properties
                .iter()
                .map(|p| {
                    Ok(format!(
                        "{}: {}",
                        quoted(&p.name)?,
                        render(&p.expression, parameter)?
                    ))
                })
                .collect::<crate::Result<Vec<_>>>()?;
            format!("{{{}}}", properties.join(", "))
        }
        _ => {
            return Err(invalid(
                "unsupported read-many filter expression; use query_items for full queries",
            ))
        }
    })
}
