// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_operation_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::CosmosOperation`].
//!
//! ## Scope (Phase 5)
//!
//! Phase 5 ships **only** the factories that do not require a resolved
//! container reference:
//!
//! - Account-scope: `create_database`, `read_all_databases`,
//!   `query_databases`, `query_offers`, `read_offer`, `replace_offer`.
//! - Database-scope: `read_database`, `delete_database`,
//!   `create_container`, `read_all_containers`, `query_containers`.
//!
//! All container-scope and item-scope factories (`read_item`,
//! `create_item`, `query_items`, `batch`, etc.) take a
//! `cosmos_container_ref_t` which does not yet exist — landing in
//! Phase 6 alongside the response surface that delivers a resolved
//! container.
//!
//! `cosmos_feed_range_*` (spec §4.6.4) is also a Phase 6 follow-up
//! since every consumer of a feed range is a container-scope factory.
//!
//! ## Lifecycle (spec §4.6.3)
//!
//! `cosmos_operation_t *` is a single-use builder + spec. The driver
//! consumes the inner `CosmosOperation` by value on submit, so the FFI
//! stores it behind `Box<Option<CosmosOperation>>` and uses
//! `Option::take` to extract it. Until Phase 6 lands the submit
//! pipeline, operations stay un-taken and are released via `_free`.
//!
//! Every mutator that targets an already-consumed handle returns
//! `OPERATION_CONSUMED` (4005) per the normative contract — even
//! though Phase 5 has no way to consume one yet, the check is wired
//! now so Phase 6 inherits it for free.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] §4.6.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CStr};
use std::num::NonZeroU32;

use azure_data_cosmos_driver::models::{
    ActivityId, CosmosOperation, ETag, MaxItemCountHint, Precondition, SessionToken,
};

use crate::account_ref::AccountRefHandle;
use crate::database_ref::DatabaseRefHandle;
use crate::error::CosmosErrorCode;

// ─────────────────────────────────────────────────────────────────────────────
// OperationHandle
//
// Storage pun mirrors the other Phase 5 handle types. The inner state
// is `Option<CosmosOperation>` to match the spec §4.6.3 contract:
// `Option::take` lets Phase 6's submit move the inner operation into
// the driver pipeline while leaving a "consumed sentinel" behind.
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct OperationInner {
    /// `None` once the operation has been consumed by a successful
    /// submit (Phase 6 mechanic) — until then, every well-formed
    /// operation is `Some(_)`. Mutators that observe `None` return
    /// `OPERATION_CONSUMED` (4005).
    pub(crate) op: Option<CosmosOperation>,
}

/// Opaque C ABI handle for a built but un-submitted operation.
///
/// Storage pun: same shape as `OperationOptionsHandle`.
#[repr(C)]
pub struct OperationDescHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct OperationDescStorage {
    _opaque: [u8; 0],
    inner: OperationInner,
}

impl OperationDescHandle {
    fn new_raw(op: CosmosOperation) -> *mut Self {
        let storage = Box::new(OperationDescStorage {
            _opaque: [],
            inner: OperationInner { op: Some(op) },
        });
        Box::into_raw(storage).cast::<OperationDescHandle>()
    }

    fn inner_mut<'a>(p: *mut OperationDescHandle) -> Option<&'a mut OperationInner> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `new_raw` and
        // has not been freed.
        let storage = unsafe { &mut *(p.cast::<OperationDescStorage>()) };
        Some(&mut storage.inner)
    }

    fn drop_raw(p: *mut OperationDescHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: pun back into the storage we originally allocated.
        unsafe {
            drop(Box::from_raw(p.cast::<OperationDescStorage>()));
        }
    }

    /// Borrows the inner state by `&self` for read-only inspection
    /// (used by tests today; Phase 6 will use it for diagnostics).
    #[allow(
        dead_code,
        reason = "first non-test caller arrives in Phase 6 (submit + diagnostics)"
    )]
    pub(crate) fn inner_ref<'a>(p: *const OperationDescHandle) -> Option<&'a OperationInner> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `new_raw` and
        // has not been freed.
        let storage = unsafe { &*(p as *const OperationDescStorage) };
        Some(&storage.inner)
    }

    /// Used by [`crate::submit::cosmos_driver_submit`] to take the
    /// inner [`CosmosOperation`] out of the FFI handle. Returns
    /// `Err(INVALID_ARGUMENT)` on NULL and `Err(OPERATION_CONSUMED)`
    /// when the operation has already been taken (re-submit attempt).
    pub(crate) fn take_inner(
        p: *mut OperationDescHandle,
    ) -> Result<CosmosOperation, CosmosErrorCode> {
        let inner = Self::inner_mut(p).ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
        inner
            .op
            .take()
            .ok_or(CosmosErrorCode::CosmosErrorCodeOperationConsumed)
    }

    /// Restores the inner [`CosmosOperation`] after a pre-flight
    /// failure in the submit pipeline. Per spec §4.6.3 #4, pre-flight
    /// rejection must NOT consume the operation; the submit code calls
    /// this when the queue check fails so the caller can mutate and
    /// retry. Has no observable effect on NULL.
    pub(crate) fn restore_inner(p: *mut OperationDescHandle, op: CosmosOperation) {
        if let Some(inner) = Self::inner_mut(p) {
            inner.op = Some(op);
        }
    }
}

