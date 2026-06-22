// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The typed, two-list **event log** — the capture subsystem's hot-path data model.
//!
//! Instead of encoding a custom byte stream as the operation runs, the recorder appends plain
//! Rust structs to two flat, append-only lists held by an [`EventLogStorage`]:
//!
//! - `spans`: a [`Span`] per timestamped scope or point event (the operation root, each request
//!   attempt, a hedge race). Each span has a [`SpanId`] and an optional `parent` so the flat list
//!   reconstructs into a tree.
//! - `attrs`: an [`Attr`] per key/value, tagged with the [`SpanId`] it belongs to (e.g. `Region`,
//!   `Endpoint`, `Status`, `RequestCharge`).
//!
//! Storing either is a single `Vec::push` of a small value — numerics are `Copy`, and a string
//! attribute can be stored zero-copy from a `&'static str` ([`AttrValue::StaticStr`]) or an
//! existing heap string ([`AttrValue::SharedStr`]), or copied ([`AttrValue::Str`]).
//!
//! ## Pooling — the lease
//!
//! [`EventLogStorage`] (the two `Vec`s) is what the [`LogPool`](super::pool::LogPool) pools. An
//! [`EventLog`] is a **lease**: it bundles an `Arc<LogPool>` with the rented storage and returns
//! that storage to the pool automatically when it is dropped (RAII). Callers therefore never have
//! to remember to "return the buffer" — they just drop the [`EventLog`] (or whatever owns it). The
//! [`EventLog`] derefs to its [`EventLogStorage`], so all the `push_*` / query helpers are
//! available directly on the lease.
//!
//! Reading the log is a cold-path concern. Past the gate, [`super::context`] walks the spans /
//! attrs into the canonical [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext), and
//! [`super::encode`] can serialize the storage into a compact binary form. Neither runs on the
//! hot path.

use super::pool::LogPool;
use crate::error::CosmosStatus;
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// The identity of a [`Span`] within an [`EventLogStorage`].
///
/// A `SpanId` is the span's index **plus one**, stored in a [`NonZeroU32`]. The `+1` lets
/// `Option<SpanId>` represent "no parent" (the operation root) for free: the niche optimization
/// uses the otherwise-impossible `0` to encode [`None`], so `Option<SpanId>` is the same size as a
/// `u32` with no separate discriminant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpanId(NonZeroU32);

impl SpanId {
    /// Creates the id for the span stored at `index` (`index + 1`, saturating at [`u32::MAX`]).
    pub fn from_index(index: usize) -> Self {
        let raw = u32::try_from(index)
            .unwrap_or(u32::MAX - 1)
            .saturating_add(1);
        // `raw` is always >= 1 here, so the NonZero construction never fails.
        SpanId(NonZeroU32::new(raw).expect("index + 1 is non-zero"))
    }

    /// The index of this span in [`EventLogStorage::spans`].
    pub fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }

    /// The raw, non-zero wire value (`index + 1`), used by the cold-path codec.
    pub(crate) fn get(self) -> u32 {
        self.0.get()
    }

    /// Reconstructs a `SpanId` from a raw wire value (`0` is rejected), used by the codec.
    pub(crate) fn from_raw(raw: u32) -> Option<Self> {
        NonZeroU32::new(raw).map(SpanId)
    }
}

/// A nanosecond time offset relative to the start of an operation.
///
/// A newtype rather than a bare `u64` so the unit is part of the type instead of a field-name
/// suffix, and so offsets can't be accidentally mixed with absolute timestamps or durations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TimeOffset(u64);

impl TimeOffset {
    /// The start of the operation.
    pub const ZERO: TimeOffset = TimeOffset(0);

    /// Creates an offset from a raw nanosecond count.
    pub fn from_nanos(nanos: u64) -> Self {
        TimeOffset(nanos)
    }

    /// The offset as a raw nanosecond count.
    pub fn as_nanos(self) -> u64 {
        self.0
    }

