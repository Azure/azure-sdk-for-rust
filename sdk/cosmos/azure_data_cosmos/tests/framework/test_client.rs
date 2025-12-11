// Licensed under the MIT License.

#![cfg_attr(not(feature = "key_auth"), allow(dead_code))]

use std::sync::{Arc, OnceLock};

use azure_core::http::{StatusCode, Transport};
use azure_data_cosmos::{
    clients::DatabaseClient, ConnectionString, CosmosClient, CosmosClientOptions, Query,
};
use futures::TryStreamExt;
use reqwest::ClientBuilder;
use tracing_subscriber::EnvFilter;

/// Represents a Cosmos DB client connected to a test account.
pub struct TestClient {
    cosmos_client: Option<CosmosClient>,
}

#[derive(Default)]
pub struct TestClientOptions {
    pub allow_invalid_certificates: bool,
}

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const ALLOW_INVALID_CERTS_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://localhost:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";

static IS_AZURE_PIPELINES: OnceLock<bool> = OnceLock::new();

fn is_azure_pipelines() -> bool {
    *IS_AZURE_PIPELINES.get_or_init(|| std::env::var("SYSTEM_TEAMPROJECTID").is_ok())
}

impl TestClient {
    /// Creates a new [`TestAccount`] from local environment variables.
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
        // Initialize tracing subscriber for logging, if not already initialized.
        // The error is ignored because it only happens if the subscriber is already initialized.
        _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        let test_client = Self::from_env(None)?;

        // CosmosClient is designed to be cloned cheaply, so we can clone it here.
        if let Some(account) = test_client.cosmos_client.clone() {
            let run = TestRunContext::new(account);
            let result = test(&run).await;
            run.cleanup().await?;
            result
        } else if is_azure_pipelines() {
            // Everything should be set up in Azure Pipelines, so we treat missing connection string as an error.
            panic!("AZURE_COSMOS_CONNECTION_STRING environment variable is not set, but is required when running tests in Azure Pipelines.");
        } else {
            // No connection string provided, so we'll skip tests that require it.
            eprintln!("NOTE: Skipping emulator/live tests because no connection string was provided in the AZURE_COSMOS_CONNECTION_STRING environment variable.");
            Ok(())
        }
    }

    pub async fn run_with_db<F>(test: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: AsyncFnOnce(&TestRunContext, &DatabaseClient) -> Result<(), Box<dyn std::error::Error>>,
    {
        Self::run(async |run_context| {
            let db_client = run_context.create_db().await?;
            test(run_context, &db_client).await
        })
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

    /// Generates a unique database ID including the [`TestAccount::context_id`].
    ///
    /// This database will be automatically deleted when [`TestAccount::cleanup`] is called.
    pub fn db_name(&self) -> String {
        format!("auto-test-{}", self.run_id)
    }

    /// Gets the underlying [`CosmosClient`].
    pub fn client(&self) -> &CosmosClient {
        &self.client
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

    /// Cleans up test resources, then drops the [`TestAccount`].
    ///
    /// Call this at the end of every test using the [`TestAccount`].
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
