// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Buffer wrapper to support read-style APIs, e.g. `AsyncReadExt::read()`.
///
/// # Safety
///
/// This type implements the minimum viable API to achieve its goal, marking appropriate
/// methods as unsafe to ensure proper use. To avoid bypassing these markers, any larger
/// functionality should be built in `ReadBufExt`, outside this module.
///
/// # Remarks
///
/// Most other buffer APIs that support a running cursor across multiple writes do not support
/// direct writes to the underlying data via `&mut [u8]`. This interface is to support such
/// cursor functionality for APIs that directly access the target slice. Such APIs are harder
/// to prove correctness with, trusting an implicit contract that whoever accesses that memory
/// is writing to it as intended and communicating the number of bytes written accordingly.
/// This type forces callers to make those decisions. An extension trait contains common
/// implementations.
///
/// To get the owned data back out, a `finalize()` must be written based on the implementing
/// type to properly truncate the data before returning ownership.
#[derive(Default)]
pub(crate) struct ReadBuf {
    buf: Vec<u8>,
    cursor: usize,
}

impl ReadBuf {
    /// Creates a new ReadBuf with a zeroed Vec of the given capacity.
    pub fn zeroed(capacity: usize) -> Self {
        Self {
            buf: vec![0; capacity],
            cursor: 0,
        }
    }

    /// Extends the capacity of this buf and zeroes the new capacity.
    pub fn extend_zeroed(&mut self, additional: usize) {
        self.buf.resize(self.buf.len() + additional, 0);
    }

    /// Returns the inner vec, truncated to len().
    pub fn finalize(mut self) -> Vec<u8> {
        self.buf.truncate(self.cursor);
        self.buf
    }

    /// Gets the length of written bytes.
    pub fn len(&self) -> usize {
        self.cursor
    }
    /// Gets if there are no written bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the capacity of the underlying buffer.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn remaining(&self) -> usize {
        self.capacity() - self.len()
    }

    /// Gets a mutable reference to the slice of bytes between `len()` and `capacity()`.
    pub fn spare_capacity_mut(&mut self) -> &mut [u8] {
        &mut self.buf[self.cursor..]
    }

    /// Set the length of this buffer.
    ///
    /// # Safety
    ///
    /// This method updates the internal cursor of the length without any checks.
    /// It is the caller's responsibility to ensure this is within the buffer's capacity.
    pub unsafe fn unchecked_set_len(&mut self, position: usize) {
        self.cursor = position;
    }
}
