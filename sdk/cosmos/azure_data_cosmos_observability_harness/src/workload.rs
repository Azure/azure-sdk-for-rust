// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Workload: database/container setup, seeding, and the concurrent op-mix loop.

use std::error::Error;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::{ContainerClient, DatabaseClient};
use azure_data_cosmos::feed::FeedScope;
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::options::CreateContainerOptions;
use azure_data_cosmos::CosmosClient;
use futures::StreamExt;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use uuid::Uuid;

use crate::config::Config;

/// Number of logical partitions the seeded data is spread across, so
/// single-partition queries return several items.
const PARTITION_COUNT: usize = 16;

const MAX_SETUP_RETRIES: u32 = 10;
const INITIAL_BACKOFF: Duration = Duration::from_secs(1);
const MAX_BACKOFF: Duration = Duration::from_secs(30);

/// The document shape written and read by the harness.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SoakItem {
    id: String,
    partition_key: String,
    value: u64,
}

/// A reference to a seeded item so operations can target it.
#[derive(Debug, Clone)]
struct SeededItem {
    id: String,
    partition_key: String,
}

/// The operation kinds exercised by the workload.
#[derive(Debug, Clone, Copy)]
enum OpKind {
    Read,
    Write,
    Query,
}

/// Aggregated, lock-free counters surfaced in the periodic console report. The
/// authoritative telemetry is the OpenTelemetry metrics emitted by the SDK; these
/// counters are only a coarse progress signal.
#[derive(Default)]
struct Stats {
    reads_ok: AtomicU64,
    reads_err: AtomicU64,
    writes_ok: AtomicU64,
    writes_err: AtomicU64,
    queries_ok: AtomicU64,
    queries_err: AtomicU64,
}

impl Stats {
    fn record(&self, op: OpKind, ok: bool) {
        let counter = match (op, ok) {
            (OpKind::Read, true) => &self.reads_ok,
            (OpKind::Read, false) => &self.reads_err,
            (OpKind::Write, true) => &self.writes_ok,
            (OpKind::Write, false) => &self.writes_err,
            (OpKind::Query, true) => &self.queries_ok,
            (OpKind::Query, false) => &self.queries_err,
        };
        counter.fetch_add(1, Ordering::Relaxed);
    }

    fn total_ops(&self) -> u64 {
        self.reads_ok.load(Ordering::Relaxed)
            + self.writes_ok.load(Ordering::Relaxed)
            + self.queries_ok.load(Ordering::Relaxed)
    }

    fn total_errors(&self) -> u64 {
        self.reads_err.load(Ordering::Relaxed)
            + self.writes_err.load(Ordering::Relaxed)
            + self.queries_err.load(Ordering::Relaxed)
    }
}

/// Runs the full workload: setup, seed, then the load loop until the duration
/// elapses or Ctrl+C is received.
pub async fn run(client: &CosmosClient, config: &Config) -> Result<(), Box<dyn Error>> {
    ensure_database(client, &config.database).await?;
    let db_client = client.database_client(&config.database);
    let container = ensure_container(&db_client, &config.container, config.throughput).await?;

    let seeded = Arc::new(seed(&container, config.seed_count, config.concurrency).await?);

    let stats = Arc::new(Stats::default());
    let cancelled = Arc::new(AtomicBool::new(false));

    spawn_cancellation(cancelled.clone(), config.duration_secs);
    let reporter = spawn_reporter(
        stats.clone(),
        cancelled.clone(),
        Duration::from_secs(config.report_interval_secs.max(1)),
    );

    // Per-worker pacing to approximate the requested aggregate RPS.
    let per_worker_delay = if config.rps > 0.0 {
        Some(Duration::from_secs_f64(
            config.concurrency as f64 / config.rps,
        ))
    } else {
        None
    };

    let weights = OpWeights::new(config.read_weight, config.write_weight, config.query_weight);

    println!(
        "Starting workload: concurrency={}, mix(read/write/query)={}/{}/{}, rps={}",
        config.concurrency,
        config.read_weight,
        config.write_weight,
        config.query_weight,
        if config.rps > 0.0 {
            format!("{:.0}", config.rps)
        } else {
            "max".to_string()
        },
    );

    let start = Instant::now();
    let mut workers = JoinSet::new();
    for _ in 0..config.concurrency {
        let container = container.clone();
        let seeded = seeded.clone();
        let stats = stats.clone();
        let cancelled = cancelled.clone();
        workers.spawn(async move {
            // Box the worker future: the query path makes it large enough to trip
            // clippy's `large_futures` lint when held inline in the task.
            Box::pin(worker_loop(
                container,
                seeded,
                stats,
                cancelled,
                weights,
                per_worker_delay,
            ))
            .await;
        });
    }
    workers.join_all().await;

    reporter.abort();
    print_final_report(&stats, start.elapsed());
    Ok(())
}

/// Precomputed cumulative weights for the operation mix.
#[derive(Clone, Copy)]
struct OpWeights {
    read: u32,
    read_write: u32,
    total: u32,
}

impl OpWeights {
    fn new(read: u32, write: u32, query: u32) -> Self {
        Self {
            read,
            read_write: read + write,
            total: read + write + query,
        }
    }

