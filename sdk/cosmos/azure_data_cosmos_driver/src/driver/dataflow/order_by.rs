// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore rescan

//! Cross-partition streaming `ORDER BY` value model.
//!
//! Shared vocabulary between [`StreamingOrderedMerge`] and the
//! continuation-token snapshot (`super::snapshot`): [`OrderByItem`] parses
//! one column's envelope value (distinguishing `Undefined` from `null`);
//! [`compare_key_tuples`] is the shared Cosmos-ordered comparator;
//! [`OrderByResumeValue`] is the bounded "last emitted" value persisted in
//! a token (arrays/objects hashed to 128 bits so a token never grows with
//! document size); [`ResumeFilter`] builds the per-range seek predicate
//! used to skip already-emitted rows on re-query — scalar boundaries stay
//! correct across a split, complex ones fall back to
//! [`ResumeFilter::PositionalRescan`] (topology-unchanged only).
//!
//! # Type order
//!
//! Canonical Cosmos ascending type order:
//! `Undefined < Null < Boolean < Number < String < Array < Object`. Must
//! stay in sync with `crate::query::eval`'s `sort_type_order`.

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::models::murmur_hash::murmurhash3_128;

use super::query_plan::SortOrder;

/// One `ORDER BY` column: an expression addressing this column's value,
/// plus sort direction. Built from the query's source expressions (e.g.
/// `c.rank`), since the resume filter is injected into the rewritten
/// query and must reference original source columns, not envelope fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OrderByColumn {
    pub(crate) expression: String,
    pub(crate) direction: SortOrder,
}

/// A stable fingerprint of a rewritten query's text, used as a
/// continuation token's query-shape discriminator: resume with a
/// structurally different query is rejected, never conflated.
pub(crate) fn query_fingerprint(rewritten_query: &str) -> String {
    let hash = murmurhash3_128(rewritten_query.as_bytes(), 0);
    format!("{hash:032x}")
}

/// A single `ORDER BY` key value parsed from a rewritten envelope's
/// `orderByItems[i]`. Distinguishes `Undefined` (no `item` field) from
/// every JSON value, including `null` — load-bearing, since Cosmos ranks
/// `Undefined` before `Null`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum OrderByItem {
    Undefined,
    Null,
    Boolean(bool),
    Number(OrderByNumber),
    String(String),
    Array(Vec<OrderByItem>),
    /// Key/value pairs in wire order; comparison sorts by key internally
    /// so differently-ordered wire keys still compare equal.
    Object(Vec<(String, OrderByItem)>),
}

/// Ascending Cosmos type rank. Must match
/// `crate::query::eval::sort_type_order`.
fn type_rank(item: &OrderByItem) -> u8 {
    match item {
        OrderByItem::Undefined => 0,
        OrderByItem::Null => 1,
        OrderByItem::Boolean(_) => 2,
        OrderByItem::Number(_) => 3,
        OrderByItem::String(_) => 4,
        OrderByItem::Array(_) => 5,
        OrderByItem::Object(_) => 6,
    }
}

/// Total-ordering comparison for `f64` (`-0.0 == 0.0`, matching IEEE/Cosmos
/// numeric equality); falls back to `total_cmp` only for `NaN` so this
/// never panics (required for a strict-total-order comparator).
fn cmp_f64(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or_else(|| a.total_cmp(&b))
}

/// Compares `i64` with `u64` exactly, without a lossy cast: a negative
/// signed value is always smaller than any unsigned value, otherwise the
/// non-negative signed value widens to `u64` losslessly.
fn cmp_i64_u64(a: i64, b: u64) -> Ordering {
    if a < 0 {
        Ordering::Less
    } else {
        (a as u64).cmp(&b)
    }
}

/// Compares an `i64` with a finite `f64` exactly. `2^63` is exactly
/// representable, so it bounds the range where `floor` fits an `i64`; the
/// fractional part then breaks a floor-tie without any lossy cast.
fn cmp_i64_f64(a: i64, b: f64) -> Ordering {
    const TWO_POW_63: f64 = 9_223_372_036_854_775_808.0;
    if b >= TWO_POW_63 {
        return Ordering::Less; // a <= i64::MAX < 2^63 <= b
    }
    if b < -TWO_POW_63 {
        return Ordering::Greater; // a >= i64::MIN = -2^63 > b
    }
    let floor = b.floor(); // -2^63 <= floor < 2^63, fits i64 exactly
    match a.cmp(&(floor as i64)) {
        // a == floor(b) but b has a fractional part, so a < b.
        Ordering::Equal if b > floor => Ordering::Less,
        other => other,
    }
}

/// Compares a `u64` with a finite `f64` exactly (see [`cmp_i64_f64`];
/// `2^64` is exactly representable and bounds the `u64` range).
fn cmp_u64_f64(a: u64, b: f64) -> Ordering {
    const TWO_POW_64: f64 = 18_446_744_073_709_551_616.0;
    if b < 0.0 {
        return Ordering::Greater; // a >= 0 > b
    }
    if b >= TWO_POW_64 {
        return Ordering::Less; // a <= u64::MAX < 2^64 <= b
    }
    let floor = b.floor(); // 0 <= floor < 2^64, fits u64 exactly
    match a.cmp(&(floor as u64)) {
        Ordering::Equal if b > floor => Ordering::Less,
        other => other,
    }
}

/// A lossless `ORDER BY` numeric value. Cosmos/JSON numbers may be signed
/// integers, unsigned integers, or finite floats, and no single `f64`
/// represents every `i64`/`u64` exactly (e.g. `2^53 + 1`). Preserving the
/// original variant keeps comparison, resume-token round-trips, and SQL
/// literals mathematically exact.
///
/// Serialization is lossless: each variant emits its native JSON number
/// and re-parses back to the same variant.
#[derive(Debug, Clone, Copy)]
pub(crate) enum OrderByNumber {
    I64(i64),
    U64(u64),
    F64(f64),
}

impl OrderByNumber {
    /// Classifies a `serde_json::Number` into the widest lossless variant:
    /// signed integer, else unsigned integer, else finite float.
    fn from_json_number(n: &serde_json::Number) -> Self {
        if let Some(i) = n.as_i64() {
            Self::I64(i)
        } else if let Some(u) = n.as_u64() {
            Self::U64(u)
        } else {
            Self::F64(n.as_f64().unwrap_or(0.0))
        }
    }

