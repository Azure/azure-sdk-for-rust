// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Dual-backend test infrastructure that runs the same Cosmos DB operations
//! against both a real account and the in-memory emulator, then compares
//! responses.
//!
//! # Configuration
//!
//! The real account backend is configured via the `AZURE_COSMOS_CONNECTION_STRING`
//! environment variable. When the variable is unset (and `AZURE_COSMOS_TEST_MODE`
//! is not `required`), only the emulator leg runs and comparison is skipped.
//!
//! Set `AZURE_COSMOS_TEST_MODE=required` to fail when the connection string is
//! missing, or `skipped` to disable the real-account leg entirely.

use azure_core::http::{StatusCode, Url};
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntime,
    in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, ConnectionString, ContainerReference, CosmosOperation, CosmosResponse,
        DatabaseReference, ItemReference, PartitionKey,
    },
    options::{
        AvailabilityStrategy, ConnectionPoolOptions, DriverOptions, OperationOptions,
        OperationOptionsBuilder, Region, ServerCertificateValidation,
    },
    CosmosDriver,
};
use std::{
    error::Error,
    sync::Arc,
    time::{Duration, Instant},
};
use uuid::Uuid;

use super::validation::{
    compare_responses, BodyValidationSpec, HeaderValidationSpec, ResponseSnapshot,
};

/// Environment variable for the real-account connection string.
const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";

/// Environment variable controlling test mode.
const TEST_MODE_ENV_VAR: &str = "AZURE_COSMOS_TEST_MODE";

/// Environment variable exposing the real account's configured default
/// consistency level (emitted by the test-resources deployment). Substatus
/// `1002` (ReadSessionNotAvailable) is only produced under Session
/// consistency, so consistency-sensitive assertions consult this.
const DEFAULT_CONSISTENCY_ENV_VAR: &str = "AZURE_COSMOS_DEFAULT_CONSISTENCY";

/// Gateway URL used by the in-memory emulator.
const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";

/// Read regions advertised by the multi-region Gateway 2.0 CI account
/// (`thin-client-mr-session-ci`: Central US write + East US 2 read).
///
/// Region-sensitive tests use [`DualBackend::wait_for_sentinel_readable_from_all_regions`]
/// to confirm a freshly provisioned container is replicated and servable from
/// every one of these regions before asserting per-region behavior — a region
/// that has not yet caught up returns a plain resource 404 (no sub-status)
/// rather than the region-specific status under test. Regions not present on
/// the configured account are silently skipped by the driver.
const MULTI_REGION_READ_REGIONS: &[Region] = &[Region::CENTRAL_US, Region::EAST_US_2];

fn collection_create_in_progress(err: &azure_data_cosmos_driver::error::CosmosError) -> bool {
    let status = err.status();
    status.status_code() == StatusCode::NotFound
        && status.sub_status().map(|s| s.value()) == Some(1013)
}

/// Holds drivers for both backends (emulator is always present, real is optional).
pub struct DualBackend {
    /// Driver wired to the in-memory emulator.
    pub emulator_driver: Arc<CosmosDriver>,
    /// Emulator store for pre-provisioning resources.
    pub emulator_store: Arc<azure_data_cosmos_driver::in_memory_emulator::EmulatorStore>,
    /// Emulator account reference for operation construction.
    pub emulator_account: AccountReference,

    /// Driver wired to a real Cosmos DB account (if configured).
    pub real_driver: Option<Arc<CosmosDriver>>,
    /// Real account reference (if configured).
    pub real_account: Option<AccountReference>,

    /// Unique run ID to isolate resources across test runs.
    pub run_id: String,
}

