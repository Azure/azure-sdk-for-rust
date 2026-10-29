// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    bootstrap, namespace, ChangeFeedFactoryMode, ChangeFeedProcessorBootstrapContext,
    ChangeFeedProcessorBootstrapIdentity, ChangeFeedProcessorBootstrapLockRecord,
    ChangeFeedProcessorBootstrapRangePlan, ChangeFeedProcessorBootstrapRecord,
    ChangeFeedProcessorBootstrapStore, LOCK_TTL_SECONDS, SCHEMA_VERSION,
};
use crate::{
    models::{
        AccountReference, ContainerReference, CosmosOperation, DatabaseReference, ItemReference,
        PartitionKey, Precondition,
    },
    options::{DriverOptions, OperationOptions},
    CosmosDriver, CosmosDriverRuntime,
};
use azure_core::http::StatusCode;
use futures::FutureExt;
use std::{
    error::Error,
    future::Future,
    io,
    panic::{resume_unwind, AssertUnwindSafe},
    process::{Child, Command},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use url::Url;
use uuid::Uuid;

const ENDPOINT_ENV: &str = "COSMOS_BOOTSTRAP_TEST_ENDPOINT";
const DATABASE_ENV: &str = "COSMOS_BOOTSTRAP_TEST_DATABASE";
const PROCESSOR_ENV: &str = "COSMOS_BOOTSTRAP_TEST_PROCESSOR";
const MODE_ENV: &str = "COSMOS_BOOTSTRAP_TEST_MODE";
const CONFLICT_ENV: &str = "COSMOS_BOOTSTRAP_TEST_EXPECT_CONFLICT";
const WORKER_TEST: &str = "change_feed_processor::bootstrap::process_tests::hosted_worker";
const LIVE_WORKER_TEST: &str = "change_feed_processor::bootstrap::process_tests::live_worker";
const CRASHED_WORKER_TEST: &str =
    "change_feed_processor::bootstrap::process_tests::live_crashed_initializer";
const LIVE_ENDPOINT_ENV: &str = "COSMOS_BOOTSTRAP_LIVE_ENDPOINT";
const LIVE_DATABASE_ENV: &str = "COSMOS_BOOTSTRAP_LIVE_DATABASE";
const LIVE_KEY_ENV: &str = "COSMOS_BOOTSTRAP_LIVE_KEY";
const LIVE_ALLOWED_ENV: &str = "COSMOS_BOOTSTRAP_ALLOW_LIVE";
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

async fn driver() -> TestResult<Arc<CosmosDriver>> {
    let endpoint: Url = std::env::var(ENDPOINT_ENV)?.parse()?;
    if endpoint.scheme() != "http"
        || !matches!(
            endpoint.host_str(),
            Some("127.0.0.1" | "localhost" | "[::1]")
        )
    {
        return Err(
            io::Error::other("Hosted bootstrap tests require a local HTTP emulator.").into(),
        );
    }
    let runtime = CosmosDriverRuntime::builder().build().await?;
    Ok(runtime
        .create_driver(
            DriverOptions::builder(AccountReference::with_account_key(endpoint, "dGVzdGtleQ=="))
                .build(),
        )
        .await?)
}

fn live_database() -> TestResult<String> {
    let database = std::env::var(LIVE_DATABASE_ENV)?;
    if !database.starts_with("cfp-bootstrap-") {
        return Err(io::Error::other(
            "Live bootstrap tests require an isolated cfp-bootstrap- database.",
        )
        .into());
    }
    Ok(database)
}

async fn live_driver() -> TestResult<Arc<CosmosDriver>> {
    if std::env::var(LIVE_ALLOWED_ENV)? != "true" {
        return Err(io::Error::other("Live bootstrap tests require explicit opt-in.").into());
    }
    let endpoint: Url = std::env::var(LIVE_ENDPOINT_ENV)?.parse()?;
    if endpoint.scheme() != "https"
        || !endpoint
            .host_str()
            .is_some_and(|host| host.ends_with(".documents.azure.com"))
    {
        return Err(io::Error::other(
            "Live bootstrap tests require an explicit Azure Cosmos HTTPS endpoint.",
        )
        .into());
    }
    let key = std::env::var(LIVE_KEY_ENV)?;
    let runtime = CosmosDriverRuntime::builder().build().await?;
    Ok(runtime
        .create_driver(
            DriverOptions::builder(AccountReference::with_account_key(endpoint, key)).build(),
        )
        .await?)
}

struct Workers(Vec<Child>);

impl Workers {
    fn spawn(
        worker_test: &'static str,
        database: &str,
        processor: &str,
        mode: &str,
        expect_conflict: bool,
        count: u32,
    ) -> TestResult<Self> {
        let mut workers = Self(Vec::new());
        for _ in 0..count {
            workers.0.push(
                Command::new(std::env::current_exe()?)
                    .args(["--ignored", "--exact", worker_test, "--nocapture"])
                    .env(DATABASE_ENV, database)
                    .env(LIVE_DATABASE_ENV, database)
                    .env(PROCESSOR_ENV, processor)
                    .env(MODE_ENV, mode)
                    .env(CONFLICT_ENV, expect_conflict.to_string())
                    .spawn()?,
            );
        }
        Ok(workers)
    }

    fn wait(&mut self) -> TestResult {
        let deadline = Instant::now() + Duration::from_secs(180);
        for child in &mut self.0 {
            loop {
                if let Some(status) = child.try_wait()? {
                    if !status.success() {
                        return Err(io::Error::other(format!(
                            "Bootstrap worker exited with {status}."
                        ))
                        .into());
                    }
                    break;
                }
                if Instant::now() >= deadline {
                    return Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "Bootstrap worker did not exit.",
                    )
                    .into());
                }
                thread::sleep(Duration::from_millis(10));
            }
        }
        Ok(())
    }
}

