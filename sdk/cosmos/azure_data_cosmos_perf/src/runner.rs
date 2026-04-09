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
    commit_sha: String,
    hostname: String,
    #[serde(rename = "TIMESTAMP")]
    timestamp: String,
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
    // Tokio runtime metrics (present only when tokio-metrics feature is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    tokio_workers: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tokio_busy_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tokio_park_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tokio_queue_depth: Option<u64>,
    // Runtime configuration snapshot (for Grafana dashboard)
    #[serde(skip_serializing_if = "Option::is_none")]
    config_concurrency: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_application_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_excluded_regions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_tokio_threads: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_ppcb_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_gateway20_allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_pyroscope_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_tokio_console_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_tokio_metrics_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_valgrind_tool: Option<String>,
}

/// Tokio runtime metrics snapshot for a single reporting interval.
#[derive(Clone, Copy)]
struct TokioSnapshot {
    workers: u64,
    busy_pct: f64,
    park_count: u64,
    queue_depth: u64,
}

/// Error document written to the results container for each individual operation failure.
#[derive(Debug, Serialize)]
struct ErrorResult {
    id: String,
    partition_key: String,
    workload_id: String,
    commit_sha: String,
    hostname: String,
    #[serde(rename = "TIMESTAMP")]
    timestamp: String,
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
    pub commit_sha: String,
    pub hostname: String,
    pub config_snapshot: ConfigSnapshot,
}

/// Snapshot of runtime configuration emitted with each PerfResult document.
#[derive(Clone)]
pub struct ConfigSnapshot {
    pub concurrency: u64,
    pub application_region: String,
    pub excluded_regions: String,
    pub tokio_threads: u64,
    pub ppcb_enabled: bool,
    pub gateway20_allowed: bool,
    pub pyroscope_enabled: bool,
    pub tokio_console_enabled: bool,
    pub tokio_metrics_enabled: bool,
    pub valgrind_tool: String,
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
        commit_sha,
        hostname,
        config_snapshot,
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

    // Set up tokio runtime metrics monitor
    #[cfg(feature = "tokio-metrics")]
    let runtime_monitor = tokio_metrics::RuntimeMonitor::new(&tokio::runtime::Handle::current());

    // Start periodic reporter
    let report_stats = stats.clone();
    let report_cancel = cancelled.clone();
    let report_results_container = results_container.clone();
    let report_workload_id = workload_id.clone();
    let report_commit_sha = commit_sha.clone();
    let report_hostname = hostname.clone();
    let report_config = config_snapshot.clone();
    #[cfg(feature = "tokio-metrics")]
    let mut runtime_intervals = runtime_monitor.intervals();

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

            // Collect tokio runtime metrics (delta since last interval)
            #[cfg(feature = "tokio-metrics")]
            let tokio_fields = {
                runtime_intervals.next().map(|rt| {
                    let workers = rt.workers_count as u64;
                    let elapsed_secs = rt.elapsed.as_secs_f64();
                    let busy_nanos = rt.total_busy_duration.as_nanos() as f64;
                    let total_nanos = elapsed_secs * 1e9 * workers as f64;
                    let busy_pct = if total_nanos > 0.0 {
                        busy_nanos / total_nanos * 100.0
                    } else {
                        0.0
                    };
                    println!(
                        "  Tokio:   Workers={}, Busy={:.1}%, ParkCount={}, QueueDepth={}",
                        workers, busy_pct, rt.total_park_count, rt.global_queue_depth
                    );
                    TokioSnapshot {
                        workers,
                        busy_pct,
                        park_count: rt.total_park_count,
                        queue_depth: rt.global_queue_depth as u64,
                    }
                })
            };
            #[cfg(not(feature = "tokio-metrics"))]
            let tokio_fields: Option<TokioSnapshot> = None;

            let summaries = report_stats.drain_summaries();
            stats::print_report(&summaries);
            upsert_results(
                &report_results_container,
                &summaries,
                metrics.as_ref(),
                tokio_fields,
                &report_config,
                &report_workload_id,
                &report_commit_sha,
                &report_hostname,
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
        let err_commit_sha = commit_sha.clone();
        let err_hostname = hostname.clone();

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
                        upsert_error(
                            &err_container,
                            op.name(),
                            &e,
                            &err_workload_id,
                            &err_commit_sha,
                            &err_hostname,
                        )
                        .await;
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
        None,
        &config_snapshot,
        &workload_id,
        &commit_sha,
        &hostname,
    )
    .await;

    reporter.abort();
}

/// Upserts perf result documents into the results container.
#[allow(clippy::too_many_arguments)]
async fn upsert_results(
    container: &ContainerClient,
    summaries: &[stats::Summary],
    metrics: Option<&stats::ProcessMetrics>,
    tokio_fields: Option<TokioSnapshot>,
    config: &ConfigSnapshot,
    workload_id: &str,
    commit_sha: &str,
    hostname: &str,
) {
    let now = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .expect("RFC 3339 formatting should never fail");
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
            partition_key: Uuid::new_v4().to_string(),
            workload_id: workload_id.to_string(),
            commit_sha: commit_sha.to_string(),
            hostname: hostname.to_string(),
            timestamp: now.clone(),
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
            tokio_workers: tokio_fields.map(|t| t.workers),
            tokio_busy_pct: tokio_fields.map(|t| t.busy_pct),
            tokio_park_count: tokio_fields.map(|t| t.park_count),
            tokio_queue_depth: tokio_fields.map(|t| t.queue_depth),
            config_concurrency: Some(config.concurrency),
            config_application_region: Some(config.application_region.clone()),
            config_excluded_regions: if config.excluded_regions.is_empty() {
                None
            } else {
                Some(config.excluded_regions.clone())
            },
            config_tokio_threads: Some(config.tokio_threads),
            config_ppcb_enabled: Some(config.ppcb_enabled),
            config_gateway20_allowed: Some(config.gateway20_allowed),
            config_pyroscope_enabled: Some(config.pyroscope_enabled),
            config_tokio_console_enabled: Some(config.tokio_console_enabled),
            config_tokio_metrics_enabled: Some(config.tokio_metrics_enabled),
            config_valgrind_tool: if config.valgrind_tool.is_empty() {
                None
            } else {
                Some(config.valgrind_tool.clone())
            },
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
    commit_sha: &str,
    hostname: &str,
) {
    let now = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .expect("RFC 3339 formatting should never fail");
    let id = Uuid::new_v4().to_string();

    let doc = ErrorResult {
        id: id.clone(),
        partition_key: Uuid::new_v4().to_string(),
        workload_id: workload_id.to_string(),
        commit_sha: commit_sha.to_string(),
        hostname: hostname.to_string(),
        timestamp: now,
        operation: operation.to_string(),
        error_message: format!("{error}"),
        source_message: error_source_chain(error),
    };
    if let Err(e) = container.upsert_item(&doc.partition_key, &doc, None).await {
        eprintln!("Warning: failed to upsert error result: {e}");
    }
}
