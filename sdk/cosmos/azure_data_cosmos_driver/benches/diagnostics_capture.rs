// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion benchmarks for the deferred, threshold-gated diagnostics capture.
//!
//! The headline metric is the **dropped/fast-success hot path**: capture pushes typed spans/attrs
//! into a pooled event log, then the gate drops it and returns the log to the pool. With a warm
//! pool this is allocation-free apart from the attempt's owned strings. The `built` case measures
//! the cost of materializing the canonical `DiagnosticsContext`, paid only past the gate.

use azure_data_cosmos_driver::diagnostics::capture::{
    finish, AttemptRecord, DiagnosticsPolicy, DiagnosticsRecorder, LogPool, Outcome,
};
use azure_data_cosmos_driver::diagnostics::ExecutionContext;
use azure_data_cosmos_driver::options::DiagnosticsOptions;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

/// Records an S2-shaped operation (retry 429 -> 200) into a recorder, then gates it.
fn capture_s2(pool: &LogPool, policy: &DiagnosticsPolicy, options: &Arc<DiagnosticsOptions>) {
    let mut rec = DiagnosticsRecorder::start(pool, "read_item", "https://acct/", "activity-bench");
    rec.record_attempt(
        AttemptRecord::new(ExecutionContext::Initial, "East US", "https://east/", 429)
            .with_service_request_id("svc-429")
            .with_request_charge(4.2)
            .with_duration_ns(3_000_000),
    );
    rec.record_attempt(
        AttemptRecord::new(ExecutionContext::Retry, "East US", "https://east/", 200)
            .with_service_request_id("svc-200")
            .with_request_charge(4.2)
            .with_duration_ns(4_000_000),
    );
    rec.record_end(Outcome::Success, 2, 200, None, Some(7_000_000));
    black_box(finish(rec, policy, Arc::clone(options)));
}

fn diagnostics_benchmarks(c: &mut Criterion) {
    let options = Arc::new(DiagnosticsOptions::default());

    // `Mode::Off`: the gate short-circuits BEFORE `build_context` runs (it is never called). In the
    // live driver, Off additionally skips constructing the recorder AND disables the
    // `DiagnosticsContextBuilder` so per-request population is free too (see the
    // `disabled_builder_records_nothing_per_request` unit test). This measures the recorder-path
    // Off cost: append + gate-drops-without-building.
    let off_policy = DiagnosticsPolicy::off();
    c.bench_function("capture_off_noop", |b| {
        let pool = LogPool::new();
        let options = Arc::clone(&options);
        b.iter(|| {
            let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/", "c");
            rec.record_attempt(
                AttemptRecord::new(ExecutionContext::Initial, "East US", "https://east/", 200)
                    .with_service_request_id("svc-200")
                    .with_request_charge(2.5)
                    .with_duration_ns(1_000_000),
            );
            rec.record_end(Outcome::Success, 1, 200, None, Some(1_000_000));
            black_box(finish(rec, &off_policy, Arc::clone(&options)));
        });
    });

    // Threshold fast-success: a 1 ms success under a 5 ms threshold. The gate decides NOT to build
    // and `finish` returns before `build_context` — the build is short-circuited, NOT built-then-
    // dropped. Contrast the cost with `capture_built_context` below to see the saving.
    let drop_policy = DiagnosticsPolicy::threshold(Duration::from_millis(5));
    c.bench_function("capture_dropped_fast_success", |b| {
        let pool = LogPool::new();
        let options = Arc::clone(&options);
        b.iter(|| {
            let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/", "c");
            rec.record_attempt(
                AttemptRecord::new(ExecutionContext::Initial, "East US", "https://east/", 200)
                    .with_service_request_id("svc-200")
                    .with_request_charge(2.5)
                    .with_duration_ns(1_000_000),
            );
            rec.record_end(Outcome::Success, 1, 200, None, Some(1_000_000));
            black_box(finish(rec, &drop_policy, Arc::clone(&options)));
        });
    });

    // Build cost: materialize the canonical DiagnosticsContext, paid only when the gate fires
    // (error / slow / Always). The gap between this and the two drop benchmarks above is the cost
    // the gate saves by deciding BEFORE the build.
    c.bench_function("capture_built_context", |b| {
        let pool = LogPool::new();
        let policy = DiagnosticsPolicy::always();
        let options = Arc::clone(&options);
        b.iter(|| capture_s2(&pool, &policy, &options));
    });
}

criterion_group!(benches, diagnostics_benchmarks);
criterion_main!(benches);
