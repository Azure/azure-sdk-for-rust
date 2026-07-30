// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response header-id model for the C ABI boundary.
//!
//! The driver exposes response headers as a strongly-typed
//! [`azure_data_cosmos_driver::models::CosmosResponseHeaders`] struct, not a
//! raw key/value map, and its header-name constants are `pub(crate)`. To hand
//! a *generic* header list to host SDKs without string-comparing names, the
//! wrapper assigns each known response header a stable numeric
//! [`CosmosHeaderId`] and pairs it with its rendered value in a
//! [`CosmosResponseHeader`]. The id → canonical wire-name mapping is exposed to
//! the SDK via [`cosmos_header_name`], so an SDK maps ids (codegen-friendly)
//! rather than matching header-name strings.
//!
//! This module ships the id model and the synthesis from the driver's typed
//! headers; the [`crate::completion`] surface carries the synthesized list
//! inline on each completion.

use std::ffi::{c_char, CString};

use azure_data_cosmos_driver::models::CosmosResponseHeaders;

// ─────────────────────────────────────────────────────────────────────────────
// Header id (cosmos_header_id_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Stable numeric identifier for a known Cosmos response header.
///
/// The id namespace is **append-only**: new headers get new ids and existing
/// ids never change value, so a generated SDK mapping table stays valid across
/// wrapper versions. [`CosmosHeaderIdUnknown`](Self::CosmosHeaderIdUnknown)
/// (`0`) is a forward-compat sentinel — an older SDK that does not recognize a
/// newer id routes it through its default branch, and
/// [`cosmos_header_name`] returns NULL for it.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosHeaderId {
    /// Unknown / unmapped header (forward-compat sentinel).
    CosmosHeaderIdUnknown = 0,
    /// `x-ms-activity-id`.
    CosmosHeaderIdActivityId = 1,
    /// `x-ms-request-charge`.
    CosmosHeaderIdRequestCharge = 2,
    /// `x-ms-session-token`.
    CosmosHeaderIdSessionToken = 3,
    /// `etag`.
    CosmosHeaderIdEtag = 4,
    /// `x-ms-continuation`.
    CosmosHeaderIdContinuation = 5,
    /// `x-ms-item-count`.
    CosmosHeaderIdItemCount = 6,
    /// `x-ms-substatus`.
    CosmosHeaderIdSubStatus = 7,
    /// `x-ms-cosmos-index-utilization`.
    CosmosHeaderIdIndexMetrics = 8,
    /// `x-ms-documentdb-query-metrics`.
    CosmosHeaderIdQueryMetrics = 9,
    /// `x-ms-request-duration-ms`.
    CosmosHeaderIdServerDurationMs = 10,
    /// `lsn`.
    CosmosHeaderIdLsn = 11,
    /// `x-ms-item-lsn`.
    CosmosHeaderIdItemLsn = 12,
    /// `x-ms-offer-replace-pending`.
    CosmosHeaderIdOfferReplacePending = 13,
    /// `x-ms-retry-after-ms`.
    CosmosHeaderIdRetryAfterMs = 14,
    /// `x-ms-cosmos-correlated-activityid`.
    CosmosHeaderIdCorrelatedActivityId = 15,
    /// `x-ms-global-committed-lsn`.
    CosmosHeaderIdGlobalCommittedLsn = 16,
    /// `x-ms-number-of-read-regions`.
    CosmosHeaderIdNumberOfReadRegions = 17,
    /// `x-ms-gatewayversion`.
    CosmosHeaderIdGatewayVersion = 18,
    /// `x-ms-serviceversion`.
    CosmosHeaderIdServiceVersion = 19,
}

