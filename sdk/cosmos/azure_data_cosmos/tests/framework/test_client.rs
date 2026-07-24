// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore: TEAMPROJECTID

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    fault_injection::FaultInjectionRule,
    feed::FeedScope,
    models::{ItemResponse, ThroughputProperties},
    options::{
        ConnectionPoolOptions, CreateContainerOptions, ItemReadOptions, Region,
        ServerCertificateValidation,
    },
    CosmosClient, CosmosError, CosmosRuntime, CosmosStatus, PartitionKey, Query, RoutingStrategy,
};
use azure_data_cosmos_driver::models::ConnectionString;
use futures::TryStreamExt;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use std::{str::FromStr, sync::OnceLock};
use tracing_subscriber::EnvFilter;

/// Represents a Cosmos DB client connected to a test account.
pub struct TestClient {
    cosmos_client: Option<CosmosClient>,
}

#[derive(Default)]
pub struct TestClientOptions {
    pub allow_invalid_certificates: bool,
}

pub const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
pub const ACCOUNT_HOST_ENV_VAR: &str = "ACCOUNT_HOST";
pub const ALLOW_INVALID_CERTS_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
pub const TEST_MODE_ENV_VAR: &str = "AZURE_COSMOS_TEST_MODE";
pub const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://127.0.0.1:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";
pub const HUB_REGION: Region = Region::EAST_US_2;
pub const SATELLITE_REGION: Region = Region::WEST_US_3;
pub const DATABASE_NAME_ENV_VAR: &str = "DATABASE_NAME";
pub const EMULATOR_HOST: &str = "127.0.0.1";
/// Asserts that the operation contacted `expected_region` at least once and
/// that more than one request was tracked (i.e. some form of retry or
/// failover happened). Does **not** require the *final* request to land on
/// `expected_region` — the driver may probe an alternate region and
/// successfully retry on the original. Used for failover scenarios where
/// either landing is valid.
pub fn assert_region_contacted_with_retry(
    diagnostics: &azure_data_cosmos::diagnostics::DiagnosticsContext,
    expected_region: &Region,
) {
    assert!(
        diagnostics.request_count() > 1,
        "expected multiple requests indicating retry/failover, got {} (regions contacted: {:?})",
        diagnostics.request_count(),
        diagnostics.regions_contacted()
    );
    assert!(
        diagnostics.regions_contacted().contains(expected_region),
        "expected at least one tracked request on region {:?}, but only contacted {:?}",
        expected_region,
        diagnostics.regions_contacted()
    );
}

/// Asserts that local retry was attempted on `expected_region` before any
/// cross-region failover: at least one tracked request must have landed on
/// the expected region. Used to validate scenarios where a transient fault
/// is exercised via local retry; this does **not** require the operation to
/// stay on `expected_region` (the driver may still fail over to an alternate
/// region after exhausting its local retry budget).
pub fn assert_local_retry_attempted_on_region(
    diagnostics: &azure_data_cosmos::diagnostics::DiagnosticsContext,
    expected_region: &Region,
) {
    let requests = diagnostics.requests();
    let on_region = requests
        .iter()
        .filter(|r| r.region() == Some(expected_region))
        .count();
    assert!(
        on_region >= 1,
        "expected at least one tracked request on region {:?}, but none did (regions contacted: {:?})",
        expected_region,
        diagnostics.regions_contacted()
    );
}

/// Asserts an operation never contacted `excluded_region`, even while other regions fail.
pub fn assert_region_not_contacted(
    diagnostics: &azure_data_cosmos::diagnostics::DiagnosticsContext,
    excluded_region: &Region,
) {
    let requests = diagnostics.requests();
    let on_region = requests
        .iter()
        .filter(|r| r.region() == Some(excluded_region))
        .count();
    assert_eq!(
        on_region, 0,
        "expected zero tracked requests on excluded region {:?}, but {} of {} requests landed there (regions contacted: {:?})",
        excluded_region,
        on_region,
        diagnostics.request_count(),
        diagnostics.regions_contacted()
    );
}

/// Default timeout for tests (80 seconds).
pub const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(80);
const CONTAINER_READINESS_ATTEMPT_TIMEOUT: Duration = Duration::from_secs(5);
const CONTAINER_READINESS_RETRY_DELAY: Duration = Duration::from_secs(1);
const FAULT_INJECTION_READINESS_MAX_ATTEMPTS: usize = 20;

async fn retry_container_readiness<T, E, F, Fut, TimeoutError>(
    region: &str,
    attempt_timeout: Duration,
    retry_delay: Duration,
    max_attempts: Option<usize>,
    timeout_error: TimeoutError,
    mut probe: F,
) -> Result<T, E>
where
    E: std::fmt::Display,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    TimeoutError: Fn(&str, usize) -> E,
{
    let mut attempts = 0;
    let mut last_error = None;
    loop {
        attempts += 1;
        match tokio::time::timeout(attempt_timeout, probe()).await {
            Ok(Ok(value)) => return Ok(value),
            Ok(Err(error)) => {
                if max_attempts.is_some_and(|limit| attempts >= limit) {
                    return Err(error);
                }
                println!("waiting for container to be ready in {region}: {error}");
                last_error = Some(error);
            }
            Err(_) => {
                if max_attempts.is_some_and(|limit| attempts >= limit) {
                    return Err(last_error.unwrap_or_else(|| timeout_error(region, attempts)));
                }
                println!("container readiness probe timed out in {region}");
            }
        }
        tokio::time::sleep(retry_delay).await;
    }
}

