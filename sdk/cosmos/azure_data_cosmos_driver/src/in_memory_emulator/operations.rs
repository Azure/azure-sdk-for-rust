// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point operation and control-plane operation handlers.

// cspell:ignore acked hexdigit llsn

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use azure_core::http::headers::{HeaderName, HeaderValue, Headers};
use azure_core::http::{AsyncRawResponse, StatusCode};
use serde::{Deserialize, Serialize};

use super::config::ContainerConfig;
use super::dispatch::{OperationType, ParsedRequest};
use super::epk::{compute_epk, extract_pk_from_body, parse_partition_key_header, Epk};
use super::response::headers::{
    ACTIVITY_ID, CONTINUATION, GLOBAL_COMMITTED_LSN, INTERNAL_PARTITION_ID, ITEM_LOCAL_LSN,
    ITEM_LSN, LAST_STATE_CHANGE_UTC, LOCAL_LSN, NUMBER_OF_READ_REGIONS, PARTITION_KEY_RANGE_ID,
    QUORUM_ACKED_LOCAL_LSN, QUORUM_ACKED_LSN, RESOURCE_QUOTA, RESOURCE_USAGE, SERVICE_VERSION,
    TRANSPORT_REQUEST_ID,
};
#[cfg(feature = "preview_dtx")]
use super::response::headers::{ETAG, REQUEST_CHARGE, SESSION_TOKEN, SUBSTATUS};
use super::response::{error_response, success_response, ResponseBuilder};
use super::ru_model::RuChargingModel;
use super::session::SessionToken;
use super::store::{
    current_timestamp, new_etag, ContainerMetadata, EmulatorStore, PhysicalPartition,
    StoredDocument,
};
use super::system_properties::{
    account_properties_to_json, container_to_json, database_to_json, feed_to_json,
    inject_system_properties, offer_to_json, pkranges_to_json,
};
#[cfg(feature = "preview_dtx")]
use crate::driver::pipeline::patch_eval::apply_patch_ops;
#[cfg(feature = "preview_dtx")]
use crate::models::PatchInstructions;
use crate::models::{
    EffectivePartitionKey, PartitionKeyDefinition, PartitionKeyValue as ModelPartitionKeyValue,
};
use crate::query::ast::{
    SqlCollection, SqlCollectionExpression, SqlQuery, SqlScalarExpression, SqlSelectSpec,
};

static OFFER_REPLACE_PENDING: HeaderName = HeaderName::from_static("x-ms-offer-replace-pending");

#[cfg(feature = "preview_dtx")]
static DTX_IDEMPOTENCY_TOKEN: HeaderName =
    HeaderName::from_static(crate::models::request_header_names::DTX_IDEMPOTENCY_TOKEN);
#[cfg(feature = "preview_dtx")]
static DTX_OPERATION_TYPE: HeaderName =
    HeaderName::from_static(crate::models::request_header_names::DTX_OPERATION_TYPE);
#[cfg(feature = "preview_dtx")]
static DTX_RESOURCE_TYPE: HeaderName =
    HeaderName::from_static(crate::models::request_header_names::DTX_RESOURCE_TYPE);

/// Sub-status paired with `410 Gone` when a physical partition is locked because
/// a split or merge is in progress.
const PARTITION_SPLIT_OR_MERGE_SUBSTATUS: u16 = 1007;

/// HTTP status a prepared-then-rolled-back write operation reports in an aborted
/// distributed transaction, paired with sub-status 5415 (DtcOperationRolledBack).
/// Mirrors the driver's `SubStatusCode::DTC_OPERATION_ROLLED_BACK`.
#[cfg(feature = "preview_dtx")]
const DTX_ROLLED_BACK_STATUS: u16 = 453;
/// Sub-status accompanying [`DTX_ROLLED_BACK_STATUS`] (DtcOperationRolledBack).
#[cfg(feature = "preview_dtx")]
const DTX_ROLLED_BACK_SUBSTATUS: u32 = 5415;
/// Sub-status paired with `412 PreconditionFailed` when a distributed
/// transaction patch operation's `condition` (filter predicate) is not met.
#[cfg(feature = "preview_dtx")]
const DTX_PATCH_CONDITION_NOT_MET_SUBSTATUS: u16 = 1110;

/// If any non-source target region's replication queue is saturated, returns
/// a 429/3075 error response so callers can short-circuit before committing.
fn replication_back_pressure_response(
    store: &EmulatorStore,
    region_name: &str,
    start: Instant,
) -> Option<AsyncRawResponse> {
    let (target, retry_ms) = store.find_overflowed_replication_target(region_name)?;
    Some(
        error_response(
            StatusCode::TooManyRequests,
            Some(3075),
            "TooManyRequests",
            &format!(
                "Replication queue for target region '{}' is saturated; the source must back off and retry.",
                target
            ),
            0.0,
            "",
            start,
        )
        .with_retry_after_ms(retry_ms)
        .build(),
    )
}

/// Post-processes a dispatched response to stamp the per-request `x-ms-activity-id`
/// (echoed from the request when present) and to ensure every response carries a
/// monotonic `x-ms-transport-request-id`.
///
/// `ResponseBuilder::new` no longer pre-seeds `x-ms-transport-request-id`; point-op
/// handlers stamp it from `store.next_transport_request_id()` via
/// `decorate_point_response`, and any response that reaches this post-processor
/// without one (control-plane, error, unsupported) gets stamped here from the same
/// store counter. The `if absent` check avoids double-incrementing for point ops.
async fn finalize_response(
    store: &Arc<EmulatorStore>,
    response: AsyncRawResponse,
    activity_id: Option<&str>,
) -> AsyncRawResponse {
    let raw = response
        .try_into_raw_response()
        .await
        .expect("emulator responses are always buffered; streaming responses are not produced by this emulator");
    let mut headers = raw.headers().clone();
    if let Some(activity_id) = activity_id {
        headers.insert(
            ACTIVITY_ID.clone(),
            HeaderValue::from(activity_id.to_string()),
        );
    }
    if headers.get_optional_str(&TRANSPORT_REQUEST_ID).is_none() {
        headers.insert(
            TRANSPORT_REQUEST_ID.clone(),
            HeaderValue::from(store.next_transport_request_id().to_string()),
        );
    }
    AsyncRawResponse::from_bytes(raw.status(), headers, raw.body().as_ref().to_vec())
}