    /// Mathematically-correct comparison across all variant pairs, with no
    /// lossy cast or overflow (`-0.0 == 0.0`, `5i64 == 5.0f64`).
    fn cosmos_cmp(&self, other: &Self) -> Ordering {
        use OrderByNumber::{F64, I64, U64};
        match (self, other) {
            (I64(a), I64(b)) => a.cmp(b),
            (U64(a), U64(b)) => a.cmp(b),
            (F64(a), F64(b)) => cmp_f64(*a, *b),
            (I64(a), U64(b)) => cmp_i64_u64(*a, *b),
            (U64(a), I64(b)) => cmp_i64_u64(*b, *a).reverse(),
            (I64(a), F64(b)) => cmp_i64_f64(*a, *b),
            (F64(a), I64(b)) => cmp_i64_f64(*b, *a).reverse(),
            (U64(a), F64(b)) => cmp_u64_f64(*a, *b),
            (F64(a), U64(b)) => cmp_u64_f64(*b, *a).reverse(),
        }
    }

    /// Renders the exact JSON value (integers stay integers) for canonical
    /// hashing of complex keys and for binding a scalar resume boundary as a
    /// query parameter.
    fn to_json_value(self) -> serde_json::Value {
        match self {
            Self::I64(i) => serde_json::Value::Number(i.into()),
            Self::U64(u) => serde_json::Value::Number(u.into()),
            Self::F64(f) => serde_json::Number::from_f64(f)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
        }
    }
}

/// Numeric equality is value-based across variants (`I64(5) == F64(5.0)`),
/// matching Cosmos, so representation never changes an equality result.
impl PartialEq for OrderByNumber {
    fn eq(&self, other: &Self) -> bool {
        self.cosmos_cmp(other) == Ordering::Equal
    }
}

// `Eq` soundness: in production an `OrderByNumber` is always finite, so the
// value-based `PartialEq` is a proper equivalence relation. The only
// production constructor is `from_json_number` (a `serde_json::Number` is
// always finite), and `deserialize` rejects non-finite floats below; the
// `From<f64>` convenience is `#[cfg(test)]`-only. Restricting to finite
// values keeps `==` reflexive, symmetric, and transitive (a `NaN` would
// otherwise break transitivity across variants, e.g. `I64(0) == F64(NaN)`).
impl Eq for OrderByNumber {}

impl From<i64> for OrderByNumber {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<u64> for OrderByNumber {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

// Test-only: production never builds an `OrderByNumber` from an arbitrary
// `f64` (that path is `from_json_number`, which is always finite), so
// keeping this out of production builds guarantees a non-finite value can
// never enter and undermine the `Eq` impl above.
#[cfg(test)]
impl From<f64> for OrderByNumber {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl serde::Serialize for OrderByNumber {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::I64(i) => serializer.serialize_i64(*i),
            Self::U64(u) => serializer.serialize_u64(*u),
            Self::F64(f) => serializer.serialize_f64(*f),
        }
    }
}

impl<'de> serde::Deserialize<'de> for OrderByNumber {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let number = serde_json::Number::deserialize(deserializer)?;
        let value = Self::from_json_number(&number);
        // A persisted resume value is always a finite JSON number. Reject
        // non-finite floats so the `Eq` equivalence stays airtight — they
        // can't arise from valid JSON but would otherwise be a silent hole.
        if let Self::F64(f) = value {
            if !f.is_finite() {
                return Err(serde::de::Error::custom(
                    "ORDER BY numeric resume value must be finite",
                ));
            }
        }
        Ok(value)
    }
}

impl OrderByItem {
    /// Converts a JSON value into an `OrderByItem`. Never produces
    /// `Undefined` — only the wire-envelope parser does, for a missing
    /// `item` key.
    pub(crate) fn from_json(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Bool(b) => Self::Boolean(*b),
            serde_json::Value::Number(n) => Self::Number(OrderByNumber::from_json_number(n)),
            serde_json::Value::String(s) => Self::String(s.clone()),
            serde_json::Value::Array(items) => {
                Self::Array(items.iter().map(Self::from_json).collect())
            }
            serde_json::Value::Object(map) => Self::Object(
                map.iter()
                    .map(|(k, v)| (k.clone(), Self::from_json(v)))
                    .collect(),
            ),
        }
    }

    /// Cosmos-compatible ascending comparison: cross-type via [`type_rank`];
    /// arrays compare lexicographically; objects by key, value, then
    /// length. No canonical intra-type order is documented for
    /// arrays/objects — only internal determinism is promised.
    pub(crate) fn cosmos_cmp(&self, other: &Self) -> Ordering {
        let rank_cmp = type_rank(self).cmp(&type_rank(other));
        if rank_cmp != Ordering::Equal {
            return rank_cmp;
        }
        match (self, other) {
            (Self::Undefined, Self::Undefined) => Ordering::Equal,
            (Self::Null, Self::Null) => Ordering::Equal,
            (Self::Boolean(a), Self::Boolean(b)) => a.cmp(b),
            (Self::Number(a), Self::Number(b)) => a.cosmos_cmp(b),
            (Self::String(a), Self::String(b)) => a.cmp(b),
            (Self::Array(a), Self::Array(b)) => {
                for (x, y) in a.iter().zip(b.iter()) {
                    let c = x.cosmos_cmp(y);
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                a.len().cmp(&b.len())
            }
            (Self::Object(a), Self::Object(b)) => {
                let mut a_sorted: Vec<&(String, OrderByItem)> = a.iter().collect();
                let mut b_sorted: Vec<&(String, OrderByItem)> = b.iter().collect();
                a_sorted.sort_by(|x, y| x.0.cmp(&y.0));
                b_sorted.sort_by(|x, y| x.0.cmp(&y.0));
                for (x, y) in a_sorted.iter().zip(b_sorted.iter()) {
                    let key_cmp = x.0.cmp(&y.0);
                    if key_cmp != Ordering::Equal {
                        return key_cmp;
                    }
                    let val_cmp = x.1.cosmos_cmp(&y.1);
                    if val_cmp != Ordering::Equal {
                        return val_cmp;
                    }
                }
                a_sorted.len().cmp(&b_sorted.len())
            }
            _ => unreachable!("rank_cmp already distinguished differing variants"),
        }
    }

    /// Converts this item to its bounded, serializable resume-value form.
    pub(crate) fn to_resume_value(&self) -> OrderByResumeValue {
        match self {
            Self::Undefined => OrderByResumeValue::Undefined,
            Self::Null => OrderByResumeValue::Null,
            Self::Boolean(b) => OrderByResumeValue::Boolean { value: *b },
            Self::Number(n) => OrderByResumeValue::Number { value: *n },
            Self::String(s) => OrderByResumeValue::String { value: s.clone() },
            Self::Array(_) => OrderByResumeValue::Complex {
                complex_type: ComplexTypeTag::Array,
                hash: ComplexHash::of(self),
            },
            Self::Object(_) => OrderByResumeValue::Complex {
                complex_type: ComplexTypeTag::Object,
                hash: ComplexHash::of(self),
            },
        }
    }
}

