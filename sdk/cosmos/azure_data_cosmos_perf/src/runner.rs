// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Concurrent operation execution engine.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_data_cosmos::clients::ContainerClient;
use rand::Rng;

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

    // Start periodic reporter
    let report_stats = stats.clone();
    let report_cancel = cancelled.clone();
    let reporter = tokio::spawn(async move {
        let mut interval = tokio::time::interval(report_interval);
        interval.tick().await; // skip first immediate tick
        loop {
            interval.tick().await;
            if report_cancel.load(Ordering::SeqCst) {
                break;
            }
            println!("\n--- Interval Report ---");
            let summaries = report_stats.drain_summaries();
            stats::print_report(&summaries);
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
    let summaries = stats.drain_summaries();
    stats::print_report(&summaries);

    reporter.abort();
}
