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

use azure_core::http::Url;
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntime,
    in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, ConnectionString, ContainerReference, CosmosOperation, CosmosResponse,
        DatabaseReference,
    },
    options::{
        ConnectionPoolOptions, DriverOptions, OperationOptions, ServerCertificateValidation,
    },
    CosmosDriver,
};
use std::{error::Error, sync::Arc};
use uuid::Uuid;

use super::validation::{
    compare_responses, BodyValidationSpec, HeaderValidationSpec, ResponseSnapshot,
};

/// Environment variable for the real-account connection string.
const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";

/// Environment variable controlling test mode.
const TEST_MODE_ENV_VAR: &str = "AZURE_COSMOS_TEST_MODE";

/// Gateway URL used by the in-memory emulator.
const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";

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