impl Drop for Workers {
    fn drop(&mut self) {
        for child in &mut self.0 {
            match child.try_wait() {
                Ok(Some(_)) => {}
                Ok(None) => {
                    if let Err(error) = child.kill() {
                        eprintln!(
                            "Failed to stop bootstrap test worker {}: {error}",
                            child.id()
                        );
                    }
                    if let Err(error) = child.wait() {
                        eprintln!(
                            "Failed to reap bootstrap test worker {}: {error}",
                            child.id()
                        );
                    }
                }
                Err(error) => eprintln!(
                    "Failed to inspect bootstrap test worker {}: {error}",
                    child.id()
                ),
            }
        }
    }
}

#[tokio::test]
#[ignore = "Subprocess helper for hosted_concurrent_processes; requires local emulator environment."]
async fn hosted_worker() -> TestResult {
    run_worker(driver().await?, Duration::from_secs(20)).await
}

#[tokio::test]
#[ignore = "Subprocess helper for live bootstrap tests; requires explicit live account opt-in."]
async fn live_worker() -> TestResult {
    assert_eq!(std::env::var(DATABASE_ENV)?, live_database()?);
    run_worker(live_driver().await?, Duration::from_secs(120)).await
}

async fn run_worker(driver: Arc<CosmosDriver>, timeout: Duration) -> TestResult {
    let database = std::env::var(DATABASE_ENV)?;
    let mode = match std::env::var(MODE_ENV)?.as_str() {
        "latest" => ChangeFeedFactoryMode::Incremental,
        "all" => ChangeFeedFactoryMode::FullFidelity,
        _ => return Err(io::Error::other("Invalid bootstrap test mode.").into()),
    };
    let expect_conflict: bool = std::env::var(CONFLICT_ENV)?.parse()?;
    let source = driver
        .resolve_container(&database, "source", OperationOptions::default())
        .await?;
    let leases = driver
        .resolve_container(&database, "leases", OperationOptions::default())
        .await?;
    let processor_name = std::env::var(PROCESSOR_ENV)?;
    let instance_name = format!("process-{}", std::process::id());
    let result = tokio::time::timeout(
        timeout,
        bootstrap(
            &driver,
            &source,
            &driver,
            &leases,
            mode,
            &processor_name,
            &instance_name,
        ),
    )
    .await?;
    if expect_conflict {
        assert_eq!(
            result.unwrap_err().status().status_code(),
            StatusCode::Conflict
        );
    } else {
        result?;
    }
    Ok(())
}

#[tokio::test]
#[ignore = "Requires COSMOS_BOOTSTRAP_TEST_ENDPOINT pointing at the local hosted emulator."]
async fn hosted_concurrent_processes() -> TestResult {
    let driver = driver().await?;
    let database = test_database_name();
    with_database(&driver, &database, || {
        exercise_workers(
            &driver,
            &database,
            "process-workers",
            ChangeFeedFactoryMode::Incremental,
            WORKER_TEST,
        )
    })
    .await
}

fn test_database_name() -> String {
    format!("cfp-bootstrap-{}", Uuid::new_v4().simple())
}

