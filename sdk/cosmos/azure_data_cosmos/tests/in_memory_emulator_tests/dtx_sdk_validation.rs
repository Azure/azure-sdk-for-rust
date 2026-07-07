// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-level DTX validation tests.

use std::{error::Error, sync::Arc};

use azure_core::{
    credentials::Secret,
    http::{headers::HeaderName, Method, Request, StatusCode, Url},
};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder, CosmosRuntimeBuilder,
    RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::{
    models::{
        AccountReference as DriverAccountReference, CosmosOperation, ItemReference, PartitionKey,
    },
    options::DriverOptions,
};
use serde_json::json;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

static DTX_IDEMPOTENCY_TOKEN: HeaderName = HeaderName::from_static("x-ms-cosmos-idempotency-token");
static DTX_OPERATION_TYPE: HeaderName = HeaderName::from_static("x-ms-cosmos-operation-type");
static DTX_RESOURCE_TYPE: HeaderName = HeaderName::from_static("x-ms-cosmos-resource-type");
const DTX_RESOURCE_TYPE_VALUE: &str = "DistributedTransactionBatch";

async fn emulator_client(
    endpoint: &str,
) -> Result<(CosmosClient, Arc<InMemoryEmulatorHttpClient>), Box<dyn Error>> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(endpoint)?,
    )])?;
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let account = AccountReference::with_authentication_key(
        endpoint.parse::<AccountEndpoint>()?,
        Secret::new("dGVzdGtleQ=="),
    );
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await?,
        )
        .build(
            account,
            RoutingStrategy::PreferredRegions(vec!["East US".into()]),
        )
        .await?;
    Ok((client, emulator))
}

fn valid_dtx_body(index: usize) -> serde_json::Value {
    serde_json::json!({
        "operations": [{
            "index": index,
            "databaseName": "db",
            "collectionName": "coll",
            "id": "item",
            "partitionKey": ["pk"],
            "operationType": "Read"
        }]
    })
}

fn raw_dtx_request(endpoint: &str, body: serde_json::Value, include_headers: bool) -> Request {
    let mut request = Request::new(
        Url::parse(&format!("{endpoint}/operations/dtc")).unwrap(),
        Method::Post,
    );
    if include_headers {
        request
            .headers_mut()
            .insert(DTX_IDEMPOTENCY_TOKEN.clone(), Uuid::new_v4().to_string());
        request
            .headers_mut()
            .insert(DTX_OPERATION_TYPE.clone(), "Read");
        request
            .headers_mut()
            .insert(DTX_RESOURCE_TYPE.clone(), DTX_RESOURCE_TYPE_VALUE);
    }
    request.set_body(serde_json::to_vec(&body).unwrap());
    request
}

async fn raw_dtx_status(
    emulator: &InMemoryEmulatorHttpClient,
    request: &Request,
) -> Result<StatusCode, Box<dyn Error>> {
    let response = emulator.execute_request(request).await?;
    let raw = response.try_into_raw_response().await?;
    Ok(raw.status())
}

#[tokio::test]
async fn write_transaction_rejects_container_from_different_account() -> Result<(), Box<dyn Error>>
{
    let (transaction_client, _) = emulator_client("https://account-a.emulator.local").await?;
    let (container_client_owner, container_emulator) =
        emulator_client("https://account-b.emulator.local").await?;
    container_emulator.store().create_database("db");
    container_emulator.store().create_container(
        "db",
        "coll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))?,
    );
    let foreign_container = container_client_owner
        .database_client("db")
        .container_client("coll")
        .await?;

    let transaction = azure_data_cosmos::DistributedWriteTransaction::new().delete_item(
        &foreign_container,
        "pk",
        "item",
        None,
    );

    let result = transaction_client
        .commit_distributed_write(transaction)
        .await;
    let error = match result {
        Ok(_) => panic!("foreign account container should be rejected"),
        Err(error) => error,
    };

    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert!(
        error
            .to_string()
            .contains("same Cosmos account as the committing client"),
        "unexpected error: {error}"
    );
    Ok(())
}