/// Dispatches a parsed request to the appropriate handler.
pub(crate) async fn handle_operation(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_headers: &Headers,
    request_body: &[u8],
) -> AsyncRawResponse {
    let start = Instant::now();
    let response = match &parsed.operation {
        OperationType::ReadAccount => handle_read_account(store, start),
        OperationType::CreateDatabase => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_create_database(store, region_name, parsed, request_body, start).await
        }
        OperationType::ReadDatabase => handle_read_database(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            start,
        ),
        OperationType::DeleteDatabase => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_delete_database(
                store,
                region_name,
                parsed.db_id.as_deref().unwrap_or(""),
                start,
            )
        }
        OperationType::CreateContainer => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_create_container(
                store,
                region_name,
                parsed.db_id.as_deref().unwrap_or(""),
                parsed,
                request_body,
                start,
            )
            .await
        }
        OperationType::ReadContainer => handle_read_container(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed.coll_id.as_deref().unwrap_or(""),
            start,
        ),
        OperationType::DeleteContainer => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_delete_container(
                store,
                region_name,
                parsed.db_id.as_deref().unwrap_or(""),
                parsed.coll_id.as_deref().unwrap_or(""),
                start,
            )
        }
        OperationType::ReadPKRanges => handle_read_pkranges(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed.coll_id.as_deref().unwrap_or(""),
            parsed.if_none_match.as_deref(),
            start,
        ),
        OperationType::ReadFeedDatabases => {
            handle_read_feed_databases(store, region_name, parsed, start)
        }
        OperationType::ReadFeedContainers => {
            handle_read_feed_containers(store, region_name, parsed, start)
        }
        OperationType::ReadFeedItems => handle_read_feed_items(store, region_name, parsed, start),
        OperationType::Create => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_create(store, region_name, parsed, request_body, start).await
        }
        OperationType::Read => handle_read(store, region_name, parsed, start),
        OperationType::Replace => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_replace(store, region_name, parsed, request_body, start).await
        }
        OperationType::Upsert => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_upsert(store, region_name, parsed, request_body, start).await
        }
        OperationType::Delete => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_delete(store, region_name, parsed, start).await
        }
        OperationType::QueryDatabases => {
            handle_query_databases(store, region_name, parsed, request_body, start)
        }
        OperationType::QueryContainers => {
            handle_query_containers(store, region_name, parsed, request_body, start)
        }
        OperationType::QueryItems => {
            handle_query_items(store, region_name, parsed, request_body, start)
        }
        OperationType::QueryPlan => {
            handle_query_plan(store, region_name, parsed, request_body, start)
        }
        OperationType::Batch => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_batch(store, region_name, parsed, request_body, start).await
        }
        OperationType::ReadFeedOffers => handle_read_feed_offers(store, region_name, parsed, start),
        OperationType::QueryOffers => {
            handle_query_offers(store, region_name, parsed, request_body, start)
        }
        OperationType::ReadOffer => handle_read_offer(store, region_name, parsed, start),
        OperationType::ReplaceOffer => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response(start);
            }
            handle_replace_offer(store, region_name, parsed, request_body, start)
        }
        #[cfg(feature = "preview_dtx")]
        OperationType::DistributedTransaction => {
            handle_distributed_transaction(store, region_name, request_headers, request_body, start)
                .await
        }
        OperationType::BadRequestPath(desc) => bad_request_path_response(desc, start),
        OperationType::InvalidInput(desc) => invalid_input_response(desc, start),
        OperationType::Unsupported(desc) => unsupported_response(desc, start),
    };

    #[cfg(feature = "preview_dtx")]
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DtxRequestBody {
        operations: Vec<DtxOperation>,
    }

    #[cfg(feature = "preview_dtx")]
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DtxOperation {
        index: usize,
        database_name: String,
        collection_name: String,
        id: String,
        partition_key: serde_json::Value,
        operation_type: String,
        #[serde(default)]
        resource_body: Option<serde_json::Value>,
        #[serde(default)]
        session_token: Option<String>,
        #[serde(default)]
        if_match: Option<String>,
        #[serde(default)]
        if_none_match: Option<String>,
    }

    #[cfg(feature = "preview_dtx")]
    async fn handle_distributed_transaction(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        request_headers: &Headers,
        request_body: &[u8],
        start: Instant,
    ) -> AsyncRawResponse {
        let Some(transaction_type) = validate_dtx_headers(request_headers) else {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Distributed transaction request is missing required DTX headers",
                0.0,
                "",
                start,
            )
            .build();
        };
        let request: DtxRequestBody = match serde_json::from_slice(request_body) {
            Ok(request) => request,
            Err(error) => {
                return error_response(
                    StatusCode::BadRequest,
                    None,
                    "BadRequest",
                    &format!("Invalid distributed transaction JSON body: {error}"),
                    0.0,
                    "",
                    start,
                )
                .build()
            }
        };

        if request.operations.is_empty() {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Distributed transaction requires at least one operation",
                0.0,
                "",
                start,
            )
            .build();
        }

        if let Err(message) = validate_dtx_operation_indexes(&request.operations) {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                &message,
                0.0,
                "",
                start,
            )
            .build();
        }

        match transaction_type {
            DtxTransactionKind::Write => {
                if !is_dtx_write_transaction(&request.operations) {
                    return error_response(
                        StatusCode::BadRequest,
                        None,
                        "BadRequest",
                        "Distributed transaction CommitDistributedTransaction header requires at least one write operation",
                        0.0,
                        "",
                        start,
                    )
                    .build();
                }
                // A write transaction can only commit in the account's write
                // region. Normal writes enforce this in the dispatch layer
                // (`handle_operation`), which the DTX path bypasses by calling
                // the `_locked` point handlers directly, so re-check it here to
                // match real-account (and normal emulator) behavior.
                if !store.config().is_write_region(region_name) {
                    return write_forbidden_response(start);
                }
                handle_dtx_write_transaction(store, region_name, &request.operations, start).await
            }
            DtxTransactionKind::Read => {
                if is_dtx_write_transaction(&request.operations) {
                    return error_response(
                        StatusCode::BadRequest,
                        None,
                        "BadRequest",
                        "Distributed transaction Read header cannot contain write operations",
                        0.0,
                        "",
                        start,
                    )
                    .build();
                }
                handle_dtx_read_transaction(store, region_name, &request.operations, start).await
            }
        }
    }

    #[cfg(feature = "preview_dtx")]
    enum DtxTransactionKind {
        Write,
        Read,
    }

    #[cfg(feature = "preview_dtx")]
    fn validate_dtx_headers(headers: &Headers) -> Option<DtxTransactionKind> {
        let token = headers.get_optional_str(&DTX_IDEMPOTENCY_TOKEN)?;
        if token.trim().is_empty() || uuid::Uuid::parse_str(token).is_err() {
            return None;
        }
        let resource_type = headers.get_optional_str(&DTX_RESOURCE_TYPE)?;
        if !resource_type
            .eq_ignore_ascii_case(crate::models::cosmos_headers::DTX_RESOURCE_TYPE_HEADER_VALUE)
        {
            return None;
        }
        match headers.get_optional_str(&DTX_OPERATION_TYPE)? {
            value if value.eq_ignore_ascii_case("CommitDistributedTransaction") => {
                Some(DtxTransactionKind::Write)
            }
            value if value.eq_ignore_ascii_case("Read") => Some(DtxTransactionKind::Read),
            _ => None,
        }
    }

    #[cfg(feature = "preview_dtx")]
    fn validate_dtx_operation_indexes(operations: &[DtxOperation]) -> Result<(), String> {
        let mut seen = vec![false; operations.len()];
        for (position, operation) in operations.iter().enumerate() {
            if operation.index >= operations.len() {
                return Err(format!(
                    "Distributed transaction operation index {} is out of range for {} operations",
                    operation.index,
                    operations.len()
                ));
            }
            if operation.index != position {
                return Err(format!(
                    "Distributed transaction operation index {} does not match request position {}",
                    operation.index, position
                ));
            }
            if std::mem::replace(&mut seen[operation.index], true) {
                return Err(format!(
                    "Distributed transaction operation index {} is duplicated",
                    operation.index
                ));
            }
        }
        Ok(())
    }

    /// Per-operation outcome captured from a nested point-operation response.
    #[cfg(feature = "preview_dtx")]
    struct DtxOpOutcome {
        status: StatusCode,
        sub_status: Option<u32>,
        etag: Option<String>,
        session_token: Option<String>,
        pk_range_id: Option<String>,
        local_lsn: Option<u64>,
        request_charge: f64,
        resource_body: Option<serde_json::Value>,
    }

    #[cfg(feature = "preview_dtx")]
    async fn dtx_point_outcome(response: AsyncRawResponse) -> DtxOpOutcome {
        let raw = match response.try_into_raw_response().await {
            Ok(raw) => raw,
            // Emulator responses are always buffered, so a failure here is an
            // internal invariant violation, not malformed input. Synthesize a
            // 500 outcome rather than panicking inside the request handler.
            Err(_) => {
                return DtxOpOutcome {
                    status: StatusCode::InternalServerError,
                    sub_status: None,
                    etag: None,
                    session_token: None,
                    pk_range_id: None,
                    local_lsn: None,
                    request_charge: 1.0,
                    resource_body: None,
                }
            }
        };
        let status = raw.status();
        let headers = raw.headers().clone();
        let body_bytes = raw.body().as_ref();
        let sub_status = headers
            .get_optional_str(&SUBSTATUS)
            .and_then(|value| value.parse::<u32>().ok());
        let etag = headers.get_optional_str(&ETAG).map(str::to_owned);
        let session_token = headers.get_optional_str(&SESSION_TOKEN).map(str::to_owned);
        let pk_range_id = headers
            .get_optional_str(&PARTITION_KEY_RANGE_ID)
            .map(str::to_owned);
        let local_lsn = headers
            .get_optional_str(&LOCAL_LSN)
            .and_then(|value| value.parse::<u64>().ok());
        let request_charge = headers
            .get_optional_str(&REQUEST_CHARGE)
            .and_then(|value| value.parse::<f64>().ok())
            .unwrap_or(1.0);
        // Capture the resource body only for successful operations. On a
        // non-success status the body is an error envelope; using a field-name
        // heuristic (e.g. a top-level `code`) would wrongly strip valid user
        // documents that happen to contain that field.
        let resource_body = if status.is_success() && !body_bytes.is_empty() {
            serde_json::from_slice::<serde_json::Value>(body_bytes).ok()
        } else {
            None
        };
        DtxOpOutcome {
            status,
            sub_status,
            etag,
            session_token,
            pk_range_id,
            local_lsn,
            request_charge,
            resource_body,
        }
    }

    /// Serializes a single per-operation result into the `.NET`-shaped wire
    /// object consumed by `DistributedTransactionResponse::from_body`.
    #[cfg(feature = "preview_dtx")]
    #[allow(clippy::too_many_arguments)]
    fn dtx_op_json(
        index: usize,
        status: StatusCode,
        sub_status: Option<u32>,
        etag: Option<&str>,
        session_token: Option<&str>,
        pk_range_id: Option<&str>,
        local_lsn: Option<u64>,
        request_charge: f64,
        resource_body: Option<&serde_json::Value>,
    ) -> serde_json::Value {
        let mut result = serde_json::Map::new();
        result.insert("index".to_owned(), serde_json::json!(index));
        result.insert(
            "statusCode".to_owned(),
            serde_json::json!(u16::from(status)),
        );
        result.insert(
            "subStatusCode".to_owned(),
            serde_json::json!(sub_status.unwrap_or_default()),
        );
        result.insert("isRetriable".to_owned(), serde_json::json!(false));
        if let Some(etag) = etag {
            result.insert("eTag".to_owned(), serde_json::json!(etag));
        }
        if let Some(session_token) = session_token {
            result.insert("sessionToken".to_owned(), serde_json::json!(session_token));
        }
        if let Some(pk_range_id) = pk_range_id {
            result.insert(
                "partitionKeyRangeId".to_owned(),
                serde_json::json!(pk_range_id),
            );
        }
        if let Some(local_lsn) = local_lsn {
            result.insert("localLsn".to_owned(), serde_json::json!(local_lsn));
        }
        result.insert(
            "requestCharge".to_owned(),
            serde_json::json!(request_charge),
        );
        if let Some(resource_body) = resource_body {
            result.insert("resourceBody".to_owned(), resource_body.clone());
        }
        serde_json::Value::Object(result)
    }

    #[cfg(feature = "preview_dtx")]
    async fn execute_dtx_point_operation(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        operation: &DtxOperation,
        start: Instant,
    ) -> AsyncRawResponse {
        if operation.operation_type.eq_ignore_ascii_case("Patch") {
            return handle_dtx_patch_operation(store, region_name, operation, start).await;
        }

        let operation_type = match operation.operation_type.as_str() {
            "Create" => OperationType::Create,
            "Read" => OperationType::Read,
            "Replace" => OperationType::Replace,
            "Upsert" => OperationType::Upsert,
            "Delete" => OperationType::Delete,
            other => {
                return error_response(
                    StatusCode::BadRequest,
                    None,
                    "BadRequest",
                    &format!("Unsupported DTX operation type '{other}'"),
                    0.0,
                    "",
                    start,
                )
                .build()
            }
        };

        let point_body = match operation.resource_body.as_ref().map(serde_json::to_vec) {
            None => Vec::new(),
            Some(Ok(body)) => body,
            Some(Err(error)) => {
                return error_response(
                    StatusCode::BadRequest,
                    None,
                    "BadRequest",
                    &format!("Failed to serialize DTX operation resource body: {error}"),
                    0.0,
                    "",
                    start,
                )
                .build()
            }
        };
        let parsed = ParsedRequest {
            operation: operation_type.clone(),
            db_id: Some(operation.database_name.clone()),
            coll_id: Some(operation.collection_name.clone()),
            doc_id: Some(operation.id.clone()),
            offer_id: None,
            partition_key_header: Some(operation.partition_key.to_string()),
            if_match: operation.if_match.clone(),
            if_none_match: operation.if_none_match.clone(),
            session_token: operation.session_token.clone(),
            activity_id: None,
            content_response_on_write: true,
            offer_throughput: None,
            offer_autopilot_settings: None,
            max_item_count: None,
            continuation: None,
            partition_key_range_id: None,
            start_epk: None,
            end_epk: None,
            is_query_plan: false,
            is_batch: false,
            is_upsert: matches!(operation_type, OperationType::Upsert),
        };

        match operation_type {
            OperationType::Create => {
                handle_create_locked(store, region_name, &parsed, &point_body, start).await
            }
            OperationType::Read => handle_read(store, region_name, &parsed, start),
            OperationType::Replace => {
                handle_replace_locked(store, region_name, &parsed, &point_body, start).await
            }
            OperationType::Upsert => {
                handle_upsert_locked(store, region_name, &parsed, &point_body, start).await
            }
            OperationType::Delete => handle_delete_locked(store, region_name, &parsed, start).await,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "preview_dtx")]
    async fn handle_dtx_patch_operation(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        operation: &DtxOperation,
        start: Instant,
    ) -> AsyncRawResponse {
        let Some(resource_body) = operation.resource_body.as_ref() else {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "DTX Patch operation requires a resourceBody",
                0.0,
                "",
                start,
            )
            .build();
        };
        let (patch, condition) = match parse_dtx_patch_body(resource_body) {
            Ok(parsed) => parsed,
            Err(message) => {
                return error_response(
                    StatusCode::BadRequest,
                    None,
                    "BadRequest",
                    &message,
                    0.0,
                    "",
                    start,
                )
                .build()
            }
        };

        let db_id = &operation.database_name;
        let coll_id = &operation.collection_name;
        let region_ref = match store.region(region_name) {
            Some(region_ref) => region_ref,
            None => return not_found_region(start),
        };
        if !region_ref.database_exists(db_id) {
            return error_response(
                StatusCode::NotFound,
                None,
                "NotFound",
                &format!("Database '{}' does not exist", operation.database_name),
                0.0,
                "",
                start,
            )
            .build();
        }

        let parsed = dtx_operation_as_parsed_request(operation);
        let result = region_ref.with_container(db_id, coll_id, |state| {
            let empty_body = serde_json::Value::Null;
            let (_, epk) = match resolve_partition_key(&parsed, &empty_body, &state.metadata) {
                Ok(value) => value,
                Err(error) => return Err(bad_partition_key_response(error, start)),
            };
            let partition = match state.find_partition(&epk) {
                Some(partition) => partition,
                None => {
                    return Err(error_response(
                        StatusCode::InternalServerError,
                        None,
                        "InternalError",
                        "No partition found for EPK",
                        1.0,
                        "",
                        start,
                    )
                    .build())
                }
            };
            if let Some(response) = check_partition_lock(partition, start) {
                return Err(response);
            }

            let charge = 1.0;
            let region_id = store.config().region_id_for(region_name);
            let new_doc = {
                let mut docs = partition.documents.write().unwrap();
                let logical = docs.entry(epk.clone()).or_default();
                let Some(current) = logical.get(&operation.id).cloned() else {
                    let token = session_token_for(
                        partition,
                        region_id,
                        incoming_session_for(&parsed, partition.id).as_ref(),
                    );
                    return Err(error_response(
                        StatusCode::NotFound,
                        None,
                        "NotFound",
                        &format!(
                            "Entity with the specified id does not exist in the system. ResourceId: {}",
                            operation.id
                        ),
                        1.0,
                        &token,
                        start,
                    )
                    .build());
                };

                if operation
                    .if_match
                    .as_ref()
                    .is_some_and(|etag| etag != &current.etag)
                {
                    let token = session_token_for(
                        partition,
                        region_id,
                        incoming_session_for(&parsed, partition.id).as_ref(),
                    );
                    return Err(error_response(
                        StatusCode::PreconditionFailed,
                        None,
                        "PreconditionFailed",
                        "One of the specified pre-condition is not met.",
                        1.0,
                        &token,
                        start,
                    )
                    .build());
                }

                match dtx_patch_condition_matches(condition.as_deref(), &current.body) {
                    Ok(true) => {}
                    Ok(false) => {
                        let token = session_token_for(
                            partition,
                            region_id,
                            incoming_session_for(&parsed, partition.id).as_ref(),
                        );
                        return Err(error_response(
                            StatusCode::PreconditionFailed,
                            Some(DTX_PATCH_CONDITION_NOT_MET_SUBSTATUS.into()),
                            "PreconditionFailed",
                            "Patch condition was not met.",
                            1.0,
                            &token,
                            start,
                        )
                        .build());
                    }
                    Err(message) => {
                        return Err(error_response(
                            StatusCode::BadRequest,
                            None,
                            "BadRequest",
                            &message,
                            1.0,
                            "",
                            start,
                        )
                        .build())
                    }
                }

                if let Some(response) =
                    check_throttle(partition, charge, store.config().throttling_enabled(), start)
                {
                    return Err(response);
                }

                let mut patched_body = current.body.clone();
                if let Err(error) = apply_patch_ops(&mut patched_body, &patch.operations) {
                    return Err(error_response(
                        StatusCode::BadRequest,
                        None,
                        "BadRequest",
                        &error.to_string(),
                        1.0,
                        "",
                        start,
                    )
                    .build());
                }

                let lsn = partition.advance_lsn();
                partition.advance_local_lsn();
                let ts = current_timestamp();
                let etag = new_etag();
                inject_system_properties(&current.rid, &current.self_link, &etag, ts, &mut patched_body);
                let body_size_bytes = serde_json::to_vec(&patched_body).map_or(0, |bytes| bytes.len());
                let new_doc = StoredDocument {
                    body: patched_body.clone(),
                    id: operation.id.clone(),
                    rid: current.rid,
                    etag: etag.clone(),
                    ts,
                    self_link: current.self_link,
                    lsn,
                    epk: epk.clone(),
                    body_size_bytes,
                    source_region: region_name.to_string(),
                };
                logical.insert(operation.id.clone(), new_doc.clone());
                new_doc
            };

            let token = session_token_for(
                partition,
                region_id,
                incoming_session_for(&parsed, partition.id).as_ref(),
            );
            let headers = Some(PointResponseHeaders::from_partition(
                partition,
                store.next_transport_request_id(),
            ));
            Ok((new_doc, token, charge, headers))
        });

        match result {
            Some(Ok((doc, token, charge, headers))) => {
                store.replicate(region_name, db_id, coll_id, &doc, false);
                let builder = success_response(StatusCode::Ok, &doc.body, charge, &token, start);
                decorate_point_response(builder, headers, Some(doc.lsn)).build()
            }
            Some(Err(response)) => response,
            None => container_not_found(db_id, coll_id, start),
        }
    }

    #[cfg(feature = "preview_dtx")]
    fn parse_dtx_patch_body(
        resource_body: &serde_json::Value,
    ) -> Result<(PatchInstructions, Option<String>), String> {
        let mut body = resource_body.clone();
        let condition = match body.as_object_mut().and_then(|map| map.remove("condition")) {
            Some(serde_json::Value::String(condition)) if !condition.trim().is_empty() => {
                Some(condition)
            }
            Some(serde_json::Value::String(_)) => None,
            Some(_) => return Err("DTX patch condition must be a string".to_owned()),
            None => None,
        };
        let patch = serde_json::from_value::<PatchInstructions>(body)
            .map_err(|error| format!("invalid DTX patch resourceBody: {error}"))?;
        Ok((patch, condition))
    }

    #[cfg(feature = "preview_dtx")]
    fn dtx_patch_condition_matches(
        condition: Option<&str>,
        document: &serde_json::Value,
    ) -> Result<bool, String> {
        let Some(condition) = condition else {
            return Ok(true);
        };
        let sql = if condition
            .trim_start()
            .to_ascii_lowercase()
            .starts_with("from ")
        {
            format!("SELECT * {condition}")
        } else {
            condition.to_owned()
        };
        let program = crate::query::parse(&sql)
            .map_err(|error| format!("invalid DTX patch condition: {error}"))?;
        crate::query::eval::matches_query(document, &program.query, &[])
            .map_err(|error| format!("failed to evaluate DTX patch condition: {error}"))
    }

    #[cfg(feature = "preview_dtx")]
    fn is_dtx_write_transaction(operations: &[DtxOperation]) -> bool {
        operations
            .iter()
            .any(|operation| !operation.operation_type.eq_ignore_ascii_case("Read"))
    }

    /// Executes a write (or mixed read/write) distributed transaction with
    /// two-phase-commit semantics: every operation is validated ("prepared")
    /// before any mutation is applied, and a runtime failure during commit
    /// rolls back every already-applied mutation.
    ///
    /// Isolation note: the prepare, commit, and rollback phases acquire the
    /// partition lock per operation rather than holding a transaction-wide lock,
    /// so the emulator's DTX atomicity guarantee assumes a single writer per
    /// partition at a time (as in the test harness). It is not isolated against
    /// concurrent writers mutating the same partition mid-transaction.
    #[cfg(feature = "preview_dtx")]
    async fn handle_dtx_write_transaction(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        operations: &[DtxOperation],
        start: Instant,
    ) -> AsyncRawResponse {
        let write_lock = store.document_write_lock();
        let _write_guard = write_lock.lock().await;
        handle_dtx_write_transaction_locked(store, region_name, operations, start).await
    }

    #[cfg(feature = "preview_dtx")]
    async fn handle_dtx_write_transaction_locked(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        operations: &[DtxOperation],
        start: Instant,
    ) -> AsyncRawResponse {
        // Phase 1 (prepare): every participant votes. Any "No" vote (a validation
        // failure such as a conflict or failed pre-condition) aborts the whole
        // transaction before a single mutation is applied.
        let votes: Vec<Option<DtxPreflightFailure>> = operations
            .iter()
            .map(|operation| preflight_dtx_write_operation(store, region_name, operation).err())
            .collect();
        if votes.iter().any(Option::is_some) {
            return dtx_write_abort_response(operations, &votes, start);
        }

        // Buffer replication for the duration of the commit so a rollback can
        // discard replicas that were never durably committed. The buffer is
        // replayed on success and dropped on abort (below), and is safe because
        // this path holds `document_write_lock` for the whole transaction.
        store.begin_dtx_replication_capture();

        // Phase 2 (commit): apply each operation, capturing a pre-image first so a
        // runtime failure (e.g. throttling) can roll back every mutation that was
        // already applied, preserving all-or-nothing semantics.
        let mut outcomes: Vec<DtxOpOutcome> = Vec::with_capacity(operations.len());
        let mut applied: Vec<(usize, DtxPreimage)> = Vec::new();
        let mut failed_index: Option<usize> = None;
        for (index, operation) in operations.iter().enumerate() {
            let is_write = !operation.operation_type.eq_ignore_ascii_case("Read");
            let preimage = if is_write {
                capture_dtx_preimage(store, region_name, operation)
            } else {
                None
            };
            let outcome = dtx_point_outcome(
                execute_dtx_point_operation(store, region_name, operation, start).await,
            )
            .await;
            // Reads legitimately return 304 Not Modified (If-None-Match); treat
            // that as committed so a mixed read/write transaction is not aborted,
            // consistent with the read path's is_read_success_status.
            let committed = if is_write {
                outcome.status.is_success()
            } else {
                is_read_success_status(outcome.status)
            };
            outcomes.push(outcome);
            if committed {
                if let Some(preimage) = preimage {
                    applied.push((index, preimage));
                }
            } else {
                failed_index = Some(index);
                break;
            }
        }

        if let Some(failed_index) = failed_index {
            for (index, preimage) in applied.iter().rev() {
                restore_dtx_preimage(store, region_name, &operations[*index], preimage);
            }
            // Drop the buffered replicas: the transaction aborted, so the
            // rolled-back writes must never reach secondary regions.
            store.abort_dtx_replication_capture();
            let failed = &outcomes[failed_index];
            return dtx_write_runtime_abort_response(
                operations,
                operations.len(),
                failed_index,
                failed.status,
                failed.sub_status,
                start,
            );
        }

        // The transaction committed; release the buffered replicas to
        // secondary regions.
        store.commit_dtx_replication_capture();
        dtx_commit_response(&outcomes, start)
    }

    /// Executes a read-only distributed transaction, producing a confirmed
    /// point-in-time snapshot across all reads. If any read fails, the reads
    /// that individually succeeded never contributed to a snapshot, so they are
    /// rewritten to 424 FailedDependency (body stripped) and the surviving
    /// failure codes are promoted into the response envelope.
    #[cfg(feature = "preview_dtx")]
    async fn handle_dtx_read_transaction(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        operations: &[DtxOperation],
        start: Instant,
    ) -> AsyncRawResponse {
        let mut outcomes: Vec<DtxOpOutcome> = Vec::with_capacity(operations.len());
        for operation in operations {
            outcomes.push(
                dtx_point_outcome(
                    execute_dtx_point_operation(store, region_name, operation, start).await,
                )
                .await,
            );
        }

        let snapshot_failed = outcomes
            .iter()
            .any(|outcome| !is_read_success_status(outcome.status));
        if snapshot_failed {
            for outcome in &mut outcomes {
                if is_read_success_status(outcome.status) {
                    outcome.status = StatusCode::FailedDependency;
                    outcome.sub_status = None;
                    outcome.etag = None;
                    outcome.session_token = None;
                    outcome.resource_body = None;
                }
            }
        }

        let envelope = promote_dtx_read_envelope(&outcomes);
        dtx_read_response(operations, envelope, &outcomes, start)
    }

    /// Snapshot of a document (and its partition LSN counters) before a write op
    /// is applied, used to roll the mutation back on abort.
    #[cfg(feature = "preview_dtx")]
    struct DtxPreimage {
        epk: Epk,
        document: Option<StoredDocument>,
        lsn: u64,
        local_lsn: u64,
        vector_clock_version: u64,
    }

    /// Captures the current stored document (if any) targeted by a write op so
    /// it can be restored verbatim if the transaction later aborts.
    #[cfg(feature = "preview_dtx")]
    fn capture_dtx_preimage(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        operation: &DtxOperation,
    ) -> Option<DtxPreimage> {
        let region_ref = store.region(region_name)?;
        region_ref
            .with_container(
                &operation.database_name,
                &operation.collection_name,
                |state| {
                    let parsed = dtx_operation_as_parsed_request(operation);
                    let body = operation
                        .resource_body
                        .as_ref()
                        .cloned()
                        .unwrap_or(serde_json::Value::Null);
                    let (_, epk) = resolve_partition_key(&parsed, &body, &state.metadata).ok()?;
                    let partition = state.find_partition(&epk)?;
                    let document = partition
                        .documents
                        .read()
                        .unwrap()
                        .get(&epk)
                        .and_then(|logical| logical.get(&operation.id))
                        .cloned();
                    Some(DtxPreimage {
                        epk,
                        document,
                        lsn: partition.current_lsn(),
                        local_lsn: partition.current_local_lsn(),
                        vector_clock_version: partition.current_version(),
                    })
                },
            )
            .flatten()
    }

    /// Restores a previously captured pre-image, undoing an applied write op.
    #[cfg(feature = "preview_dtx")]
    fn restore_dtx_preimage(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        operation: &DtxOperation,
        preimage: &DtxPreimage,
    ) {
        let Some(region_ref) = store.region(region_name) else {
            return;
        };
        region_ref.with_container(
            &operation.database_name,
            &operation.collection_name,
            |state| {
                let Some(partition) = state.find_partition(&preimage.epk) else {
                    return;
                };
                let mut documents = partition.documents.write().unwrap();
                let logical = documents.entry(preimage.epk.clone()).or_default();
                match &preimage.document {
                    Some(document) => {
                        logical.insert(operation.id.clone(), document.clone());
                    }
                    None => {
                        logical.remove(&operation.id);
                    }
                }
                // Reset the partition counters advanced by the applied write so
                // the abort leaves no LSN progress behind. Rollback runs in
                // reverse order, so the earliest pre-image restores the final
                // pre-transaction value.
                partition.restore_counters(
                    preimage.lsn,
                    preimage.local_lsn,
                    preimage.vector_clock_version,
                );
            },
        );
    }

    #[cfg(feature = "preview_dtx")]
    fn is_read_success_status(status: StatusCode) -> bool {
        matches!(u16::from(status), 200 | 304)
    }

    /// Promotes the distinct per-operation codes into a read envelope status,
    /// ignoring 424 FailedDependency: a single distinct code surfaces as-is,
    /// two or more distinct codes become 207 MultiStatus.
    #[cfg(feature = "preview_dtx")]
    fn promote_dtx_read_envelope(outcomes: &[DtxOpOutcome]) -> StatusCode {
        let mut distinct: Vec<StatusCode> = Vec::new();
        for outcome in outcomes {
            if u16::from(outcome.status) == 424 {
                continue;
            }
            if !distinct.contains(&outcome.status) {
                distinct.push(outcome.status);
            }
        }
        match distinct.as_slice() {
            [] => StatusCode::Ok,
            [single] => *single,
            _ => StatusCode::from(207_u16),
        }
    }

    #[cfg(feature = "preview_dtx")]
    struct DtxPreflightFailure {
        status: StatusCode,
        sub_status: Option<u16>,
        message: String,
    }

    #[cfg(feature = "preview_dtx")]
    fn preflight_failure(
        status: StatusCode,
        sub_status: Option<u16>,
        message: impl Into<String>,
    ) -> DtxPreflightFailure {
        DtxPreflightFailure {
            status,
            sub_status,
            message: message.into(),
        }
    }

    #[cfg(feature = "preview_dtx")]
    fn preflight_dtx_write_operation(
        store: &Arc<EmulatorStore>,
        region_name: &str,
        operation: &DtxOperation,
    ) -> Result<(), DtxPreflightFailure> {
        if operation.operation_type.eq_ignore_ascii_case("Read") {
            return Ok(());
        }

        if matches!(
            operation.operation_type.as_str(),
            "Create" | "Replace" | "Upsert" | "Patch"
        ) && operation.resource_body.is_none()
        {
            return Err(preflight_failure(
                StatusCode::BadRequest,
                None,
                format!(
                    "DTX {} operation requires a resourceBody",
                    operation.operation_type
                ),
            ));
        }

        let region_ref = store.region(region_name).ok_or_else(|| {
            preflight_failure(StatusCode::NotFound, None, "Region does not exist")
        })?;
        if !region_ref.database_exists(&operation.database_name) {
            return Err(preflight_failure(
                StatusCode::NotFound,
                None,
                format!("Database '{}' does not exist", operation.database_name),
            ));
        }

        let outcome = region_ref.with_container(
            &operation.database_name,
            &operation.collection_name,
            |state| {
                let parsed = dtx_operation_as_parsed_request(operation);
                let body = operation
                    .resource_body
                    .as_ref()
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                if matches!(
                    operation.operation_type.as_str(),
                    "Create" | "Replace" | "Upsert"
                ) {
                    match body.get("id").and_then(|value| value.as_str()) {
                        Some(body_id) if body_id == operation.id => {}
                        Some(body_id) => {
                            return Err(preflight_failure(
                                StatusCode::BadRequest,
                                None,
                                format!(
                                    "Document id in request body ('{body_id}') must match the DTX operation id ('{}')",
                                    operation.id
                                ),
                            ));
                        }
                        None => {
                            return Err(preflight_failure(
                                StatusCode::BadRequest,
                                None,
                                "DTX create, replace, and upsert operations require resourceBody.id",
                            ));
                        }
                    }
                }
                let (_, epk) = resolve_partition_key(&parsed, &body, &state.metadata).map_err(
                    |error| {
                        preflight_failure(
                            StatusCode::BadRequest,
                            None,
                            format!("invalid partition key: {error}"),
                        )
                    },
                )?;
                let partition = state.find_partition(&epk).ok_or_else(|| {
                    preflight_failure(
                        StatusCode::InternalServerError,
                        None,
                        "No partition found for EPK",
                    )
                })?;
                if partition.is_locked() {
                    return Err(preflight_failure(
                        StatusCode::Gone,
                        Some(PARTITION_SPLIT_OR_MERGE_SUBSTATUS),
                        "Partition is being split or merged.",
                    ));
                }

                let docs = partition.documents.read().unwrap();
                let existing = docs.get(&epk).and_then(|logical| logical.get(&operation.id));
                match operation.operation_type.as_str() {
                    "Create" => {
                        if existing.is_some() {
                            return Err(preflight_failure(
                                StatusCode::Conflict,
                                None,
                                format!(
                                    "Entity with the specified id already exists in the system. ResourceId: {}",
                                    operation.id
                                ),
                            ));
                        }
                    }
                    "Replace" | "Delete" => {
                        let Some(existing) = existing else {
                            return Err(preflight_failure(
                                StatusCode::NotFound,
                                None,
                                format!(
                                    "Entity with the specified id does not exist in the system. ResourceId: {}",
                                    operation.id
                                ),
                            ));
                        };
                        if operation
                            .if_match
                            .as_ref()
                            .is_some_and(|etag| etag != &existing.etag)
                        {
                            return Err(preflight_failure(
                                StatusCode::PreconditionFailed,
                                None,
                                "One of the specified pre-condition is not met.",
                            ));
                        }
                    }
                    "Patch" => {
                        let Some(existing) = existing else {
                            return Err(preflight_failure(
                                StatusCode::NotFound,
                                None,
                                format!(
                                    "Entity with the specified id does not exist in the system. ResourceId: {}",
                                    operation.id
                                ),
                            ));
                        };
                        if operation
                            .if_match
                            .as_ref()
                            .is_some_and(|etag| etag != &existing.etag)
                        {
                            return Err(preflight_failure(
                                StatusCode::PreconditionFailed,
                                None,
                                "One of the specified pre-condition is not met.",
                            ));
                        }
                        let (_, condition) = parse_dtx_patch_body(&body).map_err(|message| {
                            preflight_failure(StatusCode::BadRequest, None, message)
                        })?;
                        match dtx_patch_condition_matches(condition.as_deref(), &existing.body) {
                            Ok(true) => {}
                            Ok(false) => {
                                return Err(preflight_failure(
                                    StatusCode::PreconditionFailed,
                                    Some(DTX_PATCH_CONDITION_NOT_MET_SUBSTATUS),
                                    "Patch condition was not met.",
                                ));
                            }
                            Err(message) => {
                                return Err(preflight_failure(
                                    StatusCode::BadRequest,
                                    None,
                                    message,
                                ));
                            }
                        }
                    }
                    "Upsert" => {}
                    other => {
                        return Err(preflight_failure(
                            StatusCode::BadRequest,
                            None,
                            format!("Unsupported DTX operation type '{other}'"),
                        ));
                    }
                }
                Ok(())
            },
        );

        match outcome {
            Some(result) => result,
            None => Err(preflight_failure(
                StatusCode::NotFound,
                None,
                format!(
                    "Container '{}/{}' does not exist",
                    operation.database_name, operation.collection_name
                ),
            )),
        }
    }

    #[cfg(feature = "preview_dtx")]
    fn dtx_operation_as_parsed_request(operation: &DtxOperation) -> ParsedRequest {
        ParsedRequest {
            operation: OperationType::Read,
            db_id: Some(operation.database_name.clone()),
            coll_id: Some(operation.collection_name.clone()),
            doc_id: Some(operation.id.clone()),
            offer_id: None,
            partition_key_header: Some(operation.partition_key.to_string()),
            if_match: operation.if_match.clone(),
            if_none_match: operation.if_none_match.clone(),
            session_token: operation.session_token.clone(),
            activity_id: None,
            content_response_on_write: true,
            offer_throughput: None,
            offer_autopilot_settings: None,
            max_item_count: None,
            continuation: None,
            partition_key_range_id: None,
            start_epk: None,
            end_epk: None,
            is_query_plan: false,
            is_batch: false,
            is_upsert: false,
        }
    }

    /// Builds the 200 envelope for a fully-committed write transaction.
    #[cfg(feature = "preview_dtx")]
    fn dtx_commit_response(outcomes: &[DtxOpOutcome], start: Instant) -> AsyncRawResponse {
        let mut total_charge = 0.0;
        let operation_responses: Vec<serde_json::Value> = outcomes
            .iter()
            .enumerate()
            .map(|(index, outcome)| {
                total_charge += outcome.request_charge;
                dtx_op_json(
                    index,
                    outcome.status,
                    outcome.sub_status,
                    outcome.etag.as_deref(),
                    outcome.session_token.as_deref(),
                    outcome.pk_range_id.as_deref(),
                    outcome.local_lsn,
                    outcome.request_charge,
                    None,
                )
            })
            .collect();
        let response_body = serde_json::json!({
            "operationResponses": operation_responses,
        });
        dtx_response_builder(StatusCode::Ok, start)
            .with_request_charge(total_charge)
            .with_json_body(&response_body)
            .build()
    }

    /// Builds the 452 abort envelope for a write transaction that failed during
    /// the prepare phase. "No" voters keep their real failure code so the caller
    /// sees the root cause; every "Yes" voter was prepared but rolled back and
    /// surfaces as 453 (sub-status 5415, DtcOperationRolledBack).
    #[cfg(feature = "preview_dtx")]
    fn dtx_write_abort_response(
        operations: &[DtxOperation],
        votes: &[Option<DtxPreflightFailure>],
        start: Instant,
    ) -> AsyncRawResponse {
        let mut diagnostic: Option<String> = None;
        let operation_responses: Vec<serde_json::Value> = votes
            .iter()
            .enumerate()
            .map(|(index, vote)| match vote {
                Some(failure) => {
                    if diagnostic.is_none() {
                        diagnostic = Some(failure.message.clone());
                    }
                    dtx_op_json(
                        operations[index].index,
                        failure.status,
                        failure.sub_status.map(u32::from),
                        None,
                        None,
                        None,
                        None,
                        1.0,
                        None,
                    )
                }
                None => dtx_op_json(
                    operations[index].index,
                    StatusCode::from(DTX_ROLLED_BACK_STATUS),
                    Some(DTX_ROLLED_BACK_SUBSTATUS),
                    None,
                    None,
                    None,
                    None,
                    1.0,
                    None,
                ),
            })
            .collect();
        let response_body = serde_json::json!({
            "isRetriable": false,
            "diagnosticString": diagnostic
                .unwrap_or_else(|| "distributed transaction aborted".to_owned()),
            "operationResponses": operation_responses,
        });
        dtx_response_builder(StatusCode::from(452_u16), start)
            .with_request_charge(1.0)
            .with_json_body(&response_body)
            .build()
    }

    /// Builds the 452 abort envelope for a write transaction that failed at
    /// commit time (after prepare succeeded). The failing participant keeps its
    /// code; all others were rolled back and surface as 453 / 5415.
    #[cfg(feature = "preview_dtx")]
    fn dtx_write_runtime_abort_response(
        operations: &[DtxOperation],
        operation_count: usize,
        failed_index: usize,
        failed_status: StatusCode,
        failed_sub_status: Option<u32>,
        start: Instant,
    ) -> AsyncRawResponse {
        let operation_responses: Vec<serde_json::Value> = (0..operation_count)
            .map(|index| {
                if index == failed_index {
                    dtx_op_json(
                        operations[index].index,
                        failed_status,
                        failed_sub_status,
                        None,
                        None,
                        None,
                        None,
                        1.0,
                        None,
                    )
                } else {
                    dtx_op_json(
                        operations[index].index,
                        StatusCode::from(DTX_ROLLED_BACK_STATUS),
                        Some(DTX_ROLLED_BACK_SUBSTATUS),
                        None,
                        None,
                        None,
                        None,
                        1.0,
                        None,
                    )
                }
            })
            .collect();
        let response_body = serde_json::json!({
            "isRetriable": false,
            "diagnosticString":
                "distributed transaction rolled back after a participant failed to commit",
            "operationResponses": operation_responses,
        });
        dtx_response_builder(StatusCode::from(452_u16), start)
            .with_request_charge(1.0)
            .with_json_body(&response_body)
            .build()
    }

    /// Builds the response envelope for a read transaction from its (possibly
    /// rewritten) per-operation outcomes.
    #[cfg(feature = "preview_dtx")]
    fn dtx_read_response(
        operations: &[DtxOperation],
        envelope: StatusCode,
        outcomes: &[DtxOpOutcome],
        start: Instant,
    ) -> AsyncRawResponse {
        let mut total_charge = 0.0;
        let operation_responses: Vec<serde_json::Value> = outcomes
            .iter()
            .enumerate()
            .map(|(index, outcome)| {
                total_charge += outcome.request_charge;
                dtx_op_json(
                    operations[index].index,
                    outcome.status,
                    outcome.sub_status,
                    outcome.etag.as_deref(),
                    outcome.session_token.as_deref(),
                    outcome.pk_range_id.as_deref(),
                    None,
                    outcome.request_charge,
                    outcome.resource_body.as_ref(),
                )
            })
            .collect();
        let response_body = serde_json::json!({
            "isRetriable": u16::from(envelope) == 449,
            "operationResponses": operation_responses,
        });
        dtx_response_builder(envelope, start)
            .with_request_charge(total_charge)
            .with_json_body(&response_body)
            .build()
    }

    #[cfg(feature = "preview_dtx")]
    fn dtx_response_builder(status: StatusCode, start: Instant) -> ResponseBuilder {
        ResponseBuilder::new(status, start)
            .without_header(GLOBAL_COMMITTED_LSN.clone())
            .without_header(QUORUM_ACKED_LSN.clone())
            .without_header(QUORUM_ACKED_LOCAL_LSN.clone())
            .without_header(LOCAL_LSN.clone())
            .without_header(NUMBER_OF_READ_REGIONS.clone())
            .without_header(LAST_STATE_CHANGE_UTC.clone())
            .without_header(RESOURCE_QUOTA.clone())
            .without_header(RESOURCE_USAGE.clone())
    }
    finalize_response(store, response, parsed.activity_id.as_deref()).await
}

