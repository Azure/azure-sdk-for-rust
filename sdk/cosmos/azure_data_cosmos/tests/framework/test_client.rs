// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore: TEAMPROJECTID

#![cfg_attr(not(feature = "key_auth"), allow(dead_code))]

use azure_core::http::{StatusCode, Transport};
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::regions::{EAST_US_2, WEST_CENTRAL_US};
use azure_data_cosmos::{
    clients::DatabaseClient, ConnectionString, CosmosClient, CosmosClientOptions, PartitionKey,
    Query,
};
use futures::TryStreamExt;
use reqwest::ClientBuilder;
use std::borrow::Cow;
use std::convert::Into;
use std::time::Duration;
use std::{
    str::FromStr,
    sync::{Arc, OnceLock},
};
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
pub const ALLOW_INVALID_CERTS_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
pub const TEST_MODE_ENV_VAR: &str = "AZURE_COSMOS_TEST_MODE";
pub const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://localhost:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";
pub const HUB_REGION: &str = EAST_US_2;
pub const SATELLITE_REGION: &str = WEST_CENTRAL_US;
pub const DATABASE_NAME_ENV_VAR: &str = "DATABASE_NAME";

/// Default timeout for tests (60 seconds).
pub const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Options for configuring test execution.
#[derive(Default, Clone)]
pub struct TestOptions {
    /// CosmosClient options to use for the test.
    pub client_options: Option<CosmosClientOptions>,
    /// Timeout for the test. If None, uses DEFAULT_TEST_TIMEOUT.
    pub timeout: Option<Duration>,
}

