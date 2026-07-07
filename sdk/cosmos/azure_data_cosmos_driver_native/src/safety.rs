// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Panic- and poisoning-safety helpers for the FFI boundary.
//!
//! Two hazards are unique to a `cdylib` that runs on a host's threads:
//!
//! 1. **Unwinding across the FFI boundary is undefined behavior.** A panic
//!    that escapes an `extern "C"` function aborts (or corrupts) the host
//!    process (.NET / Java / Go / native C). [`ffi_guard`] wraps a fallible
//!    body in [`std::panic::catch_unwind`] and translates a caught panic into
//!    a caller-supplied fallback value instead of letting it unwind.
//!
//! 2. **A panic while holding a [`Mutex`] poisons it**, after which every
//!    subsequent `lock().unwrap()` panics — turning one transient failure
//!    into a permanent cascade. [`MutexExt::lock_recover`] recovers the guard
//!    from a poisoned lock instead of panicking. This is sound for the queue
//!    state guarded here: the protected data is always left in a consistent
//!    shape at every lock-release point, so a recovered guard never observes
//!    a half-mutated structure.

use std::sync::{Mutex, MutexGuard, PoisonError};

/// Runs `f` behind a panic firewall. If `f` panics, the panic is caught at
/// the FFI boundary (preventing undefined behavior from unwinding into the
/// host) and `fallback` is returned instead.
///
/// `fallback` is evaluated eagerly by the caller; pass a cheap sentinel
/// (e.g. `std::ptr::null_mut()`, `false`, or `0`).
pub(crate) fn ffi_guard<R>(fallback: R, f: impl FnOnce() -> R) -> R {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(_) => {
            tracing::error!("panic caught at FFI boundary; returning fallback value");
            fallback
        }
    }
}

/// Extension trait that adds a poison-tolerant lock to [`Mutex`].
pub(crate) trait MutexExt<T> {
    /// Locks the mutex, recovering the guard if the lock was poisoned by a
    /// panic in another thread instead of propagating the poison as a panic.
    fn lock_recover(&self) -> MutexGuard<'_, T>;
}

impl<T> MutexExt<T> for Mutex<T> {
    fn lock_recover(&self) -> MutexGuard<'_, T> {
        self.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
