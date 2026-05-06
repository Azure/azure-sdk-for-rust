// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point operation and control-plane operation handlers.

// cspell:ignore acked llsn

use std::sync::Arc;
use std::time::Instant;

use azure_core::http::headers::HeaderValue;
use azure_core::http::{AsyncRawResponse, StatusCode};

use super::config::ContainerConfig;
use super::dispatch::{OperationType, ParsedRequest};
use super::epk::{compute_epk, extract_pk_from_body, parse_partition_key_header, Epk};
use super::response::headers::{
    ACTIVITY_ID, GATEWAY_VERSION, GLOBAL_COMMITTED_LSN, INTERNAL_PARTITION_ID, ITEM_LOCAL_LSN,
    ITEM_LSN, LAST_STATE_CHANGE_UTC, LOCAL_LSN, NUMBER_OF_READ_REGIONS, PARTITION_KEY_RANGE_ID,
    QUORUM_ACKED_LOCAL_LSN, QUORUM_ACKED_LSN, RESOURCE_QUOTA, RESOURCE_USAGE, SERVICE_VERSION,
    TRANSPORT_REQUEST_ID,
};
use super::response::{error_response, success_response, ResponseBuilder};
use super::ru_model::RuChargingModel;
use super::session::SessionToken;
use super::store::{
    current_timestamp, new_etag, ContainerMetadata, EmulatorStore, PhysicalPartition,
    StoredDocument,
};
use super::system_properties::{
    account_properties_to_json, container_to_json, database_to_json, inject_system_properties,
    pkranges_to_json,
};
use crate::models::PartitionKeyDefinition;

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

async fn with_request_activity_id(
    response: AsyncRawResponse,
    activity_id: Option<&str>,
) -> AsyncRawResponse {
    let Some(activity_id) = activity_id else {
        return response;
    };

    let raw = response
        .try_into_raw_response()
        .await
        .expect("emulator responses are always buffered; streaming responses are not produced by this emulator");
    let mut headers = raw.headers().clone();
    headers.insert(
        ACTIVITY_ID.clone(),
        HeaderValue::from(activity_id.to_string()),
    );
    AsyncRawResponse::from_bytes(raw.status(), headers, raw.body().as_ref().to_vec())
}

/// Dispatches a parsed request to the appropriate handler.
pub(crate) async fn handle_operation(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
) -> AsyncRawResponse {
    let start = Instant::now();
    let response = match &parsed.operation {
        OperationType::ReadAccount => handle_read_account(store, start),
        OperationType::CreateDatabase => {
            handle_create_database(store, region_name, parsed, request_body, start)
        }
        OperationType::ReadDatabase => handle_read_database(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            start,
        ),
        OperationType::DeleteDatabase => handle_delete_database(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            start,
        ),
        OperationType::CreateContainer => handle_create_container(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed,
            request_body,
            start,
        ),
        OperationType::ReadContainer => handle_read_container(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed.coll_id.as_deref().unwrap_or(""),
            start,
        ),
        OperationType::DeleteContainer => handle_delete_container(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed.coll_id.as_deref().unwrap_or(""),
            start,
        ),
        OperationType::ReadPKRanges => handle_read_pkranges(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed.coll_id.as_deref().unwrap_or(""),
            start,
        ),
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
        OperationType::Query => unsupported_response(
            "SQL queries are not supported by the in-memory emulator. \
             See sdk/cosmos/azure_data_cosmos/docs/in-memory-emulator-spec.md \
             section 1 (Non-Goals).",
            start,
        ),
        OperationType::BadRequestPath(desc) => bad_request_path_response(desc, start),
        OperationType::Unsupported(desc) => unsupported_response(desc, start),
    };

    with_request_activity_id(response, parsed.activity_id.as_deref()).await
}

// --- Control-Plane Operations ---

fn handle_read_account(store: &Arc<EmulatorStore>, start: Instant) -> AsyncRawResponse {
    let body = account_properties_to_json(store.config());
    success_response(StatusCode::Ok, &body, 0.0, "", start)
        .with_item_count(1)
        .build()
}

fn handle_create_database(
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
    let token = store.advance_master_partition_lsn();
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

    let token = store.advance_master_partition_lsn();
    ResponseBuilder::new(StatusCode::NoContent, start)
        .with_request_charge(1.0)
        .with_session_token(&token)
        .build()
}

fn handle_create_container(
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

    let meta = store.create_container_with_config_internal(
        db_id,
        &coll_id,
        pk_def,
        ContainerConfig::default(),
    );
    let response_body = container_to_json(&meta);
    let token = store.advance_master_partition_lsn();
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

    let token = store.advance_master_partition_lsn();
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
            let body = pkranges_to_json(state);
            success_response(StatusCode::Ok, &body, 1.0, "", start)
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
) -> azure_core::Result<(Vec<super::epk::PartitionKeyComponent>, Epk)> {
    let pk_components = if let Some(pk_header) = &parsed.partition_key_header {
        parse_partition_key_header(pk_header)?
    } else if body.is_null() {
        // Read / Delete callers pass a `Null` body — there is nothing to
        // extract a partition key from. Real Cosmos rejects point operations
        // that omit the partition key header in this case with 400 BadRequest;
        // mirror that so dual-backend tests stay consistent.
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "missing 'x-ms-documentdb-partitionkey' header on point operation",
        ));
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
fn bad_partition_key_response(err: azure_core::Error, start: Instant) -> AsyncRawResponse {
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
    /// Builds the response-header snapshot from a partition reference while the
    /// caller still holds the relevant lock. Capturing here (rather than in a
    /// later, lock-free pass) is what guarantees the header LSN/document-count
    /// values agree with the response body the same handler is about to
    /// produce — a concurrent writer on the same partition cannot interleave
    /// between the body capture and the header capture.
    fn from_partition(partition: &PhysicalPartition, transport_request_id: u32) -> Self {
        let documents = partition.documents.read().unwrap();
        let documents_in_partition = documents
            .values()
            .map(std::collections::BTreeMap::len)
            .sum::<usize>();
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
        .with_header_value(GATEWAY_VERSION.clone(), "2.0.0")
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
                Some(1007),
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
            // passed under the write lock (#2 / #3): on a 429 the response
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
            // same `compute_..._ru(size)` formula on identical inputs (#6).
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
            .check_and_clear_forced_for(epk.as_str())
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
                // skipped the consistency check (#9).
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

        // Lookup existing under a *read* lock so concurrent reads on the
        // partition are not blocked while we run precondition / throttle
        // checks. We re-acquire a write lock at commit time below.
        let existing_etag = {
            let docs = partition.documents.read().unwrap();
            let existing = docs.get(&epk).and_then(|l| l.get(doc_id));
            match existing {
                Some(e) => e.etag.clone(),
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
        // operation would otherwise have committed (#2 / #3). Without this,
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
            // the write lock (#2 / #3).
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
            // See create handler for rationale (#6) — cache wire size.
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
        // commit must all happen under the write lock for correctness (#3):
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
            // locked in (#2 / #3), so the reported RU charge matches the
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
            // See create handler for rationale (#6) — cache wire size.
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
        // operation would otherwise have committed (#2 / #3).
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
            // the write lock (#2 / #3).
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
