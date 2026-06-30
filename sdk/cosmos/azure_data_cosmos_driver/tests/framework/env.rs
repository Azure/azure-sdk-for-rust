// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Environment variable handling for test framework.

use std::str::FromStr;

/// Environment variable name for Cosmos DB connection string.
pub const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";

/// Environment variable name for the pre-provisioned Gateway 2.0 account
/// endpoint. Surfaced from the `azure-sdk-tests-cosmos` service connection's
/// secret variable group by `sdk/cosmos/ci.yml` and consumed by driver tests
/// scoped to `test_category = "gateway_v2"`.
///
/// The ARM-provisioned account in the `Session SingleRegion GatewayV2` matrix
/// entry does NOT advertise a Gateway 2.0 endpoint, so driver tests that
/// inject faults gated on `TransportKind::GatewayV2` (and SDK tests in
/// `sdk/cosmos/azure_data_cosmos/tests/gateway_v2_tests/gateway_v2_e2e.rs`) must
/// point at this pre-provisioned account instead of the standard
/// `AZURE_COSMOS_CONNECTION_STRING`.
#[allow(
    dead_code,
    reason = "Consumed only by gateway_v2 test categories; unused in other test binaries that include this shared framework module."
)]
pub const GATEWAY_V2_ENDPOINT_ENV_VAR: &str = "AZURE_COSMOS_GW_V2_ENDPOINT";

/// Environment variable name for the pre-provisioned Gateway 2.0 account
/// master key. See [`GATEWAY_V2_ENDPOINT_ENV_VAR`].
#[allow(
    dead_code,
    reason = "Consumed only by gateway_v2 test categories; unused in other test binaries that include this shared framework module."
)]
pub const GATEWAY_V2_KEY_ENV_VAR: &str = "AZURE_COSMOS_GW_V2_KEY";

/// Environment variable name for the pre-provisioned **multi-region**
/// Gateway 2.0 account endpoint, consumed by driver tests scoped to
/// `test_category = "gateway_v2_multi_region"`. The single-region account
/// referenced by [`GATEWAY_V2_ENDPOINT_ENV_VAR`] cannot exercise
/// regional-failover behaviors, so multi-region tests need a dedicated
/// multi-region GW_V2 account.
#[allow(
    dead_code,
    reason = "Consumed only by gateway_v2_multi_region test categories; unused in other test binaries that include this shared framework module."
)]
pub const GATEWAY_V2_MULTI_REGION_ENDPOINT_ENV_VAR: &str =
    "AZURE_COSMOS_GW_V2_MULTI_REGION_ENDPOINT";

/// Environment variable name for the pre-provisioned multi-region Gateway 2.0
/// account master key. See [`GATEWAY_V2_MULTI_REGION_ENDPOINT_ENV_VAR`].
#[allow(
    dead_code,
    reason = "Consumed only by gateway_v2_multi_region test categories; unused in other test binaries that include this shared framework module."
)]
pub const GATEWAY_V2_MULTI_REGION_KEY_ENV_VAR: &str = "AZURE_COSMOS_GW_V2_MULTI_REGION_KEY";

/// Environment variable for shared database name.
#[allow(
    dead_code,
    reason = "Part of the shared test-framework surface; unused in test binaries that include this module without referencing it."
)]
pub const DATABASE_NAME_ENV_VAR: &str = "DATABASE_NAME";

/// Environment variable to allow invalid certificates.
#[allow(
    dead_code,
    reason = "Part of the shared test-framework surface; unused in test binaries that include this module without referencing it."
)]
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
