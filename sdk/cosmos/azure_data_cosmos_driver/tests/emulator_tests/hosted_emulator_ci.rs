// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! CI contract for the out-of-process in-memory emulator host.

use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
    future::Future,
    sync::Arc,
    time::Duration,
};

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    diagnostics::DiagnosticsContext,
    driver::{CosmosDriver, CosmosDriverRuntime},
    models::{
        ContainerReference, CosmosOperation, CosmosResponse, FeedRange, ItemReference,
        PartitionKey, ResponseBody,
    },
    options::{DriverOptions, OperationOptions, PlanOptions},
    SubStatusCode,
};
use serde::{Deserialize, Serialize};

use crate::framework::{resolve_test_env, DriverTestClient};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const MANAGEMENT_TIMEOUT: Duration = Duration::from_secs(10);
const POLL_INTERVAL: Duration = Duration::from_millis(25);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostHealth {
    gateway20_enabled: bool,
    connectivity_probes: usize,
    gateway20_requests: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
enum OperationPhase {
    Preparing,
    Swapping,
    Succeeded,
    Failed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ManagementOperation {
    operation_id: String,
    phase: OperationPhase,
    error: Option<String>,
}

struct ManagementClient {
    endpoint: url::Url,
    http: reqwest::Client,
}

impl ManagementClient {
    fn from_env() -> TestResult<Self> {
        let endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
        let endpoint = url::Url::parse(&format!("{}/", endpoint.trim_end_matches('/')))?;
        let http = reqwest::Client::builder()
            .timeout(MANAGEMENT_TIMEOUT)
            .build()?;
        Ok(Self { endpoint, http })
    }

    async fn health(&self) -> TestResult<HostHealth> {
        self.get_json("health").await
    }

    async fn start_automatic_split(
        &self,
        database: &str,
        container: &str,
        partition_id: u32,
    ) -> TestResult<ManagementOperation> {
        self.post_json(
            &format!("databases/{database}/containers/{container}/partitions/{partition_id}/split"),
            Some(&serde_json::json!({ "mode": "midpoint" })),
        )
        .await
    }

    async fn start_manual_merge(
        &self,
        database: &str,
        container: &str,
        partition_ids: [u32; 2],
    ) -> TestResult<ManagementOperation> {
        self.post_json(
            &format!("databases/{database}/containers/{container}/partitions/merge"),
            Some(&serde_json::json!({
                "partitionIds": partition_ids,
                "progressionMode": "manual"
            })),
        )
        .await
    }

    async fn advance(&self, operation_id: &str) -> TestResult<ManagementOperation> {
        self.post_json(&format!("operations/{operation_id}/advance"), None)
            .await
    }

    async fn advance_to_swapping(&self, operation_id: &str) -> TestResult<()> {
        let operation = self.advance(operation_id).await?;
        if operation.phase != OperationPhase::Swapping {
            return Err(
                format!("operation {operation_id} did not enter Swapping: {operation:?}").into(),
            );
        }
        Ok(())
    }

    async fn complete_manual(&self, operation_id: &str) -> TestResult<ManagementOperation> {
        let operation = self.advance(operation_id).await?;
        if operation.phase != OperationPhase::Swapping {
            return Err(format!(
                "operation {operation_id} left Swapping before completion: {operation:?}"
            )
            .into());
        }
        self.wait_for_phase(operation_id, OperationPhase::Succeeded)
            .await
    }

    async fn wait_for_phase(
        &self,
        operation_id: &str,
        expected: OperationPhase,
    ) -> TestResult<ManagementOperation> {
        let deadline = tokio::time::Instant::now() + MANAGEMENT_TIMEOUT;
        loop {
            let operation: ManagementOperation =
                self.get_json(&format!("operations/{operation_id}")).await?;
            if operation.phase == expected {
                return Ok(operation);
            }
            if operation.phase == OperationPhase::Failed {
                return Err(format!(
                    "operation {operation_id} failed: {}",
                    operation.error.as_deref().unwrap_or("no error reported")
                )
                .into());
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(format!(
                    "operation {operation_id} did not reach {expected:?} within {MANAGEMENT_TIMEOUT:?}; last state: {operation:?}"
                )
                .into());
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }

    async fn assert_expected_traffic_since(&self, before: &HostHealth) -> TestResult {
        let after = self.health().await?;
        match std::env::var("AZURE_COSMOS_EMULATOR_FLAVOR").as_deref() {
            Ok("inmemory-v1") => assert!(!after.gateway20_enabled),
            Ok("inmemory-v2") => {
                assert!(after.gateway20_enabled);
                assert!(after.connectivity_probes > 0);
                assert!(
                    after.gateway20_requests > before.gateway20_requests,
                    "expected Gateway V2 traffic to increase from {}, got {}",
                    before.gateway20_requests,
                    after.gateway20_requests
                );
            }
            flavor => return Err(format!("unexpected hosted emulator flavor: {flavor:?}").into()),
        }
        Ok(())
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(&self, path: &str) -> TestResult<T> {
        let response = self
            .http
            .get(self.endpoint.join(path)?)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        Ok(serde_json::from_slice(&response)?)
    }

    async fn post_json<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: Option<&serde_json::Value>,
    ) -> TestResult<T> {
        let mut request = self.http.post(self.endpoint.join(path)?);
        if let Some(body) = body {
            request = request
                .header("content-type", "application/json")
                .body(serde_json::to_vec(body)?);
        }
        let response = request.send().await?.error_for_status()?.bytes().await?;
        Ok(serde_json::from_slice(&response)?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct MergeDocument {
    id: String,
    pk: String,
    value: usize,
}

async fn seed_adjacent_partitions<F, Fut>(
    mut create: F,
) -> TestResult<([u32; 2], [MergeDocument; 2])>
where
    F: FnMut(MergeDocument) -> Fut,
    Fut: Future<Output = TestResult<CosmosResponse>>,
{
    let mut documents = BTreeMap::<u32, MergeDocument>::new();
    for index in 0..128 {
        let document = MergeDocument {
            id: format!("merge-item-{index}"),
            pk: format!("merge-pk-{index}"),
            value: index,
        };
        let response = create(document.clone()).await?;
        let partition_id = response
            .headers()
            .partition_key_range_id
            .as_deref()
            .and_then(|id| id.parse::<u32>().ok())
            .ok_or("create response did not report a partition key range id")?;
        documents.entry(partition_id).or_insert(document);

        let ids = documents.keys().copied().collect::<Vec<_>>();
        if let Some(pair) = ids.windows(2).find(|pair| pair[1] == pair[0] + 1) {
            let partition_ids = [pair[0], pair[1]];
            let documents = [
                documents[&partition_ids[0]].clone(),
                documents[&partition_ids[1]].clone(),
            ];
            return Ok((partition_ids, documents));
        }
    }
    Err("could not seed documents into two adjacent partitions".into())
}

fn assert_diagnostics_include(
    diagnostics: &DiagnosticsContext,
    status: StatusCode,
    substatus: SubStatusCode,
) {
    assert!(
        diagnostics_include(diagnostics, status, substatus),
        "expected diagnostics to include {status}/{substatus:?}: {diagnostics:?}"
    );
}

fn diagnostics_include(
    diagnostics: &DiagnosticsContext,
    status: StatusCode,
    substatus: SubStatusCode,
) -> bool {
    diagnostics.requests().iter().any(|request| {
        request.status().status_code() == status && request.status().sub_status() == Some(substatus)
    })
}

async fn create_persistent_driver() -> TestResult<Arc<CosmosDriver>> {
    let environment = resolve_test_env()?.ok_or("Cosmos DB environment is not configured")?;
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(environment.connection_pool)
        .build()
        .await?;
    Ok(runtime
        .create_driver(DriverOptions::builder(environment.account).build())
        .await?)
}

async fn read_item_on(
    driver: &CosmosDriver,
    container: &ContainerReference,
    document: &MergeDocument,
) -> azure_data_cosmos_driver::Result<CosmosResponse> {
    let item = ItemReference::from_name(
        container,
        PartitionKey::from(document.pk.clone()),
        document.id.clone(),
    );
    driver
        .execute_singleton_operation(
            CosmosOperation::read_item(item),
            OperationOptions::default(),
        )
        .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory"),
    ignore = "requires test_category 'emulator_inmemory'"
)]
async fn configured_host_mode_is_exercised() -> TestResult {
    let management = ManagementClient::from_env()?;
    let metrics_before = management.health().await?;

    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let body = br#"{"id":"host-mode-item","pk":"pk1","value":42}"#;
        context
            .create_item(&container, "host-mode-item", "pk1", body)
            .await?;
        context
            .read_item(&container, "host-mode-item", "pk1")
            .await?;
        Ok(())
    })
    .await?;

    management
        .assert_expected_traffic_since(&metrics_before)
        .await
}

/// Proves the production driver transparently refreshes stale routing after a split.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory"),
    ignore = "requires test_category 'emulator_inmemory'"
)]
async fn driver_recovers_after_real_partition_split() -> TestResult {
    let management = ManagementClient::from_env()?;
    let metrics_before = management.health().await?;

    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let db_name = database
            .name()
            .ok_or("database reference must be name-based")?;

        let created = context
            .create_item(
                &container,
                "split-item-1",
                "pk-a",
                br#"{"id":"split-item-1","pk":"pk-a","value":1}"#,
            )
            .await?;
        context
            .create_item(
                &container,
                "split-item-2",
                "pk-z",
                br#"{"id":"split-item-2","pk":"pk-z","value":2}"#,
            )
            .await?;
        let partition_id: u32 = created
            .headers()
            .partition_key_range_id
            .as_deref()
            .and_then(|id| id.parse().ok())
            .ok_or("could not determine partition id from response headers")?;

        let operation = management
            .start_automatic_split(db_name, &container_name, partition_id)
            .await?;
        management
            .wait_for_phase(&operation.operation_id, OperationPhase::Succeeded)
            .await?;

        assert!(context
            .read_item(&container, "split-item-1", "pk-a")
            .await?
            .status()
            .is_success());
        assert!(context
            .read_item(&container, "split-item-2", "pk-z")
            .await?
            .status()
            .is_success());
        Ok(())
    })
    .await?;

    management
        .assert_expected_traffic_since(&metrics_before)
        .await
}

