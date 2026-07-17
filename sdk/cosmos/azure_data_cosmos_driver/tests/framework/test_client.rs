// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver test client for emulator-based E2E tests.

use azure_core::http::StatusCode;
#[cfg(feature = "fault_injection")]
use azure_data_cosmos_driver::fault_injection::FaultInjectionRule;
#[cfg(feature = "__internal_testing")]
use azure_data_cosmos_driver::CosmosDriver;
use azure_data_cosmos_driver::{
    diagnostics::{DiagnosticsContext, PipelineType, TransportSecurity},
    driver::CosmosDriverRuntime,
    error::CosmosError,
    models::{
        AccountReference, ConnectionString, ContainerReference, CosmosOperation, CosmosResponse,
        DatabaseReference, ItemReference, PartitionKey,
    },
    options::{
        ConnectionPoolOptions, DriverOptions, OperationOptions, PartitionFailoverOptions, Region,
        ServerCertificateValidation,
    },
    SubStatusCode,
};
use std::{error::Error, future::Future, sync::Arc, time::Duration};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use super::env::{
    get_test_mode, is_azure_pipelines, CosmosTestMode, CONNECTION_STRING_ENV_VAR,
    EMULATOR_CONNECTION_STRING, GATEWAY_V2_ENDPOINT_ENV_VAR, GATEWAY_V2_KEY_ENV_VAR,
    GATEWAY_V2_MULTI_REGION_ENDPOINT_ENV_VAR, GATEWAY_V2_MULTI_REGION_KEY_ENV_VAR,
};

/// A test client that provides access to a Cosmos DB driver for testing.
pub struct DriverTestClient {
    runtime: Arc<CosmosDriverRuntime>,
    account: AccountReference,
    /// Driver-level preferred regions applied to every driver created by the
    /// per-operation helpers (`create_database`, `read_item`, …). Empty by
    /// default; populated by [`run_with_unique_db_and_hedging`] for the
    /// hedging path, which requires application-preferred regions to be set
    /// per `HEDGING_SPEC.md` §5.2.
    preferred_regions: Vec<Region>,
    /// Driver-level fault-injection rules applied to every driver created by
    /// the per-operation helpers. Empty by default; populated by the
    /// fault-injection entry points (`from_env_with_fault_injection`,
    /// `run_with_fault_injection`, etc.) so that FI rules are configured on
    /// each per-operation driver rather than on the shared runtime.
    #[cfg(feature = "fault_injection")]
    fault_injection_rules: Vec<Arc<FaultInjectionRule>>,
    /// Driver-level partition-failover / PPCB options applied to every driver
    /// created by the per-operation helpers. `None` means the driver inherits
    /// the [`PartitionFailoverOptions::default`] values; populated by the
    /// `run_with_unique_db_and_fault_injection_partition_failover_options`
    /// entry point so PPCB-tuning tests can configure thresholds and sweep
    /// intervals at the driver layer (where these knobs now live).
    partition_failover_options: Option<PartitionFailoverOptions>,
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

    // For `test_category = "gateway_v2"` and `"gateway_v2_multi_region"` builds,
    // the ARM-provisioned `AZURE_COSMOS_CONNECTION_STRING` account does NOT
    // advertise a Gateway 2.0 endpoint. Fault-injection rules scoped to
    // `TransportKind::GatewayV2` will therefore never fire against it. Use the
    // pre-provisioned Gateway 2.0 account whose credentials are surfaced in
    // `AZURE_COSMOS_GW_V2_ENDPOINT` / `AZURE_COSMOS_GW_V2_KEY` by
    // `sdk/cosmos/ci.yml`. We deliberately do NOT fall back to the standard
    // connection string here — running gateway_v2 tests against a non-Gateway-2.0
    // account would silently produce confusing failures (transport-kind-gated
    // fault rules would never match).
    #[cfg(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    ))]
    {
        return resolve_gateway_v2_env(test_mode);
    }