/// Parses a rewritten envelope's `orderByItems` array into one
/// [`OrderByItem`] per column, validating the wire shape: must be a JSON
/// array of length `expected_len`, each element a JSON object; a missing
/// `item` key parses as [`OrderByItem::Undefined`].
///
/// Returns a typed [`crate::error::CosmosError`] (not a panic) for any
/// shape violation.
pub(crate) fn parse_order_by_items(
    value: &serde_json::Value,
    expected_len: usize,
) -> crate::error::Result<Vec<OrderByItem>> {
    let elements = value.as_array().ok_or_else(|| {
        envelope_error(format!(
            "rewritten envelope `orderByItems` must be a JSON array, found {}",
            json_type_name(value)
        ))
    })?;
    if elements.len() != expected_len {
        return Err(envelope_error(format!(
            "rewritten envelope `orderByItems` has {} entries but the query defines {expected_len} ORDER BY column(s)",
            elements.len()
        )));
    }
    elements
        .iter()
        .map(|element| {
            let obj = element.as_object().ok_or_else(|| {
                envelope_error(format!(
                    "rewritten envelope `orderByItems` entry must be a JSON object, found {}",
                    json_type_name(element)
                ))
            })?;
            Ok(match obj.get("item") {
                Some(item) => OrderByItem::from_json(item),
                None => OrderByItem::Undefined,
            })
        })
        .collect()
}

fn json_type_name(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

fn envelope_error(message: impl Into<std::borrow::Cow<'static, str>>) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID)
        .with_message(message)
        .build()
}

/// Compares two same-length key tuples column-by-column, applying each
/// column's direction, stopping at the first non-equal column. Panics on
/// a length mismatch — callers must validate column-count agreement first.
pub(crate) fn compare_key_tuples(
    a: &[OrderByItem],
    b: &[OrderByItem],
    directions: &[SortOrder],
) -> Ordering {
    debug_assert_eq!(a.len(), directions.len());
    debug_assert_eq!(b.len(), directions.len());
    for (i, direction) in directions.iter().enumerate() {
        let c = a[i].cosmos_cmp(&b[i]);
        let c = match direction {
            SortOrder::Ascending => c,
            SortOrder::Descending => c.reverse(),
        };
        if c != Ordering::Equal {
            return c;
        }
    }
    Ordering::Equal
}

/// Compares two document `_rid`s in `direction`'s Cosmos document order —
/// the order the backend breaks full-key `ORDER BY` ties in within a
/// partition. Ascending uses numeric document-ordinal order (see
/// [`crate::models::resource_id::compare_document_rids`]); descending
/// reverses it. The direction is the query's first sort column, matching
/// .NET's `ReverseRidEnabled` fallback (no `reverseIndexScan` signal is
/// available in the Gateway query plan the driver consumes).
pub(crate) fn compare_rids(a: &str, b: &str, direction: SortOrder) -> Ordering {
    let ascending = crate::models::resource_id::compare_document_rids(a, b);
    match direction {
        SortOrder::Ascending => ascending,
        SortOrder::Descending => ascending.reverse(),
    }
}

/// Discriminates which complex JSON shape a hashed resume value came from,
/// so two colliding hashes from different shapes are never a tie.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ComplexTypeTag {
    Array,
    Object,
}

/// A bounded 128-bit hash of a complex (array/object) `ORDER BY` value,
/// split into low/high halves for JSON round-tripping (mirrors .NET's
/// complex-key encoding); hashes a canonical encoding so key order doesn't
/// affect the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ComplexHash {
    pub(crate) low64: u64,
    pub(crate) high64: u64,
}

impl ComplexHash {
    fn of(item: &OrderByItem) -> Self {
        let canonical = canonical_json(item);
        let bytes = serde_json::to_vec(&canonical)
            .expect("canonical_json output is always representable as JSON");
        let hash = murmurhash3_128(&bytes, 0);
        Self {
            low64: hash as u64,
            high64: (hash >> 64) as u64,
        }
    }
}

/// Renders an `OrderByItem` as a canonical JSON value for hashing
/// (sorted object keys) so structurally-equal values hash identically.
fn canonical_json(item: &OrderByItem) -> serde_json::Value {
    match item {
        OrderByItem::Undefined => serde_json::Value::Null, // unreachable for array/object elements
        OrderByItem::Null => serde_json::Value::Null,
        OrderByItem::Boolean(b) => serde_json::Value::Bool(*b),
        OrderByItem::Number(n) => n.to_json_value(),
        OrderByItem::String(s) => serde_json::Value::String(s.clone()),
        OrderByItem::Array(items) => {
            serde_json::Value::Array(items.iter().map(canonical_json).collect())
        }
        OrderByItem::Object(props) => {
            let sorted: std::collections::BTreeMap<&str, serde_json::Value> = props
                .iter()
                .map(|(k, v)| (k.as_str(), canonical_json(v)))
                .collect();
            serde_json::Value::Object(sorted.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
        }
    }
}

/// The bounded, serializable representation of one column's "last emitted"
/// value, persisted in a continuation token. Scalars round-trip exactly;
/// arrays/objects are represented only by their [`ComplexHash`], so a
/// token never grows with document size.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum OrderByResumeValue {
    Undefined,
    Null,
    Boolean {
        value: bool,
    },
    Number {
        value: OrderByNumber,
    },
    String {
        value: String,
    },
    Complex {
        complex_type: ComplexTypeTag,
        hash: ComplexHash,
    },
}

impl OrderByResumeValue {
    pub(crate) fn is_complex(&self) -> bool {
        matches!(self, Self::Complex { .. })
    }

    /// Reconstructs the [`OrderByItem`] this came from, or `None` for
    /// [`Self::Complex`] (bytes aren't recoverable from the hash).
    pub(crate) fn to_scalar_order_by_item(&self) -> Option<OrderByItem> {
        match self {
            Self::Undefined => Some(OrderByItem::Undefined),
            Self::Null => Some(OrderByItem::Null),
            Self::Boolean { value } => Some(OrderByItem::Boolean(*value)),
            Self::Number { value } => Some(OrderByItem::Number(*value)),
            Self::String { value } => Some(OrderByItem::String(value.clone())),
            Self::Complex { .. } => None,
        }
    }

