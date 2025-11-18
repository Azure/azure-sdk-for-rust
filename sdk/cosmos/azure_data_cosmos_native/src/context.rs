// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    error::{CosmosError, CosmosErrorCode, Error},
    runtime::RuntimeContext,
};

#[repr(C)]
#[derive(Default)]
pub struct CallContextOptions {
    pub include_error_details: bool,
}

/// Represents the context for a call into the Cosmos DB native SDK.
///
/// This structure can be created on the caller side, as long as the caller is able to create a C-compatible struct.
/// The `runtime_context` field must be set to a pointer to a `RuntimeContext` created by the
/// [`cosmos_runtime_context_create`](crate::runtime::cosmos_runtime_context_create) function.
///
/// The structure can also be created using [`cosmos_call_context_create`](crate::context::cosmos_call_context_create),
/// in which case Rust will manage the memory for the structure, and it must be freed using [`cosmos_call_context_free`](crate::context::cosmos_call_context_free).
///
/// This structure must remain active and at the memory address specified in the function call for the duration of the call into the SDK.
/// If calling an async function, that may mean it must be allocated on the heap to ensure it remains live (depending on the caller's language/runtime).
///
/// A single [`CallContext`] may be reused for muliple calls, but cannot be used concurrently from multiple threads.
/// When reusing a [`CallContext`] the [`CallContext::error`] field will be overwritten with the error from the most recent call.
/// Error details will NOT be freed if the context is reused; the caller is responsible for freeing any error details if needed.
#[repr(C)]
#[derive(Default)]
pub struct CallContext {
    /// Pointer to a RuntimeContext created by [`cosmos_runtime_context_create`](crate::runtime::cosmos_runtime_context_create).
    pub runtime_context: *const RuntimeContext,

    /// Indicates whether detailed case-specific error information should be included in error responses.
    ///
    /// Normally, a [`CosmosError`] contains only a static error message, which does not need to be freed.
    /// However, this also means that the error message may not contain detailed information about the specific error that occurred.
    /// If this field is set to true, the SDK will allocate a detailed error message string for each error that occurs,
    /// which must be freed by the caller using [`cosmos_string_free`](crate::string::cosmos_string_free) after each error is handled.
    pub include_error_details: bool,

    /// Holds the error information for the last operation performed using this context.
    ///
    /// The value of this is ignored on input; it is only set by the SDK to report errors.
    /// The [`CosmosError::code`] field will always match the returned error code from the function.
    /// The string associated with the error (if any) will be allocated by the SDK and must be freed
    /// by the caller using the appropriate function.
    pub error: CosmosError,
}

/// Creates a new [`CallContext`] and returns a pointer to it.
/// This must be freed using [`cosmos_call_context_free`] when no longer needed.
///
/// A [`CallContext`] may be reused for multiple calls, but cannot be used concurrently from multiple threads.
///
/// # Arguments
/// * `runtime` - Pointer to a [`RuntimeContext`] created by [`cosmos_runtime_context_create`](crate::runtime::cosmos_runtime_context_create).
/// * `options` - Pointer to [`CallContextOptions`] for call configuration, may be null.
#[no_mangle]
pub extern "C" fn cosmos_call_context_create(
    runtime_context: *const RuntimeContext,
    options: *const CallContextOptions,
) -> *mut CallContext {
    let options = if options.is_null() {
        &CallContextOptions::default()
    } else {
        unsafe { &*options }
    };
    let ctx = CallContext {
        runtime_context,
        include_error_details: options.include_error_details,
        error: CosmosError {
            code: CosmosErrorCode::Success,
            message: crate::error::messages::OPERATION_SUCCEEDED.as_ptr(),
            detail: std::ptr::null(),
        },
    };
    let ptr = Box::into_raw(Box::new(ctx));
    tracing::trace!(?ptr, "created call context");
    ptr
}

/// Frees a [`CallContext`] created by [`cosmos_call_context_create`].
#[no_mangle]
pub extern "C" fn cosmos_call_context_free(ctx: *mut CallContext) {
    if !ctx.is_null() {
        tracing::trace!(?ctx, "freeing call context");
        unsafe { drop(Box::from_raw(ctx)) }
    }
}

impl CallContext {
    pub fn from_ptr<'a>(ptr: *mut CallContext) -> &'a mut CallContext {
        debug_assert!(!ptr.is_null());
        unsafe { &mut *ptr }
    }

    pub fn runtime(&mut self) -> &crate::runtime::RuntimeContext {
        assert!(!self.runtime_context.is_null());
        unsafe { &*self.runtime_context }
    }