    #[allow(unreachable_code)]
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
        connection_pool_builder = connection_pool_builder.with_server_certificate_validation(
            ServerCertificateValidation::RequiredUnlessEmulator,
        );
    }
    let connection_pool = connection_pool_builder.build()?;

    Ok(Some(TestEnv {
        account,
        connection_pool,
    }))
}

/// Builds a [`TestEnv`] from the pre-provisioned Gateway 2.0 account
/// credentials.
///
/// Picks the endpoint/key pair based on the active `test_category`:
///
/// * `test_category = "gateway_v2_multi_region"` →
///   `AZURE_COSMOS_GW_V2_MULTI_REGION_ENDPOINT` /
///   `AZURE_COSMOS_GW_V2_MULTI_REGION_KEY` (a multi-region GW_V2 account).
/// * `test_category = "gateway_v2"` →
///   `AZURE_COSMOS_GW_V2_ENDPOINT` / `AZURE_COSMOS_GW_V2_KEY` (a single-region
///   GW_V2 account).
///
/// Returns `Ok(None)` when both env vars are missing or empty in local-dev
/// mode (the caller's `cfg` block returns `Ok(None)` directly). Panics on
/// Azure Pipelines or `Required` test mode when only one of the two vars is
/// set — that's a misconfigured pipeline rather than an intentional skip.
#[cfg(any(
    test_category = "gateway_v2",
    test_category = "gateway_v2_multi_region"
))]
fn resolve_gateway_v2_env(test_mode: CosmosTestMode) -> Result<Option<TestEnv>, Box<dyn Error>> {
    fn read(name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|v| !v.trim().is_empty())
    }

    // Multi-region tests need a multi-region GW_V2 account; single-region
    // tests use the single-region account. Both fall back to "no GW_V2
    // env vars set" rather than crossing wires between pools.
    #[cfg(test_category = "gateway_v2_multi_region")]
    let (endpoint_var, key_var) = (
        GATEWAY_V2_MULTI_REGION_ENDPOINT_ENV_VAR,
        GATEWAY_V2_MULTI_REGION_KEY_ENV_VAR,
    );
    #[cfg(all(
        test_category = "gateway_v2",
        not(test_category = "gateway_v2_multi_region")
    ))]
    let (endpoint_var, key_var) = (GATEWAY_V2_ENDPOINT_ENV_VAR, GATEWAY_V2_KEY_ENV_VAR);

    let endpoint = read(endpoint_var);
    let key = read(key_var);

    match (endpoint, key) {
        (Some(endpoint), Some(key)) => {
            let normalized = normalize_gateway_v2_endpoint(&endpoint);
            let endpoint = normalized
                .parse()
                .map_err(|e: url::ParseError| -> Box<dyn Error> {
                    format!(
                        "{} value {:?} (normalized to {:?}) is not a valid URL: {}",
                        endpoint_var, endpoint, normalized, e,
                    )
                    .into()
                })?;
            let account = AccountReference::with_master_key(endpoint, key);
            let connection_pool = ConnectionPoolOptions::builder().build()?;
            Ok(Some(TestEnv {
                account,
                connection_pool,
            }))
        }
        (None, None) => {
            if test_mode == CosmosTestMode::Required || is_azure_pipelines() {
                panic!(
                    "{} / {} are not set but test_category=\"gateway_v2\" requires a \
                     pre-provisioned Gateway 2.0 account",
                    endpoint_var, key_var,
                );
            }
            Ok(None)
        }
        (endpoint, key) => panic!(
            "exactly one of {} / {} is set ({}={}, {}={}); both must be present \
             for test_category=\"gateway_v2\"",
            endpoint_var,
            key_var,
            endpoint_var,
            if endpoint.is_some() { "set" } else { "unset" },
            key_var,
            if key.is_some() { "set" } else { "unset" },
        ),
    }
}

