// Licensed under the MIT License.

#![cfg_attr(not(feature = "key_auth"), allow(dead_code))]

use std::{borrow::Cow, sync::Arc};

use azure_core::{credentials::Secret, http::TransportOptions, test::TestMode};
use azure_core_test::TestContext;
use azure_data_cosmos::{CosmosClientOptions, Query};
use futures::StreamExt;
use reqwest::ClientBuilder;

/// Represents a Cosmos DB account for testing purposes.
///
/// A [`TestAccount`] serves two main purposes:
/// * Managing connection information to make it easy to connect to the account the tests are using.
/// * Provide a unique ID to each test and automatic clean-up of resources.
pub struct TestAccount {
    context: TestContext,
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

impl TestAccount {
    /// Creates a new [`TestAccount`] from local environment variables.
    ///
    /// If the `AZURE_COSMOS_CONNECTION_STRING` environment variable is set, it will be used to create the account.
    /// The value can be either a Cosmos DB Connection String, or the special string `emulator` to use the local emulator.
    pub async fn from_env(
        context: TestContext,
        options: Option<TestAccountOptions>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let env_var = match (
            context.recording().test_mode(),
            std::env::var(CONNECTION_STRING_ENV_VAR),
        ) {
            (TestMode::Playback, _) => Cow::Borrowed("emulator"), // The recording instrumentation should get the request before it hits a real endpoint.
            (_, Ok(s)) => Cow::Owned(s),
            (_, Err(e)) => {
                return Err(format!(
                    "failed to read {} environment variable: {}",
                    CONNECTION_STRING_ENV_VAR, e,
                )
                .into())
            }
        };

        match env_var.as_ref() {
            "emulator" => {
                Self::from_connection_string(EMULATOR_CONNECTION_STRING, context, {
                    let mut options = options.unwrap_or_default();
                    options.allow_invalid_certificates = Some(true);
                    Some(options)
                })
                .await
            }
            _ => Self::from_connection_string(&env_var, context, None).await,
        }
    }

    async fn from_connection_string(
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

        // We need the context_id to be constant, so that record/replay work.
        let context_id = context.name().to_string();

        // Disable some sanitizers that affect our tests
        context
            .recording()
            .remove_sanitizers(&[
                "AZSDK3430", // Sanitizes "id" properties. The tests need the id to be preserved.
            ])
            .await?;

        Ok(TestAccount {
            context,
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
        options: Option<CosmosClientOptions>,
    ) -> Result<azure_data_cosmos::CosmosClient, Box<dyn std::error::Error>> {
        let allow_invalid_certificates = match self.options.allow_invalid_certificates {
            Some(b) => b,
            None => std::env::var(ALLOW_INVALID_CERTS_ENV_VAR).map(|s| s.parse())??,
        };

        let mut options = options.unwrap_or_default();

        if allow_invalid_certificates {
            let client = ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .pool_max_idle_per_host(0)
                .build()?;
            options.client_options.transport = Some(TransportOptions::new(Arc::new(client)));
        }

        self.context
            .recording()
            .instrument(&mut options.client_options);

        Ok(azure_data_cosmos::CosmosClient::with_key(
            &self.endpoint,
            self.key.clone(),
            Some(options),
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
