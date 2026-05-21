// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Database, container, and item reference handles.
//!
//! - `cosmos_database_ref` is a pure value type (no network) built from an
//!   account + database name.
//! - `cosmos_container_ref` requires a network round trip to resolve container
//!   metadata (partition key definition, RIDs); it is built via
//!   `cosmos_container_ref_resolve` against an initialized driver.
//! - `cosmos_item_ref` is a pure value type built from a container reference,
//!   partition key, and item id.

use std::os::raw::c_char;

use azure_data_cosmos_driver::models::{
    ContainerReference, DatabaseReference, ItemReference, PartitionKey,
};

use crate::context::CallContext;
use crate::error::{messages, CosmosErrorCode, Error};
use crate::handles::account::cosmos_account_ref;
use crate::handles::driver::cosmos_driver;
use crate::handles::partition_key::cosmos_partition_key;
use crate::string::parse_cstr;
use crate::unwrap_required_ptr;

// ── DatabaseReference ──────────────────────────────────────────────────────

/// Opaque database reference. Cheap value type, no network access.
///
/// cbindgen:ignore
#[allow(non_camel_case_types)]
pub struct cosmos_database_ref(pub(crate) DatabaseReference);

/// Builds a name-based database reference from an account and database id.
///
/// # Safety
/// `account` and `database_id` must be non-null; `database_id` must be a
/// NUL-terminated UTF-8 C string. `out_db` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_database_ref_create(
    account: *const cosmos_account_ref,
    database_id: *const c_char,
    out_db: *mut *mut cosmos_database_ref,
) -> CosmosErrorCode {
    if out_db.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }
    let account = match unwrap_required_ptr(account, messages::INVALID_ACCOUNT_REFERENCE) {
        Ok(a) => a,
        Err(_) => return CosmosErrorCode::InvalidAccountReference,
    };
    let id = match parse_cstr(database_id, messages::INVALID_DATABASE_ID) {
        Ok(s) => s.to_string(),
        Err(_) => return CosmosErrorCode::InvalidArgument,
    };
    let db = DatabaseReference::from_name(account.0.clone(), id);
    *out_db = Box::into_raw(Box::new(cosmos_database_ref(db)));
    CosmosErrorCode::Success
}

/// Releases a database reference handle.
///
/// # Safety
/// `db` must be null or a pointer returned by `cosmos_database_ref_create`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_database_ref_free(db: *mut cosmos_database_ref) {
    if !db.is_null() {
        drop(Box::from_raw(db));
    }
}

// ── ContainerReference ─────────────────────────────────────────────────────

/// Opaque, fully-resolved container reference. Built via
/// [`cosmos_container_ref_resolve`] — which performs a network round trip the
/// first time and caches the result inside the driver.
///
/// cbindgen:ignore
#[allow(non_camel_case_types)]
pub struct cosmos_container_ref(pub(crate) ContainerReference);

/// Resolves a container reference by database + container name. May perform a
/// network call to fetch container metadata (partition key definition, RIDs).
///
/// # Safety
/// All pointer arguments must be non-null; the string arguments must be
/// NUL-terminated UTF-8.
#[no_mangle]
pub extern "C" fn cosmos_container_ref_resolve(
    ctx: *mut CallContext,
    driver: *const cosmos_driver,
    database_id: *const c_char,
    container_id: *const c_char,
    out_container: *mut *mut cosmos_container_ref,
) -> CosmosErrorCode {
    let ctx = context!(ctx);
    ctx.run_async_with_output(out_container, async {
        let driver = unwrap_required_ptr(driver, messages::INVALID_HANDLE)?;
        let db_id = parse_cstr(database_id, messages::INVALID_DATABASE_ID)?;
        let coll_id = parse_cstr(container_id, messages::INVALID_CONTAINER_ID)?;
        let container = driver.0.resolve_container(db_id, coll_id).await?;
        Ok(Box::new(cosmos_container_ref(container)))
    })
}

/// Releases a container reference handle.
///
/// # Safety
/// `c` must be null or a pointer returned by `cosmos_container_ref_resolve`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_container_ref_free(c: *mut cosmos_container_ref) {
    if !c.is_null() {
        drop(Box::from_raw(c));
    }
}

// ── ItemReference ──────────────────────────────────────────────────────────

/// Opaque item reference. Cheap value type built from a resolved container
/// reference, a partition key, and an item id.
///
/// cbindgen:ignore
#[allow(non_camel_case_types)]
pub struct cosmos_item_ref(pub(crate) ItemReference);

/// Builds a name-based item reference. The partition key handle is cloned
/// into the reference; the caller retains ownership of `pk` and must free it
/// separately.
///
/// # Safety
/// All pointer arguments must be non-null; `item_id` must be NUL-terminated
/// UTF-8.
#[no_mangle]
pub unsafe extern "C" fn cosmos_item_ref_create(
    container: *const cosmos_container_ref,
    pk: *const cosmos_partition_key,
    item_id: *const c_char,
    out_item: *mut *mut cosmos_item_ref,
) -> CosmosErrorCode {
    if out_item.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }
    let container = match unwrap_required_ptr::<cosmos_container_ref>(container, messages::INVALID_HANDLE) {
        Ok(c) => c,
        Err(_) => return CosmosErrorCode::InvalidHandle,
    };
    let pk: PartitionKey = if pk.is_null() {
        PartitionKey::EMPTY
    } else {
        (*pk).0.clone()
    };
    let id = match parse_cstr(item_id, messages::INVALID_ITEM_ID) {
        Ok(s) => s.to_string(),
        Err(_) => return CosmosErrorCode::InvalidArgument,
    };
    let item = ItemReference::from_name(&container.0, pk, id);
    *out_item = Box::into_raw(Box::new(cosmos_item_ref(item)));
    CosmosErrorCode::Success
}

/// Releases an item reference handle.
///
/// # Safety
/// `item` must be null or a pointer returned by `cosmos_item_ref_create`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_item_ref_free(item: *mut cosmos_item_ref) {
    if !item.is_null() {
        drop(Box::from_raw(item));
    }
}

// Suppress an unused-import warning while only a subset of error helpers are
// wired up.
#[allow(dead_code)]
fn _silence(_: Error) {}