    /// The JSON value to bind when this resume value is parameterized, or
    /// `None` for `Undefined`/`Null` (rendered as type-check builtins) and
    /// `Complex` (no scalar value). Integer variants keep exact i64/u64
    /// precision so a boundary never loses precision through the query body.
    fn to_parameter_value(&self) -> Option<serde_json::Value> {
        match self {
            Self::Boolean { value } => Some(serde_json::Value::Bool(*value)),
            Self::Number { value } => Some(value.to_json_value()),
            Self::String { value } => Some(serde_json::Value::String(value.clone())),
            Self::Undefined | Self::Null | Self::Complex { .. } => None,
        }
    }

    /// Ascending Cosmos type rank, matching [`type_rank`]. Used to
    /// validate a resume value's `Complex::complex_type` and to build the
    /// cross-type resume filter.
    fn type_rank(&self) -> u8 {
        match self {
            Self::Undefined => 0,
            Self::Null => 1,
            Self::Boolean { .. } => 2,
            Self::Number { .. } => 3,
            Self::String { .. } => 4,
            Self::Complex { complex_type, .. } => match complex_type {
                ComplexTypeTag::Array => 5,
                ComplexTypeTag::Object => 6,
            },
        }
    }
}

/// One of the seven Cosmos type-rank buckets, used to render `IS_*`
/// builtin guards for cross-type-aware resume filters.
fn type_check_builtin(rank: u8) -> &'static str {
    match rank {
        0 => "NOT IS_DEFINED",
        1 => "IS_NULL",
        2 => "IS_BOOLEAN",
        3 => "IS_NUMBER",
        4 => "IS_STRING",
        5 => "IS_ARRAY",
        6 => "IS_OBJECT",
        _ => unreachable!("rank is always 0..=6"),
    }
}

/// One `@name`/value binding the resume filter contributes to the query
/// body's `parameters` array. A scalar boundary value is bound as a
/// parameter — never interpolated as SQL text — so the service parses and
/// escapes it and integer precision is preserved through the JSON value.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResumeParameter {
    /// Parameter name including the leading `@`.
    pub(crate) name: String,
    pub(crate) value: serde_json::Value,
}

/// A parameter-name prefix that no existing query parameter starts with, so
/// the generated names (`<prefix>0`, `<prefix>1`, …) can never equal — and
/// thus never overwrite — a caller's binding. Deterministic: the same set of
/// existing names always yields the same prefix.
fn collision_free_prefix(existing_parameter_names: &[String]) -> String {
    let mut prefix = String::from("@cosmosResumeFilter");
    while existing_parameter_names
        .iter()
        .any(|name| name.starts_with(&prefix))
    {
        prefix.push('_');
    }
    prefix
}

/// Renders `"<expr> == <boundary>"`. `Undefined`/`Null` need only a
/// type-check builtin; every other type also compares `expr` against the
/// bound parameter `param` — never an inline literal, so no boundary text
/// ever reaches the query SQL.
fn equals_boundary(expression: &str, value: &OrderByResumeValue, param: Option<&str>) -> String {
    match value {
        OrderByResumeValue::Undefined => format!("(NOT IS_DEFINED({expression}))"),
        OrderByResumeValue::Null => format!("IS_NULL({expression})"),
        _ => format!(
            "({check}({expression}) AND {expression} = {param})",
            check = type_check_builtin(value.type_rank()),
            param = param.expect("a scalar boundary value is always bound to a parameter"),
        ),
    }
}

/// Renders `"<expr> is strictly after <boundary>"` in `direction`, or
/// `None` if there's no possible "after" (boundary is the extreme value).
/// Any value comparison references the bound parameter `param`, never a
/// literal.
fn strictly_after_boundary(
    expression: &str,
    value: &OrderByResumeValue,
    direction: SortOrder,
    param: Option<&str>,
) -> Option<String> {
    let rank = value.type_rank();
    let higher_ranks: Vec<u8> = match direction {
        SortOrder::Ascending => ((rank + 1)..=6).collect(),
        SortOrder::Descending => (0..rank).collect(),
    };
    let mut terms: Vec<String> = higher_ranks
        .into_iter()
        .map(|r| format!("{}({expression})", type_check_builtin(r)))
        .collect();
    if !matches!(
        value,
        OrderByResumeValue::Undefined | OrderByResumeValue::Null
    ) {
        let op = match direction {
            SortOrder::Ascending => ">",
            SortOrder::Descending => "<",
        };
        terms.push(format!(
            "({check}({expression}) AND {expression} {op} {param})",
            check = type_check_builtin(rank),
            param = param.expect("a scalar boundary value is always bound to a parameter"),
        ));
    }
    if terms.is_empty() {
        None
    } else {
        Some(format!("({})", terms.join(" OR ")))
    }
}

/// The result of attempting to build a value-based resume filter for one
/// range's `ORDER BY` boundary.
pub(crate) enum ResumeFilter {
    /// A `WHERE`-clause fragment returning every row at or after the resume
    /// boundary *key* (a per-row predicate that stays correct across a
    /// split), plus the `parameters` its `@name` placeholders bind. Scalar
    /// boundary values are always bound as parameters, never interpolated
    /// as SQL text, so the service parses/escapes them and no boundary
    /// string ever reaches the query SQL.
    ///
    /// The full-tie `_rid` tie-break is deliberately **not** in the SQL:
    /// `c._rid` compares as an ordinal base64 string in Cosmos SQL, which
    /// does not match the backend's numeric document-id ordering, so a SQL
    /// `_rid` bound could drop not-yet-emitted rows. The exact rid cut-off
    /// is applied client-side by [`super::streaming_ordered_merge`]'s
    /// numeric discard, matching .NET's `FilterNextAsync`.
    Exact {
        where_fragment: String,
        parameters: Vec<ResumeParameter>,
    },
    /// At least one column's value is [`OrderByResumeValue::Complex`],
    /// which can't be reconstructed from its bounded hash. The caller must
    /// reissue the query unfiltered and discard the first `rows_emitted`
    /// rows positionally — correct only while topology is unchanged (see
    /// [`super::streaming_ordered_merge::build_children`]). A row *count*
    /// is not a value-based cursor, so concurrent writes to the range
    /// between the original and resumed query can also silently shift what
    /// lands at a given position; this is a known, undetectable limitation.
    PositionalRescan,
}