    /// The non-negative gap from `earlier` to `self` (saturating at zero).
    pub fn saturating_sub(self, earlier: TimeOffset) -> TimeOffset {
        TimeOffset(self.0.saturating_sub(earlier.0))
    }
}

/// The kind of a [`Span`] — what happened.
///
/// Today the recorder emits the operation root, one span per request attempt, and a span for a
/// hedge race outcome. The enum is the extension point for the richer event vocabulary the model
/// is designed to grow into (for example a partition-key-range merge, or marking a partition
/// unavailable); such point events are recorded as zero-length spans (`start == end`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpanKind {
    /// The operation root. Always the first span, with no parent.
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

/// A single timestamped entry in an [`EventLogStorage`].
///
/// `start` / `end` are offsets relative to the operation start. A point-in-time event has
/// `end == start`; an attempt's `end` is its `start + duration`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// What this span represents.
    pub kind: SpanKind,
    /// The parent span, or [`None`] for the operation root.
    pub parent: Option<SpanId>,
    /// Start time relative to the operation start.
    pub start: TimeOffset,
    /// End time relative to the operation start (`== start` for point events).
    pub end: TimeOffset,
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
    /// Final status of the operation, an [`AttrValue::Status`] (on the operation root).
    FinalStatus = 2,
    /// Terminal outcome of the operation: `0` success, `1` error (on the operation root).
    Outcome = 3,
    /// Number of attempts the operation made (on the operation root).
    AttemptCount = 4,
    /// Execution context of an attempt (Initial / Retry / Hedging / …).
    ExecutionContext = 5,
    /// Region an attempt targeted.
    Region = 6,
    /// Endpoint URL an attempt targeted.
    Endpoint = 7,
    /// Status of an attempt, an [`AttrValue::Status`]. Absent for a transport failure (no response).
    Status = 8,
    /// Service request id / activity id from an attempt's response.
    ServiceRequestId = 9,
    /// Request charge (RU) of an attempt.
    RequestCharge = 10,
    /// Retry-safety signal for a transport failure (`sent` / `not_sent` / `unknown`).
    RequestSent = 11,
    /// Partition key range id an attempt was routed to.
    PartitionKeyRangeId = 12,
    /// Feed range an attempt covered.
    FeedRange = 13,
    /// Terminal state of a hedge race (see [`HedgeOutcome`](super::recorder::HedgeOutcome)).
    HedgeOutcome = 14,
    /// Hedge threshold in nanoseconds.
    HedgeThresholdNs = 15,
    /// Primary region of a hedge race.
    PrimaryRegion = 16,
    /// Alternate region of a hedge race.
    AlternateRegion = 17,
    /// Region that ultimately served a hedged response.
    ResponseRegion = 18,
    /// Pipeline type of an attempt ([`PipelineType`](super::PipelineType)) as a `u64` discriminant.
    PipelineType = 19,
    /// Transport security of an attempt ([`TransportSecurity`](super::TransportSecurity)).
    TransportSecurity = 20,
    /// Transport kind of an attempt ([`TransportKind`](super::TransportKind)).
    TransportKind = 21,
    /// Negotiated HTTP version of an attempt ([`TransportHttpVersion`](super::TransportHttpVersion)).
    TransportHttpVersion = 22,
    /// Server-reported request duration in milliseconds (distinct from the client-observed span).
    ServerDurationMs = 23,
}

impl AttrKey {
    /// Maps a wire byte back to an [`AttrKey`], or `None` if it is unknown.
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        let key = match value {
            0 => AttrKey::OperationName,
            1 => AttrKey::ActivityId,
            2 => AttrKey::FinalStatus,
            3 => AttrKey::Outcome,
            4 => AttrKey::AttemptCount,
            5 => AttrKey::ExecutionContext,
            6 => AttrKey::Region,
            7 => AttrKey::Endpoint,
            8 => AttrKey::Status,
            9 => AttrKey::ServiceRequestId,
            10 => AttrKey::RequestCharge,
            11 => AttrKey::RequestSent,
            12 => AttrKey::PartitionKeyRangeId,
            13 => AttrKey::FeedRange,
            14 => AttrKey::HedgeOutcome,
            15 => AttrKey::HedgeThresholdNs,
            16 => AttrKey::PrimaryRegion,
            17 => AttrKey::AlternateRegion,
            18 => AttrKey::ResponseRegion,
            19 => AttrKey::PipelineType,
            20 => AttrKey::TransportSecurity,
            21 => AttrKey::TransportKind,
            22 => AttrKey::TransportHttpVersion,
            23 => AttrKey::ServerDurationMs,
            _ => return None,
        };
        Some(key)
    }
}