async fn with_database<F, Fut>(driver: &CosmosDriver, database: &str, scenario: F) -> TestResult
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = TestResult>,
{
    let database_ref = DatabaseReference::from_name(driver.account().clone(), database.to_owned());
    println!("Creating isolated bootstrap test database {database}.");
    driver
        .execute_singleton_operation(
            CosmosOperation::create_database(driver.account().clone())
                .with_body(serde_json::to_vec(&serde_json::json!({"id":database}))?),
            OperationOptions::default(),
        )
        .await?;
    println!("Created isolated bootstrap test database {database}.");
    let outcome = AssertUnwindSafe(async {
        for name in ["source", "leases"] {
            driver.execute_singleton_operation(
                CosmosOperation::create_container(database_ref.clone()).with_body(serde_json::to_vec(
                    &serde_json::json!({
                        "id": name, "partitionKey": {"paths": ["/id"], "kind": "Hash", "version": 2},
                        "defaultTtl": -1
                    }),
                )?),
                OperationOptions::default(),
            ).await?;
        }
        scenario().await
    })
    .catch_unwind()
    .await;
    let cleanup = driver
        .execute_singleton_operation(
            CosmosOperation::delete_database(database_ref)
                .with_absolute_deadline(Some(Instant::now() + Duration::from_secs(60))),
            OperationOptions::default(),
        )
        .await;
    match &cleanup {
        Ok(_) => println!("Deleted isolated bootstrap test database {database}."),
        Err(error) => eprintln!("Failed to delete bootstrap test database {database}: {error}"),
    }
    match outcome {
        Ok(Ok(())) => cleanup.map(|_| ()).map_err(Into::into),
        Ok(Err(error)) => Err(error),
        Err(panic) => resume_unwind(panic),
    }
}

fn identity(
    source: &ContainerReference,
    processor: &str,
    mode: ChangeFeedFactoryMode,
) -> ChangeFeedProcessorBootstrapIdentity {
    ChangeFeedProcessorBootstrapIdentity {
        version: SCHEMA_VERSION,
        account: source.account().endpoint().origin().ascii_serialization(),
        container_rid: source.rid().to_owned(),
        processor_name: processor.into(),
        mode,
    }
}

async fn exercise_workers(
    driver: &CosmosDriver,
    database: &str,
    processor: &str,
    mode: ChangeFeedFactoryMode,
    worker_test: &'static str,
) -> TestResult {
    let (mode_name, other_mode) = match mode {
        ChangeFeedFactoryMode::Incremental => ("latest", "all"),
        ChangeFeedFactoryMode::FullFidelity => ("all", "latest"),
    };
    Workers::spawn(worker_test, database, processor, mode_name, false, 4)?.wait()?;
    let source = driver
        .resolve_container(database, "source", OperationOptions::default())
        .await?;
    let leases = driver
        .resolve_container(database, "leases", OperationOptions::default())
        .await?;
    let identity = identity(&source, processor, mode);
    let prefix = namespace(&identity)?;
    let store = ChangeFeedProcessorBootstrapStore {
        driver,
        container: &leases,
        deadline: Instant::now() + Duration::from_secs(60),
    };
    let plan = store
        .read::<ChangeFeedProcessorBootstrapRangePlan>(&format!("{prefix}.plan"))
        .await?
        .unwrap()
        .value;
    let topology = driver
        .resolve_all_partition_key_ranges(&source, true)
        .await?
        .expect("the source must have a complete routing map");
    let expected_ranges = topology
        .iter()
        .map(crate::models::FeedRange::try_from)
        .collect::<crate::Result<Vec<_>>>()?;
    assert_eq!(plan.ranges, expected_ranges);
    for range in &plan.ranges {
        let id = format!(
            "{prefix}.lease.{}-{}",
            range.min_inclusive(),
            range.max_exclusive()
        );
        let lease = store.read::<serde_json::Value>(&id).await?.unwrap();
        assert!(lease.value.get("continuation").is_none());
        assert!(lease.value.get("owner").is_none());
    }
    let range = &plan.ranges[0];
    let id = format!(
        "{prefix}.lease.{}-{}",
        range.min_inclusive(),
        range.max_exclusive()
    );
    let original = store.read::<serde_json::Value>(&id).await?.unwrap();
    assert!(original.value.get("continuation").is_none());
    let mut saved_lease = original.value;
    saved_lease["owner"] = "existing-worker".into();
    saved_lease["continuation"] = "saved-progress".into();
    store
        .execute(
            CosmosOperation::replace_item(ItemReference::from_name(
                &leases,
                PartitionKey::from(id.clone()),
                id.clone(),
            ))
            .with_precondition(Precondition::if_match(original.etag))
            .with_body(serde_json::to_vec(&saved_lease)?),
        )
        .await?;
    let before = store.read::<serde_json::Value>(&id).await?.unwrap();
    Workers::spawn(worker_test, database, processor, mode_name, false, 2)?.wait()?;
    Workers::spawn(worker_test, database, processor, other_mode, true, 2)?.wait()?;
    let after = store.read::<serde_json::Value>(&id).await?.unwrap();
    assert_eq!(after.etag, before.etag);
    assert_eq!(after.value, before.value);
    Ok(())
}

