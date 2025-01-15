use std::sync::{Arc, Once};

use azure_core::{credentials::Secret, TransportOptions, Uuid};
use azure_core_test::TestContext;
use azure_data_cosmos::{CosmosClientOptions, Query}; // Get rid of this
use futures::StreamExt;
use reqwest::ClientBuilder;
use time::{macros::format_description, OffsetDateTime};

/// Represents a Azure Storage account for testing purposes.
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
static TRACING: Once = Once::new();

// Assuming we will want to change this over to DefaultAzureCredential
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
}
