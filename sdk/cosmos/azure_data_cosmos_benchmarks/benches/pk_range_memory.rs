// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Memory microbenchmark for partition-key-range data structures.
//!
//! This bench focuses on the **size and allocation profile** of the types
//! involved in the PPAF / PPCB routing path:
//!
//! - [`PartitionKeyRange`] — the per-range record fetched from `/pkranges`
//! - [`ContainerRoutingMap`] — the sorted, indexed view used for EPK lookup
//! - [`PartitionKeyRangeCache`] — the per-driver cache keyed by container
//!
//! Two kinds of measurement are exposed:
//!
//! 1. A `sizes` group that simply prints `std::mem::size_of` for each type at
//!    bench start. These are stack sizes only — useful for spotting field-layout
//!    regressions but blind to heap allocations.
//! 2. A `routing_map_build` Criterion group that constructs a
//!    [`ContainerRoutingMap`] from N partition key ranges. Criterion's
//!    `iter_batched` discards the value after each iteration so the per-iter
//!    timing reflects construction cost (parse, sort, validate, build by-id
//!    index) rather than the whole batch.
//!
//! For per-allocation traces, see the `pk_range_dhat` example in this crate
//! (run with `--features dhat-heap`).
//!
//! Run with:
//!
//! ```text
//! cargo bench -p azure_data_cosmos_benchmarks --bench pk_range_memory
//! ```

use std::mem::size_of;

use azure_data_cosmos_driver::{
    models::{
        effective_partition_key::EffectivePartitionKey, partition_key_range::PartitionKeyRange,
    },
    testing::{ContainerRoutingMap, PartitionKeyRangeCache},
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

/// Range counts representative of small / medium / large containers.
const RANGE_COUNTS: &[usize] = &[1, 8, 32, 128];

/// Builds a vector of `count` contiguous partition key ranges that together
/// cover the full EPK space `["", "FF")`. Range IDs are assigned as `0..count`.
fn build_contiguous_ranges(count: usize) -> Vec<PartitionKeyRange> {
    assert!(count > 0, "need at least one range");
    if count == 1 {
        return vec![PartitionKeyRange::new("0".to_string(), "", "FF")];
    }

    // Divide the single-byte hex space [0x00, 0xFF) into `count` evenly-sized
    // buckets. The first range starts at "" (which compares less than any
    // non-empty string and is the routing map's required lower bound) and the
    // last range ends at "FF". Intermediate boundaries are formatted as
    // two-digit uppercase hex.
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

/// Logs `size_of` for each PK-range type to stderr at bench start. Criterion
/// captures stderr alongside its own output so this surfaces in the bench log.
fn report_sizes() {
    eprintln!("--- pk_range_memory: size_of (bytes) ---");
    eprintln!(
        "  PartitionKeyRange       = {}",
        size_of::<PartitionKeyRange>()
    );
    eprintln!(
        "  EffectivePartitionKey   = {}",
        size_of::<EffectivePartitionKey>()
    );
    eprintln!(
        "  ContainerRoutingMap     = {}",
        size_of::<ContainerRoutingMap>()
    );
    eprintln!(
        "  PartitionKeyRangeCache  = {}",
        size_of::<PartitionKeyRangeCache>()
    );
    eprintln!("----------------------------------------");
}

fn bench_routing_map_build(c: &mut Criterion) {
    report_sizes();

    let mut group = c.benchmark_group("routing_map_build");
    for &n in RANGE_COUNTS {
        let ranges = build_contiguous_ranges(n);
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &ranges, |b, ranges| {
            b.iter_batched(
                || ranges.clone(),
                |owned| {
                    ContainerRoutingMap::try_create(owned, None, None)
                        .expect("contiguous ranges form a valid routing map")
                        .expect("non-empty input")
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_routing_map_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("routing_map_lookup");
    for &n in RANGE_COUNTS {
        let map = ContainerRoutingMap::try_create(build_contiguous_ranges(n), None, None)
            .expect("contiguous ranges")
            .expect("non-empty");
        // Pick an EPK that always lands in the middle range — exercises
        // binary-search path without being trivially first/last.
        let probe = EffectivePartitionKey::from(format!("{:02X}", 0xFF / 2));
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::from_parameter(n),
            &(map, probe),
            |b, (m, p)| {
                b.iter(|| m.get_range_by_effective_partition_key(p).expect("hit"));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_routing_map_build, bench_routing_map_lookup);
criterion_main!(benches);
