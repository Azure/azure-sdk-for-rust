// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ops::Range;

use azure_core::{error::ErrorKind, Error};

/// A drain which accepts elements out-of-order with a specified position and
/// releases them in-order.
///
/// This drain uses a ring buffer to place elements in their correct order as
/// they are received. This means the drain cannot accept any elements with an
/// index of `position() + capacity()`. It also cannot accept any elements
/// marked with an index less than `position()`.
pub struct SequentialBoundedDrain<T> {
    ring_buf: Vec<Option<T>>,
    cursor: usize,
}

impl<T> SequentialBoundedDrain<T> {
    pub fn new(capacity: usize) -> Self {
        // Most smooth vec APIs require T: Clone. Fill manually.
        let mut vec = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            vec.push(None);
        }
        Self {
            ring_buf: vec,
            cursor: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.cursor
    }

    pub fn capacity(&self) -> usize {
        self.ring_buf.len()
    }

    pub fn currently_accepting(&self) -> Range<usize> {
        self.cursor..self.cursor + self.ring_buf.len()
    }

    /// Pushes the given item into the drain at the specified index. Overwrites
    /// the existing element in the drain, if any.
    ///
    /// # Error
    ///
    /// Returns an error when `index` is not contained within `currently_accepting()`.
    pub fn push(&mut self, index: usize, item: T) -> Result<(), Error> {
        let accepted_indices = self.currently_accepting();
        if index < accepted_indices.start {
            let start = accepted_indices.start;
            Err(Error::with_message(ErrorKind::Other, format!("Received item for position {index}, but drain has already progressed to position {start}.")))?;
        }
        if index >= accepted_indices.end {
            let end = accepted_indices.end;
            Err(Error::with_message(ErrorKind::Other, format!("Received item for position {index}, but drain has no room for items of range `{end}..`.")))?;
        }
        let len = self.ring_buf.len();
        self.ring_buf[index % len] = Some(item);
        Ok(())
    }

    /// Returns the next sequential item in the drain, if present.
    pub fn pop(&mut self) -> Option<T> {
        let len = self.ring_buf.len();
        if let Some(item) = self.ring_buf[self.cursor % len].take() {
            self.cursor += 1;
            return Some(item);
        }
        None
    }
}