/// The typed value of an [`Attr`].
///
/// The variants cover every captured value while letting the hot path avoid allocation wherever
/// possible: an unsigned integer ([`U64`](AttrValue::U64)), a float
/// ([`F64`](AttrValue::F64)), a first-class Cosmos status ([`Status`](AttrValue::Status), no bigger
/// than a `u64`), and three string flavors — a zero-copy `&'static str`
/// ([`StaticStr`](AttrValue::StaticStr), e.g. a region literal), a shared heap string
/// ([`SharedStr`](AttrValue::SharedStr), an `Arc<str>` that points at something already on the
/// heap), or an owned copy ([`Str`](AttrValue::Str)).
#[derive(Clone, Debug, PartialEq)]
pub enum AttrValue {
    /// An unsigned integer value.
    U64(u64),
    /// A floating-point value.
    F64(f64),
    /// A first-class Cosmos status (HTTP status + optional sub-status).
    Status(CosmosStatus),
    /// A borrowed `'static` string — stored with no allocation.
    StaticStr(&'static str),
    /// A shared heap string — points at an existing allocation without copying.
    SharedStr(Arc<str>),
    /// An owned string copied into the log.
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

    /// Returns the inner status if this is a [`AttrValue::Status`].
    pub fn as_status(&self) -> Option<CosmosStatus> {
        match self {
            AttrValue::Status(s) => Some(*s),
            _ => None,
        }
    }

    /// Returns the inner string for any of the string variants.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttrValue::StaticStr(s) => Some(s),
            AttrValue::SharedStr(s) => Some(s),
            AttrValue::Str(s) => Some(s),
            _ => None,
        }
    }
}

/// A single key/value attached to a [`Span`] (by its [`SpanId`]).
#[derive(Clone, Debug, PartialEq)]
pub struct Attr {
    /// The span this attribute belongs to.
    pub span: SpanId,
    /// Which value this is.
    pub key: AttrKey,
    /// The value.
    pub value: AttrValue,
}

/// The pooled, per-operation event storage: two flat, append-only lists.
///
/// This is the data that the [`LogPool`](super::pool::LogPool) reuses across operations. It is
/// usually accessed through an [`EventLog`] lease (which returns it to the pool on drop), but it
/// can also be used standalone — for example by the `encode` submodule when round-tripping the
/// compact binary form, or in tests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventLogStorage {
    spans: Vec<Span>,
    attrs: Vec<Attr>,
}

impl EventLogStorage {
    /// Span capacity of a freshly allocated storage. Sized so the common 1–4-attempt operation
    /// fits without growing (an op span + up to ~7 attempt/hedge spans).
    pub const DEFAULT_SPANS: usize = 8;
    /// Attribute capacity of a freshly allocated storage. Sized so the common 1–4-attempt
    /// operation fits without growing.
    pub const DEFAULT_ATTRS: usize = 32;
    /// The combined default entry capacity (`DEFAULT_SPANS + DEFAULT_ATTRS`). The pool retains only
    /// storages at or under this size and frees any that grew larger.
    pub const DEFAULT_CAPACITY: usize = Self::DEFAULT_SPANS + Self::DEFAULT_ATTRS;

