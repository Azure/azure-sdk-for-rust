// Licensed under the MIT License.

#![cfg_attr(not(feature = "key_auth"), allow(dead_code))]

use std::{
    borrow::Cow,
    future::Future,
    pin::Pin,
    str::FromStr,
    sync::{Arc, OnceLock},
};

use azure_core::{credentials::Secret, http::Transport, test::TestMode};
use azure_core_test::TestContext;
use azure_data_cosmos::{ConnectionString, CosmosClientOptions, Query};
use reqwest::ClientBuilder;
use tracing_subscriber::EnvFilter;

/// Represents a Cosmos DB account for testing purposes.
///
/// A [`TestAccount`] serves two main purposes:
/// * Managing connection information to make it easy to connect to the account the tests are using.
/// * Provide a unique ID to each test and automatic clean-up of resources.
pub struct TestAccount {
    run_id: String,
    connection_string: ConnectionString,
    options: TestAccountOptions,
}

#[derive(Default)]
pub struct TestAccountOptions {
    pub allow_invalid_certificates: bool,
}

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const ALLOW_INVALID_CERTS_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://localhost:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";

macro_rules! test_account {
    ($context:expr) => {
        TestAccount::from_env($context).await?
    };
}

static IS_AZURE_PIPELINES: OnceLock<bool> = OnceLock::new();

fn is_azure_pipelines() -> bool {
    *IS_AZURE_PIPELINES.get_or_init(|| std::env::var("SYSTEM_TEAMPROJECTID").is_ok())
}

impl TestAccount {
    pub async fn run<F>(test: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: for<'a> FnOnce(
            &'a TestAccount,
        ) -> Pin<
            Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + 'a>,
        >,
    {
        if let Some(account) = Self::from_env().await? {
            // Initialize tracing subscriber for logging, if not already initialized.
            // The error is ignored because it only happens if the subscriber is already initialized.
            _ = tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .try_init();

            let result = test(&account).await;
            account.cleanup().await?;
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

    /// Creates a new [`TestAccount`] from local environment variables.
    ///
    /// If the `AZURE_COSMOS_CONNECTION_STRING` environment variable is set, it will be used to create the account.
    /// The value can be either a Cosmos DB Connection String, or the special string `emulator` to use the local emulator.
    async fn from_env() -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let Ok(env_var) = std::env::var(CONNECTION_STRING_ENV_VAR) else {
            // No connection string provided, so we'll skip tests that require it.
            return Ok(None);
        };

        match env_var.as_ref() {
            "emulator" => {
                // Ignore that the test mode says playback, if the user explicitly asked for emulator, we use it.
                Self::from_connection_string(
                    EMULATOR_CONNECTION_STRING,
                    TestAccountOptions {
                        allow_invalid_certificates: true,
                    },
                )
                .await
                .map(Some)
            }
            _ => Self::from_connection_string(&env_var, TestAccountOptions::default())
                .await
                .map(Some),
        }
    }

    async fn from_connection_string(
        connection_string: &str,
        mut options: TestAccountOptions,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let connection_string = connection_string.parse()?;

        match std::env::var(ALLOW_INVALID_CERTS_ENV_VAR) {
            Ok(val) => {
                if let Ok(parsed) = val.parse::<bool>() {
                    if parsed {
                        options.allow_invalid_certificates = true;
                    }
                }
            }
            Err(_) => {}
        }

        let run_id = uuid::Uuid::new_v4().simple().to_string();

        Ok(TestAccount {
            run_id,
            connection_string,
            options,
        })
    }

    /// Create a [`CosmosClient`](azure_data_cosmos::CosmosClient) that connects to this account using a connection string.
    #[cfg(feature = "key_auth")]
    pub fn connect_with_key(
        &self,
        options: Option<CosmosClientOptions>,
    ) -> Result<azure_data_cosmos::CosmosClient, Box<dyn std::error::Error>> {
        let mut options = options.unwrap_or_default();

        if self.options.allow_invalid_certificates {
            let client = ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .pool_max_idle_per_host(0)
                .build()?;
            options.client_options.transport = Some(Transport::new(Arc::new(client)));
        }

        Ok(azure_data_cosmos::CosmosClient::with_key(
            &self.connection_string.account_endpoint,
            self.connection_string.account_key.clone(),
            Some(options),
        )?)
    }

    /// Generates a unique database ID including the [`TestAccount::context_id`].
    ///
    /// This database will be automatically deleted when [`TestAccount::cleanup`] is called.
    pub fn db_name(&self) -> String {
        format!("auto-test-{}", self.run_id)
    }

    /// Cleans up test resources, then drops the [`TestAccount`].
    ///
    /// Call this at the end of every test using the [`TestAccount`].
    #[cfg(feature = "key_auth")]
    pub async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        use futures::TryStreamExt;

        let cosmos_client = self.connect_with_key(None)?;
        let query = Query::from(format!(
            "SELECT * FROM root r WHERE r.id LIKE 'auto-test-{}'",
            self.run_id
        ));
        let mut pager = cosmos_client.query_databases(query, None)?;
        let mut ids = Vec::new();
        while let Some(db) = pager.try_next().await? {
            ids.push(db.id);
        }

        // Now that we have a list of databases created by this test, we delete them.
        // We COULD choose not to delete them and instead validate that they were deleted, but this is what I've gone with for now.
        for id in ids {
            println!("Deleting left-over database: {}", &id);
            cosmos_client.database_client(&id).delete(None).await?;
        }
        Ok(())
    }
}
