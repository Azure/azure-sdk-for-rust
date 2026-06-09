// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver test client for emulator-based E2E tests.

#[cfg(feature = "fault_injection")]
use azure_data_cosmos_driver::fault_injection::FaultInjectionRule;
use azure_data_cosmos_driver::{
    diagnostics::{DiagnosticsContext, PipelineType, TransportSecurity},
    driver::CosmosDriverRuntime,
    models::{
        AccountReference, ConnectionString, ContainerReference, CosmosOperation, CosmosResponse,
        DatabaseReference, ItemReference, PartitionKey,
    },
    options::{
        ConnectionPoolOptions, DriverOptions, EmulatorServerCertValidation, OperationOptions,
        Region,
    },
};
use std::{error::Error, future::Future, sync::Arc};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use super::env::{
    get_test_mode, is_azure_pipelines, CosmosTestMode, CONNECTION_STRING_ENV_VAR,
    EMULATOR_CONNECTION_STRING,
};

/// A test client that provides access to a Cosmos DB driver for testing.
pub struct DriverTestClient {
    runtime: Arc<CosmosDriverRuntime>,
    account: AccountReference,
}

/// Resolved test environment containing account and connection pool configuration.
pub struct TestEnv {
    pub account: AccountReference,
    pub connection_pool: ConnectionPoolOptions,
}

/// Resolves the test environment from environment variables.
///
/// Returns `Ok(None)` if the environment is not configured and tests should be skipped.
pub fn resolve_test_env() -> Result<Option<TestEnv>, Box<dyn Error>> {
    let _ = tracing_subscriber::fmt::fmt()
        .with_env_filter(
            EnvFilter::builder()
                // Tests with intentional failures cause noise, so we set the default level to "off"
                // to silence them unless the user explicitly configures it.
                .with_default_directive("off".parse().unwrap())
                .from_env_lossy(),
        )
        .try_init();

    let test_mode = get_test_mode();
    if test_mode == CosmosTestMode::Skipped {
        return Ok(None);
    }

    let connection_string = match std::env::var(CONNECTION_STRING_ENV_VAR) {
        Ok(val) if val.to_lowercase() == "emulator" => EMULATOR_CONNECTION_STRING.to_string(),
        Ok(val) => val,
        Err(_) => {
            if test_mode == CosmosTestMode::Required || is_azure_pipelines() {
                panic!(
                    "{} is not set but test mode is required",
                    CONNECTION_STRING_ENV_VAR
                );
            }
            return Ok(None);
        }
    };

    let conn_str: ConnectionString = connection_string.parse()?;
    let endpoint = conn_str.account_endpoint().parse()?;
    let key = conn_str.account_key().secret().to_string();
    let account = AccountReference::with_master_key(endpoint, key);

    let mut connection_pool_builder = ConnectionPoolOptions::builder();
    if connection_string.eq_ignore_ascii_case(EMULATOR_CONNECTION_STRING) {
        connection_pool_builder = connection_pool_builder
            .with_emulator_server_cert_validation(EmulatorServerCertValidation::DangerousDisabled);
    }
    let connection_pool = connection_pool_builder.build()?;

    Ok(Some(TestEnv {
        account,
        connection_pool,
    }))
}

impl DriverTestClient {
    /// Creates a new test client from environment variables.
    ///
    /// If the `AZURE_COSMOS_CONNECTION_STRING` environment variable is set to
    /// "emulator", uses the well-known emulator connection string. Otherwise,
    /// parses the provided connection string.
    ///
    /// Returns `None` if:
    /// - The environment variable is not set and test mode is not "required"
    /// - The test mode is "skipped"
    pub async fn from_env() -> Result<Option<Self>, Box<dyn Error>> {
        let Some(env) = resolve_test_env()? else {
            return Ok(None);
        };

        let runtime = CosmosDriverRuntime::builder()
            .with_connection_pool(env.connection_pool)
            .build()
            .await?;

        Ok(Some(Self {
            runtime,
            account: env.account,
        }))
    }

