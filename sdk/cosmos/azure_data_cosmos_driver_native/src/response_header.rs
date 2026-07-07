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

/// A single response header as an `(id, value)` pair.
///
/// The `value` pointer is a borrowed NUL-terminated UTF-8 string valid for the
/// lifetime of the owning completion (until it is freed). Use
/// [`cosmos_header_name`] to resolve `id` to its canonical wire name.
#[repr(C)]
pub struct CosmosResponseHeader {
    /// Stable numeric identifier for the header (see [`CosmosHeaderId`]).
    pub id: CosmosHeaderId,
    /// Borrowed header value (NUL-terminated UTF-8).
    pub value: *const c_char,
}

// ─────────────────────────────────────────────────────────────────────────────
// Synthesis from the driver's typed response headers
// ─────────────────────────────────────────────────────────────────────────────

/// Owns the `CString` value storage backing a borrowed
/// [`CosmosResponseHeader`] list synthesized from a driver
/// [`CosmosResponseHeaders`]. Kept alive by the owning completion so the
/// `value` pointers stay valid until the completion is freed.
pub(crate) struct OwnedResponseHeaders {
    /// `CString` heap buffers the `list` entries' `value` pointers reference.
    /// Never read directly; kept solely to own the value bytes so the borrowed
    /// `value` pointers stay valid until this owner is dropped.
    #[allow(
        dead_code,
        reason = "storage-only: owns the CString bytes the list's value pointers borrow"
    )]
    values: Vec<CString>,
    /// The `#[repr(C)]` entries handed across the ABI as `(ptr, len)`.
    list: Vec<CosmosResponseHeader>,
}

// SAFETY: the `value` raw pointers inside `list` only ever reference the
// `CString` buffers owned by `values` in the same struct, so the raw pointers
// do not alias foreign state and move with their owner. The struct is only ever
// read behind a shared borrow while the owning completion is alive.
unsafe impl Send for OwnedResponseHeaders {}
unsafe impl Sync for OwnedResponseHeaders {}

