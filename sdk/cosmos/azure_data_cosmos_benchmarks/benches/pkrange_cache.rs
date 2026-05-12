// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion benchmarks for the partition key range routing cache.
//!
//! Two scenarios are exercised:
//!
//! - `lookup_cached/{1,10,100}` — measures the hot-path cache hit only:
//!   compute the effective partition key from a `PartitionKey` and binary-search
//!   the routing map for the owning range. The cache is primed once during
//!   setup so that no I/O or parsing happens inside the measured loop.
//!
//! - `lookup_uncached/{1,10,100}` — measures the cold-cache path: each
//!   iteration starts with a fresh driver/cache, so the work spans transport
//!   I/O (mocked, zero-latency), JSON parse, routing-map index build, EPK
//!   compute, and the final lookup.
//!
//! Both groups are parameterized by the number of partition key ranges in
//! the routing map (1, 10, 100), reflecting realistic small / medium / large
//! container footprints. The mock transport is configured to return a
//! contiguously-covered EPK space sized to the scenario.
//!
//! Always uses the in-memory mock transport — there is no `live` mode for
//! this benchmark because the routing-cache code path does not vary based
//! on the underlying transport.

use std::sync::Arc;
use std::time::Duration;

use azure_data_cosmos_benchmarks as common;
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntimeBuilder,
    models::{AccountReference, PartitionKey},
    testing::HttpClientFactory,
    CosmosDriver,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use tokio::runtime::{Builder, Runtime};
use url::Url;

const RANGE_SIZES: &[u32] = &[1, 10, 100];

fn make_runtime() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create tokio runtime")
}

fn bench_pkrange_cache(c: &mut Criterion) {
    let rt = make_runtime();

    let mut group = c.benchmark_group("pkrange_cache");
    group
        .throughput(Throughput::Elements(1))
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);

    // ---- Cached lookups ------------------------------------------------
    //
    // One driver per range count, primed once. Inside the bench loop we
    // only re-resolve the partition key — no further fetches occur.
    for &n in RANGE_SIZES {
        let (driver, container, pk) = rt.block_on(common::setup_with_pkranges(n));

        // Prime the routing-map cache before measuring so the first iteration
        // doesn't pay a one-off fetch cost that would skew small samples.
        rt.block_on(async {
            driver
                .resolve_partition_key_ranges_for_key(&container, &pk, false)
                .await
                .expect("priming lookup returned None");
        });

        group.bench_with_input(BenchmarkId::new("lookup_cached", n), &n, |b, _| {
            b.to_async(&rt).iter(|| async {
                driver
                    .resolve_partition_key_ranges_for_key(&container, &pk, false)
                    .await
                    .expect("cached lookup returned None")
            });
        });
    }

    // ---- Uncached lookups ----------------------------------------------
    //
    // Each iteration constructs a fresh driver so the cache is empty. We
    // therefore pay container resolution + pkranges fetch + parse + index
    // build on every call, in addition to the actual EPK lookup.
    //
    // The pkranges payload is built once per range count and cloned into
    // each iteration's factory; this isolates the measurement from JSON
    // generation cost.
    for &n in RANGE_SIZES {
        let payload: Arc<str> = Arc::from(common::build_pkranges_payload(n));
        let pk = PartitionKey::from("pk1");

        group.bench_with_input(BenchmarkId::new("lookup_uncached", n), &n, |b, _| {
            b.to_async(&rt).iter(|| {
                let payload = payload.clone();
                let pk = pk.clone();
                async move {
                    let driver = build_fresh_driver(payload).await;
                    let container = driver
                        .resolve_container("benchdb", "benchcontainer")
                        .await
                        .expect("failed to resolve container");
                    driver
                        .resolve_partition_key_ranges_for_key(&container, &pk, false)
                        .await
                        .expect("uncached lookup returned None")
                }
            });
        });
    }

    group.finish();
}

/// Helper that builds a driver runtime + driver from scratch for the
/// uncached benchmark loop. Inlined here rather than added to `lib.rs`
/// because no other benchmark currently needs a per-iteration fresh driver.
async fn build_fresh_driver(pkranges_payload: Arc<str>) -> Arc<CosmosDriver> {
    let factory: Arc<dyn HttpClientFactory> =
        Arc::new(common::MockHttpClientFactory::new().with_pkranges_payload(pkranges_payload));
    let runtime = CosmosDriverRuntimeBuilder::new()
        .with_mock_http_client_factory(factory)
        .build()
        .await
        .expect("failed to build runtime");

    let account = AccountReference::with_master_key(
        Url::parse("https://bench.documents.azure.com:443/").unwrap(),
        "dGVzdA==",
    );

    runtime
        .get_or_create_driver(account, None)
        .await
        .expect("failed to create driver")
}

criterion_group!(benches, bench_pkrange_cache);
criterion_main!(benches);
