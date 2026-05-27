// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion benchmark comparing the driver's rate-limited
//! [`Backtrace`](azure_data_cosmos_driver::error::backtrace_bench) machinery
//! against [`std::backtrace::Backtrace`].
//!
//! The driver's [`CosmosError`](azure_data_cosmos_driver::error::CosmosError)
//! can capture a backtrace on every construction (opt-in via
//! `RUST_BACKTRACE` or the runtime builder). Two production-safety gates
//! bound the cost during an error storm:
//!
//! * **Capture throttle** — per-second cap on raw stack walks
//!   (`RUST_BACKTRACE`-enabled default `10_000`, `0` to disable); once
//!   exhausted, capture returns `None` for the rest of the 1-second
//!   window.
//! * **Resolution limiter** — per-second cap on *fresh* symbol resolution
//!   work (`RUST_BACKTRACE`-enabled default `5`, `0` to disable). Cache
//!   hits do **not** consume budget — repeat captures of the same call
//!   site render at full fidelity for free.
//! * **Per-instance render cache** — `CosmosError::backtrace()` resolves
//!   once per error and caches via `OnceLock`; later calls are a load.
//!
//! ## Bench groups
//!
//! | Group / variant | What it measures |
//! |---|---|
//! | `capture/cosmos/unbounded`        | Cold capture path with the throttle at default capacity. |
//! | `capture/cosmos/throttle_denied`  | Throttle exhausted (`set_capacity(0)`) — single AtomicU64 CAS denial. This is also the **default production state** when `RUST_BACKTRACE` is unset (capture opt-in). |
//! | `capture/cosmos/inherit_from_source` | End-to-end `CosmosErrorBuilder::with_arc_source(cosmos_err).build()` — the wrapping path skips a fresh capture and inherits the source's `Backtrace`. Proves the re-wrap cost is independent of stack walk. |
//! | `capture/std/force_capture`       | `std::backtrace::Backtrace::force_capture()` baseline (always pays full cost; no cache, no throttle). |
//! | `render/cosmos/cached`            | `Backtrace::rendered()` on the same instance — `OnceLock` hit. |
//! | `render/cosmos/fresh_warm_cache`  | Fresh `Backtrace` per iter, but call site is in the process-global frame cache — pays cache lookup only. |
//! | `render/cosmos/fresh_cold_resolution_denied` | Fresh `Backtrace` per iter with the resolution limiter exhausted — proves the denial fast-path. |
//! | `render/std/to_string`            | `format!("{}", std_bt)` baseline — std has no per-instance render cache, every call walks debug info again. |
//!
//! Run with:
//!
//! ```text
//! cargo bench -p azure_data_cosmos_benchmarks --bench backtrace_capture
//! ```