/// Frees an operation handle. Idempotent only in the sense that the
/// pointer is then NULL on the caller side — double-free is undefined
/// behavior. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_operation_free(op: *mut OperationDescHandle) {
    if op.is_null() {
        return;
    }
    tracing::trace!(?op, "freeing cosmos_operation_t");
    OperationDescHandle::drop_raw(op);
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI helpers
// ─────────────────────────────────────────────────────────────────────────────

fn try_cstr_to_str<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller contract on every entry point.
    let cstr = unsafe { CStr::from_ptr(p) };
    cstr.to_str()
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

/// Helper for every factory: validates the `out_op` slot and writes the
/// allocated handle into it on success.
fn write_op_handle(out_op: *mut *mut OperationDescHandle, op: CosmosOperation) -> i32 {
    if out_op.is_null() {
        // Drop the constructed driver operation; mirrors the Phase 3
        // pattern where NULL out-slots return INVALID_ARGUMENT without
        // emitting a handle.
        drop(op);
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    // SAFETY: caller guarantees `out_op` is writable for one
    // `*mut OperationDescHandle`.
    unsafe {
        *out_op = OperationDescHandle::new_raw(op);
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Borrows the operation for a mutator call. Returns
/// `INVALID_ARGUMENT` on NULL handle and `OPERATION_CONSUMED` on a
/// consumed handle. Otherwise returns a mutable borrow of the inner
/// `CosmosOperation`.
fn mutator_pre_flight<'a>(
    op: *mut OperationDescHandle,
) -> Result<&'a mut CosmosOperation, CosmosErrorCode> {
    let inner = OperationDescHandle::inner_mut(op)
        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
    inner
        .op
        .as_mut()
        .ok_or(CosmosErrorCode::CosmosErrorCodeOperationConsumed)
}

/// Performs a take-call-restore on the inner operation slot so a
/// consuming `with_*` setter on `CosmosOperation` can be invoked
/// through a `&mut Option<CosmosOperation>` slot.
fn apply_mutator<F>(op: *mut OperationDescHandle, f: F) -> i32
where
    F: FnOnce(CosmosOperation) -> CosmosOperation,
{
    let inner = match OperationDescHandle::inner_mut(op) {
        Some(i) => i,
        None => return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32(),
    };
    let Some(taken) = inner.op.take() else {
        return CosmosErrorCode::CosmosErrorCodeOperationConsumed.as_i32();
    };
    inner.op = Some(f(taken));
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: factories (account-scope)
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! account_factory {
    ($fn_name:ident, $driver_factory:ident) => {
        /// Account-scope operation factory.
        #[no_mangle]
        pub extern "C" fn $fn_name(
            account: *const AccountRefHandle,
            out_op: *mut *mut OperationDescHandle,
        ) -> i32 {
            let Some(account_inner) = AccountRefHandle::inner_arc(account) else {
                return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
            };
            let op = CosmosOperation::$driver_factory(account_inner.inner.clone());
            write_op_handle(out_op, op)
        }
    };
}

account_factory!(cosmos_operation_create_database, create_database);
account_factory!(cosmos_operation_read_all_databases, read_all_databases);
account_factory!(cosmos_operation_query_databases, query_databases);
account_factory!(cosmos_operation_query_offers, query_offers);

/// Read a specific throughput offer by resource link.
#[no_mangle]
pub extern "C" fn cosmos_operation_read_offer(
    account: *const AccountRefHandle,
    resource_link: *const c_char,
    out_op: *mut *mut OperationDescHandle,
) -> i32 {
    let Some(account_inner) = AccountRefHandle::inner_arc(account) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let link = match try_cstr_to_str(resource_link) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    let op = CosmosOperation::read_offer(account_inner.inner.clone(), link.to_owned());
    write_op_handle(out_op, op)
}

/// Replace a throughput offer by resource link.
#[no_mangle]
pub extern "C" fn cosmos_operation_replace_offer(
    account: *const AccountRefHandle,
    resource_link: *const c_char,
    out_op: *mut *mut OperationDescHandle,
) -> i32 {
    let Some(account_inner) = AccountRefHandle::inner_arc(account) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let link = match try_cstr_to_str(resource_link) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    let op = CosmosOperation::replace_offer(account_inner.inner.clone(), link.to_owned());
    write_op_handle(out_op, op)
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: factories (database-scope)
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! database_factory {
    ($fn_name:ident, $driver_factory:ident) => {
        /// Database-scope operation factory.
        #[no_mangle]
        pub extern "C" fn $fn_name(
            database: *const DatabaseRefHandle,
            out_op: *mut *mut OperationDescHandle,
        ) -> i32 {
            let Some(db_inner) = DatabaseRefHandle::inner_arc(database) else {
                return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
            };
            let op = CosmosOperation::$driver_factory(db_inner.inner.clone());
            write_op_handle(out_op, op)
        }
    };
}

database_factory!(cosmos_operation_read_database, read_database);
database_factory!(cosmos_operation_delete_database, delete_database);
database_factory!(cosmos_operation_create_container, create_container);
database_factory!(cosmos_operation_read_all_containers, read_all_containers);
database_factory!(cosmos_operation_query_containers, query_containers);

// ─────────────────────────────────────────────────────────────────────────────
// FFI: factories (container-scope, Phase 6)
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! container_factory {
    ($fn_name:ident, $driver_factory:ident) => {
        /// Container-scope operation factory.
        #[no_mangle]
        pub extern "C" fn $fn_name(
            container: *const crate::container_ref::ContainerRefHandle,
            out_op: *mut *mut OperationDescHandle,
        ) -> i32 {
            let Some(container_inner) =
                crate::container_ref::ContainerRefHandle::inner_arc(container)
            else {
                return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
            };
            let op = CosmosOperation::$driver_factory(container_inner.inner.clone());
            write_op_handle(out_op, op)
        }
    };
}

container_factory!(cosmos_operation_read_container, read_container);
container_factory!(cosmos_operation_replace_container, replace_container);
container_factory!(cosmos_operation_delete_container, delete_container);
container_factory!(
    cosmos_operation_read_all_items_cross_partition,
    read_all_items_cross_partition
);

/// Single-partition feed read. Mirrors
/// [`CosmosOperation::read_all_items`].
#[no_mangle]
pub extern "C" fn cosmos_operation_read_all_items(
    container: *const crate::container_ref::ContainerRefHandle,
    pk: *const crate::partition_key::PartitionKeyHandle,
    out_op: *mut *mut OperationDescHandle,
) -> i32 {
    let Some(container_inner) = crate::container_ref::ContainerRefHandle::inner_arc(container)
    else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let Some(pk_inner) = crate::partition_key::PartitionKeyHandle::inner_arc(pk) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let op = CosmosOperation::read_all_items(container_inner.inner.clone(), pk_inner.inner.clone());
    write_op_handle(out_op, op)
}

/// Query operation. NULL `feed_range` targets the entire container
/// (equivalent to `cosmos_feed_range_full`).
#[no_mangle]
pub extern "C" fn cosmos_operation_query_items(
    container: *const crate::container_ref::ContainerRefHandle,
    feed_range: *const crate::feed_range::FeedRangeHandle,
    out_op: *mut *mut OperationDescHandle,
) -> i32 {
    let Some(container_inner) = crate::container_ref::ContainerRefHandle::inner_arc(container)
    else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let fr_owned = if feed_range.is_null() {
        None
    } else {
        crate::feed_range::FeedRangeHandle::inner_arc(feed_range).map(|arc| arc.inner.clone())
    };
    let op = CosmosOperation::query_items(container_inner.inner.clone(), fr_owned);
    write_op_handle(out_op, op)
}

/// Query-plan fetch. `supported_features_mask` is the comma-separated
/// header value the driver sends as
/// `x-ms-cosmos-supported-query-features`. Most language SDKs never
/// call this directly — exposed for parity with the driver surface.
///
/// **Note (Phase 6+ follow-up):** the driver crate's
/// `CosmosOperation::query_plan` exists but signature reconciliation
/// with the spec's `query_plan_for_features` is still in flight. This
/// FFI entry point reserves the name and returns
/// `INVALID_ARGUMENT` until the driver-side shape stabilises.
#[no_mangle]
pub extern "C" fn cosmos_operation_query_plan_for_features(
    container: *const crate::container_ref::ContainerRefHandle,
    supported_features_mask: *const c_char,
    out_op: *mut *mut OperationDescHandle,
) -> i32 {
    let _ = (container, supported_features_mask, out_op);
    // Mirrors a deliberate Phase 6 deferral: the driver-side shape of
    // `query_plan` does not currently match the spec's
    // `query_plan_for_features` (driver takes a typed feature set, spec
    // takes a header string). Reserve the FFI symbol so external SDKs
    // know it exists; routing lands when the driver / spec reconcile.
    CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
}

/// Transactional batch. Sub-operation accumulation lands in Phase 9;
/// Phase 6 reserves the factory.
#[no_mangle]
pub extern "C" fn cosmos_operation_batch(
    container: *const crate::container_ref::ContainerRefHandle,
    pk: *const crate::partition_key::PartitionKeyHandle,
    out_op: *mut *mut OperationDescHandle,
) -> i32 {
    let Some(container_inner) = crate::container_ref::ContainerRefHandle::inner_arc(container)
    else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let Some(pk_inner) = crate::partition_key::PartitionKeyHandle::inner_arc(pk) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let op = CosmosOperation::batch(container_inner.inner.clone(), pk_inner.inner.clone());
    write_op_handle(out_op, op)
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: factories (item-scope, Phase 6)
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! item_factory {
    ($fn_name:ident, $driver_factory:ident) => {
        /// Item-scope operation factory. The partition key is baked
        /// into the underlying `ItemReference` at construction; both
        /// the container ref and the partition key are cloned into the
        /// operation, so callers may free both immediately after this
        /// call returns.
        #[no_mangle]
        pub extern "C" fn $fn_name(
            container: *const crate::container_ref::ContainerRefHandle,
            item_id: *const c_char,
            pk: *const crate::partition_key::PartitionKeyHandle,
            out_op: *mut *mut OperationDescHandle,
        ) -> i32 {
            let Some(container_inner) =
                crate::container_ref::ContainerRefHandle::inner_arc(container)
            else {
                return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
            };
            let item_id_str = match try_cstr_to_str(item_id) {
                Ok(s) => s,
                Err(code) => return code.as_i32(),
            };
            let Some(pk_inner) = crate::partition_key::PartitionKeyHandle::inner_arc(pk) else {
                return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
            };
            let item_ref = azure_data_cosmos_driver::models::ItemReference::from_name(
                &container_inner.inner,
                pk_inner.inner.clone(),
                item_id_str.to_owned(),
            );
            let op = CosmosOperation::$driver_factory(item_ref);
            write_op_handle(out_op, op)
        }
    };
}

item_factory!(cosmos_operation_create_item, create_item);
item_factory!(cosmos_operation_read_item, read_item);
item_factory!(cosmos_operation_upsert_item, upsert_item);
item_factory!(cosmos_operation_replace_item, replace_item);
item_factory!(cosmos_operation_delete_item, delete_item);
item_factory!(cosmos_operation_patch_item, patch_item);

// ─────────────────────────────────────────────────────────────────────────────
// FFI: mutators
// ─────────────────────────────────────────────────────────────────────────────

/// Sets the request body (raw UTF-8 JSON bytes). The wrapper **copies**
/// the bytes into a driver-owned `Vec<u8>` before returning; the caller
/// may free the source buffer immediately. NULL `body` with `body_len == 0`
/// is accepted and sets an empty body.
///
/// Per spec §4.6.2: replaces any previously-set body.
#[no_mangle]
pub extern "C" fn cosmos_operation_with_body(
    op: *mut OperationDescHandle,
    body: *const u8,
    body_len: usize,
) -> i32 {
    if body.is_null() && body_len > 0 {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let owned: Vec<u8> = if body_len == 0 {
        Vec::new()
    } else {
        // SAFETY: `body` is non-NULL when `body_len > 0` and the caller
        // guarantees the buffer has at least `body_len` valid bytes.
        unsafe { std::slice::from_raw_parts(body, body_len) }.to_vec()
    };
    apply_mutator(op, move |o| o.with_body(owned))
}

// NOTE (Phase 5+ follow-up): the driver's `CosmosRequestHeaders` is a
// typed whitelist (activity_id, session_token, precondition,
// max_item_count, etc.) with no slot for arbitrary custom headers.
// Custom request headers therefore live on `OperationOptions` and
// are set via `cosmos_operation_options_builder_set_custom_header`
// + `cosmos_operation_options_builder_with_*` resolution. A
// `cosmos_operation_with_request_header` shim that routes well-known
// names through to the typed fields (and rejects everything else) is
// deferred until the spec is reconciled with the actual driver
// surface.
//
// Body — see `cosmos_operation_with_body` above.

/// Sets the session token (drives session-consistency reads).
#[no_mangle]
pub extern "C" fn cosmos_operation_with_session_token(
    op: *mut OperationDescHandle,
    token: *const c_char,
) -> i32 {
    let s = match try_cstr_to_str(token) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    let session = SessionToken(std::borrow::Cow::Owned(s.to_owned()));
    apply_mutator(op, move |o| o.with_session_token(session))
}

/// Sets the activity-id (correlates client-side logs with server-side
/// traces).
#[no_mangle]
pub extern "C" fn cosmos_operation_with_activity_id(
    op: *mut OperationDescHandle,
    activity_id: *const c_char,
) -> i32 {
    let s = match try_cstr_to_str(activity_id) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    let id = ActivityId::from_string(s.to_owned());
    apply_mutator(op, move |o| o.with_activity_id(id))
}

/// Sets the maximum item count for feed-style operations
/// (`x-ms-max-item-count`). Pass `-1` to let the server decide; any
/// non-negative value caps the page at that many items. `0` is
/// rejected as `INVALID_OPTION_VALUE` (the driver enforces
/// `NonZeroU32` internally).
#[no_mangle]
pub extern "C" fn cosmos_operation_with_max_item_count(
    op: *mut OperationDescHandle,
    max_item_count: i32,
) -> i32 {
    let hint = if max_item_count < 0 {
        MaxItemCountHint::ServerDecides
    } else {
        let Some(nz) = NonZeroU32::new(max_item_count as u32) else {
            return CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32();
        };
        MaxItemCountHint::Limit(nz)
    };
    apply_mutator(op, move |o| o.with_max_item_count(hint))
}

/// Enables / disables the `populateIndexMetrics` request header.
#[no_mangle]
pub extern "C" fn cosmos_operation_with_populate_index_metrics(
    op: *mut OperationDescHandle,
    enabled: bool,
) -> i32 {
    apply_mutator(op, move |o| o.with_populate_index_metrics(enabled))
}

/// Enables / disables the `populateQueryMetrics` request header.
#[no_mangle]
pub extern "C" fn cosmos_operation_with_populate_query_metrics(
    op: *mut OperationDescHandle,
    enabled: bool,
) -> i32 {
    apply_mutator(op, move |o| o.with_populate_query_metrics(enabled))
}

/// Sets an `If-Match` precondition. Calling either precondition setter
/// twice (or both) on the same operation returns
/// `PRECONDITION_ALREADY_SET` (4008) per spec §4.6.2.
#[no_mangle]
pub extern "C" fn cosmos_operation_with_precondition_if_match(
    op: *mut OperationDescHandle,
    etag: *const c_char,
) -> i32 {
    with_precondition(op, etag, |e| Precondition::if_match(e))
}

/// Sets an `If-None-Match` precondition.
#[no_mangle]
pub extern "C" fn cosmos_operation_with_precondition_if_none_match(
    op: *mut OperationDescHandle,
    etag: *const c_char,
) -> i32 {
    with_precondition(op, etag, |e| Precondition::if_none_match(e))
}

fn with_precondition<F>(op: *mut OperationDescHandle, etag: *const c_char, construct: F) -> i32
where
    F: FnOnce(ETag) -> Precondition + Send + 'static,
{
    let s = match try_cstr_to_str(etag) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    // Spec §4.6.2 invariant: setting a precondition replaces nothing —
    // a second setter must return PRECONDITION_ALREADY_SET. Check first
    // through an immutable borrow, then mutate.
    let existing = match mutator_pre_flight(op) {
        Ok(inner) => inner.precondition().is_some(),
        Err(code) => return code.as_i32(),
    };
    if existing {
        return CosmosErrorCode::CosmosErrorCodePreconditionAlreadySet.as_i32();
    }
    let etag = ETag::new(s.to_owned());
    apply_mutator(op, move |o| o.with_precondition(construct(etag)))
}

/// Caps the number of Read-Modify-Write attempts the patch handler
/// may make for a `patch_item` operation. `max_attempts == 0` returns
/// `INVALID_ARGUMENT`; calling on a non-patch operation returns
/// `UNSUPPORTED_OPERATION_FOR_MUTATOR` (4009).
#[no_mangle]
pub extern "C" fn cosmos_operation_with_patch_max_attempts(
    op: *mut OperationDescHandle,
    max_attempts: u8,
) -> i32 {
    if max_attempts == 0 {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(nz) = std::num::NonZeroU8::new(max_attempts) else {
        // Unreachable given the zero-check above, but kept defensive.
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    // Verify the operation is a patch before mutating. The driver
    // accepts the setter on any operation but silently ignores it for
    // non-patch kinds; the FFI surface rejects up-front so callers see
    // a deterministic error.
    let is_patch = match mutator_pre_flight(op) {
        Ok(inner) => {
            inner.operation_type() == azure_data_cosmos_driver::models::OperationType::Patch
        }
        Err(code) => return code.as_i32(),
    };
    if !is_patch {
        return CosmosErrorCode::CosmosErrorCodeUnsupportedOperationForMutator.as_i32();
    }
    apply_mutator(op, move |o| o.with_patch_max_attempts(nz))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn ok_cstr(s: &str) -> CString {
        CString::new(s).expect("test inputs must be NUL-free")
    }

    fn make_account() -> *mut AccountRefHandle {
        crate::account_ref::tests::make_master_key_handle(
            "https://myaccount.documents.azure.com:443/",
            "fake-master-key",
        )
    }

    fn make_database() -> (*mut AccountRefHandle, *mut DatabaseRefHandle) {
        let account = make_account();
        let mut db: *mut DatabaseRefHandle = ptr::null_mut();
        let db_id = ok_cstr("mydb");
        crate::database_ref::cosmos_database_ref_create(account, db_id.as_ptr(), &mut db);
        (account, db)
    }

    #[test]
    fn free_handles_null() {
        cosmos_operation_free(ptr::null_mut());
    }

    #[test]
    fn create_database_factory() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        let rc = cosmos_operation_create_database(account, &mut op);
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        assert!(!op.is_null());

        // Wire-shape comparison: the constructed op matches what the
        // driver builds directly.
        let driver_built = {
            let arc = crate::account_ref::AccountRefHandle::inner_arc(account).unwrap();
            CosmosOperation::create_database(arc.inner.clone())
        };
        let our_built = OperationDescHandle::inner_ref(op).unwrap();
        let ours = our_built.op.as_ref().unwrap();
        assert_eq!(ours.operation_type(), driver_built.operation_type());
        assert_eq!(ours.resource_type(), driver_built.resource_type());

        cosmos_operation_free(op);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn factories_reject_null_arguments() {
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        assert_eq!(
            cosmos_operation_create_database(ptr::null(), &mut op),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        let account = make_account();
        assert_eq!(
            cosmos_operation_create_database(account, ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn database_factories() {
        let (account, db) = make_database();
        for f in [
            cosmos_operation_read_database as unsafe extern "C" fn(_, _) -> _,
            cosmos_operation_delete_database,
            cosmos_operation_create_container,
            cosmos_operation_read_all_containers,
            cosmos_operation_query_containers,
        ] {
            let mut op: *mut OperationDescHandle = ptr::null_mut();
            let rc = unsafe { f(db, &mut op) };
            assert_eq!(
                rc,
                CosmosErrorCode::CosmosErrorCodeSuccess.as_i32(),
                "factory returned non-success"
            );
            assert!(!op.is_null());
            cosmos_operation_free(op);
        }
        crate::database_ref::cosmos_database_ref_free(db);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn read_offer_validates_resource_link() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        let link = ok_cstr("offers/abc");
        assert_eq!(
            cosmos_operation_read_offer(account, link.as_ptr(), &mut op),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        cosmos_operation_free(op);

        // NULL resource_link rejected.
        assert_eq!(
            cosmos_operation_read_offer(account, ptr::null(), &mut op),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_body_round_trip() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        cosmos_operation_create_database(account, &mut op);

        let payload = b"{\"id\":\"db1\"}";
        let rc = cosmos_operation_with_body(op, payload.as_ptr(), payload.len());
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());

        let inner = OperationDescHandle::inner_ref(op).unwrap();
        assert_eq!(inner.op.as_ref().unwrap().body(), Some(payload.as_ref()));

        cosmos_operation_free(op);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_body_zero_len_accepted() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        cosmos_operation_create_database(account, &mut op);
        assert_eq!(
            cosmos_operation_with_body(op, ptr::null(), 0),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        cosmos_operation_free(op);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_body_null_with_nonzero_len_rejected() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        cosmos_operation_create_database(account, &mut op);
        assert_eq!(
            cosmos_operation_with_body(op, ptr::null(), 1),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        cosmos_operation_free(op);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_request_header_validates() {
        // Phase 5+ follow-up — cosmos_operation_with_request_header is
        // not wired (see module docs). Custom request headers are
        // exercised on cosmos_operation_options_set_custom_header in
        // the operation_options module's own tests.
    }

    #[test]
    fn with_max_item_count_handles_negative_zero_and_positive() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        cosmos_operation_create_database(account, &mut op);

        // Negative → ServerDecides.
        assert_eq!(
            cosmos_operation_with_max_item_count(op, -1),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        // Positive non-zero → Limit.
        assert_eq!(
            cosmos_operation_with_max_item_count(op, 42),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        // Zero → INVALID_OPTION_VALUE (driver enforces NonZeroU32).
        assert_eq!(
            cosmos_operation_with_max_item_count(op, 0),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );

        cosmos_operation_free(op);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_session_token_and_activity_id() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        cosmos_operation_create_database(account, &mut op);

        let token = ok_cstr("0:1#100");
        let activity = ok_cstr("11111111-1111-1111-1111-111111111111");
        assert_eq!(
            cosmos_operation_with_session_token(op, token.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_operation_with_activity_id(op, activity.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );

        cosmos_operation_free(op);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_precondition_double_set_rejected() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        cosmos_operation_create_database(account, &mut op);

        let e1 = ok_cstr("\"etag-1\"");
        let e2 = ok_cstr("\"etag-2\"");

        assert_eq!(
            cosmos_operation_with_precondition_if_match(op, e1.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        // Second setter — same kind — rejected.
        assert_eq!(
            cosmos_operation_with_precondition_if_match(op, e2.as_ptr()),
            CosmosErrorCode::CosmosErrorCodePreconditionAlreadySet.as_i32()
        );
        // Different kind — also rejected.
        assert_eq!(
            cosmos_operation_with_precondition_if_none_match(op, e2.as_ptr()),
            CosmosErrorCode::CosmosErrorCodePreconditionAlreadySet.as_i32()
        );

        cosmos_operation_free(op);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn populate_metrics_setters() {
        let account = make_account();
        let mut op: *mut OperationDescHandle = ptr::null_mut();
        cosmos_operation_create_database(account, &mut op);

        assert_eq!(
            cosmos_operation_with_populate_index_metrics(op, true),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_operation_with_populate_query_metrics(op, false),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );

        cosmos_operation_free(op);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn mutators_reject_null_operation() {
        let s = ok_cstr("x");
        assert_eq!(
            cosmos_operation_with_body(ptr::null_mut(), ptr::null(), 0),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_operation_with_session_token(ptr::null_mut(), s.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_operation_with_max_item_count(ptr::null_mut(), 1),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }
}
