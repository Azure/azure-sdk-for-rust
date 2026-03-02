// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Benchmarks for `AsyncCache` to measure lock contention, I/O-under-lock impact,
//! and read throughput under varying concurrency levels.
//!
//! These benchmarks simulate the real-world usage patterns of `AsyncCache` in the
//! Cosmos SDK, where cache misses trigger HTTP I/O (simulated with `tokio::time::sleep`)
//! while concurrent readers and writers compete for the lock.
//!
//! Run with:
//! ```sh
//! cargo bench -p azure_data_cosmos --features bench --bench async_cache
//! ```

use azure_data_cosmos::_bench::AsyncCache;
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
/// Measures the overhead of acquiring the read lock and cloning the cached value.
/// This is the fast path that runs on every Cosmos DB operation when partition-level
/// circuit breaker is enabled.
fn bench_read_hit(c: &mut Criterion) {
    let setup_rt = rt();
    let cache: Arc<AsyncCache<String, String>> = Arc::new(AsyncCache::new(None));

    // Pre-populate the cache with NUM_KEYS entries.
    setup_rt.block_on(async {
        for i in 0..NUM_KEYS {
            let key = format!("key_{i}");
            let value = format!("value_{i}");
            cache.insert(key, value).await;
        }
    });
    drop(setup_rt);

    let mut group = c.benchmark_group("async_cache/read_hit");
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
                                cache
                                    .get(key, |_| false, || async { Ok::<_, String>("new".into()) })
                                    .await
                                    .unwrap();
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
/// Measures the impact of holding the write lock during a simulated I/O operation.
/// With the current implementation, all concurrent tasks are blocked while one task
/// performs the compute. This is the key bottleneck this benchmark exposes.
fn bench_single_key_miss(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache/single_key_miss");

    for &concurrency in CONCURRENCY_LEVELS {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            &concurrency,
            |b, &concurrency| {
                b.to_async(rt()).iter(|| async move {
                    // Fresh cache each iteration so every access is a miss.
                    let cache: Arc<AsyncCache<String, String>> =
                        Arc::new(AsyncCache::new(None));

                    let mut handles = Vec::with_capacity(concurrency);
                    for _ in 0..concurrency {
                        let cache = cache.clone();
                        handles.push(tokio::spawn(async move {
                            cache
                                .get(
                                    "shared_key".to_string(),
                                    |_| false,
                                    || async {
                                        tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                        Ok::<_, String>("computed_value".into())
                                    },
                                )
                                .await
                                .unwrap();
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
/// Each concurrent task targets a different key. With the current single-RwLock
/// design, a write lock on any key blocks all other keys. After the proposed
/// per-key gating fix, tasks on different keys should run in parallel.
fn bench_multi_key_miss(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache/multi_key_miss");

    for &concurrency in CONCURRENCY_LEVELS {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            &concurrency,
            |b, &concurrency| {
                b.to_async(rt()).iter(|| async move {
                    let cache: Arc<AsyncCache<String, String>> =
                        Arc::new(AsyncCache::new(None));

                    let mut handles = Vec::with_capacity(concurrency);
                    for t in 0..concurrency {
                        let cache = cache.clone();
                        handles.push(tokio::spawn(async move {
                            let key = format!("key_{t}");
                            cache
                                .get(key, |_| false, || async {
                                    tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                    Ok::<_, String>(format!("value_{t}"))
                                })
                                .await
                                .unwrap();
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
/// Simulates the production pattern: many concurrent read_item/query_item operations
/// hitting a warm cache while one task triggers a cache refresh (e.g., container
/// metadata TTL expiry). Readers should not be significantly delayed by the writer.
fn bench_mixed_read_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache/mixed_read_write");

    for &reader_count in &[10usize, 50, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(reader_count),
            &reader_count,
            |b, &reader_count| {
                let runtime = rt();
                let cache: Arc<AsyncCache<String, String>> = Arc::new(AsyncCache::new(None));

                // Pre-populate with some keys.
                runtime.block_on(async {
                    for i in 0..NUM_KEYS {
                        cache.insert(format!("key_{i}"), format!("value_{i}")).await;
                    }
                });

                b.to_async(runtime).iter(|| {
                    let cache = cache.clone();
                    async move {
                        let mut handles = Vec::with_capacity(reader_count + 1);

                        // One writer forcing a refresh on a new key (simulated I/O).
                        {
                            let cache = cache.clone();
                            handles.push(tokio::spawn(async move {
                                cache
                                    .get(
                                        "new_key".to_string(),
                                        |_| true, // force refresh
                                        || async {
                                            tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                            Ok::<_, String>("new_value".into())
                                        },
                                    )
                                    .await
                                    .unwrap();
                            }));
                        }

                        // Many readers hitting existing warm keys.
                        for t in 0..reader_count {
                            let cache = cache.clone();
                            handles.push(tokio::spawn(async move {
                                let key = format!("key_{}", t % NUM_KEYS);
                                cache
                                    .get(key, |_| false, || async {
                                        Ok::<_, String>("should_not_compute".into())
                                    })
                                    .await
                                    .unwrap();
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

/// Benchmark: Convoy effect demonstration.
///
/// Multiple tasks all need different keys simultaneously, but each key's compute
/// involves I/O. With a single global lock, these serialize. With per-key gates,
/// they should parallelize.
///
/// Expected behavior:
/// - Current (global lock): ~N * SIMULATED_IO_LATENCY total wall time
/// - After fix (per-key gates): ~1 * SIMULATED_IO_LATENCY total wall time
fn bench_convoy_effect(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache/convoy_effect");
    configure_for_slow_bench(&mut group);

    for &key_count in &[2usize, 5, 10] {
        group.bench_with_input(
            BenchmarkId::from_parameter(key_count),
            &key_count,
            |b, &key_count| {
                b.to_async(rt()).iter(|| async move {
                    let cache: Arc<AsyncCache<String, String>> =
                        Arc::new(AsyncCache::new(None));

                    let mut handles = Vec::with_capacity(key_count);
                    for i in 0..key_count {
                        let cache = cache.clone();
                        handles.push(tokio::spawn(async move {
                            cache
                                .get(format!("independent_key_{i}"), |_| false, || async {
                                    tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                    Ok::<_, String>(format!("value_{i}"))
                                })
                                .await
                                .unwrap();
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

/// Benchmark: TTL-based expiry under concurrent access.
///
/// Pre-populates the cache with a short TTL, then fires many concurrent reads
/// after the TTL has expired. All readers discover the expiry and contend on
/// recomputing the value.
fn bench_expired_contention(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_cache/expired_contention");
    configure_for_slow_bench(&mut group);

    for &concurrency in CONCURRENCY_LEVELS {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            &concurrency,
            |b, &concurrency| {
                b.to_async(rt()).iter(|| async move {
                    // TTL of 1ms — entries expire almost immediately.
                    let cache: Arc<AsyncCache<String, String>> =
                        Arc::new(AsyncCache::new(Some(Duration::from_millis(1))));

                    // Populate then wait for expiry.
                    cache.insert("expiring_key".to_string(), "old_value".to_string()).await;
                    tokio::time::sleep(Duration::from_millis(5)).await;

                    let mut handles = Vec::with_capacity(concurrency);
                    for _ in 0..concurrency {
                        let cache = cache.clone();
                        handles.push(tokio::spawn(async move {
                            cache
                                .get(
                                    "expiring_key".to_string(),
                                    |_| false,
                                    || async {
                                        tokio::time::sleep(SIMULATED_IO_LATENCY).await;
                                        Ok::<_, String>("refreshed_value".into())
                                    },
                                )
                                .await
                                .unwrap();
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
    bench_expired_contention,
);
criterion_main!(benches);