    /// Creates empty storage with no pre-allocated capacity.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates empty storage sized for the common operation (see [`DEFAULT_CAPACITY`](Self::DEFAULT_CAPACITY)).
    pub fn with_default_capacity() -> Self {
        Self::with_capacity(Self::DEFAULT_SPANS, Self::DEFAULT_ATTRS)
    }

    /// Creates empty storage with capacity pre-reserved for `spans` spans and `attrs` attributes.
    pub fn with_capacity(spans: usize, attrs: usize) -> Self {
        Self {
            spans: Vec::with_capacity(spans),
            attrs: Vec::with_capacity(attrs),
        }
    }

    /// Reconstructs storage from already-decoded parts (used by [`super::encode`]).
    pub(crate) fn from_parts(spans: Vec<Span>, attrs: Vec<Attr>) -> Self {
        Self { spans, attrs }
    }

    /// The recorded spans, in append order. A span's [`SpanId`] is [`SpanId::from_index`] of its position.
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

    /// Appends a span and returns its [`SpanId`].
    pub fn push_span(
        &mut self,
        kind: SpanKind,
        parent: Option<SpanId>,
        start: TimeOffset,
        end: TimeOffset,
    ) -> SpanId {
        let id = SpanId::from_index(self.spans.len());
        self.spans.push(Span {
            kind,
            parent,
            start,
            end,
        });
        id
    }

    /// Sets the end time of a previously pushed span (no-op if `id` is out of range).
    pub fn set_span_end(&mut self, id: SpanId, end: TimeOffset) {
        if let Some(span) = self.spans.get_mut(id.index()) {
            span.end = end;
        }
    }

    /// Appends an unsigned-integer attribute owned by `span`.
    pub fn attr_u64(&mut self, span: SpanId, key: AttrKey, value: u64) {
        self.push_attr(span, key, AttrValue::U64(value));
    }

    /// Appends a floating-point attribute owned by `span`.
    pub fn attr_f64(&mut self, span: SpanId, key: AttrKey, value: f64) {
        self.push_attr(span, key, AttrValue::F64(value));
    }

    /// Appends a first-class [`CosmosStatus`] attribute owned by `span`.
    pub fn attr_status(&mut self, span: SpanId, key: AttrKey, value: CosmosStatus) {
        self.push_attr(span, key, AttrValue::Status(value));
    }

    /// Appends a zero-copy `&'static str` attribute owned by `span`.
    pub fn attr_static_str(&mut self, span: SpanId, key: AttrKey, value: &'static str) {
        self.push_attr(span, key, AttrValue::StaticStr(value));
    }

    /// Appends a shared (`Arc<str>`) string attribute owned by `span`, without copying.
    pub fn attr_shared_str(&mut self, span: SpanId, key: AttrKey, value: Arc<str>) {
        self.push_attr(span, key, AttrValue::SharedStr(value));
    }

    /// Appends an owned (copied) string attribute owned by `span`.
    pub fn attr_str(&mut self, span: SpanId, key: AttrKey, value: impl Into<Box<str>>) {
        self.push_attr(span, key, AttrValue::Str(value.into()));
    }

    fn push_attr(&mut self, span: SpanId, key: AttrKey, value: AttrValue) {
        self.attrs.push(Attr { span, key, value });
    }

    /// Iterates the direct children of `parent`, yielding each child's `(SpanId, &Span)`.
    pub fn children(&self, parent: SpanId) -> impl Iterator<Item = (SpanId, &Span)> {
        self.spans
            .iter()
            .enumerate()
            .filter(move |(_, span)| span.parent == Some(parent))
            .map(|(idx, span)| (SpanId::from_index(idx), span))
    }

    /// Returns the first attribute value for `span` with `key`, if present.
    pub fn attr(&self, span: SpanId, key: AttrKey) -> Option<&AttrValue> {
        self.attrs
            .iter()
            .find(|a| a.span == span && a.key == key)
            .map(|a| &a.value)
    }

    /// Convenience: the first `u64` value for `span` / `key`.
    pub fn attr_u64_of(&self, span: SpanId, key: AttrKey) -> Option<u64> {
        self.attr(span, key).and_then(AttrValue::as_u64)
    }