impl ResumeFilter {
    /// Builds the resume filter for a range whose last-emitted key tuple is
    /// `resume_values`. `columns` are the query's source expressions, so the
    /// returned [`ResumeFilter::Exact`] fragment substitutes into the
    /// Gateway's placeholder (see
    /// [`super::query_response::rewritten_query_with_resume_filter`]) and its
    /// `parameters` append to the query body (see
    /// [`super::query_response::rewrite_query_body_with_parameters`]).
    ///
    /// Each column carrying a bound value (Boolean/Number/String) gets one
    /// parameter, reused across every equality and strict-after term for
    /// that column; `Undefined`/`Null` render as type-check builtins and
    /// need none. `existing_parameter_names` (the caller's query-body
    /// parameter names, `@`-prefixed) seed a collision-free prefix so a
    /// resume binding can never overwrite a caller's.
    ///
    /// The fragment returns all rows whose key is at or after the boundary
    /// (strictly-after on any prefix column, plus a full-key-equality
    /// disjunct). It carries no `_rid` predicate — the rid tie-break is
    /// numeric and applied client-side (see [`ResumeFilter::Exact`]).
    pub(crate) fn build(
        columns: &[OrderByColumn],
        resume_values: &[OrderByResumeValue],
        existing_parameter_names: &[String],
    ) -> Self {
        debug_assert_eq!(columns.len(), resume_values.len());
        if resume_values.iter().any(OrderByResumeValue::is_complex) {
            return Self::PositionalRescan;
        }

        // One collision-safe parameter per column carrying a bound value;
        // `Undefined`/`Null` need none. Names share a prefix no caller
        // parameter starts with, so a resume binding can't overwrite one.
        let prefix = collision_free_prefix(existing_parameter_names);
        let mut parameters: Vec<ResumeParameter> = Vec::new();
        let param_names: Vec<Option<String>> = resume_values
            .iter()
            .enumerate()
            .map(|(k, value)| {
                value.to_parameter_value().map(|json| {
                    let name = format!("{prefix}{k}");
                    parameters.push(ResumeParameter {
                        name: name.clone(),
                        value: json,
                    });
                    name
                })
            })
            .collect();

        let mut or_terms: Vec<String> = Vec::with_capacity(columns.len() + 1);
        for k in 0..columns.len() {
            let mut and_terms: Vec<String> = Vec::with_capacity(k + 1);
            for i in 0..k {
                and_terms.push(equals_boundary(
                    &columns[i].expression,
                    &resume_values[i],
                    param_names[i].as_deref(),
                ));
            }
            if let Some(after) = strictly_after_boundary(
                &columns[k].expression,
                &resume_values[k],
                columns[k].direction,
                param_names[k].as_deref(),
            ) {
                and_terms.push(after);
                or_terms.push(format!("({})", and_terms.join(" AND ")));
            }
            // A `None` here means this column can never be the first point
            // of difference after the boundary; correctly contributes nothing.
        }
        // Exact tie on every key column returns the whole tie run; the
        // client-side numeric `_rid` discard then drops the already-emitted
        // prefix (a SQL `_rid` bound would be string-ordered, not numeric).
        let full_tie: Vec<String> = columns
            .iter()
            .zip(resume_values.iter())
            .enumerate()
            .map(|(k, (column, value))| {
                equals_boundary(&column.expression, value, param_names[k].as_deref())
            })
            .collect();
        or_terms.push(format!("({})", full_tie.join(" AND ")));

        Self::Exact {
            where_fragment: format!("({})", or_terms.join(" OR ")),
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asc(expression: &str) -> OrderByColumn {
        OrderByColumn {
            expression: expression.to_owned(),
            direction: SortOrder::Ascending,
        }
    }

    fn desc(expression: &str) -> OrderByColumn {
        OrderByColumn {
            expression: expression.to_owned(),
            direction: SortOrder::Descending,
        }
    }

    // ── Type order ───────────────────────────────────────────────────────

    #[test]
    fn ascending_type_order_matches_cosmos_canonical_order() {
        let items = [
            OrderByItem::Undefined,
            OrderByItem::Null,
            OrderByItem::Boolean(false),
            OrderByItem::Number(0.0.into()),
            OrderByItem::String(String::new()),
            OrderByItem::Array(vec![]),
            OrderByItem::Object(vec![]),
        ];
        for i in 0..items.len() {
            for j in 0..items.len() {
                let expected = i.cmp(&j);
                assert_eq!(items[i].cosmos_cmp(&items[j]), expected, "index {i} vs {j}");
            }
        }
    }

    #[test]
    fn undefined_sorts_before_null() {
        assert_eq!(
            OrderByItem::Undefined.cosmos_cmp(&OrderByItem::Null),
            Ordering::Less
        );
        assert_eq!(
            OrderByItem::Null.cosmos_cmp(&OrderByItem::Undefined),
            Ordering::Greater
        );
    }

    #[test]
    fn numbers_compare_numerically() {
        assert_eq!(
            OrderByItem::Number(1.0.into()).cosmos_cmp(&OrderByItem::Number(2.0.into())),
            Ordering::Less
        );
        assert_eq!(
            OrderByItem::Number((-0.0).into()).cosmos_cmp(&OrderByItem::Number(0.0.into())),
            Ordering::Equal
        );
    }

    #[test]
    fn strings_compare_lexicographically() {
        assert_eq!(
            OrderByItem::String("a".into()).cosmos_cmp(&OrderByItem::String("b".into())),
            Ordering::Less
        );
    }

    #[test]
    fn arrays_compare_lexicographically_with_prefix_shorter_first() {
        let a = OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]);
        let b = OrderByItem::Array(vec![
            OrderByItem::Number(1.0.into()),
            OrderByItem::Number(2.0.into()),
        ]);
        assert_eq!(a.cosmos_cmp(&b), Ordering::Less);

        let c = OrderByItem::Array(vec![
            OrderByItem::Number(1.0.into()),
            OrderByItem::Number(1.0.into()),
        ]);
        let d = OrderByItem::Array(vec![
            OrderByItem::Number(1.0.into()),
            OrderByItem::Number(2.0.into()),
        ]);
        assert_eq!(c.cosmos_cmp(&d), Ordering::Less);
    }

    #[test]
    fn objects_compare_by_sorted_key_regardless_of_wire_order() {
        let a = OrderByItem::Object(vec![
            ("b".to_owned(), OrderByItem::Number(1.0.into())),
            ("a".to_owned(), OrderByItem::Number(2.0.into())),
        ]);
        let b = OrderByItem::Object(vec![
            ("a".to_owned(), OrderByItem::Number(2.0.into())),
            ("b".to_owned(), OrderByItem::Number(1.0.into())),
        ]);
        assert_eq!(
            a.cosmos_cmp(&b),
            Ordering::Equal,
            "same content, different wire key order"
        );

        let c = OrderByItem::Object(vec![("a".to_owned(), OrderByItem::Number(3.0.into()))]);
        assert_eq!(
            a.cosmos_cmp(&c),
            Ordering::Less,
            "sorted-key comparison first differs on key 'a': 2 < 3"
        );
    }

