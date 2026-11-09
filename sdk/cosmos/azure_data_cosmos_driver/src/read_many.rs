// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Selection and filtering for finite, non-resumable read-many operations.

use std::{collections::BTreeMap, str::FromStr};

use azure_core::fmt::SafeDebug;
use serde::Serialize;
use serde_json::Value;

use crate::{
    models::{FeedRange, PartitionKey},
    query::ast::SqlScalarExpression,
};

mod predicate;

/// Items, logical partitions, or feed ranges selected within one container.
///
/// Complete keys and nonempty hierarchical prefixes are supported. Duplicate and
/// overlapping selections do not duplicate results. Empty selections read nothing.
/// Results are unordered and missing items are omitted.
#[derive(Clone, SafeDebug)]
#[non_exhaustive]
pub enum ItemScope {
    /// `(partition key or hierarchical prefix, item id)` pairs.
    ///
    /// A prefix plus an id returns every matching item across its logical partitions.
    Items(Vec<(PartitionKey, String)>),
    /// Complete logical partition keys or nonempty hierarchical prefixes.
    Partitions(Vec<PartitionKey>),
    /// Feed ranges whose union selects items without duplicating overlaps.
    FeedRanges(Vec<FeedRange>),
}

/// An item scope and an optional narrowing WHERE predicate.
///
/// Construct from an [`ItemScope`], then optionally call [`Self::with_filter()`].
#[derive(Clone, SafeDebug)]
pub struct ItemSelector {
    pub(crate) selection: ItemScope,
    pub(crate) filter: Option<ItemFilter>,
}

impl From<ItemScope> for ItemSelector {
    fn from(selection: ItemScope) -> Self {
        Self {
            selection,
            filter: None,
        }
    }
}

impl ItemSelector {
    /// Sets the predicate evaluated within the selected scope.
    pub fn with_filter(mut self, filter: impl Into<ItemFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }
}

/// A parameterized WHERE predicate that narrows an item selection.
///
/// Parse a scalar SQL expression using alias `c`, without the WHERE keyword, then bind its parameters with
/// [`with_parameter()`](Self::with_parameter). SELECT statements, subqueries,
/// and user-defined functions are not supported.
/// Filters are limited to 256 lexical tokens. Use parameters or JSON
/// double-quoted strings for values containing escape sequences.
///
/// ```
/// use azure_data_cosmos_driver::read_many::ItemFilter;
/// let filter = "c.active = @active".parse::<ItemFilter>()?
///     .with_parameter("@active", true)?;
/// # Ok::<(), azure_data_cosmos_driver::CosmosError>(())
/// ```
#[derive(Clone, SafeDebug)]
pub struct ItemFilter {
    pub(crate) expression: SqlScalarExpression,
    pub(crate) parameters: BTreeMap<String, Value>,
}

impl FromStr for ItemFilter {
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

impl ItemFilter {
    /// Binds a serializable value to a parameter, including the leading `@`.
    ///
    /// Returns an error for duplicate names, names absent from the predicate, or serialization failures.
    /// All referenced parameters must be bound before planning the operation.
    pub fn with_parameter(
        mut self,
        name: impl Into<String>,
        value: impl Serialize,
    ) -> crate::Result<Self> {
        let value = serde_json::to_value(value)?;
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

pub(crate) type ReadManyRequest = ItemSelector;

pub(crate) fn invalid(message: &'static str) -> crate::CosmosError {
    crate::CosmosError::builder()
        .with_status(crate::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

#[cfg(test)]
mod tests {
    use super::ItemFilter;
    use serde::{Serialize, Serializer};
    use serde_json::json;

    #[test]
    fn parameters_accept_serializable_values_and_propagate_errors() {
        #[derive(Serialize)]
        struct Criteria {
            active: bool,
            tags: Vec<&'static str>,
        }
        struct Invalid;
        impl Serialize for Invalid {
            fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom(
                    "cannot serialize filter parameter",
                ))
            }
        }
        let filter = "c.active = @active AND c.criteria = @criteria"
            .parse::<ItemFilter>()
            .unwrap()
            .with_parameter("@active", true)
            .unwrap()
            .with_parameter(
                "@criteria",
                Criteria {
                    active: true,
                    tags: vec!["a", "b"],
                },
            )
            .unwrap();
        assert_eq!(filter.parameters["@active"], json!(true));
        assert_eq!(
            filter.parameters["@criteria"],
            json!({"active":true,"tags":["a","b"]})
        );
        let error = "c.id = @id"
            .parse::<ItemFilter>()
            .unwrap()
            .with_parameter("@id", Invalid)
            .unwrap_err();
        let expected = crate::CosmosError::from(serde_json::to_value(Invalid).unwrap_err());
        assert_eq!(error.status(), expected.status());
        assert_eq!(
            std::error::Error::source(&error).unwrap().to_string(),
            "cannot serialize filter parameter"
        );
    }

    #[test]
    fn predicates_preserve_precedence_and_parameter_names() {
        let filter = "c.rank = @a OR c.name = @b -- ignored\n"
            .parse::<ItemFilter>()
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
            "COUNT(c.id) > 0",
            "true GROUP BY c.id",
            "true OFFSET 0 LIMIT 1",
            "EXISTS(SELECT * FROM c)",
            "udf.test(c.id)",
            "true; SELECT * FROM c",
            "c.x = @x /* unterminated",
        ] {
            assert!(text.parse::<ItemFilter>().is_err(), "{text}");
        }
        assert!("c.x = @x"
            .parse::<ItemFilter>()
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
                    .parse::<ItemFilter>()
                    .unwrap()
                    .query_predicate()
                    .unwrap(),
                expected
            );
        }
    }
}