use azure_data_cosmos_driver::error::{
    backtrace_bench, CosmosError, CosmosErrorBuilder, CosmosStatus,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::{hint::black_box, sync::Arc};

/// Sufficient headroom for the unbounded capture group — set well above the
/// expected per-iteration count so the throttle stays open through the whole
/// measurement window.
const UNBOUNDED_CAPACITY: u32 = 1_000_000;

fn prime_resolution_cache() {
    // Walk once and force a full render so every frame on this call stack
    // lands in the process-global IP-keyed cache. Subsequent fresh captures
    // from the same call site then take the cache-hit path.
    if let Some(bt) = backtrace_bench::capture() {
        let _ = backtrace_bench::render(&bt);
    }
}

fn bench_capture(c: &mut Criterion) {
    let throttle = backtrace_bench::capture_throttle();
    let resolution = backtrace_bench::resolution_limiter();

    let mut group = c.benchmark_group("capture");
    group.throughput(Throughput::Elements(1));

    // --- cosmos_unbounded: throttle wide open, capture pays full cost.
    throttle.set_capacity(UNBOUNDED_CAPACITY);
    backtrace_bench::reset_limiter(throttle);
    resolution.set_capacity(UNBOUNDED_CAPACITY);
    backtrace_bench::reset_limiter(resolution);
    group.bench_function(BenchmarkId::new("cosmos", "unbounded"), |b| {
        b.iter(|| {
            let bt = backtrace_bench::capture();
            black_box(bt)
        });
    });

    // --- cosmos_throttle_denied: throttle exhausted, capture returns None
    // after one AtomicU64 CAS denial. This is also the default production
    // state when `RUST_BACKTRACE` is unset (capture is opt-in).
    throttle.set_capacity(0);
    group.bench_function(BenchmarkId::new("cosmos", "throttle_denied"), |b| {
        b.iter(|| {
            let bt = backtrace_bench::capture();
            black_box(bt)
        });
    });
    // Restore throttle so later groups are not affected.
    throttle.set_capacity(UNBOUNDED_CAPACITY);
    backtrace_bench::reset_limiter(throttle);

    // --- cosmos_inherit_from_source: re-wrap path. When a `CosmosError`
    // is built with another `CosmosError` as its `Arc` source, the new
    // error inherits the source's backtrace instead of paying for a fresh
    // stack walk. Measures the end-to-end builder cost on this path.
    let inner = Arc::new(
        CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
            .with_message("inner")
            .build(),
    );
    group.bench_function(BenchmarkId::new("cosmos", "inherit_from_source"), |b| {
        b.iter(|| {
            let outer = CosmosErrorBuilder::from_error(CosmosError::builder().build())
                .with_arc_source(Arc::clone(&inner) as Arc<dyn std::error::Error + Send + Sync>)
                .with_message("outer")
                .build();
            black_box(outer)
        });
    });

    // --- std baseline: force_capture always walks the stack and produces an
    // unresolved Backtrace; resolution happens on Display.
    group.bench_function(BenchmarkId::new("std", "force_capture"), |b| {
        b.iter(|| {
            let bt = std::backtrace::Backtrace::force_capture();
            black_box(bt)
        });
    });

    group.finish();
}

fn bench_render(c: &mut Criterion) {
    let throttle = backtrace_bench::capture_throttle();
    let resolution = backtrace_bench::resolution_limiter();

    let mut group = c.benchmark_group("render");
    group.throughput(Throughput::Elements(1));

    // Make sure the throttle is open for the setup captures below.
    throttle.set_capacity(UNBOUNDED_CAPACITY);
    backtrace_bench::reset_limiter(throttle);
    resolution.set_capacity(UNBOUNDED_CAPACITY);
    backtrace_bench::reset_limiter(resolution);

    // Prime the process-global frame cache for all subsequent groups so the
    // "fresh-Backtrace-but-cache-hit" path is hot.
    prime_resolution_cache();

    // --- cosmos_cached: single Backtrace, repeated render is a OnceLock hit.
    let warm_bt = backtrace_bench::capture().expect("capture must succeed when throttle is open");
    // First render seeds the OnceLock so the measurement loop only times the
    // cache hit path.
    let _ = backtrace_bench::render(&warm_bt);
    group.bench_function(BenchmarkId::new("cosmos", "cached"), |b| {
        b.iter(|| {
            let rendered = backtrace_bench::render(&warm_bt);
            black_box(rendered)
        });
    });

    // --- cosmos_fresh_warm_cache: fresh Backtrace per iter but every frame
    // is in the process-global IP-keyed cache, so render takes the cache-hit
    // path (no resolution work, no budget consumption).
    group.bench_function(BenchmarkId::new("cosmos", "fresh_warm_cache"), |b| {
        b.iter(|| {
            let bt = backtrace_bench::capture().expect("capture must succeed");
            let rendered = backtrace_bench::render(&bt);
            black_box(rendered)
        });
    });

    // --- cosmos_fresh_cold_resolution_denied: fresh Backtrace per iter with
    // the resolution limiter exhausted. Even if the cache is warm for this
    // call site, the denial path returns immediately without re-rendering.
    // Demonstrates the "no partial backtraces" guarantee + the cheap denial.
    resolution.set_capacity(0);
    group.bench_function(
        BenchmarkId::new("cosmos", "fresh_cold_resolution_denied"),
        |b| {
            b.iter(|| {
                let bt = backtrace_bench::capture().expect("capture must succeed");
                let rendered = backtrace_bench::render(&bt);
                black_box(rendered)
            });
        },
    );
    // Restore the limiter so later or repeated runs are not affected.
    resolution.set_capacity(UNBOUNDED_CAPACITY);
    backtrace_bench::reset_limiter(resolution);

    // --- std baseline: capture once, render via Display on every iteration.
    // std::backtrace has no per-instance render cache, so each `to_string`
    // re-walks debug info; this is the apples-to-apples comparison for the
    // "render the same backtrace many times" pattern.
    let std_bt = std::backtrace::Backtrace::force_capture();
    group.bench_function(BenchmarkId::new("std", "to_string"), |b| {
        b.iter(|| {
            let s = std_bt.to_string();
            black_box(s)
        });
    });

    group.finish();
}

criterion_group!(benches, bench_capture, bench_render);
criterion_main!(benches);