    // ── OrderByNumber: lossless cross-variant comparison ────────────────

    #[test]
    fn adjacent_two_pow_53_integers_remain_distinct_and_ordered() {
        // 2^53 is the largest integer exactly representable in `f64`;
        // 2^53 + 1 is not (it would round to 2^53 or 2^53 + 2). A
        // comparator that ever cast through `f64` would collapse these two
        // values; `OrderByNumber` must keep them distinct and ordered.
        let a = OrderByNumber::from(9_007_199_254_740_992_i64); // 2^53
        let b = OrderByNumber::from(9_007_199_254_740_993_i64); // 2^53 + 1
        assert_ne!(a, b);
        assert_eq!(a.cosmos_cmp(&b), Ordering::Less);
        assert_eq!(b.cosmos_cmp(&a), Ordering::Greater);

        let ua = OrderByNumber::from(9_007_199_254_740_992_u64);
        let ub = OrderByNumber::from(9_007_199_254_740_993_u64);
        assert_ne!(ua, ub);
        assert_eq!(ua.cosmos_cmp(&ub), Ordering::Less);
    }

    #[test]
    fn i64_and_u64_extremes_compare_correctly() {
        let i_min = OrderByNumber::from(i64::MIN);
        let i_max = OrderByNumber::from(i64::MAX);
        let u_max = OrderByNumber::from(u64::MAX);
        let u_zero = OrderByNumber::from(0_u64);

        assert_eq!(
            i_min.cosmos_cmp(&u_zero),
            Ordering::Less,
            "any negative i64 is less than any u64"
        );
        assert_eq!(i_min.cosmos_cmp(&i_max), Ordering::Less);
        assert_eq!(
            i_max.cosmos_cmp(&u_max),
            Ordering::Less,
            "i64::MAX < u64::MAX"
        );
        assert_eq!(u_max.cosmos_cmp(&i_max), Ordering::Greater);
        assert_eq!(u_max.cosmos_cmp(&u_max), Ordering::Equal);
        assert_eq!(i_min.cosmos_cmp(&i_min), Ordering::Equal);
        // Same numeric value across variants widens losslessly to equal.
        assert_eq!(
            OrderByNumber::from(i64::MAX).cosmos_cmp(&OrderByNumber::from(i64::MAX as u64)),
            Ordering::Equal
        );
    }

    #[test]
    fn cross_int_float_equality_and_order() {
        assert_eq!(OrderByNumber::from(5_i64), OrderByNumber::from(5.0_f64));
        assert_eq!(OrderByNumber::from(5_u64), OrderByNumber::from(5.0_f64));
        assert_eq!(
            OrderByNumber::from(5_i64).cosmos_cmp(&OrderByNumber::from(5.5_f64)),
            Ordering::Less
        );
        assert_eq!(
            OrderByNumber::from(5_i64).cosmos_cmp(&OrderByNumber::from(4.5_f64)),
            Ordering::Greater
        );

        // `2^63`/`2^64` are the exact boundaries `cmp_i64_f64`/`cmp_u64_f64`
        // branch on; `1e300` is far enough past either to be unambiguous
        // (unlike `-2^63 - 1.0`, which rounds right back to `-2^63` at that
        // magnitude's `f64` precision).
        const TWO_POW_63: f64 = 9_223_372_036_854_775_808.0;
        const TWO_POW_64: f64 = 18_446_744_073_709_551_616.0;
        assert_eq!(
            OrderByNumber::from(i64::MAX).cosmos_cmp(&OrderByNumber::from(TWO_POW_63)),
            Ordering::Less
        );
        assert_eq!(
            OrderByNumber::from(i64::MIN).cosmos_cmp(&OrderByNumber::from(-TWO_POW_63)),
            Ordering::Equal
        );
        assert_eq!(
            OrderByNumber::from(i64::MIN).cosmos_cmp(&OrderByNumber::from(-1e300_f64)),
            Ordering::Greater
        );
        assert_eq!(
            OrderByNumber::from(u64::MAX).cosmos_cmp(&OrderByNumber::from(TWO_POW_64)),
            Ordering::Less
        );
        assert_eq!(
            OrderByNumber::from(u64::MAX).cosmos_cmp(&OrderByNumber::from(1e300_f64)),
            Ordering::Less
        );
        assert_eq!(
            OrderByNumber::from(0_u64).cosmos_cmp(&OrderByNumber::from(-1.0_f64)),
            Ordering::Greater
        );
    }

    #[test]
    fn scalar_number_parameter_value_preserves_exact_integer_precision() {
        // The resume boundary binds the number as a JSON parameter value;
        // integers must keep full precision (never degrade through f64).
        let param = |n: OrderByNumber| {
            OrderByResumeValue::Number { value: n }
                .to_parameter_value()
                .unwrap()
        };
        assert_eq!(
            param(OrderByNumber::from(i64::MIN)),
            serde_json::json!(i64::MIN)
        );
        assert_eq!(
            param(OrderByNumber::from(u64::MAX)),
            serde_json::json!(u64::MAX)
        );
        assert_eq!(
            serde_json::to_string(&param(OrderByNumber::from(9_007_199_254_740_993_i64))).unwrap(),
            "9007199254740993",
            "must not degrade to float notation and lose precision"
        );
        assert_eq!(
            param(OrderByNumber::from(5.5_f64)),
            serde_json::json!(5.5_f64)
        );
    }