// --- Control-Plane Operations ---

fn handle_read_account(store: &Arc<EmulatorStore>, start: Instant) -> AsyncRawResponse {
    let body = account_properties_to_json(store.config());
    success_response(StatusCode::Ok, &body, 0.0, "", start)
        .with_item_count(1)
        .build()
}

async fn handle_create_database(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    let db_id = match body.get("id").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Missing 'id' field in database creation request",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    // Serialize the (exists?, create) pair so two concurrent requests for
    // the same database id cannot both observe "does not exist" and both
    // emit 201/Created. The lock is per-`db_id` so unrelated database
    // creates run in parallel.
    let cp_lock = store.control_plane_lock_db(&db_id);
    let _cp_guard = cp_lock.lock().await;

    // Check if already exists
    if let Some(region_ref) = store.region(region_name) {
        if region_ref.database_exists(&db_id) {
            return error_response(
                StatusCode::Conflict,
                None,
                "Conflict",
                &format!(
                    "Entity with the specified id already exists in the system. ResourceId: {}",
                    db_id
                ),
                1.0,
                "",
                start,
            )
            .build();
        }
    }

    let meta = store.create_database_internal(&db_id);
    let response_body = database_to_json(&meta);
    let token = store.advance_master_partition_lsn(region_name);
    if parsed.content_response_on_write {
        success_response(StatusCode::Created, &response_body, 1.0, &token, start)
            .with_etag(&meta.etag)
            .build()
    } else {
        ResponseBuilder::new(StatusCode::Created, start)
            .with_request_charge(1.0)
            .with_session_token(&token)
            .with_etag(&meta.etag)
            .build()
    }
}

fn handle_read_database(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    start: Instant,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    match region_ref.get_database(db_id) {
        Some(meta) => {
            let body = database_to_json(&meta);
            success_response(StatusCode::Ok, &body, 1.0, "", start)
                .with_etag(&meta.etag)
                .build()
        }
        None => error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!(
                "Entity with the specified id does not exist in the system. ResourceId: {}",
                db_id
            ),
            0.0,
            "",
            start,
        )
        .build(),
    }
}

fn handle_delete_database(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    start: Instant,
) -> AsyncRawResponse {
    // Delete from all regions (cascade)
    let exists = store
        .region(region_name)
        .map(|r| r.database_exists(db_id))
        .unwrap_or(false);

    if !exists {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!(
                "Entity with the specified id does not exist in the system. ResourceId: {}",
                db_id
            ),
            0.0,
            "",
            start,
        )
        .build();
    }

    // Cascade-delete: purges buffered replications for this db and prunes
    // the rid-generator's per-db collection counter.
    store.cascade_delete_database(db_id);

    let token = store.advance_master_partition_lsn(region_name);
    ResponseBuilder::new(StatusCode::NoContent, start)
        .with_request_charge(1.0)
        .with_session_token(&token)
        .build()
}

async fn handle_create_container(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    // Verify database exists
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    if !region_ref.database_exists(db_id) {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            0.0,
            "",
            start,
        )
        .build();
    }

    let body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    let coll_id = match body.get("id").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Missing 'id' field",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    // Check partition key definition
    let pk_def: PartitionKeyDefinition = match body.get("partitionKey") {
        Some(pk_val) => match serde_json::from_value(pk_val.clone()) {
            Ok(pk) => pk,
            Err(_) => {
                return error_response(
                    StatusCode::BadRequest,
                    None,
                    "BadRequest",
                    "Invalid partitionKey definition",
                    0.0,
                    "",
                    start,
                )
                .build();
            }
        },
        None => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Missing partitionKey definition in container creation request",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    // Check for duplicate
    // Serialize the (exists?, create) pair on the per-(db, coll)
    // control-plane lock, mirroring `handle_create_database`. Without this,
    // two concurrent CreateContainer calls for the same id can both observe
    // "not present" and both proceed to `create_container_with_config_internal`.
    let cp_lock = store.control_plane_lock_coll(db_id, &coll_id);
    let _cp_guard = cp_lock.lock().await;

    if region_ref.container_exists(db_id, &coll_id) {
        return error_response(
            StatusCode::Conflict,
            None,
            "Conflict",
            &format!("Container '{}' already exists", coll_id),
            1.0,
            "",
            start,
        )
        .build();
    }

    // Honor caller-specified provisioned throughput from `x-ms-offer-throughput`.
    // When the header is missing, `ContainerConfig::default()` keeps
    // `provisioned_throughput_ru = None` (no throttling), matching the prior
    // behavior. When present and below the 400 RU/s minimum, surface the same
    // 400/BadRequest the real service would emit instead of silently clamping.
    let mut container_config = ContainerConfig::default();
    if let Some(ru) = parsed.offer_throughput {
        container_config = container_config.with_throughput(ru);
        if let Err(err) = container_config.clone().build() {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                &err.to_string(),
                0.0,
                "",
                start,
            )
            .build();
        }
    }

    let meta =
        store.create_container_with_config_internal(db_id, &coll_id, pk_def, container_config);
    let response_body = container_to_json(&meta);
    let token = store.advance_master_partition_lsn(region_name);
    if parsed.content_response_on_write {
        success_response(StatusCode::Created, &response_body, 1.0, &token, start)
            .with_etag(&meta.etag)
            .build()
    } else {
        ResponseBuilder::new(StatusCode::Created, start)
            .with_request_charge(1.0)
            .with_session_token(&token)
            .with_etag(&meta.etag)
            .build()
    }
}

