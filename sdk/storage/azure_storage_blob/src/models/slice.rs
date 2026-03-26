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
    pub(crate) fn from_raw(ptr: *mut T, len: usize) -> Self {
        Self { ptr, len }
    }

    /// Gets this slice as mutable.
    ///
    /// # Safety
    /// The caller must ensure borrow safety is respected.
    pub(crate) unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.ptr, self.len)
    }
}
