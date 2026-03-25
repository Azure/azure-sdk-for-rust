// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point operation and control-plane operation handlers.

use std::sync::Arc;

use azure_core::http::{AsyncRawResponse, StatusCode};

use super::config::ContainerConfig;
use super::dispatch::{OperationType, ParsedRequest};
use super::epk::{compute_epk, extract_pk_from_body, parse_partition_key_header, Epk};
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

/// Dispatches a parsed request to the appropriate handler.
pub(crate) async fn handle_operation(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
) -> AsyncRawResponse {
    match &parsed.operation {
        OperationType::ReadAccount => handle_read_account(store),
        OperationType::CreateDatabase => handle_create_database(store, region_name, request_body),
        OperationType::ReadDatabase => {
            handle_read_database(store, region_name, parsed.db_id.as_deref().unwrap_or(""))
        }
        OperationType::DeleteDatabase => {
            handle_delete_database(store, region_name, parsed.db_id.as_deref().unwrap_or(""))
        }
        OperationType::CreateContainer => handle_create_container(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            request_body,
        ),
        OperationType::ReadContainer => handle_read_container(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed.coll_id.as_deref().unwrap_or(""),
        ),
        OperationType::DeleteContainer => handle_delete_container(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed.coll_id.as_deref().unwrap_or(""),
        ),
        OperationType::ReadPKRanges => handle_read_pkranges(
            store,
            region_name,
            parsed.db_id.as_deref().unwrap_or(""),
            parsed.coll_id.as_deref().unwrap_or(""),
        ),
        OperationType::Create => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response();
            }
            handle_create(store, region_name, parsed, request_body).await
        }
        OperationType::Read => handle_read(store, region_name, parsed),
        OperationType::Replace => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response();
            }
            handle_replace(store, region_name, parsed, request_body).await
        }
        OperationType::Upsert => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response();
            }
            handle_upsert(store, region_name, parsed, request_body).await
        }
        OperationType::Delete => {
            if !store.config().is_write_region(region_name) {
                return write_forbidden_response();
            }
            handle_delete(store, region_name, parsed).await
        }
        OperationType::Query => unsupported_response("Query"),
        OperationType::Unsupported(desc) => unsupported_response(desc),
    }
}

// --- Control-Plane Operations ---

fn handle_read_account(store: &Arc<EmulatorStore>) -> AsyncRawResponse {
    let body = account_properties_to_json(store.config());
    success_response(StatusCode::Ok, &body, 0.0, "")
        .with_item_count(1)
        .build()
}

fn handle_create_database(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    request_body: &[u8],
) -> AsyncRawResponse {
    let body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                1.0,
                "",
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
                1.0,
                "",
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
            )
            .build();
        }
    }

    let meta = store.create_database_internal(&db_id);
    let response_body = database_to_json(&meta);
    success_response(StatusCode::Created, &response_body, 1.0, "")
        .with_etag(&meta.etag)
        .build()
}

fn handle_read_database(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    match region_ref.get_database(db_id) {
        Some(meta) => {
            let body = database_to_json(&meta);
            success_response(StatusCode::Ok, &body, 1.0, "")
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
            1.0,
            "",
        )
        .build(),
    }
}

fn handle_delete_database(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
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
            1.0,
            "",
        )
        .build();
    }

    // Delete from all regions
    for vr in store.config().regions() {
        if let Some(r) = store.region(vr.name()) {
            r.delete_database(db_id);
        }
    }

    ResponseBuilder::new(StatusCode::NoContent)
        .with_request_charge(1.0)
        .with_session_token("")
        .build()
}

fn handle_create_container(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    request_body: &[u8],
) -> AsyncRawResponse {
    // Verify database exists
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    if !region_ref.database_exists(db_id) {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            1.0,
            "",
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
                1.0,
                "",
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
                1.0,
                "",
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
                    1.0,
                    "",
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
                1.0,
                "",
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
    success_response(StatusCode::Created, &response_body, 1.0, "")
        .with_etag(&meta.etag)
        .build()
}

fn handle_read_container(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    coll_id: &str,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    if !region_ref.database_exists(db_id) {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            1.0,
            "",
        )
        .build();
    }

    match region_ref.get_container(db_id, coll_id) {
        Some(snapshot) => {
            let body = container_to_json(&snapshot.metadata);
            success_response(StatusCode::Ok, &body, 1.0, "")
                .with_etag(&snapshot.metadata.etag)
                .build()
        }
        None => error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Container '{}' does not exist", coll_id),
            1.0,
            "",
        )
        .build(),
    }
}