fn handle_read_container(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    coll_id: &str,
    start: Instant,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    if !region_ref.database_exists(db_id) {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            0.0,
            "",
            start,
        )
        .build();
    }

    match region_ref.get_container(db_id, coll_id) {
        Some(snapshot) => {
            let body = container_to_json(&snapshot.metadata);
            success_response(StatusCode::Ok, &body, 1.0, "", start)
                .with_etag(&snapshot.metadata.etag)
                .build()
        }
        None => error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Container '{}' does not exist", coll_id),
            0.0,
            "",
            start,
        )
        .build(),
    }
}

fn handle_delete_container(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    coll_id: &str,
    start: Instant,
) -> AsyncRawResponse {
    let exists = store
        .region(region_name)
        .map(|r| r.container_exists(db_id, coll_id))
        .unwrap_or(false);

    if !exists {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Container '{}' does not exist", coll_id),
            0.0,
            "",
            start,
        )
        .build();
    }

    // Cascade-delete: also purges any buffered replications targeted at this
    // container so a paused target region does not silently drop them later.
    store.cascade_delete_container(db_id, coll_id);

    let token = store.advance_master_partition_lsn(region_name);
    ResponseBuilder::new(StatusCode::NoContent, start)
        .with_request_charge(1.0)
        .with_session_token(&token)
        .build()
}

fn handle_read_pkranges(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    coll_id: &str,
    if_none_match: Option<&str>,
    start: Instant,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    if !region_ref.database_exists(db_id) {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            0.0,
            "",
            start,
        )
        .build();
    }

    region_ref
        .with_container(db_id, coll_id, |state| {
            // Honor If-None-Match for change-feed-style routing-map refreshes.
            // The driver's `fetch_and_build_routing_map` loops calling
            // `fetch_pk_ranges` with the previous etag as `If-None-Match` until
            // the service returns 304 (or hits `MAX_FETCH_ITERATIONS`).
            // Without 304 support the loop runs the maximum number of iterations,
            // accumulates duplicate ranges, and `ContainerRoutingMap::try_create`
            // produces an empty map — defeating PK-range pre-resolution and
            // any feature that depends on it (PPCB, PPAF).
            if let Some(client_etag) = if_none_match {
                if client_etag == state.metadata.etag {
                    return ResponseBuilder::new(StatusCode::NotModified, start)
                        .with_request_charge(1.0)
                        .with_etag(&state.metadata.etag)
                        .build();
                }
            }
            let body = pkranges_to_json(state);
            success_response(StatusCode::Ok, &body, 1.0, "", start)
                .with_etag(&state.metadata.etag)
                .with_item_count(state.physical_partitions.len() as u32)
                .build()
        })
        .unwrap_or_else(|| {
            error_response(
                StatusCode::NotFound,
                None,
                "NotFound",
                &format!("Container '{}' does not exist", coll_id),
                0.0,
                "",
                start,
            )
            .build()
        })
}

fn paginate_values(
    values: Vec<serde_json::Value>,
    max_item_count: Option<i32>,
    continuation: Option<&str>,
    start: Instant,
) -> Result<(Vec<serde_json::Value>, Option<String>), AsyncRawResponse> {
    let offset = match continuation {
        Some(token) => token.parse::<usize>().map_err(|_| {
            error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid continuation token",
                0.0,
                "",
                start,
            )
            .build()
        })?,
        None => 0,
    };

    let total = values.len();
    let limit = match max_item_count {
        Some(n) if n > 0 => n as usize,
        _ => total.saturating_sub(offset),
    };
    let end = offset.saturating_add(limit).min(total);
    let page = if offset >= total {
        Vec::new()
    } else {
        values[offset..end].to_vec()
    };
    let next = (end < total).then(|| end.to_string());
    Ok((page, next))
}

#[derive(Clone)]
struct DocumentFeedItem {
    body: serde_json::Value,
    cursor: DocumentFeedCursor,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DocumentFeedCursor {
    epk: Epk,
    id: String,
}

#[derive(Deserialize, Serialize)]
struct DocumentFeedCursorToken {
    kind: String,
    epk: String,
    id: String,
}

const DOCUMENT_FEED_CURSOR_TOKEN_KIND: &str = "document_feed_cursor_v1";

impl DocumentFeedCursor {
    fn to_token(&self) -> String {
        serde_json::to_string(&DocumentFeedCursorToken {
            kind: DOCUMENT_FEED_CURSOR_TOKEN_KIND.to_owned(),
            epk: self.epk.to_hex(),
            id: self.id.clone(),
        })
        .expect("document feed cursor token serialization cannot fail")
    }

    fn parse(token: &str, start: Instant) -> Result<Self, AsyncRawResponse> {
        let token: DocumentFeedCursorToken = serde_json::from_str(token)
            .map_err(|_| invalid_continuation_response("Invalid continuation token", start))?;
        if token.kind != DOCUMENT_FEED_CURSOR_TOKEN_KIND {
            return Err(invalid_continuation_response(
                "Invalid continuation token kind",
                start,
            ));
        }
        if !is_even_length_hex(&token.epk) {
            return Err(invalid_continuation_response(
                "Invalid continuation token EPK",
                start,
            ));
        }
        Ok(Self {
            epk: Epk::from(token.epk.as_str()),
            id: token.id,
        })
    }
}

fn is_even_length_hex(value: &str) -> bool {
    value.len().is_multiple_of(2) && value.bytes().all(|b| b.is_ascii_hexdigit())
}

fn invalid_continuation_response(message: &str, start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::BadRequest,
        None,
        "BadRequest",
        message,
        0.0,
        "",
        start,
    )
    .build()
}

fn paginate_document_feed_items(
    items: Vec<DocumentFeedItem>,
    max_item_count: Option<i32>,
    continuation: Option<&str>,
    start: Instant,
) -> Result<(Vec<serde_json::Value>, Option<String>), AsyncRawResponse> {
    let offset = match continuation {
        Some(token) => {
            let cursor = DocumentFeedCursor::parse(token, start)?;
            items.partition_point(|item| item.cursor <= cursor)
        }
        None => 0,
    };

    let total = items.len();
    let limit = match max_item_count {
        Some(n) if n > 0 => n as usize,
        _ => total.saturating_sub(offset),
    };
    let end = offset.saturating_add(limit).min(total);
    let page_items = if offset >= total {
        Vec::new()
    } else {
        items[offset..end].to_vec()
    };
    let next = (end < total)
        .then(|| page_items.last().map(|item| item.cursor.to_token()))
        .flatten();
    Ok((page_items.into_iter().map(|item| item.body).collect(), next))
}

#[derive(Clone, Copy)]
struct FeedPageOptions<'a> {
    max_item_count: Option<i32>,
    continuation: Option<&'a str>,
}

impl<'a> FeedPageOptions<'a> {
    fn from_request(parsed: &'a ParsedRequest) -> Self {
        Self {
            max_item_count: parsed.max_item_count,
            continuation: parsed.continuation.as_deref(),
        }
    }
}

#[derive(Clone)]
struct FeedResponseHeaders {
    session_token: String,
    lsn: Option<u64>,
    partition_key_range_id: Option<u32>,
    internal_partition_id: Option<String>,
}

impl FeedResponseHeaders {
    fn none() -> Self {
        Self {
            session_token: String::new(),
            lsn: None,
            partition_key_range_id: None,
            internal_partition_id: None,
        }
    }
}

fn success_feed_response(
    envelope_name: &str,
    rid: impl Into<String>,
    items: Vec<serde_json::Value>,
    page_options: FeedPageOptions<'_>,
    feed_headers: FeedResponseHeaders,
    start: Instant,
) -> AsyncRawResponse {
    let (page, next) = match paginate_values(
        items,
        page_options.max_item_count,
        page_options.continuation,
        start,
    ) {
        Ok(v) => v,
        Err(response) => return response,
    };
    let item_count = page.len() as u32;
    let body = feed_to_json(envelope_name, page, rid);
    let mut builder = success_response(
        StatusCode::Ok,
        &body,
        1.0,
        &feed_headers.session_token,
        start,
    )
    .with_item_count(item_count);
    if let Some(lsn) = feed_headers.lsn {
        builder = builder.with_lsn(lsn);
    }
    if let Some(id) = feed_headers.partition_key_range_id {
        builder = builder.with_header_value(PARTITION_KEY_RANGE_ID.clone(), id);
    }
    if let Some(id) = feed_headers.internal_partition_id {
        builder = builder.with_header_value(INTERNAL_PARTITION_ID.clone(), id);
    }
    if let Some(next) = next {
        builder = builder.with_header_value(CONTINUATION.clone(), next);
    }
    builder.build()
}

fn success_document_feed_response(
    envelope_name: &str,
    rid: impl Into<String>,
    items: Vec<DocumentFeedItem>,
    page_options: FeedPageOptions<'_>,
    feed_headers: FeedResponseHeaders,
    start: Instant,
) -> AsyncRawResponse {
    let (page, next) = match paginate_document_feed_items(
        items,
        page_options.max_item_count,
        page_options.continuation,
        start,
    ) {
        Ok(v) => v,
        Err(response) => return response,
    };
    let item_count = page.len() as u32;
    let body = feed_to_json(envelope_name, page, rid);
    let mut builder = success_response(
        StatusCode::Ok,
        &body,
        1.0,
        &feed_headers.session_token,
        start,
    )
    .with_item_count(item_count);
    if let Some(lsn) = feed_headers.lsn {
        builder = builder.with_lsn(lsn);
    }
    if let Some(id) = feed_headers.partition_key_range_id {
        builder = builder.with_header_value(PARTITION_KEY_RANGE_ID.clone(), id);
    }
    if let Some(id) = feed_headers.internal_partition_id {
        builder = builder.with_header_value(INTERNAL_PARTITION_ID.clone(), id);
    }
    if let Some(next) = next {
        builder = builder.with_header_value(CONTINUATION.clone(), next);
    }
    builder.build()
}

#[derive(Deserialize)]
struct QuerySpec {
    query: String,
    #[serde(default)]
    parameters: Vec<QueryParameter>,
}

#[derive(Deserialize)]
struct QueryParameter {
    name: String,
    value: serde_json::Value,
}

fn parse_query_spec(
    request_body: &[u8],
    start: Instant,
) -> Result<(String, Vec<(String, serde_json::Value)>), AsyncRawResponse> {
    let spec: QuerySpec = serde_json::from_slice(request_body).map_err(|e| {
        error_response(
            StatusCode::BadRequest,
            None,
            "BadRequest",
            &format!("Invalid query JSON body: {e}"),
            0.0,
            "",
            start,
        )
        .build()
    })?;
    if spec.query.trim().is_empty() {
        return Err(error_response(
            StatusCode::BadRequest,
            None,
            "BadRequest",
            "Query text must not be empty",
            0.0,
            "",
            start,
        )
        .build());
    }
    let parameters = spec
        .parameters
        .into_iter()
        .map(|p| (p.name, p.value))
        .collect();
    Ok((spec.query, parameters))
}

fn execute_query_feed(
    envelope_name: &str,
    rid: impl Into<String>,
    values: Vec<serde_json::Value>,
    parsed: &ParsedRequest,
    request_body: &[u8],
    feed_headers: FeedResponseHeaders,
    start: Instant,
) -> AsyncRawResponse {
    let (query, parameters) = match parse_query_spec(request_body, start) {
        Ok(v) => v,
        Err(response) => return response,
    };
    let results = match crate::query::eval::query_documents(&query, &parameters, &values) {
        Ok(results) => results,
        Err(e) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                &e.to_string(),
                0.0,
                "",
                start,
            )
            .build();
        }
    };
    success_feed_response(
        envelope_name,
        rid,
        results,
        FeedPageOptions::from_request(parsed),
        feed_headers,
        start,
    )
}

fn execute_document_query_feed(
    envelope_name: &str,
    rid: impl Into<String>,
    documents: Vec<DocumentFeedItem>,
    parsed: &ParsedRequest,
    request_body: &[u8],
    feed_headers: FeedResponseHeaders,
    start: Instant,
) -> AsyncRawResponse {
    let (query, parameters) = match parse_query_spec(request_body, start) {
        Ok(v) => v,
        Err(response) => return response,
    };
    match query_document_feed_items(&query, &parameters, &documents) {
        Ok(Some(results)) => success_document_feed_response(
            envelope_name,
            rid,
            results,
            FeedPageOptions::from_request(parsed),
            feed_headers,
            start,
        ),
        Ok(None) => {
            let values: Vec<_> = documents.into_iter().map(|doc| doc.body).collect();
            let results = match crate::query::eval::query_documents(&query, &parameters, &values) {
                Ok(results) => results,
                Err(e) => {
                    return error_response(
                        StatusCode::BadRequest,
                        None,
                        "BadRequest",
                        &e.to_string(),
                        0.0,
                        "",
                        start,
                    )
                    .build();
                }
            };
            success_feed_response(
                envelope_name,
                rid,
                results,
                FeedPageOptions::from_request(parsed),
                feed_headers,
                start,
            )
        }
        Err(e) => error_response(
            StatusCode::BadRequest,
            None,
            "BadRequest",
            &e.to_string(),
            0.0,
            "",
            start,
        )
        .build(),
    }
}

fn query_document_feed_items(
    sql: &str,
    parameters: &[(String, serde_json::Value)],
    documents: &[DocumentFeedItem],
) -> crate::error::Result<Option<Vec<DocumentFeedItem>>> {
    let program = crate::query::parse(sql).map_err(|e| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message(format!("failed to parse query: {e}"))
            .with_source(e)
            .build()
    })?;
    let query = &program.query;
    if !supports_document_cursor_continuation(query) {
        return Ok(None);
    }

    let mut results = Vec::new();
    for document in documents {
        if crate::query::eval::matches_query(&document.body, query, parameters).map_err(|e| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(StatusCode::BadRequest))
                .with_message(e.to_string())
                .build()
        })? {
            let body =
                crate::query::eval::project(&document.body, query, parameters).map_err(|e| {
                    crate::error::CosmosError::builder()
                        .with_status(crate::error::CosmosStatus::new(StatusCode::BadRequest))
                        .with_message(e.to_string())
                        .build()
                })?;
            results.push(DocumentFeedItem {
                body,
                cursor: document.cursor.clone(),
            });
        }
    }
    Ok(Some(results))
}

fn supports_document_cursor_continuation(query: &SqlQuery) -> bool {
    if query.select.distinct
        || query.select.top.is_some()
        || query.group_by.is_some()
        || query.order_by.is_some()
        || query.offset_limit.is_some()
        || !is_plain_root_from(query)
    {
        return false;
    }
    match &query.select.spec {
        SqlSelectSpec::Star => true,
        SqlSelectSpec::List(items) => !items
            .iter()
            .any(|item| contains_aggregate_expression(&item.expression)),
        SqlSelectSpec::Value(expr) => !contains_aggregate_expression(expr),
    }
}

fn is_plain_root_from(query: &SqlQuery) -> bool {
    match &query.from {
        None => true,
        Some(from) => matches!(
            &from.collection,
            SqlCollectionExpression::Aliased {
                collection: SqlCollection::Path { path, .. },
                ..
            } if path.is_empty()
        ),
    }
}

fn contains_aggregate_expression(expr: &SqlScalarExpression) -> bool {
    match expr {
        SqlScalarExpression::FunctionCall {
            name, is_udf, args, ..
        } => {
            (!is_udf
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "COUNT" | "SUM" | "AVG" | "MIN" | "MAX"
                ))
                || args.iter().any(contains_aggregate_expression)
        }
        SqlScalarExpression::Binary { left, right, .. }
        | SqlScalarExpression::Coalesce { left, right } => {
            contains_aggregate_expression(left) || contains_aggregate_expression(right)
        }
        SqlScalarExpression::Unary { operand, .. }
        | SqlScalarExpression::IsNull {
            expression: operand,
            ..
        } => contains_aggregate_expression(operand),
        SqlScalarExpression::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            contains_aggregate_expression(condition)
                || contains_aggregate_expression(if_true)
                || contains_aggregate_expression(if_false)
        }
        SqlScalarExpression::Between {
            expression,
            low,
            high,
            ..
        } => {
            contains_aggregate_expression(expression)
                || contains_aggregate_expression(low)
                || contains_aggregate_expression(high)
        }
        SqlScalarExpression::In {
            expression, items, ..
        } => {
            contains_aggregate_expression(expression)
                || items.iter().any(contains_aggregate_expression)
        }
        SqlScalarExpression::Like {
            expression,
            pattern,
            ..
        } => contains_aggregate_expression(expression) || contains_aggregate_expression(pattern),
        SqlScalarExpression::MemberRef { source, .. } => contains_aggregate_expression(source),
        SqlScalarExpression::MemberIndexer { source, index } => {
            contains_aggregate_expression(source) || contains_aggregate_expression(index)
        }
        SqlScalarExpression::ArrayCreate(items) => items.iter().any(contains_aggregate_expression),
        SqlScalarExpression::ObjectCreate(props) => props
            .iter()
            .any(|prop| contains_aggregate_expression(&prop.expression)),
        SqlScalarExpression::Exists(_)
        | SqlScalarExpression::Subquery(_)
        | SqlScalarExpression::Array(_) => true,
        SqlScalarExpression::Literal(_)
        | SqlScalarExpression::PropertyRef(_)
        | SqlScalarExpression::ParameterRef(_) => false,
    }
}

fn handle_read_feed_databases(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    let databases: Vec<_> = region_ref
        .list_databases()
        .iter()
        .map(database_to_json)
        .collect();
    success_feed_response(
        "Databases",
        "",
        databases,
        FeedPageOptions::from_request(parsed),
        FeedResponseHeaders::none(),
        start,
    )
}