    /// Convenience: the first `f64` value for `span` / `key`.
    pub fn attr_f64_of(&self, span: SpanId, key: AttrKey) -> Option<f64> {
        self.attr(span, key).and_then(AttrValue::as_f64)
    }

    /// Convenience: the first [`CosmosStatus`] value for `span` / `key`.
    pub fn attr_status_of(&self, span: SpanId, key: AttrKey) -> Option<CosmosStatus> {
        self.attr(span, key).and_then(AttrValue::as_status)
    }

    /// Convenience: the first string value for `span` / `key`.
    pub fn attr_str_of(&self, span: SpanId, key: AttrKey) -> Option<&str> {
        self.attr(span, key).and_then(AttrValue::as_str)
    }

    /// Serializes the storage into the compact binary form (a cold-path concern; see the `encode`
    /// submodule).
    pub fn to_compact_bytes(&self) -> Vec<u8> {
        super::encode::encode(self)
    }

    /// Restores storage from [`to_compact_bytes`](Self::to_compact_bytes) output, or `None` if the
    /// bytes are malformed.
    pub fn from_compact_bytes(bytes: &[u8]) -> Option<Self> {
        super::encode::decode(bytes)
    }
}

/// A **lease** on a pooled [`EventLogStorage`].
///
/// An `EventLog` bundles an `Arc<LogPool>` with the rented [`EventLogStorage`] and returns that
/// storage to the pool automatically when the lease is dropped (RAII). Callers therefore never
/// manage the buffer's lifecycle explicitly — they hold (or move) the `EventLog`, and dropping it
/// (on success, error, cancellation, or panic) does the right thing. The lease derefs to its
/// storage, so `push_*` and the query helpers are available directly.
#[derive(Debug)]
pub struct EventLog {
    pool: Arc<LogPool>,
    storage: EventLogStorage,
}

impl EventLog {
    /// Creates a lease wrapping `storage`, to be returned to `pool` on drop. Called by
    /// [`LogPool::rent`](super::pool::LogPool::rent).
    pub(crate) fn new(pool: Arc<LogPool>, storage: EventLogStorage) -> Self {
        Self { pool, storage }
    }

    /// Borrows the underlying storage (e.g. for reconstruction past the gate).
    pub fn storage(&self) -> &EventLogStorage {
        &self.storage
    }
}

impl Deref for EventLog {
    type Target = EventLogStorage;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for EventLog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}