impl TestOptions {
    /// Creates a new TestOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the client options.
    pub fn with_client_options(mut self, options: CosmosClientOptions) -> Self {
        self.client_options = Some(options);
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

fn get_shared_database_id() -> &'static str {
    static SHARED_DATABASE_ID: OnceLock<String> = OnceLock::new();

    let id = SHARED_DATABASE_ID.get_or_init(|| {
        std::env::var(DATABASE_NAME_ENV_VAR).unwrap_or_else(|_| {
            panic!(
                "{} is not set. Create a Cosmos DB database for tests, then set {} to its name (e.g. export {}=my-test-db).",
                DATABASE_NAME_ENV_VAR,
                DATABASE_NAME_ENV_VAR,
                DATABASE_NAME_ENV_VAR
            )
        })
    });

    id.as_str()
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
    /// Creates a new [`TestClient`] from local environment variables.
    ///
    /// If the environment variables are not set, this client will contain no underlying [`CosmosClient`].
    /// Calling `run` on such a client will skip running the closure (thus skipping the test), except when
    /// running on Azure Pipelines, when it will panic instead.
    pub fn from_env(
        options: Option<CosmosClientOptions>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Ok(env_var) = std::env::var(CONNECTION_STRING_ENV_VAR) else {
            // No connection string provided, so we'll skip tests that require it.
            return Ok(Self {
                cosmos_client: None,
            });
        };

        match env_var.as_ref() {
            "emulator" => {
                // Ignore that the test mode says playback, if the user explicitly asked for emulator, we use it.
                Self::from_connection_string(EMULATOR_CONNECTION_STRING, options, true)
            }
            _ => Self::from_connection_string(&env_var, options, false),
        }
    }

    fn from_connection_string(
        connection_string: &str,
        options: Option<CosmosClientOptions>,
        mut allow_invalid_certificates: bool,
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

        let mut options = options.unwrap_or_default();
        if allow_invalid_certificates {
            let client = ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .pool_max_idle_per_host(0)
                .build()?;
            options.client_options.transport = Some(Transport::new(Arc::new(client)));
        }

        let cosmos_client = azure_data_cosmos::CosmosClient::with_key(
            &connection_string.account_endpoint,
            connection_string.account_key.clone(),
            Some(options),
        )?;

        Ok(TestClient {
            cosmos_client: Some(cosmos_client),
        })
    }

    /// Runs a test function with a new [`TestClient`], ensuring proper setup and cleanup of the database.
    pub async fn run<F>(test: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnOnce(&TestRunContext) -> Result<(), Box<dyn std::error::Error>>,
    {
        Self::run_with_options(test, TestOptions::new()).await
    }

    /// Runs a test function with a new [`TestClient`] and custom test options.
    ///
    /// This method supports:
    /// - Timeouts (defaults to DEFAULT_TEST_TIMEOUT)
    /// - Consistency level checking (skips test if current consistency level is unsupported)
    /// - Custom CosmosClient options
    pub async fn run_with_options<F>(
        test: F,
        options: TestOptions,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnOnce(&TestRunContext) -> Result<(), Box<dyn std::error::Error>>,
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

        let test_client = Self::from_env(options.client_options)?;

        // CosmosClient is designed to be cloned cheaply, so we can clone it here.
        if let Some(account) = test_client.cosmos_client.clone() {
            let run = TestRunContext::new(account);

            // Apply timeout
            let timeout = options.timeout.unwrap_or(DEFAULT_TEST_TIMEOUT);
            let result = tokio::time::timeout(timeout, test(&run)).await;

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
        test: F,
        options: Option<TestOptions>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnOnce(&TestRunContext, &DatabaseClient) -> Result<(), Box<dyn std::error::Error>>,
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
        test: F,
        options: Option<TestOptions>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnOnce(&TestRunContext, &DatabaseClient) -> Result<(), Box<dyn std::error::Error>>,
    {
        Self::run_with_options(
            async |run_context| test(run_context, &run_context.shared_db_client()).await,
            options.unwrap_or_default(),
        )
        .await
    }
}

pub struct TestRunContext {
    run_id: String,
    client: CosmosClient,
}

impl TestRunContext {
    pub fn new(client: CosmosClient) -> Self {
        let run_id = uuid::Uuid::new_v4().simple().to_string();
        Self { run_id, client }
    }

    /// Generates a unique database ID including the [`TestRunContext::run_id`].
    ///
    /// This database will be automatically deleted when [`TestRunContext::cleanup`] is called (which will happen automatically if [`TestClient::run`] is used).
    pub fn db_name(&self) -> String {
        format!("auto-test-{}", self.run_id)
    }

    /// Gets the underlying [`CosmosClient`].
    pub fn client(&self) -> &CosmosClient {
        &self.client
    }

    /// Gets the shared database client.
    pub fn shared_db_client(&self) -> DatabaseClient {
        self.client().database_client(get_shared_database_id())
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

    /// Reads an item from the specified container with infinite retries on failure.
    /// This is useful for tests where eventual consistency may cause transient read failures.
    pub async fn read_item_infinite_retries<T>(
        &self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
    ) -> azure_core::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // Own the inputs so no borrowed data must live across `.await`.
        let partition_key = partition_key.into().to_owned();
        let item_id = item_id.to_owned();

        loop {
            match container
                .read_item(partition_key.clone(), item_id.clone().as_str(), None)
                .await
            {
                Ok(response) => return response.into_model(),
                Err(e) => {
                    println!("Read item failed: {}. Retrying...", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Queries items from the specified container with infinite retries on failure.
    /// This is useful for tests where eventual consistency may cause transient query failures.
    pub async fn query_items_infinite_retries<T>(
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

        loop {
            match container
                .query_items::<T>(query.clone(), partition_key.clone(), None)

            {
                Ok(pager) => {
                    match pager.try_collect::<Vec<T>>().await {
                        Ok(items) => return Ok(items),
                        Err(e) => {
                            println!("Query items failed: {}. Retrying...", e);
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
                Err(e) => {
                    println!("Query items failed: {}. Retrying...", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Creates a container with infinite retries on 429 (Too Many Requests) errors.
    /// This is useful for tests where rate limiting may cause transient failures.
    pub async fn create_container(
        &self,
        db_client: &DatabaseClient,
        properties: azure_data_cosmos::models::ContainerProperties,
        options: Option<azure_data_cosmos::CreateContainerOptions<'_>>,
    ) -> azure_core::Result<ContainerClient> {
        loop {
            match db_client.create_container(properties.clone(), options.clone()).await {
                Ok(response) => {
                    let created = response.into_model()?;
                    return Ok(db_client.container_client(&created.id));
                }
                Err(e) if e.http_status() == Some(StatusCode::TooManyRequests) => {
                    println!("Create container got 429 (Too Many Requests). Retrying...");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    panic!("{}", 429)
                }
                Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
                    // Container already exists, just return a client for it
                    return Ok(db_client.container_client(&properties.id));
                }
                Err(e) => {
                    panic!("Create container failed: {}. Not Retrying...", e);
                    // return Err(e)
                },
            }
        }
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

        // delete all the containers from shared database
        // read all containers
        let shared_db_client = self.shared_db_client();
        let mut container_pager = shared_db_client.query_containers(
            Query::from(
                "SELECT *
    FROM c",
            ),
            None,
        )?;
        let mut container_ids = Vec::new();
        while let Some(container) = container_pager.try_next().await? {
            container_ids.push(container.id);
        }

        // delete each container
        for container_id in container_ids {
            println!("Deleting left-over container: {}", &container_id);
            shared_db_client
                .container_client(&container_id)
                .delete(None)
                .await?;
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