fn handle_query_databases(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    let databases: Vec<_> = region_ref
        .list_databases()
        .iter()
        .map(database_to_json)
        .collect();
    execute_query_feed(
        "Databases",
        "",
        databases,
        parsed,
        request_body,
        FeedResponseHeaders::none(),
        start,
    )
}

fn handle_read_feed_containers(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    let Some(db) = region_ref.get_database(db_id) else {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            0.0,
            "",
            start,
        )
        .build();
    };
    let containers: Vec<_> = region_ref
        .list_containers(db_id)
        .iter()
        .map(container_to_json)
        .collect();
    success_feed_response(
        "DocumentCollections",
        db.rid,
        containers,
        FeedPageOptions::from_request(parsed),
        FeedResponseHeaders::none(),
        start,
    )
}

fn handle_query_containers(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    let Some(db) = region_ref.get_database(db_id) else {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            0.0,
            "",
            start,
        )
        .build();
    };
    let containers: Vec<_> = region_ref
        .list_containers(db_id)
        .iter()
        .map(container_to_json)
        .collect();
    execute_query_feed(
        "DocumentCollections",
        db.rid,
        containers,
        parsed,
        request_body,
        FeedResponseHeaders::none(),
        start,
    )
}

fn handle_read_feed_offers(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    let offers: Vec<_> = region_ref.list_offers().iter().map(offer_to_json).collect();
    success_feed_response(
        "Offers",
        "",
        offers,
        FeedPageOptions::from_request(parsed),
        FeedResponseHeaders::none(),
        start,
    )
}

fn handle_query_offers(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    let offers: Vec<_> = region_ref.list_offers().iter().map(offer_to_json).collect();
    execute_query_feed(
        "Offers",
        "",
        offers,
        parsed,
        request_body,
        FeedResponseHeaders::none(),
        start,
    )
}

fn handle_read_offer(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> AsyncRawResponse {
    let offer_id = parsed.offer_id.as_deref().unwrap_or("");
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    match region_ref.get_offer(offer_id) {
        Some(offer) => {
            let body = offer_to_json(&offer);
            success_response(StatusCode::Ok, &body, 1.0, "", start)
                .with_etag(&offer.etag)
                .build()
        }
        None => error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Offer '{}' does not exist", offer_id),
            0.0,
            "",
            start,
        )
        .build(),
    }
}

fn parse_offer_throughput(request_body: &[u8], start: Instant) -> Result<u32, AsyncRawResponse> {
    let body: serde_json::Value = serde_json::from_slice(request_body).map_err(|_| {
        error_response(
            StatusCode::BadRequest,
            None,
            "BadRequest",
            "Invalid JSON body",
            0.0,
            "",
            start,
        )
        .build()
    })?;
    let throughput = body
        .pointer("/content/offerThroughput")
        .and_then(|v| v.as_u64())
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| {
            error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Missing or invalid content.offerThroughput",
                0.0,
                "",
                start,
            )
            .build()
        })?;
    let config = ContainerConfig::default().with_throughput(throughput);
    if let Err(e) = config.build() {
        return Err(error_response(
            StatusCode::BadRequest,
            None,
            "BadRequest",
            &e.to_string(),
            0.0,
            "",
            start,
        )
        .build());
    }
    Ok(throughput)
}

fn handle_replace_offer(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let offer_id = parsed.offer_id.as_deref().unwrap_or("");
    let throughput = match parse_offer_throughput(request_body, start) {
        Ok(v) => v,
        Err(response) => return response,
    };
    let Some(offer) = store.replace_offer_internal(offer_id, throughput) else {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Offer '{}' does not exist", offer_id),
            0.0,
            "",
            start,
        )
        .build();
    };
    let token = store.advance_master_partition_lsn(region_name);
    let body = offer_to_json(&offer);
    success_response(StatusCode::Ok, &body, 1.0, &token, start)
        .with_etag(&offer.etag)
        .with_header_value(OFFER_REPLACE_PENDING.clone(), "false")
        .build()
}

fn collect_item_documents(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> Result<(String, Vec<DocumentFeedItem>, String, FeedResponseHeaders), AsyncRawResponse> {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return Err(not_found_region(start)),
    };
    if !region_ref.database_exists(db_id) {
        return Err(error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            0.0,
            "",
            start,
        )
        .build());
    }

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let requested_epk = match parsed.partition_key_header.as_deref() {
            Some(header) => match parse_partition_key_header(header) {
                Ok(components) if components.is_empty() => None,
                // A partial hierarchical partition key (fewer components than the
                // container's PK paths) targets a *prefix* of logical partitions.
                // Real Cosmos scopes such reads via the `x-ms-start-epk`/
                // `x-ms-end-epk` range (below) rather than an exact point EPK, so
                // don't compute a point to exact-match here — that would compare a
                // 2-component prefix EPK against 3-component item EPKs and drop
                // every row.
                Ok(components) if components.len() < state.metadata.partition_key.paths().len() => {
                    None
                }
                Ok(components) => Some(compute_epk(
                    &components,
                    state.metadata.partition_key.kind(),
                    state.metadata.partition_key.version(),
                )),
                Err(e) => return Err(bad_partition_key_response(e, start)),
            },
            None => None,
        };
        let start_epk = parsed.start_epk.as_deref().map(Epk::from);
        let end_epk = parsed.end_epk.as_deref().map(Epk::from);
        // A query that pins an explicit physical partition key range id must fail
        // with 410/1002 (PartitionKeyRangeGone) when that range no longer exists
        // (e.g. it was split away). Real Cosmos surfaces PartitionKeyRangeGone here
        // so the client refreshes its pkrange cache and re-resolves to the child
        // ranges; returning an empty 200 instead would silently drop the remaining
        // results of a continuation issued before the split.
        if let Some(requested_id) = parsed.partition_key_range_id.as_deref() {
            let exists = state
                .physical_partitions
                .iter()
                .any(|partition| partition.id.to_string() == requested_id);
            if !exists {
                return Err(error_response(
                    StatusCode::Gone,
                    Some(1002),
                    "Gone",
                    "The partition key range specified by the request is no longer present (split/merge).",
                    0.0,
                    "",
                    start,
                )
                .build());
            }
        }
        let mut docs = Vec::new();
        let mut token_parts = Vec::new();
        let mut max_lsn = 0_u64;
        let mut selected_partition: Option<(u32, String)> = None;
        let mut multiple_partitions = false;
        for partition in &state.physical_partitions {
            if parsed
                .partition_key_range_id
                .as_deref()
                .is_some_and(|id| id != partition.id.to_string())
            {
                continue;
            }
            let overlaps_scope = if let Some(requested_epk) = requested_epk.as_ref() {
                partition.contains_epk(requested_epk)
            } else {
                start_epk
                    .as_ref()
                    .is_none_or(|min| partition.epk_max > *min)
                    && end_epk.as_ref().is_none_or(|max| partition.epk_min < *max)
            };
            if !overlaps_scope {
                continue;
            }
            if let Some(response) = check_partition_lock(partition, start) {
                return Err(response);
            }
            match &selected_partition {
                None => selected_partition = Some((partition.id, partition.rid.clone())),
                Some((id, _)) if *id == partition.id => {}
                Some(_) => multiple_partitions = true,
            }
            max_lsn = max_lsn.max(partition.current_lsn());
            let region_id = store.config().region_id_for(region_name);
            token_parts.push(session_token_for(
                partition,
                region_id,
                incoming_session_for(parsed, partition.id).as_ref(),
            ));
            let stored = partition.documents.read().unwrap();
            for (epk, logical) in stored.iter() {
                if requested_epk
                    .as_ref()
                    .is_some_and(|requested| requested != epk)
                {
                    continue;
                }
                if start_epk.as_ref().is_some_and(|min| epk < min) {
                    continue;
                }
                if end_epk.as_ref().is_some_and(|max| epk >= max) {
                    continue;
                }
                docs.extend(logical.iter().map(|(id, doc)| DocumentFeedItem {
                    body: doc.body.clone(),
                    cursor: DocumentFeedCursor {
                        epk: epk.clone(),
                        id: id.clone(),
                    },
                }));
            }
        }
        docs.sort_by(|left, right| left.cursor.cmp(&right.cursor));
        let (partition_key_range_id, internal_partition_id) = if multiple_partitions {
            (None, None)
        } else {
            match selected_partition {
                Some((id, internal_id)) => (Some(id), Some(internal_id)),
                None => (None, None),
            }
        };
        Ok((
            state.metadata.rid.clone(),
            docs,
            token_parts.join(","),
            FeedResponseHeaders {
                session_token: String::new(),
                lsn: Some(max_lsn),
                partition_key_range_id,
                internal_partition_id,
            },
        ))
    });

    match result {
        Some(Ok(v)) => Ok(v),
        Some(Err(response)) => Err(response),
        None => Err(container_not_found(db_id, coll_id, start)),
    }
}

fn handle_read_feed_items(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> AsyncRawResponse {
    match collect_item_documents(store, region_name, parsed, start) {
        Ok((rid, docs, token, mut headers)) => {
            headers.session_token = token;
            success_document_feed_response(
                "Documents",
                rid,
                docs,
                FeedPageOptions::from_request(parsed),
                headers,
                start,
            )
        }
        Err(response) => response,
    }
}

fn handle_query_items(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    match collect_item_documents(store, region_name, parsed, start) {
        Ok((rid, docs, token, mut headers)) => {
            headers.session_token = token;
            execute_document_query_feed(
                "Documents",
                rid,
                docs,
                parsed,
                request_body,
                headers,
                start,
            )
        }
        Err(response) => response,
    }
}

fn local_distinct_type_to_dataflow(
    distinct_type: crate::query::plan::DistinctType,
) -> crate::driver::dataflow::query_plan::DistinctType {
    match distinct_type {
        crate::query::plan::DistinctType::None => {
            crate::driver::dataflow::query_plan::DistinctType::None
        }
        crate::query::plan::DistinctType::Ordered => {
            crate::driver::dataflow::query_plan::DistinctType::Ordered
        }
        crate::query::plan::DistinctType::Unordered => {
            crate::driver::dataflow::query_plan::DistinctType::Unordered
        }
    }
}

fn local_sort_order_to_dataflow(
    sort_order: crate::query::plan::SortOrder,
) -> crate::driver::dataflow::query_plan::SortOrder {
    match sort_order {
        crate::query::plan::SortOrder::Ascending => {
            crate::driver::dataflow::query_plan::SortOrder::Ascending
        }
        crate::query::plan::SortOrder::Descending => {
            crate::driver::dataflow::query_plan::SortOrder::Descending
        }
    }
}

fn local_query_info_to_dataflow(
    info: crate::query::plan::LocalQueryInfo,
) -> crate::driver::dataflow::query_plan::QueryInfo {
    crate::driver::dataflow::query_plan::QueryInfo {
        distinct_type: local_distinct_type_to_dataflow(info.distinct_type),
        top: info.top.map(|v| v as u64),
        offset: info.offset.map(|v| v as u64),
        limit: info.limit.map(|v| v as u64),
        order_by: info
            .order_by
            .into_iter()
            .map(local_sort_order_to_dataflow)
            .collect(),
        order_by_expressions: info.order_by_expressions,
        group_by_expressions: info.group_by_expressions,
        group_by_aliases: Vec::new(),
        aggregates: info
            .aggregates
            .into_iter()
            .map(|a| format!("{a:?}"))
            .collect(),
        group_by_alias_to_aggregate_type: HashMap::new(),
        rewritten_query: Some(String::new()),
        has_select_value: info.has_select_value,
        has_non_streaming_order_by: false,
    }
}

fn full_query_range() -> crate::driver::dataflow::query_plan::QueryRange {
    crate::driver::dataflow::query_plan::QueryRange {
        min: Epk::MIN.to_hex(),
        max: Epk::MAX.to_hex(),
        is_min_inclusive: true,
        is_max_inclusive: false,
    }
}

fn epk_range_to_query_range(
    range: std::ops::Range<EffectivePartitionKey>,
) -> crate::driver::dataflow::query_plan::QueryRange {
    crate::driver::dataflow::query_plan::QueryRange {
        min: range.start.to_hex(),
        max: range.end.to_hex(),
        is_min_inclusive: true,
        is_max_inclusive: true,
    }
}

fn model_partition_key_values(
    values: &[crate::query::plan::PartitionKeyValue],
) -> crate::error::Result<Vec<ModelPartitionKeyValue>> {
    values
        .iter()
        .map(|value| match value {
            crate::query::plan::PartitionKeyValue::String(s) => {
                Ok(ModelPartitionKeyValue::from(s.clone()))
            }
            crate::query::plan::PartitionKeyValue::Number(n) => {
                Ok(ModelPartitionKeyValue::from(*n))
            }
            crate::query::plan::PartitionKeyValue::Bool(b) => Ok(ModelPartitionKeyValue::from(*b)),
            crate::query::plan::PartitionKeyValue::Null => Ok(ModelPartitionKeyValue::NULL),
            crate::query::plan::PartitionKeyValue::Undefined => {
                Ok(ModelPartitionKeyValue::UNDEFINED)
            }
            crate::query::plan::PartitionKeyValue::UnboundParameter(name) => {
                Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(StatusCode::BadRequest))
                    .with_message(format!(
                        "query plan partition key filter references unbound parameter @{name}"
                    ))
                    .build())
            }
            crate::query::plan::PartitionKeyValue::InvalidParameter { name, reason } => {
                Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(StatusCode::BadRequest))
                    .with_message(format!(
                        "query plan partition key filter parameter @{name} is invalid: {reason}"
                    ))
                    .build())
            }
        })
        .collect()
}

fn query_ranges_from_pk_filter(
    filter: &crate::query::plan::PartitionKeyFilter,
    pk_definition: &PartitionKeyDefinition,
) -> crate::error::Result<Vec<crate::driver::dataflow::query_plan::QueryRange>> {
    match filter {
        crate::query::plan::PartitionKeyFilter::Equality(values) => {
            let values = model_partition_key_values(values)?;
            let range = EffectivePartitionKey::compute_range(&values, pk_definition)?;
            Ok(vec![epk_range_to_query_range(range)])
        }
        crate::query::plan::PartitionKeyFilter::InList(value_sets) => value_sets
            .iter()
            .map(|values| {
                let values = model_partition_key_values(values)?;
                EffectivePartitionKey::compute_range(&values, pk_definition)
                    .map(epk_range_to_query_range)
            })
            .collect(),
        crate::query::plan::PartitionKeyFilter::Contradictory => Ok(Vec::new()),
        crate::query::plan::PartitionKeyFilter::Unconstrained
        | crate::query::plan::PartitionKeyFilter::NotEvaluated => Ok(vec![full_query_range()]),
    }
}

