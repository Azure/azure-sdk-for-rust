// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion microbenchmark for `CosmosDriver::execute_operation` — point read.
//!
//! The reqwest transport is replaced with an in-memory mock so that the benchmark
//! measures driver overhead (routing, signing, retry state, response parsing, session
//! token management) without any network I/O.
//!
//! Cache priming (account metadata, container metadata) is performed in setup, outside
//! the measured iteration loop.
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

#[path = "common.rs"]
mod common;

use std::path::Path;

use azure_data_cosmos_driver::{models::CosmosOperation, options::OperationOptions};
use criterion::{criterion_group, criterion_main, profiler::Profiler, Criterion};
use tokio::runtime::Builder;

// ---------------------------------------------------------------------------
// pprof adapter for criterion 0.8
//
// pprof's built-in `criterion` feature targets criterion 0.5. We implement
// the criterion 0.8 `Profiler` trait ourselves using pprof's core API.
// ---------------------------------------------------------------------------

struct PprofAdapter {
    frequency: i32,
    guard: Option<pprof::ProfilerGuard<'static>>,
}

impl PprofAdapter {
    fn new(frequency: i32) -> Self {
        Self {
            frequency,
            guard: None,
        }
    }
}

impl Profiler for PprofAdapter {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        let guard = pprof::ProfilerGuardBuilder::default()
            .frequency(self.frequency)
            .blocklist(&["libc", "libgcc", "pthread", "vdso"])
            .build()
            .expect("failed to start pprof profiler");
        self.guard = Some(guard);
    }

    fn stop_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path) {
        let Some(guard) = self.guard.take() else {
            return;
        };
        match guard.report().build() {
            Ok(report) => {
                std::fs::create_dir_all(benchmark_dir).ok();
                let flamegraph_path = benchmark_dir.join(format!("{benchmark_id}.svg"));
                match std::fs::File::create(&flamegraph_path) {
                    Ok(file) => {
                        if let Err(e) = report.flamegraph(file) {
                            eprintln!("pprof: failed to write flamegraph: {e}");
                        }
                    }
                    Err(e) => eprintln!("pprof: failed to create {flamegraph_path:?}: {e}"),
                }
            }
            Err(e) => eprintln!("pprof: failed to build report: {e}"),
        }
    }
}

fn bench_point_read(c: &mut Criterion) {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create tokio runtime");

    let latencies: &[(&str, std::time::Duration)] = &[
        ("latency_0ms", std::time::Duration::ZERO),
        ("latency_2ms", std::time::Duration::from_millis(2)),
        ("latency_10ms", std::time::Duration::from_millis(10)),
    ];

    let mut group = c.benchmark_group("point_read");
    for (name, latency) in latencies {
        let (driver, item_ref) = rt.block_on(common::setup(*latency));
        group.bench_function(*name, |b| {
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
    group.finish();
}

// 997 Hz — prime frequency to avoid aliasing with timer interrupts.
// Profiling is only active when Criterion is invoked with `--profile-time`;
// normal `cargo bench` runs have zero pprof overhead.
criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PprofAdapter::new(997));
    targets = bench_point_read
);
criterion_main!(benches);
