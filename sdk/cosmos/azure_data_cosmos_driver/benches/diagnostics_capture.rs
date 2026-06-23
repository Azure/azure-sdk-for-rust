// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion benchmarks for the deferred, threshold-gated diagnostics capture.
//!
//! The headline metric is the **dropped/fast-success hot path**: capture pushes typed spans/attrs
//! into a pooled event log, then the gate drops it and returns the log to the pool. With a warm
//! pool this is allocation-free apart from the attempt's owned strings.
//!
//! Two distinct costs are measured past the gate, so they can be compared directly (this answers
//! "does building the `DiagnosticsContext` struct dominate, or does serializing it to JSON?"):
//! - `capture_built_context` — materialize the canonical `DiagnosticsContext` **struct only**. No
//!   JSON is produced (`build_context` returns the struct; the summary + JSON are lazy).
//! - `capture_built_context_and_json` — the same build **plus** `to_json_string(None)` (the
//!   detailed JSON the driver surfaces). The delta between the two is the pure
//!   serialization cost; the gap to the drop benchmarks is what the gate saves by deciding first.

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
///
/// When `serialize_json` is set, the built `DiagnosticsContext` is also serialized to its detailed
/// JSON string, so the caller can isolate the JSON-serialization cost from the struct-build cost.
fn capture_s2(
    pool: &Arc<LogPool>,
    policy: &DiagnosticsPolicy,
    options: &Arc<DiagnosticsOptions>,
    serialize_json: bool,
) {
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
    let built = finish(rec, policy, Arc::clone(options));
    if serialize_json {
        if let Some(ctx) = &built {
            // `None` => the options default verbosity (detailed); this is the canonical JSON the
            // driver exposes. The context is freshly built each iteration, so the lazy cache is
            // cold and the full serialization cost is paid every time.
            black_box(ctx.to_json_string(None));
        }
    }
    black_box(built);
}

fn diagnostics_benchmarks(c: &mut Criterion) {
    let options = Arc::new(DiagnosticsOptions::default());

    // Threshold fast-success: a 1 ms success under a 5 ms threshold. The gate decides NOT to build
    // and `finish` returns before `build_context` — the build is short-circuited, NOT built-then-
    // dropped. Contrast the cost with `capture_built_context` below to see the saving.
    let drop_policy = DiagnosticsPolicy::threshold(Duration::from_millis(5));
    c.bench_function("capture_dropped_fast_success", |b| {
        let pool = Arc::new(LogPool::default());
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

    // Build cost (STRUCT ONLY): materialize the canonical DiagnosticsContext, paid only when the
    // gate fires (error / slow / Always). No JSON is serialized here. The gap between this and the
    // drop benchmark above is the cost the gate saves by deciding BEFORE the build.
    c.bench_function("capture_built_context", |b| {
        let pool = Arc::new(LogPool::default());
        let policy = DiagnosticsPolicy::always();
        let options = Arc::clone(&options);
        b.iter(|| capture_s2(&pool, &policy, &options, false));
    });

    // Build + JSON: the same struct build PLUS serializing it to the detailed JSON string. The
    // delta over `capture_built_context` is the pure serialization cost — the number that decides
    // whether the threshold gate should live in the driver (if the struct build dominates) or stay
    // an on-demand/cached concern in the SDK (if JSON serialization is what is expensive).
    c.bench_function("capture_built_context_and_json", |b| {
        let pool = Arc::new(LogPool::default());
        let policy = DiagnosticsPolicy::always();
        let options = Arc::clone(&options);
        b.iter(|| capture_s2(&pool, &policy, &options, true));
    });
}

criterion_group!(benches, diagnostics_benchmarks);
criterion_main!(benches);