fn handle_query_plan(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    if !region_ref.database_exists(db_id) {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            0.0,
            "",
            start,
        )
        .build();
    }
    let Some(container) = region_ref.get_container(db_id, coll_id) else {
        return container_not_found(db_id, coll_id, start);
    };
    let (query, parameters) = match parse_query_spec(request_body, start) {
        Ok(v) => v,
        Err(response) => return response,
    };
    let program = match crate::query::parse(&query) {
        Ok(program) => program,
        Err(e) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                &format!("failed to parse query: {e}"),
                0.0,
                "",
                start,
            )
            .build();
        }
    };
    let pk_paths: Vec<&str> = container
        .metadata
        .partition_key
        .paths()
        .iter()
        .map(|p| p.as_ref())
        .collect();
    let local_plan = match crate::query::plan::generate_query_plan_with_parameters(
        &program.query,
        &pk_paths,
        &parameters,
    ) {
        Ok(plan) => plan,
        Err(e) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                &e.to_string(),
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    let query_ranges = match query_ranges_from_pk_filter(
        &local_plan.pk_filters,
        &container.metadata.partition_key,
    ) {
        Ok(ranges) => ranges,
        Err(e) => {
            return error_response(
                e.status().status_code(),
                e.status().sub_status().map(|s| u32::from(s.value())),
                "BadRequest",
                &e.to_string(),
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    let plan = crate::driver::dataflow::query_plan::QueryPlan {
        partitioned_query_execution_info_version: 2,
        query_info: Some(local_query_info_to_dataflow(local_plan.query_info)),
        query_ranges,
        hybrid_search_query_info: None,
    };
    let mut body = match serde_json::to_value(plan) {
        Ok(body) => body,
        Err(e) => {
            return error_response(
                StatusCode::InternalServerError,
                None,
                "InternalError",
                &format!("failed to serialize query plan: {e}"),
                0.0,
                "",
                start,
            )
            .build();
        }
    };
    if let Some(query_info) = body.get_mut("queryInfo").and_then(|v| v.as_object_mut()) {
        query_info.insert("dCountInfo".to_owned(), serde_json::Value::Null);
    }
    success_response(StatusCode::Ok, &body, 1.0, "", start)
        .with_item_count(1)
        .build()
}

#[derive(Clone, Deserialize)]
#[serde(tag = "operationType", rename_all_fields = "camelCase")]
enum BatchOperation {
    Create {
        id: Option<String>,
        resource_body: serde_json::Value,
    },
    Upsert {
        id: Option<String>,
        resource_body: serde_json::Value,
        #[serde(default)]
        if_match: Option<String>,
        #[serde(default)]
        if_none_match: Option<String>,
    },
    Replace {
        id: String,
        resource_body: serde_json::Value,
        #[serde(default)]
        if_match: Option<String>,
    },
    Read {
        id: String,
        #[serde(default)]
        if_match: Option<String>,
        #[serde(default)]
        if_none_match: Option<String>,
    },
    Delete {
        id: String,
        #[serde(default)]
        if_match: Option<String>,
    },
}

fn batch_result(
    status_code: u16,
    resource_body: Option<serde_json::Value>,
    etag: Option<&str>,
    request_charge: f64,
) -> serde_json::Value {
    let mut result = serde_json::Map::new();
    result.insert("statusCode".to_string(), serde_json::json!(status_code));
    if let Some(body) = resource_body {
        result.insert("resourceBody".to_string(), body);
    }
    if let Some(etag) = etag {
        result.insert("eTag".to_string(), serde_json::json!(etag));
    }
    result.insert(
        "requestCharge".to_string(),
        serde_json::json!(request_charge),
    );
    serde_json::Value::Object(result)
}

fn failed_batch_results(
    len: usize,
    failure_index: usize,
    failure_status: u16,
    failure_body: Option<serde_json::Value>,
) -> Vec<serde_json::Value> {
    (0..len)
        .map(|i| {
            if i == failure_index {
                batch_result(failure_status, failure_body.clone(), None, 1.0)
            } else {
                batch_result(424, None, None, 1.0)
            }
        })
        .collect()
}

fn batch_bad_request(message: impl AsRef<str>, start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::BadRequest,
        None,
        "BadRequest",
        message.as_ref(),
        0.0,
        "",
        start,
    )
    .build()
}

fn batch_doc_id(
    explicit_id: Option<&str>,
    body: &serde_json::Value,
    start: Instant,
) -> Result<String, AsyncRawResponse> {
    let body_id = body.get("id").and_then(|v| v.as_str());
    match (explicit_id, body_id) {
        (Some(id), Some(body_id)) if id != body_id => Err(batch_bad_request(
            "Document id in request body must match the batch operation id",
            start,
        )),
        (Some(id), _) => Ok(id.to_string()),
        (None, Some(body_id)) => Ok(body_id.to_string()),
        (None, None) => Err(batch_bad_request("Missing 'id' field in document", start)),
    }
}

fn validate_batch_body_partition_key(
    body: &serde_json::Value,
    expected_components: &[super::epk::PartitionKeyComponent],
    meta: &ContainerMetadata,
    start: Instant,
) -> Result<(), AsyncRawResponse> {
    let body_components = extract_pk_from_body(body, meta.partition_key.paths())
        .map_err(|e| bad_partition_key_response(e, start))?;
    if body_components != expected_components {
        return Err(batch_bad_request(
            "Transactional batch operations must use the batch partition key",
            start,
        ));
    }
    Ok(())
}

async fn handle_batch(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    #[cfg(feature = "preview_dtx")]
    let write_lock = store.document_write_lock();
    #[cfg(feature = "preview_dtx")]
    let _write_guard = write_lock.lock().await;

    const MAX_BATCH_OPERATIONS: usize = 100;
    const MAX_BATCH_PAYLOAD_BYTES: usize = 2 * 1024 * 1024;

    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");

    if request_body.len() > MAX_BATCH_PAYLOAD_BYTES {
        return error_response(
            StatusCode::PayloadTooLarge,
            None,
            "RequestEntityTooLarge",
            "Transactional batch payload exceeds the maximum allowed size",
            0.0,
            "",
            start,
        )
        .build();
    }

    let operations: Vec<BatchOperation> = match serde_json::from_slice(request_body) {
        Ok(ops) => ops,
        Err(e) => return batch_bad_request(format!("Invalid batch JSON body: {e}"), start),
    };
    if operations.len() > MAX_BATCH_OPERATIONS {
        return batch_bad_request("Transactional batch cannot exceed 100 operations", start);
    }

    let batch_pk_components = match parsed.partition_key_header.as_deref() {
        Some(header) => match parse_partition_key_header(header) {
            Ok(components) if !components.is_empty() => components,
            Ok(_) => {
                return batch_bad_request(
                    "Transactional batch requires a non-empty partition key",
                    start,
                )
            }
            Err(e) => return bad_partition_key_response(e, start),
        },
        None => {
            return batch_bad_request(
                "Transactional batch requires x-ms-documentdb-partitionkey",
                start,
            )
        }
    };

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };
    if !region_ref.database_exists(db_id) {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            0.0,
            "",
            start,
        )
        .build();
    }

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let epk = compute_epk(
            &batch_pk_components,
            state.metadata.partition_key.kind(),
            state.metadata.partition_key.version(),
        );
        let partition = match state.find_partition(&epk) {
            Some(p) => p,
            None => {
                return Err(error_response(
                    StatusCode::InternalServerError,
                    None,
                    "InternalError",
                    "No partition found for EPK",
                    1.0,
                    "",
                    start,
                )
                .build());
            }
        };
        if let Some(response) = check_partition_lock(partition, start) {
            return Err(response);
        }

        let has_write = operations
            .iter()
            .any(|op| !matches!(op, BatchOperation::Read { .. }));
        // A transactional batch must evaluate all operations against one
        // stable partition snapshot, including read-only batches. Holding the
        // document write lock prevents concurrent point writes from changing
        // the snapshot while the batch is being evaluated.
        let mut docs_guard = partition.documents.write().unwrap();
        let mut working_docs = docs_guard.clone();
        let batch_lsn = if has_write {
            partition.current_lsn() + 1
        } else {
            partition.current_lsn()
        };
        let mut results = Vec::with_capacity(operations.len());
        let mut changes: Vec<(StoredDocument, bool)> = Vec::new();

        for (index, operation) in operations.iter().enumerate() {
            let logical = working_docs.entry(epk.clone()).or_default();
            match operation {
                BatchOperation::Create { id, resource_body } => {
                    validate_batch_body_partition_key(
                        resource_body,
                        &batch_pk_components,
                        &state.metadata,
                        start,
                    )?;
                    let doc_id = batch_doc_id(id.as_deref(), resource_body, start)?;
                    if logical.contains_key(&doc_id) {
                        results = failed_batch_results(operations.len(), index, 409, None);
                        return Ok((results, Vec::new(), String::new(), 1.0, None, None));
                    }
                    let mut body = resource_body.clone();
                    let (_, doc_rid) = store.rid_generator().next_document_rid(
                        state.metadata.numeric_db_id,
                        state.metadata.numeric_coll_id,
                    );
                    let ts = current_timestamp();
                    let etag = new_etag();
                    let self_link = format!("{}docs/{}/", state.metadata.self_link, doc_rid);
                    inject_system_properties(&doc_rid, &self_link, &etag, ts, &mut body);
                    let body_size_bytes = serde_json::to_vec(resource_body).map_or(0, |v| v.len());
                    let stored = StoredDocument {
                        body: body.clone(),
                        id: doc_id.clone(),
                        rid: doc_rid,
                        etag: etag.clone(),
                        ts,
                        self_link,
                        lsn: batch_lsn,
                        epk: epk.clone(),
                        body_size_bytes,
                        source_region: region_name.to_string(),
                    };
                    logical.insert(doc_id, stored.clone());
                    changes.push((stored.clone(), false));
                    results.push(batch_result(
                        201,
                        parsed.content_response_on_write.then_some(body),
                        Some(&etag),
                        1.0,
                    ));
                }
                BatchOperation::Upsert {
                    id,
                    resource_body,
                    if_match,
                    if_none_match,
                } => {
                    validate_batch_body_partition_key(
                        resource_body,
                        &batch_pk_components,
                        &state.metadata,
                        start,
                    )?;
                    let doc_id = batch_doc_id(id.as_deref(), resource_body, start)?;
                    if let Some(existing) = logical.get(&doc_id) {
                        if if_match.as_ref().is_some_and(|etag| etag != &existing.etag)
                            || if_none_match.as_deref() == Some("*")
                        {
                            results = failed_batch_results(operations.len(), index, 412, None);
                            return Ok((results, Vec::new(), String::new(), 1.0, None, None));
                        }
                    }
                    let status = if logical.contains_key(&doc_id) {
                        200
                    } else {
                        201
                    };
                    let mut body = resource_body.clone();
                    let (doc_rid, self_link) = logical
                        .get(&doc_id)
                        .map(|existing| (existing.rid.clone(), existing.self_link.clone()))
                        .unwrap_or_else(|| {
                            let (_, rid) = store.rid_generator().next_document_rid(
                                state.metadata.numeric_db_id,
                                state.metadata.numeric_coll_id,
                            );
                            let link = format!("{}docs/{}/", state.metadata.self_link, rid);
                            (rid, link)
                        });
                    let ts = current_timestamp();
                    let etag = new_etag();
                    inject_system_properties(&doc_rid, &self_link, &etag, ts, &mut body);
                    let body_size_bytes = serde_json::to_vec(resource_body).map_or(0, |v| v.len());
                    let stored = StoredDocument {
                        body: body.clone(),
                        id: doc_id.clone(),
                        rid: doc_rid,
                        etag: etag.clone(),
                        ts,
                        self_link,
                        lsn: batch_lsn,
                        epk: epk.clone(),
                        body_size_bytes,
                        source_region: region_name.to_string(),
                    };
                    logical.insert(doc_id, stored.clone());
                    changes.push((stored.clone(), false));
                    results.push(batch_result(
                        status,
                        parsed.content_response_on_write.then_some(body),
                        Some(&etag),
                        1.0,
                    ));
                }
                BatchOperation::Replace {
                    id,
                    resource_body,
                    if_match,
                } => {
                    validate_batch_body_partition_key(
                        resource_body,
                        &batch_pk_components,
                        &state.metadata,
                        start,
                    )?;
                    let doc_id = batch_doc_id(Some(id), resource_body, start)?;
                    let Some(existing) = logical.get(&doc_id).cloned() else {
                        results = failed_batch_results(operations.len(), index, 404, None);
                        return Ok((results, Vec::new(), String::new(), 1.0, None, None));
                    };
                    if if_match.as_ref().is_some_and(|etag| etag != &existing.etag) {
                        results = failed_batch_results(operations.len(), index, 412, None);
                        return Ok((results, Vec::new(), String::new(), 1.0, None, None));
                    }
                    let mut body = resource_body.clone();
                    let ts = current_timestamp();
                    let etag = new_etag();
                    inject_system_properties(
                        &existing.rid,
                        &existing.self_link,
                        &etag,
                        ts,
                        &mut body,
                    );
                    let body_size_bytes = serde_json::to_vec(resource_body).map_or(0, |v| v.len());
                    let stored = StoredDocument {
                        body: body.clone(),
                        id: doc_id.clone(),
                        rid: existing.rid,
                        etag: etag.clone(),
                        ts,
                        self_link: existing.self_link,
                        lsn: batch_lsn,
                        epk: epk.clone(),
                        body_size_bytes,
                        source_region: region_name.to_string(),
                    };
                    logical.insert(doc_id, stored.clone());
                    changes.push((stored.clone(), false));
                    results.push(batch_result(
                        200,
                        parsed.content_response_on_write.then_some(body),
                        Some(&etag),
                        1.0,
                    ));
                }
                BatchOperation::Read {
                    id,
                    if_match,
                    if_none_match,
                } => {
                    let Some(existing) = logical.get(id) else {
                        results = failed_batch_results(operations.len(), index, 404, None);
                        return Ok((results, Vec::new(), String::new(), 1.0, None, None));
                    };
                    if if_match.as_ref().is_some_and(|etag| etag != &existing.etag) {
                        results = failed_batch_results(operations.len(), index, 412, None);
                        return Ok((results, Vec::new(), String::new(), 1.0, None, None));
                    }
                    if if_none_match
                        .as_ref()
                        .is_some_and(|etag| etag == &existing.etag)
                    {
                        results.push(batch_result(304, None, Some(&existing.etag), 1.0));
                    } else {
                        results.push(batch_result(
                            200,
                            Some(existing.body.clone()),
                            Some(&existing.etag),
                            1.0,
                        ));
                    }
                }
                BatchOperation::Delete { id, if_match } => {
                    let Some(existing) = logical.get(id).cloned() else {
                        results = failed_batch_results(operations.len(), index, 404, None);
                        return Ok((results, Vec::new(), String::new(), 1.0, None, None));
                    };
                    if if_match.as_ref().is_some_and(|etag| etag != &existing.etag) {
                        results = failed_batch_results(operations.len(), index, 412, None);
                        return Ok((results, Vec::new(), String::new(), 1.0, None, None));
                    }
                    logical.remove(id);
                    let tombstone = StoredDocument {
                        body: serde_json::Value::Null,
                        id: id.clone(),
                        rid: existing.rid,
                        etag: existing.etag.clone(),
                        ts: current_timestamp(),
                        self_link: existing.self_link,
                        lsn: batch_lsn,
                        epk: epk.clone(),
                        body_size_bytes: 0,
                        source_region: region_name.to_string(),
                    };
                    changes.push((tombstone, true));
                    results.push(batch_result(204, None, Some(&existing.etag), 1.0));
                }
            }
        }

        if has_write {
            *docs_guard = working_docs;
            partition.advance_lsn();
            partition.advance_local_lsn();
        }
        let documents_in_partition = docs_guard
            .values()
            .map(std::collections::BTreeMap::len)
            .sum::<usize>();
        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(
            partition,
            region_id,
            incoming_session_for(parsed, partition.id).as_ref(),
        );
        let headers = Some(PointResponseHeaders::from_partition_snapshot(
            partition,
            store.next_transport_request_id(),
            documents_in_partition,
        ));
        let charge = results
            .iter()
            .filter_map(|r| r.get("requestCharge").and_then(|v| v.as_f64()))
            .sum::<f64>();
        Ok((results, changes, token, charge, headers, Some(batch_lsn)))
    });

    match result {
        Some(Ok((results, changes, token, charge, headers, lsn))) => {
            for (doc, is_delete) in changes {
                store.replicate(region_name, db_id, coll_id, &doc, is_delete);
            }
            // A real Cosmos DB account returns 207 MultiStatus when any
            // individual operation in the batch failed (statusCode >= 300),
            // and 200 OK only when every operation succeeded.
            let has_failure = results.iter().any(|r| {
                r.get("statusCode")
                    .and_then(|v| v.as_u64())
                    .is_some_and(|s| s >= 300)
            });
            let status = if has_failure {
                StatusCode::MultiStatus
            } else {
                StatusCode::Ok
            };
            let body = serde_json::Value::Array(results);
            let mut builder = success_response(status, &body, charge, &token, start);
            if let Some(lsn) = lsn {
                builder = builder.with_lsn(lsn);
            }
            decorate_point_response(builder, headers, None).build()
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id, start),
    }
}

// --- Point Operations ---

/// Resolves the partition key components and EPK for a point operation.
///
/// Returns `BadRequest` when the partition key header or extracted document
/// values are malformed (matches gateway behavior so client bugs surface
/// with the same status code as against a real account).
fn resolve_partition_key(
    parsed: &ParsedRequest,
    body: &serde_json::Value,
    meta: &ContainerMetadata,
) -> crate::error::Result<(Vec<super::epk::PartitionKeyComponent>, Epk)> {
    let pk_components = if let Some(pk_header) = &parsed.partition_key_header {
        parse_partition_key_header(pk_header)?
    } else if body.is_null() {
        // Read / Delete callers pass a `Null` body — there is nothing to
        // extract a partition key from. Real Cosmos rejects point operations
        // that omit the partition key header in this case with 400 BadRequest;
        // mirror that so dual-backend tests stay consistent.
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("missing 'x-ms-documentdb-partitionkey' header on point operation")
            .build());
    } else {
        extract_pk_from_body(body, meta.partition_key.paths())?
    };

    let epk = compute_epk(
        &pk_components,
        meta.partition_key.kind(),
        meta.partition_key.version(),
    );

    Ok((pk_components, epk))
}

/// Builds a 400 BadRequest response from a partition-key resolution error.
fn bad_partition_key_response(err: crate::error::CosmosError, start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::BadRequest,
        None,
        "BadRequest",
        &err.to_string(),
        0.0,
        "",
        start,
    )
    .build()
}

/// Builds a V2 session token for a partition in the given region.
///
/// `current_local_lsn` reflects the writes applied at *this* region (locally
/// produced + replicated in) and is the value the real Cosmos DB gateway
/// includes in the per-region segment of the token. Using `current_lsn`
/// (which tracks the global high-water LSN) for both components produces
/// tokens that look correct only on single-region accounts.
fn session_token_for(
    partition: &PhysicalPartition,
    region_id: u64,
    incoming: Option<&SessionToken>,
) -> String {
    use super::session::{LocalLsn, RegionId};
    let prior: &[(u64, u64)] = incoming.map_or(&[], |t| t.region_progress.as_slice());
    SessionToken::format_v2(
        partition.id,
        partition.current_version(),
        partition.current_lsn(),
        RegionId(region_id),
        LocalLsn(partition.current_local_lsn()),
        prior,
    )
}

/// Pulls the incoming session-token entry for a specific partition out of the
/// request, if any. Used so the response token can preserve per-region
/// progress the client has already accumulated for partitions other than the
/// local one. Malformed composite tokens are silently treated as missing
/// (handlers that need to surface a 400 do so independently).
fn incoming_session_for(parsed: &ParsedRequest, pkrange_id: u32) -> Option<SessionToken> {
    let raw = parsed.session_token.as_deref()?;
    let tokens = super::session::parse_composite_session_token(raw).ok()?;
    tokens.into_iter().find(|t| t.pkrange_id == pkrange_id)
}

pub(crate) struct PointResponseHeaders {
    partition_key_range_id: u32,
    internal_partition_id: String,
    transport_request_id: u32,
    global_committed_lsn: u64,
    quorum_acked_lsn: u64,
    quorum_acked_local_lsn: u64,
    local_lsn: u64,
    resource_usage: String,
}

impl PointResponseHeaders {
    /// Builds the response-header snapshot from a partition reference.
    ///
    /// Captured under the containers read lock but **after** the per-partition
    /// write lock has already been released, so the document-count component of
    /// x-ms-resource-usage is best-effort and may race with concurrent
    /// writers on the same partition. This matches real Cosmos DB, where
    /// x-ms-resource-usage is also a best-effort snapshot.
    fn from_partition(partition: &PhysicalPartition, transport_request_id: u32) -> Self {
        let documents = partition.documents.read().unwrap();
        let documents_in_partition = documents
            .values()
            .map(std::collections::BTreeMap::len)
            .sum::<usize>();
        Self::from_partition_snapshot(partition, transport_request_id, documents_in_partition)
    }

    fn from_partition_snapshot(
        partition: &PhysicalPartition,
        transport_request_id: u32,
        documents_in_partition: usize,
    ) -> Self {
        Self {
            partition_key_range_id: partition.id,
            internal_partition_id: partition.rid.clone(),
            transport_request_id,
            global_committed_lsn: partition.current_lsn(),
            quorum_acked_lsn: partition.current_lsn(),
            quorum_acked_local_lsn: partition.current_local_lsn(),
            local_lsn: partition.current_local_lsn(),
            resource_usage: format!(
                "documentSize=0;documentsSize={documents_in_partition};documentsCount={documents_in_partition};collectionSize={documents_in_partition};"
            ),
        }
    }
}