    fn pick(&self) -> OpKind {
        let roll = rand::rng().random_range(0..self.total);
        if roll < self.read {
            OpKind::Read
        } else if roll < self.read_write {
            OpKind::Write
        } else {
            OpKind::Query
        }
    }
}

/// A single worker: repeatedly picks and executes an operation until cancelled.
async fn worker_loop(
    container: ContainerClient,
    seeded: Arc<Vec<SeededItem>>,
    stats: Arc<Stats>,
    cancelled: Arc<AtomicBool>,
    weights: OpWeights,
    per_worker_delay: Option<Duration>,
) {
    while !cancelled.load(Ordering::Relaxed) {
        let op_start = Instant::now();
        let op = weights.pick();
        let ok = execute(&container, &seeded, op).await;
        stats.record(op, ok);

        if let Some(delay) = per_worker_delay {
            // Pace to a steady cadence, accounting for the time the op took.
            tokio::time::sleep_until((op_start + delay).into()).await;
        }
    }
}

/// Executes a single operation, returning whether it succeeded. Errors are
/// intentionally swallowed here — the SDK's diagnostics handlers already emit the
/// rich per-failure telemetry that this harness exists to validate.
async fn execute(container: &ContainerClient, seeded: &[SeededItem], op: OpKind) -> bool {
    match op {
        OpKind::Read => {
            let item = random_item(seeded);
            container
                .read_item(item.partition_key.clone(), &item.id, None)
                .await
                .is_ok()
        }
        OpKind::Write => {
            let item = random_item(seeded);
            let doc = SoakItem {
                id: item.id.clone(),
                partition_key: item.partition_key.clone(),
                value: rand::rng().random_range(0..u64::MAX),
            };
            container
                .upsert_item(item.partition_key.clone(), &item.id, &doc, None)
                .await
                .is_ok()
        }
        OpKind::Query => {
            let partition_key = random_item(seeded).partition_key;
            match container
                .query_items::<SoakItem>(
                    "SELECT * FROM c",
                    FeedScope::partition(partition_key),
                    None,
                )
                .await
            {
                Ok(iterator) => {
                    let mut pages = iterator.into_pages();
                    while let Some(page) = pages.next().await {
                        if page.is_err() {
                            return false;
                        }
                    }
                    true
                }
                Err(_) => false,
            }
        }
    }
}

/// Returns a random seeded item reference.
fn random_item(seeded: &[SeededItem]) -> SeededItem {
    let idx = rand::rng().random_range(0..seeded.len());
    seeded[idx].clone()
}

/// Spawns tasks that flip `cancelled` on Ctrl+C and (optionally) after
/// `duration_secs`.
fn spawn_cancellation(cancelled: Arc<AtomicBool>, duration_secs: u64) {
    let ctrl_c_flag = cancelled.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            println!("\nCtrl+C received, shutting down...");
            ctrl_c_flag.store(true, Ordering::SeqCst);
        }
    });

    if duration_secs > 0 {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(duration_secs)).await;
            println!("\nDuration elapsed, shutting down...");
            cancelled.store(true, Ordering::SeqCst);
        });
    }
}

/// Spawns the periodic console reporter.
fn spawn_reporter(
    stats: Arc<Stats>,
    cancelled: Arc<AtomicBool>,
    interval: Duration,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        ticker.tick().await; // skip the immediate first tick
        let mut last_ops = 0u64;
        let mut last = Instant::now();
        loop {
            ticker.tick().await;
            if cancelled.load(Ordering::SeqCst) {
                break;
            }
            let now = Instant::now();
            let total = stats.total_ops();
            let errors = stats.total_errors();
            let elapsed = now
                .duration_since(last)
                .as_secs_f64()
                .max(f64::MIN_POSITIVE);
            let rate = (total - last_ops) as f64 / elapsed;
            println!(
                "[report] ops={total} errors={errors} throughput={rate:.0}/s \
                 (r_ok={} w_ok={} q_ok={})",
                stats.reads_ok.load(Ordering::Relaxed),
                stats.writes_ok.load(Ordering::Relaxed),
                stats.queries_ok.load(Ordering::Relaxed),
            );
            last_ops = total;
            last = now;
        }
    })
}

/// Prints the closing summary.
fn print_final_report(stats: &Stats, elapsed: Duration) {
    let total = stats.total_ops();
    let errors = stats.total_errors();
    let rate = total as f64 / elapsed.as_secs_f64().max(f64::MIN_POSITIVE);
    println!("\n=== Final report ({:.1}s) ===", elapsed.as_secs_f64());
    println!("  total ops:   {total} ({rate:.0}/s)");
    println!("  total errors:{errors}");
    println!(
        "  reads:   ok={} err={}",
        stats.reads_ok.load(Ordering::Relaxed),
        stats.reads_err.load(Ordering::Relaxed),
    );
    println!(
        "  writes:  ok={} err={}",
        stats.writes_ok.load(Ordering::Relaxed),
        stats.writes_err.load(Ordering::Relaxed),
    );
    println!(
        "  queries: ok={} err={}",
        stats.queries_ok.load(Ordering::Relaxed),
        stats.queries_err.load(Ordering::Relaxed),
    );
}