    /// Runs a synchronous operation with no outputs, capturing any error into the CallContext.
    pub fn run_sync(&mut self, f: impl FnOnce() -> Result<(), Error>) -> CosmosErrorCode {
        tracing::trace!("starting sync operation");
        let r = f();
        tracing::trace!("sync operation complete");
        match r {
            Ok(()) => {
                self.error = Error::SUCCESS.into_ffi(self.include_error_details);
                CosmosErrorCode::Success
            }
            Err(err) => self.set_error_and_return_code(err),
        }
    }

    /// Runs a synchronous operation with a single output, capturing any error into the CallContext.
    pub fn run_sync_with_output<T: IntoRaw>(
        &mut self,
        out: *mut T::Output,
        f: impl FnOnce() -> Result<T, Error>,
    ) -> CosmosErrorCode {
        if out.is_null() {
            self.error = Error::new(
                CosmosErrorCode::InvalidArgument,
                crate::error::messages::NULL_OUTPUT_POINTER,
            )
            .into_ffi(self.include_error_details);
            return CosmosErrorCode::InvalidArgument;
        }

        tracing::trace!("starting sync operation");
        let r = f();
        tracing::trace!("sync operation complete");
        match r {
            Ok(value) => {
                unsafe {
                    *out = value.into_raw();
                }
                self.error = Error::SUCCESS.into_ffi(self.include_error_details);
                CosmosErrorCode::Success
            }
            Err(err) => self.set_error_and_return_code(err),
        }
    }

    /// Runs an asynchronous operation with no outputs, capturing any error into the CallContext.
    pub fn run_async(
        &mut self,
        f: impl std::future::Future<Output = Result<(), Error>>,
    ) -> CosmosErrorCode {
        tracing::trace!("starting async operation");
        let r = self.runtime().block_on(f);
        tracing::trace!("async operation complete");
        match r {
            Ok(()) => {
                self.error = Error::SUCCESS.into_ffi(self.include_error_details);
                CosmosErrorCode::Success
            }
            Err(err) => self.set_error_and_return_code(err),
        }
    }

    /// Runs an asynchronous operation with a single output, capturing any error into the CallContext.
    pub fn run_async_with_output<T: IntoRaw>(
        &mut self,
        out: *mut T::Output,
        f: impl std::future::Future<Output = Result<T, Error>>,
    ) -> CosmosErrorCode {
        if out.is_null() {
            self.error = Error::new(
                CosmosErrorCode::InvalidArgument,
                crate::error::messages::NULL_OUTPUT_POINTER,
            )
            .into_ffi(self.include_error_details);
            return CosmosErrorCode::InvalidArgument;
        }

        tracing::trace!("starting async operation");
        let r = self.runtime().block_on(f);
        tracing::trace!("async operation complete");
        match r {
            Ok(value) => {
                unsafe {
                    *out = value.into_raw();
                }
                self.error = Error::SUCCESS.into_ffi(self.include_error_details);
                CosmosErrorCode::Success
            }
            Err(err) => self.set_error_and_return_code(err),
        }
    }

    fn set_error_and_return_code(&mut self, err: Error) -> CosmosErrorCode {
        tracing::error!(%err, "operation failed, preparing error for FFI");
        let err = err.into_ffi(self.include_error_details);
        let code = err.code;
        self.error = err;
        code
    }
}

#[macro_export]
macro_rules! context {
    ($param: expr) => {
        if $param.is_null() {
            tracing::error!("call context pointer is null");
            return $crate::error::CosmosErrorCode::CallContextMissing;
        } else {
            let ctx = $crate::context::CallContext::from_ptr($param);
            if ctx.runtime_context.is_null() {
                tracing::error!(call_context_pointer = ?$param, "call context has null runtime pointer");
                return $crate::error::CosmosErrorCode::RuntimeContextMissing;
            } else {
                tracing::trace!(
                    runtime_pointer = ?ctx.runtime_context,
                    call_context_pointer = ?$param,
                    "restored call context from pointer",
                );
                ctx
            }
        }
    };
}

/// Marker trait that indicates that a type can be converted into a pointer type for FFI output parameters.
pub trait IntoRaw {
    type Output;

    fn into_raw(self) -> Self::Output;
}

impl<T> IntoRaw for Box<T> {
    type Output = *mut T;

    fn into_raw(self) -> *mut T {
        let pointer = Box::into_raw(self);
        tracing::trace!(
            ?pointer,
            type_name = std::any::type_name::<T>(),
            "converting Box to raw pointer",
        );
        pointer
    }
}

impl IntoRaw for std::ffi::CString {
    type Output = *const std::ffi::c_char;

    fn into_raw(self) -> *const std::ffi::c_char {
        let pointer = self.into_raw();
        tracing::trace!(?pointer, "converting CString to raw pointer",);
        pointer
    }
}
