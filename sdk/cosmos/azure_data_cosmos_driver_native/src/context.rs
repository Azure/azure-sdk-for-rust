// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`CallContext`] — per-call glue that carries the runtime pointer and
//! captures the most recent error.

use crate::error::{messages, CosmosError, CosmosErrorCode, Error};
use crate::runtime::RuntimeContext;

/// Options used when creating a [`CallContext`] through
/// [`cosmos_call_context_create`].
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CallContextOptions {
    /// When true, error responses include a heap-allocated detail string in
    /// [`CosmosError::detail`] that the caller must free.
    pub include_error_details: bool,
}

/// Per-call context. May be stack-allocated by the caller (it is `#[repr(C)]`
/// and POD), or heap-allocated via [`cosmos_call_context_create`].
///
/// A single context may be reused across calls but is NOT thread-safe; use
/// one per caller thread.
#[repr(C)]
pub struct CallContext {
    /// Required: pointer to a [`RuntimeContext`] returned by
    /// `cosmos_runtime_create`.
    pub runtime: *const RuntimeContext,
    /// See [`CallContextOptions::include_error_details`].
    pub include_error_details: bool,
    /// Receives the most recent error. Ignored on input.
    pub error: CosmosError,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            runtime: std::ptr::null(),
            include_error_details: false,
            error: CosmosError::default(),
        }
    }
}

/// Heap-allocates a new [`CallContext`]. Release with
/// [`cosmos_call_context_free`].
#[no_mangle]
pub extern "C" fn cosmos_call_context_create(
    runtime: *const RuntimeContext,
    options: *const CallContextOptions,
) -> *mut CallContext {
    let opts = if options.is_null() {
        CallContextOptions::default()
    } else {
        unsafe { *options }
    };
    let ctx = CallContext {
        runtime,
        include_error_details: opts.include_error_details,
        error: CosmosError {
            code: CosmosErrorCode::Success,
            message: messages::OPERATION_SUCCEEDED.as_ptr(),
            detail: std::ptr::null(),
        },
    };
    Box::into_raw(Box::new(ctx))
}

/// Releases a [`CallContext`] created by [`cosmos_call_context_create`].
///
/// # Safety
/// `ctx` must be either null or a pointer previously returned by
/// `cosmos_call_context_create`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_call_context_free(ctx: *mut CallContext) {
    if !ctx.is_null() {
        drop(Box::from_raw(ctx));
    }
}

impl CallContext {
    /// Recovers a `&mut CallContext` from a raw pointer. Caller must have
    /// validated the pointer (e.g. via [`context!`](crate::context)).
    pub fn from_ptr<'a>(ptr: *mut CallContext) -> &'a mut CallContext {
        debug_assert!(!ptr.is_null());
        unsafe { &mut *ptr }
    }

    /// Returns the runtime referenced by this context. Panics if the runtime
    /// pointer is null (callers go through the [`context!`] macro which
    /// validates this before invoking any method).
    pub fn runtime(&self) -> &RuntimeContext {
        assert!(!self.runtime.is_null(), "runtime pointer is null");
        unsafe { &*self.runtime }
    }

    /// Runs a synchronous closure, capturing any error into `self.error`.
    pub fn run_sync(&mut self, f: impl FnOnce() -> Result<(), Error>) -> CosmosErrorCode {
        match f() {
            Ok(()) => {
                self.error = CosmosError::SUCCESS;
                CosmosErrorCode::Success
            }
            Err(e) => self.set_error(e),
        }
    }

    /// Runs a synchronous closure that produces a value, writing the value
    /// through `out` on success.
    pub fn run_sync_with_output<T: IntoRaw>(
        &mut self,
        out: *mut T::Output,
        f: impl FnOnce() -> Result<T, Error>,
    ) -> CosmosErrorCode {
        if out.is_null() {
            return self.set_error(Error::new(
                CosmosErrorCode::InvalidArgument,
                messages::NULL_OUTPUT_POINTER,
            ));
        }
        match f() {
            Ok(v) => {
                unsafe { *out = v.into_raw() };
                self.error = CosmosError::SUCCESS;
                CosmosErrorCode::Success
            }
            Err(e) => self.set_error(e),
        }
    }

    /// Runs an asynchronous future on the runtime's executor.
    pub fn run_async(
        &mut self,
        f: impl std::future::Future<Output = Result<(), Error>>,
    ) -> CosmosErrorCode {
        let r = self.runtime().block_on(f);
        match r {
            Ok(()) => {
                self.error = CosmosError::SUCCESS;
                CosmosErrorCode::Success
            }
            Err(e) => self.set_error(e),
        }
    }

    /// Runs an asynchronous future on the runtime's executor and writes its
    /// result through `out` on success.
    pub fn run_async_with_output<T: IntoRaw>(
        &mut self,
        out: *mut T::Output,
        f: impl std::future::Future<Output = Result<T, Error>>,
    ) -> CosmosErrorCode {
        if out.is_null() {
            return self.set_error(Error::new(
                CosmosErrorCode::InvalidArgument,
                messages::NULL_OUTPUT_POINTER,
            ));
        }
        let r = self.runtime().block_on(f);
        match r {
            Ok(v) => {
                unsafe { *out = v.into_raw() };
                self.error = CosmosError::SUCCESS;
                CosmosErrorCode::Success
            }
            Err(e) => self.set_error(e),
        }
    }

    fn set_error(&mut self, err: Error) -> CosmosErrorCode {
        let ffi = err.into_ffi(self.include_error_details);
        let code = ffi.code;
        self.error = ffi;
        code
    }
}

/// Validates that the `*mut CallContext` argument is non-null and references a
/// non-null runtime, returning the appropriate error code otherwise.
#[macro_export]
macro_rules! context {
    ($ptr:expr) => {{
        if $ptr.is_null() {
            return $crate::error::CosmosErrorCode::CallContextMissing;
        }
        let ctx = $crate::context::CallContext::from_ptr($ptr);
        if ctx.runtime.is_null() {
            return $crate::error::CosmosErrorCode::RuntimeContextMissing;
        }
        ctx
    }};
}

/// Trait for converting Rust values into raw FFI representations.
pub trait IntoRaw {
    type Output;
    fn into_raw(self) -> Self::Output;
}

impl<T> IntoRaw for Box<T> {
    type Output = *mut T;
    fn into_raw(self) -> *mut T {
        Box::into_raw(self)
    }
}

impl IntoRaw for std::ffi::CString {
    type Output = *const std::ffi::c_char;
    fn into_raw(self) -> *const std::ffi::c_char {
        self.into_raw()
    }
}

impl IntoRaw for crate::bytes::CosmosBytes {
    type Output = crate::bytes::CosmosBytes;
    fn into_raw(self) -> Self {
        self
    }
}
