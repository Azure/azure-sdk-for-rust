// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Selection and filtering for finite, non-resumable read-many operations.

use std::{collections::BTreeMap, str::FromStr};

use azure_core::fmt::SafeDebug;
use serde_json::Value;

use crate::{models::PartitionKey, query::ast::SqlScalarExpression};

mod predicate;

/// Items or complete logical partitions to read from one container.
///
/// Duplicate selections are removed before execution. Empty selections read
/// nothing. Results are unordered and missing items are omitted.
#[derive(Clone, SafeDebug)]
#[non_exhaustive]
pub enum ReadManySelection {
    /// Exact `(partition key, item id)` pairs.
    Items(Vec<(PartitionKey, String)>),
    /// Complete logical partition keys; hierarchical prefixes are not supported.
    Partitions(Vec<PartitionKey>),
}

/// A parameterized WHERE predicate that only narrows a read-many selection.
///
/// Parse a scalar SQL expression using alias `c`, then bind its parameters with
/// [`with_parameter()`](Self::with_parameter). SELECT statements, subqueries,
/// and user-defined functions are not supported.
/// Filters are limited to 256 lexical tokens. Use parameters or JSON
/// double-quoted strings for values containing escape sequences.
///
/// ```
/// use azure_data_cosmos_driver::read_many::ReadManyFilter;
/// let filter = "c.active = @active".parse::<ReadManyFilter>()?
///     .with_parameter("@active", serde_json::json!(true))?;
/// # Ok::<(), azure_data_cosmos_driver::CosmosError>(())
/// ```
#[derive(Clone, SafeDebug)]
pub struct ReadManyFilter {
    pub(crate) expression: SqlScalarExpression,
    pub(crate) parameters: BTreeMap<String, Value>,
}

impl FromStr for ReadManyFilter {
    type Err = crate::CosmosError;

    fn from_str(value: &str) -> crate::Result<Self> {
        let expression = crate::query::parser::parse_predicate(value)
            .map_err(|_| invalid("invalid read-many filter predicate"))?;
        // Validate supported expression forms independently of parameter binding.
        predicate::render(&expression, &mut |name| Ok(name.to_owned()))?;
        Ok(Self {
            expression,
            parameters: BTreeMap::new(),
        })
    }
}

impl ReadManyFilter {
    /// Binds a JSON value to a parameter, including the leading `@`.
    ///
    /// Returns an error for duplicate names or names absent from the predicate.
    /// All referenced parameters must be bound before planning the operation.
    pub fn with_parameter(mut self, name: impl Into<String>, value: Value) -> crate::Result<Self> {
        let name = name.into();
        let mut found = false;
        predicate::render(&self.expression, &mut |reference| {
            found |= reference == name;
            Ok(reference.to_owned())
        })?;
        if !found || self.parameters.contains_key(&name) {
            return Err(invalid("filter parameter is duplicated or not referenced"));
        }
        self.parameters.insert(name, value);
        Ok(self)
    }

    pub(crate) fn query_predicate(&self) -> crate::Result<String> {
        predicate::render(&self.expression, &mut |name| {
            if !self.parameters.contains_key(name) {
                return Err(invalid("read-many filter has an unbound parameter"));
            }
            Ok(name.to_owned())
        })
    }
}

#[derive(Clone, SafeDebug)]
pub(crate) struct ReadManyRequest {
    pub(crate) selection: ReadManySelection,
    pub(crate) filter: Option<ReadManyFilter>,
}

pub(crate) fn invalid(message: &'static str) -> crate::CosmosError {
    crate::CosmosError::builder()
        .with_status(crate::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

#[cfg(test)]
mod tests {
    use super::ReadManyFilter;
    use serde_json::json;

    #[test]
    fn predicates_preserve_precedence_and_parameter_names() {
        let filter = "c.rank = @a OR c.name = @b -- ignored\n"
            .parse::<ReadManyFilter>()
            .unwrap()
            .with_parameter("@a", json!(2))
            .unwrap()
            .with_parameter("@b", json!("quote\""))
            .unwrap();
        assert_eq!(
            filter.query_predicate().unwrap(),
            "((c[\"rank\"] = @a) OR (c[\"name\"] = @b))"
        );
        assert!(filter.clone().with_parameter("@a", json!(3)).is_err());
        assert!(filter.with_parameter("@unused", json!(3)).is_err());
    }

    #[test]
    fn rejects_query_fragments_and_unbound_parameters() {
        for text in [
            "",
            "SELECT * FROM c",
            "true) OR true",
            "true ORDER BY c.id",
            "EXISTS(SELECT * FROM c)",
            "udf.test(c.id)",
            "true; SELECT * FROM c",
            "c.x = @x /* unterminated",
        ] {
            assert!(text.parse::<ReadManyFilter>().is_err(), "{text}");
        }
        assert!("c.x = @x"
            .parse::<ReadManyFilter>()
            .unwrap()
            .query_predicate()
            .is_err());
    }

    #[test]
    fn object_property_escapes_preserve_decoded_names() {
        for (source, expected) in [
            (r#"({"\u0061": true})["a"]"#, r#"{"a": true}["a"]"#),
            (r#"({"a\"b": true})["a\"b"]"#, r#"{"a\"b": true}["a\"b"]"#),
            (r#"({"a\\b": true})["a\\b"]"#, r#"{"a\\b": true}["a\\b"]"#),
        ] {
            assert_eq!(
                source
                    .parse::<ReadManyFilter>()
                    .unwrap()
                    .query_predicate()
                    .unwrap(),
                expected
            );
        }
    }
}
