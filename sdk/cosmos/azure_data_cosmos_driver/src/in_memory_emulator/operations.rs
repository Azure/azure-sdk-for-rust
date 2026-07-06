// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point operation and control-plane operation handlers.

// cspell:ignore acked llsn

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_core::http::{AsyncRawResponse, StatusCode};
use serde::Deserialize;

use super::config::ContainerConfig;
use super::dispatch::{OperationType, ParsedRequest};
use super::epk::{compute_epk, extract_pk_from_body, parse_partition_key_header, Epk};
use super::response::headers::{
    ACTIVITY_ID, CONTINUATION, GLOBAL_COMMITTED_LSN, INTERNAL_PARTITION_ID, ITEM_LOCAL_LSN,
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
    account_properties_to_json, container_to_json, database_to_json, feed_to_json,
    inject_system_properties, offer_to_json, pkranges_to_json,
};
use crate::models::PartitionKeyDefinition;

static OFFER_REPLACE_PENDING: HeaderName = HeaderName::from_static("x-ms-offer-replace-pending");

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
        OperationType::BadRequestPath(desc) => bad_request_path_response(desc, start),
        OperationType::Unsupported(desc) => unsupported_response(desc, start),
    };

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

fn success_feed_response(
    envelope_name: &str,
    rid: impl Into<String>,
    items: Vec<serde_json::Value>,
    page_options: FeedPageOptions<'_>,
    charge: f64,
    session_token: &str,
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
    let mut builder = success_response(StatusCode::Ok, &body, charge, session_token, start)
        .with_item_count(item_count);
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
    session_token: &str,
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
        1.0,
        session_token,
        start,
    )
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
        1.0,
        "",
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
    execute_query_feed("Databases", "", databases, parsed, request_body, "", start)
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
        1.0,
        "",
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
        "",
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
        1.0,
        "",
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
    execute_query_feed("Offers", "", offers, parsed, request_body, "", start)
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
) -> Result<(String, Vec<serde_json::Value>, String), AsyncRawResponse> {
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
        let mut docs = Vec::new();
        let mut token_parts = Vec::new();
        for partition in &state.physical_partitions {
            if parsed
                .partition_key_range_id
                .as_deref()
                .is_some_and(|id| id != partition.id.to_string())
            {
                continue;
            }
            if let Some(response) = check_partition_lock(partition, start) {
                return Err(response);
            }
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
                docs.extend(logical.values().map(|doc| doc.body.clone()));
            }
        }
        Ok((state.metadata.rid.clone(), docs, token_parts.join(",")))
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
        Ok((rid, docs, token)) => success_feed_response(
            "Documents",
            rid,
            docs,
            FeedPageOptions::from_request(parsed),
            1.0,
            &token,
            start,
        ),
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
        Ok((rid, docs, token)) => {
            execute_query_feed("Documents", rid, docs, parsed, request_body, &token, start)
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
        rewritten_query: None,
        has_select_value: info.has_select_value,
        has_non_streaming_order_by: false,
    }
}

fn full_query_range() -> crate::driver::dataflow::query_plan::QueryRange {
    crate::driver::dataflow::query_plan::QueryRange {
        min: Epk::MIN.as_str().to_string(),
        max: Epk::MAX.as_str().to_string(),
        is_min_inclusive: true,
        is_max_inclusive: false,
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

    let plan = crate::driver::dataflow::query_plan::QueryPlan {
        partitioned_query_execution_info_version: 1,
        query_info: Some(local_query_info_to_dataflow(local_plan.query_info)),
        query_ranges: vec![full_query_range()],
        hybrid_search_query_info: None,
    };
    let body = match serde_json::to_value(plan) {
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
