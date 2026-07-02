// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Flat C ABI request surface for the two canonical submit entry points
//! ([`crate::submit::cosmos_submit_operation`] and
//! [`crate::submit::cosmos_submit_singleton_operation`]).
//!
//! ## Why this module exists
//!
//! The original surface exposed ~60 functions: one factory per operation
//! kind (`cosmos_operation_create_item`, `_read_database`, …), a family of
//! per-operation mutators (`cosmos_operation_with_body`, …), and a full
//! options-builder (`cosmos_operation_options_builder_*`). That is a lot of
//! ABI for host SDKs to bind and keep in sync.
//!
//! This module collapses all of it into **two `#[repr(C)]` structs** the
//! host fills out in its own language and hands across the boundary in a
//! single submit call:
//!
//! - [`CosmosOperationOptions`] — a flat mirror of the driver's
//!   [`azure_data_cosmos_driver::options::OperationOptions`]. Every field is
//!   tri-state encoded so the host can express "inherit" vs. an explicit
//!   value/`false` without an opaque builder (see [`OptOf`]).
//! - [`CosmosOperationRequest`] — `kind` + the reference handles, ids,
//!   partition key, feed range, body, per-op tweaks, continuation token, and
//!   a pointer to the options struct.
//!
//! The wrapper validates the request, builds the driver's `CosmosOperation`
//! + `OperationOptions` internally, and dispatches to the requested driver
//! method.
//!
//! Reference *handles* (`cosmos_account_ref_t`, `cosmos_container_ref_t`, …)
//! and the partition-key / feed-range handles are still handles, not flat
//! data: they wrap `Arc`-shared Rust state that cannot be safely round-
//! tripped as plain `#[repr(C)]` bytes.

use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::num::{NonZeroU32, NonZeroU8};

use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_core::http::Etag;
use azure_data_cosmos_driver::models::{
    ActivityId, ContainerReference, ContinuationToken, CosmosOperation, ItemReference,
    MaxItemCountHint, PartitionKey, Precondition, SessionToken, ThroughputControlGroupName,
};
use azure_data_cosmos_driver::options::{
    ContentResponseOnWrite, EndToEndOperationLatencyPolicy, ExcludedRegions, OperationOptions,
    ReadConsistencyStrategy, Region, ThroughputControlOptions,
};

use crate::account_ref::AccountRefHandle;
use crate::container_ref::ContainerRefHandle;
use crate::database_ref::DatabaseRefHandle;
use crate::error::CosmosErrorCode;
use crate::feed_range::FeedRangeHandle;
use crate::partition_key::{CosmosPartitionKeyComponent, PartitionKeyHandle};

// ─────────────────────────────────────────────────────────────────────────────
// Sentinels for tri-state scalar fields
// ─────────────────────────────────────────────────────────────────────────────

/// `OptOf` documents the encoding the host uses for the scalar option
/// fields on [`CosmosOperationOptions`]:
///
/// - **enum fields** (`*_strategy`, `content_response_on_write`): `0` = unset
///   (inherit), any other value = the corresponding driver variant.
/// - **tri-state bools** (`session_capturing_disabled`): `0` = unset,
///   `1` = `false`, `2` = `true`.
/// - **i32 numeric fields** (retry counters): `< 0` = unset,
///   `>= 0` = the value.
/// - **i64 duration fields** (`*_ms`): `< 0` = unset, `>= 0` = milliseconds.
/// - **string / array fields** (`throughput_control_group`,
///   `excluded_regions`, `custom_headers`): NULL / length `0` = unset.
///
/// It is a documentation marker only — the fields are plain integers /
/// pointers so the struct stays `#[repr(C)]`.
pub struct OptOf;

const TRISTATE_UNSET: i8 = 0;
const TRISTATE_FALSE: i8 = 1;
const TRISTATE_TRUE: i8 = 2;

/// Decodes a tri-state bool. Returns `Err(INVALID_OPTION_VALUE)` for an
/// out-of-range discriminant.
fn decode_tristate_bool(v: i8) -> Result<Option<bool>, CosmosErrorCode> {
    match v {
        TRISTATE_UNSET => Ok(None),
        TRISTATE_FALSE => Ok(Some(false)),
        TRISTATE_TRUE => Ok(Some(true)),
        _ => Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue),
    }
}