    #[test]
    fn deserialize_rejects_non_finite_numbers() {
        // JSON has no NaN/Inf, so a non-finite float can never round-trip
        // into an `OrderByNumber` — the `Eq` soundness contract. Overflowing
        // exponents that a parser might coerce to infinity are rejected too.
        for json in [
            r#"{"type":"number","value":1e400}"#,
            r#"{"type":"number","value":-1e400}"#,
        ] {
            assert!(
                serde_json::from_str::<OrderByResumeValue>(json).is_err(),
                "non-finite resume value {json} must be rejected on deserialize"
            );
        }
        // Finite values still round-trip, and `Eq` holds on the result.
        let value: OrderByResumeValue =
            serde_json::from_str(r#"{"type":"number","value":5.5}"#).unwrap();
        assert_eq!(value, OrderByResumeValue::Number { value: 5.5.into() });
    }

    #[test]
    fn large_integer_resume_values_round_trip_through_json_exactly() {
        for number in [
            OrderByNumber::from(i64::MIN),
            OrderByNumber::from(i64::MAX),
            OrderByNumber::from(u64::MAX),
            OrderByNumber::from(9_007_199_254_740_993_i64), // 2^53 + 1
        ] {
            let item = OrderByItem::Number(number);
            let resume = item.to_resume_value();
            let json = serde_json::to_string(&resume).unwrap();
            let back: OrderByResumeValue = serde_json::from_str(&json).unwrap();
            assert_eq!(
                back, resume,
                "token round-trip must preserve the exact value for {number:?}"
            );
            assert_eq!(
                back.to_scalar_order_by_item(),
                Some(OrderByItem::Number(number)),
                "must reconstruct the exact numeric variant, not a lossy float, for {number:?}"
            );
            // `OrderByNumber`'s `PartialEq` is intentionally value-based
            // (`I64(5) == F64(5.0)`), so for a value like `i64::MIN` that
            // also has an exact `f64` representation, the assertions above
            // alone can't catch a regression that silently degrades the
            // variant to `F64`. Comparing `Debug` output (which prints the
            // variant tag) closes that gap.
            assert_eq!(
                format!("{back:?}"),
                format!("{resume:?}"),
                "round-trip must preserve the exact variant (I64/U64/F64), not just the \
                 numeric value, for {number:?}"
            );
        }
    }

    #[test]
    fn complex_hash_distinguishes_adjacent_large_integers() {
        // If the hash ever cast through `f64`, these two would collide
        // (both round to the same nearest-representable float).
        let a = OrderByItem::Array(vec![OrderByItem::Number(9_007_199_254_740_992_i64.into())]);
        let b = OrderByItem::Array(vec![OrderByItem::Number(9_007_199_254_740_993_i64.into())]);
        assert_ne!(a.to_resume_value(), b.to_resume_value());

        // Same at the very top of the `u64` range.
        let c = OrderByItem::Object(vec![(
            "n".to_owned(),
            OrderByItem::Number((u64::MAX - 1).into()),
        )]);
        let d = OrderByItem::Object(vec![("n".to_owned(), OrderByItem::Number(u64::MAX.into()))]);
        assert_ne!(c.to_resume_value(), d.to_resume_value());
    }

    // ── Envelope parsing ─────────────────────────────────────────────────

    #[test]
    fn parses_defined_and_undefined_items() {
        let value = serde_json::json!([{"item": 1}, {}, {"item": null}]);
        let items = parse_order_by_items(&value, 3).unwrap();
        assert_eq!(
            items,
            vec![
                OrderByItem::Number(1.0.into()),
                OrderByItem::Undefined,
                OrderByItem::Null,
            ]
        );
    }

    #[test]
    fn rejects_wrong_length() {
        let value = serde_json::json!([{"item": 1}]);
        let err = parse_order_by_items(&value, 2).unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[test]
    fn rejects_non_array() {
        let value = serde_json::json!({"item": 1});
        let err = parse_order_by_items(&value, 1).unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[test]
    fn rejects_non_object_element() {
        let value = serde_json::json!([1]);
        let err = parse_order_by_items(&value, 1).unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    // ── Multi-column comparator ──────────────────────────────────────────

    #[test]
    fn multi_column_mixed_direction() {
        let directions = vec![SortOrder::Ascending, SortOrder::Descending];
        let a = vec![
            OrderByItem::Number(1.0.into()),
            OrderByItem::String("b".into()),
        ];
        let b = vec![
            OrderByItem::Number(1.0.into()),
            OrderByItem::String("a".into()),
        ];
        // Column 0 ties; column 1 DESC means "b" sorts before "a".
        assert_eq!(compare_key_tuples(&a, &b, &directions), Ordering::Less);
    }

    // ── Resume value round trip ──────────────────────────────────────────

    #[test]
    fn scalar_resume_values_round_trip_through_json() {
        for item in [
            OrderByItem::Undefined,
            OrderByItem::Null,
            OrderByItem::Boolean(true),
            OrderByItem::Number(5.5.into()),
            OrderByItem::String("s".into()),
        ] {
            let resume = item.to_resume_value();
            let json = serde_json::to_string(&resume).unwrap();
            let back: OrderByResumeValue = serde_json::from_str(&json).unwrap();
            assert_eq!(back, resume);
            // A scalar resume value reconstructs to the exact item it came
            // from (the basis of the `_rid`-aware client-side discard).
            assert_eq!(resume.to_scalar_order_by_item(), Some(item));
        }
    }

    #[test]
    fn complex_resume_values_hash_deterministically_and_distinguish_shape() {
        let array = OrderByItem::Array(vec![
            OrderByItem::Number(1.0.into()),
            OrderByItem::Number(2.0.into()),
        ]);
        let object = OrderByItem::Object(vec![("x".to_owned(), OrderByItem::Number(1.0.into()))]);

        let array_resume = array.to_resume_value();
        assert!(array_resume.is_complex());
        // An array and an object never share a resume value even if their
        // hashes were to collide: the shape tag differs.
        assert_ne!(array_resume, object.to_resume_value());

        // Same content, different wire key order -> identical resume value
        // (structurally-equal objects hash identically).
        let reordered_object_source = serde_json::json!({"x": 1.0});
        let reordered = OrderByItem::from_json(&reordered_object_source);
        assert_eq!(reordered.to_resume_value(), object.to_resume_value());
    }

    #[test]
    fn distinct_arrays_do_not_share_a_resume_value() {
        let a = OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]);
        let b = OrderByItem::Array(vec![OrderByItem::Number(2.0.into())]);
        assert_ne!(a.to_resume_value(), b.to_resume_value());
    }

    // ── Resume filter ────────────────────────────────────────────────────

    #[test]
    fn single_column_ascending_filter_is_strict_greater_than_with_full_tie_fallback() {
        let columns = vec![asc("c.rank")];
        let resume = vec![OrderByResumeValue::Number { value: 5.0.into() }];
        let filter = ResumeFilter::build(&columns, &resume, &[]);
        let (text, parameters) = exact(&filter).expect("expected Exact filter");
        // The boundary value is bound as a parameter, never inlined as SQL.
        assert!(text.contains("c.rank > @cosmosResumeFilter0"), "{text}");
        assert!(
            text.contains("IS_NUMBER(c.rank) AND c.rank = @cosmosResumeFilter0"),
            "{text}"
        );
        assert!(!text.contains("_rid"), "{text}: rid stays client-side");
        assert_eq!(
            parameters,
            &[ResumeParameter {
                name: "@cosmosResumeFilter0".to_owned(),
                value: serde_json::json!(5.0),
            }]
        );
    }

    #[test]
    fn single_column_descending_filter_uses_less_than() {
        let columns = vec![desc("c.rank")];
        let resume = vec![OrderByResumeValue::Number { value: 5.0.into() }];
        let filter = ResumeFilter::build(&columns, &resume, &[]);
        let (text, _) = exact(&filter).unwrap();
        assert!(text.contains("c.rank < @cosmosResumeFilter0"), "{text}");
    }

    #[test]
    fn multi_column_filter_builds_seek_predicate_prefix() {
        let columns = vec![asc("c.rank"), desc("c.name")];
        let resume = vec![
            OrderByResumeValue::Number { value: 5.0.into() },
            OrderByResumeValue::String {
                value: "mid".to_owned(),
            },
        ];
        let filter = ResumeFilter::build(&columns, &resume, &[]);
        let (text, parameters) = exact(&filter).unwrap();
        // First disjunct: strictly after on column 0 alone.
        assert!(text.contains("c.rank > @cosmosResumeFilter0"), "{text}");
        // Second disjunct: tie on column 0, strictly after (DESC => <) on column 1.
        assert!(
            text.contains("c.rank = @cosmosResumeFilter0")
                && text.contains("c.name < @cosmosResumeFilter1"),
            "{text}"
        );
        // Final disjunct: exact tie on both key columns (no `_rid` clause;
        // the numeric rid cut-off is applied client-side).
        assert!(text.contains("c.name = @cosmosResumeFilter1"), "{text}");
        assert!(!text.contains("_rid"), "{text}: rid stays client-side");
        // The string boundary is never inlined; it's bound as a parameter.
        assert!(!text.contains("'mid'"), "{text}: no raw string literal");
        assert_eq!(
            parameters,
            &[
                ResumeParameter {
                    name: "@cosmosResumeFilter0".to_owned(),
                    value: serde_json::json!(5.0),
                },
                ResumeParameter {
                    name: "@cosmosResumeFilter1".to_owned(),
                    value: serde_json::json!("mid"),
                },
            ]
        );
    }

    #[test]
    fn string_boundary_binds_parameter_and_never_inlines_text() {
        // A boundary string with a quote, backslash, newline, tab, and a
        // non-ASCII symbol must never leak into the SQL text — it's bound
        // verbatim as a parameter value, so escaping is the service's job.
        let nasty = "a' OR 1=1 -- \\ \n\t\u{2713}\u{7}";
        let columns = vec![asc("c.name")];
        let resume = vec![OrderByResumeValue::String {
            value: nasty.to_owned(),
        }];
        let filter = ResumeFilter::build(&columns, &resume, &[]);
        let (text, parameters) = exact(&filter).unwrap();
        assert!(text.contains("c.name > @cosmosResumeFilter0"), "{text}");
        assert!(text.contains("c.name = @cosmosResumeFilter0"), "{text}");
        // None of the adversarial characters may appear in the SQL text.
        for needle in ["OR 1=1", "'", "\\", "\n", "\t", "\u{2713}"] {
            assert!(
                !text.contains(needle),
                "boundary text {needle:?} must not appear in SQL: {text}"
            );
        }
        assert_eq!(
            parameters,
            &[ResumeParameter {
                name: "@cosmosResumeFilter0".to_owned(),
                value: serde_json::Value::String(nasty.to_owned()),
            }],
            "the exact boundary string round-trips as the parameter value"
        );
    }

    #[test]
    fn parameter_names_avoid_collision_with_existing_parameters() {
        // A caller already using names that start with the default prefix
        // must not be silently overwritten: the generated prefix extends
        // until no existing name shares it.
        let columns = vec![asc("c.rank"), asc("c.name")];
        let resume = vec![
            OrderByResumeValue::Number { value: 1.0.into() },
            OrderByResumeValue::String {
                value: "x".to_owned(),
            },
        ];
        let existing = vec![
            "@cosmosResumeFilter0".to_owned(),
            "@cosmosResumeFilter1".to_owned(),
        ];
        let filter = ResumeFilter::build(&columns, &resume, &existing);
        let (text, parameters) = exact(&filter).unwrap();
        for p in parameters {
            assert!(
                !existing.contains(&p.name),
                "generated parameter {} collides with an existing caller parameter",
                p.name
            );
            assert!(text.contains(&p.name), "{text} must reference {}", p.name);
        }
        // Deterministic extension: one extra underscore clears the collision.
        assert_eq!(parameters[0].name, "@cosmosResumeFilter_0");
        assert_eq!(parameters[1].name, "@cosmosResumeFilter_1");
    }

    #[test]
    fn complex_resume_value_forces_positional_rescan() {
        let columns = vec![asc("c.tags")];
        let resume =
            vec![OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]).to_resume_value()];
        let filter = ResumeFilter::build(&columns, &resume, &[]);
        assert!(matches!(filter, ResumeFilter::PositionalRescan));
    }