fn handle_delete_container(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    coll_id: &str,
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
            1.0,
            "",
        )
        .build();
    }

    // Delete from all regions
    for vr in store.config().regions() {
        if let Some(r) = store.region(vr.name()) {
            r.delete_container(db_id, coll_id);
        }
    }

    ResponseBuilder::new(StatusCode::NoContent)
        .with_request_charge(1.0)
        .with_session_token("")
        .build()
}

fn handle_read_pkranges(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    db_id: &str,
    coll_id: &str,
) -> AsyncRawResponse {
    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    if !region_ref.database_exists(db_id) {
        return error_response(
            StatusCode::NotFound,
            None,
            "NotFound",
            &format!("Database '{}' does not exist", db_id),
            1.0,
            "",
        )
        .build();
    }

    region_ref
        .with_container(db_id, coll_id, |state| {
            let body = pkranges_to_json(state);
            success_response(StatusCode::Ok, &body, 1.0, "")
                .with_item_count(state.physical_partitions.len() as u32)
                .build()
        })
        .unwrap_or_else(|| {
            error_response(
                StatusCode::NotFound,
                None,
                "NotFound",
                &format!("Container '{}' does not exist", coll_id),
                1.0,
                "",
            )
            .build()
        })
}

// --- Point Operations ---

/// Resolves the partition key components and EPK for a point operation.
fn resolve_partition_key(
    parsed: &ParsedRequest,
    body: &serde_json::Value,
    meta: &ContainerMetadata,
) -> (Vec<super::epk::PartitionKeyComponent>, Epk) {
    let pk_components = if let Some(pk_header) = &parsed.partition_key_header {
        parse_partition_key_header(pk_header)
    } else {
        extract_pk_from_body(body, meta.partition_key.paths())
    };

    let epk = compute_epk(
        &pk_components,
        meta.partition_key.kind(),
        meta.partition_key.version(),
    );

    (pk_components, epk)
}

/// Builds a V2 session token for a partition in the given region.
fn session_token_for(partition: &PhysicalPartition, region_id: u64) -> String {
    SessionToken::format_v2(
        partition.id,
        partition.current_version(),
        partition.current_lsn(),
        region_id,
    )
}

/// Returns a 410/1007 response if the partition is locked (split/merge in progress).
fn check_partition_lock(partition: &PhysicalPartition) -> Option<AsyncRawResponse> {
    if partition.is_locked() {
        Some(
            error_response(
                StatusCode::Gone,
                Some(1007),
                "Gone",
                "Partition is being split or merged.",
                0.0,
                "",
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
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");

    let mut body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                1.0,
                "",
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
                1.0,
                "",
            )
            .build();
        }
    };

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let (_, epk) = resolve_partition_key(parsed, &body, &state.metadata);

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
                )
                .build());
            }
        };

        // Check partition lock (split/merge in progress)
        if let Some(response) = check_partition_lock(partition) {
            return Err(response);
        }

        // Check for conflict
        {
            let docs = partition.documents.read().unwrap();
            if let Some(logical) = docs.get(&epk) {
                if logical.contains_key(&doc_id) {
                    let region_id = store.config().region_id_for(region_name);
                    let token = session_token_for(partition, region_id);
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
                    )
                    .build());
                }
            }
        }

        // Generate system properties
        let lsn = partition.advance_lsn();
        let (_, doc_rid) = store.rid_generator().next_document_rid(
            state.metadata.numeric_db_id,
            state.metadata.numeric_coll_id,
        );
        let ts = current_timestamp();
        let etag = new_etag();
        let self_link = format!("{}docs/{}/", state.metadata.self_link, doc_rid);

        let stored_doc = StoredDocument {
            body: body.clone(),
            id: doc_id.clone(),
            rid: doc_rid,
            etag: etag.clone(),
            ts,
            self_link,
            lsn,
            epk: epk.clone(),
        };

        // Inject system properties into the body
        inject_system_properties(&stored_doc, &mut body);
        let stored_doc = StoredDocument {
            body: body.clone(),
            ..stored_doc
        };

        // Store the document
        {
            let mut docs = partition.documents.write().unwrap();
            let logical = docs.entry(epk).or_default();
            logical.insert(doc_id, stored_doc.clone());
        }

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(partition, region_id);
        let num_props = RuChargingModel::count_properties(&body);
        let charge = store
            .config()
            .ru_model()
            .compute_create_ru(request_body.len(), num_props);

        // Check throttle
        if let Some(response) = check_throttle(partition, charge, store.config().throttling_enabled()) {
            return Err(response);
        }

        Ok((stored_doc, token, charge, body))
    });

    match result {
        Some(Ok((doc, token, charge, response_body))) => {
            // Trigger replication
            store.replicate(region_name, db_id, coll_id, &doc, false);

            if parsed.content_response_on_write {
                success_response(StatusCode::Created, &response_body, charge, &token)
                    .with_etag(&doc.etag)
                    .with_item_count(1)
                    .build()
            } else {
                ResponseBuilder::new(StatusCode::Created)
                    .with_request_charge(charge)
                    .with_session_token(&token)
                    .with_etag(&doc.etag)
                    .build()
            }
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id),
    }
}

