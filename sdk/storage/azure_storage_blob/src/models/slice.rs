// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// A slice that implements Send and avoids lifetimes of references.
pub(crate) struct SendSlice<T> {
    ptr: *mut T,
    len: usize,
}

// SAFETY: SendSlice contains `*mut T`.
// Caller guarantees this points to valid memory.
unsafe impl<T> Send for SendSlice<T> {}

impl<T> SendSlice<T> {
    /// Creates a slice from raw parts, which can `Send` across threads.
    ///
    /// # Safety
    /// The caller must ensure all safety required by `std::slice::from_raw_parts_mut()`
    /// with the given values.
    ///
    /// This memory can be sent across thread boundaries as mutable. When sending this
    /// slice to another thread, the caller must:
    /// - Ensure no other references alias this memory until this slice is dropped AND any
    ///   results from `as_mut_slice()` are also dropped.
    /// - Ensure the aliased memory is not dropped until this slice is dropped AND any
    ///   results from `as_mut_slice()` are also dropped.
    pub unsafe fn from_raw(ptr: *mut T, len: usize) -> Self {
        Self { ptr, len }
    }

    /// Gets this slice as mutable.
    ///
    /// # Safety
    /// The caller must ensure borrow safety is respected.
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.ptr, self.len)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
