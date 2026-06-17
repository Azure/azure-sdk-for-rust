// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The typed, two-list **event log** — the capture subsystem's hot-path data model.
//!
//! Instead of encoding a custom byte stream as the operation runs, the recorder appends plain
//! Rust structs to two flat, append-only lists:
//!
//! - [`spans`](EventLog::spans): a [`Span`] per timestamped scope or point event (the operation
//!   root, each request attempt, a hedge race). A span's **id is its index** in this list, and a
//!   span carries an optional `parent` id so the flat list reconstructs into a tree.
//! - [`attrs`](EventLog::attrs): an [`Attr`] per key/value, tagged with the id of the [`Span`] it
//!   belongs to (e.g. `Region`, `Endpoint`, `Status`, `RequestCharge`).
//!
//! Storing either is a single `Vec::push` of a small value — numerics are `Copy`, and the only
//! heap touched is the owned string of a [`AttrValue::Str`] (which the original byte log copied
//! into its buffer anyway). The two `Vec` backbones are pooled and reused across operations (see
//! [`LogPool`](super::pool::LogPool)); a returned log is `clear`-ed, freeing any strings while
//! keeping the allocated capacity.
//!
//! Reading the log is a cold-path concern. Past the gate, [`super::context`] walks the spans /
//! attrs into the canonical [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext), and
//! [`super::encode`] can serialize the two lists into a compact binary form. Neither runs on the
//! hot path.

/// Sentinel `parent` value meaning "this span has no parent" (i.e. the operation root).
pub const NO_PARENT: u32 = u32::MAX;

/// The kind of a [`Span`] — what happened.
///
/// Today the recorder emits the operation root, one span per request attempt, and a span for a
/// hedge race outcome. The enum is the extension point for the richer event vocabulary the model
/// is designed to grow into (for example a partition-key-range merge, or marking a partition
/// unavailable); such point events are recorded as zero-length spans (`start_ns == end_ns`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpanKind {
    /// The operation root. Always span id `0`, with no parent.
    Operation = 0,
    /// A single request attempt (one HTTP round-trip, including a hedge leg).
    Attempt = 1,
    /// The terminal outcome of a hedging race (regions + who won).
    Hedge = 2,
}

impl SpanKind {
    /// Maps a wire byte back to a [`SpanKind`], defaulting unknown values to [`SpanKind::Operation`].
    pub(crate) fn from_u8(value: u8) -> Self {
        match value {
            1 => SpanKind::Attempt,
            2 => SpanKind::Hedge,
            _ => SpanKind::Operation,
        }
    }
}

/// A single timestamped entry in the [`EventLog`]. Its id is its index in [`EventLog::spans`].
///
/// `start_ns` / `end_ns` are nanoseconds relative to the operation start. A point-in-time event
/// has `end_ns == start_ns`; an attempt's `end_ns` is its `start_ns + duration`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// What this span represents.
    pub kind: SpanKind,
    /// The id (index) of the parent span, or [`NO_PARENT`] for the operation root.
    pub parent: u32,
    /// Start time relative to the operation start, in nanoseconds.
    pub start_ns: u64,
    /// End time relative to the operation start, in nanoseconds (`== start_ns` for point events).
    pub end_ns: u64,
}

/// The key of an [`Attr`] — which value is being attached to a span.
///
/// Numeric and string-valued keys are intentionally in one flat namespace; the [`AttrValue`]
/// stored against a key is what distinguishes their representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AttrKey {
    /// Operation name (on the operation root).
    OperationName = 0,
    /// Operation activity id (on the operation root).
    ActivityId = 1,
    /// Final HTTP status of the operation (on the operation root).
    FinalStatus = 2,
    /// Final Cosmos sub-status of the operation (on the operation root).
    FinalSubStatus = 3,
    /// Terminal outcome of the operation: `0` success, `1` error (on the operation root).
    Outcome = 4,
    /// Number of attempts the operation made (on the operation root).
    AttemptCount = 5,
    /// Execution context of an attempt (Initial / Retry / Hedging / …).
    ExecutionContext = 6,
    /// Region an attempt targeted.
    Region = 7,
    /// Endpoint URL an attempt targeted.
    Endpoint = 8,
    /// HTTP status code of an attempt (`0` for a transport failure with no response).
    Status = 9,
    /// Cosmos sub-status of an attempt.
    SubStatus = 10,
    /// Service request id / activity id from an attempt's response.
    ServiceRequestId = 11,
    /// Request charge (RU) of an attempt.
    RequestCharge = 12,
    /// Retry-safety signal for a transport failure (`sent` / `not_sent` / `unknown`).
    RequestSent = 13,
    /// Partition key range id an attempt was routed to.
    PartitionKeyRangeId = 14,
    /// Feed range an attempt covered.
    FeedRange = 15,
    /// Terminal state of a hedge race (see [`HedgeOutcome`](super::recorder::HedgeOutcome)).
    HedgeOutcome = 16,
    /// Hedge threshold in nanoseconds.
    HedgeThresholdNs = 17,
    /// Primary region of a hedge race.
    PrimaryRegion = 18,
    /// Alternate region of a hedge race.
    AlternateRegion = 19,
    /// Region that ultimately served a hedged response.
    ResponseRegion = 20,
}

