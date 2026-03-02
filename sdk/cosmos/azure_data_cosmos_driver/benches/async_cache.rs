// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Benchmarks for `AsyncCache` (driver crate) to measure lock contention,
//! single-pending-I/O semantics, and read throughput under varying concurrency.
//!
//! This `AsyncCache` uses per-key `AsyncLazy` entries for single-flight I/O,
//! which should show better behavior than a global-lock-during-I/O design.
//! These benchmarks verify that and measure the remaining contention points
//! (global `RwLock<HashMap>` for map access).
//!
//! Run with:
//! ```sh
//! cargo bench -p azure_data_cosmos_driver --features bench --bench async_cache
//! ```

use azure_data_cosmos_driver::driver::_cache_bench::AsyncCache;
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

/// Simulated I/O latency for cache miss compute operations.
const SIMULATED_IO_LATENCY: Duration = Duration::from_millis(5);

/// Number of distinct keys used in multi-key benchmarks.
const NUM_KEYS: usize = 10;

/// Concurrency levels to test.
const CONCURRENCY_LEVELS: &[usize] = &[1, 10, 50, 100];

/// Creates a tokio multi-thread runtime for async benchmarks.
fn rt() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

/// Benchmark: Read throughput on a warm cache (all hits, no I/O).
///
/// Measures the overhead of acquiring the read lock and returning the `Arc<V>`.
/// This is the fast path that runs on every cached metadata lookup.
fn bench_read_hit(c: &mut Criterion) {
    let setup_rt = rt();
    let cache: Arc<AsyncCache<String, String>> = Arc::new(AsyncCache::new());

    // Pre-populate the cache with NUM_KEYS entries.
    setup_rt.block_on(async {
        for i in 0..NUM_KEYS {
            let key = format!("key_{i}");
            let val = format!("value_{i}");
            cache
                .get_or_insert_with(key, || async move { val })
                .await;
        }
    });
    drop(setup_rt);

    let mut group = c.benchmark_group("async_cache_driver/read_hit");
    for &concurrency in CONCURRENCY_LEVELS {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            &concurrency,
            |b, &concurrency| {
                b.to_async(rt()).iter(|| {
                    let cache = cache.clone();
                    async move {
                        let mut handles = Vec::with_capacity(concurrency);
                        for t in 0..concurrency {
                            let cache = cache.clone();
                            handles.push(tokio::spawn(async move {
                                let key = format!("key_{}", t % NUM_KEYS);
                                // Use get() for pure read path
                                cache.get(&key).await.unwrap();
                            }));
                        }
                        for h in handles {
                            h.await.unwrap();
                        }
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: Cache miss with simulated I/O (single key).
///
/// All concurrent tasks hit the same missing key. With single-pending-I/O
/// semantics, only one task should run the factory. Others should wait for
/// the shared `AsyncLazy` result.
fn bench_single_key_miss(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache_driver/single_key_miss");

    for &concurrency in CONCURRENCY_LEVELS {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            &concurrency,
            |b, &concurrency| {
                b.to_async(rt()).iter(|| async move {
                    // Fresh cache each iteration so every access is a miss.
                    let cache: Arc<AsyncCache<String, String>> = Arc::new(AsyncCache::new());

                    let mut handles = Vec::with_capacity(concurrency);
                    for _ in 0..concurrency {
                        let cache = cache.clone();
                        handles.push(tokio::spawn(async move {
                            cache
                                .get_or_insert_with("shared_key".to_string(), || async {
                                    tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                    "computed_value".to_string()
                                })
                                .await;
                        }));
                    }
                    for h in handles {
                        h.await.unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: Cache miss with simulated I/O across independent keys.
///
/// Each concurrent task targets a different key. With per-key `AsyncLazy`,
/// independent keys should initialize in parallel since the global write lock
/// is only held briefly for HashMap insertion (not during I/O).
///
/// Expected: ~constant time regardless of key count (vs. N * latency with
/// the `azure_data_cosmos` AsyncCache).
fn bench_multi_key_miss(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache_driver/multi_key_miss");

    for &concurrency in CONCURRENCY_LEVELS {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            &concurrency,
            |b, &concurrency| {
                b.to_async(rt()).iter(|| async move {
                    let cache: Arc<AsyncCache<String, String>> = Arc::new(AsyncCache::new());

                    let mut handles = Vec::with_capacity(concurrency);
                    for t in 0..concurrency {
                        let cache = cache.clone();
                        handles.push(tokio::spawn(async move {
                            let key = format!("key_{t}");
                            cache
                                .get_or_insert_with(key, || async move {
                                    tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                    format!("value_{t}")
                                })
                                .await;
                        }));
                    }
                    for h in handles {
                        h.await.unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: Mixed read/write workload (readers + one writer with I/O).
///
/// Simulates the production pattern: many concurrent lookups hitting a warm
/// cache while one task triggers a cache miss. Because I/O runs outside
/// the global lock in this design, readers should not be blocked by the writer.
fn bench_mixed_read_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache_driver/mixed_read_write");

    for &reader_count in &[10usize, 50, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(reader_count),
            &reader_count,
            |b, &reader_count| {
                let runtime = rt();
                let cache: Arc<AsyncCache<String, String>> = Arc::new(AsyncCache::new());

                // Pre-populate with some keys.
                runtime.block_on(async {
                    for i in 0..NUM_KEYS {
                        cache
                            .get_or_insert_with(format!("key_{i}"), || async move {
                                format!("value_{i}")
                            })
                            .await;
                    }
                });

                b.to_async(runtime).iter(|| {
                    let cache = cache.clone();
                    async move {
                        let mut handles = Vec::with_capacity(reader_count + 1);

                        // One writer inserting a new key (simulated I/O).
                        {
                            let cache = cache.clone();
                            handles.push(tokio::spawn(async move {
                                cache
                                    .get_or_insert_with("new_key".to_string(), || async {
                                        tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                        "new_value".to_string()
                                    })
                                    .await;
                            }));
                        }

                        // Many readers hitting existing warm keys.
                        for t in 0..reader_count {
                            let cache = cache.clone();
                            handles.push(tokio::spawn(async move {
                                let key = format!("key_{}", t % NUM_KEYS);
                                cache.get(&key).await.unwrap();
                            }));
                        }

                        for h in handles {
                            h.await.unwrap();
                        }
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: Convoy effect test across independent keys.
///
/// Multiple tasks all need different keys simultaneously, each involving I/O.
/// With per-key `AsyncLazy`, these should parallelize (wall time ≈ 1× latency).
/// With a global I/O lock, they would serialize (wall time ≈ N× latency).
fn bench_convoy_effect(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache_driver/convoy_effect");
    configure_for_slow_bench(&mut group);

    for &key_count in &[2usize, 5, 10] {
        group.bench_with_input(
            BenchmarkId::from_parameter(key_count),
            &key_count,
            |b, &key_count| {
                b.to_async(rt()).iter(|| async move {
                    let cache: Arc<AsyncCache<String, String>> = Arc::new(AsyncCache::new());

                    let mut handles = Vec::with_capacity(key_count);
                    for i in 0..key_count {
                        let cache = cache.clone();
                        handles.push(tokio::spawn(async move {
                            cache
                                .get_or_insert_with(
                                    format!("independent_key_{i}"),
                                    || async move {
                                        tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                        format!("value_{i}")
                                    },
                                )
                                .await;
                        }));
                    }
                    for h in handles {
                        h.await.unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: Invalidate-then-reinsert under concurrent access.
///
/// Simulates cache invalidation (e.g., stale container metadata) followed by
/// re-population while concurrent readers try to access the same key.
fn bench_invalidate_reinsert(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache_driver/invalidate_reinsert");
    configure_for_slow_bench(&mut group);

    for &concurrency in CONCURRENCY_LEVELS {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            &concurrency,
            |b, &concurrency| {
                b.to_async(rt()).iter(|| async move {
                    let cache: Arc<AsyncCache<String, String>> = Arc::new(AsyncCache::new());

                    // Pre-populate.
                    cache
                        .get_or_insert_with("target_key".to_string(), || async {
                            "old_value".to_string()
                        })
                        .await;

                    // Invalidate the key.
                    cache.invalidate(&"target_key".to_string()).await;

                    // Now N tasks race to re-populate.
                    let mut handles = Vec::with_capacity(concurrency);
                    for _ in 0..concurrency {
                        let cache = cache.clone();
                        handles.push(tokio::spawn(async move {
                            cache
                                .get_or_insert_with("target_key".to_string(), || async {
                                    tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                    "refreshed_value".to_string()
                                })
                                .await;
                        }));
                    }
                    for h in handles {
                        h.await.unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

/// Reduces sample size and measurement time for benchmarks that include real sleep.
fn configure_for_slow_bench(group: &mut BenchmarkGroup<WallTime>) {
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(10));
}

criterion_group!(
    benches,
    bench_read_hit,
    bench_single_key_miss,
    bench_multi_key_miss,
    bench_mixed_read_write,
    bench_convoy_effect,
    bench_invalidate_reinsert,
);
criterion_main!(benches);
