// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore: TEAMPROJECTID

#![cfg_attr(not(feature = "key_auth"), allow(dead_code))]
#![cfg(feature = "fault_injection")]

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::fault_injection::FaultInjectionClientBuilder;
use azure_data_cosmos::models::{CosmosResponse, ThroughputProperties};
use azure_data_cosmos::options::ItemOptions;
use azure_data_cosmos::regions::{RegionName, EAST_US_2, WEST_US_3};
use azure_data_cosmos::{
    clients::DatabaseClient, ConnectionString, CosmosClient, CreateContainerOptions, PartitionKey,
    Query,
};
use futures::TryStreamExt;
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
pub const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://localhost:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";
pub const HUB_REGION: RegionName = EAST_US_2;
pub const SATELLITE_REGION: RegionName = WEST_US_3;
pub const DATABASE_NAME_ENV_VAR: &str = "DATABASE_NAME";
pub const EMULATOR_HOST: &str = "localhost";

/// Default timeout for tests (80 seconds).
pub const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(80);

/// Options for configuring test execution.
#[derive(Default)]
pub struct TestOptions {
    /// Preferred regions for the normal (non-fault) client.
    pub client_preferred_regions: Vec<RegionName>,
    /// Fault injection builder for the fault injection client.
    /// If provided, a separate client will be created with fault injection capabilities.
    /// The builder is applied after transport setup (e.g., invalid certificate acceptance)
    /// so that the FaultClient wraps the correct inner HTTP client.
    pub fault_injection_builder: Option<FaultInjectionClientBuilder>,
    /// Preferred regions for the fault injection client.
    /// Used in combination with `fault_injection_builder`.
    pub fault_client_preferred_regions: Vec<RegionName>,
    /// Timeout for the test. If None, uses DEFAULT_TEST_TIMEOUT.
    pub timeout: Option<Duration>,
}

impl TestOptions {
    /// Creates a new TestOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the preferred regions for the normal (non-fault) client.
    pub fn with_client_preferred_regions(mut self, regions: Vec<RegionName>) -> Self {
        self.client_preferred_regions = regions;
        self
    }

    /// Sets the fault injection builder for the fault injection client.
    /// The builder will be applied after transport setup so the FaultClient
    /// properly wraps the configured HTTP client (e.g., one that accepts invalid certificates).
    pub fn with_fault_injection_builder(mut self, builder: FaultInjectionClientBuilder) -> Self {
        self.fault_injection_builder = Some(builder);
        self
    }

    /// Sets the preferred regions for the fault injection client.
    pub fn with_fault_client_preferred_regions(mut self, regions: Vec<RegionName>) -> Self {
        self.fault_client_preferred_regions = regions;
        self
    }

    /// Sets the timeout for the test.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
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
        // The SDK resolves "localhost" to "127.0.0.1" in request URLs.
        return "127.0.0.1".to_string();
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

