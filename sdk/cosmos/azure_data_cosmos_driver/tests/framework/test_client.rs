// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver test client for emulator-based E2E tests.

use azure_data_cosmos_driver::{
    diagnostics::{DiagnosticsContext, PipelineType, TransportSecurity},
    driver::CosmosDriverRuntime,
    models::{
        AccountReference, ConnectionString, ContainerReference, CosmosOperation, CosmosResponse,
        DatabaseReference, ItemReference, PartitionKey,
    },
    options::{ConnectionPoolOptions, EmulatorServerCertValidation, OperationOptions},
};
use std::{error::Error, future::Future, sync::Arc};
use uuid::Uuid;

use super::env::{
    get_test_mode, is_azure_pipelines, CosmosTestMode, CONNECTION_STRING_ENV_VAR,
    EMULATOR_CONNECTION_STRING,
};

/// A test client that provides access to a Cosmos DB driver for testing.
pub struct DriverTestClient {
    runtime: CosmosDriverRuntime,
    account: AccountReference,
}

impl DriverTestClient {
    /// Creates a new test client from environment variables.
    ///
    /// If the `AZURE_COSMOS_CONNECTION_STRING` environment variable is set to
    /// "emulator", uses the well-known emulator connection string. Otherwise,
    /// parses the provided connection string.
    ///
    /// Returns `None` if:
    /// - The environment variable is not set and test mode is not "required"
    /// - The test mode is "skipped"
    pub async fn from_env() -> Result<Option<Self>, Box<dyn Error>> {
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

        // Build runtime with emulator certificate handling
        let connection_pool = ConnectionPoolOptions::builder()
            .with_emulator_server_cert_validation(EmulatorServerCertValidation::DANGEROUS_DISABLED)
            .build()?;

        let runtime = CosmosDriverRuntime::builder()
            .with_connection_pool(connection_pool)
            .build()
            .await?;

        Ok(Some(Self { runtime, account }))
    }

    /// Runs a test with access to a driver and run context.
    ///
    /// The test will be skipped if the environment is not configured.
    pub async fn run<F, Fut>(f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(DriverTestRunContext) -> Fut,
        Fut: Future<Output = Result<(), Box<dyn Error>>>,
    {
        let Some(client) = Self::from_env().await? else {
            println!("Skipping test: Cosmos DB environment not configured");
            return Ok(());
        };

        let run_context = DriverTestRunContext::new(client);
        f(run_context).await
    }

    /// Runs a test with a unique database that will be cleaned up after the test.
    pub async fn run_with_unique_db<F, Fut>(f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(DriverTestRunContext, DatabaseReference) -> Fut,
        Fut: Future<Output = Result<(), Box<dyn Error>>>,
    {
        Self::run(async |context| {
            let db_name = context.unique_database_name();
            let db_ref = context.create_database(&db_name).await?;

            // Run the test
            let result = f(context.clone(), db_ref.clone()).await;

            // Cleanup (best effort)
            let _ = context.delete_database(&db_ref).await;

            result
        })
        .await
    }
}

/// Context for a test run, providing helpers for driver operations.
#[derive(Clone)]
pub struct DriverTestRunContext {
    client: Arc<DriverTestClient>,
    run_id: String,
}

impl DriverTestRunContext {
    fn new(client: DriverTestClient) -> Self {
        Self {
            client: Arc::new(client),
            run_id: Uuid::new_v4().to_string()[..8].to_string(),
        }
    }

    /// Returns the account reference for this context.
    pub fn account(&self) -> &AccountReference {
        &self.client.account
    }

    /// Generates a unique database name for this test run.
    pub fn unique_database_name(&self) -> String {
        format!("test-db-{}", self.run_id)
    }

    /// Generates a unique container name for this test run.
    pub fn unique_container_name(&self) -> String {
        let uuid_str = Uuid::new_v4().to_string();
        format!("test-container-{}", &uuid_str[..8])
    }

    /// Creates a database using the driver.
    pub async fn create_database(
        &self,
        db_name: &str,
    ) -> Result<DatabaseReference, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let body = format!(r#"{{"id": "{}"}}"#, db_name);
        let operation = CosmosOperation::create_database(self.client.account.clone())
            .with_body(body.into_bytes());

        let result = driver
            .execute_operation(operation, OperationOptions::new())
            .await?;

        // Check for success status (201 Created)
        let status = result.diagnostics().status();
        if !status.map(|s| s.is_success()).unwrap_or(false) {
            return Err(format!("Failed to create database, status: {:?}", status).into());
        }

        Ok(DatabaseReference::from_name(
            self.client.account.clone(),
            db_name.to_string(),
        ))
    }

    /// Deletes a database using the driver.
    pub async fn delete_database(
        &self,
        database: &DatabaseReference,
    ) -> Result<(), Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let operation = CosmosOperation::delete_database(database.clone());

        let result = driver
            .execute_operation(operation, OperationOptions::new())
            .await?;

        // Check for success status (204 No Content)
        let status = result.diagnostics().status();
        if !status.map(|s| s.is_success()).unwrap_or(false) {
            return Err(format!("Failed to delete database, status: {:?}", status).into());
        }

        Ok(())
    }

    /// Creates a container using the driver.
    pub async fn create_container(
        &self,
        database: &DatabaseReference,
        container_name: &str,
        partition_key_path: &str,
    ) -> Result<ContainerReference, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let body = format!(
            r#"{{"id": "{}", "partitionKey": {{"paths": ["{}"], "kind": "Hash"}}}}"#,
            container_name, partition_key_path
        );
        let operation =
            CosmosOperation::create_container(database.clone()).with_body(body.into_bytes());

        let result = driver
            .execute_operation(operation, OperationOptions::new())
            .await?;

        // Check for success status (201 Created)
        let status = result.diagnostics().status();
        if !status.map(|s| s.is_success()).unwrap_or(false) {
            return Err(format!("Failed to create container, status: {:?}", status).into());
        }

        Ok(ContainerReference::from_database(
            database,
            container_name.to_string(),
        ))
    }

