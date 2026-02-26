// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Concurrent operation execution engine.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_data_cosmos::clients::ContainerClient;
use rand::RngExt;
use serde::Serialize;
use sysinfo::System;
use tokio::task::JoinSet;
use uuid::Uuid;

use crate::operations::Operation;
use crate::stats::{self, Stats};

/// Walks the `std::error::Error::source()` chain and joins messages with " → ".
fn error_source_chain(error: &dyn std::error::Error) -> Option<String> {
    let mut sources = Vec::new();
    let mut current = error.source();
    while let Some(src) = current {
        sources.push(src.to_string());
        current = src.source();
    }
    if sources.is_empty() {
        None
    } else {
        Some(sources.join(" → "))
    }
}

/// Structured perf result document stored in Cosmos DB for long-term monitoring.
#[derive(Debug, Serialize)]
struct PerfResult {
    id: String,
    partition_key: String,
    workload_id: String,
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
    system_cpu_percent: f32,
    system_total_memory_bytes: u64,
    system_used_memory_bytes: u64,
}

/// Error document written to the results container for each individual operation failure.
#[derive(Debug, Serialize)]
struct ErrorResult {
    id: String,
    partition_key: String,
    workload_id: String,
    timestamp: u64,
    operation: String,
    error_message: String,
    source_message: Option<String>,
}

/// Configuration for a perf test run.
pub struct RunConfig {
    pub container: ContainerClient,
    pub operations: Vec<Arc<dyn Operation>>,
    pub stats: Arc<Stats>,
    pub concurrency: usize,
    pub duration: Option<Duration>,
    pub report_interval: Duration,
    pub results_container: ContainerClient,
    pub workload_id: String,
}

/// Runs operations concurrently until cancelled or duration expires.
///
/// Spawns `concurrency` tasks, each continuously picking a random operation
/// from `operations` and executing it against `container`. Latency and errors
/// are recorded in `stats`. A background reporter prints summaries at the
/// given `report_interval` and upserts results into `results_container`.
pub async fn run(config: RunConfig) {
    let RunConfig {
        container,
        operations,
        stats,
        concurrency,
        duration,
        report_interval,
        results_container,
        workload_id,
    } = config;
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
    let report_workload_id = workload_id.clone();
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
            upsert_results(
                &report_results_container,
                &summaries,
                metrics.as_ref(),
                &report_workload_id,
            )
            .await;
        }
    });

    // Spawn fixed worker pool — each worker loops until cancelled
    let start = Instant::now();
    let mut workers = JoinSet::new();

    for _ in 0..concurrency {
        let ops = operations.clone();
        let container = container.clone();
        let stats = stats.clone();
        let cancelled = cancelled.clone();
        let err_container = results_container.clone();
        let err_workload_id = workload_id.clone();

        workers.spawn(async move {
            while !cancelled.load(Ordering::Relaxed) {
                let op_idx = rand::rng().random_range(0..ops.len());
                let op = &ops[op_idx];

                let op_start = Instant::now();
                match op.execute(&container).await {
                    Ok(()) => {
                        stats.record_latency(op.name(), op_start.elapsed());
                    }
                    Err(e) => {
                        stats.record_error(op.name());
                        upsert_error(&err_container, op.name(), &e, &err_workload_id).await;
                    }
                }
            }
        });
    }

    // Wait for all workers to finish
    workers.join_all().await;

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
    upsert_results(
        &results_container,
        &summaries,
        metrics.as_ref(),
        &workload_id,
    )
    .await;

    reporter.abort();
}

/// Upserts perf result documents into the results container.
async fn upsert_results(
    container: &ContainerClient,
    summaries: &[stats::Summary],
    metrics: Option<&stats::ProcessMetrics>,
    workload_id: &str,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (cpu, mem, sys_cpu, sys_total, sys_used) = metrics
        .map(|m| {
            (
                m.cpu_percent,
                m.memory_bytes,
                m.system_cpu_percent,
                m.system_total_memory_bytes,
                m.system_used_memory_bytes,
            )
        })
        .unwrap_or((0.0, 0, 0.0, 0, 0));

    for s in summaries {
        let result = PerfResult {
            id: Uuid::new_v4().to_string(),
            partition_key: s.name.clone(),
            workload_id: workload_id.to_string(),
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
            system_cpu_percent: sys_cpu,
            system_total_memory_bytes: sys_total,
            system_used_memory_bytes: sys_used,
        };

        if let Err(e) = container
            .upsert_item(&result.partition_key, &result, None)
            .await
        {
            eprintln!("Warning: failed to upsert perf result: {e}");
        }
    }
}

/// Writes a single error document to the results container.
///
/// Failures are logged to stderr but never propagated—this must not stop the workload.
async fn upsert_error(
    container: &ContainerClient,
    operation: &str,
    error: &azure_core::Error,
    workload_id: &str,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let id = Uuid::new_v4().to_string();

    let doc = ErrorResult {
        id: id.clone(),
        partition_key: operation.to_string(),
        workload_id: workload_id.to_string(),
        timestamp: now,
        operation: operation.to_string(),
        error_message: format!("{error}"),
        source_message: error_source_chain(error),
    };
    if let Err(e) = container.upsert_item(&doc.partition_key, &doc, None).await {
        eprintln!("Warning: failed to upsert error result: {e}");
    }
}
