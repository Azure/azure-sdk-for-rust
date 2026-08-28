// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cross-partition streaming `ORDER BY` value model.
//!
//! Shared vocabulary between [`StreamingOrderedMerge`] and the
//! continuation-token snapshot (`super::snapshot`): [`OrderByItem`] parses
//! one column's envelope value (distinguishing `Undefined` from `null`);
//! [`compare_key_tuples`] is the shared Cosmos-ordered comparator;
//! [`OrderByResumeValue`] is the bounded "last emitted" value persisted in
//! a token. On resume, a range's boundary is sent to the backend as
//! a structured [`resume_filter_json`] — the .NET-compatible `resumeFilter`
//! field of the query body — and the already-emitted prefix of the boundary
//! tie run is trimmed client-side via [`classify_row_vs_boundary`].
//!
//! # Complex sort keys
//!
//! Sorting on a value that evaluates to an array or object is **not
//! supported** in a cross-partition query: [`parse_order_by_items`] rejects
//! such a row with
//! [`crate::error::CosmosStatus::CLIENT_ORDER_BY_COMPLEX_VALUE_UNSUPPORTED`].
//! Ordering complex values requires reproducing the backend's structural
//! hash, and resuming across one requires a hash-shaped resume filter;
//! neither is worth the correctness risk until there is a real scenario
//! for it. The Python and JavaScript SDKs reject them the same way.
//!
//! A query scoped to a single logical partition is unaffected: it never
//! reaches this module (the driver short-circuits it to a trivial pipeline
//! before query planning), so the backend does the sorting and any sort key
//! is fine. The peer SDKs draw the same line — their rejection also lives
//! in the cross-partition merge comparator.
//!
//! # Type order
//!
//! Canonical Cosmos ascending type order:
//! `Undefined < Null < Boolean < Number < String < Array < Object`. Must
//! stay in sync with `crate::query::eval`'s `sort_type_order`.

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::query_plan::SortOrder;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CosmosType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Array,
    Object,
}