    // The emulator host is just "localhost" without a scheme, so return it directly.
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
        fault_client_preferred_regions: Vec<RegionName>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_env_inner(Vec::new(), None, fault_client_preferred_regions).await
    }

    pub async fn from_env(
        preferred_regions: Vec<RegionName>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_env_inner(preferred_regions, None, Vec::new()).await
    }

    pub async fn from_env_with_fault_builder(
        fault_builder: FaultInjectionClientBuilder,
        preferred_regions: Vec<RegionName>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_env_inner(preferred_regions, Some(fault_builder), Vec::new()).await
    }

    /// Creates a new [`TestClient`] from local environment variables.
    ///
    /// If the environment variables are not set, this client will contain no underlying [`CosmosClient`].
    /// Calling `run` on such a client will skip running the closure (thus skipping the test), except when
    /// running on Azure Pipelines, when it will panic instead.
    async fn from_env_inner(
        preferred_regions: Vec<RegionName>,
        fault_builder: Option<FaultInjectionClientBuilder>,
        fault_client_preferred_regions: Vec<RegionName>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Ok(env_var) = std::env::var(CONNECTION_STRING_ENV_VAR) else {
            // No connection string provided, so we'll skip tests that require it.
            return Ok(Self {
                cosmos_client: None,
            });
        };

        match env_var.as_ref() {
            "emulator" => {
                if !fault_client_preferred_regions.is_empty() {
                    eprintln!(
                        "warning: fault_client_preferred_regions are ignored for emulator connections; \
                         the emulator always uses its own transport with invalid-cert acceptance"
                    );
                }
                // Ignore that the test mode says playback, if the user explicitly asked for emulator, we use it.
                Self::from_connection_string(
                    EMULATOR_CONNECTION_STRING,
                    preferred_regions,
                    true,
                    fault_builder,
                    Vec::new(),
                )
                .await
            }
            _ => {
                Self::from_connection_string(
                    &env_var,
                    preferred_regions,
                    false,
                    fault_builder,
                    fault_client_preferred_regions,
                )
                .await
            }
        }
    }

    async fn from_connection_string(
        connection_string: &str,
        preferred_regions: Vec<RegionName>,
        mut allow_invalid_certificates: bool,
        fault_builder: Option<FaultInjectionClientBuilder>,
        fault_client_preferred_regions: Vec<RegionName>,
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

        let credential = connection_string.account_key.clone();
        let mut builder = azure_data_cosmos::CosmosClient::builder();

        // Apply preferred regions for the client
        if !preferred_regions.is_empty() {
            builder = builder.with_application_preferred_regions(preferred_regions);
        }

        // Configure invalid certificate acceptance (e.g., for emulator)
        #[cfg(feature = "allow_invalid_certificates")]
        if allow_invalid_certificates {
            builder = builder.with_allow_emulator_invalid_certificates(true);
        }
        #[cfg(not(feature = "allow_invalid_certificates"))]
        if allow_invalid_certificates {
            return Err(
                "The 'allow_invalid_certificates' feature must be enabled to accept invalid certificates. \
                 Add `allow_invalid_certificates` to the features list."
                    .into(),
            );
        }

        // Configure fault injection if builder provided
        if let Some(fault_builder) = fault_builder {
            builder = builder.with_fault_injection(fault_builder);
        }

        // Apply fault client preferred regions
        if !fault_client_preferred_regions.is_empty() {
            builder = builder.with_application_preferred_regions(fault_client_preferred_regions);
        }

        let endpoint: azure_data_cosmos::CosmosAccountEndpoint =
            connection_string.account_endpoint.parse()?;
        let cosmos_client = builder
            .build(azure_data_cosmos::CosmosAccountReference::with_master_key(
                endpoint, credential,
            ))
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
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        let test_client = Self::from_env(options.client_preferred_regions.clone()).await?;

        // Create fault injection client if builder or preferred regions were provided
        // builder should be passed in for emulator tests to ensure the FaultClient
        // wraps the HTTP client with invalid cert acceptance,
        // which is required for emulator connectivity
        let fault_client = if let Some(builder) = options.fault_injection_builder {
            Some(
                Self::from_env_with_fault_builder(
                    builder,
                    options.fault_client_preferred_regions.clone(),
                )
                .await?,
            )
        } else if !options.fault_client_preferred_regions.is_empty() {
            Some(Self::from_env_with_fault_options(options.fault_client_preferred_regions).await?)
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
                    let test_result = test(&run).await;

                    if let Err(e) = &test_result {
                        println!("Error running test: {}", e);
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
                Ok(test_result) => test_result,
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
                test(run_context, &db_client).await
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
                    Err(e) if e.http_status() == Some(StatusCode::Conflict) => {}
                    Err(e) => return Err(e.into()),
                }
                let db_client = run_context.shared_db_client();
                db_client.read(None).await?;
                test(run_context, &db_client).await
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
/// if `TestOptions::with_fault_injection_builder()` was called
/// or if `TestOptions::with_fault_client_preferred_regions()` was called.
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
    /// Returns `Some(&CosmosClient)` if `TestOptions::with_fault_injection_builder()` or
    /// if `TestOptions::with_fault_client_preferred_regions()` was called,
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
    /// Returns `Some(DatabaseClient)` if `TestOptions::with_fault_injection_builder()` or
    /// if `TestOptions::with_fault_client_preferred_regions()` was called,
    /// otherwise returns `None`.
    pub fn fault_db_client(&self) -> Option<DatabaseClient> {
        self.fault_client()
            .map(|c| c.database_client(get_shared_database_id()))
    }

    /// Creates a new, empty, database for this test run with default throughput options.
    pub async fn create_db(&self) -> azure_core::Result<DatabaseClient> {
        // The TestAccount has a unique context_id that includes the test name.
        let db_name = self.db_name();
        let response = match self.client().create_database(&db_name, None).await {
            // The database creation was successful.
            Ok(props) => props,
            Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
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

        let db_client = self.client().database_client(&props.id);
        Ok(db_client)
    }

    /// Reads an item from the specified container with exponential backoff retries on 404 errors.
    /// This is useful for tests where eventual consistency may cause transient read failures.
    pub async fn read_item<T>(
        &self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemOptions>,
    ) -> azure_core::Result<CosmosResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
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
                Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
                    println!(
                        "Read item failed with {:?}: {}. Retrying after {:?}...",
                        e.http_status(),
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
    ) -> azure_core::Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + std::marker::Send + 'static,
    {
        let query = query.into();
        let partition_key = partition_key.into().to_owned();
        let mut backoff = Duration::from_millis(100);
        const MAX_BACKOFF: Duration = Duration::from_secs(10);

        loop {
            match container.query_items::<T>(query.clone(), partition_key.clone(), None) {
                Ok(pager) => match pager.try_collect::<Vec<T>>().await {
                    Ok(items) => return Ok(items),
                    Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
                        println!(
                            "Query items failed with {:?}: {}. Retrying after {:?}...",
                            e.http_status(),
                            e,
                            backoff
                        );
                        tokio::time::sleep(backoff).await;
                        backoff = (backoff * 2).min(MAX_BACKOFF);
                    }
                    Err(e) => return Err(e),
                },
                Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
                    println!(
                        "Query items failed with {:?}: {}. Retrying after {:?}...",
                        e.http_status(),
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
        options: Option<azure_data_cosmos::CreateContainerOptions>,
    ) -> azure_core::Result<ContainerClient> {
        let mut backoff = Duration::from_millis(100);
        const MAX_BACKOFF: Duration = Duration::from_secs(10);

        loop {
            match db_client
                .create_container(properties.clone(), options.clone())
                .await
            {
                Ok(response) => {
                    let created = response.into_model()?;
                    return Ok(db_client.container_client(&created.id).await);
                }
                Err(e) if e.http_status() == Some(StatusCode::TooManyRequests) => {
                    println!(
                        "Create container got 429 (Too Many Requests). Retrying after {:?}...",
                        backoff
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                }
                Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
                    // Container already exists, delete and recreate it, then return a client
                    let container_client = db_client.container_client(&properties.id).await;
                    container_client.delete(None).await?;

                    // recreate
                    let response = db_client
                        .create_container(properties.clone(), options.clone())
                        .await?;
                    let created = response.into_model()?;
                    return Ok(db_client.container_client(&created.id).await);
                }
                Err(e) => return Err(e),
            }
        }
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
    pub async fn create_container_with_throughput(
        &self,
        db_client: &DatabaseClient,
        properties: azure_data_cosmos::models::ContainerProperties,
        throughput: ThroughputProperties,
    ) -> azure_core::Result<ContainerClient> {
        let created_properties = db_client
            .create_container(
                properties,
                Some(CreateContainerOptions::default().with_throughput(throughput)),
            )
            .await?
            .into_model()?;

        // Create two clients with different preferred regions to ensure container is available in both
        let hub_client = Self::create_client_with_preferred_region(HUB_REGION).await?;
        let satellite_client = Self::create_client_with_preferred_region(SATELLITE_REGION).await?;

        let container_id = &created_properties.id;

        // Wait for hub region client to successfully read the container
        loop {
            match hub_client
                .database_client(db_client.id())
                .container_client(container_id)
                .await
                .read(None)
                .await
            {
                Ok(_) => break,
                Err(e) => {
                    println!(
                        "waiting for container to be created in hub region ({}): {}",
                        HUB_REGION.as_str(),
                        e
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        // Wait for satellite region client to successfully read the container
        loop {
            match satellite_client
                .database_client(db_client.id())
                .container_client(container_id)
                .await
                .read(None)
                .await
            {
                Ok(_) => break,
                Err(e) => {
                    println!(
                        "waiting for container to be created in satellite region ({}): {}",
                        SATELLITE_REGION.as_str(),
                        e
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        Ok(db_client.container_client(container_id).await)
    }

    /// Creates a CosmosClient with a specific preferred region.
    async fn create_client_with_preferred_region(
        region: RegionName,
    ) -> Result<CosmosClient, azure_core::Error> {
        let env_var = std::env::var(CONNECTION_STRING_ENV_VAR)
            .unwrap_or_else(|_| EMULATOR_CONNECTION_STRING.to_string());

        let connection_string = if env_var == "emulator" {
            EMULATOR_CONNECTION_STRING
        } else {
            &env_var
        };

        let parsed: ConnectionString = connection_string.parse().map_err(|e| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("Failed to parse connection string: {}", e),
            )
        })?;

        let endpoint: azure_data_cosmos::CosmosAccountEndpoint =
            parsed.account_endpoint.parse().map_err(|e| {
                azure_core::Error::new(
                    azure_core::error::ErrorKind::Other,
                    format!("Failed to parse account endpoint: {}", e),
                )
            })?;
        CosmosClient::builder()
            .with_application_preferred_regions(vec![region])
            .build(azure_data_cosmos::CosmosAccountReference::with_master_key(
                endpoint,
                parsed.account_key.clone(),
            ))
            .await
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
        let mut pager = self.client().query_databases(query, None)?;
        let mut ids = Vec::new();
        while let Some(db) = pager.try_next().await? {
            ids.push(db.id);
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