impl DualBackend {
    /// Sets up both backends.
    ///
    /// The emulator is always created. The real backend is created only when
    /// the environment is configured (see module-level docs).
    pub async fn setup() -> Result<Self, Box<dyn Error>> {
        let _ = tracing_subscriber::fmt::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();

        let run_id = Uuid::new_v4().to_string()[..8].to_string();

        // ── Emulator ─────────────────────────────────────────────
        let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
        )])
        .unwrap()
        .with_consistency(ConsistencyLevel::Session);

        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
        let emulator_store = emulator.store();
        let emulator_runtime = emulator.runtime_builder().build().await?;

        let emulator_account = AccountReference::with_master_key(
            Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
            "dGVzdGtleQ==",
        );
        let emulator_driver = emulator_runtime
            .create_driver(DriverOptions::builder(emulator_account.clone()).build())
            .await?;

        // ── Real account (optional) ─────────────────────────────
        let (real_driver, real_account) = match resolve_real_account().await {
            Ok(Some((driver, account))) => (Some(driver), Some(account)),
            Ok(None) => {
                println!("  [dual-backend] Real account not configured — emulator-only mode");
                (None, None)
            }
            Err(e) => {
                return Err(format!("Failed to set up real account: {}", e).into());
            }
        };

        Ok(Self {
            emulator_driver,
            emulator_store,
            emulator_account,
            real_driver,
            real_account,
            run_id,
        })
    }

    /// Returns `true` when a real Cosmos DB account is available for comparison.
    pub fn has_real_backend(&self) -> bool {
        self.real_driver.is_some()
    }

    /// Whether the configured real account uses Session default consistency.
    ///
    /// Substatus `1002` (ReadSessionNotAvailable) is only produced under Session
    /// consistency; Eventual/Strong reads never emit it. Reads
    /// [`DEFAULT_CONSISTENCY_ENV_VAR`], defaulting to `true` when unset (local
    /// dev accounts and the Session CI legs are Session; the Eventual/Strong
    /// legs set it explicitly).
    pub fn real_account_uses_session_consistency() -> bool {
        std::env::var(DEFAULT_CONSISTENCY_ENV_VAR)
            .map(|v| v.eq_ignore_ascii_case("Session"))
            .unwrap_or(true)
    }

    /// Generates a unique database name scoped to this run.
    pub fn unique_db_name(&self) -> String {
        format!("dual-test-{}", self.run_id)
    }

    /// Pre-provisions a database and container in the emulator store. Defaults to
    /// V2 `Hash`; use [`Self::provision_emulator_v1`] for V1 coverage.
    pub fn provision_emulator(&self, db: &str, container: &str, pk_path: &str) {
        self.provision_emulator_with_version(db, container, pk_path, 2);
    }

    /// Pre-provisions a V1 `Hash` container in the emulator store.
    pub fn provision_emulator_v1(&self, db: &str, container: &str, pk_path: &str) {
        self.provision_emulator_with_version(db, container, pk_path, 1);
    }

    fn provision_emulator_with_version(
        &self,
        db: &str,
        container: &str,
        pk_path: &str,
        pk_version: u32,
    ) {
        self.emulator_store.create_database(db);
        self.emulator_store.create_container(
            db,
            container,
            serde_json::from_value(serde_json::json!({
                "paths": [pk_path],
                "kind": "Hash",
                "version": pk_version,
            }))
            .unwrap(),
        );
    }

    /// Creates a database on the real account via the driver.
    pub async fn create_real_database(&self, db_name: &str) -> Result<(), Box<dyn Error>> {
        if let (Some(driver), Some(account)) = (&self.real_driver, &self.real_account) {
            let body = serde_json::to_vec(&serde_json::json!({"id": db_name}))?;
            let op = CosmosOperation::create_database(account.clone()).with_body(body);
            let result = driver
                .execute_singleton_operation(op, OperationOptions::default())
                .await?;
            assert!(
                result.status().is_success(),
                "Failed to create real database '{}': {}",
                db_name,
                result.status(),
            );
        }
        Ok(())
    }

    /// Creates a container on the real account via the driver. Defaults to V2
    /// `Hash` partition key.
    pub async fn create_real_container(
        &self,
        db_name: &str,
        container_name: &str,
        pk_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.create_real_container_with_version(db_name, container_name, pk_path, 2)
            .await
    }

    /// Creates a V1 `Hash` container on the real account via the driver.
    pub async fn create_real_container_v1(
        &self,
        db_name: &str,
        container_name: &str,
        pk_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.create_real_container_with_version(db_name, container_name, pk_path, 1)
            .await
    }

    async fn create_real_container_with_version(
        &self,
        db_name: &str,
        container_name: &str,
        pk_path: &str,
        pk_version: u32,
    ) -> Result<(), Box<dyn Error>> {
        if let (Some(driver), Some(account)) = (&self.real_driver, &self.real_account) {
            let db_ref = DatabaseReference::from_name(account.clone(), db_name.to_string());
            let body = serde_json::to_vec(&serde_json::json!({
                "id": container_name,
                "partitionKey": {"paths": [pk_path], "kind": "Hash", "version": pk_version},
            }))?;
            let op = CosmosOperation::create_container(db_ref).with_body(body);
            let result = driver
                .execute_singleton_operation(op, OperationOptions::default())
                .await?;
            assert!(
                result.status().is_success(),
                "Failed to create real container '{}/{}': {}",
                db_name,
                container_name,
                result.status(),
            );
        }
        Ok(())
    }

    /// Deletes a database on the real account (best-effort cleanup).
    pub async fn cleanup_real_database(&self, db_name: &str) {
        if let (Some(driver), Some(account)) = (&self.real_driver, &self.real_account) {
            let db_ref = DatabaseReference::from_name(account.clone(), db_name.to_string());
            let _ = driver
                .execute_singleton_operation(
                    CosmosOperation::delete_database(db_ref),
                    OperationOptions::default(),
                )
                .await;
        }
    }

    /// Waits until an existing sentinel item is point-readable from every region
    /// in [`MULTI_REGION_READ_REGIONS`] that the configured account actually
    /// advertises, proving the container is replicated and servable account-wide.
    /// No-op when the real backend is not configured.
    ///
    /// The caller must have already created `(pk, item_id)`. For each region this
    /// builds a region-pinned driver with cross-region failover and hedging
    /// disabled and point-reads the sentinel, then confirms via the response
    /// diagnostics that the *targeted* region served the read — so a healthy
    /// region can neither mask a lagging one nor produce a false pass. A region
    /// the account does not advertise is served by a different region and is
    /// silently skipped. Panics if an advertised region never catches up within
    /// the bounded poll window.
    pub async fn wait_for_sentinel_readable_from_all_regions(
        &self,
        db: &str,
        container: &str,
        pk: &str,
        item_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let Some(account) = self.real_account.clone() else {
            return Ok(());
        };

        // Pin reads to a single region and forbid failover/hedging so a success
        // served by the targeted region is unambiguous.
        let opts = OperationOptionsBuilder::new()
            .with_max_failover_retry_count(0)
            .with_availability_strategy(AvailabilityStrategy::Disabled)
            .build();

        let setup_timeout = super::setup_timeout();

        for region in MULTI_REGION_READ_REGIONS {
            let runtime = CosmosDriverRuntime::builder().build().await?;
            let driver = runtime
                .create_driver(
                    DriverOptions::builder(account.clone())
                        .with_preferred_regions(vec![region.clone()])
                        .build(),
                )
                .await?;

            let mut proven = false; // targeted region itself served the sentinel
            let mut absent = false; // a substitute region served it → not in this account
            let mut last_error = None;
            let mut backoff = Duration::from_millis(250);
            let deadline = Instant::now() + setup_timeout;

            while Instant::now() < deadline {
                match driver.resolve_container(db, container).await {
                    Ok(region_container) => {
                        let probe = driver
                            .execute_singleton_operation(
                                CosmosOperation::read_item(ItemReference::from_name(
                                    &region_container,
                                    PartitionKey::from(pk.to_string()),
                                    item_id.to_string(),
                                )),
                                opts.clone(),
                            )
                            .await;
                        match probe {
                            Ok(response) if response.status().is_success() => {
                                // `with_preferred_regions` only reorders; a region the
                                // account lacks is dropped and the read is served by a
                                // different region. Success is proof only when the
                                // targeted region served it.
                                if response
                                    .diagnostics_ref()
                                    .regions_contacted()
                                    .contains(region)
                                {
                                    proven = true;
                                } else {
                                    absent = true;
                                }
                                break;
                            }
                            Ok(response) => {
                                last_error = Some(format!("read returned {}", response.status()));
                            }
                            Err(e) => {
                                last_error = Some(e.to_string());
                            }
                        }
                    }
                    Err(e) if collection_create_in_progress(&e) => {
                        last_error = Some(e.to_string());
                    }
                    Err(e) => return Err(Box::new(e)),
                }

                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    break;
                }
                tokio::time::sleep(backoff.min(remaining)).await;
                backoff = (backoff + backoff).min(Duration::from_secs(5));
            }
            // An advertised region must catch up; one the account lacks is skipped.
            assert!(
                proven || absent,
                "Sentinel item '{item_id}' never became readable from advertised region {region:?} within {setup_timeout:?}; last error: {last_error:?}",
            );
        }
        Ok(())
    }

    /// Executes an operation against both backends and compares results.
    ///
    /// `build_op` receives an `AccountReference` and a `ContainerReference` and
    /// must return a `(CosmosOperation, OperationOptions)` pair. It is called
    /// twice: once for the emulator, once for the real account.
    ///
    /// Returns `(emulator_response, Option<real_response>)`.
    pub async fn execute_and_compare<F>(
        &self,
        emulator_container: &ContainerReference,
        real_container: Option<&ContainerReference>,
        build_op: F,
        header_spec: &HeaderValidationSpec,
        body_spec: BodyValidationSpec,
    ) -> Result<(CosmosResponse, Option<CosmosResponse>), Box<dyn Error>>
    where
        F: Fn(&ContainerReference) -> (CosmosOperation, OperationOptions),
    {
        // Run against emulator
        let (emu_op, emu_opts) = build_op(emulator_container);
        let emu_response = self
            .emulator_driver
            .execute_singleton_operation(emu_op, emu_opts)
            .await?;

        // Run against real account (if available)
        let real_response =
            if let (Some(driver), Some(real_ctr)) = (&self.real_driver, real_container) {
                let (real_op, real_opts) = build_op(real_ctr);
                let resp = driver
                    .execute_singleton_operation(real_op, real_opts)
                    .await?;
                Some(resp)
            } else {
                None
            };

        // Compare when both are available
        if let Some(ref real_resp) = real_response {
            let real_snap = ResponseSnapshot::capture(real_resp, "real");
            let emu_snap = ResponseSnapshot::capture(&emu_response, "emulator");
            compare_responses(&real_snap, &emu_snap, header_spec, body_spec);
        }

        Ok((emu_response, real_response))
    }

    /// Convenience: execute an operation built from `AccountReference` (for
    /// account-level operations like create-database).
    pub async fn execute_account_op_and_compare<F>(
        &self,
        build_op: F,
        header_spec: &HeaderValidationSpec,
        body_spec: BodyValidationSpec,
    ) -> Result<(CosmosResponse, Option<CosmosResponse>), Box<dyn Error>>
    where
        F: Fn(&AccountReference) -> (CosmosOperation, OperationOptions),
    {
        let (emu_op, emu_opts) = build_op(&self.emulator_account);
        let emu_response = self
            .emulator_driver
            .execute_singleton_operation(emu_op, emu_opts)
            .await?;

        let real_response =
            if let (Some(driver), Some(account)) = (&self.real_driver, &self.real_account) {
                let (real_op, real_opts) = build_op(account);
                let resp = driver
                    .execute_singleton_operation(real_op, real_opts)
                    .await?;
                Some(resp)
            } else {
                None
            };

        if let Some(ref real_resp) = real_response {
            let real_snap = ResponseSnapshot::capture(real_resp, "real");
            let emu_snap = ResponseSnapshot::capture(&emu_response, "emulator");
            compare_responses(&real_snap, &emu_snap, header_spec, body_spec);
        }

        Ok((emu_response, real_response))
    }
}

