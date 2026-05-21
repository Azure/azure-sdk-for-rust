// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver handle (Phase 3 — skeleton).
//!
//! Wraps `Arc<azure_data_cosmos_driver::CosmosDriver>` obtained from
//! `CosmosDriverRuntime::get_or_create_driver`.

use std::sync::Arc;

use azure_data_cosmos_driver::CosmosDriver;

use crate::context::CallContext;
use crate::error::CosmosErrorCode;
use crate::handles::account::cosmos_account_ref;
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