/// Proves a real manual merge exposes its locked partitions as 410/1007.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory"),
    ignore = "requires test_category 'emulator_inmemory'"
)]
async fn driver_observes_410_1007_while_merge_is_swapping() -> TestResult {
    let management = ManagementClient::from_env()?;
    let metrics_before = management.health().await?;

    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let db_name = database
            .name()
            .ok_or("database reference must be name-based")?;
        let (partition_ids, documents) = seed_adjacent_partitions(|document| {
            let context = context.clone();
            let container = container.clone();
            async move {
                let body = serde_json::to_vec(&document)?;
                context
                    .create_item(&container, &document.id, document.pk.clone(), &body)
                    .await
            }
        })
        .await?;

        let operation = management
            .start_manual_merge(db_name, &container_name, partition_ids)
            .await?;
        assert_eq!(operation.phase, OperationPhase::Preparing);
        management
            .advance_to_swapping(&operation.operation_id)
            .await?;

        let read = tokio::time::timeout(
            MANAGEMENT_TIMEOUT,
            context.read_item(&container, &documents[0].id, documents[0].pk.clone()),
        )
        .await;
        management.complete_manual(&operation.operation_id).await?;

        let error = read?.expect_err("a read of a merge-locked partition should return 410/1007");
        let error = error
            .downcast_ref::<azure_data_cosmos_driver::CosmosError>()
            .ok_or("merge-locked read did not return a CosmosError")?;
        assert_eq!(error.status().status_code(), StatusCode::Gone);
        assert_eq!(
            error.status().sub_status(),
            Some(SubStatusCode::COMPLETING_SPLIT)
        );
        let diagnostics = error
            .diagnostics()
            .ok_or("410/1007 error did not include diagnostics")?;
        assert_diagnostics_include(
            diagnostics.as_ref(),
            StatusCode::Gone,
            SubStatusCode::COMPLETING_SPLIT,
        );
        Ok(())
    })
    .await?;

    management
        .assert_expected_traffic_since(&metrics_before)
        .await
}