fn container_readiness_timeout_error(region: &str, attempts: usize) -> CosmosError {
    azure_data_cosmos_driver::error::CosmosError::builder()
        .with_status(CosmosStatus::new(StatusCode::RequestTimeout))
        .with_message(format!(
            "container readiness probe timed out in {region} after {attempts} attempts"
        ))
        .build()
        .into()
}

/// Options for configuring test execution.
#[derive(Default)]
pub struct TestOptions {
    /// Application region for the normal (non-fault) client.
    pub client_application_region: Option<Region>,
    /// Fault injection rules for the fault injection client.
    ///
    /// Setting this to `Some(rules)` — even an empty `Vec` — provisions a
    /// dedicated fault-injection [`CosmosClient`] alongside the regular
    /// test client; the rules are forwarded to the driver runtime after
    /// transport setup (e.g., invalid-certificate acceptance) so that the
    /// driver's `FaultClient` wraps the correct inner HTTP client.
    /// `None` (the default) means no fault-injection client is created.
    pub fault_injection_rules: Option<Vec<std::sync::Arc<FaultInjectionRule>>>,
    /// Application region for the fault injection client.
    /// Used in combination with `fault_injection_rules`.
    pub fault_client_application_region: Option<Region>,
    /// Timeout for the test. If None, uses DEFAULT_TEST_TIMEOUT.
    pub timeout: Option<Duration>,
    /// When `true`, builds the underlying [`CosmosClient`]s with a
    /// [`CosmosRuntime`] configured for
    /// [`ServerCertificateValidation::RequiredUnlessEmulator`], so that
    /// requests against the Cosmos DB emulator (which presents a
    /// self-signed certificate) succeed without TLS validation errors.
    ///
    /// This is the signal that an emulator-only test should opt into the
    /// relaxed runtime; tests targeting live accounts must leave it
    /// `false` so that the default `ServerCertificateValidation::Required`
    /// applies.
    pub allow_invalid_certificates: bool,
}

impl TestOptions {
    /// Creates a new TestOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [`TestOptions`] preconfigured for the Cosmos DB emulator:
    /// the underlying [`CosmosClient`] will be built on a [`CosmosRuntime`]
    /// that accepts the emulator's self-signed certificate
    /// (via [`ServerCertificateValidation::RequiredUnlessEmulator`]).
    pub fn for_emulator() -> Self {
        Self::default().with_allow_invalid_certificates(true)
    }

    /// Sets the application region for the normal (non-fault) client.
    pub fn with_client_application_region(mut self, region: Region) -> Self {
        self.client_application_region = Some(region);
        self
    }

    /// Sets the fault injection rules for the fault injection client.
    ///
    /// The rules will be applied after transport setup so the `FaultClient`
    /// properly wraps the configured HTTP client (e.g., one that accepts
    /// invalid certificates). Passing an empty `Vec` still provisions the
    /// fault-injection client — useful for tests that exercise the
    /// "no rules configured" path.
    pub fn with_fault_injection_rules(
        mut self,
        rules: Vec<std::sync::Arc<FaultInjectionRule>>,
    ) -> Self {
        self.fault_injection_rules = Some(rules);
        self
    }

    /// Sets the application region for the fault injection client.
    pub fn with_fault_client_application_region(mut self, region: Region) -> Self {
        self.fault_client_application_region = Some(region);
        self
    }

    /// Sets the timeout for the test.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Opts the underlying [`CosmosClient`]s into a [`CosmosRuntime`] that
    /// accepts the emulator's self-signed certificate
    /// (via [`ServerCertificateValidation::RequiredUnlessEmulator`]).
    ///
    /// Set this to `true` for emulator-only integration tests; leave it
    /// `false` (the default) for tests targeting a live Cosmos DB account.
    pub fn with_allow_invalid_certificates(mut self, allow: bool) -> Self {
        self.allow_invalid_certificates = allow;
        self
    }
}

static IS_AZURE_PIPELINES: OnceLock<bool> = OnceLock::new();

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum CosmosTestMode {
    /// Tests are enabled and will fail if the env vars are not set
    Required,

    /// Tests are disabled and will not attempt to run.
    Skipped,

    /// Tests can run if the env vars are set, but will not fail if they are not.
    Allowed,
}

const DEFAULT_EMULATOR_DATABASE_NAME: &str = "emulator-test-db";

/// Resolves the connection string from the environment, handling the `"emulator"` shorthand.
pub fn resolve_connection_string() -> Option<ConnectionString> {
    let env_var = std::env::var(CONNECTION_STRING_ENV_VAR).ok()?;
    let raw = if env_var == "emulator" {
        EMULATOR_CONNECTION_STRING
    } else {
        &env_var
    };
    raw.parse().ok()
}

fn get_shared_database_id() -> &'static str {
    static SHARED_DATABASE_ID: OnceLock<String> = OnceLock::new();

    let id = SHARED_DATABASE_ID.get_or_init(|| {
        std::env::var(DATABASE_NAME_ENV_VAR)
            .unwrap_or_else(|_| DEFAULT_EMULATOR_DATABASE_NAME.to_string())
    });

    id.as_str()
}

pub fn get_effective_hub_endpoint() -> String {
    let host = get_global_endpoint();

    if host == EMULATOR_HOST {
        // Return the IP address directly for emulator connections.
        return host;
    }

    // Insert the hub region after the account name, before .documents.azure.com
    // e.g., "account_name.documents.azure.com" -> "account_name-eastus2.documents.azure.com"
    let region_suffix = HUB_REGION.as_str().to_lowercase().replace(' ', "");

    if let Some(pos) = host.find(".documents.azure.com") {
        let account_name = &host[..pos];
        let result = format!("{}-{}.documents.azure.com", account_name, region_suffix);
        result
    } else {
        // Fallback: just return the host as-is if it doesn't match expected format
        host.to_string()
    }
}

