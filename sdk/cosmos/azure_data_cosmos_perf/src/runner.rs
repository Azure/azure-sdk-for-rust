// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Concurrent operation execution engine.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_data_cosmos::clients::ContainerClient;
use rand::Rng;
use sysinfo::System;

use crate::operations::Operation;
use crate::stats::{self, Stats};

/// Runs operations concurrently until cancelled or duration expires.
///
/// Spawns `concurrency` tasks, each continuously picking a random operation
/// from `operations` and executing it against `container`. Latency and errors
/// are recorded in `stats`. A background reporter prints summaries at the
/// given `report_interval`.
pub async fn run(
    container: ContainerClient,
    operations: Vec<Arc<dyn Operation>>,
    stats: Arc<Stats>,
    concurrency: usize,
    duration: Option<Duration>,
    report_interval: Duration,
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

    // Create CSV report file
    let csv_path = stats::create_report_file().expect("failed to create CSV report file");
    println!("Report file: {}", csv_path.display());

    // Start periodic reporter
    let report_stats = stats.clone();
    let report_cancel = cancelled.clone();
    let report_csv_path = csv_path.clone();
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
            if let Err(e) = stats::append_csv(&report_csv_path, &summaries, metrics.as_ref()) {
                eprintln!("Warning: failed to write CSV: {e}");
            }
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
    if let Err(e) = stats::append_csv(&csv_path, &summaries, metrics.as_ref()) {
        eprintln!("Warning: failed to write final CSV: {e}");
    }
    println!("Report saved to: {}", csv_path.display());

    reporter.abort();
}
