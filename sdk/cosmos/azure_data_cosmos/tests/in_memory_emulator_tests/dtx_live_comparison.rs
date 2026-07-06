// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! DTX dual-backend comparison tests.

use std::{error::Error, sync::Arc, time::Duration};

use azure_core::http::StatusCode;
use azure_core::Bytes;
use azure_data_cosmos_driver::{
    models::{
        ContainerReference, DistributedTransactionOperation, DistributedTransactionOperationKind,
        DistributedTransactionRequest, DistributedTransactionResponse,
        DistributedTransactionResultBody, DistributedTransactionTarget, DistributedTransactionType,
        PartitionKey,
    },
    options::OperationOptions,
    CosmosDriver,
};
use serde_json::json;
use tokio::time::sleep;

use super::dual_backend::DualBackend;

#[derive(Debug, PartialEq, Eq)]
struct DtxSnapshot {
    write_status: StatusCode,
    write_op_status: StatusCode,
    read_status: StatusCode,
    read_op_status: StatusCode,
    read_id: String,
    read_value: i64,
    write_has_session_token: bool,
    read_has_session_token: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct MultiContainerDtxSnapshot {
    write_status: StatusCode,
    write_op_statuses: Vec<StatusCode>,
    read_status: StatusCode,
    read_op_statuses: Vec<StatusCode>,
    read_values: Vec<i64>,
    write_has_session_tokens: Vec<bool>,
    read_has_session_tokens: Vec<bool>,
}

struct DtxReadTarget<'a> {
    container: &'a ContainerReference,
    pk: String,
    id: String,
}

#[tokio::test]
#[ignore = "requires AZURE_COSMOS_CONNECTION_STRING for a live account whose front door routes /operations/dtc"]
async fn dtx_create_read_matches_live_account() -> Result<(), Box<dyn Error>> {
    let backend = DualBackend::setup().await?;
    let db_name = format!("{}-dtx", backend.unique_db_name());
    let container_name = "dtx-coll";
    let second_container_name = "dtx-coll-2";
    let pk_path = "/pk";

    backend.provision_emulator(&db_name, container_name, pk_path);
    backend.provision_emulator(&db_name, second_container_name, pk_path);

    if backend.has_real_backend() {
        backend.create_real_database(&db_name).await?;
        backend
            .create_real_container(&db_name, container_name, pk_path)
            .await?;
        backend
            .create_real_container(&db_name, second_container_name, pk_path)
            .await?;
    }

    let result = async {
        let emulator_container =
            resolve_container_with_retry(&backend.emulator_driver, &db_name, container_name)
                .await?;
        let emulator_second_container =
            resolve_container_with_retry(&backend.emulator_driver, &db_name, second_container_name)
                .await?;
        let emulator_snapshot =
            run_create_read_dtx(&backend.emulator_driver, &emulator_container, "emu").await?;
        let emulator_multi_snapshot = run_multi_container_create_read_dtx(
            &backend.emulator_driver,
            &emulator_container,
            &emulator_second_container,
            "emu",
        )
        .await?;

        let Some(real_driver) = backend.real_driver.as_ref() else {
            println!(
                "  [dtx-dual-backend] Real account not configured; emulator-only DTX leg passed"
            );
            return Ok::<(), Box<dyn Error>>(());
        };

        let real_container =
            resolve_container_with_retry(real_driver, &db_name, container_name).await?;
        let real_second_container =
            resolve_container_with_retry(real_driver, &db_name, second_container_name).await?;
        let real_snapshot = run_create_read_dtx(real_driver, &real_container, "real").await?;
        let real_multi_snapshot = run_multi_container_create_read_dtx(
            real_driver,
            &real_container,
            &real_second_container,
            "real",
        )
        .await?;

        compare_snapshots(&real_snapshot, &emulator_snapshot)?;
        compare_multi_container_snapshots(&real_multi_snapshot, &emulator_multi_snapshot)?;
        Ok(())
    }
    .await;

    backend.cleanup_real_database(&db_name).await;
    result
}

async fn resolve_container_with_retry(
    driver: &Arc<CosmosDriver>,
    db_name: &str,
    container_name: &str,
) -> Result<ContainerReference, Box<dyn Error>> {
    let mut last_error = None;
    for _ in 0..10 {
        match driver.resolve_container(db_name, container_name).await {
            Ok(container) => return Ok(container),
            Err(error) => {
                last_error = Some(error);
                sleep(Duration::from_millis(500)).await;
            }
        }
    }

    Err(format!(
        "failed to resolve container {db_name}/{container_name}: {:?}",
        last_error
    )
    .into())
}