pub fn get_global_endpoint() -> String {
    let account_host =
        std::env::var(ACCOUNT_HOST_ENV_VAR).unwrap_or_else(|_| EMULATOR_HOST.to_string());

    let account_endpoint = account_host.trim_end_matches('/');

    // The emulator host is just "127.0.0.1" without a scheme, so return it directly.
    if account_endpoint == EMULATOR_HOST {
        return EMULATOR_HOST.to_string();
    }

    // Parse the URL to extract the host and insert the hub region
    // Expected format: https://accountname.documents.azure.com:443
    // Target format: accountname.documents.azure.com (host only, no scheme/port)
    let url = url::Url::parse(account_endpoint).expect("Failed to parse account endpoint URL");

    let host = url
        .host_str()
        .expect("Failed to get host from account endpoint")
        .to_string();
    host
}

impl FromStr for CosmosTestMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "required" => Ok(CosmosTestMode::Required),
            "skipped" => Ok(CosmosTestMode::Skipped),
            "allowed" => Ok(CosmosTestMode::Allowed),
            _ => Err(()),
        }
    }
}

fn is_azure_pipelines() -> bool {
    *IS_AZURE_PIPELINES.get_or_init(|| std::env::var("SYSTEM_TEAMPROJECTID").is_ok())
}

impl TestClient {
    pub async fn from_env_with_fault_options(
        fault_client_application_region: Option<Region>,
        allow_invalid_certificates: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_env_inner(
            None,
            Vec::new(),
            fault_client_application_region,
            allow_invalid_certificates,
        )
        .await
    }

    pub async fn from_env(
        application_region: Option<Region>,
        allow_invalid_certificates: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_env_inner(
            application_region,
            Vec::new(),
            None,
            allow_invalid_certificates,
        )
        .await
    }

    pub async fn from_env_with_fault_rules(
        fault_rules: Vec<std::sync::Arc<FaultInjectionRule>>,
        application_region: Option<Region>,
        allow_invalid_certificates: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_env_inner(
            None,
            fault_rules,
            application_region,
            allow_invalid_certificates,
        )
        .await
    }

    /// Creates a new [`TestClient`] from local environment variables.
    ///
    /// If the environment variables are not set, this client will contain no underlying [`CosmosClient`].
    /// Calling `run` on such a client will skip running the closure (thus skipping the test), except when
    /// running on Azure Pipelines, when it will panic instead.
    async fn from_env_inner(
        application_region: Option<Region>,
        fault_rules: Vec<std::sync::Arc<FaultInjectionRule>>,
        fault_client_application_region: Option<Region>,
        allow_invalid_certificates: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Ok(env_var) = std::env::var(CONNECTION_STRING_ENV_VAR) else {
            // No connection string provided, so we'll skip tests that require it.
            return Ok(Self {
                cosmos_client: None,
            });
        };

        match env_var.as_ref() {
            "emulator" => {
                if fault_client_application_region.is_some() {
                    eprintln!(
                        "warning: fault_client_application_region is ignored for emulator connections; \
                         the emulator always uses its own transport with invalid-cert acceptance"
                    );
                }
                // Ignore that the test mode says playback, if the user explicitly asked for emulator, we use it.
                Self::from_connection_string(
                    EMULATOR_CONNECTION_STRING,
                    application_region,
                    true,
                    fault_rules,
                    None,
                )
                .await
            }
            _ => {
                Self::from_connection_string(
                    &env_var,
                    application_region,
                    allow_invalid_certificates,
                    fault_rules,
                    fault_client_application_region,
                )
                .await
            }
        }
    }

    async fn from_connection_string(
        connection_string: &str,
        application_region: Option<Region>,
        mut allow_invalid_certificates: bool,
        fault_rules: Vec<std::sync::Arc<FaultInjectionRule>>,
        fault_client_application_region: Option<Region>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let connection_string: ConnectionString = connection_string.parse()?;

        if let Ok(val) = std::env::var(ALLOW_INVALID_CERTS_ENV_VAR) {
            if let Ok(parsed) = val.parse::<bool>() {
                if parsed {
                    // Override to allow invalid certificates
                    allow_invalid_certificates = true;
                }
            }
        }

        let credential = connection_string.account_key().clone();
        let mut builder = azure_data_cosmos::CosmosClient::builder();

        // Determine the region selection strategy
        let region = application_region
            .or(fault_client_application_region)
            .unwrap_or(HUB_REGION);
        let strategy = RoutingStrategy::ProximityTo(region);

        if allow_invalid_certificates {
            let runtime = CosmosRuntime::builder()
                .with_connection_pool(
                    ConnectionPoolOptions::builder()
                        .with_server_certificate_validation(
                            ServerCertificateValidation::RequiredUnlessEmulator,
                        )
                        .build()?,
                )
                .build()
                .await?;
            builder = builder.with_runtime(runtime);
        }

        // Configure fault injection if rules provided
        if !fault_rules.is_empty() {
            builder = builder.with_fault_injection_rules(fault_rules)?;
        }

        let endpoint: azure_data_cosmos::AccountEndpoint =
            connection_string.account_endpoint().parse()?;
        let cosmos_client = builder
            .build(
                azure_data_cosmos::AccountReference::with_authentication_key(endpoint, credential),
                strategy,
            )
            .await?;

        Ok(TestClient {
            cosmos_client: Some(cosmos_client),
        })
    }