fn decorate_point_response(
    builder: ResponseBuilder,
    headers: Option<PointResponseHeaders>,
    item_lsn: Option<u64>,
) -> ResponseBuilder {
    let Some(headers) = headers else {
        return builder;
    };

    let builder = builder
        .with_header_value(
            PARTITION_KEY_RANGE_ID.clone(),
            headers.partition_key_range_id,
        )
        .with_header_value(INTERNAL_PARTITION_ID.clone(), headers.internal_partition_id)
        .with_header_value(TRANSPORT_REQUEST_ID.clone(), headers.transport_request_id)
        .with_header_value(GLOBAL_COMMITTED_LSN.clone(), headers.global_committed_lsn)
        .with_header_value(QUORUM_ACKED_LSN.clone(), headers.quorum_acked_lsn)
        .with_header_value(
            QUORUM_ACKED_LOCAL_LSN.clone(),
            headers.quorum_acked_local_lsn,
        )
        .with_header_value(LOCAL_LSN.clone(), headers.local_lsn)
        .with_header_value(NUMBER_OF_READ_REGIONS.clone(), 0)
        .with_header_value(
            LAST_STATE_CHANGE_UTC.clone(),
            "Thu, 01 Jan 1970 00:00:00 GMT",
        )
        // GATEWAY_VERSION is intentionally NOT overridden here — `ResponseBuilder::new`
        // already pre-seeds it to `"version=emulator"` for every response. Doc-plane and
        // control-plane responses both flow through that default, so dual-backend tests
        // do not need a per-handler allowlist for divergent gateway version values.
        .with_header_value(SERVICE_VERSION.clone(), "version=emulator")
        .with_header_value(
            RESOURCE_QUOTA.clone(),
            "documentSize=10240;documentsSize=10485760;documentsCount=-1;collectionSize=10485760;",
        )
        .with_header_value(RESOURCE_USAGE.clone(), headers.resource_usage);

    if let Some(item_lsn) = item_lsn {
        builder
            .with_header_value(ITEM_LSN.clone(), item_lsn)
            .with_header_value(ITEM_LOCAL_LSN.clone(), headers.local_lsn)
    } else {
        builder
    }
}

/// Returns a 410/1007 response if the partition is locked (split/merge in progress).
fn check_partition_lock(partition: &PhysicalPartition, start: Instant) -> Option<AsyncRawResponse> {
    if partition.is_locked() {
        Some(
            error_response(
                StatusCode::Gone,
                Some(PARTITION_SPLIT_OR_MERGE_SUBSTATUS.into()),
                "Gone",
                "Partition is being split or merged.",
                0.0,
                "",
                start,
            )
            .build(),
        )
    } else {
        None
    }
}

/// Returns a 429/3200 response if the partition's throughput budget is exhausted.
fn check_throttle(
    partition: &PhysicalPartition,
    charge: f64,
    throttling_enabled: bool,
    start: Instant,
) -> Option<AsyncRawResponse> {
    if !throttling_enabled {
        return None;
    }
    if let Some(tracker) = &partition.throughput_tracker {
        if let Err(retry_after_ms) = tracker.try_consume(charge) {
            return Some(
                error_response(
                    StatusCode::TooManyRequests,
                    Some(3200),
                    "TooManyRequests",
                    "Request rate is large. Please retry after sometime.",
                    0.0,
                    "",
                    start,
                )
                .with_retry_after_ms(retry_after_ms)
                .build(),
            );
        }
    }
    None
}

async fn handle_create(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    #[cfg(feature = "preview_dtx")]
    let write_lock = store.document_write_lock();
    #[cfg(feature = "preview_dtx")]
    let _write_guard = write_lock.lock().await;

    handle_create_locked(store, region_name, parsed, request_body, start).await
}

async fn handle_create_locked(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");

    if let Some(resp) = replication_back_pressure_response(store, region_name, start) {
        return resp;
    }

    let mut body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    let doc_id = match body.get("id").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Missing 'id' field in document",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let (_, epk) = match resolve_partition_key(parsed, &body, &state.metadata) {
            Ok(v) => v,
            Err(e) => return Err(bad_partition_key_response(e, start)),
        };

        let partition = match state.find_partition(&epk) {
            Some(p) => p,
            None => {
                return Err(error_response(
                    StatusCode::InternalServerError,
                    None,
                    "InternalError",
                    "No partition found for EPK",
                    1.0,
                    "",
                    start,
                )
                .build());
            }
        };

        // Check partition lock (split/merge in progress)
        if let Some(response) = check_partition_lock(partition, start) {
            return Err(response);
        }

        // Check for conflict
        {
            let docs = partition.documents.read().unwrap();
            if let Some(logical) = docs.get(&epk) {
                if logical.contains_key(&doc_id) {
                    let region_id = store.config().region_id_for(region_name);
                    let token = session_token_for(partition, region_id, incoming_session_for(parsed, partition.id).as_ref());
                    return Err(error_response(
                        StatusCode::Conflict,
                        None,
                        "Conflict",
                        &format!(
                            "Entity with the specified id already exists in the system. ResourceId: {}",
                            doc_id
                        ),
                        1.0,
                        &token,
                        start,
                    )
                    .build());
                }
            }
        }

        // Compute RU charge eagerly, but do NOT debit the throttle bucket
        // until we are sure we will commit the write. Throttling under the
        // read-lock probe means concurrent conflicts (returning 1.0 RU) would
        // mismatch the bucket debit, producing non-deterministic
        // RU-budget assertions in throttling tests.
        let num_props = RuChargingModel::count_properties(&body);
        let charge = store
            .config()
            .ru_model()
            .compute_create_ru(request_body.len(), num_props);

        let stored_doc = {
            let mut docs = partition.documents.write().unwrap();
            let logical = docs.entry(epk.clone()).or_default();
            if logical.contains_key(&doc_id) {
                let region_id = store.config().region_id_for(region_name);
                let token = session_token_for(partition, region_id, incoming_session_for(parsed, partition.id).as_ref());
                return Err(error_response(
                    StatusCode::Conflict,
                    None,
                    "Conflict",
                    &format!(
                        "Entity with the specified id already exists in the system. ResourceId: {}",
                        doc_id
                    ),
                    1.0,
                    &token,
                    start,
                )
                .build());
            }

            // Debit the throttle bucket only now that the conflict check has
            // passed under the write lock: on a 429 the response
            // RU charge matches the actual debit.
            if let Some(response) = check_throttle(partition, charge, store.config().throttling_enabled(), start) {
                return Err(response);
            }

            let lsn = partition.advance_lsn();
            partition.advance_local_lsn();
            let (_, doc_rid) = store.rid_generator().next_document_rid(
                state.metadata.numeric_db_id,
                state.metadata.numeric_coll_id,
            );
            let ts = current_timestamp();
            let etag = new_etag();
            let self_link = format!("{}docs/{}/", state.metadata.self_link, doc_rid);

            inject_system_properties(&doc_rid, &self_link, &etag, ts, &mut body);
            // Cache the *wire* size (the bytes the caller sent), not the
            // post-injection size, so read-RU and create-RU evaluate the
            // same `compute_..._ru(size)` formula on identical inputs.
            // Without this the same doc was charged 1 KB on create and 2 KB
            // on read whenever the system-prop overhead pushed it across a
            // power-of-two bucket.
            let body_size_bytes = request_body.len();
            let stored_doc = StoredDocument {
                body: body.clone(),
                id: doc_id.clone(),
                rid: doc_rid,
                etag: etag.clone(),
                ts,
                self_link,
                lsn,
                epk: epk.clone(),
                body_size_bytes,
                source_region: region_name.to_string(),
            };
            logical.insert(doc_id.clone(), stored_doc.clone());
            stored_doc
        };

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(partition, region_id, incoming_session_for(parsed, partition.id).as_ref());
        let headers = Some(PointResponseHeaders::from_partition(
            partition,
            store.next_transport_request_id(),
        ));

        Ok((stored_doc, token, charge, body, headers))
    });

    match result {
        Some(Ok((doc, token, charge, response_body, headers))) => {
            // Trigger replication
            store.replicate(region_name, db_id, coll_id, &doc, false);

            let builder = if parsed.content_response_on_write {
                success_response(StatusCode::Created, &response_body, charge, &token, start)
                    .with_etag(&doc.etag)
                    .with_lsn(doc.lsn)
            } else {
                ResponseBuilder::new(StatusCode::Created, start)
                    .with_request_charge(charge)
                    .with_session_token(&token)
                    .with_etag(&doc.etag)
                    .with_lsn(doc.lsn)
            };

            decorate_point_response(builder, headers, Some(doc.lsn)).build()
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id, start),
    }
}

fn handle_read(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");
    let doc_id = parsed.doc_id.as_deref().unwrap_or("");

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let empty_body = serde_json::Value::Null;
        let (_, epk) = match resolve_partition_key(parsed, &empty_body, &state.metadata) {
            Ok(v) => v,
            Err(e) => return Err(bad_partition_key_response(e, start)),
        };

        let partition = match state.find_partition(&epk) {
            Some(p) => p,
            None => {
                return Err(error_response(
                    StatusCode::InternalServerError,
                    None,
                    "InternalError",
                    "No partition found for EPK",
                    1.0,
                    "",
                    start,
                )
                .build());
            }
        };

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(
            partition,
            region_id,
            incoming_session_for(parsed, partition.id).as_ref(),
        );

        // Check partition lock
        if let Some(response) = check_partition_lock(partition, start) {
            return Err(response);
        }

        // Check forced session unavailability (one-shot)
        if partition
            .session_state
            .check_and_clear_forced_for(&epk.to_hex())
        {
            return Err(error_response(
                StatusCode::NotFound,
                Some(1002),
                "ReadSessionNotAvailable",
                "The read session is not available for the input session token.",
                0.0,
                &token,
                start,
            )
            .build());
        }

        // Session consistency check (V2-aware: compare version first, then globalLSN).
        //
        // On a 1002 (ReadSessionNotAvailable) the response token echoes the
        // *requested* LSN/version rather than the partition's current
        // watermark. Returning the partition's higher LSN would mislead the
        // client into thinking its caught up — the caller would retry with
        // a token that the partition trivially satisfies and treat the
        // failure as transient. Echoing back what they asked for makes the
        // mismatch visible.
        if store.config().consistency().is_session() {
            if let Some(session_header) = &parsed.session_token {
                let tokens = match super::session::parse_composite_session_token(session_header) {
                    Ok(tokens) => tokens,
                    Err(parse_err) => {
                        return Err(error_response(
                            StatusCode::BadRequest,
                            None,
                            "BadRequest",
                            &format!("Invalid session token: {}", parse_err),
                            0.0,
                            &token,
                            start,
                        )
                        .build());
                    }
                };
                // Reject stale pkrange ids (e.g. parent of a completed split that
                // is *not* an ancestor of this request's partition) with 410/1002
                // — real Cosmos surfaces PartitionKeyRangeGone here so the client
                // refreshes its pkrange cache and retries. Without this, a stale
                // token referencing some other (now-defunct) partition silently
                // skipped the consistency check.
                //
                // Tokens referencing a *direct ancestor* of this partition are
                // considered valid: the EPK-routed successor partition's LSN is
                // at least as advanced as any pre-split LSN the client could
                // legitimately have observed, so the consistency check below is
                // satisfied trivially. This matches the real gateway, which
                // routes by EPK and treats stale-but-related tokens as best-
                // effort rather than fatal.
                for st in &tokens {
                    if st.pkrange_id == super::store::MASTER_PARTITION_ID
                        || st.pkrange_id == partition.id
                        || partition.parents.contains(&st.pkrange_id)
                    {
                        continue;
                    }
                    let exists = state
                        .physical_partitions
                        .iter()
                        .any(|p| p.id == st.pkrange_id);
                    if !exists {
                        return Err(error_response(
                            StatusCode::Gone,
                            Some(1002),
                            "Gone",
                            "The partition key range referenced by the session token is no longer present (split/merge).",
                            0.0,
                            &token,
                            start,
                        )
                        .build());
                    }
                }
                for st in &tokens {
                    if st.pkrange_id == partition.id {
                        let partition_version = partition.current_version();
                        // 1002 echoes back what the client requested. We
                        // intentionally pass `LocalLsn(st.global_lsn)` so the
                        // emitted token mirrors the requested global LSN —
                        // this is *not* the partition's true local LSN. See
                        // the comment block above for why echoing is needed.
                        let request_token = SessionToken::format_v2(
                            partition.id,
                            st.version,
                            st.global_lsn,
                            super::session::RegionId(region_id),
                            super::session::LocalLsn(st.global_lsn),
                            // Preserve the rest of the client's known
                            // multi-region progress on the echoed token.
                            &st.region_progress,
                        );
                        if st.version > partition_version
                            || (st.version == partition_version
                                && st.global_lsn > partition.current_lsn())
                        {
                            return Err(error_response(
                                StatusCode::NotFound,
                                Some(1002),
                                "ReadSessionNotAvailable",
                                "The read session is not available for the input session token.",
                                0.0,
                                &request_token,
                                start,
                            )
                            .build());
                        }
                    }
                }
            }
        }

        // Lookup document
        let docs = partition.documents.read().unwrap();
        if let Some(logical) = docs.get(&epk) {
            if let Some(doc) = logical.get(doc_id) {
                let charge = store
                    .config()
                    .ru_model()
                    .compute_read_ru(doc.body_size_bytes);
                let lsn = partition.current_lsn();
                let body = doc.body.clone();
                let etag = doc.etag.clone();
                drop(docs);
                let headers = Some(PointResponseHeaders::from_partition(
                    partition,
                    store.next_transport_request_id(),
                ));
                if parsed.if_none_match.as_deref() == Some(etag.as_str())
                    || parsed.if_none_match.as_deref() == Some("*")
                {
                    let builder = ResponseBuilder::new(StatusCode::NotModified, start)
                        .with_request_charge(charge)
                        .with_session_token(&token)
                        .with_etag(&etag);
                    return Err(decorate_point_response(builder, headers, Some(lsn)).build());
                }
                return Ok((body, etag, token, charge, lsn, headers));
            }
        }

        Err(error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!(
                "Entity with the specified id does not exist in the system. ResourceId: {}",
                doc_id
            ),
            0.0,
            &token,
            start,
        )
        .build())
    });

    match result {
        Some(Ok((body, etag, token, charge, lsn, headers))) => {
            let builder = success_response(StatusCode::Ok, &body, charge, &token, start)
                .with_etag(&etag)
                .with_lsn(lsn);
            decorate_point_response(builder, headers, Some(lsn)).build()
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id, start),
    }
}

async fn handle_replace(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    #[cfg(feature = "preview_dtx")]
    let write_lock = store.document_write_lock();
    #[cfg(feature = "preview_dtx")]
    let _write_guard = write_lock.lock().await;

    handle_replace_locked(store, region_name, parsed, request_body, start).await
}