/// Decodes an `i32` numeric option (`< 0` = unset) into `Option<u32>`.
fn decode_opt_u32(v: i32) -> Option<u32> {
    if v < 0 {
        None
    } else {
        Some(v as u32)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// cosmos_read_consistency_strategy_t  (0 = unset)
// ─────────────────────────────────────────────────────────────────────────────

/// Tri-state mirror of [`ReadConsistencyStrategy`] for the flat options
/// struct. `0` (`Unset`) means "inherit from a lower-priority layer".
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosReadConsistencyStrategy {
    /// Inherit from account / runtime / environment.
    CosmosReadConsistencyStrategyUnset = 0,
    /// Use the default behavior for the chosen consistency level.
    CosmosReadConsistencyStrategyDefault = 1,
    /// Eventual consistency.
    CosmosReadConsistencyStrategyEventual = 2,
    /// Session consistency (the driver's typical default).
    CosmosReadConsistencyStrategySession = 3,
    /// Read the latest version across all regions (single-master / Strong).
    CosmosReadConsistencyStrategyGlobalStrong = 4,
}

impl CosmosReadConsistencyStrategy {
    /// Validates a host-supplied `i32` discriminant and returns the matching
    /// variant, or [`CosmosErrorCode::CosmosErrorCodeInvalidOptionValue`] for
    /// an unknown value.
    ///
    /// The flat options struct stores this field as a raw `i32` (not as this
    /// enum directly): loading an out-of-range value as a fieldless
    /// `#[repr(i32)]` enum would be instant undefined behavior, so the host's
    /// bytes are validated here before any variant is materialized.
    fn from_i32(raw: i32) -> Result<Self, CosmosErrorCode> {
        Ok(match raw {
            0 => Self::CosmosReadConsistencyStrategyUnset,
            1 => Self::CosmosReadConsistencyStrategyDefault,
            2 => Self::CosmosReadConsistencyStrategyEventual,
            3 => Self::CosmosReadConsistencyStrategySession,
            4 => Self::CosmosReadConsistencyStrategyGlobalStrong,
            _ => return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue),
        })
    }

    fn to_driver(self) -> Result<Option<ReadConsistencyStrategy>, CosmosErrorCode> {
        Ok(match self {
            Self::CosmosReadConsistencyStrategyUnset => None,
            Self::CosmosReadConsistencyStrategyDefault => Some(ReadConsistencyStrategy::Default),
            Self::CosmosReadConsistencyStrategyEventual => Some(ReadConsistencyStrategy::Eventual),
            Self::CosmosReadConsistencyStrategySession => Some(ReadConsistencyStrategy::Session),
            Self::CosmosReadConsistencyStrategyGlobalStrong => {
                Some(ReadConsistencyStrategy::GlobalStrong)
            }
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// cosmos_content_response_on_write_t  (0 = unset)
// ─────────────────────────────────────────────────────────────────────────────

/// Tri-state mirror of [`ContentResponseOnWrite`]. `0` (`Unset`) inherits.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosContentResponseOnWriteOpt {
    /// Inherit from a lower-priority layer.
    CosmosContentResponseOnWriteOptUnset = 0,
    /// Server returns no body on write responses.
    CosmosContentResponseOnWriteOptDisabled = 1,
    /// Server returns the written resource in the response body.
    CosmosContentResponseOnWriteOptEnabled = 2,
}

impl CosmosContentResponseOnWriteOpt {
    /// Validates a host-supplied `i32` discriminant and returns the matching
    /// variant, or [`CosmosErrorCode::CosmosErrorCodeInvalidOptionValue`] for
    /// an unknown value. See [`CosmosReadConsistencyStrategy::from_i32`] for
    /// why the field is read as a raw `i32`.
    fn from_i32(raw: i32) -> Result<Self, CosmosErrorCode> {
        Ok(match raw {
            0 => Self::CosmosContentResponseOnWriteOptUnset,
            1 => Self::CosmosContentResponseOnWriteOptDisabled,
            2 => Self::CosmosContentResponseOnWriteOptEnabled,
            _ => return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue),
        })
    }

    fn to_driver(self) -> Result<Option<ContentResponseOnWrite>, CosmosErrorCode> {
        Ok(match self {
            Self::CosmosContentResponseOnWriteOptUnset => None,
            Self::CosmosContentResponseOnWriteOptDisabled => Some(ContentResponseOnWrite::Disabled),
            Self::CosmosContentResponseOnWriteOptEnabled => Some(ContentResponseOnWrite::Enabled),
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// cosmos_header_kv_t
// ─────────────────────────────────────────────────────────────────────────────

/// A single custom request/operation header. Both pointers are
/// NUL-terminated UTF-8 and borrowed for the duration of the submit call;
/// the wrapper copies them before returning.
#[repr(C)]
pub struct CosmosHeaderKv {
    /// Header name (NUL-terminated UTF-8).
    pub name: *const c_char,
    /// Header value (NUL-terminated UTF-8).
    pub value: *const c_char,
}

/// Decodes a `(ptr, len)` header array into a driver `HashMap`. NULL / `0`
/// length yields `None`; an empty-but-non-NULL slice yields `Some(empty)`.
///
/// # Safety
///
/// `headers` must either be NULL or point at `len` initialized
/// [`CosmosHeaderKv`] entries whose `name`/`value` pointers are valid
/// NUL-terminated UTF-8 for the duration of the call.
unsafe fn decode_headers(
    headers: *const CosmosHeaderKv,
    len: usize,
) -> Result<Option<HashMap<HeaderName, HeaderValue>>, CosmosErrorCode> {
    if headers.is_null() || len == 0 {
        return Ok(None);
    }
    // SAFETY: caller contract above.
    let slice = unsafe { std::slice::from_raw_parts(headers, len) };
    let mut map = HashMap::with_capacity(len);
    for kv in slice {
        let name = cstr_to_str(kv.name)?;
        let value = cstr_to_str(kv.value)?;
        map.insert(
            HeaderName::from(name.to_owned()),
            HeaderValue::from(value.to_owned()),
        );
    }
    Ok(Some(map))
}

// ─────────────────────────────────────────────────────────────────────────────
// cosmos_operation_options_t  (flat, #[repr(C)])
// ─────────────────────────────────────────────────────────────────────────────

/// Flat C ABI mirror of the driver's
/// [`azure_data_cosmos_driver::options::OperationOptions`].
///
/// Every field is tri-state encoded (see [`OptOf`]) so the host can
/// distinguish "inherit from a lower layer" from an explicit value. A
/// pointer to this struct rides on [`CosmosOperationRequest::options`]; pass
/// NULL there to use the driver/runtime defaults for every field.
///
/// Construct with [`cosmos_operation_options_default`] to obtain an
/// all-unset value, then set the fields you care about.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosOperationOptions {
    /// Read consistency strategy, encoded as a [`CosmosReadConsistencyStrategy`]
    /// discriminant. `0` (`Unset`) inherits. Stored as a raw `i32` so an
    /// out-of-range host value is validated (not UB) on conversion.
    pub read_consistency_strategy: i32,
    /// Whether write responses include the resource body, encoded as a
    /// [`CosmosContentResponseOnWriteOpt`] discriminant. `0` (`Unset`)
    /// inherits. Stored as a raw `i32` for the same reason as above.
    pub content_response_on_write: i32,
    /// Disable automatic session token management. Tri-state bool.
    pub session_capturing_disabled: i8,
    /// Max region-failover retries. `< 0` = unset.
    pub max_failover_retry_count: i32,
    /// Max session-consistency retries on 404/1002. `< 0` = unset.
    pub max_session_retry_count: i32,
    /// End-to-end timeout (milliseconds). `< 0` = unset.
    pub end_to_end_timeout_ms: i64,
    /// Endpoint unavailability TTL (milliseconds). `< 0` = unset.
    pub endpoint_unavailability_ttl_ms: i64,
    /// Throughput control group name (NUL-terminated UTF-8). NULL = unset.
    pub throughput_control_group: *const c_char,
    /// Excluded regions — array of NUL-terminated UTF-8 region ids.
    /// NULL / `0` length = unset; non-NULL with `0` length is rejected.
    pub excluded_regions: *const *const c_char,
    /// Number of entries in `excluded_regions`.
    pub excluded_regions_len: usize,
    /// Custom headers added to every request for the operation.
    /// NULL / `0` length = none.
    pub custom_headers: *const CosmosHeaderKv,
    /// Number of entries in `custom_headers`.
    pub custom_headers_len: usize,
}

impl CosmosOperationOptions {
    /// Builds the driver [`OperationOptions`] from this flat struct.
    ///
    /// # Safety
    ///
    /// All non-NULL string / array pointers must reference valid
    /// NUL-terminated UTF-8 (and, for arrays, the declared number of
    /// elements) for the duration of the call.
    pub(crate) unsafe fn to_driver(&self) -> Result<OperationOptions, CosmosErrorCode> {
        let mut opts = OperationOptions::default();

        opts.read_consistency_strategy =
            CosmosReadConsistencyStrategy::from_i32(self.read_consistency_strategy)?.to_driver()?;
        opts.content_response_on_write =
            CosmosContentResponseOnWriteOpt::from_i32(self.content_response_on_write)?
                .to_driver()?;
        opts.session_capturing_disabled = decode_tristate_bool(self.session_capturing_disabled)?;

        opts.max_failover_retry_count = decode_opt_u32(self.max_failover_retry_count);
        opts.max_session_retry_count = decode_opt_u32(self.max_session_retry_count);

        if self.end_to_end_timeout_ms >= 0 {
            let dur = std::time::Duration::from_millis(self.end_to_end_timeout_ms as u64);
            opts.end_to_end_latency_policy = Some(EndToEndOperationLatencyPolicy::from(dur));
        }
        if self.endpoint_unavailability_ttl_ms >= 0 {
            opts.endpoint_unavailability_ttl = Some(std::time::Duration::from_millis(
                self.endpoint_unavailability_ttl_ms as u64,
            ));
        }

        if !self.throughput_control_group.is_null() {
            let name = cstr_to_str(self.throughput_control_group)?;
            let mut throughput_control = ThroughputControlOptions::default();
            throughput_control.group_name = Some(ThroughputControlGroupName::from(name.to_owned()));
            opts.throughput_control = Some(throughput_control);
        }

        // SAFETY: caller contract on the array pointer + length.
        if let Some(regions) =
            unsafe { decode_regions(self.excluded_regions, self.excluded_regions_len)? }
        {
            opts.excluded_regions = Some(regions);
        }

        // SAFETY: caller contract on the array pointer + length.
        if let Some(headers) =
            unsafe { decode_headers(self.custom_headers, self.custom_headers_len)? }
        {
            opts.custom_headers = Some(headers);
        }

        Ok(opts)
    }
}

/// Decodes a `(ptr, len)` region-id array into an [`ExcludedRegions`].
/// NULL / `0` length yields `None`. A non-NULL pointer with `0` length is
/// rejected as a malformed input.
///
/// # Safety
///
/// `regions` must either be NULL or point at `len` valid NUL-terminated
/// UTF-8 string pointers for the duration of the call.
unsafe fn decode_regions(
    regions: *const *const c_char,
    len: usize,
) -> Result<Option<ExcludedRegions>, CosmosErrorCode> {
    if regions.is_null() {
        return Ok(None);
    }
    if len == 0 {
        // Non-NULL pointer with zero length is ambiguous; reject it so the
        // host uses NULL to mean "unset" unambiguously.
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
    }
    // SAFETY: caller contract above.
    let slice = unsafe { std::slice::from_raw_parts(regions, len) };
    let mut out = Vec::with_capacity(len);
    for &p in slice {
        let s = cstr_to_str(p)?;
        out.push(Region::from(s.to_owned()));
    }
    Ok(Some(ExcludedRegions(out)))
}

/// Returns an all-unset [`CosmosOperationOptions`] by value. The host
/// mutates the fields it cares about and leaves the rest at their inherit
/// sentinels.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_default() -> CosmosOperationOptions {
    CosmosOperationOptions {
        read_consistency_strategy: CosmosReadConsistencyStrategy::CosmosReadConsistencyStrategyUnset
            as i32,
        content_response_on_write:
            CosmosContentResponseOnWriteOpt::CosmosContentResponseOnWriteOptUnset as i32,
        session_capturing_disabled: TRISTATE_UNSET,
        max_failover_retry_count: -1,
        max_session_retry_count: -1,
        end_to_end_timeout_ms: -1,
        endpoint_unavailability_ttl_ms: -1,
        throughput_control_group: std::ptr::null(),
        excluded_regions: std::ptr::null(),
        excluded_regions_len: 0,
        custom_headers: std::ptr::null(),
        custom_headers_len: 0,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// cosmos_operation_kind_t
// ─────────────────────────────────────────────────────────────────────────────

/// Discriminates which driver `CosmosOperation` factory a
/// [`CosmosOperationRequest`] maps to. Append-only: new kinds get new
/// trailing discriminants so the ABI stays stable.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosOperationKind {
    /// Invalid / uninitialized — always rejected with `INVALID_ARGUMENT`.
    CosmosOperationKindInvalid = 0,

    // Account-scope.
    /// `CosmosOperation::create_database` — requires `account`.
    CosmosOperationKindCreateDatabase = 1,
    /// `CosmosOperation::read_all_databases` — requires `account`.
    CosmosOperationKindReadAllDatabases = 2,
    /// `CosmosOperation::query_databases` — requires `account` + body.
    CosmosOperationKindQueryDatabases = 3,
    /// `CosmosOperation::query_offers` — requires `account` + body.
    CosmosOperationKindQueryOffers = 4,
    /// `CosmosOperation::read_offer` — requires `account` + `resource_link`.
    CosmosOperationKindReadOffer = 5,
    /// `CosmosOperation::replace_offer` — requires `account` + `resource_link` + body.
    CosmosOperationKindReplaceOffer = 6,

    // Database-scope.
    /// `CosmosOperation::read_database` — requires `database`.
    CosmosOperationKindReadDatabase = 7,
    /// `CosmosOperation::delete_database` — requires `database`.
    CosmosOperationKindDeleteDatabase = 8,
    /// `CosmosOperation::create_container` — requires `database` + body.
    CosmosOperationKindCreateContainer = 9,
    /// `CosmosOperation::read_all_containers` — requires `database`.
    CosmosOperationKindReadAllContainers = 10,
    /// `CosmosOperation::query_containers` — requires `database` + body.
    CosmosOperationKindQueryContainers = 11,

    // Container-scope.
    /// `CosmosOperation::read_container` — requires `container`.
    CosmosOperationKindReadContainer = 12,
    /// `CosmosOperation::replace_container` — requires `container` + body.
    CosmosOperationKindReplaceContainer = 13,
    /// `CosmosOperation::delete_container` — requires `container`.
    CosmosOperationKindDeleteContainer = 14,
    /// `CosmosOperation::read_all_items` — requires `container` + `partition_key`.
    CosmosOperationKindReadAllItems = 15,
    /// `CosmosOperation::read_all_items_cross_partition` — requires `container`.
    CosmosOperationKindReadAllItemsCrossPartition = 16,
    /// `CosmosOperation::query_items` — requires `container` + body; `feed_range` optional.
    CosmosOperationKindQueryItems = 17,
    /// `CosmosOperation::batch` — requires `container` + `partition_key` + body.
    CosmosOperationKindBatch = 18,

    // Item-scope.
    /// `CosmosOperation::create_item` — requires `container` + `partition_key` + body.
    CosmosOperationKindCreateItem = 19,
    /// `CosmosOperation::read_item` — requires `container` + `partition_key` + `item_id`.
    CosmosOperationKindReadItem = 20,
    /// `CosmosOperation::upsert_item` — requires `container` + `partition_key` + body.
    CosmosOperationKindUpsertItem = 21,
    /// `CosmosOperation::replace_item` — requires `container` + `partition_key` + `item_id` + body.
    CosmosOperationKindReplaceItem = 22,
    /// `CosmosOperation::delete_item` — requires `container` + `partition_key` + `item_id`.
    CosmosOperationKindDeleteItem = 23,
    /// `CosmosOperation::patch_item` — requires `container` + `partition_key` + `item_id` + body.
    CosmosOperationKindPatchItem = 24,
}

impl CosmosOperationKind {
    /// Validates a host-supplied `i32` discriminant and returns the matching
    /// variant, or [`CosmosErrorCode::CosmosErrorCodeInvalidArgument`] for an
    /// unknown value.
    ///
    /// The request struct stores `kind` as a raw `i32` (not as this enum
    /// directly): loading an out-of-range value as a fieldless `#[repr(i32)]`
    /// enum would be instant undefined behavior, so the host's bytes are
    /// validated here before any variant is materialized.
    fn from_i32(raw: i32) -> Result<Self, CosmosErrorCode> {
        Ok(match raw {
            0 => Self::CosmosOperationKindInvalid,
            1 => Self::CosmosOperationKindCreateDatabase,
            2 => Self::CosmosOperationKindReadAllDatabases,
            3 => Self::CosmosOperationKindQueryDatabases,
            4 => Self::CosmosOperationKindQueryOffers,
            5 => Self::CosmosOperationKindReadOffer,
            6 => Self::CosmosOperationKindReplaceOffer,
            7 => Self::CosmosOperationKindReadDatabase,
            8 => Self::CosmosOperationKindDeleteDatabase,
            9 => Self::CosmosOperationKindCreateContainer,
            10 => Self::CosmosOperationKindReadAllContainers,
            11 => Self::CosmosOperationKindQueryContainers,
            12 => Self::CosmosOperationKindReadContainer,
            13 => Self::CosmosOperationKindReplaceContainer,
            14 => Self::CosmosOperationKindDeleteContainer,
            15 => Self::CosmosOperationKindReadAllItems,
            16 => Self::CosmosOperationKindReadAllItemsCrossPartition,
            17 => Self::CosmosOperationKindQueryItems,
            18 => Self::CosmosOperationKindBatch,
            19 => Self::CosmosOperationKindCreateItem,
            20 => Self::CosmosOperationKindReadItem,
            21 => Self::CosmosOperationKindUpsertItem,
            22 => Self::CosmosOperationKindReplaceItem,
            23 => Self::CosmosOperationKindDeleteItem,
            24 => Self::CosmosOperationKindPatchItem,
            _ => return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument),
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// cosmos_precondition_kind_t
// ─────────────────────────────────────────────────────────────────────────────

/// Selects which optimistic-concurrency precondition (if any) to attach.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosPreconditionKind {
    /// No precondition.
    CosmosPreconditionKindNone = 0,
    /// `If-Match: <etag>` — requires `precondition_etag`.
    CosmosPreconditionKindIfMatch = 1,
    /// `If-None-Match: <etag>` — requires `precondition_etag`.
    CosmosPreconditionKindIfNoneMatch = 2,
}

impl CosmosPreconditionKind {
    /// Validates a host-supplied `i32` discriminant and returns the matching
    /// variant, or [`CosmosErrorCode::CosmosErrorCodeInvalidArgument`] for an
    /// unknown value. See [`CosmosOperationKind::from_i32`] for why the field
    /// is read as a raw `i32`.
    fn from_i32(raw: i32) -> Result<Self, CosmosErrorCode> {
        Ok(match raw {
            0 => Self::CosmosPreconditionKindNone,
            1 => Self::CosmosPreconditionKindIfMatch,
            2 => Self::CosmosPreconditionKindIfNoneMatch,
            _ => return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument),
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// cosmos_operation_request_t
// ─────────────────────────────────────────────────────────────────────────────

/// Self-describing request passed to the two submit entry points. The host
/// fills out the fields relevant to `kind`; irrelevant fields must be left
/// NULL / sentinel (strict validation rejects mismatches with
/// `INVALID_ARGUMENT`).
///
/// All pointers are borrowed for the duration of the submit call only.
#[repr(C)]
pub struct CosmosOperationRequest {
    /// Which operation to build, encoded as a [`CosmosOperationKind`]
    /// discriminant. Stored as a raw `i32` so an out-of-range host value is
    /// validated (not UB) before dispatch.
    pub kind: i32,

    /// Account reference. Required for account-scope kinds; otherwise NULL.
    pub account: *const AccountRefHandle,
    /// Database reference. Required for database-scope kinds; otherwise NULL.
    pub database: *const DatabaseRefHandle,
    /// Container reference. Required for container/item-scope kinds; otherwise NULL.
    pub container: *const ContainerRefHandle,

    /// Item id (NUL-terminated UTF-8). Required for item-scope kinds that
    /// address a specific document; otherwise NULL.
    pub item_id: *const c_char,
    /// Offer resource link (NUL-terminated UTF-8). Required for the offer
    /// kinds; otherwise NULL.
    pub resource_link: *const c_char,

    /// Partition key handle. Required for item-scope, `read_all_items`, and
    /// `batch` (unless the inline `partition_key_components` array is supplied
    /// instead); otherwise NULL.
    pub partition_key: *const PartitionKeyHandle,
    /// Inline partition-key components, assembled by the host in one array so
    /// no separate partition-key construction call is needed. When
    /// `partition_key_components` is non-NULL and `partition_key_len > 0` this
    /// takes precedence over the `partition_key` handle. Each element is a
    /// [`CosmosPartitionKeyComponent`].
    pub partition_key_components: *const CosmosPartitionKeyComponent,
    /// Number of elements in `partition_key_components`. `0` = use the
    /// `partition_key` handle instead.
    pub partition_key_len: usize,
    /// Feed range handle. Optional for `query_items`; otherwise NULL.
    pub feed_range: *const FeedRangeHandle,

    /// Pointer to the first byte of the request body, borrowed for the
    /// duration of the submit call. NULL iff `body_len == 0` ("no body"). The
    /// wrapper copies the bytes into a driver-owned `Vec<u8>` before returning,
    /// so the host may free the buffer immediately after submit. Mirrors the
    /// completion's `body` / `body_len` output fields.
    pub body: *const u8,
    /// Number of bytes addressable from `body`. `0` = no body.
    pub body_len: usize,

    /// Session token override (NUL-terminated UTF-8). NULL = unset.
    pub session_token: *const c_char,
    /// Activity id override (NUL-terminated UTF-8). NULL = auto-generate.
    pub activity_id: *const c_char,
    /// Continuation token to resume a feed (NUL-terminated UTF-8). NULL = none.
    /// Only meaningful for feed kinds dispatched through
    /// `cosmos_submit_operation`.
    pub continuation_token: *const c_char,

    /// Max item count hint for feeds. `< 0` = unset.
    pub max_item_count: i32,
    /// PATCH read-modify-write attempt budget. `0` = unset.
    pub patch_max_attempts: u8,
    /// Populate index metrics. Tri-state bool (`0` unset / `1` false / `2` true).
    pub populate_index_metrics: i8,
    /// Populate query metrics. Tri-state bool (`0` unset / `1` false / `2` true).
    pub populate_query_metrics: i8,

    /// Precondition selector, encoded as a [`CosmosPreconditionKind`]
    /// discriminant. Stored as a raw `i32` so an out-of-range host value is
    /// validated (not UB) before use.
    pub precondition_kind: i32,
    /// ETag for the precondition (NUL-terminated UTF-8). Required iff
    /// `precondition_kind` is not `None`.
    pub precondition_etag: *const c_char,

    /// Per-call options. NULL = use driver/runtime defaults.
    pub options: *const CosmosOperationOptions,
}

// ─────────────────────────────────────────────────────────────────────────────
// Conversion: request → (CosmosOperation, OperationOptions)
// ─────────────────────────────────────────────────────────────────────────────

/// Holds the fully-built driver inputs ready to hand to a driver method.
pub(crate) struct BuiltRequest {
    pub(crate) operation: CosmosOperation,
    pub(crate) options: OperationOptions,
    /// Inbound continuation token (if the host supplied one). Threaded into
    /// [`azure_data_cosmos_driver::driver::CosmosDriver::plan_operation`] by
    /// the feed submit entry point; ignored by the singleton entry point.
    pub(crate) continuation: Option<ContinuationToken>,
}

/// Validates `request` and builds the driver `CosmosOperation` +
/// `OperationOptions` (+ optional inbound continuation token).
///
/// # Safety
///
/// `request` must be non-NULL and all of its non-NULL pointer fields must
/// reference valid data (per the per-field contracts on
/// [`CosmosOperationRequest`]) for the duration of the call.
pub(crate) unsafe fn build_request(
    request: *const CosmosOperationRequest,
) -> Result<BuiltRequest, CosmosErrorCode> {
    if request.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: non-NULL checked; caller guarantees a valid struct.
    let req = unsafe { &*request };

    let operation = unsafe { build_operation(req)? };

    let options = if req.options.is_null() {
        OperationOptions::default()
    } else {
        // SAFETY: non-NULL checked; caller guarantees a valid struct.
        unsafe { (*req.options).to_driver()? }
    };

    let continuation = if req.continuation_token.is_null() {
        None
    } else {
        let token = require_cstr(req.continuation_token)?;
        Some(ContinuationToken::from_string(token.to_owned()))
    };

    Ok(BuiltRequest {
        operation,
        options,
        continuation,
    })
}

/// Builds just the [`CosmosOperation`] (factory + inline mutators) from a
/// validated request.
///
/// # Safety
///
/// See [`build_request`].
unsafe fn build_operation(
    req: &CosmosOperationRequest,
) -> Result<CosmosOperation, CosmosErrorCode> {
    use CosmosOperationKind as K;

    // Validate the host-supplied `kind` discriminant before materializing the
    // enum (the field is a raw `i32` to avoid UB on an out-of-range value).
    let kind = CosmosOperationKind::from_i32(req.kind)?;

    // Borrow the reference handles the kind needs. `require_*` enforces
    // strict scope checking — a kind that needs a container rejects a NULL
    // container, and (below) each factory ignores fields outside its scope
    // only after we've confirmed the in-scope ones are present.
    let op = match kind {
        K::CosmosOperationKindInvalid => {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
        }

        // ── Account-scope ────────────────────────────────────────────────
        K::CosmosOperationKindCreateDatabase => {
            CosmosOperation::create_database(require_account(req)?)
        }
        K::CosmosOperationKindReadAllDatabases => {
            CosmosOperation::read_all_databases(require_account(req)?)
        }
        K::CosmosOperationKindQueryDatabases => {
            CosmosOperation::query_databases(require_account(req)?)
        }
        K::CosmosOperationKindQueryOffers => CosmosOperation::query_offers(require_account(req)?),
        K::CosmosOperationKindReadOffer => {
            let link = require_cstr(req.resource_link)?;
            CosmosOperation::read_offer(require_account(req)?, link.to_owned())
        }
        K::CosmosOperationKindReplaceOffer => {
            let link = require_cstr(req.resource_link)?;
            CosmosOperation::replace_offer(require_account(req)?, link.to_owned())
        }

        // ── Database-scope ───────────────────────────────────────────────
        K::CosmosOperationKindReadDatabase => {
            CosmosOperation::read_database(require_database(req)?)
        }
        K::CosmosOperationKindDeleteDatabase => {
            CosmosOperation::delete_database(require_database(req)?)
        }
        K::CosmosOperationKindCreateContainer => {
            CosmosOperation::create_container(require_database(req)?)
        }
        K::CosmosOperationKindReadAllContainers => {
            CosmosOperation::read_all_containers(require_database(req)?)
        }
        K::CosmosOperationKindQueryContainers => {
            CosmosOperation::query_containers(require_database(req)?)
        }

        // ── Container-scope ──────────────────────────────────────────────
        K::CosmosOperationKindReadContainer => {
            CosmosOperation::read_container(require_container(req)?)
        }
        K::CosmosOperationKindReplaceContainer => {
            CosmosOperation::replace_container(require_container(req)?)
        }
        K::CosmosOperationKindDeleteContainer => {
            CosmosOperation::delete_container(require_container(req)?)
        }
        K::CosmosOperationKindReadAllItems => {
            let container = require_container(req)?;
            let pk = require_partition_key(req)?;
            CosmosOperation::read_all_items(container, pk)
        }
        K::CosmosOperationKindReadAllItemsCrossPartition => {
            CosmosOperation::read_all_items_cross_partition(require_container(req)?)
        }
        K::CosmosOperationKindQueryItems => {
            let container = require_container(req)?;
            // feed_range is optional; NULL → None (whole-container query).
            let feed_range = if req.feed_range.is_null() {
                None
            } else {
                Some(
                    FeedRangeHandle::from_ptr(req.feed_range)
                        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?
                        .inner
                        .clone(),
                )
            };
            CosmosOperation::query_items(container, feed_range)
        }
        K::CosmosOperationKindBatch => {
            let container = require_container(req)?;
            let pk = require_partition_key(req)?;
            CosmosOperation::batch(container, pk)
        }

        // ── Item-scope ───────────────────────────────────────────────────
        K::CosmosOperationKindCreateItem => CosmosOperation::create_item(require_item_ref(req)?),
        K::CosmosOperationKindReadItem => CosmosOperation::read_item(require_item_ref(req)?),
        K::CosmosOperationKindUpsertItem => CosmosOperation::upsert_item(require_item_ref(req)?),
        K::CosmosOperationKindReplaceItem => CosmosOperation::replace_item(require_item_ref(req)?),
        K::CosmosOperationKindDeleteItem => CosmosOperation::delete_item(require_item_ref(req)?),
        K::CosmosOperationKindPatchItem => CosmosOperation::patch_item(require_item_ref(req)?),
    };

    // SAFETY: caller contract on the request's pointer fields.
    unsafe { apply_inline_mutators(op, req) }
}

/// Applies the inline per-operation tweaks (body, session token, activity
/// id, max item count, precondition, patch attempts, metrics flags) carried
/// directly on the request struct.
///
/// The continuation token is **not** handled here: it is a planner input,
/// not an operation header, so the submit entry point threads it through
/// [`azure_data_cosmos_driver::driver::CosmosDriver::plan_operation`].
///
/// # Safety
///
/// See [`build_request`].
unsafe fn apply_inline_mutators(
    mut op: CosmosOperation,
    req: &CosmosOperationRequest,
) -> Result<CosmosOperation, CosmosErrorCode> {
    // Body.
    if req.body_len > 0 {
        if req.body.is_null() {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        }
        // SAFETY: body non-NULL and body_len valid per caller contract.
        let bytes = unsafe { std::slice::from_raw_parts(req.body, req.body_len) };
        op = op.with_body(bytes.to_vec());
    }

    // Session token.
    if !req.session_token.is_null() {
        let token = require_cstr(req.session_token)?;
        op = op.with_session_token(SessionToken::new(token.to_owned()));
    }

    // Activity id.
    if !req.activity_id.is_null() {
        let aid = require_cstr(req.activity_id)?;
        op = op.with_activity_id(ActivityId::from_string(aid.to_owned()));
    }

    // Max item count (`< 0` = unset; `0` rejected — driver enforces NonZeroU32).
    if req.max_item_count >= 0 {
        let hint = if req.max_item_count == 0 {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
        } else {
            // SAFETY-of-invariant: value is > 0 here.
            MaxItemCountHint::Limit(NonZeroU32::new(req.max_item_count as u32).unwrap())
        };
        op = op.with_max_item_count(hint);
    }

    // Precondition. Validate the host-supplied discriminant before
    // materializing the enum (the field is a raw `i32` to avoid UB).
    match CosmosPreconditionKind::from_i32(req.precondition_kind)? {
        CosmosPreconditionKind::CosmosPreconditionKindNone => {}
        CosmosPreconditionKind::CosmosPreconditionKindIfMatch => {
            let etag = require_cstr(req.precondition_etag)?;
            op = op.with_precondition(Precondition::if_match(Etag::from(etag.to_owned())));
        }
        CosmosPreconditionKind::CosmosPreconditionKindIfNoneMatch => {
            let etag = require_cstr(req.precondition_etag)?;
            op = op.with_precondition(Precondition::if_none_match(Etag::from(etag.to_owned())));
        }
    }

    // Patch attempts (`0` = unset).
    if req.patch_max_attempts > 0 {
        let n = NonZeroU8::new(req.patch_max_attempts)
            .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue)?;
        op = op.with_patch_max_attempts(n);
    }

    // Metrics flags (tri-state bools).
    if let Some(v) = decode_tristate_bool(req.populate_index_metrics)? {
        op = op.with_populate_index_metrics(v);
    }
    if let Some(v) = decode_tristate_bool(req.populate_query_metrics)? {
        op = op.with_populate_query_metrics(v);
    }

    Ok(op)
}

// ─────────────────────────────────────────────────────────────────────────────
// Strict-scope reference accessors
// ─────────────────────────────────────────────────────────────────────────────

fn require_account(
    req: &CosmosOperationRequest,
) -> Result<azure_data_cosmos_driver::models::AccountReference, CosmosErrorCode> {
    let inner = AccountRefHandle::from_ptr(req.account)
        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
    Ok(inner.inner.clone())
}

fn require_database(
    req: &CosmosOperationRequest,
) -> Result<azure_data_cosmos_driver::models::DatabaseReference, CosmosErrorCode> {
    let inner = DatabaseRefHandle::from_ptr(req.database)
        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
    Ok(inner.inner.clone())
}

fn require_container(req: &CosmosOperationRequest) -> Result<ContainerReference, CosmosErrorCode> {
    let inner = ContainerRefHandle::from_ptr(req.container)
        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
    Ok(inner.inner.clone())
}

fn require_partition_key(req: &CosmosOperationRequest) -> Result<PartitionKey, CosmosErrorCode> {
    // Inline tagged-union path (preferred): assemble the partition key in one
    // shot from the components carried directly on the request, with no
    // builder/handle FFI round-trips.
    if !req.partition_key_components.is_null() && req.partition_key_len > 0 {
        // SAFETY: per the `CosmosOperationRequest` contract (see
        // `build_request`), `partition_key_components` points to
        // `partition_key_len` initialized components valid for this call.
        return unsafe {
            crate::partition_key::partition_key_from_components(
                req.partition_key_components,
                req.partition_key_len,
            )
        };
    }
    // Fallback: a pre-built `cosmos_partition_key_t` handle.
    let inner = PartitionKeyHandle::from_ptr(req.partition_key)
        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
    Ok(inner.inner.clone())
}

/// Builds an [`ItemReference`] from `container` + `partition_key` + `item_id`,
/// rejecting a NULL in any of the three.
fn require_item_ref(req: &CosmosOperationRequest) -> Result<ItemReference, CosmosErrorCode> {
    let container = require_container(req)?;
    let pk = require_partition_key(req)?;
    let item_id = require_cstr(req.item_id)?;
    Ok(ItemReference::from_name(&container, pk, item_id.to_owned()))
}

// ─────────────────────────────────────────────────────────────────────────────
// String helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Borrows a NUL-terminated UTF-8 string from a non-NULL pointer.
fn cstr_to_str<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: non-NULL checked; caller guarantees NUL-terminated UTF-8.
    let cstr = unsafe { CStr::from_ptr(p) };
    cstr.to_str()
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

/// Like [`cstr_to_str`] but returns `INVALID_ARGUMENT` on NULL — used where
/// the string is mandatory for the chosen `kind` / precondition.
fn require_cstr<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    cstr_to_str(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tristate_bool_decodes_sentinels() {
        assert_eq!(decode_tristate_bool(TRISTATE_UNSET), Ok(None));
        assert_eq!(decode_tristate_bool(TRISTATE_FALSE), Ok(Some(false)));
        assert_eq!(decode_tristate_bool(TRISTATE_TRUE), Ok(Some(true)));
    }

    #[test]
    fn tristate_bool_rejects_out_of_range() {
        assert_eq!(
            decode_tristate_bool(3),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue)
        );
        assert_eq!(
            decode_tristate_bool(-1),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue)
        );
    }

    #[test]
    fn opt_u32_treats_negative_as_unset() {
        assert_eq!(decode_opt_u32(-1), None);
        assert_eq!(decode_opt_u32(i32::MIN), None);
        assert_eq!(decode_opt_u32(0), Some(0));
        assert_eq!(decode_opt_u32(7), Some(7));
        assert_eq!(decode_opt_u32(i32::MAX), Some(i32::MAX as u32));
    }

    #[test]
    fn read_consistency_strategy_maps_to_driver() {
        use CosmosReadConsistencyStrategy as S;
        assert_eq!(S::CosmosReadConsistencyStrategyUnset.to_driver(), Ok(None));
        assert_eq!(
            S::CosmosReadConsistencyStrategyDefault.to_driver(),
            Ok(Some(ReadConsistencyStrategy::Default))
        );
        assert_eq!(
            S::CosmosReadConsistencyStrategyEventual.to_driver(),
            Ok(Some(ReadConsistencyStrategy::Eventual))
        );
        assert_eq!(
            S::CosmosReadConsistencyStrategySession.to_driver(),
            Ok(Some(ReadConsistencyStrategy::Session))
        );
        assert_eq!(
            S::CosmosReadConsistencyStrategyGlobalStrong.to_driver(),
            Ok(Some(ReadConsistencyStrategy::GlobalStrong))
        );
    }

    #[test]
    fn content_response_on_write_maps_to_driver() {
        use CosmosContentResponseOnWriteOpt as C;
        assert_eq!(
            C::CosmosContentResponseOnWriteOptUnset.to_driver(),
            Ok(None)
        );
        assert_eq!(
            C::CosmosContentResponseOnWriteOptDisabled.to_driver(),
            Ok(Some(ContentResponseOnWrite::Disabled))
        );
        assert_eq!(
            C::CosmosContentResponseOnWriteOptEnabled.to_driver(),
            Ok(Some(ContentResponseOnWrite::Enabled))
        );
    }

    #[test]
    fn read_consistency_from_i32_validates_range() {
        use CosmosReadConsistencyStrategy as S;
        assert_eq!(S::from_i32(0), Ok(S::CosmosReadConsistencyStrategyUnset));
        assert_eq!(
            S::from_i32(4),
            Ok(S::CosmosReadConsistencyStrategyGlobalStrong)
        );
        assert_eq!(
            S::from_i32(5),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue)
        );
        assert_eq!(
            S::from_i32(-1),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue)
        );
    }

    #[test]
    fn content_response_from_i32_validates_range() {
        use CosmosContentResponseOnWriteOpt as C;
        assert_eq!(C::from_i32(0), Ok(C::CosmosContentResponseOnWriteOptUnset));
        assert_eq!(
            C::from_i32(2),
            Ok(C::CosmosContentResponseOnWriteOptEnabled)
        );
        assert_eq!(
            C::from_i32(3),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue)
        );
    }

    #[test]
    fn operation_kind_from_i32_validates_range() {
        use CosmosOperationKind as K;
        assert_eq!(K::from_i32(0), Ok(K::CosmosOperationKindInvalid));
        assert_eq!(K::from_i32(24), Ok(K::CosmosOperationKindPatchItem));
        assert_eq!(
            K::from_i32(25),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
        );
        assert_eq!(
            K::from_i32(-1),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
        );
    }

    #[test]
    fn precondition_kind_from_i32_validates_range() {
        use CosmosPreconditionKind as P;
        assert_eq!(P::from_i32(0), Ok(P::CosmosPreconditionKindNone));
        assert_eq!(P::from_i32(2), Ok(P::CosmosPreconditionKindIfNoneMatch));
        assert_eq!(
            P::from_i32(3),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
        );
    }

    #[test]
    fn options_default_is_all_unset() {
        let o = cosmos_operation_options_default();
        assert_eq!(
            o.read_consistency_strategy,
            CosmosReadConsistencyStrategy::CosmosReadConsistencyStrategyUnset as i32
        );
        assert_eq!(
            o.content_response_on_write,
            CosmosContentResponseOnWriteOpt::CosmosContentResponseOnWriteOptUnset as i32
        );
        assert_eq!(o.session_capturing_disabled, TRISTATE_UNSET);
        assert_eq!(o.max_failover_retry_count, -1);
        assert_eq!(o.max_session_retry_count, -1);
        assert_eq!(o.end_to_end_timeout_ms, -1);
        assert_eq!(o.endpoint_unavailability_ttl_ms, -1);
        assert!(o.throughput_control_group.is_null());
        assert!(o.excluded_regions.is_null());
        assert_eq!(o.excluded_regions_len, 0);
        assert!(o.custom_headers.is_null());
        assert_eq!(o.custom_headers_len, 0);
    }

    #[test]
    fn default_options_convert_to_empty_driver_options() {
        // An all-unset flat struct must produce a driver `OperationOptions`
        // with every field left at `None` (pure inherit).
        let o = cosmos_operation_options_default();
        // SAFETY: all pointer fields are NULL / len 0, so no dereference occurs.
        let driver = unsafe { o.to_driver() }.expect("default options convert");
        assert_eq!(driver.read_consistency_strategy, None);
        assert_eq!(driver.content_response_on_write, None);
        assert_eq!(driver.session_capturing_disabled, None);
        assert_eq!(driver.max_failover_retry_count, None);
        assert_eq!(driver.max_session_retry_count, None);
        assert_eq!(driver.end_to_end_latency_policy, None);
        assert_eq!(driver.excluded_regions, None);
        assert!(driver.throughput_control.is_none());
    }

    #[test]
    fn cstr_to_str_rejects_null() {
        assert_eq!(
            cstr_to_str(std::ptr::null()),
            Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
        );
    }
}
