// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Concurrent operation execution engine.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_data_cosmos::clients::ContainerClient;
use rand::Rng;
use serde::Serialize;
use sysinfo::System;
use uuid::Uuid;

use crate::operations::Operation;
use crate::stats::{self, Stats};

/// Structured perf result document stored in Cosmos DB for long-term monitoring.
#[derive(Debug, Serialize)]
struct PerfResult {
    id: String,
    timestamp: u64,
    operation: String,
    count: u64,
    errors: u64,
    min_ms: f64,
    max_ms: f64,
    mean_ms: f64,
    p50_ms: f64,
    p90_ms: f64,
    p99_ms: f64,
    cpu_percent: f32,
    memory_bytes: u64,
}

/// Runs operations concurrently until cancelled or duration expires.
///
/// Spawns `concurrency` tasks, each continuously picking a random operation
/// from `operations` and executing it against `container`. Latency and errors
/// are recorded in `stats`. A background reporter prints summaries at the
/// given `report_interval` and upserts results into `results_container`.
pub async fn run(
    container: ContainerClient,
    operations: Vec<Arc<dyn Operation>>,
    stats: Arc<Stats>,
    concurrency: usize,
    duration: Option<Duration>,
    report_interval: Duration,
    results_container: ContainerClient,
) {
    let cancelled = Arc::new(AtomicBool::new(false));

    // Set up Ctrl+C handler
    let cancel_flag = cancelled.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for Ctrl+C");
        println!("\nShutting down...");
        cancel_flag.store(true, Ordering::SeqCst);
    });

    // Set up duration-based cancellation
    if let Some(dur) = duration {
        let cancel_flag = cancelled.clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;
            println!("\nDuration elapsed, shutting down...");
            cancel_flag.store(true, Ordering::SeqCst);
        });
    }

    // Start periodic reporter
    let report_stats = stats.clone();
    let report_cancel = cancelled.clone();
    let report_results_container = results_container.clone();
    let reporter = tokio::spawn(async move {
        let mut sys = System::new();
        let mut interval = tokio::time::interval(report_interval);
        interval.tick().await; // skip first immediate tick
        loop {
            interval.tick().await;
            if report_cancel.load(Ordering::SeqCst) {
                break;
            }
            println!("\n--- Interval Report ---");
            let metrics = stats::refresh_process_metrics(&mut sys);
            if let Some(ref m) = metrics {
                stats::print_process_metrics(m);
            }
            let summaries = report_stats.drain_summaries();
            stats::print_report(&summaries);
            upsert_results(&report_results_container, &summaries, metrics.as_ref()).await;
        }
    });

    // Spawn worker tasks
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
    let start = Instant::now();

    // Main loop: keep spawning operations until cancelled
    loop {
        if cancelled.load(Ordering::SeqCst) {
            break;
        }

        let permit = match semaphore.clone().acquire_owned().await {
            Ok(p) => p,
            Err(_) => break,
        };

        let ops = operations.clone();
        let container = container.clone();
        let stats = stats.clone();
        let cancelled = cancelled.clone();

        tokio::spawn(async move {
            let _permit = permit;

            if cancelled.load(Ordering::Relaxed) {
                return;
            }

            let op_idx = rand::rng().random_range(0..ops.len());
            let op = &ops[op_idx];

            let op_start = Instant::now();
            match op.execute(&container).await {
                Ok(()) => {
                    stats.record_latency(op.name(), op_start.elapsed());
                }
                Err(_) => {
                    stats.record_error(op.name());
                }
            }
        });
    }

    // Wait for in-flight operations by acquiring all permits
    let _ = semaphore.acquire_many(concurrency as u32).await;

    // Print final report
    let total_elapsed = start.elapsed();
    println!(
        "\n=== Final Report (total: {:.1}s) ===",
        total_elapsed.as_secs_f64()
    );
    let mut sys = System::new();
    let metrics = stats::refresh_process_metrics(&mut sys);
    if let Some(ref m) = metrics {
        stats::print_process_metrics(m);
    }
    let summaries = stats.drain_summaries();
    stats::print_report(&summaries);
    upsert_results(&results_container, &summaries, metrics.as_ref()).await;

    reporter.abort();
}

/// Upserts perf result documents into the results container.
async fn upsert_results(
    container: &ContainerClient,
    summaries: &[stats::Summary],
    metrics: Option<&stats::ProcessMetrics>,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (cpu, mem) = metrics
        .map(|m| (m.cpu_percent, m.memory_bytes))
        .unwrap_or((0.0, 0));

    for s in summaries {
        let result = PerfResult {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            operation: s.name.clone(),
            count: s.count,
            errors: s.errors,
            min_ms: s.min.as_secs_f64() * 1000.0,
            max_ms: s.max.as_secs_f64() * 1000.0,
            mean_ms: s.mean.as_secs_f64() * 1000.0,
            p50_ms: s.p50.as_secs_f64() * 1000.0,
            p90_ms: s.p90.as_secs_f64() * 1000.0,
            p99_ms: s.p99.as_secs_f64() * 1000.0,
            cpu_percent: cpu,
            memory_bytes: mem,
        };

        if let Err(e) = container.upsert_item(&result.id, &result, None).await {
            eprintln!("Warning: failed to upsert perf result: {e}");
        }
    }
}