impl AttrKey {
    /// Maps a wire byte back to an [`AttrKey`], or `None` if it is unknown.
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        let key = match value {
            0 => AttrKey::OperationName,
            1 => AttrKey::ActivityId,
            2 => AttrKey::FinalStatus,
            3 => AttrKey::FinalSubStatus,
            4 => AttrKey::Outcome,
            5 => AttrKey::AttemptCount,
            6 => AttrKey::ExecutionContext,
            7 => AttrKey::Region,
            8 => AttrKey::Endpoint,
            9 => AttrKey::Status,
            10 => AttrKey::SubStatus,
            11 => AttrKey::ServiceRequestId,
            12 => AttrKey::RequestCharge,
            13 => AttrKey::RequestSent,
            14 => AttrKey::PartitionKeyRangeId,
            15 => AttrKey::FeedRange,
            16 => AttrKey::HedgeOutcome,
            17 => AttrKey::HedgeThresholdNs,
            18 => AttrKey::PrimaryRegion,
            19 => AttrKey::AlternateRegion,
            20 => AttrKey::ResponseRegion,
            _ => return None,
        };
        Some(key)
    }
}

/// The typed value of an [`Attr`].
///
/// Three variants cover every captured value: an unsigned integer (statuses, counts, enum
/// discriminants, nanosecond thresholds), a float (request charge), and an owned string (regions,
/// endpoints, ids). Numerics are `Copy`; the string is the only heap allocation and is freed when
/// the log is cleared on return to the pool.
#[derive(Clone, Debug, PartialEq)]
pub enum AttrValue {
    /// An unsigned integer value.
    U64(u64),
    /// A floating-point value.
    F64(f64),
    /// An owned string value.
    Str(Box<str>),
}

impl AttrValue {
    /// Returns the inner value if this is a [`AttrValue::U64`].
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            AttrValue::U64(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the inner value if this is a [`AttrValue::F64`].
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            AttrValue::F64(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the inner string if this is a [`AttrValue::Str`].
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttrValue::Str(v) => Some(v),
            _ => None,
        }
    }
}

/// A single key/value attached to a [`Span`] (by its id).
#[derive(Clone, Debug, PartialEq)]
pub struct Attr {
    /// The id (index) of the span this attribute belongs to.
    pub span: u32,
    /// Which value this is.
    pub key: AttrKey,
    /// The value.
    pub value: AttrValue,
}

/// The per-operation event log: two flat, append-only lists.
///
/// See the [module docs](self) for the design. Build one with [`EventLog::new`] /
/// [`EventLog::with_capacity`] (or rent from a [`LogPool`](super::pool::LogPool)), append with the
/// `push_*` / `attr_*` helpers on the hot path, and reconstruct on the cold path.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventLog {
    spans: Vec<Span>,
    attrs: Vec<Attr>,
}

impl EventLog {
    /// Creates an empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty log with capacity pre-reserved for `spans` spans and `attrs` attributes.
    pub fn with_capacity(spans: usize, attrs: usize) -> Self {
        Self {
            spans: Vec::with_capacity(spans),
            attrs: Vec::with_capacity(attrs),
        }
    }

    /// Reconstructs a log from already-decoded parts (used by [`super::encode`]).
    pub(crate) fn from_parts(spans: Vec<Span>, attrs: Vec<Attr>) -> Self {
        Self { spans, attrs }
    }

    /// The recorded spans, in append order. A span's id is its index here.
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    /// The recorded attributes, in append order.
    pub fn attrs(&self) -> &[Attr] {
        &self.attrs
    }

