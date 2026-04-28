// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for backup endpoint fallback during driver initialization.
//!
//! Verifies that the driver can bootstrap via a backup endpoint when the
//! primary global endpoint is unreachable.

use crate::framework::resolve_test_env;
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntimeBuilder,
    models::{AccountReference, CosmosOperation, DatabaseReference},
    options::OperationOptions,
};
use std::error::Error;

/// Tests that the driver can initialize when the primary global endpoint is
/// unreachable but a valid backup endpoint is provided.
///
/// Uses IANA Discard protocol port (`localhost:9`) as the non-routable primary.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn driver_initializes_via_backup_when_primary_unreachable() -> Result<(), Box<dyn Error>> {
    let env = resolve_test_env()?.expect("Cosmos DB environment must be configured");

    // Create an account with a non-routable primary and the real endpoint as backup.
    let fake_primary: url::Url = "https://localhost:9/".parse()?;
    let account = AccountReference::builder(fake_primary)
        .auth(env.account.auth().clone())
        .with_backup_endpoints(vec![env.account.endpoint().clone()])
        .build()?;

    let runtime = CosmosDriverRuntimeBuilder::new()
        .with_connection_pool(env.connection_pool)
        .build()
        .await?;

    let driver = runtime.get_or_create_driver(account, None).await;

    assert!(
        driver.is_ok(),
        "driver should initialize via backup endpoint, but got: {:?}",
        driver.err()
    );

    Ok(())
}

/// Tests that the driver can execute operations after bootstrapping via a
/// backup endpoint.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn driver_operations_work_after_backup_boot() -> Result<(), Box<dyn Error>> {
    let env = resolve_test_env()?.expect("Cosmos DB environment must be configured");

    let fake_primary: url::Url = "https://localhost:9/".parse()?;
    let account = AccountReference::builder(fake_primary)
        .auth(env.account.auth().clone())
        .with_backup_endpoints(vec![env.account.endpoint().clone()])
        .build()?;

    let runtime = CosmosDriverRuntimeBuilder::new()
        .with_connection_pool(env.connection_pool)
        .build()
        .await?;

    let driver = runtime.get_or_create_driver(account.clone(), None).await?;

    // Create a database to verify the driver is operational.
    let db_name = format!(
        "backup-test-{}",
        uuid::Uuid::new_v4().to_string()[..8].to_owned()
    );
    let body = format!(r#"{{"id": "{}"}}"#, db_name);
    let operation = CosmosOperation::create_database(account.clone()).with_body(body.into_bytes());

    let result = driver
        .execute_operation(operation, OperationOptions::default())
        .await;

    assert!(
        result.is_ok(),
        "should be able to create database after backup boot: {:?}",
        result.err()
    );

    // Cleanup
    let db_ref = DatabaseReference::from_name(account, db_name);
    let _ = driver
        .execute_operation(
            CosmosOperation::delete_database(db_ref),
            OperationOptions::default(),
        )
        .await;

    Ok(())
}