#[tokio::test]
async fn read_transaction_rejects_container_from_different_account() -> Result<(), Box<dyn Error>> {
    let (transaction_client, _) = emulator_client("https://account-c.emulator.local").await?;
    let (container_client_owner, container_emulator) =
        emulator_client("https://account-d.emulator.local").await?;
    container_emulator.store().create_database("db");
    container_emulator.store().create_container(
        "db",
        "coll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))?,
    );
    let foreign_container = container_client_owner
        .database_client("db")
        .container_client("coll")
        .await?;

    let transaction = azure_data_cosmos::DistributedReadTransaction::new().read_item(
        &foreign_container,
        "pk",
        "item",
        None,
    );

    let result = transaction_client
        .execute_distributed_read(transaction)
        .await;
    let error = match result {
        Ok(_) => panic!("foreign account container should be rejected"),
        Err(error) => error,
    };

    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert!(
        error
            .to_string()
            .contains("same Cosmos account as the committing client"),
        "unexpected error: {error}"
    );
    Ok(())
}

#[tokio::test]
async fn emulator_dtx_rejects_missing_required_headers() -> Result<(), Box<dyn Error>> {
    let endpoint = "https://wire-a.emulator.local";
    let (_, emulator) = emulator_client(endpoint).await?;
    let request = raw_dtx_request(endpoint, valid_dtx_body(0), false);

    let status = raw_dtx_status(&emulator, &request).await?;

    assert_eq!(status, StatusCode::BadRequest);
    Ok(())
}

#[tokio::test]
async fn emulator_dtx_rejects_mismatched_operation_index() -> Result<(), Box<dyn Error>> {
    let endpoint = "https://wire-b.emulator.local";
    let (_, emulator) = emulator_client(endpoint).await?;
    emulator.store().create_database("db");
    emulator.store().create_container(
        "db",
        "coll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))?,
    );
    let request = raw_dtx_request(endpoint, valid_dtx_body(1), true);

    let status = raw_dtx_status(&emulator, &request).await?;

    assert_eq!(status, StatusCode::BadRequest);
    Ok(())
}

#[tokio::test]
async fn emulator_dtx_echoes_request_operation_index() -> Result<(), Box<dyn Error>> {
    let endpoint = "https://wire-c.emulator.local";
    let (_, emulator) = emulator_client(endpoint).await?;
    emulator.store().create_database("db");
    emulator.store().create_container(
        "db",
        "coll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))?,
    );
    let request = raw_dtx_request(endpoint, valid_dtx_body(0), true);

    let response = emulator.execute_request(&request).await?;
    let raw = response.try_into_raw_response().await?;
    let body: serde_json::Value = serde_json::from_slice(raw.body().as_ref())?;

    assert_eq!(raw.status(), StatusCode::NotFound);
    assert_eq!(body["operationResponses"][0]["index"], 0);
    Ok(())
}

#[tokio::test]
async fn emulator_point_writes_wait_for_dtx_write_guard() -> Result<(), Box<dyn Error>> {
    let endpoint = "https://wire-d.emulator.local";
    let (_, emulator) = emulator_client(endpoint).await?;
    emulator.store().create_database("db");
    emulator.store().create_container(
        "db",
        "coll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))?,
    );
    let guard_lock = emulator.store().document_write_lock_for_tests();
    let guard = guard_lock.lock().await;

    let runtime = emulator.runtime_builder().build().await?;
    let driver = runtime
        .create_driver(
            DriverOptions::builder(DriverAccountReference::with_master_key(
                Url::parse(endpoint)?,
                "dGVzdGtleQ==",
            ))
            .build(),
        )
        .await?;
    let container = driver.resolve_container("db", "coll").await?;
    let mut write_task = tokio::spawn(async move {
        let body = serde_json::to_vec(&json!({
            "id": "blocked-item",
            "pk": "pk"
        }))
        .expect("test item body should serialize");
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(ItemReference::from_name(
                    &container,
                    PartitionKey::from("pk"),
                    "blocked-item",
                ))
                .with_body(body),
                Default::default(),
            )
            .await
    });

    assert!(
        timeout(Duration::from_millis(25), &mut Box::pin(&mut write_task))
            .await
            .is_err(),
        "point write should wait while DTX write guard is held"
    );
    drop(guard);

    write_task.await??;
    Ok(())
}
