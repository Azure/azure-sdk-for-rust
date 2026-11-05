// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use azure_data_cosmos::{
    options::Region, AccountEndpoint, AccountReference, CosmosClient, RoutingStrategy,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::e2e_test_cases::{
    fixture::{connection_string_value, fixture_test_lock, TestResult},
    support::selected_scenario_profile,
};

#[derive(Debug, PartialEq, Eq)]
enum BootstrapEndpoint {
    Primary,
    FirstBackup,
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn unreachable_primary_uses_ordered_backup() -> TestResult {
    if selected_scenario_profile("bootstrap.backup-fallback")
        .await?
        .is_none()
    {
        return Ok(());
    }
    let _guard = fixture_test_lock().lock().await;

    let connection_string = std::env::var("AZURE_COSMOS_CONNECTION_STRING")?;
    let reachable: AccountEndpoint =
        connection_string_value(&connection_string, "AccountEndpoint")?.parse()?;
    let key = connection_string_value(&connection_string, "AccountKey")?;

    let observations = Arc::new(Mutex::new(Vec::new()));
    let primary_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let unreachable_primary: AccountEndpoint =
        format!("http://{}/", primary_listener.local_addr()?).parse()?;
    let backup_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let first_backup: AccountEndpoint =
        format!("http://{}/", backup_listener.local_addr()?).parse()?;

    let primary_observations = Arc::clone(&observations);
    let primary_observed = Arc::new(AtomicBool::new(false));
    let primary_first_observation = Arc::clone(&primary_observed);
    let primary_sentinel = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = primary_listener.accept().await else {
                break;
            };
            if !primary_first_observation.swap(true, Ordering::SeqCst) {
                primary_observations
                    .lock()
                    .unwrap()
                    .push(BootstrapEndpoint::Primary);
            }
            let mut request = [0_u8; 2048];
            let _ = stream.read(&mut request).await;
            let _ = stream
                .write_all(
                    b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .await;
        }
    });

    let backup_observations = Arc::clone(&observations);
    let backup_observed = Arc::new(AtomicBool::new(false));
    let backup_first_observation = Arc::clone(&backup_observed);
    let backup_sentinel = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = backup_listener.accept().await else {
                break;
            };
            if !backup_first_observation.swap(true, Ordering::SeqCst) {
                backup_observations
                    .lock()
                    .unwrap()
                    .push(BootstrapEndpoint::FirstBackup);
            }
            let mut request = [0_u8; 2048];
            let _ = stream.read(&mut request).await;
            let _ = stream
                .write_all(
                    b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .await;
        }
    });

    let client = CosmosClient::builder()
        .with_backup_endpoints(vec![first_backup, reachable])
        .build(
            AccountReference::with_authentication_key(unreachable_primary, key),
            RoutingStrategy::ProximityTo(Region::EAST_US),
        )
        .await;
    primary_sentinel.abort();
    backup_sentinel.abort();
    let client = client?;
    assert_eq!(
        *observations.lock().unwrap(),
        [BootstrapEndpoint::Primary, BootstrapEndpoint::FirstBackup],
        "bootstrap must exhaust the primary before trying backups in order"
    );

    // Construction resolves account metadata, so reaching a usable public
    // client proves the ordered backup path completed bootstrap.
    let _database = client.database_client("backup-bootstrap-probe");
    Ok(())
}