#[tokio::test]
#[ignore = "Requires explicit live opt-in, endpoint, and an account key; creates and deletes test resources."]
async fn live_concurrent_processes_and_ttl_recovery() -> TestResult {
    let driver = live_driver().await?;
    let database = test_database_name();
    with_database(&driver, &database, || {
        exercise_live_workers(&driver, &database)
    })
    .await
}

async fn exercise_live_workers(driver: &CosmosDriver, database: &str) -> TestResult {
    for (processor, mode) in [
        ("live-latest", ChangeFeedFactoryMode::Incremental),
        ("live-all-versions", ChangeFeedFactoryMode::FullFidelity),
    ] {
        exercise_workers(driver, database, processor, mode, LIVE_WORKER_TEST).await?;
    }
    Workers::spawn(
        CRASHED_WORKER_TEST,
        database,
        "ttl-recovery",
        "latest",
        false,
        1,
    )?
    .wait()?;
    let source = driver
        .resolve_container(database, "source", OperationOptions::default())
        .await?;
    let leases = driver
        .resolve_container(database, "leases", OperationOptions::default())
        .await?;
    let identity = identity(&source, "ttl-recovery", ChangeFeedFactoryMode::Incremental);
    let prefix = namespace(&identity)?;
    let store = ChangeFeedProcessorBootstrapStore {
        driver,
        container: &leases,
        deadline: Instant::now() + Duration::from_secs(180),
    };
    let abandoned = store
        .read::<ChangeFeedProcessorBootstrapLockRecord>(&format!("{prefix}.lock"))
        .await?
        .unwrap();
    assert_eq!(abandoned.value.record.ttl, LOCK_TTL_SECONDS);
    let started = Instant::now();
    Workers::spawn(
        LIVE_WORKER_TEST,
        database,
        "ttl-recovery",
        "latest",
        false,
        2,
    )?
    .wait()?;
    println!(
        "Live bootstrap recovered after service TTL expiry in {:.1}s.",
        started.elapsed().as_secs_f64()
    );
    assert!(store
        .read::<ChangeFeedProcessorBootstrapLockRecord>(&format!("{prefix}.lock"))
        .await?
        .is_none());
    for suffix in ["definition", "plan", "info"] {
        let record = store
            .read::<ChangeFeedProcessorBootstrapRecord>(&format!("{prefix}.{suffix}"))
            .await?
            .unwrap();
        assert_eq!(record.value.workload, identity);
        assert_eq!(record.value.ttl, -1);
    }
    let plan = store
        .read::<ChangeFeedProcessorBootstrapRangePlan>(&format!("{prefix}.plan"))
        .await?
        .unwrap()
        .value;
    for range in plan.ranges {
        let id = format!(
            "{prefix}.lease.{}-{}",
            range.min_inclusive(),
            range.max_exclusive()
        );
        let lease = store.read::<serde_json::Value>(&id).await?.unwrap();
        assert!(lease.value.get("continuation").is_none());
        assert!(lease.value.get("owner").is_none());
    }
    Ok(())
}