fn handle_read(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");
    let doc_id = parsed.doc_id.as_deref().unwrap_or("");

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let empty_body = serde_json::Value::Null;
        let (_, epk) = resolve_partition_key(parsed, &empty_body, &state.metadata);

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
                )
                .build());
            }
        };

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(partition, region_id);

        // Check partition lock
        if let Some(response) = check_partition_lock(partition) {
            return Err(response);
        }

        // Check forced session unavailability (one-shot)
        if partition.session_state.check_and_clear_forced() {
            return Err(error_response(
                StatusCode::NotFound,
                Some(1002),
                "ReadSessionNotAvailable",
                "The read session is not available for the input session token.",
                1.0,
                &token,
            )
            .build());
        }

        // Session consistency check (V2-aware: compare version first, then globalLSN)
        if store.config().consistency().is_session() {
            if let Some(session_header) = &parsed.session_token {
                let tokens = super::session::parse_composite_session_token(session_header);
                for st in &tokens {
                    if st.pkrange_id == partition.id {
                        let partition_version = partition.current_version();
                        if st.version > partition_version {
                            return Err(error_response(
                                StatusCode::NotFound,
                                Some(1002),
                                "ReadSessionNotAvailable",
                                "The read session is not available for the input session token.",
                                1.0,
                                &token,
                            )
                            .build());
                        }
                        if st.version == partition_version
                            && st.global_lsn > partition.current_lsn()
                        {
                            return Err(error_response(
                                StatusCode::NotFound,
                                Some(1002),
                                "ReadSessionNotAvailable",
                                "The read session is not available for the input session token.",
                                1.0,
                                &token,
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
                    .compute_read_ru(serde_json::to_vec(&doc.body).unwrap_or_default().len());
                return Ok((doc.body.clone(), doc.etag.clone(), token, charge));
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
            1.0,
            &token,
        )
        .build())
    });

    match result {
        Some(Ok((body, etag, token, charge))) => {
            success_response(StatusCode::Ok, &body, charge, &token)
                .with_etag(&etag)
                .with_item_count(1)
                .build()
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id),
    }
}

async fn handle_replace(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");
    let doc_id = parsed.doc_id.as_deref().unwrap_or("");

    let mut body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                1.0,
                "",
            )
            .build();
        }
    };

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let (_, epk) = resolve_partition_key(parsed, &body, &state.metadata);

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
                )
                .build());
            }
        };

        // Check partition lock (split/merge in progress)
        if let Some(response) = check_partition_lock(partition) {
            return Err(response);
        }

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(partition, region_id);

        // Lookup existing
        let mut docs = partition.documents.write().unwrap();
        let logical = match docs.get_mut(&epk) {
            Some(l) => l,
            None => {
                return Err(error_response(
                    StatusCode::NotFound,
                    None,
                    "NotFound",
                    &format!(
                        "Entity with the specified id does not exist in the system. ResourceId: {}",
                        doc_id
                    ),
                    1.0,
                    &token,
                )
                .build());
            }
        };

        let existing = match logical.get(doc_id) {
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
                    1.0,
                    &token,
                )
                .build());
            }
        };

        // If-Match precondition check
        if let Some(if_match) = &parsed.if_match {
            if *if_match != existing.etag {
                return Err(error_response(
                    StatusCode::PreconditionFailed,
                    None,
                    "PreconditionFailed",
                    "One of the specified pre-condition is not met.",
                    1.0,
                    &token,
                )
                .build());
            }
        }

        // Replace
        let lsn = partition.advance_lsn();
        let ts = current_timestamp();
        let etag = new_etag();

        let new_doc = StoredDocument {
            body: body.clone(),
            id: doc_id.to_string(),
            rid: existing.rid.clone(),
            etag: etag.clone(),
            ts,
            self_link: existing.self_link.clone(),
            lsn,
            epk: epk.clone(),
        };

        inject_system_properties(&new_doc, &mut body);
        let new_doc = StoredDocument {
            body: body.clone(),
            ..new_doc
        };

        logical.insert(doc_id.to_string(), new_doc.clone());

        let num_props = RuChargingModel::count_properties(&body);
        let charge = store
            .config()
            .ru_model()
            .compute_replace_ru(request_body.len(), num_props);

        // Check throttle
        if let Some(response) =
            check_throttle(partition, charge, store.config().throttling_enabled())
        {
            return Err(response);
        }

        Ok((new_doc, token, charge, body))
    });

    match result {
        Some(Ok((doc, token, charge, response_body))) => {
            store.replicate(region_name, db_id, coll_id, &doc, false);

            if parsed.content_response_on_write {
                success_response(StatusCode::Ok, &response_body, charge, &token)
                    .with_etag(&doc.etag)
                    .with_item_count(1)
                    .build()
            } else {
                ResponseBuilder::new(StatusCode::Ok)
                    .with_request_charge(charge)
                    .with_session_token(&token)
                    .with_etag(&doc.etag)
                    .build()
            }
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id),
    }
}

