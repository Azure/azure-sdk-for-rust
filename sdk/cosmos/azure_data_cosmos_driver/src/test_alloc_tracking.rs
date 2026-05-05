// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(test)]
static WATCHED_PTR: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static DEALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) struct TrackingAllocator;

#[cfg(test)]
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let watched = WATCHED_PTR.load(Ordering::Acquire);
        if watched != 0 && watched == ptr as usize {
            DEALLOC_COUNT.fetch_add(1, Ordering::AcqRel);
        }
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[cfg(test)]
pub(crate) fn watch_ptr<T>(ptr: *const T) {
    DEALLOC_COUNT.store(0, Ordering::Release);
    WATCHED_PTR.store(ptr as usize, Ordering::Release);
}

#[cfg(test)]
pub(crate) fn dealloc_count() -> usize {
    DEALLOC_COUNT.load(Ordering::Acquire)
}

#[cfg(test)]
pub(crate) fn clear() {
    WATCHED_PTR.store(0, Ordering::Release);
    DEALLOC_COUNT.store(0, Ordering::Release);
}