/// A single `ORDER BY` key value parsed from a rewritten envelope's
/// `orderByItems[i]`. Distinguishes `Undefined` (no `item` field) from
/// every JSON value, including `null` — load-bearing, since Cosmos ranks
/// `Undefined` before `Null`.
#[derive(Debug, Clone)]
pub(crate) enum OrderByItem {
    Undefined,
    Null,
    Boolean(bool),
    Number(OrderByNumber),
    String(String),
    Array(Vec<OrderByItem>),
    /// Key/value pairs in wire order.
    Object(Vec<(String, OrderByItem)>),
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

/// Compares an `i64` with an `f64` for a *total* order. A finite `f64` is
/// compared exactly (see below); a `NaN` is ordered by `f64::total_cmp`
/// convention so the comparator stays a strict total order even if a test or
/// internal construction slips a `NaN` in — negative `NaN` sorts before every
/// finite value, positive `NaN` after (production deserialization rejects
/// non-finite values, so this never arises from real data). `2^63` is exactly
/// representable, so it bounds the range where `floor` fits an `i64`; the
/// fractional part then breaks a floor-tie without any lossy cast.
fn cmp_i64_f64(a: i64, b: f64) -> Ordering {
    const TWO_POW_63: f64 = 9_223_372_036_854_775_808.0;
    if b.is_nan() {
        // `a` is finite: below +NaN, above -NaN (matches `f64::total_cmp`).
        return if b.is_sign_negative() {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
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

/// Compares a `u64` with an `f64` for a *total* order (see [`cmp_i64_f64`] for
/// the `NaN` convention; `2^64` is exactly representable and bounds the `u64`
/// range).
fn cmp_u64_f64(a: u64, b: f64) -> Ordering {
    const TWO_POW_64: f64 = 18_446_744_073_709_551_616.0;
    if b.is_nan() {
        // `a` is finite: below +NaN, above -NaN (matches `f64::total_cmp`).
        return if b.is_sign_negative() {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
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
    pub(crate) fn from_json_number(n: &serde_json::Number) -> Self {
        if let Some(i) = n.as_i64() {
            Self::I64(i)
        } else if let Some(u) = n.as_u64() {
            Self::U64(u)
        } else {
            Self::F64(n.as_f64().unwrap_or(0.0))
        }
    }

    /// Renders the exact JSON value (integers stay integers) for the wire
    /// form of a scalar resume boundary (its `resumeFilter.value` entry).
    fn to_json_value(self) -> serde_json::Value {
        match self {
            Self::I64(i) => serde_json::Value::Number(i.into()),
            Self::U64(u) => serde_json::Value::Number(u.into()),
            Self::F64(f) => serde_json::Number::from_f64(f)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
        }
    }

    #[cfg(test)]
    fn cosmos_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

/// Numeric equality is value-based across variants (`I64(5) == F64(5.0)`),
/// matching Cosmos, so representation never changes an equality result.
impl PartialEq for OrderByNumber {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// `Eq` soundness: `cmp` is a strict total order for *every* `OrderByNumber`,
// including `NaN` (the cross-variant helpers order `NaN` by `f64::total_cmp`
// convention, and `F64`/`F64` already does), so the `cmp`-derived `PartialEq`
// is a proper equivalence relation — reflexive, symmetric, and transitive —
// even if a test or internal construction introduces a `NaN`. Production never
// does: the only production constructor is `from_json_number` (a
// `serde_json::Number` is always finite), `deserialize` rejects non-finite
// floats below, and the `From<f64>` convenience is `#[cfg(test)]`-only.
impl Eq for OrderByNumber {}

impl PartialOrd for OrderByNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderByNumber {
    fn cmp(&self, other: &Self) -> Ordering {
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
}

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
    fn cosmos_type(&self) -> CosmosType {
        match self {
            Self::Undefined => CosmosType::Undefined,
            Self::Null => CosmosType::Null,
            Self::Boolean(_) => CosmosType::Boolean,
            Self::Number(_) => CosmosType::Number,
            Self::String(_) => CosmosType::String,
            Self::Array(_) => CosmosType::Array,
            Self::Object(_) => CosmosType::Object,
        }
    }

    /// The JSON type name of this item, for diagnostics.
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::Null => "null",
            Self::Boolean(_) => "bool",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
        }
    }

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
}

/// Compares two defined JSON values using the production Cosmos `ORDER BY`
/// relation shared by the streaming merge and the in-memory query oracle.
#[cfg(any(test, feature = "__internal_in_memory_emulator"))]
pub(crate) fn compare_json_values(left: &serde_json::Value, right: &serde_json::Value) -> Ordering {
    OrderByItem::from_json(left).cmp(&OrderByItem::from_json(right))
}

impl PartialEq for OrderByItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for OrderByItem {}

impl PartialOrd for OrderByItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderByItem {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_cmp = self.cosmos_type().cmp(&other.cosmos_type());
        if rank_cmp != Ordering::Equal {
            return rank_cmp;
        }
        match (self, other) {
            (Self::Undefined, Self::Undefined) => Ordering::Equal,
            (Self::Null, Self::Null) => Ordering::Equal,
            (Self::Boolean(a), Self::Boolean(b)) => a.cmp(b),
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::String(a), Self::String(b)) => compare_strings(a, b),
            // Unreachable from the streaming merge — `parse_order_by_items`
            // rejects complex sort keys. Only the in-memory query oracle
            // gets here, so a deterministic structural order is enough: the
            // backend's own array/object order is a structural hash we
            // deliberately no longer reproduce.
            (Self::Array(a), Self::Array(b)) => a.cmp(b),
            (Self::Object(a), Self::Object(b)) => {
                let key = |o: &Vec<(String, Self)>| {
                    let mut sorted = o.clone();
                    sorted.sort_by(|(l, _), (r, _)| compare_strings(l, r));
                    sorted
                };
                key(a).cmp(&key(b))
            }
            _ => unreachable!("rank_cmp already distinguished differing variants"),
        }
    }
}

fn compare_strings(a: &str, b: &str) -> Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

impl OrderByItem {
    /// Converts this item to its bounded, serializable resume-value form.
    /// Returns `None` for a complex (array/object) value, which has no
    /// resume representation — [`parse_order_by_items`] rejects those
    /// before they can reach a boundary.
    pub(crate) fn to_resume_value(&self) -> Option<OrderByResumeValue> {
        Some(match self {
            Self::Undefined => OrderByResumeValue::Undefined,
            Self::Null => OrderByResumeValue::Null,
            Self::Boolean(b) => OrderByResumeValue::Boolean { value: *b },
            Self::Number(n) => OrderByResumeValue::Number { value: *n },
            Self::String(s) => OrderByResumeValue::String { value: s.clone() },
            Self::Array(_) | Self::Object(_) => return None,
        })
    }

    #[cfg(test)]
    fn cosmos_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

/// Parses a rewritten envelope's `orderByItems` array into one
/// [`OrderByItem`] per column, validating the wire shape: must be a JSON
/// array of length `expected_len`, each element a JSON object; a missing
/// `item` key parses as [`OrderByItem::Undefined`].
///
/// Rejects a sort key that evaluates to an array or object — see the
/// module docs. Returns a typed [`crate::error::CosmosError`] (not a
/// panic) for any violation.
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
        .enumerate()
        .map(|(column, element)| {
            let obj = element.as_object().ok_or_else(|| {
                envelope_error(format!(
                    "rewritten envelope `orderByItems` entry must be a JSON object, found {}",
                    json_type_name(element)
                ))
            })?;
            let Some(item) = obj.get("item") else {
                return Ok(OrderByItem::Undefined);
            };
            if matches!(
                item,
                serde_json::Value::Array(_) | serde_json::Value::Object(_)
            ) {
                return Err(complex_order_by_error(column, json_type_name(item)));
            }
            Ok(OrderByItem::from_json(item))
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

/// The user-facing "complex sort key" rejection. `column` is the
/// zero-based `ORDER BY` column index.
pub(crate) fn complex_order_by_error(column: usize, found: &str) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_ORDER_BY_COMPLEX_VALUE_UNSUPPORTED)
        .with_message(format!(
            "ORDER BY column {column} sorts on a value that evaluated to a JSON {found}. \
             Sorting on array or object values is not currently supported; ORDER BY a \
             scalar value instead."
        ))
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
        let c = a[i].cmp(&b[i]);
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
/// reverses it. Callers derive the effective direction from each backend
/// page's query-execution metadata, with the first sort column as the legacy
/// fallback.
pub(crate) fn compare_rids(a: &str, b: &str, direction: SortOrder) -> Ordering {
    let ascending = crate::models::resource_id::compare_document_rids(a, b);
    match direction {
        SortOrder::Ascending => ascending,
        SortOrder::Descending => ascending.reverse(),
    }
}

/// The bounded, serializable representation of one column's "last emitted"
/// value, persisted in a continuation token. Every variant round-trips
/// exactly, so a token never grows with document size — complex sort keys
/// have no resume representation because they are rejected outright (see
/// the module docs).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum OrderByResumeValue {
    Undefined,
    Null,
    Boolean { value: bool },
    Number { value: OrderByNumber },
    String { value: String },
}

impl OrderByResumeValue {
    /// Reconstructs the [`OrderByItem`] this came from.
    #[cfg(test)]
    pub(crate) fn to_order_by_item(&self) -> OrderByItem {
        match self {
            Self::Undefined => OrderByItem::Undefined,
            Self::Null => OrderByItem::Null,
            Self::Boolean { value } => OrderByItem::Boolean(*value),
            Self::Number { value } => OrderByItem::Number(*value),
            Self::String { value } => OrderByItem::String(value.clone()),
        }
    }

    /// The .NET-compatible wire form of this resume value for a query
    /// body's `resumeFilter.value` array (distinct from the token/snapshot
    /// serde form): `Undefined` -> `[]`; every other variant -> the raw
    /// JSON value (integers keep exact i64/u64 precision).
    fn to_wire_value(&self) -> serde_json::Value {
        match self {
            Self::Undefined => serde_json::Value::Array(Vec::new()),
            Self::Null => serde_json::Value::Null,
            Self::Boolean { value } => serde_json::Value::Bool(*value),
            Self::Number { value } => value.to_json_value(),
            Self::String { value } => serde_json::Value::String(value.clone()),
        }
    }

    fn cosmos_type(&self) -> CosmosType {
        match self {
            Self::Undefined => CosmosType::Undefined,
            Self::Null => CosmosType::Null,
            Self::Boolean { .. } => CosmosType::Boolean,
            Self::Number { .. } => CosmosType::Number,
            Self::String { .. } => CosmosType::String,
        }
    }
}

/// Builds the .NET-compatible `resumeFilter` object inserted into a
/// resumed query body, mirroring
/// `Microsoft.Azure.Cosmos.Query.Core.SqlQueryResumeFilter`:
/// `{"value":[<wire values>],"rid":<rid?>,"exclude":<bool>}`.
///
/// `rid` is omitted when `None`. Rust persists a last-emitted boundary per
/// range and always resumes a range with its own boundary as the .NET
/// "target" partition does — `rid` present and `exclude:false` — then trims
/// the already-emitted prefix of the boundary tie run client-side (see
/// [`classify_row_vs_boundary`]).
pub(crate) fn resume_filter_json(
    resume_values: &[OrderByResumeValue],
    rid: Option<&str>,
    exclude: bool,
) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    obj.insert(
        "value".to_owned(),
        serde_json::Value::Array(
            resume_values
                .iter()
                .map(OrderByResumeValue::to_wire_value)
                .collect(),
        ),
    );
    if let Some(rid) = rid {
        obj.insert("rid".to_owned(), serde_json::Value::String(rid.to_owned()));
    }
    obj.insert("exclude".to_owned(), serde_json::Value::Bool(exclude));
    serde_json::Value::Object(obj)
}

/// One column's ascending Cosmos comparison of a returned row's `item`
/// against a persisted resume-boundary `value`. Cross-type pairs order by
/// Cosmos type rank; same-type values by value. A complex `item` only
/// ranks by type here — it is rejected before reaching the discard (see
/// the module docs), so it never ties with a boundary column.
fn column_cmp(item: &OrderByItem, value: &OrderByResumeValue) -> Ordering {
    let rank_cmp = item.cosmos_type().cmp(&value.cosmos_type());
    if rank_cmp != Ordering::Equal {
        return rank_cmp;
    }
    match (item, value) {
        (OrderByItem::Undefined, OrderByResumeValue::Undefined) => Ordering::Equal,
        (OrderByItem::Null, OrderByResumeValue::Null) => Ordering::Equal,
        (OrderByItem::Boolean(a), OrderByResumeValue::Boolean { value }) => a.cmp(value),
        (OrderByItem::Number(a), OrderByResumeValue::Number { value }) => a.cmp(value),
        (OrderByItem::String(a), OrderByResumeValue::String { value }) => compare_strings(a, value),
        _ => unreachable!("type rank already distinguished differing variants"),
    }
}

/// Where a returned row's key tuple sits relative to a persisted resume
/// boundary, for the client-side discard.
pub(crate) enum RowVsBoundary {
    /// The row sorts strictly before the boundary — already emitted.
    Before,
    /// Exact full-key tie; the caller breaks it by `_rid`.
    Tie,
    /// The row sorts at or after the boundary — never treated as
    /// already-emitted, so the discard keeps it.
    After,
}

/// Classifies a returned row's key tuple against a persisted resume boundary
/// (bounded [`OrderByResumeValue`]s), applying each column's direction and
/// stopping at the first non-equal column — the row-vs-boundary counterpart
/// of [`compare_key_tuples`], used by the client-side discard.
///
/// Every column yields a definite ordering (see [`column_cmp`]), so a row
/// is reported [`RowVsBoundary::Before`] (droppable) exactly when it sorts
/// before the boundary. Panics on a length mismatch; callers validate
/// column-count agreement first.
pub(crate) fn classify_row_vs_boundary(
    keys: &[OrderByItem],
    resume_values: &[OrderByResumeValue],
    directions: &[SortOrder],
) -> RowVsBoundary {
    debug_assert_eq!(keys.len(), directions.len());
    debug_assert_eq!(resume_values.len(), directions.len());
    for (i, direction) in directions.iter().enumerate() {
        let ord = match direction {
            SortOrder::Ascending => column_cmp(&keys[i], &resume_values[i]),
            SortOrder::Descending => column_cmp(&keys[i], &resume_values[i]).reverse(),
        };
        match ord {
            Ordering::Less => return RowVsBoundary::Before,
            Ordering::Greater => return RowVsBoundary::After,
            Ordering::Equal => {}
        }
    }
    RowVsBoundary::Tie
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn strings_compare_in_utf16_ordinal_order() {
        assert_eq!(
            OrderByItem::String("a".into()).cosmos_cmp(&OrderByItem::String("b".into())),
            Ordering::Less
        );
        assert_eq!(
            OrderByItem::String("\u{10000}".into())
                .cosmos_cmp(&OrderByItem::String("\u{e000}".into())),
            Ordering::Less
        );
    }

    #[test]
    fn complex_items_compare_structurally_for_the_in_memory_oracle() {
        // Complex sort keys are rejected before they ever reach the merge
        // (see `parse_order_by_items`), so this ordering only has to be
        // deterministic and total for the in-memory query oracle.
        let arr = |vs: &[i64]| {
            OrderByItem::Array(
                vs.iter()
                    .map(|v| OrderByItem::Number((*v).into()))
                    .collect(),
            )
        };
        assert_eq!(arr(&[2]).cmp(&arr(&[3])), Ordering::Less);
        assert_eq!(arr(&[1]).cmp(&arr(&[1, 2])), Ordering::Less);
        assert_eq!(arr(&[1, 2]).cmp(&arr(&[1, 2])), Ordering::Equal);
    }

    #[test]
    fn objects_compare_structurally_and_ignore_property_order() {
        // Property order is a serialization detail, so the same content in a
        // different wire order must still tie.
        let a = OrderByItem::Object(vec![
            ("b".to_owned(), OrderByItem::Number(1_i64.into())),
            ("a".to_owned(), OrderByItem::Number(2_i64.into())),
        ]);
        let b = OrderByItem::Object(vec![
            ("a".to_owned(), OrderByItem::Number(2_i64.into())),
            ("b".to_owned(), OrderByItem::Number(1_i64.into())),
        ]);
        assert_eq!(a.cmp(&b), Ordering::Equal);

        let o2 = OrderByItem::Object(vec![("a".to_owned(), OrderByItem::Number(2_i64.into()))]);
        let o3 = OrderByItem::Object(vec![("a".to_owned(), OrderByItem::Number(3_i64.into()))]);
        assert_eq!(o2.cmp(&o3), Ordering::Less);
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
    fn nan_orders_totally_against_finite_ints_and_floats() {
        // `cmp` must stay a strict total order even if a `NaN` slips in via a
        // test/internal construction. `f64::total_cmp` convention: `-NaN`
        // sorts before every finite value, `+NaN` after. (Production rejects
        // non-finite — see `deserialize_rejects_non_finite_numbers`.)
        let neg_nan = OrderByNumber::from(f64::NAN.copysign(-1.0));
        let pos_nan = OrderByNumber::from(f64::NAN);
        for finite in [
            OrderByNumber::from(i64::MIN),
            OrderByNumber::from(-1_i64),
            OrderByNumber::from(0_u64),
            OrderByNumber::from(3.5_f64),
            OrderByNumber::from(u64::MAX),
            OrderByNumber::from(f64::INFINITY),
        ] {
            assert_eq!(
                neg_nan.cosmos_cmp(&finite),
                Ordering::Less,
                "-NaN < {finite:?}"
            );
            assert_eq!(
                finite.cosmos_cmp(&neg_nan),
                Ordering::Greater,
                "{finite:?} > -NaN"
            );
            assert_eq!(
                pos_nan.cosmos_cmp(&finite),
                Ordering::Greater,
                "+NaN > {finite:?}"
            );
            assert_eq!(
                finite.cosmos_cmp(&pos_nan),
                Ordering::Less,
                "{finite:?} < +NaN"
            );
        }
        assert_eq!(neg_nan.cosmos_cmp(&pos_nan), Ordering::Less);
        assert_eq!(pos_nan.cosmos_cmp(&neg_nan), Ordering::Greater);
        // Reflexive on identical bit patterns.
        assert_eq!(pos_nan.cosmos_cmp(&pos_nan), Ordering::Equal);
        assert_eq!(neg_nan.cosmos_cmp(&neg_nan), Ordering::Equal);
    }

    #[test]
    fn cmp_is_transitive_and_antisymmetric_with_nan_and_finite_values() {
        // A single intransitive triple would make sorting / `BTreeMap`
        // undefined. Exhaustively verify antisymmetry and `a <= b <= c =>
        // a <= c` over a set spanning -NaN, finite ints/floats across every
        // variant, and +NaN.
        let values = [
            OrderByNumber::from(f64::NAN.copysign(-1.0)),
            OrderByNumber::from(i64::MIN),
            OrderByNumber::from(-2.5_f64),
            OrderByNumber::from(-1_i64),
            OrderByNumber::from(0_u64),
            OrderByNumber::from(0.0_f64),
            OrderByNumber::from(1_i64),
            OrderByNumber::from(9_007_199_254_740_993_i64), // 2^53 + 1
            OrderByNumber::from(u64::MAX),
            OrderByNumber::from(f64::INFINITY),
            OrderByNumber::from(f64::NAN),
        ];
        for a in &values {
            for b in &values {
                assert_eq!(
                    a.cosmos_cmp(b),
                    b.cosmos_cmp(a).reverse(),
                    "antisymmetry: {a:?} vs {b:?}"
                );
                for c in &values {
                    if a.cosmos_cmp(b) != Ordering::Greater && b.cosmos_cmp(c) != Ordering::Greater
                    {
                        assert_ne!(
                            a.cosmos_cmp(c),
                            Ordering::Greater,
                            "transitivity: {a:?} <= {b:?} <= {c:?} but a > c"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn scalar_number_wire_value_preserves_exact_integer_precision() {
        // The resume boundary sends the number as the raw JSON `resumeFilter`
        // value; integers must keep full precision (never degrade through f64).
        let wire = |n: OrderByNumber| OrderByResumeValue::Number { value: n }.to_wire_value();
        assert_eq!(
            wire(OrderByNumber::from(i64::MIN)),
            serde_json::json!(i64::MIN)
        );
        assert_eq!(
            wire(OrderByNumber::from(u64::MAX)),
            serde_json::json!(u64::MAX)
        );
        assert_eq!(
            serde_json::to_string(&wire(OrderByNumber::from(9_007_199_254_740_993_i64))).unwrap(),
            "9007199254740993",
            "must not degrade to float notation and lose precision"
        );
        assert_eq!(
            wire(OrderByNumber::from(5.5_f64)),
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
            let resume = item.to_resume_value().expect("scalar");
            let json = serde_json::to_string(&resume).unwrap();
            let back: OrderByResumeValue = serde_json::from_str(&json).unwrap();
            assert_eq!(
                back, resume,
                "token round-trip must preserve the exact value for {number:?}"
            );
            assert_eq!(
                back.to_order_by_item(),
                OrderByItem::Number(number),
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
            let resume = item.to_resume_value().expect("scalar");
            let json = serde_json::to_string(&resume).unwrap();
            let back: OrderByResumeValue = serde_json::from_str(&json).unwrap();
            assert_eq!(back, resume);
            // A scalar resume value reconstructs to the exact item it came
            // from (the basis of the `_rid`-aware client-side discard).
            assert_eq!(resume.to_order_by_item(), item);
        }
    }

    #[test]
    fn complex_items_have_no_resume_value() {
        // Complex sort keys are rejected at envelope-parse time, so they
        // never need a token representation.
        let array = OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]);
        let object = OrderByItem::Object(vec![("x".to_owned(), OrderByItem::Number(1.0.into()))]);
        assert_eq!(array.to_resume_value(), None);
        assert_eq!(object.to_resume_value(), None);
    }

    // ── Resume filter wire model ─────────────────────────────────────────

    #[test]
    fn wire_value_matches_dotnet_serialization_contract() {
        // Undefined -> `[]`; Null/Bool/Number/String -> raw JSON value.
        assert_eq!(
            OrderByResumeValue::Undefined.to_wire_value(),
            serde_json::json!([])
        );
        assert_eq!(
            OrderByResumeValue::Null.to_wire_value(),
            serde_json::Value::Null
        );
        assert_eq!(
            OrderByResumeValue::Boolean { value: true }.to_wire_value(),
            serde_json::json!(true)
        );
        assert_eq!(
            OrderByResumeValue::Number {
                value: 5_i64.into()
            }
            .to_wire_value(),
            serde_json::json!(5)
        );
        assert_eq!(
            OrderByResumeValue::String {
                value: "mid".to_owned(),
            }
            .to_wire_value(),
            serde_json::json!("mid")
        );
    }

    #[test]
    fn wire_value_number_keeps_exact_i64_u64_f64() {
        for (n, expected) in [
            (OrderByNumber::from(i64::MIN), serde_json::json!(i64::MIN)),
            (OrderByNumber::from(u64::MAX), serde_json::json!(u64::MAX)),
            // 2^53 + 1 is not exactly representable as f64; must stay integer.
            (
                OrderByNumber::from(9_007_199_254_740_993_i64),
                serde_json::json!(9_007_199_254_740_993_i64),
            ),
            (OrderByNumber::from(1.5_f64), serde_json::json!(1.5_f64)),
        ] {
            assert_eq!(
                OrderByResumeValue::Number { value: n }.to_wire_value(),
                expected
            );
        }
        assert_eq!(
            serde_json::to_string(
                &OrderByResumeValue::Number {
                    value: OrderByNumber::from(9_007_199_254_740_993_i64),
                }
                .to_wire_value()
            )
            .unwrap(),
            "9007199254740993",
            "an exact integer beyond 2^53 must not degrade to float notation"
        );
    }

    #[test]
    fn resume_filter_json_is_target_style_with_rid_and_exclude_false() {
        let filter = resume_filter_json(
            &[
                OrderByResumeValue::Number { value: 5.0.into() },
                OrderByResumeValue::String {
                    value: "mid".to_owned(),
                },
            ],
            Some("rid-1"),
            false,
        );
        assert_eq!(
            filter,
            serde_json::json!({
                "value": [5.0, "mid"],
                "rid": "rid-1",
                "exclude": false,
            })
        );
    }

    #[test]
    fn resume_filter_json_omits_rid_when_absent() {
        let filter = resume_filter_json(&[OrderByResumeValue::Null], None, true);
        assert_eq!(
            filter,
            serde_json::json!({"value": [null], "exclude": true})
        );
        assert!(
            filter.get("rid").is_none(),
            "an absent rid must be omitted, not serialized as null"
        );
    }

    #[test]
    fn classify_row_vs_boundary_scalar_ascending() {
        let directions = [SortOrder::Ascending];
        let boundary = [OrderByResumeValue::Number { value: 5.0.into() }];
        let classify = |v: f64| {
            classify_row_vs_boundary(&[OrderByItem::Number(v.into())], &boundary, &directions)
        };
        assert!(matches!(classify(4.0), RowVsBoundary::Before));
        assert!(matches!(classify(5.0), RowVsBoundary::Tie));
        assert!(matches!(classify(6.0), RowVsBoundary::After));
    }

    #[test]
    fn classify_row_vs_boundary_applies_descending_direction() {
        let directions = [SortOrder::Descending];
        let boundary = [OrderByResumeValue::Number { value: 5.0.into() }];
        // Under DESC, a larger value sorts *before* the boundary.
        assert!(matches!(
            classify_row_vs_boundary(&[OrderByItem::Number(6.0.into())], &boundary, &directions,),
            RowVsBoundary::Before
        ));
    }

    #[test]
    fn classify_row_vs_boundary_orders_across_types() {
        // A String row is always after a Number boundary (type rank).
        assert!(matches!(
            classify_row_vs_boundary(
                &[OrderByItem::String("x".into())],
                &[OrderByResumeValue::Number { value: 5.0.into() }],
                &[SortOrder::Ascending],
            ),
            RowVsBoundary::After
        ));
        // A Number row is reliably *before* a String boundary (lower rank),
        // even though the boundary column is not the row's own type.
        assert!(matches!(
            classify_row_vs_boundary(
                &[OrderByItem::Number(5.0.into())],
                &[OrderByResumeValue::String {
                    value: "x".to_owned()
                }],
                &[SortOrder::Ascending],
            ),
            RowVsBoundary::Before
        ));
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
            let resume = item.to_resume_value().expect("scalar");
            assert_eq!(resume.to_order_by_item(), item);
        }
    }
}