async fn handle_upsert(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
    request_body: &[u8],
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");

    let mut body: serde_json::Value = match serde_json::from_slice(request_body) {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BadRequest,
                None,
                "BadRequest",
                "Invalid JSON body",
                1.0,
                "",
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
                1.0,
                "",
            )
            .build();
        }
    };

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let (_, epk) = resolve_partition_key(parsed, &body, &state.metadata);

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
                )
                .build());
            }
        };

        // Check partition lock
        if let Some(response) = check_partition_lock(partition) {
            return Err(response);
        }

        let lsn = partition.advance_lsn();
        let ts = current_timestamp();
        let etag = new_etag();

        let mut docs = partition.documents.write().unwrap();
        let logical = docs.entry(epk.clone()).or_default();

        let (status, rid, self_link) = if let Some(existing) = logical.get(&doc_id) {
            // Replace path
            (
                StatusCode::Ok,
                existing.rid.clone(),
                existing.self_link.clone(),
            )
        } else {
            // Create path
            let (_, doc_rid) = store
                .rid_generator()
                .next_document_rid(state.metadata.numeric_db_id, state.metadata.numeric_coll_id);
            let self_link = format!("{}docs/{}/", state.metadata.self_link, doc_rid);
            (StatusCode::Created, doc_rid, self_link)
        };

        let new_doc = StoredDocument {
            body: body.clone(),
            id: doc_id.clone(),
            rid,
            etag: etag.clone(),
            ts,
            self_link,
            lsn,
            epk: epk.clone(),
        };

        inject_system_properties(&new_doc, &mut body);
        let new_doc = StoredDocument {
            body: body.clone(),
            ..new_doc
        };

        logical.insert(doc_id, new_doc.clone());

        let num_props = RuChargingModel::count_properties(&body);
        let charge = if status == StatusCode::Created {
            store
                .config()
                .ru_model()
                .compute_create_ru(request_body.len(), num_props)
        } else {
            store
                .config()
                .ru_model()
                .compute_replace_ru(request_body.len(), num_props)
        };

        // Check throttle
        if let Some(response) =
            check_throttle(partition, charge, store.config().throttling_enabled())
        {
            return Err(response);
        }

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(partition, region_id);
        Ok((new_doc, status, token, charge, body))
    });

    match result {
        Some(Ok((doc, status, token, charge, response_body))) => {
            store.replicate(region_name, db_id, coll_id, &doc, false);

            if parsed.content_response_on_write {
                success_response(status, &response_body, charge, &token)
                    .with_etag(&doc.etag)
                    .with_item_count(1)
                    .build()
            } else {
                ResponseBuilder::new(status)
                    .with_request_charge(charge)
                    .with_session_token(&token)
                    .with_etag(&doc.etag)
                    .build()
            }
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id),
    }
}