async fn run_multi_container_create_read_dtx(
    driver: &Arc<CosmosDriver>,
    first_container: &ContainerReference,
    second_container: &ContainerReference,
    label: &str,
) -> Result<MultiContainerDtxSnapshot, Box<dyn Error>> {
    validate_resolved_container(first_container, label)?;
    validate_resolved_container(second_container, label)?;

    let first_id = format!("dtx-multi-a-{label}");
    let first_pk = format!("dtx-multi-pk-a-{label}");
    let second_id = format!("dtx-multi-b-{label}");
    let second_pk = format!("dtx-multi-pk-b-{label}");
    let first_value = 101_i64;
    let second_value = 202_i64;

    let write_operations = vec![
        dtx_create_operation(
            first_container,
            first_pk.clone(),
            first_id.clone(),
            first_value,
        )?,
        dtx_create_operation(
            second_container,
            second_pk.clone(),
            second_id.clone(),
            second_value,
        )?,
    ];
    let write_response = driver
        .execute_distributed_transaction(
            DistributedTransactionRequest::new(DistributedTransactionType::Write, write_operations),
            OperationOptions::default(),
        )
        .await?;
    ensure_dtx_success(&write_response, label, "multi-container write")?;

    let read_response = read_multi_container_with_retry(
        driver,
        [
            DtxReadTarget {
                container: first_container,
                pk: first_pk,
                id: first_id,
            },
            DtxReadTarget {
                container: second_container,
                pk: second_pk,
                id: second_id,
            },
        ],
        label,
    )
    .await?;
    ensure_dtx_success(&read_response, label, "multi-container read")?;

    Ok(MultiContainerDtxSnapshot {
        write_status: write_response.status_code,
        write_op_statuses: write_response
            .operation_results
            .iter()
            .map(|result| result.status_code)
            .collect(),
        read_status: read_response.status_code,
        read_op_statuses: read_response
            .operation_results
            .iter()
            .map(|result| result.status_code)
            .collect(),
        read_values: vec![
            operation_resource_body_as_json(&read_response, label, 0)?
                .get("value")
                .and_then(|value| value.as_i64())
                .unwrap_or_default(),
            operation_resource_body_as_json(&read_response, label, 1)?
                .get("value")
                .and_then(|value| value.as_i64())
                .unwrap_or_default(),
        ],
        write_has_session_tokens: write_response
            .operation_results
            .iter()
            .map(|result| result.session_token.is_some())
            .collect(),
        read_has_session_tokens: read_response
            .operation_results
            .iter()
            .map(|result| result.session_token.is_some())
            .collect(),
    })
}