#[tokio::test]
#[ignore = "Crash subprocess helper for live TTL recovery; requires explicit live account opt-in."]
async fn live_crashed_initializer() -> TestResult {
    let database = live_database()?;
    assert_eq!(std::env::var(DATABASE_ENV)?, database);
    let driver = live_driver().await?;
    let source = driver
        .resolve_container(&database, "source", OperationOptions::default())
        .await?;
    let leases = driver
        .resolve_container(&database, "leases", OperationOptions::default())
        .await?;
    let context = ChangeFeedProcessorBootstrapContext::new(
        &driver,
        &source,
        &driver,
        &leases,
        ChangeFeedFactoryMode::Incremental,
        &std::env::var(PROCESSOR_ENV)?,
        &format!("crashing-process-{}", std::process::id()),
    )?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context
        .try_claim()
        .await?
        .expect("fresh workload must grant its first claim");
    context
        .plan(&claim)
        .await?
        .expect("fresh workload must persist its initial plan");
    // Exit without running cleanup so only the service's TTL can release the claim.
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::{test_database_name, with_database, TestResult};
    use crate::{
        in_memory_emulator::{InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion},
        models::{AccountReference, CosmosOperation, DatabaseReference},
        options::{DriverOptions, OperationOptions},
        CosmosDriver, CosmosError,
    };
    use azure_core::http::StatusCode;
    use futures::FutureExt;
    use std::{io, panic::AssertUnwindSafe, sync::Arc};

    async fn memory_driver() -> TestResult<Arc<CosmosDriver>> {
        let endpoint = "https://eastus.emulator.local".parse()?;
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(
            vec![VirtualRegion::new("East US", endpoint)],
        )?));
        let runtime = emulator.runtime_builder().build().await?;
        Ok(runtime
            .create_driver(
                DriverOptions::builder(AccountReference::with_account_key(
                    "https://eastus.emulator.local".parse()?,
                    "dGVzdGtleQ==",
                ))
                .build(),
            )
            .await?)
    }

    async fn assert_database_missing(driver: &CosmosDriver, database: &str) {
        let error = driver
            .execute_singleton_operation(
                CosmosOperation::read_database(DatabaseReference::from_name(
                    driver.account().clone(),
                    database.to_owned(),
                )),
                OperationOptions::default(),
            )
            .await
            .unwrap_err();
        assert_eq!(error.status().status_code(), StatusCode::NotFound);
    }

    #[tokio::test]
    async fn database_fixture_cleans_up_after_success() -> TestResult {
        let driver = memory_driver().await?;
        let database = test_database_name();
        with_database(&driver, &database, || async {
            for name in ["source", "leases"] {
                let container = driver
                    .resolve_container(&database, name, OperationOptions::default())
                    .await?;
                let response = driver
                    .execute_singleton_operation(
                        CosmosOperation::read_container(container),
                        OperationOptions::default(),
                    )
                    .await?;
                let properties: serde_json::Value = response.into_body().into_single()?;
                assert_eq!(
                    properties["partitionKey"]["paths"],
                    serde_json::json!(["/id"])
                );
                assert_eq!(properties["defaultTtl"], -1);
            }
            Ok(())
        })
        .await?;
        assert_database_missing(&driver, &database).await;
        Ok(())
    }

    #[tokio::test]
    async fn database_fixture_cleans_up_after_error() -> TestResult {
        let driver = memory_driver().await?;
        let database = test_database_name();
        let error = with_database(&driver, &database, || async {
            Err(io::Error::other("scenario failure").into())
        })
        .await
        .unwrap_err();
        assert_eq!(error.to_string(), "scenario failure");
        assert_database_missing(&driver, &database).await;
        Ok(())
    }

    #[tokio::test]
    async fn database_fixture_cleans_up_before_resuming_panic() -> TestResult {
        let driver = memory_driver().await?;
        let database = test_database_name();
        let panic = AssertUnwindSafe(with_database(&driver, &database, || async {
            panic!("scenario assertion failure");
        }))
        .catch_unwind()
        .await
        .unwrap_err();
        assert_eq!(
            panic.downcast_ref::<&str>(),
            Some(&"scenario assertion failure")
        );
        assert_database_missing(&driver, &database).await;
        Ok(())
    }

    #[tokio::test]
    async fn database_fixture_does_not_adopt_or_delete_existing_database() -> TestResult {
        let driver = memory_driver().await?;
        let database = test_database_name();
        with_database(&driver, &database, || async {
            let error = with_database(&driver, &database, || async {
                panic!("an existing database must not run the scenario");
            })
            .await
            .unwrap_err();
            assert_eq!(
                error
                    .downcast_ref::<CosmosError>()
                    .unwrap()
                    .status()
                    .status_code(),
                StatusCode::Conflict
            );
            for name in ["source", "leases"] {
                driver
                    .resolve_container(&database, name, OperationOptions::default())
                    .await?;
            }
            Ok(())
        })
        .await?;
        assert_database_missing(&driver, &database).await;
        Ok(())
    }
}