async fn handle_delete(
    store: &Arc<EmulatorStore>,
    region_name: &str,
    parsed: &ParsedRequest,
) -> AsyncRawResponse {
    let db_id = parsed.db_id.as_deref().unwrap_or("");
    let coll_id = parsed.coll_id.as_deref().unwrap_or("");
    let doc_id = parsed.doc_id.as_deref().unwrap_or("");

    let region_ref = match store.region(region_name) {
        Some(r) => r,
        None => return not_found_region(),
    };

    let result = region_ref.with_container(db_id, coll_id, |state| {
        let empty_body = serde_json::Value::Null;
        let (_, epk) = resolve_partition_key(parsed, &empty_body, &state.metadata);

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
                )
                .build());
            }
        };

        // Check partition lock (split/merge in progress)
        if let Some(response) = check_partition_lock(partition) {
            return Err(response);
        }

        let region_id = store.config().region_id_for(region_name);
        let token = session_token_for(partition, region_id);

        let mut docs = partition.documents.write().unwrap();
        let logical = match docs.get_mut(&epk) {
            Some(l) => l,
            None => {
                return Err(error_response(
                    StatusCode::NotFound,
                    None,
                    "NotFound",
                    &format!(
                        "Entity with the specified id does not exist in the system. ResourceId: {}",
                        doc_id
                    ),
                    1.0,
                    &token,
                )
                .build());
            }
        };

        let existing = match logical.get(doc_id) {
            Some(e) => e.clone(),
            None => {
                return Err(error_response(
                    StatusCode::NotFound,
                    None,
                    "NotFound",
                    &format!(
                        "Entity with the specified id does not exist in the system. ResourceId: {}",
                        doc_id
                    ),
                    1.0,
                    &token,
                )
                .build());
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
                )
                .build());
            }
        }

        let lsn = partition.advance_lsn();
        logical.remove(doc_id);

        // Create a "tombstone" for replication
        let tombstone = StoredDocument {
            body: serde_json::Value::Null,
            id: doc_id.to_string(),
            rid: existing.rid,
            etag: existing.etag,
            ts: current_timestamp(),
            self_link: existing.self_link,
            lsn,
            epk,
        };

        let num_props = RuChargingModel::count_properties(&existing.body);
        let body_size = serde_json::to_vec(&existing.body).unwrap_or_default().len();
        let charge = store
            .config()
            .ru_model()
            .compute_replace_ru(body_size, num_props);

        // Check throttle
        if let Some(response) =
            check_throttle(partition, charge, store.config().throttling_enabled())
        {
            return Err(response);
        }

        Ok((tombstone, token, charge))
    });

    match result {
        Some(Ok((tombstone, token, charge))) => {
            store.replicate(region_name, db_id, coll_id, &tombstone, true);

            ResponseBuilder::new(StatusCode::NoContent)
                .with_request_charge(charge)
                .with_session_token(&token)
                .build()
        }
        Some(Err(response)) => response,
        None => container_not_found(db_id, coll_id),
    }
}

// --- Helper Responses ---

fn write_forbidden_response() -> AsyncRawResponse {
    error_response(
        StatusCode::Forbidden,
        Some(3),
        "Forbidden",
        "Write operations are not allowed on this region.",
        0.0,
        "",
    )
    .build()
}

fn unsupported_response(operation: &str) -> AsyncRawResponse {
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
    )
    .build()
}

fn not_found_region() -> AsyncRawResponse {
    error_response(
        StatusCode::NotFound,
        None,
        "NotFound",
        "Region not found",
        0.0,
        "",
    )
    .build()
}

fn container_not_found(db_id: &str, coll_id: &str) -> AsyncRawResponse {
    error_response(
        StatusCode::NotFound,
        None,
        "NotFound",
        &format!("Container '{}/{}' does not exist", db_id, coll_id),
        1.0,
        "",
    )
    .build()
}
