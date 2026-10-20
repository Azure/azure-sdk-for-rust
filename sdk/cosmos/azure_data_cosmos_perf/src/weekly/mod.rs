// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod contract;
mod workload;

use std::{
    future::Future,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, RwLock,
    },
    time::Duration,
};

use azure_core::{credentials::Secret, http::Context};
use azure_data_cosmos::{
    diagnostics::{DiagnosticsContext, DiagnosticsHandler},
    models::{ContainerProperties, ThroughputProperties},
    options::CreateContainerOptions,
    AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder, RoutingStrategy,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    sync::{watch, Barrier},
    task::JoinSet,
    time::Instant,
};

use self::{
    contract::{evaluate, Baseline, Environment, Metric, Operation, Profile},
    workload::Worker,
};
use crate::{
    config::{AuthMethod, Config},
    operations::PerfItem,
    stats::Stats,
};

type Error = Box<dyn std::error::Error>;

struct RoutingEvidence {
    region: String,
    enabled: AtomicBool,
    gateway_v2: AtomicU64,
    remote_requests: AtomicU64,
}

impl DiagnosticsHandler for RoutingEvidence {
    fn handle(&self, diagnostics: &DiagnosticsContext, _cx: &Context<'_>) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }
        for request in diagnostics.requests().iter() {
            if !request.pipeline_type().is_data_plane() {
                continue;
            }
            if request.transport_kind().is_gateway_v2() {
                self.gateway_v2.fetch_add(1, Ordering::Relaxed);
            }
            if request
                .region()
                .is_none_or(|r| normalize_region(r.as_ref()) != self.region)
            {
                self.remote_requests.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

fn normalize_region(region: &str) -> String {
    region
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

pub(crate) async fn run(config: &Config) -> Result<(), Error> {
    let profile: Profile =
        read_json(config.perf_profile.as_ref().ok_or("missing profile")?).await?;
    profile.validate()?;
    let baseline: Option<Baseline> = if config.calibrate {
        None
    } else {
        let baseline: Baseline = read_json(
            config
                .thresholds
                .as_ref()
                .ok_or("thresholds are required unless --calibrate is set")?,
        )
        .await?;
        baseline.validate(&profile)?;
        Some(baseline)
    };
    let environment: Environment = read_json(
        config
            .perf_environment
            .as_ref()
            .ok_or("--perf-environment is required")?,
    )
    .await?;
    if environment.vm_size != profile.vm_size
        || environment.consistency != profile.consistency
        || environment.endpoint.trim_end_matches('/') != config.endpoint.trim_end_matches('/')
        || normalize_region(&environment.region) != normalize_region(&config.application_region)
        || config.commit_sha.as_ref() != Some(&environment.source_commit)
        || !valid_sha(&environment.source_commit)
        || !valid_sha(&environment.baseline_commit)
        || !environment.matches_account_endpoint()
    {
        return Err(
            "preflight environment does not match source, endpoint, region, or profile".into(),
        );
    }
    let output = config
        .perf_output
        .as_ref()
        .ok_or("--perf-output is required")?;
    let output_copy = output.clone();
    tokio::task::spawn_blocking(move || std::fs::create_dir_all(output_copy)).await??;
    if !matches!(config.auth, AuthMethod::Key) {
        return Err("the weekly profile requires the fixed account's key authentication".into());
    }
    let key = config.key.as_ref().ok_or("AZURE_COSMOS_KEY is required")?;
    let evidence = Arc::new(RoutingEvidence {
        region: normalize_region(&environment.region),
        enabled: AtomicBool::new(false),
        gateway_v2: AtomicU64::new(0),
        remote_requests: AtomicU64::new(0),
    });
    let endpoint: AccountEndpoint = config.endpoint.parse()?;
    let client = CosmosClientBuilder::new()
        .with_diagnostics_handler(evidence.clone())
        .build(
            AccountReference::with_authentication_key(endpoint, Secret::from(key.clone())),
            RoutingStrategy::ProximityTo(environment.region.clone().into()),
        )
        .await?;
    let database = format!("cosmos-perf-{}", uuid::Uuid::new_v4());
    let ownership = serde_json::json!({
        "database": database,
        "workload_id": config.workload_id,
    });
    let outcome = async {
        client.create_database(&database, None).await?;
        let workers = setup(&client, &database, &profile).await?;
        evidence.enabled.store(true, Ordering::Relaxed);
        // Prove point requests negotiated V2 before spending the measurement window.
        let probe = workers
            .iter()
            .find(|w| w.operation == Operation::ReadItem)
            .ok_or("missing read probe")?;
        probe
            .container
            .read_item(&probe.item.partition_key, &probe.item.id, None)
            .await?;
        if evidence.gateway_v2.load(Ordering::Relaxed) == 0
            || evidence.remote_requests.load(Ordering::Relaxed) != 0
        {
            return Err("preflight point read did not use local Gateway V2".into());
        }
        measure(
            workers,
            &profile,
            baseline.as_ref(),
            &environment,
            &evidence,
            output,
            &config.workload_id,
        )
        .await
    };
    let cleanup = async {
        evidence.enabled.store(false, Ordering::Relaxed);
        match tokio::time::timeout(
            Duration::from_secs(60),
            client.database_client(&database).delete(None),
        )
        .await
        {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(error))
                if error.status().status_code() == azure_core::http::StatusCode::NotFound =>
            {
                Ok(())
            }
            Ok(Err(error)) => Err(error.into()),
            Err(_) => Err("database cleanup timed out".into()),
        }
    };
    Box::pin(with_owned_database(
        output.join("owned-database.json"),
        &ownership,
        outcome,
        cleanup,
    ))
    .await
}

async fn with_owned_database(
    manifest: PathBuf,
    ownership: &impl Serialize,
    operation: impl Future<Output = Result<(), Error>>,
    cleanup: impl Future<Output = Result<(), Error>>,
) -> Result<(), Error> {
    // Persist intent before the creation future can issue a request.
    write_json(manifest, ownership).await?;
    let outcome = operation.await;
    match cleanup.await {
        Ok(()) => outcome,
        Err(error) => {
            Err(format!("benchmark outcome: {outcome:?}; cleanup failed: {error}").into())
        }
    }
}

async fn setup(
    client: &CosmosClient,
    database: &str,
    profile: &Profile,
) -> Result<Vec<Worker>, Error> {
    let db = client.database_client(database);
    let mut containers = Vec::new();
    for name in ["reference", "mutations", "lifecycle"] {
        let properties = ContainerProperties::new(name.to_string(), "/partition_key".into());
        let options = CreateContainerOptions::default().with_throughput(
            ThroughputProperties::manual(u64::from(profile.throughput_per_container)),
        );
        db.create_container(properties, Some(options)).await?;
        let container = db.container_client(name, None).await?;
        containers.push(container);
    }
    let reference = &containers[0];
    for index in 0..profile.seed_count {
        let item = PerfItem {
            id: format!("seed-{index}"),
            partition_key: format!("seed-{index}"),
            value: u64::from(index),
            payload: "x".repeat(profile.payload_bytes as usize),
        };
        reference
            .create_item(&item.partition_key, &item.id, &item, None)
            .await?;
    }
    let ranges = reference.read_feed_ranges(None).await?;
    if ranges.is_empty() {
        return Err("reference container has no feed ranges".into());
    }
    let ranges = Arc::new(RwLock::new(Arc::new(ranges)));
    let mut workers = Vec::new();
    for allocation in &profile.operations {
        for index in 0..allocation.workers {
            let container = match allocation.operation {
                Operation::UpsertItem | Operation::ReplaceItem | Operation::PatchItem => {
                    &containers[1]
                }
                Operation::CreateItem | Operation::DeleteItem | Operation::TransactionalBatch => {
                    &containers[2]
                }
                _ => reference,
            };
            let worker = Worker::new(
                allocation.operation,
                index,
                container.clone(),
                ranges.clone(),
                profile,
            );
            if !matches!(
                allocation.operation,
                Operation::CreateItem | Operation::DeleteItem | Operation::TransactionalBatch
            ) {
                container
                    .create_item(
                        &worker.item.partition_key,
                        &worker.item.id,
                        &worker.item,
                        None,
                    )
                    .await?;
            }
            workers.push(worker);
        }
    }
    Ok(workers)
}

#[derive(Serialize)]
struct Report<'a> {
    version: u32,
    status: &'a str,
    workload_id: &'a str,
    profile: &'a Profile,
    environment: &'a Environment,
    elapsed_seconds: f64,
    measured_seconds: u64,
    gateway_v2_requests: u64,
    nonlocal_or_unknown_region_requests: u64,
    warmup_completions: u64,
    deadline_inflight_errors: u64,
    cancelled_measured_requests: u64,
    infrastructure_errors: Vec<String>,
    operations: Vec<Metric>,
}

async fn measure(
    workers: Vec<Worker>,
    profile: &Profile,
    baseline: Option<&Baseline>,
    environment: &Environment,
    evidence: &RoutingEvidence,
    output: &Path,
    workload_id: &str,
) -> Result<(), Error> {
    let names: Vec<_> = Operation::ALL.iter().map(|op| op.name()).collect();
    let total = Arc::new(Stats::new(&names));
    let interval_stats = Arc::new(Stats::new(&names));
    let warmup = Arc::new(AtomicU64::new(0));
    let unfinished = Arc::new(AtomicU64::new(0));
    let cancelled = Arc::new(AtomicU64::new(0));
    let (start_tx, start_rx) = watch::channel(None);
    let ready = Arc::new(Barrier::new(workers.len() + 1));
    let mut tasks = JoinSet::new();
    for worker in workers {
        let context = WorkerContext {
            start: start_rx.clone(),
            ready: ready.clone(),
            duration: Duration::from_secs(profile.duration_seconds),
            warmup: Duration::from_secs(profile.warmup_seconds),
            timeout: Duration::from_secs(profile.request_timeout_seconds),
            total: total.clone(),
            interval: interval_stats.clone(),
            warmup_count: warmup.clone(),
            unfinished: unfinished.clone(),
            cancelled: cancelled.clone(),
        };
        tasks.spawn(run_worker(worker, context));
    }
    ready.wait().await;
    let start = Instant::now();
    start_tx.send(Some(start))?;
    let deadline = start + Duration::from_secs(profile.duration_seconds);
    let mut ticker = tokio::time::interval(Duration::from_secs(profile.report_seconds));
    ticker.tick().await;
    let mut infrastructure_errors = Vec::new();
    let mut interval_index = 0;
    let interrupt = tokio::signal::ctrl_c();
    tokio::pin!(interrupt);
    while !tasks.is_empty() {
        tokio::select! {
            result = tasks.join_next() => {
                match result {
                    Some(Ok(Ok(()))) => {}
                    Some(Ok(Err(error))) => infrastructure_errors.push(error),
                    Some(Err(error)) => infrastructure_errors.push(format!("worker task failed: {error}")),
                    None => {}
                }
                if !infrastructure_errors.is_empty() {
                    tasks.abort_all();
                    while tasks.join_next().await.is_some() {}
                }
            }
            result = &mut interrupt => {
                infrastructure_errors.push(format!("interrupted: {result:?}"));
                tasks.abort_all();
                while tasks.join_next().await.is_some() {}
            }
            _ = ticker.tick() => {
                interval_index += 1;
                let summaries = interval_stats.drain_summaries();
                crate::stats::print_report(&summaries);
                let metrics: Vec<_> = Operation::ALL.iter().map(|&op| evaluate(
                    op, summaries.iter().find(|s| s.name == op.name()), profile.report_seconds as f64, None
                )).collect();
                if let Err(error) = write_json(output.join(format!("interval-{interval_index}.json")), &metrics).await {
                    infrastructure_errors.push(format!("interval artifact write failed: {error}"));
                    tasks.abort_all();
                    while tasks.join_next().await.is_some() {}
                }
            }
        }
    }
    if Instant::now() < deadline {
        infrastructure_errors.push("workload stopped before its configured deadline".into());
    }
    if evidence.remote_requests.load(Ordering::Relaxed) > 0 {
        infrastructure_errors.push("requests used a nonlocal or unknown region".into());
    }
    let seconds = profile.duration_seconds - profile.warmup_seconds;
    let summaries = total.drain_summaries();
    let metrics: Vec<_> = Operation::ALL
        .iter()
        .map(|&op| {
            let threshold = baseline.and_then(|b| b.thresholds.iter().find(|t| t.operation == op));
            evaluate(
                op,
                summaries.iter().find(|s| s.name == op.name()),
                seconds as f64,
                threshold,
            )
        })
        .collect();
    let failed =
        !infrastructure_errors.is_empty() || metrics.iter().any(|m| !m.failures.is_empty());
    let status = if failed {
        "failed"
    } else if baseline.is_none() {
        "calibration"
    } else {
        "passed"
    };
    let report = Report {
        version: 1,
        status,
        workload_id,
        profile,
        environment,
        elapsed_seconds: start.elapsed().as_secs_f64(),
        measured_seconds: seconds,
        gateway_v2_requests: evidence.gateway_v2.load(Ordering::Relaxed),
        nonlocal_or_unknown_region_requests: evidence.remote_requests.load(Ordering::Relaxed),
        warmup_completions: warmup.load(Ordering::Relaxed),
        deadline_inflight_errors: unfinished.load(Ordering::Relaxed),
        cancelled_measured_requests: cancelled.load(Ordering::Relaxed),
        infrastructure_errors,
        operations: metrics,
    };
    write_json(output.join("summary.json"), &report).await?;
    println!("Performance outcome: {status}");
    for metric in &report.operations {
        println!(
            "{}: {:.2} ops/s, p99 {:.3} ms, error rate {:.6}: {:?}",
            metric.operation.name(),
            metric.successful_ops_per_second,
            metric.p99_ms,
            metric.error_rate,
            metric.failures
        );
    }
    if failed {
        return Err("performance validation failed; see summary.json".into());
    }
    Ok(())
}

struct WorkerContext {
    start: watch::Receiver<Option<Instant>>,
    ready: Arc<Barrier>,
    duration: Duration,
    warmup: Duration,
    timeout: Duration,
    total: Arc<Stats>,
    interval: Arc<Stats>,
    warmup_count: Arc<AtomicU64>,
    unfinished: Arc<AtomicU64>,
    cancelled: Arc<AtomicU64>,
}

async fn run_worker(mut worker: Worker, mut context: WorkerContext) -> Result<(), String> {
    context.ready.wait().await;
    context
        .start
        .wait_for(Option::is_some)
        .await
        .map_err(|e| e.to_string())?;
    let start = (*context.start.borrow()).ok_or("missing common start")?;
    let deadline = start + context.duration;
    let measured_start = start + context.warmup;
    let mut logged_error = false;
    loop {
        if Instant::now() >= deadline {
            break;
        }
        match tokio::time::timeout_at(
            (Instant::now() + context.timeout).min(deadline),
            worker.prepare(),
        )
        .await
        {
            Ok(Ok(())) => {}
            Err(_) if Instant::now() >= deadline => break,
            other => {
                return Err(format!(
                    "{} preparation failed: {other:?}",
                    worker.operation.name()
                ))
            }
        }
        let began = Instant::now();
        if began >= deadline {
            break;
        }
        let mut attempt = Attempt {
            total: &context.total,
            interval: &context.interval,
            cancelled: &context.cancelled,
            operation: worker.operation.name(),
            measured: began >= measured_start,
            completed: false,
        };
        let result =
            tokio::time::timeout_at((began + context.timeout).min(deadline), worker.execute())
                .await;
        let ended = Instant::now();
        let successful = matches!(result, Ok(Ok(())));
        match classify_sample(began, ended, measured_start, deadline, successful) {
            Sample::Warmup => {
                context.warmup_count.fetch_add(1, Ordering::Relaxed);
            }
            Sample::Success => {
                for stats in [&context.total, &context.interval] {
                    stats.record_latency(worker.operation.name(), ended - began, None);
                }
            }
            Sample::Error | Sample::Unfinished => {
                for stats in [&context.total, &context.interval] {
                    stats.record_error(worker.operation.name());
                }
                if ended >= deadline {
                    context.unfinished.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
        attempt.completed = true;
        if !successful && !logged_error {
            match &result {
                Ok(Err(error)) => eprintln!(
                    "{} first failure: status={}",
                    worker.operation.name(),
                    error.status()
                ),
                Err(_) => eprintln!(
                    "{} first failure: request deadline exceeded",
                    worker.operation.name()
                ),
                Ok(Ok(())) => {}
            }
            logged_error = true;
        }
        // An ambiguous lifecycle mutation must not grow abandoned data indefinitely.
        if matches!(
            worker.operation,
            Operation::CreateItem | Operation::DeleteItem | Operation::TransactionalBatch
        ) && !successful
        {
            if ended < deadline {
                return Err(format!(
                    "{} failed; stopping rather than reusing ambiguous lifecycle state",
                    worker.operation.name()
                ));
            }
        } else if successful {
            match tokio::time::timeout(context.timeout, worker.cleanup()).await {
                Ok(Ok(())) => {}
                other => {
                    return Err(format!(
                        "{} cleanup failed: {other:?}",
                        worker.operation.name()
                    ))
                }
            }
        }
        tokio::task::yield_now().await;
    }
    Ok(())
}

struct Attempt<'a> {
    total: &'a Stats,
    interval: &'a Stats,
    cancelled: &'a AtomicU64,
    operation: &'static str,
    measured: bool,
    completed: bool,
}

impl Drop for Attempt<'_> {
    fn drop(&mut self) {
        if self.measured && !self.completed {
            self.total.record_error(self.operation);
            self.interval.record_error(self.operation);
            self.cancelled.fetch_add(1, Ordering::Relaxed);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Sample {
    Warmup,
    Success,
    Error,
    Unfinished,
}

fn classify_sample(
    began: Instant,
    ended: Instant,
    measured_start: Instant,
    deadline: Instant,
    success: bool,
) -> Sample {
    if began < measured_start {
        Sample::Warmup
    } else if ended >= deadline {
        Sample::Unfinished
    } else if success {
        Sample::Success
    } else {
        Sample::Error
    }
}

fn valid_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|c| c.is_ascii_hexdigit())
}

async fn read_json<T: DeserializeOwned + Send + 'static>(path: &Path) -> Result<T, Error> {
    let path = path.to_owned();
    Ok(tokio::task::spawn_blocking(
        move || -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
            Ok(serde_json::from_slice(&std::fs::read(path)?)?)
        },
    )
    .await?
    .map_err(|e| e.to_string())?)
}

async fn write_json(path: PathBuf, value: &impl Serialize) -> Result<(), Error> {
    let bytes = serde_json::to_vec_pretty(value)?;
    tokio::task::spawn_blocking(move || -> std::io::Result<()> {
        let temporary = path.with_extension("tmp");
        std::fs::write(&temporary, bytes)?;
        std::fs::rename(temporary, path)
    })
    .await??;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::contract::{Baseline, Environment, Operation, Profile, Threshold};
    use super::{
        classify_sample, measure, setup, with_owned_database, Attempt, RoutingEvidence, Sample,
    };
    use crate::stats::Stats;
    use azure_core::{credentials::Secret, http::Url};
    use azure_data_cosmos::{
        AccountReference, CosmosClientBuilder, CosmosRuntimeBuilder, RoutingStrategy,
    };
    use azure_data_cosmos_driver::in_memory_emulator::{
        InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    };
    use std::{
        sync::{
            atomic::{AtomicBool, AtomicU64, Ordering},
            Arc,
        },
        time::Duration,
    };
    use tokio::{sync::oneshot, task::JoinSet, time::Instant};

    #[tokio::test]
    async fn aborting_peers_counts_only_unfinished_measured_attempts_once() {
        let total = Arc::new(Stats::new(&["ReadItem"]));
        let interval = Arc::new(Stats::new(&["ReadItem"]));
        let cancelled = Arc::new(AtomicU64::new(0));
        let mut tasks = JoinSet::new();
        for measured in [false, true] {
            let (ready, started) = oneshot::channel();
            let total = total.clone();
            let interval = interval.clone();
            let cancelled = cancelled.clone();
            tasks.spawn(async move {
                let _attempt = Attempt {
                    total: &total,
                    interval: &interval,
                    cancelled: &cancelled,
                    operation: "ReadItem",
                    measured,
                    completed: false,
                };
                ready.send(()).unwrap();
                std::future::pending::<()>().await;
            });
            started.await.unwrap();
        }
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
        assert_eq!(cancelled.load(Ordering::Relaxed), 1);
        assert_eq!(total.drain_summaries()[0].errors, 1);
        assert_eq!(interval.drain_summaries()[0].errors, 1);
    }

    #[tokio::test]
    async fn creation_error_still_has_durable_ownership_and_runs_cleanup() {
        let manifest =
            std::env::temp_dir().join(format!("cosmos-perf-{}.json", uuid::Uuid::new_v4()));
        let created = AtomicBool::new(false);
        let cleaned = AtomicBool::new(false);
        let ownership = serde_json::json!({"database": "owned-test-db", "workload_id": "test"});
        let result = with_owned_database(
            manifest.clone(),
            &ownership,
            async {
                let persisted: serde_json::Value = super::read_json(&manifest).await?;
                assert_eq!(persisted, ownership);
                created.store(true, Ordering::Relaxed);
                Err("creation committed but response was lost".into())
            },
            async {
                assert!(created.load(Ordering::Relaxed));
                cleaned.store(true, Ordering::Relaxed);
                Ok(())
            },
        )
        .await;
        assert!(result.is_err());
        assert!(cleaned.load(Ordering::Relaxed));
        tokio::task::spawn_blocking(move || std::fs::remove_file(manifest))
            .await
            .unwrap()
            .unwrap();
    }

    #[tokio::test]
    async fn cancellation_during_creation_preserves_cleanup_manifest() {
        let manifest =
            std::env::temp_dir().join(format!("cosmos-perf-{}.json", uuid::Uuid::new_v4()));
        let path = manifest.clone();
        let (ready, started) = oneshot::channel();
        // LocalSet permits the helper's boxed non-Send error without changing production errors.
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async move {
                let task = tokio::task::spawn_local(async move {
                    with_owned_database(
                        path,
                        &serde_json::json!({"database":"owned-test-db"}),
                        async {
                            ready.send(()).unwrap();
                            std::future::pending().await
                        },
                        async { Ok(()) },
                    )
                    .await
                });
                started.await.unwrap();
                task.abort();
                assert!(task.await.unwrap_err().is_cancelled());
            })
            .await;
        let persisted: serde_json::Value = super::read_json(&manifest).await.unwrap();
        assert_eq!(persisted["database"], "owned-test-db");
        tokio::task::spawn_blocking(move || std::fs::remove_file(manifest))
            .await
            .unwrap()
            .unwrap();
    }

    #[test]
    fn warmup_and_deadline_crossings_are_classified_by_request_time() {
        let start = Instant::now();
        let at = |seconds| start + Duration::from_secs(seconds);
        assert_eq!(
            classify_sample(at(299), at(301), at(300), at(2700), true),
            Sample::Warmup
        );
        assert_eq!(
            classify_sample(at(300), at(301), at(300), at(2700), true),
            Sample::Success
        );
        assert_eq!(
            classify_sample(at(300), at(301), at(300), at(2700), false),
            Sample::Error
        );
        assert_eq!(
            classify_sample(at(2699), at(2700), at(300), at(2700), true),
            Sample::Unfinished
        );
        assert_eq!(
            classify_sample(at(299), at(2700), at(300), at(2700), false),
            Sample::Warmup
        );
    }

    #[test]
    fn full_histogram_survives_interval_drains() {
        let total = Stats::new(&["ReadItem"]);
        let interval = Stats::new(&["ReadItem"]);
        for index in 0..1000 {
            let latency = Duration::from_millis(if index < 990 { 1 } else { 1000 });
            total.record_latency("ReadItem", latency, None);
            interval.record_latency("ReadItem", latency, None);
            if index % 37 == 0 {
                interval.drain_summaries();
            }
        }
        let summary = total.drain_summaries();
        assert_eq!(summary[0].count, 1000);
        assert!(summary[0].p99 < Duration::from_millis(2));
        assert_eq!(summary[0].max, Duration::from_secs(1));
    }

    #[tokio::test]
    async fn all_workloads_execute_concurrently_without_deleting_reference_items() {
        let url = "https://eastus.emulator.local";
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(
            VirtualAccountConfig::new(vec![VirtualRegion::new(
                "East US",
                Url::parse(url).unwrap(),
            )])
            .unwrap(),
        ));
        let runtime = CosmosRuntimeBuilder::from(emulator.runtime_builder())
            .build()
            .await
            .unwrap();
        let client = CosmosClientBuilder::new()
            .with_runtime(runtime)
            .build(
                AccountReference::with_authentication_key(
                    url.parse().unwrap(),
                    Secret::new("dGVzdGtleQ=="),
                ),
                RoutingStrategy::ProximityTo("East US".into()),
            )
            .await
            .unwrap();
        client
            .create_database("perf-integration", None)
            .await
            .unwrap();
        let mut profile: Profile =
            serde_json::from_str(include_str!("../../../eng/pipelines/perf-profile.json")).unwrap();
        for allocation in &mut profile.operations {
            allocation.workers = 1;
        }
        let workers = setup(&client, "perf-integration", &profile).await.unwrap();
        assert_eq!(workers.len(), Operation::ALL.len());
        let mut tasks = JoinSet::new();
        for mut worker in workers {
            tasks.spawn(async move {
                for _ in 0..3 {
                    worker.prepare().await.unwrap();
                    worker
                        .execute()
                        .await
                        .unwrap_or_else(|e| panic!("{}: {e}", worker.operation.name()));
                    worker.cleanup().await.unwrap();
                }
                worker.operation
            });
        }
        let mut completed = Vec::new();
        while let Some(result) = tasks.join_next().await {
            completed.push(result.unwrap());
        }
        completed.sort();
        let mut expected = Operation::ALL.to_vec();
        expected.sort();
        assert_eq!(completed, expected);
        let reference = client
            .database_client("perf-integration")
            .container_client("reference", None)
            .await
            .unwrap();
        assert!(reference
            .read_item("ReadItem-0", "ReadItem-0", None)
            .await
            .is_ok());
        assert!(reference.read_item("seed-0", "seed-0", None).await.is_ok());

        client.create_database("perf-gate", None).await.unwrap();
        profile.duration_seconds = 2;
        profile.warmup_seconds = 1;
        profile.report_seconds = 1;
        profile.max_pages = 1;
        let workers = setup(&client, "perf-gate", &profile).await.unwrap();
        let baseline = Baseline {
            version: 1,
            approved: true,
            profile: Some(profile.clone()),
            thresholds: Operation::ALL
                .into_iter()
                .map(|operation| Threshold {
                    operation,
                    min_successful_ops_per_second: 1e12,
                    max_p99_ms: 10000.0,
                    max_error_rate: 0.99,
                })
                .collect(),
        };
        let environment = Environment {
            source_commit: "1".repeat(40),
            baseline_commit: "2".repeat(40),
            account_name: "emulator".into(),
            endpoint: url.into(),
            region: "East US".into(),
            vm_size: profile.vm_size.clone(),
            consistency: profile.consistency.clone(),
        };
        let evidence = RoutingEvidence {
            region: "eastus".into(),
            enabled: AtomicBool::new(false),
            gateway_v2: AtomicU64::new(0),
            remote_requests: AtomicU64::new(0),
        };
        let output = std::env::temp_dir().join(format!("cosmos-perf-{}", uuid::Uuid::new_v4()));
        let directory = output.clone();
        tokio::task::spawn_blocking(move || std::fs::create_dir(&directory))
            .await
            .unwrap()
            .unwrap();
        let result = measure(
            workers,
            &profile,
            Some(&baseline),
            &environment,
            &evidence,
            &output,
            "test",
        )
        .await;
        assert!(
            result.is_err(),
            "an impossible throughput threshold must fail"
        );
        let report: serde_json::Value = super::read_json(&output.join("summary.json"))
            .await
            .unwrap();
        assert_eq!(report["status"], "failed");
        assert_eq!(report["measured_seconds"], 1);
        assert!(report["warmup_completions"].as_u64().unwrap() > 0);
        assert_eq!(report["infrastructure_errors"], serde_json::json!([]));
        let operations = report["operations"].as_array().unwrap();
        assert_eq!(operations.len(), Operation::ALL.len());
        for operation in operations {
            assert!(operation["successes"].as_u64().unwrap() > 0, "{operation}");
            assert!(operation["failures"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v == "successful throughput below minimum"));
        }
        tokio::task::spawn_blocking(move || std::fs::remove_dir_all(output))
            .await
            .unwrap()
            .unwrap();
    }
}
