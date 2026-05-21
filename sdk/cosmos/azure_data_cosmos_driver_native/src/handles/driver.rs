// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver handle (Phase 3 — skeleton).
//!
//! Wraps `Arc<azure_data_cosmos_driver::CosmosDriver>` obtained from
//! `CosmosDriverRuntime::get_or_create_driver`.

use std::sync::Arc;

use azure_data_cosmos_driver::options::OperationOptions;
use azure_data_cosmos_driver::CosmosDriver;

use crate::context::CallContext;
use crate::error::{messages, CosmosErrorCode, Error};
use crate::handles::account::cosmos_account_ref;
use crate::handles::operation::cosmos_operation;
use crate::handles::response::cosmos_response;
use crate::unwrap_required_ptr;

/// Opaque handle to a shared `CosmosDriver`.
///
/// cbindgen:ignore
#[allow(non_camel_case_types, dead_code)]
pub struct cosmos_driver(pub(crate) Arc<CosmosDriver>);

/// Obtains (or creates) a driver instance for the given account. The
/// underlying driver is cached by the runtime, so repeated calls with the
/// same account return handles to the same instance.
#[no_mangle]
pub extern "C" fn cosmos_driver_get_or_create(
    ctx: *mut CallContext,
    account: *const cosmos_account_ref,
    // TODO(phase-3): driver options handle (`*const cosmos_driver_options`).
    _options: *const std::ffi::c_void,
    out_driver: *mut *mut cosmos_driver,
) -> CosmosErrorCode {
    let ctx = context!(ctx);
    let rt = ctx.runtime().driver_runtime();
    ctx.run_async_with_output(out_driver, async {
        let account = unwrap_required_ptr(account, crate::error::messages::INVALID_HANDLE)?;
        let driver = rt.get_or_create_driver(account.0.clone(), None).await?;
        Ok(Box::new(cosmos_driver(driver)))
    })
}

/// Releases the driver handle. The underlying driver remains alive as long as
/// any other handle (or the runtime registry) holds a reference.
///
/// # Safety
/// `driver` must be null or a pointer returned by `cosmos_driver_get_or_create`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_driver_free(driver: *mut cosmos_driver) {
    if !driver.is_null() {
        drop(Box::from_raw(driver));
    }
}

/// Executes a previously-built operation and yields a response handle on
/// success. The operation handle is consumed by this call (its inner
/// `CosmosOperation` is moved out); `cosmos_operation_free` remains safe to
/// call on the now-empty handle.
///
/// Per the spec (§6) non-success HTTP statuses are NOT mapped to error codes
/// here — they are surfaced via `cosmos_response_status_code`. Only
/// transport, auth, or marshalling failures produce a non-`Success` code on
/// this call.
///
/// `options` is reserved for future use and must currently be `NULL`.
///
/// # Safety
/// `ctx`, `driver`, `op`, and `out_response` must all be non-null. `op` must
/// be a handle returned by a `cosmos_operation_*` factory and not yet
/// executed.
#[no_mangle]
pub extern "C" fn cosmos_driver_execute(
    ctx: *mut CallContext,
    driver: *const cosmos_driver,
    op: *mut cosmos_operation,
    _options: *const std::ffi::c_void,
    out_response: *mut *mut cosmos_response,
) -> CosmosErrorCode {
    let ctx = context!(ctx);
    if op.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    // Extract the operation up front (synchronously) so we don't hold a
    // mutable borrow across the await point below.
    let operation = unsafe {
        match (*op).take() {
            Some(o) => o,
            None => return CosmosErrorCode::OperationConsumed,
        }
    };

    ctx.run_async_with_output(out_response, async move {
        let driver = unwrap_required_ptr(driver, messages::INVALID_HANDLE)?;
        let resp = driver
            .0
            .execute_operation(operation, OperationOptions::default())
            .await
            .map_err(Error::from)?;
        Ok(Box::new(cosmos_response::new_inner(resp)))
    })
}