impl Drop for EventLog {
    fn drop(&mut self) {
        // RAII: hand the storage back to the pool. `give_back` clears it and applies the retention
        // bound, so a partially written or oversized storage never poisons the next rent.
        self.pool.give_back(std::mem::take(&mut self.storage));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::StatusCode;

    fn op_storage() -> (EventLogStorage, SpanId) {
        let mut storage = EventLogStorage::new();
        let op = storage.push_span(
            SpanKind::Operation,
            None,
            TimeOffset::ZERO,
            TimeOffset::ZERO,
        );
        (storage, op)
    }

    #[test]
    fn span_ids_index_and_round_trip() {
        let id = SpanId::from_index(0);
        assert_eq!(id.index(), 0);
        assert_eq!(id.get(), 1);
        assert_eq!(SpanId::from_raw(1), Some(id));
        assert_eq!(SpanId::from_raw(0), None, "0 encodes None/no-parent");
        // Option<SpanId> uses the niche, so it is the same size as a bare u32.
        assert_eq!(
            std::mem::size_of::<Option<SpanId>>(),
            std::mem::size_of::<u32>()
        );
    }

    #[test]
    fn push_span_returns_sequential_ids() {
        let (mut storage, op) = op_storage();
        let a0 = storage.push_span(
            SpanKind::Attempt,
            Some(op),
            TimeOffset::from_nanos(10),
            TimeOffset::from_nanos(20),
        );
        let a1 = storage.push_span(
            SpanKind::Attempt,
            Some(op),
            TimeOffset::from_nanos(20),
            TimeOffset::from_nanos(30),
        );
        assert_eq!((op.index(), a0.index(), a1.index()), (0, 1, 2));
        assert_eq!(storage.spans().len(), 3);
        assert_eq!(storage.spans()[1].parent, Some(op));
    }

    #[test]
    fn attrs_are_tagged_and_queryable() {
        let (mut storage, op) = op_storage();
        let attempt = storage.push_span(
            SpanKind::Attempt,
            Some(op),
            TimeOffset::ZERO,
            TimeOffset::from_nanos(5),
        );
        storage.attr_str(attempt, AttrKey::Region, "eastus");
        storage.attr_status(
            attempt,
            AttrKey::Status,
            CosmosStatus::from_parts(StatusCode::from(200), None),
        );
        storage.attr_f64(attempt, AttrKey::RequestCharge, 4.2);

        assert_eq!(
            storage.attr_str_of(attempt, AttrKey::Region),
            Some("eastus")
        );
        assert_eq!(
            storage
                .attr_status_of(attempt, AttrKey::Status)
                .map(|s| u16::from(s.status_code())),
            Some(200)
        );
        assert_eq!(
            storage.attr_f64_of(attempt, AttrKey::RequestCharge),
            Some(4.2)
        );
        assert_eq!(storage.attr_str_of(attempt, AttrKey::Endpoint), None);
    }

    #[test]
    fn string_variants_share_one_accessor() {
        let (mut storage, op) = op_storage();
        storage.attr_static_str(op, AttrKey::Region, "westus");
        let shared: Arc<str> = Arc::from("eastus");
        storage.attr_shared_str(op, AttrKey::PrimaryRegion, shared);
        storage.attr_str(op, AttrKey::Endpoint, "https://acct/");
        assert_eq!(storage.attr_str_of(op, AttrKey::Region), Some("westus"));
        assert_eq!(
            storage.attr_str_of(op, AttrKey::PrimaryRegion),
            Some("eastus")
        );
        assert_eq!(
            storage.attr_str_of(op, AttrKey::Endpoint),
            Some("https://acct/")
        );
    }

    #[test]
    fn children_filters_by_parent() {
        let (mut storage, op) = op_storage();
        storage.push_span(
            SpanKind::Attempt,
            Some(op),
            TimeOffset::ZERO,
            TimeOffset::from_nanos(1),
        );
        storage.push_span(
            SpanKind::Attempt,
            Some(op),
            TimeOffset::from_nanos(1),
            TimeOffset::from_nanos(2),
        );
        storage.push_span(
            SpanKind::Hedge,
            Some(op),
            TimeOffset::from_nanos(2),
            TimeOffset::from_nanos(2),
        );
        let kinds: Vec<_> = storage.children(op).map(|(_, s)| s.kind).collect();
        assert_eq!(
            kinds,
            vec![SpanKind::Attempt, SpanKind::Attempt, SpanKind::Hedge]
        );
        assert_eq!(storage.children(SpanId::from_index(99)).count(), 0);
    }

    #[test]
    fn set_span_end_updates_in_place() {
        let (mut storage, op) = op_storage();
        storage.set_span_end(op, TimeOffset::from_nanos(7_000_000));
        assert_eq!(storage.spans()[0].end, TimeOffset::from_nanos(7_000_000));
        storage.set_span_end(SpanId::from_index(42), TimeOffset::from_nanos(1));
        // out of range: no panic
    }

    #[test]
    fn clear_empties_but_can_be_reused() {
        let mut storage = EventLogStorage::with_capacity(4, 8);
        let op = storage.push_span(
            SpanKind::Operation,
            None,
            TimeOffset::ZERO,
            TimeOffset::ZERO,
        );
        storage.attr_str(op, AttrKey::OperationName, "read_item");
        assert!(!storage.is_empty());
        storage.clear();
        assert!(storage.is_empty());
        assert!(storage.capacity() >= 12, "capacity is retained for reuse");
    }
}
