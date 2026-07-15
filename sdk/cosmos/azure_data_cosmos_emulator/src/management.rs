// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore hexdigit

use std::{
    collections::HashMap,
    io,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    EmulatorStore, Epk, InMemoryEmulatorHttpClient, ManualControlPlaneOperation, WriteMode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{net::TcpListener, sync::Mutex};

use crate::{config::GatewayBinding, metrics::HostMetrics};

const MAX_CONTROL_PLANE_LOCK_DURATION_MS: u64 = 60_000;

#[derive(Clone)]
struct ManagementState {
    emulator: Arc<InMemoryEmulatorHttpClient>,
    account_id: Arc<str>,
    bindings: Arc<[GatewayBinding]>,
    metrics: Arc<HostMetrics>,
    operations: Arc<OperationRegistry>,
}

#[derive(Default)]
struct OperationRegistry {
    next_id: AtomicU64,
    records: Mutex<HashMap<String, OperationRecord>>,
}

pub(crate) async fn serve(
    listener: TcpListener,
    emulator: Arc<InMemoryEmulatorHttpClient>,
    account_id: String,
    bindings: Vec<GatewayBinding>,
    metrics: Arc<HostMetrics>,
) -> io::Result<()> {
    tracing::info!(endpoint = %listener.local_addr()?, "emulator management API ready");
    axum::serve(listener, router(emulator, account_id, bindings, metrics)).await
}

fn router(
    emulator: Arc<InMemoryEmulatorHttpClient>,
    account_id: String,
    bindings: Vec<GatewayBinding>,
    metrics: Arc<HostMetrics>,
) -> Router {
    let state = ManagementState {
        emulator,
        account_id: account_id.into(),
        bindings: bindings.into(),
        metrics,
        operations: Arc::new(OperationRegistry::default()),
    };
    Router::new()
        .route("/health", get(health))
        .route("/account", get(account))
        .route(
            "/databases/{database}/containers/{container}/partitions/{partition_id}/split",
            post(split_partition),
        )
        .route(
            "/databases/{database}/containers/{container}/partitions/merge",
            post(merge_partitions),
        )
        .route("/operations/{operation_id}", get(get_operation))
        .route(
            "/operations/{operation_id}/advance",
            post(advance_operation),
        )
        .route(
            "/config/per-partition-failover",
            put(set_per_partition_failover),
        )
        .route(
            "/regions/{region}/replication/pause",
            post(pause_replication),
        )
        .route(
            "/regions/{region}/replication/resume",
            post(resume_replication),
        )
        .with_state(state)
}

async fn health(State(state): State<ManagementState>) -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "gateway20Enabled": state
            .bindings
            .iter()
            .any(|binding| binding.gateway20_url.is_some()),
        "connectivityProbes": state.metrics.connectivity_probes(),
        "gateway20Requests": state.metrics.gateway20_requests()
    }))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AccountResponse {
    id: String,
    write_mode: &'static str,
    consistency: String,
    per_partition_failover: bool,
    regions: Vec<RegionResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RegionResponse {
    name: String,
    gateway_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway20_endpoint: Option<String>,
}

async fn account(State(state): State<ManagementState>) -> Json<AccountResponse> {
    let config = state.emulator.store().config().clone();
    let write_mode = match config.write_mode() {
        WriteMode::Single => "single",
        WriteMode::Multi => "multi",
    };
    let regions = state
        .bindings
        .iter()
        .map(|binding| RegionResponse {
            name: binding.region_name.clone(),
            gateway_endpoint: binding.gateway_url.as_str().to_owned(),
            gateway20_endpoint: binding
                .gateway20_url
                .as_ref()
                .map(|url| url.as_str().to_owned()),
        })
        .collect();
    Json(AccountResponse {
        id: state.account_id.to_string(),
        write_mode,
        consistency: config.consistency().as_str().to_owned(),
        per_partition_failover: config.per_partition_failover_enabled(),
        regions,
    })
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum ProgressionMode {
    #[default]
    Automatic,
    Manual,
}

#[derive(Clone, Copy, Debug, Serialize)]
enum OperationStatus {
    Running,
    Succeeded,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
enum OperationPhase {
    Preparing,
    Swapping,
    Succeeded,
    Failed,
}

struct OperationRecord {
    status: OperationStatus,
    phase: OperationPhase,
    result: Option<serde_json::Value>,
    error: Option<String>,
    action: Option<OperationAction>,
}

enum OperationAction {
    PendingSplit {
        database: String,
        container: String,
        parent: u32,
        mode: SplitMode,
        split_epk: Epk,
    },
    ActiveSplit {
        database: String,
        container: String,
        parent: u32,
        mode: SplitMode,
        split_epk: Epk,
        operation: ManualControlPlaneOperation,
    },
    PendingMerge {
        database: String,
        container: String,
        partitions: [u32; 2],
    },
    ActiveMerge {
        database: String,
        container: String,
        partitions: [u32; 2],
        operation: ManualControlPlaneOperation,
    },
}

impl OperationRegistry {
    fn next_id(&self, kind: &str) -> String {
        let sequence = self.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        format!("op-{kind}-{sequence}")
    }
}

impl OperationRecord {
    fn running(action: Option<OperationAction>) -> Self {
        Self {
            status: OperationStatus::Running,
            phase: OperationPhase::Preparing,
            result: None,
            error: None,
            action,
        }
    }

    fn succeed(&mut self, result: serde_json::Value) {
        self.status = OperationStatus::Succeeded;
        self.phase = OperationPhase::Succeeded;
        self.result = Some(result);
        self.error = None;
        self.action = None;
    }

    fn fail(&mut self, error: impl Into<String>) {
        self.status = OperationStatus::Failed;
        self.phase = OperationPhase::Failed;
        self.result = None;
        self.error = Some(error.into());
        self.action = None;
    }
}

fn operation_response(operation_id: &str, operation: &OperationRecord) -> serde_json::Value {
    let mut response = json!({
        "operationId": operation_id,
        "status": operation.status,
        "phase": operation.phase,
    });
    let object = response
        .as_object_mut()
        .expect("operation response must be an object");
    if let Some(serde_json::Value::Object(result)) = &operation.result {
        object.extend(result.clone());
    }
    if let Some(error) = &operation.error {
        object.insert("error".to_owned(), error.clone().into());
    }
    response
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum SplitMode {
    #[default]
    Midpoint,
    Epk,
    Storage,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SplitRequest {
    #[serde(default)]
    mode: SplitMode,
    epk: Option<String>,
    #[serde(default)]
    progression_mode: ProgressionMode,
    lock_duration_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SplitResponse {
    database: String,
    container: String,
    parent: u32,
    children: Vec<u32>,
    mode: SplitMode,
    split_epk: String,
}

struct SplitOperationDetails {
    database: String,
    container: String,
    parent: u32,
    mode: SplitMode,
    split_epk: Epk,
}

async fn split_partition(
    State(state): State<ManagementState>,
    Path((database, container, partition_id)): Path<(String, String, u32)>,
    request: Option<Json<SplitRequest>>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let request = request.map(|request| request.0).unwrap_or_default();
    let lock_duration = validate_progression(request.progression_mode, request.lock_duration_ms)?;
    let store = state.emulator.store();
    let split_epk = match request.mode {
        SplitMode::Midpoint => store.midpoint_split_epk(&database, &container, partition_id)?,
        SplitMode::Epk => {
            let value = request
                .epk
                .as_deref()
                .ok_or_else(|| ApiError::bad_request("epk is required when mode is 'epk'"))?;
            parse_epk(value)?
        }
        SplitMode::Storage => store.storage_split_epk(&database, &container, partition_id)?,
    };
    let operation_id = state.operations.next_id("split");
    let action = match request.progression_mode {
        ProgressionMode::Automatic => None,
        ProgressionMode::Manual => Some(OperationAction::PendingSplit {
            database: database.clone(),
            container: container.clone(),
            parent: partition_id,
            mode: request.mode,
            split_epk: split_epk.clone(),
        }),
    };
    let operation = OperationRecord::running(action);
    let response = operation_response(&operation_id, &operation);
    state
        .operations
        .records
        .lock()
        .await
        .insert(operation_id.clone(), operation);

    if request.progression_mode == ProgressionMode::Automatic {
        spawn_automatic_split(
            state,
            operation_id,
            SplitOperationDetails {
                database,
                container,
                parent: partition_id,
                mode: request.mode,
                split_epk,
            },
            lock_duration,
        );
    }

    Ok((StatusCode::ACCEPTED, Json(response)))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MergeRequest {
    partition_ids: [u32; 2],
    #[serde(default)]
    progression_mode: ProgressionMode,
    lock_duration_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
struct MergeResponse {
    merged: [u32; 2],
    into: u32,
}

async fn merge_partitions(
    State(state): State<ManagementState>,
    Path((database, container)): Path<(String, String)>,
    Json(request): Json<MergeRequest>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let lock_duration = validate_progression(request.progression_mode, request.lock_duration_ms)?;
    let operation_id = state.operations.next_id("merge");
    let action = match request.progression_mode {
        ProgressionMode::Automatic => None,
        ProgressionMode::Manual => Some(OperationAction::PendingMerge {
            database: database.clone(),
            container: container.clone(),
            partitions: request.partition_ids,
        }),
    };
    let operation = OperationRecord::running(action);
    let response = operation_response(&operation_id, &operation);
    state
        .operations
        .records
        .lock()
        .await
        .insert(operation_id.clone(), operation);

    if request.progression_mode == ProgressionMode::Automatic {
        spawn_automatic_merge(
            state,
            operation_id,
            database,
            container,
            request.partition_ids,
            lock_duration,
        );
    }

    Ok((StatusCode::ACCEPTED, Json(response)))
}

fn spawn_automatic_split(
    state: ManagementState,
    operation_id: String,
    details: SplitOperationDetails,
    lock_duration: Duration,
) {
    tokio::spawn(async move {
        let SplitOperationDetails {
            database,
            container,
            parent,
            mode,
            split_epk,
        } = details;
        let store = state.emulator.store();
        let operation =
            store.begin_manual_split_partition(&database, &container, parent, split_epk.clone());
        set_operation_swapping(&state.operations, &operation_id).await;
        if !lock_duration.is_zero() {
            tokio::time::sleep(lock_duration).await;
        }
        let outcome = match operation.complete().await {
            Ok(()) => {
                let children = store.child_partition_ids(&database, &container, &[parent]);
                if children.len() == 2 {
                    Ok(json!(SplitResponse {
                        database,
                        container,
                        parent,
                        children,
                        mode,
                        split_epk: split_epk.to_hex(),
                    }))
                } else {
                    Err("split completed without producing exactly two child partitions".to_owned())
                }
            }
            Err(error) => Err(error.to_string()),
        };
        finish_operation(&state.operations, &operation_id, outcome).await;
    });
}

fn spawn_automatic_merge(
    state: ManagementState,
    operation_id: String,
    database: String,
    container: String,
    partitions: [u32; 2],
    lock_duration: Duration,
) {
    tokio::spawn(async move {
        let store = state.emulator.store();
        let operation = store.begin_manual_merge_partitions(
            &database,
            &container,
            partitions[0],
            partitions[1],
        );
        set_operation_swapping(&state.operations, &operation_id).await;
        if !lock_duration.is_zero() {
            tokio::time::sleep(lock_duration).await;
        }
        let outcome = match operation.complete().await {
            Ok(()) => {
                let children = store.child_partition_ids(&database, &container, &partitions);
                match children.as_slice() {
                    [merged_child] => Ok(json!(MergeResponse {
                        merged: partitions,
                        into: *merged_child,
                    })),
                    _ => Err(
                        "merge completed without producing exactly one child partition".to_owned(),
                    ),
                }
            }
            Err(error) => Err(error.to_string()),
        };
        finish_operation(&state.operations, &operation_id, outcome).await;
    });
}

async fn set_operation_swapping(operations: &OperationRegistry, operation_id: &str) {
    if let Some(operation) = operations.records.lock().await.get_mut(operation_id) {
        operation.phase = OperationPhase::Swapping;
    }
}

async fn finish_operation(
    operations: &OperationRegistry,
    operation_id: &str,
    outcome: Result<serde_json::Value, String>,
) {
    if let Some(operation) = operations.records.lock().await.get_mut(operation_id) {
        match outcome {
            Ok(result) => operation.succeed(result),
            Err(error) => operation.fail(error),
        }
    }
}

async fn get_operation(
    State(state): State<ManagementState>,
    Path(operation_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let operations = state.operations.records.lock().await;
    let operation = operations
        .get(&operation_id)
        .ok_or_else(|| ApiError::not_found(format!("operation '{operation_id}' does not exist")))?;
    Ok(Json(operation_response(&operation_id, operation)))
}

enum ManualCompletion {
    Split {
        database: String,
        container: String,
        parent: u32,
        mode: SplitMode,
        split_epk: Epk,
        operation: ManualControlPlaneOperation,
    },
    Merge {
        database: String,
        container: String,
        partitions: [u32; 2],
        operation: ManualControlPlaneOperation,
    },
}

async fn advance_operation(
    State(state): State<ManagementState>,
    Path(operation_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let completion = {
        let mut operations = state.operations.records.lock().await;
        let operation = operations.get_mut(&operation_id).ok_or_else(|| {
            ApiError::not_found(format!("operation '{operation_id}' does not exist"))
        })?;
        if operation.phase == OperationPhase::Succeeded || operation.phase == OperationPhase::Failed
        {
            return Err(ApiError::conflict(format!(
                "operation '{operation_id}' is already terminal"
            )));
        }
        let action = operation.action.take().ok_or_else(|| {
            ApiError::conflict(format!(
                "operation '{operation_id}' is automatic or already advancing"
            ))
        })?;
        match action {
            OperationAction::PendingSplit {
                database,
                container,
                parent,
                mode,
                split_epk,
            } => {
                let manual_operation = state.emulator.store().begin_manual_split_partition(
                    &database,
                    &container,
                    parent,
                    split_epk.clone(),
                );
                operation.phase = OperationPhase::Swapping;
                operation.action = Some(OperationAction::ActiveSplit {
                    database,
                    container,
                    parent,
                    mode,
                    split_epk,
                    operation: manual_operation,
                });
                return Ok(Json(operation_response(&operation_id, operation)));
            }
            OperationAction::PendingMerge {
                database,
                container,
                partitions,
            } => {
                let manual_operation = state.emulator.store().begin_manual_merge_partitions(
                    &database,
                    &container,
                    partitions[0],
                    partitions[1],
                );
                operation.phase = OperationPhase::Swapping;
                operation.action = Some(OperationAction::ActiveMerge {
                    database,
                    container,
                    partitions,
                    operation: manual_operation,
                });
                return Ok(Json(operation_response(&operation_id, operation)));
            }
            OperationAction::ActiveSplit {
                database,
                container,
                parent,
                mode,
                split_epk,
                operation,
            } => ManualCompletion::Split {
                database,
                container,
                parent,
                mode,
                split_epk,
                operation,
            },
            OperationAction::ActiveMerge {
                database,
                container,
                partitions,
                operation,
            } => ManualCompletion::Merge {
                database,
                container,
                partitions,
                operation,
            },
        }
    };

    let store = state.emulator.store();
    let outcome = match completion {
        ManualCompletion::Split {
            database,
            container,
            parent,
            mode,
            split_epk,
            operation,
        } => match operation.complete().await {
            Ok(()) => {
                let children = store.child_partition_ids(&database, &container, &[parent]);
                if children.len() == 2 {
                    Ok(json!(SplitResponse {
                        database,
                        container,
                        parent,
                        children,
                        mode,
                        split_epk: split_epk.to_hex(),
                    }))
                } else {
                    Err("split completed without producing exactly two child partitions".to_owned())
                }
            }
            Err(error) => Err(error.to_string()),
        },
        ManualCompletion::Merge {
            database,
            container,
            partitions,
            operation,
        } => match operation.complete().await {
            Ok(()) => {
                let children = store.child_partition_ids(&database, &container, &partitions);
                match children.as_slice() {
                    [merged_child] => Ok(json!(MergeResponse {
                        merged: partitions,
                        into: *merged_child,
                    })),
                    _ => Err(
                        "merge completed without producing exactly one child partition".to_owned(),
                    ),
                }
            }
            Err(error) => Err(error.to_string()),
        },
    };
    finish_operation(&state.operations, &operation_id, outcome).await;

    let operations = state.operations.records.lock().await;
    let operation = operations
        .get(&operation_id)
        .ok_or_else(|| ApiError::not_found(format!("operation '{operation_id}' does not exist")))?;
    Ok(Json(operation_response(&operation_id, operation)))
}

#[derive(Debug, Deserialize, Serialize)]
struct EnabledRequest {
    enabled: bool,
}

async fn set_per_partition_failover(
    State(state): State<ManagementState>,
    Json(request): Json<EnabledRequest>,
) -> Json<EnabledRequest> {
    state
        .emulator
        .store()
        .config()
        .set_per_partition_failover(request.enabled);
    Json(request)
}

async fn pause_replication(
    State(state): State<ManagementState>,
    Path(region): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    ensure_region(&state.emulator.store(), &region)?;
    state.emulator.store().pause_replication(&region);
    Ok(Json(json!({ "region": region, "replication": "paused" })))
}

async fn resume_replication(
    State(state): State<ManagementState>,
    Path(region): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    ensure_region(&state.emulator.store(), &region)?;
    state.emulator.store().resume_replication(&region);
    Ok(Json(json!({ "region": region, "replication": "resumed" })))
}

fn ensure_region(store: &EmulatorStore, name: &str) -> ApiResult<()> {
    if store
        .config()
        .regions()
        .iter()
        .any(|region| region.name() == name)
    {
        Ok(())
    } else {
        Err(ApiError::not_found(format!(
            "region '{name}' is not configured"
        )))
    }
}

fn parse_epk(value: &str) -> ApiResult<Epk> {
    if value.is_empty()
        || !value.len().is_multiple_of(2)
        || !value.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(ApiError::bad_request(
            "epk must be a non-empty, even-length hexadecimal string",
        ));
    }
    Ok(Epk::from(value))
}

fn validate_progression(
    progression_mode: ProgressionMode,
    lock_duration_ms: Option<u64>,
) -> ApiResult<Duration> {
    if progression_mode == ProgressionMode::Manual && lock_duration_ms.is_some() {
        return Err(ApiError::bad_request(
            "lockDurationMs is accepted only when progressionMode is 'automatic'",
        ));
    }
    let lock_duration_ms = lock_duration_ms.unwrap_or_default();
    if lock_duration_ms > MAX_CONTROL_PLANE_LOCK_DURATION_MS {
        return Err(ApiError::bad_request(format!(
            "lockDurationMs must be <= {MAX_CONTROL_PLANE_LOCK_DURATION_MS}"
        )));
    }
    Ok(Duration::from_millis(lock_duration_ms))
}

type ApiResult<T> = std::result::Result<T, ApiError>;

#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    fn conflict(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            message: message.into(),
        }
    }
}

impl From<azure_data_cosmos_driver::error::CosmosError> for ApiError {
    fn from(error: azure_data_cosmos_driver::error::CosmosError) -> Self {
        let status = StatusCode::from_u16(u16::from(error.status().status_code()))
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        Self {
            status,
            message: error.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(json!({ "error": self.message }))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{headers::HeaderValue, Method, Request};
    use azure_data_cosmos_driver::{
        in_memory_emulator::{ContainerConfig, VirtualAccountConfig, VirtualRegion},
        models::PartitionKeyDefinition,
    };
    use url::Url;

    #[tokio::test]
    async fn manual_split_and_merge_advance_through_operation_phases() {
        let gateway_url = Url::parse("http://127.0.0.1:18081/").unwrap();
        let account =
            VirtualAccountConfig::new(vec![VirtualRegion::new("East US", gateway_url.clone())])
                .unwrap();
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(account));
        let store = emulator.store();
        store.create_database("testdb");
        let partition_key: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        store.create_container_with_config(
            "testdb",
            "testcoll",
            partition_key,
            ContainerConfig::new()
                .with_partition_count(1)
                .build()
                .unwrap(),
        );
        for (id, pk, value) in [("1", "a", "small"), ("2", "z", "much-larger-value")] {
            let mut request = Request::new(
                gateway_url.join("dbs/testdb/colls/testcoll/docs").unwrap(),
                Method::Post,
            );
            request.headers_mut().insert(
                "x-ms-documentdb-partitionkey",
                HeaderValue::from(format!(r#"["{pk}"]"#)),
            );
            request.set_body(
                serde_json::to_vec(&serde_json::json!({
                    "id": id, "pk": pk, "value": value
                }))
                .unwrap(),
            );
            let response = emulator.execute_request(&request).await.unwrap();
            assert!(response.status().is_success());
        }

        let state = ManagementState {
            emulator,
            account_id: "test-account".into(),
            bindings: Vec::<GatewayBinding>::new().into(),
            metrics: Arc::new(HostMetrics::default()),
            operations: Arc::new(OperationRegistry::default()),
        };
        let response = split_partition(
            State(state.clone()),
            Path(("testdb".to_owned(), "testcoll".to_owned(), 0)),
            Some(Json(SplitRequest {
                mode: SplitMode::Storage,
                epk: None,
                progression_mode: ProgressionMode::Manual,
                lock_duration_ms: None,
            })),
        )
        .await
        .unwrap();

        assert_eq!(response.0, StatusCode::ACCEPTED);
        assert_eq!(response.1["phase"], "Preparing");
        let operation_id = response.1["operationId"].as_str().unwrap().to_owned();

        let swapping = advance_operation(State(state.clone()), Path(operation_id.clone()))
            .await
            .unwrap();
        assert_eq!(swapping["phase"], "Swapping");
        assert!(state
            .emulator
            .store()
            .child_partition_ids("testdb", "testcoll", &[0])
            .is_empty());

        let succeeded = advance_operation(State(state.clone()), Path(operation_id.clone()))
            .await
            .unwrap();
        assert_eq!(succeeded["status"], "Succeeded");
        assert_eq!(succeeded["phase"], "Succeeded");
        assert_eq!(succeeded["children"].as_array().unwrap().len(), 2);
        assert!(!succeeded["splitEpk"].as_str().unwrap().is_empty());

        let queried = get_operation(State(state.clone()), Path(operation_id))
            .await
            .unwrap();
        assert_eq!(queried["phase"], "Succeeded");

        let repeated = split_partition(
            State(state.clone()),
            Path(("testdb".to_owned(), "testcoll".to_owned(), 0)),
            Some(Json(SplitRequest {
                mode: SplitMode::Epk,
                epk: Some(succeeded["splitEpk"].as_str().unwrap().to_owned()),
                progression_mode: ProgressionMode::Automatic,
                lock_duration_ms: None,
            })),
        )
        .await
        .unwrap();
        let repeated_id = repeated.1["operationId"].as_str().unwrap().to_owned();
        let mut repeated_phase = None;
        for _ in 0..100 {
            let operation = get_operation(State(state.clone()), Path(repeated_id.clone()))
                .await
                .unwrap();
            repeated_phase = operation["phase"].as_str().map(str::to_owned);
            if repeated_phase.as_deref() == Some("Failed") {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(repeated_phase.as_deref(), Some("Failed"));

        let children = succeeded["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(|child| child.as_u64().unwrap() as u32)
            .collect::<Vec<_>>();
        let merge = merge_partitions(
            State(state.clone()),
            Path(("testdb".to_owned(), "testcoll".to_owned())),
            Json(MergeRequest {
                partition_ids: [children[0], children[1]],
                progression_mode: ProgressionMode::Manual,
                lock_duration_ms: None,
            }),
        )
        .await
        .unwrap();
        let merge_id = merge.1["operationId"].as_str().unwrap().to_owned();
        let merge_swapping = advance_operation(State(state.clone()), Path(merge_id.clone()))
            .await
            .unwrap();
        assert_eq!(merge_swapping["phase"], "Swapping");
        let merge_succeeded = advance_operation(State(state.clone()), Path(merge_id))
            .await
            .unwrap();
        assert_eq!(merge_succeeded["phase"], "Succeeded");
        assert_eq!(merge_succeeded["merged"].as_array().unwrap().len(), 2);

        let invalid_duration = split_partition(
            State(state),
            Path(("testdb".to_owned(), "testcoll".to_owned(), 3)),
            Some(Json(SplitRequest {
                mode: SplitMode::Midpoint,
                epk: None,
                progression_mode: ProgressionMode::Manual,
                lock_duration_ms: Some(0),
            })),
        )
        .await
        .unwrap_err();
        assert_eq!(invalid_duration.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn custom_epk_rejects_malformed_hex() {
        assert!(parse_epk("ABC").is_err());
        assert!(parse_epk("not-hex").is_err());
    }
}
