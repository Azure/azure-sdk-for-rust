// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for backup endpoint fallback during driver initialization.
//!
//! Verifies that the driver can bootstrap via a backup endpoint when the
//! primary global endpoint is unreachable.

use crate::framework::env::{
    get_test_mode, is_azure_pipelines, CosmosTestMode, CONNECTION_STRING_ENV_VAR,
    EMULATOR_CONNECTION_STRING,
};
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntimeBuilder,
    models::{AccountReference, ConnectionString, CosmosOperation, DatabaseReference},
    options::{ConnectionPoolOptions, EmulatorServerCertValidation, OperationOptions},
};
use std::error::Error;

/// Resolves the test environment, returning the real account reference and
/// connection pool options. Returns `None` if the environment is not configured.
fn resolve_backup_test_env(
) -> Result<Option<(AccountReference, ConnectionPoolOptions)>, Box<dyn Error>> {
    let _ = tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    let test_mode = get_test_mode();
    if test_mode == CosmosTestMode::Skipped {
        return Ok(None);
    }

    let connection_string = match std::env::var(CONNECTION_STRING_ENV_VAR) {
        Ok(val) if val.to_lowercase() == "emulator" => EMULATOR_CONNECTION_STRING.to_string(),
        Ok(val) => val,
        Err(_) => {
            if test_mode == CosmosTestMode::Required || is_azure_pipelines() {
                panic!(
                    "{} is not set but test mode is required",
                    CONNECTION_STRING_ENV_VAR
                );
            }
            return Ok(None);
        }
    };

    let conn_str: ConnectionString = connection_string.parse()?;
    let endpoint = conn_str.account_endpoint().parse()?;
    let key = conn_str.account_key().secret().to_string();
    let account = AccountReference::with_master_key(endpoint, key);

    let mut pool_builder = ConnectionPoolOptions::builder();
    if connection_string.eq_ignore_ascii_case(EMULATOR_CONNECTION_STRING) {
        pool_builder = pool_builder
            .with_emulator_server_cert_validation(EmulatorServerCertValidation::DangerousDisabled);
    }
    let connection_pool = pool_builder.build()?;

    Ok(Some((account, connection_pool)))
}

/// Tests that the driver can initialize when the primary global endpoint is
/// unreachable but a valid backup endpoint is provided.
///
/// Uses RFC 5737 TEST-NET address (`192.0.2.1`) as the non-routable primary.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn driver_initializes_via_backup_when_primary_unreachable() -> Result<(), Box<dyn Error>> {
    let Some((real_account, connection_pool)) = resolve_backup_test_env()? else {
        println!("Skipping test: Cosmos DB environment not configured");
        return Ok(());
    };

    // Create an account with a non-routable primary and the real endpoint as backup.
    let fake_primary: url::Url = "https://192.0.2.1:443/".parse()?;
    let account = AccountReference::builder(fake_primary)
        .auth(real_account.auth().clone())
        .with_backup_endpoints(vec![real_account.endpoint().clone()])
        .build()?;

    let runtime = CosmosDriverRuntimeBuilder::new()
        .with_connection_pool(connection_pool)
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
    let Some((real_account, connection_pool)) = resolve_backup_test_env()? else {
        println!("Skipping test: Cosmos DB environment not configured");
        return Ok(());
    };

    let fake_primary: url::Url = "https://192.0.2.1:443/".parse()?;
    let account = AccountReference::builder(fake_primary)
        .auth(real_account.auth().clone())
        .with_backup_endpoints(vec![real_account.endpoint().clone()])
        .build()?;

    let runtime = CosmosDriverRuntimeBuilder::new()
        .with_connection_pool(connection_pool)
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
