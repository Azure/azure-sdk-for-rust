// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::collections::HashMap;

use serde::Deserialize;

/// Models the response returned by the Gateway when making a query plan request.
#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "python_conversions",
    derive(pyo3::FromPyObject),
    pyo3(from_item_all)
)]
#[serde(rename_all = "camelCase")]
pub struct QueryPlan {
    /// The version of the query plan.
    #[cfg_attr(
        feature = "python_conversions",
        pyo3(item("partitionedQueryExecutionInfoVersion"))
    )]
    pub partitioned_query_execution_info_version: usize,

    /// The query plan itself
    #[cfg_attr(feature = "python_conversions", pyo3(item("queryInfo")))]
    #[serde(default)]
    pub query_info: Option<QueryInfo>,

    /// The partition key ranges that this query references.
    ///
    /// These can be used by the pipeline to limit the partition key ranges that get queried.
    #[cfg_attr(feature = "python_conversions", pyo3(item("queryRanges")))]
    pub query_ranges: Vec<QueryRange>,

    /// Information about hybrid search queries, if applicable.
    pub hybrid_search_query_info: Option<HybridSearchQueryInfo>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(
    feature = "python_conversions",
    derive(pyo3::FromPyObject),
    pyo3(from_item_all)
)]
#[serde(rename_all = "camelCase")]
pub struct HybridSearchQueryInfo {
    /// Provides the query to be used for global statistics gathering.
    pub global_statistics_query: String,

    /// Provides the individual component queries that make up the hybrid search query.
    pub component_query_infos: Vec<QueryInfo>,

    /// The weights assigned to each component query, if any.
    #[serde(default)]
    pub component_weights: Vec<f64>,

    /// The number of results to skip.
    pub skip: Option<u64>,

    /// The number of results to take.
    ///
    /// This should always be present, because hybrid search queries require a TOP clause.
    pub take: Option<u64>,

    /// Indicates if global statistics are required for this query.
    pub requires_global_statistics: bool,
}

/// The kind of DISTINCT tracking required by the query.
#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
pub enum DistinctType {
    /// The query does not require deduplicating results.
    #[default]
    None,

    /// The query requires that order be considered when deduplicating results.
    Ordered,

    /// The query does not require that order be considered when deduplicating results.
    Unordered,
}

#[cfg(feature = "python_conversions")]
impl<'a> pyo3::FromPyObject<'a> for DistinctType {
    /// Converts a [`pyo3::PyAny`] value, which should represent a `str`, into a [`DistinctType`]
    fn extract_bound(ob: &pyo3::Bound<'a, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        use pyo3::types::PyAnyMethods;
        use pyo3::types::PyStringMethods;
        let ob = ob.downcast::<pyo3::types::PyString>()?;
        match ob.to_str()? {
            "None" => Ok(Self::None),
            "Ordered" => Ok(Self::Ordered),
            "Unordered" => Ok(Self::Unordered),
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "invalid DistinctType",
            )),
        }
    }
}