/// Normalizes a Gateway 2.0 endpoint string so it can be parsed by `Url::parse`:
/// trims whitespace, prepends `https://` when no scheme is present, and appends a
/// trailing `/` to match Cosmos's canonical `https://<host>/` form.
#[cfg(any(
    test_category = "gateway_v2",
    test_category = "gateway_v2_multi_region"
))]
fn normalize_gateway_v2_endpoint(raw: &str) -> String {
    let trimmed = raw.trim();
    let with_scheme = if trimmed.contains("://") {
        trimmed.to_owned()
    } else {
        format!("https://{trimmed}")
    };
    if with_scheme.ends_with('/') {
        with_scheme
    } else {
        format!("{with_scheme}/")
    }
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
            preferred_regions: Vec::new(),
            #[cfg(feature = "fault_injection")]
            fault_injection_rules: Vec::new(),
            partition_failover_options: None,
        }))
    }

    /// Creates a new test client from environment variables with fault injection rules.
    ///
    /// Behaves like [`from_env`](Self::from_env) but configures the per-operation
    /// drivers with fault injection rules that will intercept matching operations.
    #[cfg(feature = "fault_injection")]
    pub async fn from_env_with_fault_injection(
        rules: Vec<Arc<FaultInjectionRule>>,
    ) -> Result<Option<Self>, Box<dyn Error>> {
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
            preferred_regions: Vec::new(),
            fault_injection_rules: rules,
            partition_failover_options: None,
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
            .with_default_operation_options(operation_options)
            .build()
            .await?;

        let client = Self {
            runtime,
            account: env.account,
            preferred_regions: Vec::new(),
            fault_injection_rules: rules,
            partition_failover_options: None,
        };
        let context = DriverTestRunContext::new(client);

        let db_name = context.unique_database_name();
        let db_ref = context.create_database(&db_name).await?;

        let result = f(context.clone(), db_ref.clone()).await;

        // Cleanup (best effort)
        let _ = context.delete_database(&db_ref).await;

        result
    }

    /// Like [`run_with_unique_db_and_fault_injection`](Self::run_with_unique_db_and_fault_injection)
    /// but additionally applies the given [`PartitionFailoverOptions`] to
    /// every driver created by the per-operation helpers.
    ///
    /// These are driver-level (not runtime-level) options: PPCB enable,
    /// failure thresholds, partition-unavailability duration and failback
    /// sweep interval, etc. Tests use this entry point to tune the PPCB
    /// fast/loose so the harness does not have to wait the 300s production
    /// failback default.
    #[cfg(feature = "fault_injection")]
    pub async fn run_with_unique_db_and_fault_injection_partition_failover_options<F, Fut>(
        rules: Vec<Arc<FaultInjectionRule>>,
        partition_failover_options: PartitionFailoverOptions,
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
            .build()
            .await?;

        let client = Self {
            runtime,
            account: env.account,
            preferred_regions: Vec::new(),
            fault_injection_rules: rules,
            partition_failover_options: Some(partition_failover_options),
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
    /// The `preferred_regions` are stored on the client and applied to every
    /// driver created by the per-operation helpers (`read_item`,
    /// `create_item_with_pk`, …) via their internal
    /// [`DriverOptions`].
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
            .with_default_operation_options(runtime_operation_options)
            .build()
            .await?;

        let client = Self {
            runtime,
            account: env.account,
            preferred_regions,
            fault_injection_rules: rules,
            partition_failover_options: None,
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

    /// Returns the underlying driver runtime.
    ///
    /// Useful for assertions that need to inspect runtime-level resolved
    /// configuration that is populated once at build time, such as the
    /// `{ENV}_OVERRIDE` kill-switch layer exposed via
    /// [`CosmosDriverRuntime::env_override_operation_options`].
    pub fn runtime(&self) -> &Arc<CosmosDriverRuntime> {
        &self.client.runtime
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

    /// Builds the per-operation [`DriverOptions`] used by the helpers in
    /// this context. Carries the client's account, any `preferred_regions`
    /// configured by the hedging entry point, and any fault-injection rules
    /// configured by the FI entry points, so that every driver created by
    /// these helpers inherits them.
    fn driver_options(&self) -> Result<DriverOptions, Box<dyn Error>> {
        let mut builder = DriverOptions::builder(self.client.account.clone());
        if !self.client.preferred_regions.is_empty() {
            builder = builder.with_preferred_regions(self.client.preferred_regions.clone());
        }
        if let Some(pfo) = &self.client.partition_failover_options {
            builder = builder.with_partition_failover_options(pfo.clone());
        }
        #[cfg(feature = "fault_injection")]
        if !self.client.fault_injection_rules.is_empty() {
            builder =
                builder.with_fault_injection_rules(self.client.fault_injection_rules.clone())?;
        }
        Ok(builder.build())
    }

    fn is_transport_generated_503(error: &(dyn Error + 'static)) -> bool {
        let mut current = Some(error);
        while let Some(err) = current {
            if let Some(cosmos) = err.downcast_ref::<CosmosError>() {
                return cosmos.status().is_transport_generated_503();
            }
            current = err.source();
        }
        false
    }

    /// Retries setup/seed operations that fail with a synthetic transport-generated
    /// 503. Emulator and live-test accounts can briefly publish regional
    /// endpoints before DNS is ready, which makes otherwise unrelated tests flaky
    /// while they seed data. This helper is intentionally opt-in so tests that
    /// assert operation failures do not mask the failure being exercised.
    pub async fn retry_transient_transport<T, F, Fut>(
        &self,
        operation: &str,
        mut f: F,
    ) -> Result<T, Box<dyn Error>>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, Box<dyn Error>>>,
    {
        let mut delay = Duration::from_millis(250);
        let mut last_error = None;
        for attempt in 1..=6 {
            match f().await {
                Ok(result) => return Ok(result),
                Err(error) if Self::is_transport_generated_503(error.as_ref()) && attempt < 6 => {
                    last_error = Some(error.to_string());
                    eprintln!(
                        "transient transport failure during {operation}; retrying attempt {attempt}/6 after {delay:?}: {}",
                        last_error.as_deref().unwrap_or("<unknown>")
                    );
                    tokio::time::sleep(delay).await;
                    delay = (delay * 2).min(Duration::from_secs(5));
                }
                Err(error) => return Err(error),
            }
        }
        Err(format!(
            "{operation} failed after transient transport retries: {}",
            last_error.unwrap_or_else(|| "<no error captured>".into())
        )
        .into())
    }

    /// Creates a database using the driver.
    pub async fn create_database(
        &self,
        db_name: &str,
    ) -> Result<DatabaseReference, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .create_driver(self.driver_options()?)
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
            .create_driver(self.driver_options()?)
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
            .create_driver(self.driver_options()?)
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
        // After a successful container CREATE the just-created collection
        // can be briefly unreadable on individual regional gateways with
        // 404/1013 (`CollectionCreateInProgress`) — the create returns
        // 201 from the write region before all regional gateways finish
        // propagating the new collection metadata. Retry with backoff
        // while that race settles. This is a no-op on accounts where the
        // resolve already succeeds on the first call (notably the
        // emulator and most production single-region accounts).
        let mut delay_ms = 500u64;
        let mut last_err_msg: Option<String> = None;
        for _ in 0..12 {
            match driver
                .resolve_container_by_name(db_name, container_name)
                .await
            {
                Ok(c) => return Ok(c),
                Err(e) => {
                    // Match on the typed status/sub-status (404/1013) rather
                    // than substring-scanning the error message.
                    let status = e.status();
                    let create_in_progress = status.status_code() == StatusCode::NotFound
                        && status.sub_status()
                            == Some(SubStatusCode::COLLECTION_CREATE_IN_PROGRESS);
                    if create_in_progress {
                        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                        delay_ms = (delay_ms * 2).min(5000);
                        last_err_msg = Some(format!("{e}"));
                        continue;
                    }
                    return Err(e.into());
                }
            }
        }
        Err(format!(
            "resolve_container_by_name failed after 12 retries: {}",
            last_err_msg.unwrap_or_else(|| "<no error captured>".into())
        )
        .into())
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
            .create_driver(self.driver_options()?)
            .await?;

        let pk = partition_key.into();
        let item_ref = ItemReference::from_name(container, pk, item_id.to_owned());
        let operation = CosmosOperation::create_item(item_ref).with_body(body.to_vec());

        let result = driver
            .execute_singleton_operation(operation, OperationOptions::default())
            .await?;

        Ok(result)
    }

    /// Creates an item as setup data, retrying transient transport-generated
    /// 503s that can occur while emulator/live-test regional DNS converges.
    pub async fn create_seed_item(
        &self,
        container: &ContainerReference,
        item_id: &str,
        partition_key: impl Into<PartitionKey>,
        body: &[u8],
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        let partition_key = partition_key.into();
        self.retry_transient_transport("seed create item", || {
            self.create_item(container, item_id, partition_key.clone(), body)
        })
        .await
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

    /// Creates an item as setup data by partition key only, retrying transient
    /// transport-generated 503s. See [`create_seed_item`](Self::create_seed_item).
    pub async fn create_seed_item_with_pk(
        &self,
        container: &ContainerReference,
        partition_key: impl Into<PartitionKey>,
        body: &[u8],
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        let partition_key = partition_key.into();
        self.create_seed_item(container, "_", partition_key, body)
            .await
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
            .create_driver(self.driver_options()?)
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
            .create_driver(self.driver_options()?)
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
            .create_driver(self.driver_options()?)
            .await?;
        Ok(driver
            .resolve_all_partition_key_ranges(container, force_refresh)
            .await)
    }

    /// Creates a single long-lived driver that the caller holds for the
    /// duration of a test.
    ///
    /// Unlike the per-operation helpers — each of which builds a throwaway
    /// driver — every operation issued through this driver shares its
    /// in-memory routing and hub-region cache state. This is required for
    /// tests that assert on cross-operation cache behavior, where state
    /// populated by one read must be observed by a later read.
    #[cfg(feature = "__internal_testing")]
    pub async fn create_persistent_driver(&self) -> Result<Arc<CosmosDriver>, Box<dyn Error>> {
        Ok(self
            .client
            .runtime
            .create_driver(self.driver_options()?)
            .await?)
    }

    /// Reads an item on a caller-provided persistent driver so the driver's
    /// in-memory cache state persists across reads. Mirrors
    /// [`read_item`](Self::read_item) but reuses the given driver instead of
    /// creating a fresh one per call.
    #[cfg(feature = "__internal_testing")]
    pub async fn read_item_on(
        &self,
        driver: &Arc<CosmosDriver>,
        container: &ContainerReference,
        item_id: &str,
        partition_key: impl Into<PartitionKey>,
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        let pk = partition_key.into();
        let item_ref = ItemReference::from_name(container, pk, item_id.to_owned());
        let operation = CosmosOperation::read_item(item_ref);
        let result = driver
            .execute_singleton_operation(operation, OperationOptions::default())
            .await?;
        Ok(result)
    }

    /// Returns a snapshot of the per-partition hub-region cache for the given
    /// persistent driver.
    #[cfg(feature = "__internal_testing")]
    pub fn hub_region_cache_snapshot(&self, driver: &Arc<CosmosDriver>) -> Vec<(String, String)> {
        driver.__test_only_hub_region_cache_snapshot()
    }

    /// Forces `per_partition_automatic_failover_enabled = true` on the given
    /// driver's in-memory partition state. Required for live tests that
    /// exercise the hub-region caching path when the test account does not
    /// advertise the PPAF account property
    /// (`enable_per_partition_failover_behavior`). Without this override, the
    /// hub-region latch in `build_session_retry_state` never arms and the
    /// cache stays empty regardless of the wire flow.
    #[cfg(feature = "__internal_testing")]
    pub fn force_ppaf_enabled(&self, driver: &Arc<CosmosDriver>) {
        driver.__test_only_force_ppaf_enabled();
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