/// Proves stale pre-merge routing recovers and both parent documents survive.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory"),
    ignore = "requires test_category 'emulator_inmemory'"
)]
async fn driver_recovers_stale_routing_after_merge() -> TestResult {
    let management = ManagementClient::from_env()?;
    let metrics_before = management.health().await?;
    let driver = create_persistent_driver().await?;

    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let db_name = database
            .name()
            .ok_or("database reference must be name-based")?;
        let (partition_ids, documents) = seed_adjacent_partitions(|document| {
            let context = context.clone();
            let container = container.clone();
            async move {
                let body = serde_json::to_vec(&document)?;
                context
                    .create_item(&container, &document.id, document.pk.clone(), &body)
                    .await
            }
        })
        .await?;
        // Prime the production driver's routing cache with the pre-merge
        // physical ranges.
        let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
            .with_body(br#"{"query":"SELECT * FROM c","parameters":[]}"#.to_vec());
        let mut warm_plan = Box::pin(driver.plan_operation(
            operation,
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        ))
        .await?;
        while driver
            .execute_plan(
                &mut warm_plan,
                Some(container.clone()),
                OperationOptions::default(),
            )
            .await?
            .is_some()
        {}

        let operation = management
            .start_manual_merge(db_name, &container_name, partition_ids)
            .await?;
        management
            .advance_to_swapping(&operation.operation_id)
            .await?;
        management.complete_manual(&operation.operation_id).await?;

        let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
            .with_body(br#"{"query":"SELECT * FROM c","parameters":[]}"#.to_vec());
        let mut stale_plan = Box::pin(driver.plan_operation(
            operation,
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        ))
        .await?;
        let mut queried_ids = HashSet::new();
        while let Some(response) = driver
            .execute_plan(
                &mut stale_plan,
                Some(container.clone()),
                OperationOptions::default(),
            )
            .await?
        {
            match response.into_body() {
                ResponseBody::NoPayload => {}
                ResponseBody::Items(items) => {
                    for item in items.iter() {
                        let value: serde_json::Value = serde_json::from_slice(item)?;
                        queried_ids.insert(value["id"].as_str().unwrap().to_owned());
                    }
                }
                ResponseBody::Bytes(body) => {
                    let value: serde_json::Value = serde_json::from_slice(&body)?;
                    for item in value["Documents"].as_array().unwrap() {
                        queried_ids.insert(item["id"].as_str().unwrap().to_owned());
                    }
                }
            }
        }
        assert!(queried_ids.contains(&documents[0].id));
        assert!(queried_ids.contains(&documents[1].id));

        let recovered: MergeDocument = read_item_on(&driver, &container, &documents[0])
            .await?
            .into_body()
            .into_single()?;
        let preserved: MergeDocument = read_item_on(&driver, &container, &documents[1])
            .await?
            .into_body()
            .into_single()?;
        assert_eq!(recovered, documents[0]);
        assert_eq!(preserved, documents[1]);
        Ok(())
    })
    .await?;

    management
        .assert_expected_traffic_since(&metrics_before)
        .await
}