/// Models the query plan for a query.
#[derive(Debug, Deserialize, Default)]
#[cfg_attr(
    feature = "python_conversions",
    derive(pyo3::FromPyObject),
    pyo3(from_item_all)
)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct QueryInfo {
    /// Indicates if the query contains a DISTINCT clause
    #[cfg_attr(feature = "python_conversions", pyo3(item("distinctType")))]
    pub distinct_type: DistinctType,

    /// If this value is `Some`, the value provided represents a limit to the number of results that should be returned.
    ///
    /// This represents the `TOP` clause.
    #[cfg_attr(feature = "python_conversions", pyo3(default))]
    pub top: Option<u64>,

    /// If this value is `Some`, the value provided represents a number of results to skip before returning items.
    ///
    /// This represents the `OFFSET` clause (in an `OFFSET`/`LIMIT`).
    #[cfg_attr(feature = "python_conversions", pyo3(default))]
    pub offset: Option<u64>,

    /// If this value is `Some`, the value provided represents a limit to the number of results that should be returned.
    ///
    /// This represents the `LIMIT` clause (in an `OFFSET`/`LIMIT`).
    #[cfg_attr(feature = "python_conversions", pyo3(default))]
    pub limit: Option<u64>,

    /// Describes the sort orders used by `ORDER BY` clauses in the query.
    ///
    /// For example, for `SELECT * FROM c ORDER BY c.foo, c.bar DESC`, this would return:
    /// [`SortOrder::Ascending`], [`SortOrder::Descending`].
    #[cfg_attr(feature = "python_conversions", pyo3(item("orderBy"), default))]
    pub order_by: Vec<SortOrder>,
    #[cfg_attr(
        feature = "python_conversions",
        pyo3(item("orderByExpressions"), default)
    )]

    /// Describes the expressions used by `ORDER BY` clauses in the query.
    ///
    /// For example, for `SELECT * FROM c ORDER BY c.foo, c.bar DESC`, this would return:
    /// `c.foo` and `c.bar`.
    pub order_by_expressions: Vec<String>,
    #[cfg_attr(
        feature = "python_conversions",
        pyo3(item("groupByExpressions"), default)
    )]

    /// Describes the expressions used by the `GROUP BY` clauses in the query.
    pub group_by_expressions: Vec<String>,

    #[cfg_attr(feature = "python_conversions", pyo3(item("groupByAliases"), default))]
    pub group_by_aliases: Vec<String>,

    /// A list of the aggregates used in the `SELECT` portion of a `GROUP BY` query.
    #[cfg_attr(feature = "python_conversions", pyo3(default))]
    pub aggregates: Vec<String>,

    #[cfg_attr(
        feature = "python_conversions",
        pyo3(item("groupByAliasToAggregateType"), default)
    )]
    pub group_by_alias_to_aggregate_type: HashMap<String, String>,

    /// If this string has a non-zero length, then it contains a rewritten form of the query that
    /// should be used to make the individual single-partition queries.
    #[cfg_attr(feature = "python_conversions", pyo3(item("rewrittenQuery"), default))]
    pub rewritten_query: String,

    /// Indicates if the query contains a `SELECT VALUE` clause.
    #[cfg_attr(feature = "python_conversions", pyo3(item("hasSelectValue"), default))]
    pub has_select_value: bool,

    /// Indicates if the query contains a non-streaming `ORDER BY`.
    #[cfg_attr(
        feature = "python_conversions",
        pyo3(item("hasNonStreamingOrderBy"), default)
    )]
    pub has_non_streaming_order_by: bool,
}

/// The sort order used by a particular `ORDER BY` expression.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[cfg(feature = "python_conversions")]
impl<'a> pyo3::FromPyObject<'a> for SortOrder {
    /// Converts a [`pyo3::PyAny`] value, which should represent a `str`, into a [`SortOrder`]
    fn extract_bound(ob: &pyo3::Bound<'a, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        use pyo3::types::PyAnyMethods;
        use pyo3::types::PyStringMethods;
        let ob = ob.downcast::<pyo3::types::PyString>()?;
        match ob.to_str()? {
            "Ascending" => Ok(Self::Ascending),
            "Descending" => Ok(Self::Descending),
            _ => Err(pyo3::exceptions::PyValueError::new_err("invalid SortOrder")),
        }
    }
}

/// Describes a partition key range that is covered by the query.
#[derive(Debug, Deserialize)]
#[cfg_attr(
    feature = "python_conversions",
    derive(pyo3::FromPyObject),
    pyo3(from_item_all)
)]
#[serde(rename_all = "camelCase")]
pub struct QueryRange {
    /// The minimum hashed partition key value covered by the query.
    pub min: String,

    /// The maximum hashed partition key value covered by the query.
    pub max: String,

    /// A boolean indicating if the minimum value is inclusive. If false, it's exclusive.
    #[cfg_attr(feature = "python_conversions", pyo3(item("isMinInclusive")))]
    pub is_min_inclusive: bool,

    /// A boolean indicating if the maximum value is inclusive. If false, it's exclusive.
    #[cfg_attr(feature = "python_conversions", pyo3(item("isMaxInclusive")))]
    pub is_max_inclusive: bool,
}

impl QueryRange {
    /// Creates a new [`QueryRange`].
    pub fn new(
        min: impl Into<String>,
        max: impl Into<String>,
        is_min_inclusive: bool,
        is_max_inclusive: bool,
    ) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
            is_min_inclusive,
            is_max_inclusive,
        }
    }
}