/// Seeds `count` documents across [`PARTITION_COUNT`] logical partitions.
async fn seed(
    container: &ContainerClient,
    count: usize,
    concurrency: usize,
) -> Result<Vec<SeededItem>, Box<dyn Error>> {
    println!("Seeding {count} items across {PARTITION_COUNT} partitions...");
    let seeded: Vec<SeededItem> = (0..count)
        .map(|i| SeededItem {
            id: Uuid::new_v4().to_string(),
            partition_key: format!("pk-{}", i % PARTITION_COUNT),
        })
        .collect();

    let mut workers = JoinSet::new();
    let mut next = 0;
    while next < count || !workers.is_empty() {
        while next < count && workers.len() < concurrency.max(1) {
            let container = container.clone();
            let item = seeded[next].clone();
            let value = next as u64;
            workers.spawn(async move {
                let doc = SoakItem {
                    id: item.id.clone(),
                    partition_key: item.partition_key.clone(),
                    value,
                };
                container
                    .upsert_item(doc.partition_key.clone(), &doc.id, &doc, None)
                    .await
                    .map(|_| ())
            });
            next += 1;
        }
        if let Some(joined) = workers.join_next().await {
            match joined {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    workers.abort_all();
                    return Err(e.into());
                }
                Err(e) => {
                    workers.abort_all();
                    return Err(format!("seed worker task failed: {e}").into());
                }
            }
        }
    }
    println!("Seeding complete.");
    Ok(seeded)
}

/// Ensures a database exists, creating it if necessary, with retry for
/// multi-region replication lag.
async fn ensure_database(client: &CosmosClient, db_name: &str) -> Result<(), Box<dyn Error>> {
    let db_client = client.database_client(db_name);
    match db_client.read(None).await {
        Ok(_) => {
            println!("Database '{db_name}' already exists.");
            return Ok(());
        }
        Err(e) if is_not_found(&e) => println!("Database '{db_name}' not found, creating..."),
        Err(e) => return Err(e.into()),
    }

    match client.create_database(db_name, None).await {
        Ok(_) => println!("Database '{db_name}' created."),
        Err(e) if is_conflict(&e) => println!("Database '{db_name}' created concurrently."),
        Err(e) => return Err(e.into()),
    }

    let mut backoff = INITIAL_BACKOFF;
    for attempt in 1..=MAX_SETUP_RETRIES {
        match db_client.read(None).await {
            Ok(_) => return Ok(()),
            Err(e) if is_not_found(&e) => {
                println!("Database not yet visible (attempt {attempt}/{MAX_SETUP_RETRIES})...");
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
            Err(e) => return Err(e.into()),
        }
    }
    Err(format!("database '{db_name}' not readable after {MAX_SETUP_RETRIES} retries").into())
}

/// Ensures a container exists, creating it if necessary, and returns a resolved
/// [`ContainerClient`].
async fn ensure_container(
    db_client: &DatabaseClient,
    container_name: &str,
    throughput: usize,
) -> Result<ContainerClient, Box<dyn Error>> {
    match db_client.container_client(container_name).await {
        Ok(container) => match container.read(None).await {
            Ok(_) => {
                println!("Container '{container_name}' already exists.");
                return Ok(container);
            }
            Err(e) if is_not_found(&e) => {}
            Err(e) => return Err(e.into()),
        },
        Err(e) if is_not_found(&e) => {}
        Err(e) => return Err(e.into()),
    }

    let props = ContainerProperties::new(container_name.to_string(), "/partition_key".into());
    let create_opts =
        CreateContainerOptions::default().with_throughput(ThroughputProperties::manual(throughput));
    match db_client.create_container(props, Some(create_opts)).await {
        Ok(_) => println!("Container '{container_name}' created ({throughput} RU/s)."),
        Err(e) if is_conflict(&e) => println!("Container '{container_name}' created concurrently."),
        Err(e) => return Err(e.into()),
    }

    let mut backoff = INITIAL_BACKOFF;
    for attempt in 1..=MAX_SETUP_RETRIES {
        match db_client.container_client(container_name).await {
            Ok(container) => match container.read(None).await {
                Ok(_) => {
                    println!("Container '{container_name}' confirmed readable.");
                    return Ok(container);
                }
                Err(e) if is_not_found(&e) => {
                    println!(
                        "Container not yet readable (attempt {attempt}/{MAX_SETUP_RETRIES})..."
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                }
                Err(e) => return Err(e.into()),
            },
            Err(e) if is_not_found(&e) => {
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
            Err(e) => return Err(e.into()),
        }
    }
    Err(
        format!("container '{container_name}' not readable after {MAX_SETUP_RETRIES} retries")
            .into(),
    )
}

fn is_not_found(error: &azure_data_cosmos::CosmosError) -> bool {
    error.status().status_code() == StatusCode::NotFound
}

fn is_conflict(error: &azure_data_cosmos::CosmosError) -> bool {
    error.status().status_code() == StatusCode::Conflict
}