    /// Creates an item using the driver.
    pub async fn create_item(
        &self,
        container: &ContainerReference,
        item_id: &str,
        partition_key: impl Into<PartitionKey>,
        body: &[u8],
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let pk = partition_key.into();
        let item = ItemReference::from_name(container, pk, item_id.to_owned());
        let operation = CosmosOperation::create_item(item).with_body(body.to_vec());

        let result = driver
            .execute_operation(operation, OperationOptions::new())
            .await?;

        Ok(result)
    }

    /// Reads an item using the driver.
    pub async fn read_item(
        &self,
        container: &ContainerReference,
        item_id: &str,
        partition_key: impl Into<PartitionKey>,
    ) -> Result<CosmosResponse, Box<dyn Error>> {
        let driver = self
            .client
            .runtime
            .get_or_create_driver(self.client.account.clone(), None)
            .await?;

        let pk = partition_key.into();
        let item_ref = ItemReference::from_name(container, pk, item_id.to_owned());
        let operation = CosmosOperation::read_item(item_ref);

        let result = driver
            .execute_operation(operation, OperationOptions::new())
            .await?;

        Ok(result)
    }

    /// Validates diagnostics for a successful data plane operation.
    pub fn validate_data_plane_diagnostics(
        &self,
        diagnostics: &DiagnosticsContext,
        expected_status: u16,
    ) {
        // Check status code
        let status = diagnostics.status_code();
        assert!(status.is_some(), "Diagnostics should have a status code");
        assert_eq!(
            u16::from(status.unwrap()),
            expected_status,
            "Status code should match expected"
        );

        // Check activity ID
        assert!(
            !diagnostics.activity_id().as_str().is_empty(),
            "Activity ID should not be empty"
        );

        // Check duration
        assert!(
            !diagnostics.duration().is_zero(),
            "Duration should be non-zero"
        );

        // Check requests
        let requests = diagnostics.requests();
        assert!(!requests.is_empty(), "Should have at least one request");

        // Check first request has correct pipeline type
        let first_request = &requests[0];
        assert_eq!(
            first_request.pipeline_type(),
            PipelineType::DataPlane,
            "Should use data plane pipeline for item operations"
        );

        // Check transport security for emulator
        if first_request.endpoint().contains("localhost") {
            assert_eq!(
                first_request.transport_security(),
                TransportSecurity::EmulatorWithInsecureCertificates,
                "Should use emulator transport security for localhost"
            );
        }

        // Check request charge is non-negative
        assert!(
            first_request.request_charge().value() >= 0.0,
            "Request charge should be non-negative"
        );
    }

    /// Validates diagnostics for a successful control plane operation.
    pub fn validate_control_plane_diagnostics(
        &self,
        diagnostics: &DiagnosticsContext,
        expected_status: u16,
    ) {
        // Check status code
        let status = diagnostics.status_code();
        assert!(status.is_some(), "Diagnostics should have a status code");
        assert_eq!(
            u16::from(status.unwrap()),
            expected_status,
            "Status code should match expected"
        );

        // Check requests
        let requests = diagnostics.requests();
        assert!(!requests.is_empty(), "Should have at least one request");

        // Check first request has correct pipeline type
        let first_request = &requests[0];
        assert_eq!(
            first_request.pipeline_type(),
            PipelineType::Metadata,
            "Should use metadata pipeline for control plane operations"
        );
    }
}