    /// Runs a test function with a new [`TestClient`], ensuring proper setup and cleanup of the database.
    pub async fn run<F>(test: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnMut(&TestRunContext) -> Result<(), Box<dyn std::error::Error>>,
    {
        Self::run_with_options(test, TestOptions::new()).await
    }

    /// Runs a test function with a new [`TestClient`] and custom test options.
    ///
    /// This method supports:
    /// - Timeouts (defaults to DEFAULT_TEST_TIMEOUT)
    /// - Custom CosmosClient options for the normal client
    /// - Preferred regions for the fault injection client
    ///
    /// The test function receives a [`TestRunContext`] which provides access to both:
    /// - A normal client via `client()` and `shared_db_client()`
    /// - A fault injection client via `fault_client()` and `fault_db_client()` (if fault injection was configured)
    pub async fn run_with_options<F>(
        mut test: F,
        options: TestOptions,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnMut(&TestRunContext) -> Result<(), Box<dyn std::error::Error>>,
    {
        let test_mode = if let Ok(s) = std::env::var(TEST_MODE_ENV_VAR) {
            CosmosTestMode::from_str(&s).map_err(|_| {
                format!(
                    "Invalid value for {}: {}. Expected 'required', 'skipped', or 'allowed'.",
                    TEST_MODE_ENV_VAR, s
                )
            })?
        } else {
            CosmosTestMode::Allowed
        };

        if test_mode == CosmosTestMode::Skipped {
            println!(
                "Skipping Cosmos DB tests because {} is set to 'skipped'.",
                TEST_MODE_ENV_VAR
            );
            return Ok(());
        }

        // Initialize tracing subscriber for logging, if not already initialized.
        // The error is ignored because it only happens if the subscriber is already initialized.
        _ = tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::builder()
                    // Tests with intentional failures cause noise, so we set the default level to "off"
                    // to silence them unless the user explicitly configures it.
                    .with_default_directive("off".parse().unwrap())
                    .from_env_lossy(),
            )
            .try_init();

        let test_client = Self::from_env(
            options.client_application_region.clone(),
            options.allow_invalid_certificates,
        )
        .await?;

        // Create fault injection client if rules or application region were provided.
        // Rules should be passed in for emulator tests to ensure the FaultClient
        // wraps the HTTP client with invalid cert acceptance,
        // which is required for emulator connectivity.
        // An explicitly-set empty Vec still provisions the fault client (some
        // tests exercise the "no rules configured" path); `None` means
        // `with_fault_injection_rules` was never called.
        let fault_client = if let Some(rules) = options.fault_injection_rules {
            Some(
                Self::from_env_with_fault_rules(
                    rules,
                    options.fault_client_application_region.clone(),
                    options.allow_invalid_certificates,
                )
                .await?,
            )
        } else if options.fault_client_application_region.is_some() {
            Some(
                Self::from_env_with_fault_options(
                    options.fault_client_application_region,
                    options.allow_invalid_certificates,
                )
                .await?,
            )
        } else {
            None
        };

        // CosmosClient is designed to be cloned cheaply, so we can clone it here.
        if let Some(account) = test_client.cosmos_client.clone() {
            let fault_cosmos_client = fault_client.and_then(|fc| fc.cosmos_client);
            let run = TestRunContext::new(account, fault_cosmos_client);

            // Apply timeout around entire test including retries on 429s
            let timeout = options.timeout.unwrap_or(DEFAULT_TEST_TIMEOUT);

            let result = tokio::time::timeout(timeout, async {
                let mut backoff = Duration::from_millis(500);
                const MAX_BACKOFF: Duration = Duration::from_secs(30);

                loop {
                    let test_result = Box::pin(test(&run)).await;

                    if let Err(e) = &test_result {
                        println!("CosmosError running test: {}", e);
                        // Check if the error is a 429
                        let is_429 = e.to_string().contains("TooManyRequests")
                            || e.to_string().contains("Too Many Requests");

                        if is_429 {
                            println!(
                                "Test got 429 (Too Many Requests). Retrying after {:?}...",
                                backoff
                            );
                            tokio::time::sleep(backoff).await;
                            backoff = (backoff * 2).min(MAX_BACKOFF);
                            continue;
                        }
                    }

                    break test_result;
                }
            })
            .await;

            // Always cleanup, even if test timed out
            run.cleanup().await?;

            match result {
                Ok(test_result) => {
                    if let Err(e) = &test_result {
                        if e.downcast_ref::<super::InconclusiveError>().is_some() {
                            // Make it clear to the reader that the failure is an inconclusive one
                            eprintln!(concat!("This test returned an inconclusive result. ",
                                "This does NOT indicate a failure, but rather that the test was unable to complete successfully ",
                                "due to an external factor (e.g. a split not completing in time). ",
                                "Inconclusive results do not need to block PRs unless the PR is specifically touching code related to this test."));
                        }
                    }
                    test_result
                }
                Err(_) => Err(format!("Test timed out after {} seconds", timeout.as_secs()).into()),
            }
        } else if test_mode == CosmosTestMode::Required {
            panic!("Cosmos Test Mode is 'required' but no connection string was provided in the AZURE_COSMOS_CONNECTION_STRING environment variable.");
        } else {
            // Test mode is 'allowed' but no connection string was provided, so we skip the test.
            eprintln!("Skipping emulator/live tests because no connection string was provided in the AZURE_COSMOS_CONNECTION_STRING environment variable.");
            Ok(())
        }
    }

    pub async fn run_with_unique_db<F>(
        mut test: F,
        options: Option<TestOptions>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnMut(&TestRunContext, &DatabaseClient) -> Result<(), Box<dyn std::error::Error>>,
    {
        Self::run_with_options(
            async |run_context| {
                let db_client = run_context.create_db().await?;
                Box::pin(test(run_context, &db_client)).await
            },
            options.unwrap_or_default(),
        )
        .await
    }

    pub async fn run_with_shared_db<F>(
        mut test: F,
        options: Option<TestOptions>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnMut(&TestRunContext, &DatabaseClient) -> Result<(), Box<dyn std::error::Error>>,
    {
        Self::run_with_options(
            async |run_context| {
                // Ensure the shared database exists (create if needed, ignore conflict).
                let db_id = get_shared_database_id();
                // Emulator is always strong consistency, so we can skip the read check in that case
                match run_context.client().create_database(db_id, None).await {
                    Ok(_) => {}
                    Err(e) if e.status().status_code() == StatusCode::Conflict => {}
                    Err(e) => return Err(e.into()),
                }
                let db_client = run_context.shared_db_client();
                db_client.read(None).await?;
                Box::pin(test(run_context, &db_client)).await
            },
            options.unwrap_or_default(),
        )
        .await
    }
}

/// Context for a test run, providing access to both normal and fault injection clients.
///
/// The normal client is always available via `client()` and `shared_db_client()`.
/// The fault injection client is available via `fault_client()` and `fault_db_client()`
/// if `TestOptions::with_fault_injection_rules()` was called
/// or if `TestOptions::with_fault_client_application_region()` was called.
pub struct TestRunContext {
    run_id: String,
    /// The normal (non-fault) Cosmos client.
    client: CosmosClient,
    /// The fault injection Cosmos client (if configured).
    fault_client: Option<CosmosClient>,
}

impl TestRunContext {
    pub fn new(client: CosmosClient, fault_client: Option<CosmosClient>) -> Self {
        let run_id = azure_core::Uuid::new_v4().simple().to_string();
        Self {
            run_id,
            client,
            fault_client,
        }
    }

    /// Generates a unique database ID including the [`TestRunContext::run_id`].
    ///
    /// This database will be automatically deleted when [`TestRunContext::cleanup`] is called (which will happen automatically if [`TestClient::run`] is used).
    pub fn db_name(&self) -> String {
        format!("auto-test-{}", self.run_id)
    }

    /// Gets the underlying normal (non-fault) [`CosmosClient`].
    pub fn client(&self) -> &CosmosClient {
        &self.client
    }

    /// Gets the fault injection [`CosmosClient`], if configured.
    ///
    /// Returns `Some(&CosmosClient)` if `TestOptions::with_fault_injection_rules()` or
    /// if `TestOptions::with_fault_client_application_region()` was called,
    /// otherwise returns `None`.
    pub fn fault_client(&self) -> Option<&CosmosClient> {
        self.fault_client.as_ref()
    }

    /// Gets the shared database client using the normal (non-fault) client.
    pub fn shared_db_client(&self) -> DatabaseClient {
        self.client().database_client(get_shared_database_id())
    }

    /// Gets the shared database client using the fault injection client.
    ///
    /// Returns `Some(DatabaseClient)` if `TestOptions::with_fault_injection_rules()` or
    /// if `TestOptions::with_fault_client_application_region()` was called,
    /// otherwise returns `None`.
    pub fn fault_db_client(&self) -> Option<DatabaseClient> {
        self.fault_client()
            .map(|c| c.database_client(get_shared_database_id()))
    }

    /// Creates a new, empty, database for this test run with default throughput options.
    pub async fn create_db(&self) -> azure_data_cosmos::Result<DatabaseClient> {
        // The TestAccount has a unique context_id that includes the test name.
        let db_name = self.db_name();
        let response = match self.client().create_database(&db_name, None).await {
            // The database creation was successful.
            Ok(props) => props,
            Err(e) if e.status().status_code() == StatusCode::Conflict => {
                // The database already exists, from a previous test run.
                // Delete it and re-create it.
                let db_client = self.client().database_client(&db_name);
                db_client.delete(None).await?;

                // Re-create the database.
                self.client().create_database(&db_name, None).await?
            }
            Err(e) => {
                // Some other error occurred.
                return Err(e);
            }
        };

        let props = response.into_model()?;

        let id = props
            .id
            .as_deref()
            .expect("Cosmos DB should always return a database id on create");
        let db_client = self.client().database_client(id);
        Ok(db_client)
    }

    /// Reads an item from the specified container with exponential backoff retries on 404 errors.
    /// This is useful for tests where eventual consistency may cause transient read failures.
    pub async fn read_item(
        &self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemReadOptions>,
    ) -> azure_data_cosmos::Result<ItemResponse> {
        // Own the inputs so no borrowed data must live across `.await`.
        let partition_key = partition_key.into().to_owned();
        let item_id = item_id.to_owned();
        let mut backoff = Duration::from_millis(100);
        const MAX_BACKOFF: Duration = Duration::from_secs(10);

        loop {
            match container
                .read_item(
                    partition_key.clone(),
                    item_id.clone().as_str(),
                    options.clone(),
                )
                .await
            {
                Ok(response) => return Ok(response),
                Err(e) if e.status().status_code() == StatusCode::NotFound => {
                    println!(
                        "Read item failed with {:?}: {}. Retrying after {:?}...",
                        e.status().status_code(),
                        e,
                        backoff
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Queries items from the specified container with exponential backoff retries on 404 errors.
    /// This is useful for tests where eventual consistency may cause transient query failures.
    pub async fn query_items<T>(
        &self,
        container: &ContainerClient,
        query: impl Into<Query>,
        partition_key: impl Into<PartitionKey>,
    ) -> azure_data_cosmos::Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + std::marker::Send + 'static,
    {
        let query = query.into();
        let partition_key = partition_key.into().to_owned();
        let mut backoff = Duration::from_millis(100);
        const MAX_BACKOFF: Duration = Duration::from_secs(10);

        loop {
            match container
                .query_items::<T>(
                    query.clone(),
                    FeedScope::partition(partition_key.clone()),
                    None,
                )
                .await
            {
                Ok(pager) => match pager.try_collect::<Vec<T>>().await {
                    Ok(items) => return Ok(items),
                    Err(e) if e.status().status_code() == StatusCode::NotFound => {
                        println!(
                            "Query items failed with {:?}: {}. Retrying after {:?}...",
                            e.status().status_code(),
                            e,
                            backoff
                        );
                        tokio::time::sleep(backoff).await;
                        backoff = (backoff * 2).min(MAX_BACKOFF);
                    }
                    Err(e) => return Err(e),
                },
                Err(e) if e.status().status_code() == StatusCode::NotFound => {
                    println!(
                        "Query items failed with {:?}: {}. Retrying after {:?}...",
                        e.status().status_code(),
                        e,
                        backoff
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Creates a container with exponential backoff retries on 429 (Too Many Requests) errors.
    /// This is useful for tests where rate limiting may cause transient failures.
    pub async fn create_container(
        &self,
        db_client: &DatabaseClient,
        properties: azure_data_cosmos::models::ContainerProperties,
        options: Option<azure_data_cosmos::options::CreateContainerOptions>,
    ) -> azure_data_cosmos::Result<ContainerClient> {
        let mut backoff = Duration::from_millis(100);
        const MAX_BACKOFF: Duration = Duration::from_secs(10);

        loop {
            match db_client
                .create_container(properties.clone(), options.clone())
                .await
            {
                Ok(response) => {
                    let created = response.into_model()?;
                    return db_client.container_client(&created.id).await;
                }
                Err(e) if e.status().status_code() == StatusCode::TooManyRequests => {
                    println!(
                        "Create container got 429 (Too Many Requests). Retrying after {:?}...",
                        backoff
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                }
                Err(e) if e.status().status_code() == StatusCode::Conflict => {
                    // Container already exists, delete and recreate it, then return a client
                    let container_client = db_client.container_client(&properties.id).await?;
                    container_client.delete(None).await?;

                    // recreate
                    let response = db_client
                        .create_container(properties.clone(), options.clone())
                        .await?;
                    let created = response.into_model()?;
                    return db_client.container_client(&created.id).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Creates a container and waits until both the normal and fault-injection
    /// clients can read it in their configured region.
    pub fn create_container_for_fault_injection<'a>(
        &'a self,
        db_client: &'a DatabaseClient,
        properties: azure_data_cosmos::models::ContainerProperties,
        throughput: ThroughputProperties,
    ) -> Pin<Box<dyn Future<Output = azure_data_cosmos::Result<ContainerClient>> + Send + 'a>> {
        let fault_client = self
            .fault_client
            .clone()
            .expect("fault-injection client must be configured");

        Box::pin(async move {
            let created = db_client
                .create_container(
                    properties,
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?
                .into_model()?;
            let container_id = created.id;

            let original_db_client = db_client;
            let original_container_id = container_id.clone();
            let original_readiness = retry_container_readiness(
                "original client",
                CONTAINER_READINESS_ATTEMPT_TIMEOUT,
                CONTAINER_READINESS_RETRY_DELAY,
                Some(FAULT_INJECTION_READINESS_MAX_ATTEMPTS),
                container_readiness_timeout_error,
                move || {
                    let db_client = original_db_client;
                    let container_id = original_container_id.clone();
                    async move {
                        let container = db_client.container_client(&container_id).await?;
                        container.read(None).await?;
                        Ok::<_, azure_data_cosmos::CosmosError>(container)
                    }
                },
            );

            let fault_db_id = db_client.id().to_owned();
            let fault_container_id = container_id;
            let fault_readiness = retry_container_readiness(
                "fault-injection client",
                CONTAINER_READINESS_ATTEMPT_TIMEOUT,
                CONTAINER_READINESS_RETRY_DELAY,
                Some(FAULT_INJECTION_READINESS_MAX_ATTEMPTS),
                container_readiness_timeout_error,
                move || {
                    let fault_client = fault_client.clone();
                    let db_id = fault_db_id.clone();
                    let container_id = fault_container_id.clone();
                    async move {
                        let container = fault_client
                            .database_client(&db_id)
                            .container_client(&container_id)
                            .await?;
                        container.read(None).await?;
                        Ok::<_, azure_data_cosmos::CosmosError>(container)
                    }
                },
            );

            let (container, _) = tokio::try_join!(original_readiness, fault_readiness)?;
            Ok(container)
        })
    }

    /// Creates a container with specified throughput and waits for it to be fully created.
    ///
    /// This method:
    /// 1. Creates the container with the specified properties and throughput
    /// 2. Creates two clients with preferred regions (hub and satellite)
    /// 3. Polls until both clients can successfully read the container
    /// 4. Returns a [`ContainerClient`] for the created container
    ///
    /// This is useful for tests that need to ensure the container is fully available
    /// in multiple regions before performing operations on it.
    pub fn create_container_with_throughput<'a>(
        &'a self,
        db_client: &'a DatabaseClient,
        properties: azure_data_cosmos::models::ContainerProperties,
        throughput: ThroughputProperties,
    ) -> Pin<Box<dyn Future<Output = azure_data_cosmos::Result<ContainerClient>> + Send + 'a>> {
        Box::pin(async move {
            let created_properties = db_client
                .create_container(
                    properties,
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?
                .into_model()?;

            // Create two clients with different preferred regions to ensure container is available in both
            let hub_client = Self::create_client_with_preferred_region(HUB_REGION).await?;
            let satellite_client =
                Self::create_client_with_preferred_region(SATELLITE_REGION).await?;

            let container_id = &created_properties.id;

            let db_id = db_client.id().to_owned();

            let hub_probe_client = hub_client.clone();
            let hub_db_id = db_id.clone();
            let hub_container_id = container_id.clone();
            retry_container_readiness(
                HUB_REGION.as_str(),
                CONTAINER_READINESS_ATTEMPT_TIMEOUT,
                CONTAINER_READINESS_RETRY_DELAY,
                None,
                container_readiness_timeout_error,
                move || {
                    let client = hub_probe_client.clone();
                    let db_id = hub_db_id.clone();
                    let container_id = hub_container_id.clone();
                    async move {
                        let container = client
                            .database_client(&db_id)
                            .container_client(&container_id)
                            .await?;
                        container.read(None).await?;
                        Ok::<_, azure_data_cosmos::CosmosError>(container)
                    }
                },
            )
            .await?;

            let satellite_probe_client = satellite_client.clone();
            let satellite_db_id = db_id.clone();
            let satellite_container_id = container_id.clone();
            retry_container_readiness(
                SATELLITE_REGION.as_str(),
                CONTAINER_READINESS_ATTEMPT_TIMEOUT,
                CONTAINER_READINESS_RETRY_DELAY,
                None,
                container_readiness_timeout_error,
                move || {
                    let client = satellite_probe_client.clone();
                    let db_id = satellite_db_id.clone();
                    let container_id = satellite_container_id.clone();
                    async move {
                        let container = client
                            .database_client(&db_id)
                            .container_client(&container_id)
                            .await?;
                        container.read(None).await?;
                        Ok::<_, azure_data_cosmos::CosmosError>(container)
                    }
                },
            )
            .await?;

            let original_db_client = db_client;
            let original_container_id = container_id.clone();
            retry_container_readiness(
                "original client",
                CONTAINER_READINESS_ATTEMPT_TIMEOUT,
                CONTAINER_READINESS_RETRY_DELAY,
                Some(30),
                container_readiness_timeout_error,
                move || {
                    let db_client = original_db_client;
                    let container_id = original_container_id.clone();
                    async move {
                        let container = db_client.container_client(&container_id).await?;
                        container.read(None).await?;
                        Ok::<_, azure_data_cosmos::CosmosError>(container)
                    }
                },
            )
            .await
        })
    }

    /// Creates a CosmosClient with a specific preferred region.
    async fn create_client_with_preferred_region(
        region: Region,
    ) -> Result<CosmosClient, azure_data_cosmos::CosmosError> {
        let env_var = std::env::var(CONNECTION_STRING_ENV_VAR)
            .unwrap_or_else(|_| EMULATOR_CONNECTION_STRING.to_string());

        let connection_string = if env_var == "emulator" {
            EMULATOR_CONNECTION_STRING
        } else {
            &env_var
        };

        let parsed: ConnectionString = connection_string.parse()?;

        let endpoint: azure_data_cosmos::AccountEndpoint = parsed.account_endpoint().parse()?;
        let builder = CosmosClient::builder().with_runtime(
            CosmosRuntime::builder()
                .with_connection_pool(
                    ConnectionPoolOptions::builder()
                        .with_server_certificate_validation(
                            ServerCertificateValidation::RequiredUnlessEmulator,
                        )
                        .build()?,
                )
                .build()
                .await?,
        );

        builder
            .build(
                azure_data_cosmos::AccountReference::with_authentication_key(
                    endpoint,
                    parsed.account_key().clone(),
                ),
                RoutingStrategy::ProximityTo(region),
            )
            .await
    }

    /// Builds a [`CosmosClient`] authenticated with an Entra ID (AAD) token
    /// credential, targeting the same account the key client uses.
    ///
    /// This is the entry point for AAD integration tests. The returned client
    /// performs data-plane operations under AAD; database/container management
    /// must still go through the key client (`client()`), because the
    /// data-plane RBAC role granted in `test-resources.bicep` does not permit
    /// management-plane operations.
    ///
    /// See [`build_aad_client_from_env`] for credential-selection details.
    pub async fn aad_client(
        &self,
    ) -> Result<(CosmosClient, Option<super::CredentialRecorder>), Box<dyn std::error::Error>> {
        build_aad_client_from_env(HUB_REGION).await
    }

    /// Cleans up test resources.
    ///
    /// This should be called at the end of a test run to delete any databases created during the test.
    /// If using [`TestClient::run`], this will be called automatically.
    pub async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::from(format!(
            "SELECT * FROM root r WHERE r.id LIKE 'auto-test-{}'",
            self.run_id
        ));
        let mut pager = self.client().query_databases(query, None).await?;
        let mut ids = Vec::new();
        while let Some(db) = pager.try_next().await? {
            if let Some(id) = db.id {
                ids.push(id);
            }
        }

        // Now that we have a list of databases created by this test, we delete them.
        // We COULD choose not to delete them and instead validate that they were deleted, but this is what I've gone with for now.
        for id in ids {
            println!("Deleting left-over database: {}", &id);
            self.client().database_client(&id).delete(None).await?;
        }
        Ok(())
    }
}

/// Returns `true` if `endpoint`'s host is a loopback/local host, indicating the
/// Cosmos DB emulator rather than a live account.
fn host_is_local(endpoint: &str) -> bool {
    match url::Url::parse(endpoint) {
        Ok(url) => matches!(
            url.host_str(),
            Some("127.0.0.1") | Some("localhost") | Some("::1") | Some("[::1]")
        ),
        Err(_) => false,
    }
}

/// Builds a [`CosmosClient`] authenticated with an Entra ID (AAD) token
/// credential, reading the target account from the same environment the
/// key-auth client uses (`AZURE_COSMOS_CONNECTION_STRING`).
///
/// Credential selection is based on the target host:
/// - **Emulator** (`AZURE_COSMOS_CONNECTION_STRING=emulator`, or an endpoint
///   whose host is `127.0.0.1`/`localhost`): uses [`CosmosEmulatorCredential`],
///   which mints the emulator's fake JWT signed with the account's master key,
///   and enables invalid-certificate acceptance. A [`CredentialRecorder`] is
///   returned so tests can assert the AAD path (not key auth) was exercised.
/// - **Live account**: uses `azure_core_test::credentials::from_env`, which
///   resolves to `AzurePipelinesCredential` in CI (matching the principal the
///   bicep grants the data-plane RBAC role to) and `DeveloperToolsCredential`
///   locally. No recorder is returned in this case.
pub async fn build_aad_client_from_env(
    region: Region,
) -> Result<(CosmosClient, Option<super::CredentialRecorder>), Box<dyn std::error::Error>> {
    use super::CosmosEmulatorCredential;

    let env_var = std::env::var(CONNECTION_STRING_ENV_VAR)?;
    let is_emulator_shorthand = env_var == "emulator";
    let connection_string_str = if is_emulator_shorthand {
        EMULATOR_CONNECTION_STRING
    } else {
        env_var.as_str()
    };

    let parsed: ConnectionString = connection_string_str.parse()?;
    let endpoint_str = parsed.account_endpoint().to_string();
    let endpoint: azure_data_cosmos::AccountEndpoint = endpoint_str.parse()?;

    let is_emulator = is_emulator_shorthand || host_is_local(&endpoint_str);

    let mut builder = CosmosClient::builder();
    let strategy = RoutingStrategy::ProximityTo(region);

    let (credential, recorder): (
        std::sync::Arc<dyn azure_core::credentials::TokenCredential>,
        Option<super::CredentialRecorder>,
    ) = if is_emulator {
        // The emulator serves a self-signed certificate, so route the client
        // through a runtime that skips certificate validation for emulator
        // hosts (mirroring the key-auth emulator client setup).
        let runtime = CosmosRuntime::builder()
            .with_connection_pool(
                ConnectionPoolOptions::builder()
                    .with_server_certificate_validation(
                        ServerCertificateValidation::RequiredUnlessEmulator,
                    )
                    .build()?,
            )
            .build()
            .await?;
        builder = builder.with_runtime(runtime);

        // Sign the fake JWT with the same master key the emulator validates against.
        let master_key = parsed.account_key().secret().to_string();
        let credential = std::sync::Arc::new(CosmosEmulatorCredential::with_master_key(master_key));
        let recorder = credential.recorder();
        (credential, Some(recorder))
    } else {
        (azure_core_test::credentials::from_env(None)?, None)
    };

    let account = azure_data_cosmos::AccountReference::with_credential(endpoint, credential);
    let client = builder.build(account, strategy).await?;
    Ok((client, recorder))
}

#[cfg(test)]
mod tests {
    use super::retry_container_readiness;
    use std::{
        future::pending,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::Duration,
    };

    #[tokio::test]
    async fn container_readiness_retries_timeout_and_error() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let count = attempts.clone();

        let result = retry_container_readiness(
            "test-region",
            Duration::from_millis(10),
            Duration::ZERO,
            None,
            |_, _| "timed out",
            move || {
                let count = count.clone();
                async move {
                    match count.fetch_add(1, Ordering::SeqCst) {
                        0 => pending::<Result<usize, &'static str>>().await,
                        1 => Err("not ready"),
                        _ => Ok(42),
                    }
                }
            },
        )
        .await
        .unwrap();

        assert_eq!(result, 42);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn container_readiness_returns_last_error_after_attempt_limit() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let count = attempts.clone();

        let error = retry_container_readiness(
            "test-region",
            Duration::from_millis(10),
            Duration::ZERO,
            Some(2),
            |_, _| "timed out",
            move || {
                let count = count.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<(), _>("permanent failure")
                }
            },
        )
        .await
        .unwrap_err();

        assert_eq!(error, "permanent failure");
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn container_readiness_bounds_timeout_only_failures() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let count = attempts.clone();

        let error = retry_container_readiness(
            "test-region",
            Duration::from_millis(10),
            Duration::ZERO,
            Some(2),
            |_, attempts| match attempts {
                2 => "timed out after two attempts",
                _ => unreachable!(),
            },
            move || {
                let count = count.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    pending::<Result<(), &'static str>>().await
                }
            },
        )
        .await
        .unwrap_err();

        assert_eq!(error, "timed out after two attempts");
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }
}
