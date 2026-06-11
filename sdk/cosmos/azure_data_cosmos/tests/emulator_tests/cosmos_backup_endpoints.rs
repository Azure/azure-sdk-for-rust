// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-layer integration tests for backup endpoint fallback.
//!
//! Verifies that `CosmosClientBuilder::with_backup_endpoints()` correctly
//! threads backup endpoints through to the driver and that the client can
//! initialize when the primary endpoint is unreachable.
//!
//! These tests are gated on `test_category = "emulator"` only — they are
//! intentionally not run against `test_category = "emulator_vnext"` because
//! the vnext (Linux) emulator exposes a single gateway endpoint and does not
//! model the multi-endpoint topology backup-endpoint fallback is designed to
//! exercise.

use super::framework;

use azure_data_cosmos::{AccountEndpoint, AccountReference, CosmosClient, RoutingStrategy};
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

    let real_endpoint: AccountEndpoint = connection_string.account_endpoint().parse()?;
    let fake_endpoint: AccountEndpoint = "https://localhost:9/".parse()?;

    let builder = CosmosClient::builder().with_backup_endpoints(vec![real_endpoint]);

    let client = builder
        .build(
            AccountReference::with_authentication_key(
                fake_endpoint,
                connection_string.account_key().clone(),
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