async fn read_multi_container_with_retry(
    driver: &Arc<CosmosDriver>,
    targets: [DtxReadTarget<'_>; 2],
    label: &str,
) -> Result<DistributedTransactionResponse, Box<dyn Error>> {
    let mut last_response = None;
    for _ in 0..10 {
        let read_operations = vec![
            DistributedTransactionOperation::new(
                DistributedTransactionOperationKind::Read,
                DistributedTransactionTarget::new(
                    targets[0].container.clone(),
                    PartitionKey::from(targets[0].pk.clone()),
                    targets[0].id.clone(),
                ),
            ),
            DistributedTransactionOperation::new(
                DistributedTransactionOperationKind::Read,
                DistributedTransactionTarget::new(
                    targets[1].container.clone(),
                    PartitionKey::from(targets[1].pk.clone()),
                    targets[1].id.clone(),
                ),
            ),
        ];
        let response = driver
            .execute_distributed_transaction(
                DistributedTransactionRequest::new(
                    DistributedTransactionType::Read,
                    read_operations,
                ),
                OperationOptions::default(),
            )
            .await?;
        if response.status_code.is_success()
            && response
                .operation_results
                .iter()
                .all(|result| result.status_code.is_success())
        {
            return Ok(response);
        }
        last_response = Some(response);
        sleep(Duration::from_millis(500)).await;
    }

    let response = last_response.ok_or("multi-container read did not execute")?;
    Err(format!(
        "{label} multi-container read did not converge: envelope={} ops={:?} diagnostic={:?} error={:?}",
        response.status_code,
        response
            .operation_results
            .iter()
            .map(|result| result.status_code)
            .collect::<Vec<_>>(),
        response.diagnostic_string,
        response.error_message
    )
    .into())
}

fn dtx_create_operation(
    container: &ContainerReference,
    pk: String,
    id: String,
    value: i64,
) -> Result<DistributedTransactionOperation, Box<dyn Error>> {
    let item = json!({"id": id, "pk": pk, "value": value});
    Ok(DistributedTransactionOperation::new(
        DistributedTransactionOperationKind::Create,
        DistributedTransactionTarget::new(container.clone(), PartitionKey::from(pk), id),
    )
    .with_resource_body(Bytes::from(serde_json::to_vec(&item)?)))
}

fn validate_resolved_container(
    container: &ContainerReference,
    label: &str,
) -> Result<(), Box<dyn Error>> {
    if container.rid().is_empty() || container.database_rid().is_empty() {
        return Err(format!(
            "{label} resolved container has empty RID fields: db_rid='{}' coll_rid='{}'",
            container.database_rid(),
            container.rid()
        )
        .into());
    }
    Ok(())
}

async fn run_create_read_dtx(
    driver: &Arc<CosmosDriver>,
    container: &ContainerReference,
    label: &str,
) -> Result<DtxSnapshot, Box<dyn Error>> {
    validate_resolved_container(container, label)?;

    let id = format!("dtx-item-{label}");
    let pk = format!("dtx-pk-{label}");
    let value = 42_i64;
    let item = json!({"id": id, "pk": pk, "value": value});

    let write_operation = DistributedTransactionOperation::new(
        DistributedTransactionOperationKind::Create,
        DistributedTransactionTarget::new(
            container.clone(),
            PartitionKey::from(pk.clone()),
            id.clone(),
        ),
    )
    .with_resource_body(Bytes::from(serde_json::to_vec(&item)?));
    let write_request = DistributedTransactionRequest::new(
        DistributedTransactionType::Write,
        vec![write_operation],
    );
    let write_response = driver
        .execute_distributed_transaction(write_request, OperationOptions::default())
        .await?;

    ensure_dtx_success(&write_response, label, "write")?;
    let write_op = write_response
        .operation_results
        .first()
        .ok_or("write DTX response did not contain an operation result")?;

    let read_operation = DistributedTransactionOperation::new(
        DistributedTransactionOperationKind::Read,
        DistributedTransactionTarget::new(container.clone(), PartitionKey::from(pk), id.clone()),
    );
    let read_request =
        DistributedTransactionRequest::new(DistributedTransactionType::Read, vec![read_operation]);
    let read_response = driver
        .execute_distributed_transaction(read_request, OperationOptions::default())
        .await?;
    ensure_dtx_success(&read_response, label, "read")?;
    let read_op = read_response
        .operation_results
        .first()
        .ok_or("read DTX response did not contain an operation result")?;
    let resource = resource_body_as_json(&read_response, label)?;

    Ok(DtxSnapshot {
        write_status: write_response.status_code,
        write_op_status: write_op.status_code,
        read_status: read_response.status_code,
        read_op_status: read_op.status_code,
        read_id: resource
            .get("id")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned(),
        read_value: resource
            .get("value")
            .and_then(|value| value.as_i64())
            .unwrap_or_default(),
        write_has_session_token: write_op.session_token.is_some(),
        read_has_session_token: read_op.session_token.is_some(),
    })
}

fn ensure_dtx_success(
    response: &DistributedTransactionResponse,
    label: &str,
    operation: &str,
) -> Result<(), Box<dyn Error>> {
    if response.status_code.is_success()
        && response
            .operation_results
            .iter()
            .all(|result| result.status_code.is_success())
    {
        return Ok(());
    }

    Err(format!(
        "{label} {operation} DTX failed: envelope={} ops={:?} diagnostic={:?} error={:?}",
        response.status_code,
        response
            .operation_results
            .iter()
            .map(|result| result.status_code)
            .collect::<Vec<_>>(),
        response.diagnostic_string,
        response.error_message
    )
    .into())
}

fn resource_body_as_json(
    response: &DistributedTransactionResponse,
    label: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let read_op = response
        .operation_results
        .first()
        .ok_or("response did not contain an operation result")?;
    operation_resource_body(read_op, response, label)
}

fn operation_resource_body_as_json(
    response: &DistributedTransactionResponse,
    label: &str,
    index: usize,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let read_op = response
        .operation_results
        .get(index)
        .ok_or("response did not contain the requested operation result")?;
    operation_resource_body(read_op, response, label)
}

fn operation_resource_body(
    read_op: &azure_data_cosmos_driver::models::DistributedTransactionOperationResult,
    response: &DistributedTransactionResponse,
    label: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    match &read_op.resource_body {
        DistributedTransactionResultBody::Bytes(bytes) => Ok(serde_json::from_slice(bytes)?),
        DistributedTransactionResultBody::None => Err(format!(
            "{label} read DTX response did not contain a resource body: envelope={} op={} diagnostic={:?} error={:?}",
            response.status_code, read_op.status_code, response.diagnostic_string, response.error_message
        )
        .into()),
        _ => Err(format!(
            "{label} read DTX response used an unsupported resource body variant"
        )
        .into()),
    }
}

fn compare_multi_container_snapshots(
    real: &MultiContainerDtxSnapshot,
    emulator: &MultiContainerDtxSnapshot,
) -> Result<(), Box<dyn Error>> {
    if real.write_status != emulator.write_status {
        return Err(format!(
            "multi-container write envelope status mismatch: real={} emulator={}",
            real.write_status, emulator.write_status
        )
        .into());
    }
    if real.write_op_statuses != emulator.write_op_statuses {
        return Err(format!(
            "multi-container write op status mismatch: real={:?} emulator={:?}",
            real.write_op_statuses, emulator.write_op_statuses
        )
        .into());
    }
    if real.read_status != emulator.read_status {
        return Err(format!(
            "multi-container read envelope status mismatch: real={} emulator={}",
            real.read_status, emulator.read_status
        )
        .into());
    }
    if real.read_op_statuses != emulator.read_op_statuses {
        return Err(format!(
            "multi-container read op status mismatch: real={:?} emulator={:?}",
            real.read_op_statuses, emulator.read_op_statuses
        )
        .into());
    }
    if real.read_values != emulator.read_values {
        return Err(format!(
            "multi-container read values mismatch: real={:?} emulator={:?}",
            real.read_values, emulator.read_values
        )
        .into());
    }
    if real.write_has_session_tokens != emulator.write_has_session_tokens {
        return Err(format!(
            "multi-container write session-token mismatch: real={:?} emulator={:?}",
            real.write_has_session_tokens, emulator.write_has_session_tokens
        )
        .into());
    }
    if real.read_has_session_tokens != emulator.read_has_session_tokens {
        return Err(format!(
            "multi-container read session-token mismatch: real={:?} emulator={:?}",
            real.read_has_session_tokens, emulator.read_has_session_tokens
        )
        .into());
    }
    Ok(())
}

fn compare_snapshots(real: &DtxSnapshot, emulator: &DtxSnapshot) -> Result<(), Box<dyn Error>> {
    if real.write_status != emulator.write_status {
        return Err(format!(
            "write envelope status mismatch: real={} emulator={}",
            real.write_status, emulator.write_status
        )
        .into());
    }
    if real.write_op_status != emulator.write_op_status {
        return Err(format!(
            "write op status mismatch: real={} emulator={}",
            real.write_op_status, emulator.write_op_status
        )
        .into());
    }
    if real.read_status != emulator.read_status {
        return Err(format!(
            "read envelope status mismatch: real={} emulator={}",
            real.read_status, emulator.read_status
        )
        .into());
    }
    if real.read_op_status != emulator.read_op_status {
        return Err(format!(
            "read op status mismatch: real={} emulator={}",
            real.read_op_status, emulator.read_op_status
        )
        .into());
    }
    if real.read_value != emulator.read_value {
        return Err(format!(
            "read value mismatch: real={} emulator={}",
            real.read_value, emulator.read_value
        )
        .into());
    }
    if !real.write_has_session_token || !emulator.write_has_session_token {
        return Err(
            "write DTX response must include per-operation session token on both backends".into(),
        );
    }
    if !real.read_has_session_token || !emulator.read_has_session_token {
        return Err(
            "read DTX response must include per-operation session token on both backends".into(),
        );
    }
    Ok(())
}
