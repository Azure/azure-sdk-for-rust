// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation handles (Phase 5 — initial set).
//!
//! Exposes a representative subset of `CosmosOperation::*` factories together
//! with the `with_*` mutators most callers need (`with_body`,
//! `with_partition_key`). The full set is enumerated in
//! `docs/NATIVE_WRAPPER_SPEC.md` §4.6 and will be filled in incrementally.

use azure_data_cosmos_driver::models::CosmosOperation;

use crate::bytes::CosmosBytesView;
use crate::error::CosmosErrorCode;
use crate::handles::partition_key::cosmos_partition_key;
use crate::handles::references::{cosmos_container_ref, cosmos_database_ref, cosmos_item_ref};

/// Opaque operation handle. Move-only: passing it to `cosmos_driver_execute`
/// consumes the inner operation. `cosmos_operation_free` is always safe to
/// call (a consumed handle simply holds `None`).
///
/// cbindgen:ignore
#[allow(non_camel_case_types)]
pub struct cosmos_operation(pub(crate) Option<CosmosOperation>);

impl cosmos_operation {
    fn boxed(op: CosmosOperation) -> *mut cosmos_operation {
        Box::into_raw(Box::new(cosmos_operation(Some(op))))
    }

    /// Takes the inner operation, leaving `None` behind. Returns `None` if
    /// the operation has already been consumed.
    pub(crate) fn take(&mut self) -> Option<CosmosOperation> {
        self.0.take()
    }
}

fn map<F>(op: *mut cosmos_operation, f: F) -> CosmosErrorCode
where
    F: FnOnce(CosmosOperation) -> CosmosOperation,
{
    if op.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }
    let slot = unsafe { &mut *op };
    match slot.0.take() {
        Some(inner) => {
            slot.0 = Some(f(inner));
            CosmosErrorCode::Success
        }
        None => CosmosErrorCode::OperationConsumed,
    }
}

// ── Database-scope factories ───────────────────────────────────────────────

/// Builds a `read_database` operation.
///
/// # Safety
/// `database` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_read_database(
    database: *const cosmos_database_ref,
) -> *mut cosmos_operation {
    if database.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::read_database((*database).0.clone()))
}

/// Builds a `delete_database` operation.
///
/// # Safety
/// `database` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_delete_database(
    database: *const cosmos_database_ref,
) -> *mut cosmos_operation {
    if database.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::delete_database((*database).0.clone()))
}

// ── Container-scope factories ──────────────────────────────────────────────

/// Builds a `read_container` operation.
///
/// # Safety
/// `container` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_read_container(
    container: *const cosmos_container_ref,
) -> *mut cosmos_operation {
    if container.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::read_container((*container).0.clone()))
}

/// Builds a `delete_container` operation.
///
/// # Safety
/// `container` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_delete_container(
    container: *const cosmos_container_ref,
) -> *mut cosmos_operation {
    if container.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::delete_container((*container).0.clone()))
}

// ── Item-scope factories ───────────────────────────────────────────────────

/// Builds a `create_item` operation. Use `cosmos_operation_with_body` to
/// attach the document JSON.
///
/// # Safety
/// `item` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_create_item(
    item: *const cosmos_item_ref,
) -> *mut cosmos_operation {
    if item.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::create_item((*item).0.clone()))
}

/// Builds a `read_item` operation.
///
/// # Safety
/// `item` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_read_item(
    item: *const cosmos_item_ref,
) -> *mut cosmos_operation {
    if item.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::read_item((*item).0.clone()))
}

/// Builds an `upsert_item` operation. Use `cosmos_operation_with_body` to
/// attach the document JSON.
///
/// # Safety
/// `item` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_upsert_item(
    item: *const cosmos_item_ref,
) -> *mut cosmos_operation {
    if item.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::upsert_item((*item).0.clone()))
}

/// Builds a `replace_item` operation. Use `cosmos_operation_with_body` to
/// attach the new document JSON.
///
/// # Safety
/// `item` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_replace_item(
    item: *const cosmos_item_ref,
) -> *mut cosmos_operation {
    if item.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::replace_item((*item).0.clone()))
}

/// Builds a `delete_item` operation.
///
/// # Safety
/// `item` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_delete_item(
    item: *const cosmos_item_ref,
) -> *mut cosmos_operation {
    if item.is_null() {
        return std::ptr::null_mut();
    }
    cosmos_operation::boxed(CosmosOperation::delete_item((*item).0.clone()))
}

// ── Mutators ───────────────────────────────────────────────────────────────

/// Attaches a request body (raw bytes). Overwrites any previous body.
///
/// The bytes are copied into the operation; the caller's buffer may be freed
/// immediately after this call returns.
///
/// # Safety
/// `op` must be non-null. `body.data` must be either null (treated as empty)
/// or point to at least `body.len` valid bytes.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_with_body(
    op: *mut cosmos_operation,
    body: CosmosBytesView,
) -> CosmosErrorCode {
    let bytes = body.as_slice().to_vec();
    map(op, |inner| inner.with_body(bytes))
}

/// Attaches (or overrides) the partition key on the operation. The partition
/// key handle is cloned; the caller retains ownership of `pk`.
///
/// # Safety
/// `op` and `pk` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_with_partition_key(
    op: *mut cosmos_operation,
    pk: *const cosmos_partition_key,
) -> CosmosErrorCode {
    if pk.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }
    let pk_clone = (*pk).0.clone();
    map(op, |inner| inner.with_partition_key(pk_clone))
}

/// Releases an operation handle. Safe to call after `cosmos_driver_execute`.
///
/// # Safety
/// `op` must be null or a pointer returned by a `cosmos_operation_*` factory.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_free(op: *mut cosmos_operation) {
    if !op.is_null() {
        drop(Box::from_raw(op));
    }
}