/// Tries to connect to a real Cosmos DB account from environment variables.
///
/// Returns `Ok(None)` when the env var is unset and mode is not `required`.
async fn resolve_real_account(
) -> Result<Option<(Arc<CosmosDriver>, AccountReference)>, Box<dyn Error>> {
    let mode = std::env::var(TEST_MODE_ENV_VAR)
        .unwrap_or_default()
        .to_lowercase();

    if mode == "skipped" {
        return Ok(None);
    }

    let conn_str_raw = match std::env::var(CONNECTION_STRING_ENV_VAR) {
        Ok(val) if !val.is_empty() => val,
        _ => {
            if mode == "required" {
                panic!(
                    "{} is not set but test mode is 'required'",
                    CONNECTION_STRING_ENV_VAR
                );
            }
            return Ok(None);
        }
    };

    // The CI test-setup script sets the value to "emulator" as a sentinel
    // when the Docker Cosmos DB Emulator is running. That is not a real
    // connection string — treat it as "not configured" for dual-backend.
    if conn_str_raw.eq_ignore_ascii_case("emulator") {
        return Ok(None);
    }

    let conn_str: ConnectionString = conn_str_raw.parse()?;
    let endpoint: Url = conn_str.account_endpoint().parse()?;
    let key = conn_str.account_key().secret().to_string();
    let account = AccountReference::with_master_key(endpoint, key);

    let mut pool_builder = ConnectionPoolOptions::builder();
    // If connecting to the local emulator, disable cert validation
    if conn_str.account_endpoint().contains("localhost") {
        pool_builder = pool_builder.with_server_certificate_validation(
            ServerCertificateValidation::RequiredUnlessEmulator,
        );
    }
    let pool = pool_builder.build()?;

    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(pool)
        .build()
        .await?;

    let driver = runtime
        .create_driver(DriverOptions::builder(account.clone()).build())
        .await?;

    Ok(Some((driver, account)))
}
