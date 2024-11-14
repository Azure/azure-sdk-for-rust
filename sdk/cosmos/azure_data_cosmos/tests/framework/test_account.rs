#![cfg_attr(not(feature = "key_auth"), allow(dead_code))]

use azure_core::{credentials::Secret, Uuid};
use azure_data_cosmos::{CosmosClientOptions, Query};
use futures::StreamExt;
use time::{macros::format_description, OffsetDateTime};

/// Represents a Cosmos DB account for testing purposes.
///
/// A [`TestAccount`] serves two main purposes:
/// * Managing connection information to make it easy to connect to whatever account the tests are targetting.
/// * Provide a unique ID to each test and automatic clean-up of resources.
pub struct TestAccount {
    pub context_id: String,
    endpoint: String,
    key: Secret,
    cleaned_up: bool,
    preserve: bool,
}

const CONNECTION_STRING_ENV_VAR: &str = "AZSDK_COSMOS_CONNECTION_STRING";
const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://localhost:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";

impl TestAccount {
    /// Creates a new [`TestAccount`] from local environment variables.
    ///
    /// If the `AZSDK_COSMOS_TEST_ACCOUNT` environment variable is set, it will be used to create the account.
    /// The value can be either a Cosmos DB Connection String, or the special string `emulator` to use the local emulator.
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let env_var = std::env::var(CONNECTION_STRING_ENV_VAR).map_err(|e| {
            format!(
                "failed to read {} environment variable: {}",
                CONNECTION_STRING_ENV_VAR, e,
            )
        })?;
        match env_var.as_str() {
            "emulator" => Self::from_connection_string(EMULATOR_CONNECTION_STRING),
            _ => Self::from_connection_string(&env_var),
        }
    }

    fn from_connection_string(connection_string: &str) -> Result<Self, Box<dyn std::error::Error>> {
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
            "{}_{}",
            OffsetDateTime::now_utc().format(format_description!(
                "[year]_[month]_[day]T[hour]_[minute]_[second]"
            ))?,
            Uuid::new_v4().as_simple()
        );

        Ok(TestAccount {
            context_id,
            endpoint,
            key,
            cleaned_up: false,
            preserve: std::env::var("AZSDK_COSMOS_PRESERVE_DBS").unwrap_or_default() == "true",
        })
    }

    #[cfg(feature = "key_auth")]
    pub fn connect_with_key(
        &self,
        options: Option<CosmosClientOptions>,
    ) -> Result<azure_data_cosmos::CosmosClient, Box<dyn std::error::Error>> {
        Ok(azure_data_cosmos::CosmosClient::with_key(
            &self.endpoint,
            self.key.clone(),
            options,
        )?)
    }

    pub fn unique_id(&self, base_id: &str) -> String {
        format!("{}_{}", base_id, self.context_id)
    }

    #[cfg(feature = "key_auth")]
    pub async fn cleanup(mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.preserve {
            let cosmos_client = self.connect_with_key(None)?;
            let query =
                Query::from("SELECT * FROM root r WHERE r.id LIKE CONCAT('%_', @context_id)")
                    .with_parameter("@context_id", &self.context_id)?;
            let mut pager = cosmos_client.query_databases(query, None)?;
            let mut ids = Vec::new();
            while let Some(page) = pager.next().await {
                let results = page?.deserialize_body().await?;
                for db in results.databases {
                    ids.push(db.id);
                }
            }

            // Now that we have a list of databases created by this test, we delete them unless we were asked to preserve them.
            // We COULD choose not to delete them and instead validate that they were deleted, but this is what I've gone with for now.
            for id in ids {
                println!("Deleting left-over database: {}", &id);
                cosmos_client.database_client(&id).delete(None).await?;
            }
        }

        self.cleaned_up = true;
        Ok(())
    }
}

impl Drop for TestAccount {
    fn drop(&mut self) {
        // Async Drop isn't implemented yet, so all we do here is validate that you properly called ".cleanup().await?" to clean up your test databases.
        assert!(
            self.cleaned_up,
            "TestAccount was not cleaned up by calling 'TestAccount::cleanup().await?'",
        );
    }
}