    /// Creates a new test client from environment variables with fault injection rules.
    ///
    /// Behaves like [`from_env`](Self::from_env) but configures the runtime with fault injection
    /// rules that will intercept matching operations.
    #[cfg(feature = "fault_injection")]
    pub async fn from_env_with_fault_injection(
        rules: Vec<Arc<FaultInjectionRule>>,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let Some(env) = resolve_test_env()? else {
            return Ok(None);
        };

        let runtime = CosmosDriverRuntime::builder()
            .with_connection_pool(env.connection_pool)
            .with_fault_injection_rules(rules)?
            .build()
            .await?;

        Ok(Some(Self {
            runtime,
            account: env.account,
        }))
    }

    /// Runs a test with access to a driver and run context.
    ///
    /// The test will be skipped if the environment is not configured.
    pub async fn run<F, Fut>(f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(DriverTestRunContext) -> Fut,
        Fut: Future<Output = Result<(), Box<dyn Error>>>,
    {
        let Some(client) = Self::from_env().await? else {
            println!("Skipping test: Cosmos DB environment not configured");
            return Ok(());
        };

        let run_context = DriverTestRunContext::new(client);
        f(run_context).await
    }

    /// Runs a test with fault injection rules and access to a driver and run context.
    ///
    /// The test will be skipped if the environment is not configured.
    #[cfg(feature = "fault_injection")]
    pub async fn run_with_fault_injection<F, Fut>(
        rules: Vec<Arc<FaultInjectionRule>>,
        f: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(DriverTestRunContext) -> Fut,
        Fut: Future<Output = Result<(), Box<dyn Error>>>,
    {
        let Some(client) = Self::from_env_with_fault_injection(rules).await? else {
            println!("Skipping test: Cosmos DB environment not configured");
            return Ok(());
        };

        let run_context = DriverTestRunContext::new(client);
        f(run_context).await
    }

    /// Runs a test with fault injection rules and a unique database that will be cleaned up
    /// after the test.
    #[cfg(feature = "fault_injection")]
    pub async fn run_with_unique_db_and_fault_injection<F, Fut>(
        rules: Vec<Arc<FaultInjectionRule>>,
        f: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(DriverTestRunContext, DatabaseReference) -> Fut,
        Fut: Future<Output = Result<(), Box<dyn Error>>>,
    {
        Self::run_with_fault_injection(rules, async |context| {
            let db_name = context.unique_database_name();
            let db_ref = context.create_database(&db_name).await?;

            let result = f(context.clone(), db_ref.clone()).await;

            // Cleanup (best effort)
            let _ = context.delete_database(&db_ref).await;

            result
        })
        .await
    }

    /// Like [`run_with_unique_db_and_fault_injection`](Self::run_with_unique_db_and_fault_injection)
    /// but also applies the given [`OperationOptions`] to the driver runtime.
    #[cfg(feature = "fault_injection")]
    pub async fn run_with_unique_db_and_fault_injection_options<F, Fut>(
        rules: Vec<Arc<FaultInjectionRule>>,
        operation_options: OperationOptions,
        f: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(DriverTestRunContext, DatabaseReference) -> Fut,
        Fut: Future<Output = Result<(), Box<dyn Error>>>,
    {
        let Some(env) = resolve_test_env()? else {
            println!("Skipping test: Cosmos DB environment not configured");
            return Ok(());
        };

        let runtime = CosmosDriverRuntime::builder()
            .with_connection_pool(env.connection_pool)
            .with_fault_injection_rules(rules)?
            .with_operation_options(operation_options)
            .build()
            .await?;

        let client = Self {
            runtime,
            account: env.account,
        };
        let context = DriverTestRunContext::new(client);

        let db_name = context.unique_database_name();
        let db_ref = context.create_database(&db_name).await?;

        let result = f(context.clone(), db_ref.clone()).await;

        // Cleanup (best effort)
        let _ = context.delete_database(&db_ref).await;

        result
    }

    /// Like [`run_with_unique_db_and_fault_injection_options`](Self::run_with_unique_db_and_fault_injection_options)
    /// but additionally pre-configures driver-level `preferred_regions`,
    /// which is required for cross-region hedging eligibility per
    /// `HEDGING_SPEC.md` §5.2 (the §5.1 `should_hedge()` short-circuits
    /// when no application-preferred regions are configured).
    ///
    /// Pre-warms the runtime's per-account driver cache with explicit
    /// [`DriverOptions`] so subsequent `get_or_create_driver(.., None)`
    /// calls from the per-operation test helpers (`read_item`,
    /// `create_item_with_pk`, …) hit the cached driver and inherit the
    /// configured `preferred_regions`.
    #[cfg(feature = "fault_injection")]
    pub async fn run_with_unique_db_and_hedging<F, Fut>(
        rules: Vec<Arc<FaultInjectionRule>>,
        runtime_operation_options: OperationOptions,
        preferred_regions: Vec<Region>,
        f: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(DriverTestRunContext, DatabaseReference) -> Fut,
        Fut: Future<Output = Result<(), Box<dyn Error>>>,
    {
        let Some(env) = resolve_test_env()? else {
            println!("Skipping test: Cosmos DB environment not configured");
            return Ok(());
        };

        let runtime = CosmosDriverRuntime::builder()
            .with_connection_pool(env.connection_pool)
            .with_fault_injection_rules(rules)?
            .with_operation_options(runtime_operation_options)
            .build()
            .await?;

        // Pre-warm the driver cache so per-operation helpers (which call
        // `get_or_create_driver(.., None)`) hit the cached driver with our
        // `preferred_regions`. The cache is keyed on the account endpoint
        // (see `CosmosDriverRuntime::get_or_create_driver`).
        let driver_options = DriverOptions::builder(env.account.clone())
            .with_preferred_regions(preferred_regions)
            .build();
        let _ = runtime
            .get_or_create_driver(env.account.clone(), Some(driver_options))
            .await?;

        let client = Self {
            runtime,
            account: env.account,
        };
        let context = DriverTestRunContext::new(client);

        let db_name = context.unique_database_name();
        let db_ref = context.create_database(&db_name).await?;

        let result = f(context.clone(), db_ref.clone()).await;

        // Cleanup (best effort)
        let _ = context.delete_database(&db_ref).await;

        result
    }

    /// Runs a test with a unique database that will be cleaned up after the test.
    pub async fn run_with_unique_db<F, Fut>(f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(DriverTestRunContext, DatabaseReference) -> Fut,
        Fut: Future<Output = Result<(), Box<dyn Error>>>,
    {
        Self::run(async |context| {
            let db_name = context.unique_database_name();
            let db_ref = context.create_database(&db_name).await?;

            // Run the test
            let result = f(context.clone(), db_ref.clone()).await;

            // Cleanup (best effort)
            let _ = context.delete_database(&db_ref).await;

            result
        })
        .await
    }
}

/// Context for a test run, providing helpers for driver operations.
#[derive(Clone)]
pub struct DriverTestRunContext {
    client: Arc<DriverTestClient>,
    run_id: String,
}

impl DriverTestRunContext {
    fn new(client: DriverTestClient) -> Self {
        Self {
            client: Arc::new(client),
            run_id: Uuid::new_v4().to_string()[..8].to_string(),
        }
    }
    /// Generates a unique database name for this test run.
    pub fn unique_database_name(&self) -> String {
        format!("test-db-{}", self.run_id)
    }

    /// Generates a unique container name for this test run.
    pub fn unique_container_name(&self) -> String {
        let uuid_str = Uuid::new_v4().to_string();
        format!("test-container-{}", &uuid_str[..8])
    }

    /// Creates a database using the driver.
    pub async fn create_database(
        &self,
        db_name: &str,
    ) -> Result<DatabaseReference, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let body = format!(r#"{{"id": "{}"}}"#, db_name);
        let operation = CosmosOperation::create_database(self.client.account.clone())
            .with_body(body.into_bytes());

        let result = driver
            .execute_singleton_operation(operation, OperationOptions::default())
            .await?;

        // Check for success status (201 Created)
        let diagnostics = result.diagnostics();
        let status = diagnostics.status();
        if !status.map(|s| s.is_success()).unwrap_or(false) {
            return Err(format!("Failed to create database, status: {:?}", status).into());
        }

        Ok(DatabaseReference::from_name(
            self.client.account.clone(),
            db_name.to_string(),
        ))
    }

    /// Deletes a database using the driver.
    pub async fn delete_database(
        &self,
        database: &DatabaseReference,
    ) -> Result<(), Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let operation = CosmosOperation::delete_database(database.clone());

        let result = driver
            .execute_singleton_operation(operation, OperationOptions::default())
            .await?;

        // Check for success status (204 No Content)
        let diagnostics = result.diagnostics();
        let status = diagnostics.status();
        if !status.map(|s| s.is_success()).unwrap_or(false) {
            return Err(format!("Failed to delete database, status: {:?}", status).into());
        }

        Ok(())
    }

    /// Creates a container using the driver.
    pub async fn create_container(
        &self,
        database: &DatabaseReference,
        container_name: &str,
        partition_key_path: &str,
    ) -> Result<ContainerReference, Box<dyn Error>> {
        self.create_container_with_pk_paths(database, container_name, &[partition_key_path])
            .await
    }

    /// Creates a container with one or more partition-key paths.
    ///
    /// Pass a single-element slice for a flat partition key (sets `kind`
    /// to `Hash`), or multiple paths for a hierarchical partition key
    /// (sets `kind` to `MultiHash`). Mirrors the `paths` array in the
    /// [PartitionKey definition][pk-spec], with the `kind` derived from
    /// the number of paths.
    ///
    /// [pk-spec]: https://learn.microsoft.com/en-us/rest/api/cosmos-db/create-a-collection
    pub async fn create_container_with_pk_paths(
        &self,
        database: &DatabaseReference,
        container_name: &str,
        partition_key_paths: &[&str],
    ) -> Result<ContainerReference, Box<dyn Error>> {
        assert!(
            !partition_key_paths.is_empty(),
            "container requires at least one partition-key path"
        );
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let paths_json = partition_key_paths
            .iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<_>>()
            .join(",");
        let kind = if partition_key_paths.len() == 1 {
            "Hash"
        } else {
            "MultiHash"
        };
        let body = format!(
            r#"{{"id": "{}", "partitionKey": {{"paths": [{}], "kind": "{}", "version": 2}}}}"#,
            container_name, paths_json, kind
        );
        let operation =
            CosmosOperation::create_container(database.clone()).with_body(body.into_bytes());

        let result = driver
            .execute_singleton_operation(operation, OperationOptions::default())
            .await?;

        // Check for success status (201 Created)
        let diagnostics = result.diagnostics();
        let status = diagnostics.status();
        if !status.map(|s| s.is_success()).unwrap_or(false) {
            return Err(format!("Failed to create container, status: {:?}", status).into());
        }
        let db_name = database
            .name()
            .ok_or_else(|| "database reference must be name-based".to_string())?;
        let container = driver
            .resolve_container_by_name(db_name, container_name)
            .await?;
        Ok(container)
    }

    /// Creates an item using the driver.
    ///
    /// The `item_id` is used to build the [`ItemReference`]. For `Create`
    /// operations the item ID is part of the body JSON, not the URL path
    /// (Cosmos POSTs to the collection feed), so the value of `item_id`
    /// here is used only for PK-range routing and does not need to match the
    /// body's `"id"` field exactly.
    pub async fn create_item(
        &self,
        container: &ContainerReference,
        item_id: &str,
        partition_key: impl Into<PartitionKey>,
        body: &[u8],
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let pk = partition_key.into();
        let item_ref = ItemReference::from_name(container, pk, item_id.to_owned());
        let operation = CosmosOperation::create_item(item_ref).with_body(body.to_vec());

        let result = driver
            .execute_singleton_operation(operation, OperationOptions::default())
            .await?;

        Ok(result)
    }

    /// Creates an item by partition key only (item id is embedded in `body`).
    ///
    /// Convenience overload for tests that include the item id inside the JSON
    /// body and do not want to pass it separately. Uses `"_"` as the routing
    /// item-id placeholder, which is acceptable because `Create` operations
    /// route to the collection feed URL and do not include the item id in the
    /// path.
    pub async fn create_item_with_pk(
        &self,
        container: &ContainerReference,
        partition_key: impl Into<PartitionKey>,
        body: &[u8],
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        self.create_item(container, "_", partition_key, body).await
    }

    /// Reads an item using the driver.
    pub async fn read_item(
        &self,
        container: &ContainerReference,
        item_id: &str,
        partition_key: impl Into<PartitionKey>,
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let pk = partition_key.into();
        let item_ref = ItemReference::from_name(container, pk, item_id.to_owned());
        let operation = CosmosOperation::read_item(item_ref);

        let result = driver
            .execute_singleton_operation(operation, OperationOptions::default())
            .await?;

        Ok(result)
    }

    /// Patches an item using the driver's `OperationType::Patch` RMW loop.
    ///
    /// Mirrors [`read_item`](Self::read_item)'s shape but builds the
    /// [`CosmosOperation::patch_item`] body from a
    /// [`PatchInstructions`](azure_data_cosmos_driver::models::PatchInstructions). The
    /// returned [`CosmosResponse`] is the synthetic response produced by the
    /// patch handler — its body is the locally-merged post-image and its
    /// status/diagnostics are inherited from the underlying conditional
    /// `Replace`.
    ///
    /// If `max_attempts` is `None`, the handler uses
    /// `DEFAULT_PATCH_MAX_ATTEMPTS` (5).
    pub async fn patch_item(
        &self,
        container: &ContainerReference,
        item_id: &str,
        partition_key: impl Into<PartitionKey>,
        patch: &azure_data_cosmos_driver::models::PatchInstructions,
        max_attempts: Option<std::num::NonZeroU8>,
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let pk = partition_key.into();
        let item_ref = ItemReference::from_name(container, pk, item_id.to_owned());
        let body = serde_json::to_vec(patch)?;
        let mut operation = CosmosOperation::patch_item(item_ref).with_body(body);
        if let Some(n) = max_attempts {
            operation = operation.with_patch_max_attempts(n);
        }

        let result = driver
            .execute_operation(operation, OperationOptions::default())
            .await?
            .expect("PATCH operation must return a response");

        Ok(result)
    }

    /// Resolves all partition key ranges for a container, optionally forcing
    /// a refresh of the cached routing map. Exposes the driver's internal
    /// `resolve_all_partition_key_ranges` for tests that need to exercise the
    /// pkrange-cache refresh path directly.
    pub async fn resolve_all_partition_key_ranges(
        &self,
        container: &ContainerReference,
        force_refresh: bool,
    ) -> Result<
        Option<Vec<azure_data_cosmos_driver::models::partition_key_range::PartitionKeyRange>>,
        Box<dyn Error>,
    > {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;
        Ok(driver
            .resolve_all_partition_key_ranges(container, force_refresh)
            .await)
    }

    /// Validates diagnostics for a successful data plane operation.
    pub fn validate_data_plane_diagnostics(
        &self,
        diagnostics: &DiagnosticsContext,
        expected_status: u16,
    ) {
        // Check status code
        let status = diagnostics.status();
        assert!(status.is_some(), "Diagnostics should have a status code");
        assert_eq!(
            u16::from(status.unwrap().status_code()),
            expected_status,
            "Status code should match expected"
        );

        // Check activity ID
        assert!(
            !diagnostics.activity_id().as_str().is_empty(),
            "Activity ID should not be empty"
        );

        // Check duration
        assert!(
            !diagnostics.duration().is_zero(),
            "Duration should be non-zero"
        );

        // Check requests
        let requests = diagnostics.requests();
        assert!(!requests.is_empty(), "Should have at least one request");

        // Check first request has correct pipeline type
        let first_request = &requests[0];
        assert_eq!(
            first_request.pipeline_type(),
            PipelineType::DataPlane,
            "Should use data plane pipeline for item operations"
        );

        // Check transport security for emulator. The legacy emulator and the
        // vnext emulator in HTTPS mode use a self-signed cert and surface as
        // `EmulatorWithInsecureCertificates`. The vnext emulator in HTTP mode
        // has no TLS at all and is classified as `Secure` today (the enum
        // predates plain-HTTP emulator support — tracked separately).
        if first_request.endpoint().contains("localhost")
            || first_request.endpoint().contains("127.0.0.1")
        {
            let expected = if first_request.endpoint().starts_with("https://") {
                TransportSecurity::EmulatorWithInsecureCertificates
            } else {
                TransportSecurity::Secure
            };
            assert_eq!(
                first_request.transport_security(),
                expected,
                "Unexpected transport security for emulator endpoint {}",
                first_request.endpoint()
            );
        }

        // Check request charge is non-negative
        assert!(
            first_request.request_charge().value() >= 0.0,
            "Request charge should be non-negative"
        );
    }
}