impl CosmosHeaderId {
    /// The canonical wire header name for this id, or `None` for
    /// [`CosmosHeaderIdUnknown`](Self::CosmosHeaderIdUnknown).
    ///
    /// The returned `&CStr` is statically allocated (baked into the binary via
    /// [`c_str!`]).
    fn wire_name(self) -> Option<&'static std::ffi::CStr> {
        let name = match self {
            Self::CosmosHeaderIdUnknown => return None,
            Self::CosmosHeaderIdActivityId => c_str!("x-ms-activity-id"),
            Self::CosmosHeaderIdRequestCharge => c_str!("x-ms-request-charge"),
            Self::CosmosHeaderIdSessionToken => c_str!("x-ms-session-token"),
            Self::CosmosHeaderIdEtag => c_str!("etag"),
            Self::CosmosHeaderIdContinuation => c_str!("x-ms-continuation"),
            Self::CosmosHeaderIdItemCount => c_str!("x-ms-item-count"),
            Self::CosmosHeaderIdSubStatus => c_str!("x-ms-substatus"),
            Self::CosmosHeaderIdIndexMetrics => c_str!("x-ms-cosmos-index-utilization"),
            Self::CosmosHeaderIdQueryMetrics => c_str!("x-ms-documentdb-query-metrics"),
            Self::CosmosHeaderIdServerDurationMs => c_str!("x-ms-request-duration-ms"),
            Self::CosmosHeaderIdLsn => c_str!("lsn"),
            Self::CosmosHeaderIdItemLsn => c_str!("x-ms-item-lsn"),
            Self::CosmosHeaderIdOfferReplacePending => c_str!("x-ms-offer-replace-pending"),
            Self::CosmosHeaderIdRetryAfterMs => c_str!("x-ms-retry-after-ms"),
            Self::CosmosHeaderIdCorrelatedActivityId => c_str!("x-ms-cosmos-correlated-activityid"),
            Self::CosmosHeaderIdGlobalCommittedLsn => c_str!("x-ms-global-committed-lsn"),
            Self::CosmosHeaderIdNumberOfReadRegions => c_str!("x-ms-number-of-read-regions"),
            Self::CosmosHeaderIdGatewayVersion => c_str!("x-ms-gatewayversion"),
            Self::CosmosHeaderIdServiceVersion => c_str!("x-ms-serviceversion"),
        };
        Some(name)
    }
}

/// Returns the canonical wire header name (NUL-terminated UTF-8) for a header
/// id, or NULL for [`CosmosHeaderId::CosmosHeaderIdUnknown`] / an unrecognized
/// id.
///
/// The returned pointer is **statically allocated** and lives for the lifetime
/// of the process; callers must **not** free it. This is the id → name mapping
/// an SDK uses to render or match headers by their well-known names without
/// hardcoding the table itself.
#[no_mangle]
pub extern "C" fn cosmos_header_name(id: CosmosHeaderId) -> *const c_char {
    id.wire_name().map_or(std::ptr::null(), |n| n.as_ptr())
}

// ─────────────────────────────────────────────────────────────────────────────
// Response header entry (cosmos_response_header_t)
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// Tagged value (cosmos_value_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Discriminant for a [`CosmosValue`].
///
/// Stored on the value as a raw `u8` (validated, never transmuted), so hosts
/// that see an out-of-range discriminant from a newer runtime version route
/// it through their default branch rather than triggering undefined behavior.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosValueKind {
    /// String payload — read from `payload.string_value` (borrowed
    /// NUL-terminated UTF-8, valid until the completion is freed).
    CosmosValueKindString = 0,
    /// Signed 64-bit integer payload — read from `payload.i64_value`.
    CosmosValueKindI64 = 1,
    /// 64-bit floating-point payload — read from `payload.f64_value`.
    CosmosValueKindF64 = 2,
    /// Boolean payload — read from `payload.bool_value`.
    CosmosValueKindBool = 3,
    /// Unsigned 64-bit integer payload — read from `payload.u64_value`.
    /// Used for headers whose driver type is `u64` (LSNs, `retry-after-ms`)
    /// so the full range is preserved instead of being saturated to
    /// [`i64::MAX`].
    CosmosValueKindU64 = 4,
}

/// Payload half of the [`CosmosValue`] tagged union. Only the field selected
/// by the sibling `kind` discriminant may be read; reading any other field is
/// undefined behavior.
///
/// The wrapper only ever writes this union (Rust → C direction), so hosts
/// observing a well-formed `CosmosValue` never see an invalid bit pattern
/// under a given kind.
#[repr(C)]
#[derive(Clone, Copy)]
pub union CosmosValuePayload {
    /// Borrowed NUL-terminated UTF-8 string, valid until the owning
    /// completion is freed. Read iff `kind == CosmosValueKindString`.
    pub string_value: *const c_char,
    /// Signed 64-bit integer. Read iff `kind == CosmosValueKindI64`.
    pub i64_value: i64,
    /// 64-bit floating-point value. Read iff `kind == CosmosValueKindF64`.
    pub f64_value: f64,
    /// Boolean value. Read iff `kind == CosmosValueKindBool`.
    pub bool_value: bool,
    /// Unsigned 64-bit integer. Read iff `kind == CosmosValueKindU64`.
    pub u64_value: u64,
}