    #[test]
    fn undefined_boundary_ascending_uses_is_defined_guard_and_no_parameter() {
        let columns = vec![asc("c.rank")];
        let resume = vec![OrderByResumeValue::Undefined];
        let filter = ResumeFilter::build(&columns, &resume, &[]);
        let (text, parameters) = exact(&filter).unwrap();
        assert!(text.contains("IS_DEFINED(c.rank)"), "{text}");
        assert!(text.contains("NOT IS_DEFINED(c.rank)"), "{text}: tie term");
        assert!(
            parameters.is_empty(),
            "undefined boundaries render as type-check builtins, binding no parameter"
        );
    }

    #[test]
    fn scalar_resume_values_reconstruct_their_order_by_item() {
        for item in [
            OrderByItem::Undefined,
            OrderByItem::Null,
            OrderByItem::Boolean(true),
            OrderByItem::Number(3.5.into()),
            OrderByItem::String("x".into()),
        ] {
            let resume = item.to_resume_value();
            assert_eq!(resume.to_scalar_order_by_item(), Some(item));
        }
        let complex = OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]).to_resume_value();
        assert_eq!(complex.to_scalar_order_by_item(), None);
    }

    #[test]
    fn query_fingerprint_is_deterministic_and_distinguishes_text() {
        let a = query_fingerprint("SELECT * FROM c ORDER BY c.rank");
        let b = query_fingerprint("SELECT * FROM c ORDER BY c.rank");
        let c = query_fingerprint("SELECT * FROM c ORDER BY c.name");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    fn exact(filter: &ResumeFilter) -> Option<(&str, &[ResumeParameter])> {
        match filter {
            ResumeFilter::Exact {
                where_fragment,
                parameters,
            } => Some((where_fragment.as_str(), parameters.as_slice())),
            ResumeFilter::PositionalRescan => None,
        }
    }
}