async fn handle_replace_locked(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");
    let doc_id = parsed.doc_id.as_deref().unwrap_or("");

    if let Some(resp) = replication_back_pressure_response(store, region_name, start) {
        return resp;
    }

    let mut body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    match body.get("id").and_then(|value| value.as_str()) {
        Some(body_id) if body_id == doc_id => {}
        Some(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Document id in request body must match the resource id in the request URI",
                0.0,
                "",
                start,
            )
            .build();
        }
        None => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Missing 'id' field in document",
                0.0,
                "",
                start,
            )
            .build();
        }
    }

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let (_, epk) = match resolve_partition_key(parsed, &body, &state.metadata) {
            Ok(v) => v,
            Err(e) => return Err(bad_partition_key_response(e, start)),
        };

        let partition = match state.find_partition(&epk) {
            Some(p) => p,
            None => {
                return Err(error_response(
                    StatusCode::InternalServerError,
                    None,
                    "InternalError",
                    "No partition found for EPK",
                    1.0,
                    "",
                    start,
                )
                .build());
            }
        };

        // Check partition lock (split/merge in progress)
        if let Some(response) = check_partition_lock(partition, start) {
            return Err(response);
        }

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(partition, region_id, incoming_session_for(parsed, partition.id).as_ref());

        // Cosmos rejects PK mutation on Replace: the partition key value(s)
        // extracted from the new body must match the existing document's
        // stored EPK. Without this check the new body could route to a
        // different physical partition while the original doc would remain
        // orphaned on the old partition (silent divergence in tests).
        let body_components = match super::epk::extract_pk_from_body(
            &body,
            state.metadata.partition_key.paths(),
        ) {
            Ok(v) => v,
            Err(e) => return Err(bad_partition_key_response(e, start)),
        };
        let body_epk = super::epk::compute_epk(
            &body_components,
            state.metadata.partition_key.kind(),
            state.metadata.partition_key.version(),
        );
        if body_epk != epk {
            return Err(error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "PartitionKey extracted from document doesn't match the partition key supplied on the request. \
                 Partition key values are immutable on Replace.",
                1.0,
                &token,
                start,
            )
            .build());
        }

        // Lookup existing under a *read* lock so concurrent reads on the
        // partition are not blocked while we run precondition / throttle
        // checks. We re-acquire a write lock at commit time below.
        let existing_etag = {
            let docs = partition.documents.read().unwrap();
            let existing = docs.get(&epk).and_then(|l| l.get(doc_id));
            match existing {
                Some(e) => {
                    if e.epk != epk {
                        return Err(error_response(
                            StatusCode::BadRequest,
                            None,
                            "BadRequest",
                            "PartitionKey of the existing document does not match the partition key on the request. \
                             Partition key values are immutable on Replace.",
                            1.0,
                            &token,
                            start,
                        )
                        .build());
                    }
                    e.etag.clone()
                }
                None => {
                    return Err(error_response(
                        StatusCode::NotFound,
                        None,
                        "NotFound",
                        &format!(
                            "Entity with the specified id does not exist in the system. ResourceId: {}",
                            doc_id
                        ),
                        0.0,
                        &token,
                        start,
                    )
                    .build());
                }
            }
        };

        // If-Match precondition check
        if let Some(if_match) = &parsed.if_match {
            if *if_match != existing_etag {
                return Err(error_response(
                    StatusCode::PreconditionFailed,
                    None,
                    "PreconditionFailed",
                    "One of the specified pre-condition is not met.",
                    1.0,
                    &token,
                    start,
                )
                .build());
            }
        }

        // Compute RU charge eagerly. Throttle debit is deferred to the
        // post-precondition write-lock window so a 429 only fires when the
        // operation would otherwise have committed. Without this,
        // a throttled-and-then-NotFound replace would still have charged
        // the per-second budget for work that never landed.
        let num_props = RuChargingModel::count_properties(&body);
        let charge = store
            .config()
            .ru_model()
            .compute_replace_or_delete_ru(request_body.len(), num_props);

        // Replace
        let new_doc = {
            let mut docs = partition.documents.write().unwrap();
            let logical = match docs.get_mut(&epk) {
                Some(logical) => logical,
                None => {
                    return Err(error_response(
                        StatusCode::NotFound,
                        None,
                        "NotFound",
                        &format!(
                            "Entity with the specified id does not exist in the system. ResourceId: {}",
                            doc_id
                        ),
                        0.0,
                        &token,
                        start,
                    )
                    .build());
                }
            };
            let current = match logical.get(doc_id).cloned() {
                Some(current) => current,
                None => {
                    return Err(error_response(
                        StatusCode::NotFound,
                        None,
                        "NotFound",
                        &format!(
                            "Entity with the specified id does not exist in the system. ResourceId: {}",
                            doc_id
                        ),
                        0.0,
                        &token,
                        start,
                    )
                    .build());
                }
            };
            if let Some(if_match) = &parsed.if_match {
                if *if_match != current.etag {
                    return Err(error_response(
                        StatusCode::PreconditionFailed,
                        None,
                        "PreconditionFailed",
                        "One of the specified pre-condition is not met.",
                        1.0,
                        &token,
                        start,
                    )
                    .build());
                }
            }

            // Debit the throttle bucket only after preconditions pass under
            // the write lock.
            if let Some(response) = check_throttle(
                partition,
                charge,
                store.config().throttling_enabled(),
                start,
            ) {
                return Err(response);
            }

            let lsn = partition.advance_lsn();
            partition.advance_local_lsn();
            let ts = current_timestamp();
            let etag = new_etag();

            inject_system_properties(&current.rid, &current.self_link, &etag, ts, &mut body);
            // See create handler for rationale — cache wire size.
            let body_size_bytes = request_body.len();
            let new_doc = StoredDocument {
                body: body.clone(),
                id: doc_id.to_string(),
                rid: current.rid,
                etag: etag.clone(),
                ts,
                self_link: current.self_link,
                lsn,
                epk: epk.clone(),
                body_size_bytes,
                source_region: region_name.to_string(),
            };
            logical.insert(doc_id.to_string(), new_doc.clone());
            new_doc
        };

        // Recompute the session token after the write committed so the success
        // response reflects the advanced LSN. The earlier `token` is computed
        // before `advance_lsn` and is only correct for the error paths above
        // (which do not advance the partition), mirroring how handle_create and
        // handle_upsert compute the token post-commit.
        let token = session_token_for(
            partition,
            region_id,
            incoming_session_for(parsed, partition.id).as_ref(),
        );
        let headers = Some(PointResponseHeaders::from_partition(
            partition,
            store.next_transport_request_id(),
        ));

        Ok((new_doc, token, charge, body, headers))
    });

    match result {
        Some(Ok((doc, token, charge, response_body, headers))) => {
            store.replicate(region_name, db_id, coll_id, &doc, false);

            let builder = if parsed.content_response_on_write {
                success_response(StatusCode::Ok, &response_body, charge, &token, start)
                    .with_etag(&doc.etag)
                    .with_lsn(doc.lsn)
            } else {
                ResponseBuilder::new(StatusCode::Ok, start)
                    .with_request_charge(charge)
                    .with_session_token(&token)
                    .with_etag(&doc.etag)
                    .with_lsn(doc.lsn)
            };

            decorate_point_response(builder, headers, Some(doc.lsn)).build()
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id, start),
    }
}

async fn handle_upsert(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    #[cfg(feature = "preview_dtx")]
    let write_lock = store.document_write_lock();
    #[cfg(feature = "preview_dtx")]
    let _write_guard = write_lock.lock().await;

    handle_upsert_locked(store, region_name, parsed, request_body, start).await
}

async fn handle_upsert_locked(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
    start: Instant,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");

    if let Some(resp) = replication_back_pressure_response(store, region_name, start) {
        return resp;
    }

    let mut body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    let doc_id = match body.get("id").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Missing 'id' field in document",
                0.0,
                "",
                start,
            )
            .build();
        }
    };

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let (_, epk) = match resolve_partition_key(parsed, &body, &state.metadata) {
            Ok(v) => v,
            Err(e) => return Err(bad_partition_key_response(e, start)),
        };

        let partition = match state.find_partition(&epk) {
            Some(p) => p,
            None => {
                return Err(error_response(
                    StatusCode::InternalServerError,
                    None,
                    "InternalError",
                    "No partition found for EPK",
                    1.0,
                    "",
                    start,
                )
                .build());
            }
        };

        // Check partition lock
        if let Some(response) = check_partition_lock(partition, start) {
            return Err(response);
        }

        // The create-vs-replace decision, RU charge, throttle debit, and
        // commit must all happen under the write lock for correctness:
        // a previous version probed existence under a read lock, then
        // re-acquired a write lock and inserted unconditionally, which let a
        // concurrent create slip in between probe and commit. The upsert
        // would then return 201 Created while overwriting an existing
        // document, charge create-RU for what was semantically a replace,
        // and allocate a fresh `_rid` for a document the prior writer's
        // client believed already had a stable RID.
        //
        // RID allocation is deferred to the write lock so we don't burn a
        // monotonic counter slot on a path that turns out to be a replace.
        let num_props = RuChargingModel::count_properties(&body);
        let (new_doc, status, charge) = {
            let mut docs = partition.documents.write().unwrap();
            let logical = docs.entry(epk.clone()).or_default();
            let (status, rid, self_link) = match logical.get(&doc_id) {
                Some(existing) => (
                    StatusCode::Ok,
                    existing.rid.clone(),
                    existing.self_link.clone(),
                ),
                None => {
                    let (_, doc_rid) = store.rid_generator().next_document_rid(
                        state.metadata.numeric_db_id,
                        state.metadata.numeric_coll_id,
                    );
                    let self_link = format!("{}docs/{}/", state.metadata.self_link, doc_rid);
                    (StatusCode::Created, doc_rid, self_link)
                }
            };

            let charge = if status == StatusCode::Created {
                store
                    .config()
                    .ru_model()
                    .compute_create_ru(request_body.len(), num_props)
            } else {
                store
                    .config()
                    .ru_model()
                    .compute_replace_or_delete_ru(request_body.len(), num_props)
            };

            // Throttle debit only after the create-vs-replace decision is
            // locked in, so the reported RU charge matches the
            // bucket debit even when the operation is rejected with 429.
            if let Some(response) = check_throttle(
                partition,
                charge,
                store.config().throttling_enabled(),
                start,
            ) {
                return Err(response);
            }

            let lsn = partition.advance_lsn();
            partition.advance_local_lsn();
            let ts = current_timestamp();
            let etag = new_etag();

            inject_system_properties(&rid, &self_link, &etag, ts, &mut body);
            // See create handler for rationale — cache wire size.
            let body_size_bytes = request_body.len();
            let new_doc = StoredDocument {
                body: body.clone(),
                id: doc_id.clone(),
                rid,
                etag: etag.clone(),
                ts,
                self_link,
                lsn,
                epk: epk.clone(),
                body_size_bytes,
                source_region: region_name.to_string(),
            };
            logical.insert(doc_id.clone(), new_doc.clone());
            (new_doc, status, charge)
        };

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(
            partition,
            region_id,
            incoming_session_for(parsed, partition.id).as_ref(),
        );
        let headers = Some(PointResponseHeaders::from_partition(
            partition,
            store.next_transport_request_id(),
        ));
        Ok((new_doc, status, token, charge, body, headers))
    });

    match result {
        Some(Ok((doc, status, token, charge, response_body, headers))) => {
            store.replicate(region_name, db_id, coll_id, &doc, false);

            let builder = if parsed.content_response_on_write {
                success_response(status, &response_body, charge, &token, start)
                    .with_etag(&doc.etag)
                    .with_lsn(doc.lsn)
            } else {
                ResponseBuilder::new(status, start)
                    .with_request_charge(charge)
                    .with_session_token(&token)
                    .with_etag(&doc.etag)
                    .with_lsn(doc.lsn)
            };

            decorate_point_response(builder, headers, Some(doc.lsn)).build()
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id, start),
    }
}

async fn handle_delete(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> AsyncRawResponse {
    #[cfg(feature = "preview_dtx")]
    let write_lock = store.document_write_lock();
    #[cfg(feature = "preview_dtx")]
    let _write_guard = write_lock.lock().await;

    handle_delete_locked(store, region_name, parsed, start).await
}

async fn handle_delete_locked(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    start: Instant,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");
    let doc_id = parsed.doc_id.as_deref().unwrap_or("");

    if let Some(resp) = replication_back_pressure_response(store, region_name, start) {
        return resp;
    }

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(start),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let empty_body = serde_json::Value::Null;
        let (_, epk) = match resolve_partition_key(parsed, &empty_body, &state.metadata) {
            Ok(v) => v,
            Err(e) => return Err(bad_partition_key_response(e, start)),
        };

        let partition = match state.find_partition(&epk) {
            Some(p) => p,
            None => {
                return Err(error_response(
                    StatusCode::InternalServerError,
                    None,
                    "InternalError",
                    "No partition found for EPK",
                    1.0,
                    "",
                    start,
                )
                .build());
            }
        };

        // Check partition lock (split/merge in progress)
        if let Some(response) = check_partition_lock(partition, start) {
            return Err(response);
        }

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(partition, region_id, incoming_session_for(parsed, partition.id).as_ref());

        // Look up the existing doc under a *read* lock; only escalate to
        // a write lock at commit time so throttled / precondition-failed
        // requests do not serialize other writers/readers.
        let existing = {
            let docs = partition.documents.read().unwrap();
            match docs.get(&epk).and_then(|l| l.get(doc_id)).cloned() {
                Some(e) => e,
                None => {
                    return Err(error_response(
                        StatusCode::NotFound,
                        None,
                        "NotFound",
                        &format!(
                            "Entity with the specified id does not exist in the system. ResourceId: {}",
                            doc_id
                        ),
                        0.0,
                        &token,
                        start,
                    )
                    .build());
                }
            }
        };

        // If-Match precondition
        if let Some(if_match) = &parsed.if_match {
            if *if_match != existing.etag {
                return Err(error_response(
                    StatusCode::PreconditionFailed,
                    None,
                    "PreconditionFailed",
                    "One of the specified pre-condition is not met.",
                    1.0,
                    &token,
                    start,
                )
                .build());
            }
        }

        // Compute RU charge eagerly. Throttle debit is deferred to the
        // post-precondition write-lock window so a 429 only fires when the
        // operation would otherwise have committed.
        let num_props = RuChargingModel::count_properties(&existing.body);
        let body_size = existing.body_size_bytes;
        let charge = store
            .config()
            .ru_model()
            .compute_replace_or_delete_ru(body_size, num_props);

        let tombstone = {
            let mut docs = partition.documents.write().unwrap();
            let logical = match docs.get_mut(&epk) {
                Some(logical) => logical,
                None => {
                    return Err(error_response(
                        StatusCode::NotFound,
                        None,
                        "NotFound",
                        &format!(
                            "Entity with the specified id does not exist in the system. ResourceId: {}",
                            doc_id
                        ),
                        0.0,
                        &token,
                        start,
                    )
                    .build());
                }
            };
            let current = match logical.get(doc_id).cloned() {
                Some(current) => current,
                None => {
                    return Err(error_response(
                        StatusCode::NotFound,
                        None,
                        "NotFound",
                        &format!(
                            "Entity with the specified id does not exist in the system. ResourceId: {}",
                            doc_id
                        ),
                        0.0,
                        &token,
                        start,
                    )
                    .build());
                }
            };
            if let Some(if_match) = &parsed.if_match {
                if *if_match != current.etag {
                    return Err(error_response(
                        StatusCode::PreconditionFailed,
                        None,
                        "PreconditionFailed",
                        "One of the specified pre-condition is not met.",
                        1.0,
                        &token,
                        start,
                    )
                    .build());
                }
            }

            // Debit the throttle bucket only after preconditions pass under
            // the write lock.
            if let Some(response) = check_throttle(
                partition,
                charge,
                store.config().throttling_enabled(),
                start,
            ) {
                return Err(response);
            }

            let lsn = partition.advance_lsn();
            partition.advance_local_lsn();
            logical.remove(doc_id);

            StoredDocument {
                body: serde_json::Value::Null,
                id: doc_id.to_string(),
                rid: current.rid,
                etag: current.etag,
                ts: current_timestamp(),
                self_link: current.self_link,
                lsn,
                epk: current.epk,
                body_size_bytes: 0,
                source_region: region_name.to_string(),
            }
        };

        // Recompute the session token after the delete committed so the success
        // response reflects the advanced LSN. The earlier `token` is computed
        // before `advance_lsn` and is only correct for the error paths above
        // (which do not advance the partition), mirroring how handle_create and
        // handle_upsert compute the token post-commit.
        let token = session_token_for(
            partition,
            region_id,
            incoming_session_for(parsed, partition.id).as_ref(),
        );
        let headers = Some(PointResponseHeaders::from_partition(
            partition,
            store.next_transport_request_id(),
        ));

        Ok((tombstone, token, charge, headers))
    });

    match result {
        Some(Ok((tombstone, token, charge, headers))) => {
            store.replicate(region_name, db_id, coll_id, &tombstone, true);

            let builder = ResponseBuilder::new(StatusCode::NoContent, start)
                .with_request_charge(charge)
                .with_session_token(&token)
                .with_lsn(tombstone.lsn);
            decorate_point_response(builder, headers, None).build()
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id, start),
    }
}

// --- Helper Responses ---

fn write_forbidden_response(start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::Forbidden,
        Some(3),
        "Forbidden",
        "Write operations are not allowed on this region.",
        0.0,
        "",
        start,
    )
    .build()
}

fn bad_request_path_response(path: &str, start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::BadRequest,
        None,
        "BadRequest",
        &format!("Invalid request path: {}", path),
        0.0,
        "",
        start,
    )
    .build()
}

fn invalid_input_response(message: &str, start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::BadRequest,
        None,
        "BadRequest",
        &format!("One of the input values is invalid. {message}"),
        0.0,
        "",
        start,
    )
    .build()
}

fn unsupported_response(operation: &str, start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::NotImplemented,
        None,
        "NotImplemented",
        &format!(
            "Operation '{}' is not supported by the in-memory emulator.",
            operation
        ),
        0.0,
        "",
        start,
    )
    .build()
}

fn not_found_region(start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::NotFound,
        None,
        "NotFound",
        "Region not found",
        0.0,
        "",
        start,
    )
    .build()
}

fn container_not_found(db_id: &str, coll_id: &str, start: Instant) -> AsyncRawResponse {
    error_response(
        StatusCode::NotFound,
        None,
        "NotFound",
        &format!("Container '{}/{}' does not exist", db_id, coll_id),
        0.0,
        "",
        start,
    )
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document_item(epk: &str, id: &str) -> DocumentFeedItem {
        DocumentFeedItem {
            body: serde_json::json!({ "id": id }),
            cursor: DocumentFeedCursor {
                epk: Epk::from(epk),
                id: id.to_owned(),
            },
        }
    }

    fn ids(values: &[serde_json::Value]) -> Vec<&str> {
        values
            .iter()
            .map(|value| value["id"].as_str().expect("test document has id"))
            .collect()
    }

    #[test]
    fn document_feed_cursor_skips_already_returned_low_child_prefix_after_split() {
        let start = Instant::now();
        let parent = vec![
            document_item("01", "hash-a-0"),
            document_item("02", "hash-a-1"),
            document_item("80", "hash-e-0"),
        ];
        let (_page, continuation) =
            paginate_document_feed_items(parent, Some(1), None, start).unwrap();

        let low_child = vec![
            document_item("01", "hash-a-0"),
            document_item("02", "hash-a-1"),
        ];
        let (page, next) =
            paginate_document_feed_items(low_child, Some(10), continuation.as_deref(), start)
                .unwrap();

        assert_eq!(ids(&page), vec!["hash-a-1"]);
        assert!(next.is_none());
    }

    #[test]
    fn document_feed_cursor_does_not_skip_high_child_after_split() {
        let start = Instant::now();
        let parent = vec![
            document_item("01", "hash-a-0"),
            document_item("02", "hash-a-1"),
            document_item("80", "hash-e-0"),
        ];
        let (_page, continuation) =
            paginate_document_feed_items(parent, Some(1), None, start).unwrap();

        let high_child = vec![document_item("80", "hash-e-0")];
        let (page, next) =
            paginate_document_feed_items(high_child, Some(10), continuation.as_deref(), start)
                .unwrap();

        assert_eq!(ids(&page), vec!["hash-e-0"]);
        assert!(next.is_none());
    }

    #[test]
    fn document_feed_cursor_rejects_malformed_epk_hex() {
        let token = serde_json::to_string(&DocumentFeedCursorToken {
            kind: DOCUMENT_FEED_CURSOR_TOKEN_KIND.to_owned(),
            epk: "00zz".to_owned(),
            id: "item1".to_owned(),
        })
        .unwrap();

        let err = DocumentFeedCursor::parse(&token, Instant::now()).unwrap_err();
        assert_eq!(err.status(), StatusCode::BadRequest);
    }

    #[test]
    fn document_feed_cursor_pagination_requires_cursor_sorted_items() {
        let start = Instant::now();
        let mut items = vec![
            document_item("80", "hash-e-0"),
            document_item("01", "hash-a-0"),
            document_item("02", "hash-a-1"),
        ];
        items.sort_by(|left, right| left.cursor.cmp(&right.cursor));

        let (page, continuation) =
            paginate_document_feed_items(items, Some(1), None, start).unwrap();

        assert_eq!(ids(&page), vec!["hash-a-0"]);
        assert!(continuation.is_some());
    }
}
