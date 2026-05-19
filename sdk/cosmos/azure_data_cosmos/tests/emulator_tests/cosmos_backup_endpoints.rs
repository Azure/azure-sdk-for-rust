// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-layer integration tests for backup endpoint fallback.
//!
//! Verifies that `CosmosClientBuilder::with_backup_endpoints()` correctly
//! threads backup endpoints through to the driver and that the client can
//! initialize when the primary endpoint is unreachable.

#![cfg(feature = "key_auth")]

use super::framework;

use azure_data_cosmos::{
    CosmosAccountEndpoint, CosmosAccountReference, CosmosClient, RoutingStrategy,
};
use framework::{resolve_connection_string, HUB_REGION};
use std::error::Error;

/// Tests that the SDK client can initialize when the primary global endpoint
/// is unreachable but a valid backup endpoint is provided.
///
/// Uses IANA Discard protocol port (`localhost:9`) for instant connection failure.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn client_boots_via_backup_when_primary_unreachable() -> Result<(), Box<dyn Error>> {
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
}
