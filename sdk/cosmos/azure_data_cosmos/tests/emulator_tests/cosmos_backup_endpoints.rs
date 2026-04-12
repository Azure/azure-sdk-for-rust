// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-layer integration tests for backup endpoint fallback.
//!
//! Verifies that `CosmosClientBuilder::with_backup_endpoints()` correctly
//! threads backup endpoints through to the driver and that the client can
//! initialize and operate when the primary endpoint is unreachable.

#![cfg(feature = "key_auth")]

use super::framework;

use azure_data_cosmos::{
    ConnectionString, CosmosAccountEndpoint, CosmosAccountReference, CosmosClient, RoutingStrategy,
};
use framework::{TestClient, CONNECTION_STRING_ENV_VAR, EMULATOR_CONNECTION_STRING, HUB_REGION};
use std::error::Error;

/// Resolves the connection string from the environment, handling the `"emulator"` shorthand.
fn resolve_connection_string() -> Option<ConnectionString> {
    let env_var = std::env::var(CONNECTION_STRING_ENV_VAR).ok()?;
    let raw = if env_var == "emulator" {
        EMULATOR_CONNECTION_STRING
    } else {
        &env_var
    };
    raw.parse().ok()
}

/// Tests that the SDK client can initialize when the primary global endpoint
/// is unreachable but a valid backup endpoint is provided.
///
/// Uses IANA Discard protocol port (`localhost:9`) for instant connection failure.
#[tokio::test]
async fn client_boots_via_backup_when_primary_unreachable() -> Result<(), Box<dyn Error>> {
    TestClient::run(async |_run_context| {
        let connection_string =
            resolve_connection_string().expect("Cosmos DB connection string must be configured");

        let real_endpoint: CosmosAccountEndpoint = connection_string.account_endpoint.parse()?;
        let fake_endpoint: CosmosAccountEndpoint = "https://localhost:9/".parse()?;

        let mut builder = CosmosClient::builder().with_backup_endpoints(vec![real_endpoint]);

        #[cfg(feature = "allow_invalid_certificates")]
        {
            builder = builder.with_allow_emulator_invalid_certificates(true);
        }

        let client = builder
            .build(
                CosmosAccountReference::with_master_key(
                    fake_endpoint,
                    connection_string.account_key.clone(),
                ),
                RoutingStrategy::ProximityTo(HUB_REGION),
            )
            .await;

        assert!(
            client.is_ok(),
            "client should initialize via backup endpoint, but got: {:?}",
            client.err()
        );

        Ok(())
    })
    .await
}

/// Tests that the SDK client can query databases after initializing via a
/// backup endpoint.
#[tokio::test]
async fn client_can_query_after_backup_boot() -> Result<(), Box<dyn Error>> {
    TestClient::run(async |_run_context| {
        let connection_string =
            resolve_connection_string().expect("Cosmos DB connection string must be configured");

        let real_endpoint: CosmosAccountEndpoint = connection_string.account_endpoint.parse()?;
        let fake_endpoint: CosmosAccountEndpoint = "https://localhost:9/".parse()?;

        let mut builder = CosmosClient::builder().with_backup_endpoints(vec![real_endpoint]);

        #[cfg(feature = "allow_invalid_certificates")]
        {
            builder = builder.with_allow_emulator_invalid_certificates(true);
        }

        let client = builder
            .build(
                CosmosAccountReference::with_master_key(
                    fake_endpoint,
                    connection_string.account_key.clone(),
                ),
                RoutingStrategy::ProximityTo(HUB_REGION),
            )
            .await?;

        // Verify the client can query databases (end-to-end through SDK → driver)
        use futures::TryStreamExt;
        let query = azure_data_cosmos::Query::from("SELECT * FROM root r");
        let mut pager = client.query_databases(query, None)?;
        let page = pager.try_next().await;
        assert!(
            page.is_ok(),
            "should be able to query databases after backup boot: {:?}",
            page.err()
        );

        Ok(())
    })
    .await
}
