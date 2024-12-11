#![cfg_attr(not(feature = "key_auth"), allow(dead_code))]

use std::sync::{Arc, Once};

use azure_core::{credentials::Secret, TransportOptions, Uuid};
use azure_core_test::TestContext;
use azure_data_cosmos::{CosmosClientOptions, Query};
use futures::StreamExt;
use reqwest::ClientBuilder;
use time::{macros::format_description, OffsetDateTime};

/// Represents a Cosmos DB account for testing purposes.
///
/// A [`TestAccount`] serves two main purposes:
/// * Managing connection information to make it easy to connect to the account the tests are using.
/// * Provide a unique ID to each test and automatic clean-up of resources.
pub struct TestAccount {
    pub context_id: String,
    endpoint: String,
    key: Secret,
    options: TestAccountOptions,
}

#[derive(Default)]
pub struct TestAccountOptions {
    pub allow_invalid_certificates: Option<bool>,
}

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const ALLOW_INVALID_CERTS_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://localhost:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";

static TRACING: Once = Once::new();

impl TestAccount {
    /// Creates a new [`TestAccount`] from local environment variables.
    ///
    /// If the `AZURE_COSMOS_CONNECTION_STRING` environment variable is set, it will be used to create the account.
    /// The value can be either a Cosmos DB Connection String, or the special string `emulator` to use the local emulator.
    pub fn from_env(
        context: TestContext,
        options: Option<TestAccountOptions>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let env_var = std::env::var(CONNECTION_STRING_ENV_VAR).map_err(|e| {
            format!(
                "failed to read {} environment variable: {}",
                CONNECTION_STRING_ENV_VAR, e,
            )
        })?;
        match env_var.as_str() {
            "emulator" => Self::from_connection_string(EMULATOR_CONNECTION_STRING, context, {
                let mut options = options.unwrap_or_default();
                options.allow_invalid_certificates = Some(true);
                Some(options)
            }),
            _ => Self::from_connection_string(&env_var, context, None),
        }
    }

    fn from_connection_string(
        connection_string: &str,
        context: TestContext,
        options: Option<TestAccountOptions>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let options = options.unwrap_or_default();
        let splat = connection_string.split(';');
        let mut account_endpoint = None;
        let mut account_key = None;
        for part in splat {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            let (key, value) = part.split_once('=').ok_or("invalid connection string")?;
            match key {
                "AccountEndpoint" => account_endpoint = Some(value.to_string()),
                "AccountKey" => account_key = Some(Secret::new(value.to_string())),
                _ => {}
            }
        }

        let Some(endpoint) = account_endpoint else {
            return Err("invalid connection string, missing 'AccountEndpoint'".into());
        };

        let Some(key) = account_key else {
            return Err("invalid connection string, missing 'AccountKey'".into());
        };

        let context_id = format!(
            "{}_{}_{}",
            context.test_name(),
            OffsetDateTime::now_utc().format(format_description!(
                "[year]_[month]_[day]T[hour]_[minute]_[second]"
            ))?,
            Uuid::new_v4().as_simple()
        );

        TRACING.call_once(|| {
            // Enable tracing for tests, if it's not already enabled
            _ = tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .try_init();

            // Ignore the failure. The most likely failure is that a global default trace dispatcher has already been set.
            // And we don't want the tracing support to bring down the tests.
        });

        Ok(TestAccount {
            context_id,
            endpoint,
            key,
            options,
        })
    }

    /// Create a [`CosmosClient`](azure_data_cosmos::CosmosClient) that connects to this account using a connection string.
    #[cfg(feature = "key_auth")]
    pub fn connect_with_key(
        &self,
        mut options: Option<CosmosClientOptions>,
    ) -> Result<azure_data_cosmos::CosmosClient, Box<dyn std::error::Error>> {
        let allow_invalid_certificates = match self.options.allow_invalid_certificates {
            Some(b) => b,
            None => std::env::var(ALLOW_INVALID_CERTS_ENV_VAR).map(|s| s.parse())??,
        };

        if allow_invalid_certificates {
            let client = ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .pool_max_idle_per_host(0)
                .build()?;
            options = {
                let mut o = options.unwrap_or_default();
                o.client_options.transport = Some(TransportOptions::new(Arc::new(client)));
                Some(o)
            };
        }

        Ok(azure_data_cosmos::CosmosClient::with_key(
            &self.endpoint,
            self.key.clone(),
            options,
        )?)
    }

    /// Generates a unique database ID including the [`TestAccount::context_id`].
    ///
    /// This database will be automatically deleted when [`TestAccount::cleanup`] is called.
    pub fn unique_db(&self, base_id: &str) -> String {
        format!("{}_{}", base_id, self.context_id)
    }

    /// Cleans up test resources, then drops the [`TestAccount`].
    ///
    /// Call this at the end of every test using the [`TestAccount`].
    #[cfg(feature = "key_auth")]
    pub async fn cleanup(self) -> Result<(), Box<dyn std::error::Error>> {
        let cosmos_client = self.connect_with_key(None)?;
        let query = Query::from("SELECT * FROM root r WHERE r.id LIKE CONCAT('%_', @context_id)")
            .with_parameter("@context_id", &self.context_id)?;
        let mut pager = cosmos_client.query_databases(query, None)?;
        let mut ids = Vec::new();
        while let Some(page) = pager.next().await {
            let results = page?.into_body().await?;
            for db in results.databases {
                ids.push(db.id);
            }
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
