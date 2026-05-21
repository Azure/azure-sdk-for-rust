// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation handles (Phase 5 — stub).
//!
//! Will expose every `azure_data_cosmos_driver::models::CosmosOperation::*`
//! factory and `with_*` mutator. The handle is move-only: passing it to
//! `cosmos_driver_execute` consumes it.

use azure_data_cosmos_driver::models::CosmosOperation;

/// Opaque operation handle.
///
/// cbindgen:ignore
#[allow(non_camel_case_types, dead_code)]
pub struct cosmos_operation(pub(crate) Option<CosmosOperation>);

/// Releases an operation handle. Safe to call after `cosmos_driver_execute`
/// (the handle simply holds `None` in that case).
///
/// # Safety
/// `op` must be null or a pointer returned by a `cosmos_operation_*`
/// factory.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_free(op: *mut cosmos_operation) {
    if !op.is_null() {
        drop(Box::from_raw(op));
    }
}

// TODO(phase-5): all factories (create_item / read_item / replace_item /
// upsert_item / delete_item / patch_item / read_database / delete_database /
// create_database / create_container / read_container / replace_container /
// delete_container / query_items / query_databases / query_containers /
// read_all_items / read_all_databases / read_all_containers) and the
// matching with_* mutators.
