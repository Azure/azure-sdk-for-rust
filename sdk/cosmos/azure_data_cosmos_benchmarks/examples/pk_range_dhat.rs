// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Heap-allocation profile of the partition-key-range data structures.
//!
//! Builds a populated [`PartitionKeyRangeCache`] / [`ContainerRoutingMap`]
//! workload, drops it, and writes a `dhat-heap.json` trace to the working
//! directory. Open the resulting file with the DHAT viewer
//! (<https://nnethercote.github.io/dh_view/dh_view.html>) to inspect every
//! allocation captured during the run, with a stack trace for each one.
//!
//! Run with:
//!
//! ```text
//! cargo run -p azure_data_cosmos_benchmarks \
//!     --release --features dhat-heap --example pk_range_dhat
//! ```
//!
//! The `--release` flag matters: debug allocations look very different from
//! release allocations because of `Vec` over-reservation, `Box::new` inlining,
//! and SSO disablement in `String`.

use azure_data_cosmos_driver::{
    models::partition_key_range::PartitionKeyRange,
    testing::{ContainerRoutingMap, PartitionKeyRangeCache},
};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

/// Number of partition key ranges to materialise in the workload. Pick a
/// value representative of a "large" container; tweak as needed when
/// investigating specific regressions.
const RANGE_COUNT: usize = 128;

fn build_contiguous_ranges(count: usize) -> Vec<PartitionKeyRange> {
    assert!(count > 0);
    if count == 1 {
        return vec![PartitionKeyRange::new("0".to_string(), "", "FF")];
    }
    let step = 0xFF / count;
    let mut ranges = Vec::with_capacity(count);
    let mut prev = String::new();
    for i in 0..count {
        let next = if i + 1 == count {
            "FF".to_string()
        } else {
            format!("{:02X}", (i + 1) * step)
        };
        ranges.push(PartitionKeyRange::new(
            i.to_string(),
            prev.clone(),
            next.clone(),
        ));
        prev = next;
    }
    ranges
}

fn main() {
    // Profiler must be installed before any allocation we want to track. The
    // `Vec`/`String` allocations above this point (program init, env setup)
    // are excluded from the trace.
    let _profiler = dhat::Profiler::new_heap();

    // Allocations from this point on are captured.
    let ranges = build_contiguous_ranges(RANGE_COUNT);

    let map = ContainerRoutingMap::try_create(ranges, None, None)
        .expect("contiguous ranges form a valid routing map")
        .expect("non-empty input");

    // Touch the map so the optimizer cannot drop it before profiling completes.
    let lookup_count = map.ranges().iter().map(|r| r.id.len()).sum::<usize>();
    eprintln!(
        "built ContainerRoutingMap with {} ranges (sum of id lengths = {})",
        map.ranges().len(),
        lookup_count,
    );

    // Exercise the cache type's basic shape so its allocator footprint shows
    // up in the trace alongside the routing-map allocations.
    let cache = PartitionKeyRangeCache::new();
    eprintln!(
        "constructed PartitionKeyRangeCache: {:?}",
        &cache as *const _
    );

    // Drop everything explicitly so the profiler captures the deallocations.
    drop(map);
    drop(cache);

    // Profiler is dropped here, writing `dhat-heap.json` to the cwd.
}
