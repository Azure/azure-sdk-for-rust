// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion benchmarks for the deferred, threshold-gated diagnostics capture.
//!
//! The headline metric is the **dropped/fast-success hot path** (AC-1): capture appends a
//! compact record stream, then the gate drops it and returns the buffer to the pool. With a
//! warm pool this should be allocation-free and on the order of tens of nanoseconds. The
//! `summary` and `detailed` cases measure the build cost paid only past the gate.

use azure_data_cosmos_driver::diagnostics::capture::{
    finish, DiagnosticsPolicy, DiagnosticsRecorder, LogPool, Outcome,
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::time::Duration;

/// Records an S2-shaped operation (retry 429 -> 200) into a recorder, then gates it.
fn capture_s2(pool: &LogPool, policy: &DiagnosticsPolicy) {
    let mut rec = DiagnosticsRecorder::start(
        pool,
        "read_item",
        "https://acct/dbs/d/colls/c/docs/1",
        "client-bench",
    );
    rec.record_attempt(0, 429, Some("svc-429"), Some(4.2), 0, 3_000_000);
    rec.record_attempt(1, 200, Some("svc-200"), Some(4.2), 3_000_000, 4_000_000);
    rec.record_end(Outcome::Success, 2, Some(7_000_000));
    black_box(finish(rec, policy));
}

fn diagnostics_benchmarks(c: &mut Criterion) {
    // Dropped / fast-success path: a 1 ms success under a 5 ms threshold -> nothing built.
    // The pool warms up after the first iteration, so steady-state is append + return-to-pool.
    let drop_policy = DiagnosticsPolicy::threshold(Duration::from_millis(5));
    c.bench_function("capture_dropped_fast_success", |b| {
        let pool = LogPool::new();
        b.iter(|| {
            let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct", "c");
            rec.record_attempt(0, 200, Some("svc-200"), Some(2.5), 0, 1_000_000);
            rec.record_end(Outcome::Success, 1, Some(1_000_000));
            black_box(finish(rec, &drop_policy));
        });
    });

    // Build cost (summary only), paid only past the gate.
    c.bench_function("capture_built_summary", |b| {
        let pool = LogPool::new();
        let policy = DiagnosticsPolicy::always();
        b.iter(|| capture_s2(&pool, &policy));
    });

    // Build cost (summary + AZD1 binary), the opt-in detail tier.
    c.bench_function("capture_built_detailed", |b| {
        let pool = LogPool::new();
        let policy = DiagnosticsPolicy {
            binary: true,
            ..DiagnosticsPolicy::always()
        };
        b.iter(|| capture_s2(&pool, &policy));
    });
}

criterion_group!(benches, diagnostics_benchmarks);
criterion_main!(benches);
