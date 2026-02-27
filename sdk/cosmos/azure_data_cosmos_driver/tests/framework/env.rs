// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Environment variable handling for test framework.

use std::str::FromStr;

/// Environment variable name for Cosmos DB connection string.
pub const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";

/// Environment variable for shared database name.
pub const DATABASE_NAME_ENV_VAR: &str = "DATABASE_NAME";

/// Environment variable to allow invalid certificates.
pub const ALLOW_INVALID_CERTS_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";

/// Environment variable for test mode.
pub const TEST_MODE_ENV_VAR: &str = "AZURE_COSMOS_TEST_MODE";

/// The well-known connection string for the Cosmos DB emulator.
///
/// This uses the default localhost endpoint and the well-known development key
/// that is used by all Cosmos DB emulator installations.
pub const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://localhost:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";

/// Test mode for Cosmos DB tests.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum CosmosTestMode {
    /// Tests are enabled and will fail if the env vars are not set.
    Required,

    /// Tests are disabled and will not attempt to run.
    Skipped,

    /// Tests can run if the env vars are set, but will not fail if they are not.
    #[default]
    Allowed,
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

/// Gets the test mode from the environment.
pub fn get_test_mode() -> CosmosTestMode {
    std::env::var(TEST_MODE_ENV_VAR)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_default()
}

/// Returns true if the tests are running on Azure Pipelines.
pub fn is_azure_pipelines() -> bool {
    std::env::var("SYSTEM_TEAMPROJECTID").is_ok()
}