impl OwnedResponseHeaders {
    /// An empty header list (no allocations). Used for completions that carry
    /// no response (errors, cancellations, degenerate side-payload shells).
    pub(crate) fn empty() -> Self {
        Self {
            values: Vec::new(),
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

/// Synthesizes a [`CosmosResponseHeader`] list from the driver's typed
/// [`CosmosResponseHeaders`], assigning each populated field its stable
/// [`CosmosHeaderId`] and rendering its value as a string.
///
/// Only the commonly-needed headers are covered today; exhaustive coverage (or
/// a driver-side raw header map) is a follow-up. A header whose rendered value
/// would contain an interior NUL byte is skipped rather than truncated.
pub(crate) fn synthesize_response_headers(headers: &CosmosResponseHeaders) -> OwnedResponseHeaders {
    // (id, rendered-value) pairs collected in declaration order.
    let mut pairs: Vec<(CosmosHeaderId, String)> = Vec::new();

    /// Pushes `(id, render)` when the typed field is present, binding the
    /// borrowed field payload to `$v` so `$render` can render it as a `String`.
    macro_rules! opt {
        ($field:expr, $id:ident, |$v:ident| $render:expr) => {
            if let Some($v) = $field.as_ref() {
                pairs.push((CosmosHeaderId::$id, $render));
            }
        };
    }

    opt!(headers.activity_id, CosmosHeaderIdActivityId, |v| v
        .as_str()
        .to_owned());
    opt!(headers.request_charge, CosmosHeaderIdRequestCharge, |v| v
        .value()
        .to_string());
    opt!(headers.session_token, CosmosHeaderIdSessionToken, |v| v
        .as_str()
        .to_owned());
    opt!(headers.etag, CosmosHeaderIdEtag, |v| v.to_string());
    opt!(headers.continuation, CosmosHeaderIdContinuation, |v| v
        .clone());
    opt!(headers.item_count, CosmosHeaderIdItemCount, |v| v
        .to_string());
    opt!(headers.substatus, CosmosHeaderIdSubStatus, |v| v
        .value()
        .to_string());
    opt!(headers.index_metrics, CosmosHeaderIdIndexMetrics, |v| v
        .clone());
    opt!(headers.query_metrics, CosmosHeaderIdQueryMetrics, |v| v
        .clone());
    opt!(
        headers.server_duration_ms,
        CosmosHeaderIdServerDurationMs,
        |v| v.to_string()
    );
    opt!(headers.lsn, CosmosHeaderIdLsn, |v| v.to_string());
    opt!(headers.item_lsn, CosmosHeaderIdItemLsn, |v| v.to_string());
    opt!(
        headers.offer_replace_pending,
        CosmosHeaderIdOfferReplacePending,
        |v| (if *v { "true" } else { "false" }).to_owned()
    );
    opt!(headers.retry_after_ms, CosmosHeaderIdRetryAfterMs, |v| v
        .to_string());
    opt!(
        headers.correlated_activity_id,
        CosmosHeaderIdCorrelatedActivityId,
        |v| v.clone()
    );
    opt!(
        headers.global_committed_lsn,
        CosmosHeaderIdGlobalCommittedLsn,
        |v| v.to_string()
    );
    opt!(
        headers.number_of_read_regions,
        CosmosHeaderIdNumberOfReadRegions,
        |v| v.to_string()
    );
    opt!(headers.gateway_version, CosmosHeaderIdGatewayVersion, |v| v
        .clone());
    opt!(headers.service_version, CosmosHeaderIdServiceVersion, |v| v
        .clone());

    // Materialize the owned `CString` storage first, then build the `#[repr(C)]`
    // entries pointing into it. Capacity is reserved up-front so no reallocation
    // moves a `CString` after its pointer is captured (the heap buffer is stable
    // regardless, but this keeps the invariant obvious).
    let mut values: Vec<CString> = Vec::with_capacity(pairs.len());
    let mut list: Vec<CosmosResponseHeader> = Vec::with_capacity(pairs.len());
    for (id, rendered) in pairs {
        // Skip a value with an interior NUL rather than truncating it.
        let Ok(cstring) = CString::new(rendered) else {
            continue;
        };
        let value = cstring.as_ptr();
        values.push(cstring);
        list.push(CosmosResponseHeader { id, value });
    }

    OwnedResponseHeaders { values, list }
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
    fn populated_headers_render_ids_and_values() {
        // Set only the plain-typed fields that need no wrapper constructor, so
        // the test exercises the synthesis loop, id assignment, value rendering,
        // and the (ptr, len) view without depending on typed-value builders.
        let mut headers = CosmosResponseHeaders::default();
        headers.continuation = Some("next-page".to_owned());
        headers.item_count = Some(42);
        headers.lsn = Some(1234);
        headers.retry_after_ms = Some(500);
        headers.offer_replace_pending = Some(true);
        headers.gateway_version = Some("2.0.0".to_owned());

        let owned = synthesize_response_headers(&headers);
        let (ptr, len) = owned.as_ptr_len();
        assert!(!ptr.is_null());
        assert_eq!(len, 6);

        // SAFETY: `ptr` addresses `len` initialized entries owned by `owned`.
        let entries = unsafe { std::slice::from_raw_parts(ptr, len) };
        let decoded: Vec<(CosmosHeaderId, String)> = entries
            .iter()
            .map(|e| {
                // SAFETY: each `value` is a NUL-terminated string owned by `owned`.
                let s = unsafe { CStr::from_ptr(e.value) }
                    .to_string_lossy()
                    .into_owned();
                (e.id, s)
            })
            .collect();

        assert_eq!(
            decoded,
            vec![
                (
                    CosmosHeaderId::CosmosHeaderIdContinuation,
                    "next-page".to_owned()
                ),
                (CosmosHeaderId::CosmosHeaderIdItemCount, "42".to_owned()),
                (CosmosHeaderId::CosmosHeaderIdLsn, "1234".to_owned()),
                (
                    CosmosHeaderId::CosmosHeaderIdOfferReplacePending,
                    "true".to_owned()
                ),
                (CosmosHeaderId::CosmosHeaderIdRetryAfterMs, "500".to_owned()),
                (
                    CosmosHeaderId::CosmosHeaderIdGatewayVersion,
                    "2.0.0".to_owned()
                ),
            ]
        );
    }
}