/// A tagged union carrying a header (or, in the future, diagnostic) value in
/// its native type — a string, signed integer, floating point number, or
/// boolean. Numeric headers avoid the stringify-on-emit / parse-on-read round
/// trip the earlier `*const c_char`-only surface required.
///
/// The `kind` field is a raw `u8` matching a [`CosmosValueKind`] discriminant.
/// The wrapper is the sole producer of these values so hosts never observe
/// an out-of-range kind in practice; forward-compat readers should still
/// treat an unrecognized discriminant as "unknown / skip".
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosValue {
    /// Which payload field is active, matching a [`CosmosValueKind`].
    pub kind: u8,
    /// Native-typed payload; read the field selected by `kind`.
    pub payload: CosmosValuePayload,
}

impl CosmosValue {
    /// Builds a value carrying a borrowed C-string pointer.
    fn string(ptr: *const c_char) -> Self {
        Self {
            kind: CosmosValueKind::CosmosValueKindString as u8,
            payload: CosmosValuePayload { string_value: ptr },
        }
    }

    /// Builds a value carrying a signed 64-bit integer.
    fn i64(v: i64) -> Self {
        Self {
            kind: CosmosValueKind::CosmosValueKindI64 as u8,
            payload: CosmosValuePayload { i64_value: v },
        }
    }

    /// Builds a value carrying an f64.
    fn f64(v: f64) -> Self {
        Self {
            kind: CosmosValueKind::CosmosValueKindF64 as u8,
            payload: CosmosValuePayload { f64_value: v },
        }
    }

    /// Builds a value carrying a boolean.
    fn bool(v: bool) -> Self {
        Self {
            kind: CosmosValueKind::CosmosValueKindBool as u8,
            payload: CosmosValuePayload { bool_value: v },
        }
    }

    /// Builds a value carrying an unsigned 64-bit integer.
    fn u64(v: u64) -> Self {
        Self {
            kind: CosmosValueKind::CosmosValueKindU64 as u8,
            payload: CosmosValuePayload { u64_value: v },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Response header entry (cosmos_response_header_t)
// ─────────────────────────────────────────────────────────────────────────────

/// A single response header as an `(id, value)` pair.
///
/// The `value` carries the header's native type via a [`CosmosValue`] tagged
/// union — string payloads borrow storage owned by the completion (valid
/// until it is freed), while numeric and boolean payloads live inline. Use
/// [`cosmos_header_name`] to resolve `id` to its canonical wire name.
#[repr(C)]
pub struct CosmosResponseHeader {
    /// Stable numeric identifier for the header (see [`CosmosHeaderId`]).
    pub id: CosmosHeaderId,
    /// Native-typed header value (see [`CosmosValue`] / [`CosmosValueKind`]).
    pub value: CosmosValue,
}

// ─────────────────────────────────────────────────────────────────────────────
// Synthesis from the driver's typed response headers
// ─────────────────────────────────────────────────────────────────────────────

/// Owns the `CString` value storage backing a borrowed
/// [`CosmosResponseHeader`] list synthesized from a driver
/// [`CosmosResponseHeaders`]. Kept alive by the owning completion so string
/// payloads stay valid until the completion is freed. Numeric and boolean
/// payloads are inline in each [`CosmosValue`] and need no separate storage.
pub(crate) struct OwnedResponseHeaders {
    /// `CString` heap buffers each `String`-kind entry's `payload.string_value`
    /// references. Never read directly; kept solely to own the value bytes so
    /// the borrowed string pointers stay valid until this owner is dropped.
    #[allow(
        dead_code,
        reason = "storage-only: owns the CString bytes the list's string payloads borrow"
    )]
    strings: Vec<CString>,
    /// The `#[repr(C)]` entries handed across the ABI as `(ptr, len)`.
    list: Vec<CosmosResponseHeader>,
}

// SAFETY: the string payloads inside `list` only ever reference `CString`
// buffers owned by `strings` in the same struct, so the raw pointers do not
// alias foreign state and move with their owner. Numeric / boolean payloads
// carry no pointers. The struct is only ever read behind a shared borrow
// while the owning completion is alive.
unsafe impl Send for OwnedResponseHeaders {}
unsafe impl Sync for OwnedResponseHeaders {}

impl OwnedResponseHeaders {
    /// An empty header list (no allocations). Used for completions that carry
    /// no response (errors, cancellations, degenerate side-payload shells).
    pub(crate) fn empty() -> Self {
        Self {
            strings: Vec::new(),
            list: Vec::new(),
        }
    }

