// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion benchmark for `CosmosDriver::execute_operation` — point read.
//!
//! By default the reqwest transport is replaced with an in-memory mock so that
//! the benchmark measures driver overhead (routing, signing, retry state,
//! response parsing, session token management) without any network I/O.
//!
//! Set `AZURE_BENCH_MODE=live` to run against a real Cosmos DB endpoint. See
//! `azure_data_cosmos_benchmarks::setup_live` for the required environment
//! variables.
//!
//! Cache priming (account metadata, container metadata) is performed in setup,
//! outside the measured iteration loop.
//!
//! # CPU flamegraph profiling
//!
//! Run with `--profile-time` to generate a flamegraph SVG via pprof:
//!
//! ```text
//! cargo bench -p azure_data_cosmos_benchmarks --bench point_read -- --profile-time 30
//! ```
//!
//! Output: `target/criterion/point_read/profile/flamegraph.svg`

use azure_data_cosmos_benchmarks::{self as common, BenchConfig};

use std::time::Duration;

use azure_data_cosmos_driver::{models::CosmosOperation, options::OperationOptions};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tokio::runtime::Builder;

fn bench_point_read(c: &mut Criterion) {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create tokio runtime");

    let mut group = c.benchmark_group("point_read");
    group.throughput(Throughput::Elements(1));

    match common::load_bench_config() {
        BenchConfig::Mock => {
            let (driver, item_ref) = rt.block_on(common::setup());
            group.bench_function("mock", |b| {
                b.to_async(&rt).iter(|| async {
                    driver
                        .execute_operation(
                            CosmosOperation::read_item(item_ref.clone()),
                            OperationOptions::default(),
                        )
                        .await
                        .expect("execute_operation failed")
                });
            });
        }
        BenchConfig::Live => {
            let (driver, item_ref) = rt.block_on(common::setup_live());
            group
                .sample_size(50)
                .measurement_time(Duration::from_secs(30));
            group.bench_function("live", |b| {
                b.to_async(&rt).iter(|| async {
                    driver
                        .execute_operation(
                            CosmosOperation::read_item(item_ref.clone()),
                            OperationOptions::default(),
                        )
                        .await
                        .expect("execute_operation failed")
                });
            });
        }
    }

    group.finish();
}

criterion_group!(benches, bench_point_read);
criterion_main!(benches);