    /// Whether nothing has been recorded yet.
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() && self.attrs.is_empty()
    }

    /// Clears both lists, freeing any owned strings while retaining capacity (for pooling).
    pub fn clear(&mut self) {
        self.spans.clear();
        self.attrs.clear();
    }

    /// Total retained capacity across both lists, used by the pool's retention bound.
    pub(crate) fn capacity(&self) -> usize {
        self.spans.capacity() + self.attrs.capacity()
    }

    /// Appends a span and returns its id (its index in [`EventLog::spans`]).
    pub fn push_span(&mut self, kind: SpanKind, parent: u32, start_ns: u64, end_ns: u64) -> u32 {
        let id = self.spans.len() as u32;
        self.spans.push(Span {
            kind,
            parent,
            start_ns,
            end_ns,
        });
        id
    }

    /// Sets the end time of a previously pushed span (no-op if `id` is out of range).
    pub fn set_span_end(&mut self, id: u32, end_ns: u64) {
        if let Some(span) = self.spans.get_mut(id as usize) {
            span.end_ns = end_ns;
        }
    }

    /// Appends an unsigned-integer attribute owned by `span`.
    pub fn attr_u64(&mut self, span: u32, key: AttrKey, value: u64) {
        self.attrs.push(Attr {
            span,
            key,
            value: AttrValue::U64(value),
        });
    }

    /// Appends a floating-point attribute owned by `span`.
    pub fn attr_f64(&mut self, span: u32, key: AttrKey, value: f64) {
        self.attrs.push(Attr {
            span,
            key,
            value: AttrValue::F64(value),
        });
    }

    /// Appends a string attribute owned by `span`.
    pub fn attr_str(&mut self, span: u32, key: AttrKey, value: impl Into<Box<str>>) {
        self.attrs.push(Attr {
            span,
            key,
            value: AttrValue::Str(value.into()),
        });
    }

    /// Iterates the direct children of `parent`, yielding each child's `(id, &span)`.
    pub fn children(&self, parent: u32) -> impl Iterator<Item = (u32, &Span)> {
        self.spans
            .iter()
            .enumerate()
            .filter(move |(_, span)| span.parent == parent)
            .map(|(idx, span)| (idx as u32, span))
    }

    /// Returns the first attribute value for `span` with `key`, if present.
    pub fn attr(&self, span: u32, key: AttrKey) -> Option<&AttrValue> {
        self.attrs
            .iter()
            .find(|a| a.span == span && a.key == key)
            .map(|a| &a.value)
    }

    /// Convenience: the first `u64` value for `span` / `key`.
    pub fn attr_u64_of(&self, span: u32, key: AttrKey) -> Option<u64> {
        self.attr(span, key).and_then(AttrValue::as_u64)
    }

    /// Convenience: the first `f64` value for `span` / `key`.
    pub fn attr_f64_of(&self, span: u32, key: AttrKey) -> Option<f64> {
        self.attr(span, key).and_then(AttrValue::as_f64)
    }

    /// Convenience: the first string value for `span` / `key`.
    pub fn attr_str_of(&self, span: u32, key: AttrKey) -> Option<&str> {
        self.attr(span, key).and_then(AttrValue::as_str)
    }

    /// Serializes the log into the compact binary form (a cold-path concern; see [`super::encode`]).
    pub fn to_compact_bytes(&self) -> Vec<u8> {
        super::encode::encode(self)
    }

    /// Restores a log from [`to_compact_bytes`](Self::to_compact_bytes) output, or `None` if the
    /// bytes are malformed.
    pub fn from_compact_bytes(bytes: &[u8]) -> Option<Self> {
        super::encode::decode(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_span_returns_sequential_ids() {
        let mut log = EventLog::new();
        let op = log.push_span(SpanKind::Operation, NO_PARENT, 0, 0);
        let a0 = log.push_span(SpanKind::Attempt, op, 10, 20);
        let a1 = log.push_span(SpanKind::Attempt, op, 20, 30);
        assert_eq!((op, a0, a1), (0, 1, 2));
        assert_eq!(log.spans().len(), 3);
    }

    #[test]
    fn attrs_are_tagged_and_queryable() {
        let mut log = EventLog::new();
        let op = log.push_span(SpanKind::Operation, NO_PARENT, 0, 0);
        let attempt = log.push_span(SpanKind::Attempt, op, 0, 5);
        log.attr_str(attempt, AttrKey::Region, "eastus");
        log.attr_u64(attempt, AttrKey::Status, 200);
        log.attr_f64(attempt, AttrKey::RequestCharge, 4.2);

        assert_eq!(log.attr_str_of(attempt, AttrKey::Region), Some("eastus"));
        assert_eq!(log.attr_u64_of(attempt, AttrKey::Status), Some(200));
        assert_eq!(log.attr_f64_of(attempt, AttrKey::RequestCharge), Some(4.2));
        assert_eq!(log.attr_str_of(attempt, AttrKey::Endpoint), None);
    }

    #[test]
    fn children_filters_by_parent() {
        let mut log = EventLog::new();
        let op = log.push_span(SpanKind::Operation, NO_PARENT, 0, 0);
        log.push_span(SpanKind::Attempt, op, 0, 1);
        log.push_span(SpanKind::Attempt, op, 1, 2);
        log.push_span(SpanKind::Hedge, op, 2, 2);
        let kinds: Vec<_> = log.children(op).map(|(_, s)| s.kind).collect();
        assert_eq!(
            kinds,
            vec![SpanKind::Attempt, SpanKind::Attempt, SpanKind::Hedge]
        );
        assert_eq!(log.children(99).count(), 0);
    }

    #[test]
    fn set_span_end_updates_in_place() {
        let mut log = EventLog::new();
        let op = log.push_span(SpanKind::Operation, NO_PARENT, 0, 0);
        log.set_span_end(op, 7_000_000);
        assert_eq!(log.spans()[0].end_ns, 7_000_000);
        log.set_span_end(42, 1); // out of range: no panic
    }

    #[test]
    fn clear_empties_but_can_be_reused() {
        let mut log = EventLog::with_capacity(4, 8);
        let op = log.push_span(SpanKind::Operation, NO_PARENT, 0, 0);
        log.attr_str(op, AttrKey::OperationName, "read_item");
        assert!(!log.is_empty());
        log.clear();
        assert!(log.is_empty());
        assert!(log.capacity() >= 12, "capacity is retained for reuse");
    }
}