    /// Borrowed `(ptr, len)` view of the header list, normalizing an empty list
    /// to `(NULL, 0)`. Valid until this value is dropped.
    pub(crate) fn as_ptr_len(&self) -> (*const CosmosResponseHeader, usize) {
        if self.list.is_empty() {
            (std::ptr::null(), 0)
        } else {
            (self.list.as_ptr(), self.list.len())
        }
    }
}

/// Intermediate representation of a synthesized header value before its
/// `CString` storage is materialized. Numeric / boolean variants carry their
/// payload inline; the string variant carries an owned `String` that becomes
/// a heap `CString` owned by the [`OwnedResponseHeaders`].
enum SynthesizedValue {
    String(String),
    I64(i64),
    F64(f64),
    Bool(bool),
    U64(u64),
}

/// Synthesizes a [`CosmosResponseHeader`] list from the driver's typed
/// [`CosmosResponseHeaders`], assigning each populated field its stable
/// [`CosmosHeaderId`] and a [`CosmosValue`] carrying its native type.
///
/// Numeric headers (request-charge, item-count, LSNs, retry-after, …) carry
/// their value inline as an integer or floating-point number rather than a
/// string, so SDK consumers no longer need to stringify-then-parse. Only the
/// commonly-needed headers are covered today; exhaustive coverage (or a
/// driver-side raw header map) is a follow-up. A string-typed value with an
/// interior NUL byte is skipped rather than truncated.
pub(crate) fn synthesize_response_headers(headers: &CosmosResponseHeaders) -> OwnedResponseHeaders {
    // (id, native-typed value) pairs collected in declaration order.
    let mut pairs: Vec<(CosmosHeaderId, SynthesizedValue)> = Vec::new();

    /// Pushes `(id, SynthesizedValue::$variant($render))` when the typed field
    /// is present, binding the borrowed field payload to `$v` so `$render` can
    /// render it as the native-typed value.
    macro_rules! opt {
        ($field:expr, $id:ident, $variant:ident, |$v:ident| $render:expr) => {
            if let Some($v) = $field.as_ref() {
                pairs.push((CosmosHeaderId::$id, SynthesizedValue::$variant($render)));
            }
        };
    }

    opt!(headers.activity_id, CosmosHeaderIdActivityId, String, |v| v
        .as_str()
        .to_owned());
    opt!(
        headers.request_charge,
        CosmosHeaderIdRequestCharge,
        F64,
        |v| v.value()
    );
    opt!(
        headers.session_token,
        CosmosHeaderIdSessionToken,
        String,
        |v| v.as_str().to_owned()
    );
    opt!(headers.etag, CosmosHeaderIdEtag, String, |v| v.to_string());
    opt!(
        headers.continuation,
        CosmosHeaderIdContinuation,
        String,
        |v| v.clone()
    );
    opt!(headers.item_count, CosmosHeaderIdItemCount, I64, |v| {
        i64::from(*v)
    });
    opt!(headers.substatus, CosmosHeaderIdSubStatus, I64, |v| {
        i64::from(v.value())
    });
    opt!(
        headers.index_metrics,
        CosmosHeaderIdIndexMetrics,
        String,
        |v| v.clone()
    );
    opt!(
        headers.query_metrics,
        CosmosHeaderIdQueryMetrics,
        String,
        |v| v.clone()
    );
    opt!(
        headers.server_duration_ms,
        CosmosHeaderIdServerDurationMs,
        F64,
        |v| *v
    );
    opt!(headers.lsn, CosmosHeaderIdLsn, U64, |v| *v);
    opt!(headers.item_lsn, CosmosHeaderIdItemLsn, U64, |v| *v);
    opt!(
        headers.offer_replace_pending,
        CosmosHeaderIdOfferReplacePending,
        Bool,
        |v| *v
    );
    opt!(
        headers.retry_after_ms,
        CosmosHeaderIdRetryAfterMs,
        U64,
        |v| *v
    );
    opt!(
        headers.correlated_activity_id,
        CosmosHeaderIdCorrelatedActivityId,
        String,
        |v| v.clone()
    );
    opt!(
        headers.global_committed_lsn,
        CosmosHeaderIdGlobalCommittedLsn,
        I64,
        |v| *v
    );
    opt!(
        headers.number_of_read_regions,
        CosmosHeaderIdNumberOfReadRegions,
        I64,
        |v| i64::from(*v)
    );
    opt!(
        headers.gateway_version,
        CosmosHeaderIdGatewayVersion,
        String,
        |v| v.clone()
    );
    opt!(
        headers.service_version,
        CosmosHeaderIdServiceVersion,
        String,
        |v| v.clone()
    );

    // Materialize the owned `CString` storage first, then build the `#[repr(C)]`
    // entries pointing into it. Capacity is reserved up-front so no reallocation
    // moves a `CString` after its pointer is captured (the heap buffer is stable
    // regardless, but this keeps the invariant obvious).
    let mut strings: Vec<CString> = Vec::with_capacity(pairs.len());
    let mut list: Vec<CosmosResponseHeader> = Vec::with_capacity(pairs.len());
    for (id, synth) in pairs {
        let value = match synth {
            SynthesizedValue::String(s) => {
                // Skip a value with an interior NUL rather than truncating it.
                let Ok(cstring) = CString::new(s) else {
                    continue;
                };
                let ptr = cstring.as_ptr();
                strings.push(cstring);
                CosmosValue::string(ptr)
            }
            SynthesizedValue::I64(v) => CosmosValue::i64(v),
            SynthesizedValue::F64(v) => CosmosValue::f64(v),
            SynthesizedValue::Bool(v) => CosmosValue::bool(v),
            SynthesizedValue::U64(v) => CosmosValue::u64(v),
        };
        list.push(CosmosResponseHeader { id, value });
    }

    OwnedResponseHeaders { strings, list }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    fn name_of(id: CosmosHeaderId) -> Option<String> {
        let p = cosmos_header_name(id);
        if p.is_null() {
            None
        } else {
            // SAFETY: non-NULL return is a static NUL-terminated string.
            Some(unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned())
        }
    }

    #[test]
    fn unknown_id_has_no_name() {
        assert!(cosmos_header_name(CosmosHeaderId::CosmosHeaderIdUnknown).is_null());
    }

    #[test]
    fn known_ids_map_to_canonical_names() {
        assert_eq!(
            name_of(CosmosHeaderId::CosmosHeaderIdActivityId).as_deref(),
            Some("x-ms-activity-id")
        );
        assert_eq!(
            name_of(CosmosHeaderId::CosmosHeaderIdRequestCharge).as_deref(),
            Some("x-ms-request-charge")
        );
        assert_eq!(
            name_of(CosmosHeaderId::CosmosHeaderIdEtag).as_deref(),
            Some("etag")
        );
        assert_eq!(
            name_of(CosmosHeaderId::CosmosHeaderIdSubStatus).as_deref(),
            Some("x-ms-substatus")
        );
        assert_eq!(
            name_of(CosmosHeaderId::CosmosHeaderIdServiceVersion).as_deref(),
            Some("x-ms-serviceversion")
        );
    }

    #[test]
    fn empty_headers_synthesize_to_empty_list() {
        let headers = CosmosResponseHeaders::default();
        let owned = synthesize_response_headers(&headers);
        let (ptr, len) = owned.as_ptr_len();
        assert!(ptr.is_null());
        assert_eq!(len, 0);
    }

    #[test]
    fn populated_headers_render_ids_and_native_types() {
        // Set only the plain-typed fields that need no wrapper constructor, so
        // the test exercises the synthesis loop, id assignment, native-typed
        // value emission, and the (ptr, len) view without depending on
        // typed-value builders. `server_duration_ms` exercises the `F64`
        // variant; `lsn` / `retry_after_ms` exercise the `U64` variant with a
        // value above `i64::MAX` to prove the full unsigned range survives
        // (the earlier saturating-to-`i64` path silently clamped these).
        let mut headers = CosmosResponseHeaders::default();
        headers.continuation = Some("next-page".to_owned());
        headers.item_count = Some(42);
        headers.server_duration_ms = Some(12.5);
        headers.lsn = Some(u64::MAX - 1);
        headers.retry_after_ms = Some(500);
        headers.offer_replace_pending = Some(true);
        headers.gateway_version = Some("2.0.0".to_owned());

        let owned = synthesize_response_headers(&headers);
        let (ptr, len) = owned.as_ptr_len();
        assert!(!ptr.is_null());
        assert_eq!(len, 7);

        // SAFETY: `ptr` addresses `len` initialized entries owned by `owned`.
        let entries = unsafe { std::slice::from_raw_parts(ptr, len) };

        // Renders each entry's `(id, kind, textual-value)` so the assertion
        // exercises every variant of the tagged union: string payloads are
        // dereferenced through the borrowed C-string pointer, and numeric /
        // boolean payloads are read via their matching union field.
        let decoded: Vec<(CosmosHeaderId, CosmosValueKind, String)> = entries
            .iter()
            .map(|e| {
                let kind = match e.value.kind {
                    0 => CosmosValueKind::CosmosValueKindString,
                    1 => CosmosValueKind::CosmosValueKindI64,
                    2 => CosmosValueKind::CosmosValueKindF64,
                    3 => CosmosValueKind::CosmosValueKindBool,
                    4 => CosmosValueKind::CosmosValueKindU64,
                    other => panic!("unexpected kind {other}"),
                };
                let rendered = match kind {
                    CosmosValueKind::CosmosValueKindString => {
                        // SAFETY: kind == String → payload is a valid
                        // NUL-terminated string owned by `owned`.
                        unsafe { CStr::from_ptr(e.value.payload.string_value) }
                            .to_string_lossy()
                            .into_owned()
                    }
                    // SAFETY: kind matches the union field being read below.
                    CosmosValueKind::CosmosValueKindI64 => {
                        unsafe { e.value.payload.i64_value }.to_string()
                    }
                    CosmosValueKind::CosmosValueKindF64 => {
                        unsafe { e.value.payload.f64_value }.to_string()
                    }
                    CosmosValueKind::CosmosValueKindBool => {
                        if unsafe { e.value.payload.bool_value } {
                            "true".to_owned()
                        } else {
                            "false".to_owned()
                        }
                    }
                    CosmosValueKind::CosmosValueKindU64 => {
                        unsafe { e.value.payload.u64_value }.to_string()
                    }
                };
                (e.id, kind, rendered)
            })
            .collect();

        assert_eq!(
            decoded,
            vec![
                (
                    CosmosHeaderId::CosmosHeaderIdContinuation,
                    CosmosValueKind::CosmosValueKindString,
                    "next-page".to_owned()
                ),
                (
                    CosmosHeaderId::CosmosHeaderIdItemCount,
                    CosmosValueKind::CosmosValueKindI64,
                    "42".to_owned()
                ),
                (
                    CosmosHeaderId::CosmosHeaderIdServerDurationMs,
                    CosmosValueKind::CosmosValueKindF64,
                    12.5f64.to_string()
                ),
                (
                    CosmosHeaderId::CosmosHeaderIdLsn,
                    CosmosValueKind::CosmosValueKindU64,
                    (u64::MAX - 1).to_string()
                ),
                (
                    CosmosHeaderId::CosmosHeaderIdOfferReplacePending,
                    CosmosValueKind::CosmosValueKindBool,
                    "true".to_owned()
                ),
                (
                    CosmosHeaderId::CosmosHeaderIdRetryAfterMs,
                    CosmosValueKind::CosmosValueKindU64,
                    "500".to_owned()
                ),
                (
                    CosmosHeaderId::CosmosHeaderIdGatewayVersion,
                    CosmosValueKind::CosmosValueKindString,
                    "2.0.0".to_owned()
                ),
            ]
        );
    }
}
