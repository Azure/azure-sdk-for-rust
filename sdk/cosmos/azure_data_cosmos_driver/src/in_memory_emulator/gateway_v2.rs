// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Hosted Gateway V2 adapter for the in-memory emulator.

use azure_core::{
    http::{
        headers::{HeaderName, HeaderValue, Headers},
        AsyncRawResponse, Method, Request, StatusCode,
    },
    Bytes,
};
use uuid::Uuid;

use crate::{
    driver::transport::rntbd::{
        tokens::{RntbdRequestToken, TokenValue},
        RntbdRequestFrame, RntbdResponse,
    },
    models::{
        cosmos_headers::request_header_names, effective_partition_key::EffectivePartitionKey,
        CosmosStatus, OperationType, ResourceType,
    },
    options::ReadConsistencyStrategy,
};

use super::{dispatch::EFFECTIVE_PARTITION_KEY, ConsistencyLevel, InMemoryEmulatorHttpClient};

/// Binary-wire observations for one hosted Gateway V2 request.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GatewayV2PayloadAudit {
    pub binary_negotiated: bool,
    pub binary_request_payload: bool,
    pub binary_response_payload: bool,
    pub read_consistency_strategy: Option<ReadConsistencyStrategy>,
}

impl InMemoryEmulatorHttpClient {
    /// Executes a hosted Gateway V2 request and returns an RNTBD-framed response.
    #[doc(hidden)]
    pub async fn execute_gateway_v2_request(
        &self,
        request: &Request,
    ) -> crate::error::Result<AsyncRawResponse> {
        Ok(self.execute_gateway_v2_request_audited(request).await?.0)
    }

    /// Executes a hosted Gateway V2 request and reports binary wire evidence.
    #[doc(hidden)]
    pub async fn execute_gateway_v2_request_audited(
        &self,
        request: &Request,
    ) -> crate::error::Result<(AsyncRawResponse, GatewayV2PayloadAudit)> {
        let request_body: Bytes = request.body().into();
        // A frame that fails to parse has no usable `activityId` field, so
        // there is nothing to echo back — but the response must still be a
        // well-formed RNTBD frame (not a bare JSON body) so that any client
        // speaking Gateway 2.0, not just the Rust driver, can decode the
        // failure the same way it decodes every other error on this path.
        let frame = match RntbdRequestFrame::read(request_body.as_ref()) {
            Ok(frame) => frame,
            Err(error) => {
                return Ok((
                    encode_error_response(gateway_v2_bad_request(error), Uuid::new_v4()).await?,
                    GatewayV2PayloadAudit::default(),
                ));
            }
        };
        let mut audit = GatewayV2PayloadAudit {
            binary_request_payload: frame
                .body
                .as_deref()
                .is_some_and(crate::binary_json::is_binary),
            ..Default::default()
        };
        audit.binary_negotiated = frame.metadata.iter().any(|token| {
            RntbdRequestToken::try_from(token.id.0)
                == Ok(RntbdRequestToken::SupportedSerializationFormats)
                && matches!(token.value, TokenValue::Byte(flags) if flags & 0x02 != 0)
        });
        audit.read_consistency_strategy = frame.metadata.iter().find_map(|token| {
            if RntbdRequestToken::try_from(token.id.0)
                != Ok(RntbdRequestToken::ReadConsistencyStrategy)
            {
                return None;
            }
            match token.value {
                TokenValue::Byte(value) => ReadConsistencyStrategy::from_rntbd_wire_byte(value),
                _ => None,
            }
        });
        let activity_id = frame.activity_id;
        let request = match decode_request(request, frame, self.store().config().consistency()) {
            Ok(request) => request,
            Err(error) => {
                return Ok((encode_error_response(error, activity_id).await?, audit));
            }
        };
        let response = self.execute_request(&request).await?;
        let (response, binary_response_payload) =
            encode_response_audited(response, activity_id).await?;
        audit.binary_response_payload = binary_response_payload;
        Ok((response, audit))
    }
}

async fn encode_error_response(
    error: crate::error::CosmosError,
    activity_id: Uuid,
) -> crate::error::Result<AsyncRawResponse> {
    let mut headers = Headers::new();
    headers.insert("x-ms-activity-id", activity_id.to_string());
    if let Some(sub_status) = error.status().sub_status() {
        headers.insert("x-ms-substatus", sub_status.value().to_string());
    }
    let body = serde_json::to_vec(&serde_json::json!({
        "code": "BadRequest",
        "message": error.to_string(),
    }))
    .map_err(gateway_v2_internal_error)?;
    encode_response(
        AsyncRawResponse::from_bytes(error.status().status_code(), headers, body),
        activity_id,
    )
    .await
}

#[derive(Default)]
struct RequestMetadata {
    payload_present: Option<bool>,
    database: Option<String>,
    collection: Option<String>,
    document: Option<String>,
    partition_key: Option<String>,
    partition_key_range_id: Option<String>,
    continuation: Option<String>,
    session_token: Option<String>,
    match_condition: Option<String>,
    if_modified_since: Option<String>,
    a_im: Option<String>,
    change_feed_wire_format_version: Option<String>,
    effective_partition_key: Option<String>,
    start_epk_hash: Option<String>,
    end_epk_hash: Option<String>,
    start_epk: Option<String>,
    end_epk: Option<String>,
    read_feed_key_type: Option<&'static str>,
    page_size: Option<u32>,
    return_minimal: bool,
    batch_atomic: Option<bool>,
    batch_continue_on_error: Option<bool>,
    supported_query_features: Option<String>,
    query_version: Option<String>,
    allow_tentative_writes: bool,
    consistency_level: Option<u8>,
    read_consistency_strategy: Option<u8>,
    supported_serialization_formats: Option<u8>,
}

fn decode_request(
    outer_request: &Request,
    frame: RntbdRequestFrame,
    account_consistency: ConsistencyLevel,
) -> crate::error::Result<Request> {
    if frame.resource_type != ResourceType::Document {
        return Err(gateway_v2_bad_request(format!(
            "hosted Gateway V2 supports Document resources, got {:?}",
            frame.resource_type
        )));
    }
    let body_present = frame.body.is_some();
    let metadata = decode_metadata(frame.metadata)?;
    let payload_present = metadata
        .payload_present
        .ok_or_else(|| gateway_v2_bad_request("RNTBD request is missing PayloadPresent"))?;
    if payload_present != body_present {
        return Err(gateway_v2_bad_request(format!(
            "RNTBD PayloadPresent was {payload_present} but body presence was {body_present}"
        )));
    }
    if metadata.allow_tentative_writes {
        return Err(gateway_v2_bad_request(
            "hosted Gateway V2 does not yet support AllowTentativeWrites",
        ));
    }
    let read_consistency_strategy = metadata
        .read_consistency_strategy
        .map(read_consistency_strategy_from_wire)
        .transpose()?;
    if let Some(value) = metadata.consistency_level {
        if !matches!(value, 0x00..=0x04) {
            return Err(gateway_v2_bad_request(
                "RNTBD request contains an unknown ConsistencyLevel value",
            ));
        }
        if value != consistency_wire_byte(account_consistency) {
            return Err(gateway_v2_bad_request(
                "hosted Gateway V2 does not yet support per-request consistency overrides",
            ));
        }
    }
    if matches!(
        frame.operation_type,
        OperationType::Read | OperationType::Replace | OperationType::Patch | OperationType::Delete
    ) && metadata.partition_key.is_none()
        && metadata.effective_partition_key.is_some()
    {
        return Err(gateway_v2_bad_request(
            "hosted Gateway V2 requires the string PartitionKey token for point operations",
        ));
    }
    tracing::debug!(
        operation = ?frame.operation_type,
        partition_key_range_id = ?metadata.partition_key_range_id,
        start_epk = ?metadata.start_epk,
        end_epk = ?metadata.end_epk,
        "decoded hosted Gateway V2 request target"
    );
    let database = metadata
        .database
        .as_deref()
        .ok_or_else(|| gateway_v2_bad_request("RNTBD request is missing DatabaseName"))?;
    let collection = metadata
        .collection
        .as_deref()
        .ok_or_else(|| gateway_v2_bad_request("RNTBD request is missing CollectionName"))?;

    let (method, document_required) = match frame.operation_type {
        OperationType::Create | OperationType::Upsert => (Method::Post, false),
        OperationType::Read => (Method::Get, true),
        OperationType::Replace => (Method::Put, true),
        OperationType::Patch => (Method::Patch, true),
        OperationType::Delete => (Method::Delete, true),
        OperationType::Query
        | OperationType::SqlQuery
        | OperationType::QueryPlan
        | OperationType::Batch => (Method::Post, false),
        OperationType::ReadFeed => (Method::Get, false),
        operation => {
            return Err(gateway_v2_bad_request(format!(
                "unsupported hosted Gateway V2 operation {operation:?}"
            )));
        }
    };

    let outer_path = outer_request.url().path().to_owned();
    let mut url = outer_request.url().clone();
    url.set_query(None);
    url.set_fragment(None);
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|_| gateway_v2_bad_request("thin-client endpoint cannot be a base URL"))?;
        segments
            .clear()
            .push("dbs")
            .push(database)
            .push("colls")
            .push(collection)
            .push("docs");
        if document_required {
            let document = metadata.document.as_deref().ok_or_else(|| {
                gateway_v2_bad_request("RNTBD point request is missing DocumentName")
            })?;
            segments.push(document);
        }
    }
    if outer_path != url.path() {
        return Err(gateway_v2_bad_request(format!(
            "Gateway 2.0 outer path '{outer_path}' does not match RNTBD target '{}'",
            url.path()
        )));
    }

    let mut request = Request::new(url, method);
    request.headers_mut().insert(
        "x-ms-activity-id",
        HeaderValue::from(frame.activity_id.to_string()),
    );
    if let Some(value) = metadata.partition_key {
        request
            .headers_mut()
            .insert("x-ms-documentdb-partitionkey", value);
    }
    if let Some(value) = metadata.partition_key_range_id {
        request
            .headers_mut()
            .insert("x-ms-documentdb-partitionkeyrangeid", value);
    }
    if let Some(value) = metadata.effective_partition_key {
        request.headers_mut().insert(EFFECTIVE_PARTITION_KEY, value);
    }
    if let Some(value) = metadata.continuation {
        request.headers_mut().insert("x-ms-continuation", value);
    }
    if let Some(value) = metadata.session_token {
        request.headers_mut().insert("x-ms-session-token", value);
    }
    if let Some(strategy) = read_consistency_strategy {
        request
            .headers_mut()
            .insert("x-ms-cosmos-read-consistency-strategy", strategy.as_str());
    }
    if let Some(value) = metadata.page_size {
        let value = if value == u32::MAX {
            "-1".to_owned()
        } else {
            value.to_string()
        };
        request.headers_mut().insert("x-ms-max-item-count", value);
    }
    if let Some(value) = metadata.match_condition {
        let header = match frame.operation_type {
            OperationType::Read | OperationType::ReadFeed => "if-none-match",
            _ => "if-match",
        };
        request.headers_mut().insert(header, value);
    }
    if let Some(value) = metadata.if_modified_since {
        request.headers_mut().insert("if-modified-since", value);
    }
    if let Some(value) = metadata.a_im {
        request.headers_mut().insert("a-im", value);
    }
    if let Some(value) = metadata.change_feed_wire_format_version {
        request
            .headers_mut()
            .insert("x-ms-cosmos-changefeed-wire-format-version", value);
    }
    if let Some(value) = metadata.start_epk {
        request.headers_mut().insert("x-ms-start-epk", value);
    }
    if let Some(value) = metadata.end_epk {
        request.headers_mut().insert("x-ms-end-epk", value);
    }
    if let Some(value) = metadata.read_feed_key_type {
        request
            .headers_mut()
            .insert(request_header_names::READ_FEED_KEY_TYPE, value);
    }
    if let Some(value) = metadata.start_epk_hash {
        request
            .headers_mut()
            .insert(request_header_names::THINCLIENT_PKRANGE_MIN, value);
    }
    if let Some(value) = metadata.end_epk_hash {
        request
            .headers_mut()
            .insert(request_header_names::THINCLIENT_PKRANGE_MAX, value);
    }
    if metadata.return_minimal {
        request.headers_mut().insert("prefer", "return=minimal");
    }
    if let Some(value) = metadata.supported_query_features {
        request
            .headers_mut()
            .insert("x-ms-cosmos-supported-query-features", value);
    }
    if let Some(value) = metadata.query_version {
        request
            .headers_mut()
            .insert("x-ms-cosmos-query-version", value);
    }
    if let Some(flags) = metadata.supported_serialization_formats {
        // Re-emit the negotiation header so the shared dispatch models a binary
        // response. Flag bits mirror the driver: 0x01 JsonText, 0x02 CosmosBinary,
        // 0x04 HybridRow.
        let mut formats = Vec::new();
        if flags & 0x01 != 0 {
            formats.push("JsonText");
        }
        if flags & 0x02 != 0 {
            formats.push("CosmosBinary");
        }
        if flags & 0x04 != 0 {
            formats.push("HybridRow");
        }
        if !formats.is_empty() {
            request.headers_mut().insert(
                "x-ms-cosmos-supported-serialization-formats",
                formats.join(","),
            );
        }
    }

    match frame.operation_type {
        OperationType::Upsert => request
            .headers_mut()
            .insert("x-ms-documentdb-is-upsert", "true"),
        OperationType::Query | OperationType::SqlQuery => request
            .headers_mut()
            .insert("x-ms-documentdb-isquery", "true"),
        OperationType::QueryPlan => request
            .headers_mut()
            .insert("x-ms-cosmos-is-query-plan-request", "true"),
        OperationType::Batch => {
            request
                .headers_mut()
                .insert(request_header_names::IS_BATCH_REQUEST, "true");
            for (name, value) in [
                (request_header_names::BATCH_ATOMIC, metadata.batch_atomic),
                (
                    request_header_names::BATCH_CONTINUE_ON_ERROR,
                    metadata.batch_continue_on_error,
                ),
            ] {
                if let Some(value) = value {
                    request.headers_mut().insert(
                        HeaderName::from_static(name),
                        if value { "true" } else { "false" },
                    );
                }
            }
        }
        _ => {}
    }
    if let Some(body) = frame.body {
        request.set_body(body);
    }
    Ok(request)
}

fn read_consistency_strategy_from_wire(value: u8) -> crate::error::Result<ReadConsistencyStrategy> {
    ReadConsistencyStrategy::from_rntbd_wire_byte(value).ok_or_else(|| {
        gateway_v2_bad_request(format!(
            "RNTBD request contains unknown ReadConsistencyStrategy value {value:#04x}"
        ))
    })
}

fn decode_metadata(
    tokens: Vec<crate::driver::transport::rntbd::Token>,
) -> crate::error::Result<RequestMetadata> {
    let mut metadata = RequestMetadata::default();
    for token in tokens {
        let Ok(kind) = RntbdRequestToken::try_from(token.id.value()) else {
            continue;
        };
        match kind {
            RntbdRequestToken::DatabaseName => {
                metadata.database = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::CollectionName => {
                metadata.collection = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::DocumentName => {
                metadata.document = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::PartitionKey => {
                metadata.partition_key = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::PartitionKeyRangeId => {
                metadata.partition_key_range_id = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::ContinuationToken => {
                metadata.continuation = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::SessionToken => {
                metadata.session_token = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::Match => {
                metadata.match_condition = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::IfModifiedSince => {
                metadata.if_modified_since = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::AIm => {
                metadata.a_im = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::ChangeFeedWireFormatVersion => {
                metadata.change_feed_wire_format_version = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::StartEpkHash => {
                metadata.start_epk_hash = Some(expect_hex(kind, token.value)?);
            }
            RntbdRequestToken::EndEpkHash => {
                metadata.end_epk_hash = Some(expect_hex(kind, token.value)?);
            }
            RntbdRequestToken::StartEpk => {
                metadata.start_epk = Some(expect_epk_text(kind, token.value)?);
            }
            RntbdRequestToken::EndEpk => {
                metadata.end_epk = Some(expect_epk_text(kind, token.value)?);
            }
            RntbdRequestToken::ReadFeedKeyType => {
                metadata.read_feed_key_type = Some(match expect_byte(kind, token.value)? {
                    1 => "ResourceId",
                    2 => "EffectivePartitionKey",
                    3 => request_header_names::READ_FEED_KEY_TYPE_EPK_RANGE,
                    _ => return Err(gateway_v2_bad_request("unknown ReadFeedKeyType value")),
                });
            }
            RntbdRequestToken::EffectivePartitionKey => {
                metadata.effective_partition_key = Some(expect_hex(kind, token.value)?);
            }
            RntbdRequestToken::PageSize => {
                metadata.page_size = Some(expect_ulong(kind, token.value)?);
            }
            RntbdRequestToken::ReturnPreference => {
                metadata.return_minimal = expect_byte(kind, token.value)? != 0;
            }
            RntbdRequestToken::IsBatchAtomic => {
                metadata.batch_atomic = Some(expect_byte(kind, token.value)? != 0);
            }
            RntbdRequestToken::ShouldBatchContinueOnError => {
                metadata.batch_continue_on_error = Some(expect_byte(kind, token.value)? != 0);
            }
            RntbdRequestToken::SupportedQueryFeatures => {
                metadata.supported_query_features = Some(expect_string(kind, token.value)?);
            }
            RntbdRequestToken::QueryVersion => {
                metadata.query_version = Some(expect_small_string(kind, token.value)?);
            }
            RntbdRequestToken::SupportedSerializationFormats => {
                // Captured here and re-emitted as the HTTP negotiation header in
                // `decode_request`, so the shared dispatch models a binary response.
                metadata.supported_serialization_formats = Some(expect_byte(kind, token.value)?);
            }
            RntbdRequestToken::AllowTentativeWrites => {
                metadata.allow_tentative_writes = expect_byte(kind, token.value)? != 0;
            }
            RntbdRequestToken::ConsistencyLevel => {
                metadata.consistency_level = Some(expect_byte(kind, token.value)?);
            }
            RntbdRequestToken::ReadConsistencyStrategy => {
                metadata.read_consistency_strategy = Some(expect_byte(kind, token.value)?);
            }
            RntbdRequestToken::PayloadPresent => {
                if metadata.payload_present.is_some() {
                    return Err(gateway_v2_bad_request(
                        "RNTBD request contains duplicate PayloadPresent tokens",
                    ));
                }
                metadata.payload_present = Some(expect_byte(kind, token.value)? != 0);
            }
            RntbdRequestToken::ResourceId => {
                expect_bytes(kind, token.value)?;
            }
            RntbdRequestToken::AuthorizationToken
            | RntbdRequestToken::CollectionRid
            | RntbdRequestToken::GlobalDatabaseAccountName => {
                expect_string(kind, token.value)?;
            }
            RntbdRequestToken::Date => {
                expect_small_string(kind, token.value)?;
            }
            RntbdRequestToken::TransportRequestId | RntbdRequestToken::SDKSupportedCapabilities => {
                expect_ulong(kind, token.value)?;
            }
        }
    }
    Ok(metadata)
}

fn expect_string(kind: RntbdRequestToken, value: TokenValue) -> crate::error::Result<String> {
    match value {
        TokenValue::String(value) => Ok(value),
        other => Err(wrong_token_type(kind, "String", other)),
    }
}

fn expect_epk_text(kind: RntbdRequestToken, value: TokenValue) -> crate::error::Result<String> {
    let value = String::from_utf8(expect_bytes(kind, value)?).map_err(gateway_v2_bad_request)?;
    EffectivePartitionKey::try_from(value.as_str()).map_err(gateway_v2_bad_request)?;
    Ok(value)
}

fn expect_small_string(kind: RntbdRequestToken, value: TokenValue) -> crate::error::Result<String> {
    match value {
        TokenValue::SmallString(value) => Ok(value),
        other => Err(wrong_token_type(kind, "SmallString", other)),
    }
}

fn expect_bytes(kind: RntbdRequestToken, value: TokenValue) -> crate::error::Result<Vec<u8>> {
    match value {
        TokenValue::Bytes(value) => Ok(value),
        other => Err(wrong_token_type(kind, "Bytes", other)),
    }
}

fn expect_hex(kind: RntbdRequestToken, value: TokenValue) -> crate::error::Result<String> {
    Ok(expect_bytes(kind, value)?
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect())
}

fn expect_byte(kind: RntbdRequestToken, value: TokenValue) -> crate::error::Result<u8> {
    match value {
        TokenValue::Byte(value) => Ok(value),
        other => Err(wrong_token_type(kind, "Byte", other)),
    }
}

fn expect_ulong(kind: RntbdRequestToken, value: TokenValue) -> crate::error::Result<u32> {
    match value {
        TokenValue::ULong(value) => Ok(value),
        other => Err(wrong_token_type(kind, "ULong", other)),
    }
}

fn wrong_token_type(
    kind: RntbdRequestToken,
    expected: &str,
    actual: TokenValue,
) -> crate::error::CosmosError {
    gateway_v2_bad_request(format!(
        "RNTBD token {kind:?} must use {expected}, got {actual:?}"
    ))
}

fn consistency_wire_byte(value: ConsistencyLevel) -> u8 {
    match value {
        ConsistencyLevel::Strong => 0x00,
        ConsistencyLevel::BoundedStaleness => 0x01,
        ConsistencyLevel::Session => 0x02,
        ConsistencyLevel::Eventual => 0x03,
        ConsistencyLevel::ConsistentPrefix => 0x04,
    }
}

async fn encode_response(
    response: AsyncRawResponse,
    request_activity_id: Uuid,
) -> crate::error::Result<AsyncRawResponse> {
    Ok(encode_response_audited(response, request_activity_id)
        .await?
        .0)
}

async fn encode_response_audited(
    response: AsyncRawResponse,
    request_activity_id: Uuid,
) -> crate::error::Result<(AsyncRawResponse, bool)> {
    let response = response
        .try_into_raw_response()
        .await
        .map_err(gateway_v2_internal_error)?;
    let binary_response_payload = crate::binary_json::is_binary(response.body().as_ref());
    let headers = response.headers();
    let status = header_u32(headers, "x-ms-substatus")
        .map(|sub_status| CosmosStatus::new(response.status()).with_sub_status(sub_status as u16))
        .unwrap_or_else(|| CosmosStatus::new(response.status()));
    let activity_id = header_string(headers, "x-ms-activity-id")
        .and_then(|value| Uuid::parse_str(&value).ok())
        .unwrap_or(request_activity_id);
    let rntbd = RntbdResponse {
        status,
        activity_id,
        body: response.body().as_ref().to_vec(),
        continuation_token: header_string(headers, "x-ms-continuation"),
        etag: header_string(headers, "etag"),
        retry_after_ms: header_u32(headers, "x-ms-retry-after-ms"),
        last_state_change_date_time: None,
        storage_max_resource_quota: None,
        storage_resource_quota_usage: None,
        schema_version: None,
        lsn: header_i64(headers, "lsn"),
        item_count: header_u32(headers, "x-ms-item-count"),
        request_charge: header_f64(headers, "x-ms-request-charge"),
        backend_request_duration_ms: header_f64(headers, "x-ms-request-duration-ms"),
        owner_full_name: header_string(headers, "x-ms-alt-content-path"),
        owner_id: None,
        quorum_acked_lsn: None,
        current_write_quorum: None,
        current_replica_set_size: None,
        partition_key_range_id: header_string(headers, "x-ms-documentdb-partitionkeyrangeid"),
        xp_role: None,
        number_of_read_regions: None,
        item_lsn: header_i64(headers, "x-ms-item-lsn"),
        global_committed_lsn: header_i64(headers, "x-ms-global-committed-lsn"),
        local_lsn: None,
        quorum_acked_local_lsn: None,
        item_local_lsn: None,
        transport_request_id: header_u32(headers, "x-ms-transport-request-id"),
        session_token: header_string(headers, "x-ms-session-token"),
        query_metrics: header_string(headers, "x-ms-documentdb-query-metrics"),
        index_utilization: header_string(headers, "x-ms-cosmos-index-utilization"),
        query_execution_info: None,
        pending_pk_delete: None,
        physical_partition_id: None,
        conflict_resolved_timestamp: None,
    };
    let mut body = Vec::new();
    rntbd.write(&mut body).map_err(gateway_v2_internal_error)?;
    let mut outer_headers = Headers::new();
    outer_headers.insert("content-type", "application/octet-stream");
    Ok((
        AsyncRawResponse::from_bytes(status.status_code(), outer_headers, body),
        binary_response_payload,
    ))
}

fn header_string(headers: &Headers, name: &'static str) -> Option<String> {
    headers
        .get_optional_str(&HeaderName::from_static(name))
        .map(str::to_owned)
}

fn header_u32(headers: &Headers, name: &'static str) -> Option<u32> {
    header_string(headers, name)?.parse().ok()
}

fn header_i64(headers: &Headers, name: &'static str) -> Option<i64> {
    header_string(headers, name)?.parse().ok()
}

fn header_f64(headers: &Headers, name: &'static str) -> Option<f64> {
    header_string(headers, name)?.parse().ok()
}

fn gateway_v2_bad_request(error: impl std::fmt::Display) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(CosmosStatus::new(StatusCode::BadRequest))
        .with_message(error.to_string())
        .build()
}

fn gateway_v2_internal_error(error: impl std::fmt::Display) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(CosmosStatus::new(StatusCode::InternalServerError))
        .with_message(error.to_string())
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        driver::transport::{
            cosmos_transport_client::HttpRequest, rntbd::Token, wrap_request_for_gateway_v2,
            AuthorizationContext, WrapInputs,
        },
        in_memory_emulator::{ContainerConfig, VirtualAccountConfig, VirtualRegion},
        models::{
            DefaultConsistencyLevel, FeedRange, PartitionKey, PartitionKeyDefinition,
            PartitionKeyValue,
        },
    };
    use url::Url;

    #[test]
    fn batch_control_tokens_preserve_explicit_false_and_absence() {
        for atomic in [None, Some(false), Some(true)] {
            for continue_on_error in [None, Some(false), Some(true)] {
                let mut metadata = vec![
                    Token::database_name("db".into()),
                    Token::collection_name("coll".into()),
                    Token::partition_key(r#"["A"]"#.into()),
                    Token::payload_present(true),
                ];
                if let Some(value) = atomic {
                    metadata.push(Token::is_batch_atomic(value));
                }
                if let Some(value) = continue_on_error {
                    metadata.push(Token::should_batch_continue_on_error(value));
                }
                let frame = RntbdRequestFrame {
                    resource_type: ResourceType::Document,
                    operation_type: OperationType::Batch,
                    activity_id: Uuid::nil(),
                    metadata,
                    body: Some(br#"[{"operationType":"Read","id":"batch"}]"#.to_vec()),
                };
                let outer = Request::new(
                    "http://127.0.0.1:18444/dbs/db/colls/coll/docs"
                        .parse()
                        .unwrap(),
                    Method::Post,
                );
                let decoded = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap();
                for (name, value) in [
                    (request_header_names::BATCH_ATOMIC, atomic),
                    (
                        request_header_names::BATCH_CONTINUE_ON_ERROR,
                        continue_on_error,
                    ),
                ] {
                    assert_eq!(
                        decoded
                            .headers()
                            .get_optional_str(&HeaderName::from_static(name)),
                        value.map(|value| if value { "true" } else { "false" }),
                    );
                }
            }
        }
    }

    #[test]
    fn batch_control_tokens_require_byte_values() {
        for kind in [
            RntbdRequestToken::IsBatchAtomic,
            RntbdRequestToken::ShouldBatchContinueOnError,
        ] {
            let error = decode_metadata(vec![Token::new(kind, TokenValue::String("true".into()))])
                .err()
                .expect("batch controls must reject non-Byte tokens");
            assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        }
    }

    async fn execute_encoded_atomic_batch(
        emulator: &InMemoryEmulatorHttpClient,
        thin_url: &Url,
        operations: serde_json::Value,
    ) -> serde_json::Value {
        let payload = serde_json::to_vec(&operations).unwrap();
        let mut headers = Headers::new();
        headers.insert("authorization", "test-authorization");
        headers.insert("x-ms-date", "Wed, 21 Oct 2015 07:28:00 GMT");
        headers.insert("x-ms-activity-id", Uuid::nil().to_string());
        headers.insert(request_header_names::PARTITION_KEY, r#"["A"]"#);
        headers.insert(request_header_names::BATCH_ATOMIC, "True");
        headers.insert(request_header_names::BATCH_CONTINUE_ON_ERROR, "True");
        let context =
            AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db/colls/coll");
        let wrapped = wrap_request_for_gateway_v2(
            HttpRequest {
                url: thin_url.join("dbs/db/colls/coll/docs").unwrap(),
                method: Method::Post,
                headers,
                body: Some(Bytes::from(payload.clone())),
                timeout: None,
                #[cfg(feature = "fault_injection")]
                evaluation_collector: None,
            },
            &WrapInputs {
                auth_context: &context,
                operation_type: OperationType::Batch,
                resource_type: ResourceType::Document,
                effective_partition_key: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                read_consistency_strategy: ReadConsistencyStrategy::Default,
                account_name: Some("account"),
                collection_rid: None,
            },
        )
        .unwrap();
        let frame = RntbdRequestFrame::read(wrapped.body.as_ref().unwrap()).unwrap();
        let controls: Vec<_> = frame
            .metadata
            .iter()
            .filter(|token| matches!(token.id.value(), 0x0073 | 0x0074))
            .map(|token| (token.id.value(), token.as_byte()))
            .collect();
        assert_eq!(controls, [(0x0073, Some(1)), (0x0074, Some(1))]);
        assert_eq!(frame.body.as_deref(), Some(payload.as_slice()));
        let response = execute_frame(emulator, thin_url, frame).await;
        serde_json::from_slice(&response.body).unwrap()
    }

    #[tokio::test]
    async fn encoded_atomic_batches_preserve_reads_updates_and_rollback() {
        let gateway_url: Url = "http://127.0.0.1:18081/".parse().unwrap();
        let thin_url: Url = "http://127.0.0.1:18444/".parse().unwrap();
        let region = VirtualRegion::new("East US", gateway_url.clone())
            .with_gateway_v2_url(thin_url.clone());
        let emulator = InMemoryEmulatorHttpClient::new(
            VirtualAccountConfig::new(vec![region])
                .unwrap()
                .with_consistency(ConsistencyLevel::Session),
        );
        let store = emulator.store();
        store.create_database("db");
        store.create_container(
            "db",
            "coll",
            serde_json::from_value(serde_json::json!({
                "paths":["/pk"],"kind":"Hash","version":2
            }))
            .unwrap(),
        );
        let created = execute_encoded_atomic_batch(
            &emulator,
            &thin_url,
            serde_json::json!([
                {"operationType":"Create","resourceBody":{"id":"existing","pk":"A","value":1}}
            ]),
        )
        .await;
        assert_eq!(created[0]["statusCode"], 201);
        let read = execute_encoded_atomic_batch(
            &emulator,
            &thin_url,
            serde_json::json!([
                {"operationType":"Read","id":"existing"}
            ]),
        )
        .await;
        assert_eq!(read[0]["statusCode"], 200);
        assert_eq!(read[0]["resourceBody"]["value"], 1);
        let updated = execute_encoded_atomic_batch(
            &emulator,
            &thin_url,
            serde_json::json!([
                {"operationType":"Upsert","resourceBody":{"id":"existing","pk":"A","value":2}},
                {"operationType":"Read","id":"existing"}
            ]),
        )
        .await;
        assert_eq!(updated[0]["statusCode"], 200);
        assert_eq!(updated[1]["statusCode"], 200);
        assert_eq!(updated[1]["resourceBody"]["value"], 2);
        let failed = execute_encoded_atomic_batch(
            &emulator,
            &thin_url,
            serde_json::json!([
                {"operationType":"Create","resourceBody":{"id":"rolled-back","pk":"A"}},
                {"operationType":"Read","id":"missing"}
            ]),
        )
        .await;
        assert_eq!(failed[0]["statusCode"], 424);
        assert_eq!(failed[1]["statusCode"], 404);
        let mut request = Request::new(
            gateway_url
                .join("dbs/db/colls/coll/docs/rolled-back")
                .unwrap(),
            Method::Get,
        );
        request
            .headers_mut()
            .insert(request_header_names::PARTITION_KEY, r#"["A"]"#);
        assert_eq!(
            emulator.execute_request(&request).await.unwrap().status(),
            StatusCode::NotFound
        );
    }

    #[tokio::test]
    async fn create_item_round_trips_through_gateway_v2() {
        let thin_url = Url::parse("http://127.0.0.1:18444/").unwrap();
        let region = VirtualRegion::new("East US", "http://127.0.0.1:18081/".parse().unwrap())
            .with_gateway_v2_url(thin_url.clone());
        let emulator =
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![region]).unwrap());
        let store = emulator.store();
        store.create_database("db");
        let partition_key: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        store.create_container_with_config(
            "db",
            "coll",
            partition_key,
            ContainerConfig::new()
                .with_partition_count(1)
                .build()
                .unwrap(),
        );

        let activity_id = Uuid::new_v4();
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Create,
            activity_id,
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::partition_key(r#"["pk1"]"#.to_owned()),
                Token::payload_present(true),
            ],
            body: Some(
                serde_json::to_vec(&serde_json::json!({
                    "id": "item1", "pk": "pk1", "value": 42
                }))
                .unwrap(),
            ),
        };
        let mut bytes = Vec::new();
        frame.write(&mut bytes).unwrap();
        let mut request = Request::new(
            thin_url.join("dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        request.set_body(bytes);

        let response = emulator
            .execute_gateway_v2_request(&request)
            .await
            .unwrap()
            .try_into_raw_response()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::Created);
        let response = RntbdResponse::read(response.body().as_ref()).unwrap();
        assert_eq!(response.status.status_code(), StatusCode::Created);
        assert_eq!(response.activity_id, activity_id);
        assert!(!response.body.is_empty());
    }

    #[tokio::test]
    async fn ordinary_read_feed_remains_flat_and_paginates_through_gateway_v2() {
        let thin_url = Url::parse("http://127.0.0.1:18444/").unwrap();
        let gateway_url = Url::parse("http://127.0.0.1:18081/").unwrap();
        let region = VirtualRegion::new("East US", gateway_url.clone())
            .with_gateway_v2_url(thin_url.clone());
        let emulator =
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![region]).unwrap());
        let store = emulator.store();
        store.create_database("db");
        let partition_key: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        store.create_container("db", "coll", partition_key);

        let read_page = |continuation: Option<String>| {
            let mut metadata = vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::page_size(1),
                Token::payload_present(false),
            ];
            if let Some(continuation) = continuation {
                metadata.push(Token::continuation_token(continuation));
            }
            RntbdRequestFrame {
                resource_type: ResourceType::Document,
                operation_type: OperationType::ReadFeed,
                activity_id: Uuid::new_v4(),
                metadata,
                body: None,
            }
        };

        let empty = execute_frame(&emulator, &thin_url, read_page(None)).await;
        assert_eq!(empty.status.status_code(), StatusCode::Ok);
        let empty_body: serde_json::Value = serde_json::from_slice(&empty.body).unwrap();
        assert_eq!(empty_body["Documents"].as_array().unwrap().len(), 0);
        assert!(empty.continuation_token.is_none());

        for id in ["item-1", "item-2"] {
            let mut seed = Request::new(
                gateway_url.join("dbs/db/colls/coll/docs").unwrap(),
                Method::Post,
            );
            seed.headers_mut().insert(
                "x-ms-documentdb-partitionkey",
                HeaderValue::from_static(r#"["A"]"#),
            );
            seed.set_body(serde_json::to_vec(&serde_json::json!({ "id": id, "pk": "A" })).unwrap());
            assert_eq!(
                emulator.execute_request(&seed).await.unwrap().status(),
                StatusCode::Created
            );
        }

        let legacy_change_feed = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::ReadFeed,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::a_im("Incremental Feed".to_owned()),
                Token::page_size(1),
                Token::payload_present(false),
            ],
            body: None,
        };
        let legacy = execute_frame(&emulator, &thin_url, legacy_change_feed).await;
        let legacy_body: serde_json::Value = serde_json::from_slice(&legacy.body).unwrap();
        let legacy_document = &legacy_body["Documents"].as_array().unwrap()[0];
        assert!(legacy_document.get("id").is_some());
        assert!(legacy_document.get("current").is_none());

        let structured_change_feed = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::ReadFeed,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::a_im("Incremental Feed".to_owned()),
                Token::change_feed_wire_format_version("2021-09-15".to_owned()),
                Token::page_size(1),
                Token::payload_present(false),
            ],
            body: None,
        };
        let structured = execute_frame(&emulator, &thin_url, structured_change_feed).await;
        let structured_body: serde_json::Value = serde_json::from_slice(&structured.body).unwrap();
        let structured_document = &structured_body["Documents"].as_array().unwrap()[0];
        assert!(structured_document.get("current").is_some());
        let structured_metadata = structured_document["metadata"].as_object().unwrap();
        assert!(structured_metadata.get("crts").is_some());
        assert!(structured_metadata.get("operationType").is_none());

        let first = execute_frame(&emulator, &thin_url, read_page(None)).await;
        let first_body: serde_json::Value = serde_json::from_slice(&first.body).unwrap();
        let first_document = &first_body["Documents"].as_array().unwrap()[0];
        assert!(first_document.get("id").is_some());
        assert!(first_document.get("current").is_none());
        let second = execute_frame(
            &emulator,
            &thin_url,
            read_page(first.continuation_token.clone()),
        )
        .await;
        let second_body: serde_json::Value = serde_json::from_slice(&second.body).unwrap();
        assert_eq!(second_body["Documents"].as_array().unwrap().len(), 1);
        assert!(second.continuation_token.is_none());
    }

    #[tokio::test]
    async fn ordinary_read_feed_stays_flat_through_gateway_v2_encoder_and_bridge() {
        let thin_url = Url::parse("http://127.0.0.1:18444/").unwrap();
        let gateway_url = Url::parse("http://127.0.0.1:18081/").unwrap();
        let region = VirtualRegion::new("East US", gateway_url.clone())
            .with_gateway_v2_url(thin_url.clone());
        let emulator =
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![region]).unwrap());
        let store = emulator.store();
        store.create_database("db");
        let partition_key: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        store.create_container("db", "coll", partition_key);

        let mut seed = Request::new(
            gateway_url.join("dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        seed.headers_mut().insert(
            "x-ms-documentdb-partitionkey",
            HeaderValue::from_static(r#"["A"]"#),
        );
        seed.set_body(
            serde_json::to_vec(&serde_json::json!({ "id": "item-1", "pk": "A" })).unwrap(),
        );
        assert_eq!(
            emulator.execute_request(&seed).await.unwrap().status(),
            StatusCode::Created
        );

        let activity_id = Uuid::new_v4();
        let mut headers = Headers::new();
        headers.insert("authorization", "auth-token");
        headers.insert("x-ms-date", "Wed, 21 Oct 2015 07:28:00 GMT");
        headers.insert("x-ms-activity-id", activity_id.to_string());
        let request = HttpRequest {
            url: thin_url.join("dbs/db/colls/coll/docs").unwrap(),
            method: Method::Get,
            headers,
            body: None,
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        };
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db/colls/coll/docs",
        );
        let wrapped = wrap_request_for_gateway_v2(
            request,
            &WrapInputs {
                auth_context: &auth_context,
                operation_type: OperationType::ReadFeed,
                resource_type: ResourceType::Document,
                effective_partition_key: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                read_consistency_strategy: ReadConsistencyStrategy::Default,
                account_name: Some("account"),
                collection_rid: None,
            },
        )
        .unwrap();

        let mut request = Request::new(
            thin_url.join("dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        request.set_body(
            wrapped
                .body
                .expect("Gateway V2 request must carry an RNTBD frame"),
        );
        let response = emulator
            .execute_gateway_v2_request(&request)
            .await
            .unwrap()
            .try_into_raw_response()
            .await
            .unwrap();
        let response = RntbdResponse::read(response.body().as_ref()).unwrap();
        let body: serde_json::Value = serde_json::from_slice(&response.body).unwrap();
        let item = &body["Documents"].as_array().unwrap()[0];
        assert_eq!(item["id"], "item-1");
        assert!(item.get("current").is_none());
        assert!(item.get("metadata").is_none());
    }

    async fn execute_frame(
        emulator: &InMemoryEmulatorHttpClient,
        thin_url: &Url,
        frame: RntbdRequestFrame,
    ) -> RntbdResponse {
        let mut bytes = Vec::new();
        frame.write(&mut bytes).unwrap();
        let mut request = Request::new(
            thin_url.join("dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        request.set_body(bytes);
        let response = emulator
            .execute_gateway_v2_request(&request)
            .await
            .unwrap()
            .try_into_raw_response()
            .await
            .unwrap();
        RntbdResponse::read(response.body().as_ref()).unwrap()
    }

    #[tokio::test]
    async fn service_not_found_uses_matching_outer_and_inner_status() {
        let gateway_v2_url = Url::parse("http://127.0.0.1:18444/").unwrap();
        let region = VirtualRegion::new("East US", "http://127.0.0.1:18081/".parse().unwrap())
            .with_gateway_v2_url(gateway_v2_url.clone());
        let emulator =
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![region]).unwrap());
        emulator.store().create_database("db");
        let partition_key: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        emulator
            .store()
            .create_container("db", "coll", partition_key);
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::document_name("missing".to_owned()),
                Token::partition_key(r#"["pk1"]"#.to_owned()),
                Token::payload_present(false),
            ],
            body: None,
        };
        let mut bytes = Vec::new();
        frame.write(&mut bytes).unwrap();
        let mut request = Request::new(
            gateway_v2_url
                .join("dbs/db/colls/coll/docs/missing")
                .unwrap(),
            Method::Post,
        );
        request.set_body(bytes);

        let response = emulator
            .execute_gateway_v2_request(&request)
            .await
            .unwrap()
            .try_into_raw_response()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NotFound);
        let framed = RntbdResponse::read(response.body().as_ref()).unwrap();
        assert_eq!(framed.status.status_code(), StatusCode::NotFound);
    }

    #[tokio::test]
    async fn semantic_validation_error_is_framed_with_matching_status() {
        let gateway_v2_url = Url::parse("http://127.0.0.1:18444/").unwrap();
        let region = VirtualRegion::new("East US", "http://127.0.0.1:18081/".parse().unwrap())
            .with_gateway_v2_url(gateway_v2_url.clone());
        let emulator =
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![region]).unwrap());
        let activity_id = Uuid::new_v4();
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Create,
            activity_id,
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::allow_tentative_writes(true),
                Token::payload_present(true),
            ],
            body: Some(br#"{"id":"item"}"#.to_vec()),
        };
        let mut bytes = Vec::new();
        frame.write(&mut bytes).unwrap();
        let mut request = Request::new(
            gateway_v2_url.join("dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        request.set_body(bytes);

        let response = emulator
            .execute_gateway_v2_request(&request)
            .await
            .unwrap()
            .try_into_raw_response()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BadRequest);
        let framed = RntbdResponse::read(response.body().as_ref()).unwrap();
        assert_eq!(framed.status.status_code(), StatusCode::BadRequest);
        assert_eq!(framed.activity_id, activity_id);
    }

    #[tokio::test]
    async fn malformed_frame_bytes_are_framed_with_matching_status() {
        let gateway_v2_url = Url::parse("http://127.0.0.1:18444/").unwrap();
        let region = VirtualRegion::new("East US", "http://127.0.0.1:18081/".parse().unwrap())
            .with_gateway_v2_url(gateway_v2_url.clone());
        let emulator =
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![region]).unwrap());
        let mut request = Request::new(
            gateway_v2_url.join("dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        // Not a valid RNTBD frame at all: too short to even contain a header
        // length prefix, let alone resource/operation type and activity id.
        // A generic h2c client that sends garbage bytes must still get back
        // a response it can decode as RNTBD, not a bare JSON error body.
        request.set_body(vec![0x01, 0x02, 0x03]);

        let response = emulator
            .execute_gateway_v2_request(&request)
            .await
            .unwrap()
            .try_into_raw_response()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BadRequest);
        let framed = RntbdResponse::read(response.body().as_ref())
            .expect("malformed-frame error response must itself be a well-formed RNTBD frame");
        assert_eq!(framed.status.status_code(), StatusCode::BadRequest);
    }

    #[tokio::test]
    async fn multi_region_gateway_v2_endpoints_each_serve_their_own_region() {
        let east_gateway = Url::parse("http://127.0.0.1:18081/").unwrap();
        let east_gateway_v2 = Url::parse("http://127.0.0.1:18444/").unwrap();
        let west_gateway = Url::parse("http://127.0.0.1:18082/").unwrap();
        let west_gateway_v2 = Url::parse("http://127.0.0.1:18445/").unwrap();
        let east = VirtualRegion::new("East US", east_gateway.clone())
            .with_gateway_v2_url(east_gateway_v2.clone());
        let west = VirtualRegion::new("West US", west_gateway)
            .with_gateway_v2_url(west_gateway_v2.clone());
        let emulator =
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![east, west]).unwrap());
        let store = emulator.store();
        store.create_database("db");
        let partition_key: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        store.create_container_with_config(
            "db",
            "coll",
            partition_key,
            ContainerConfig::new()
                .with_partition_count(1)
                .build()
                .unwrap(),
        );

        // Seed through the write region's standard gateway, then let the
        // write replicate to the second region before reading it back
        // through each region's own Gateway 2.0 endpoint.
        let mut seed = Request::new(
            east_gateway.join("dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        seed.headers_mut().insert(
            "x-ms-documentdb-partitionkey",
            HeaderValue::from_static(r#"["pk1"]"#),
        );
        seed.set_body(
            serde_json::to_vec(&serde_json::json!({ "id": "item1", "pk": "pk1", "value": 42 }))
                .unwrap(),
        );
        assert!(emulator
            .execute_request(&seed)
            .await
            .unwrap()
            .status()
            .is_success());
        store.drain_pending_replications().await;

        for gateway_v2_url in [&east_gateway_v2, &west_gateway_v2] {
            let activity_id = Uuid::new_v4();
            let frame = RntbdRequestFrame {
                resource_type: ResourceType::Document,
                operation_type: OperationType::Read,
                activity_id,
                metadata: vec![
                    Token::database_name("db".to_owned()),
                    Token::collection_name("coll".to_owned()),
                    Token::document_name("item1".to_owned()),
                    Token::partition_key(r#"["pk1"]"#.to_owned()),
                    Token::payload_present(false),
                ],
                body: None,
            };
            let mut bytes = Vec::new();
            frame.write(&mut bytes).unwrap();
            let mut request = Request::new(
                gateway_v2_url.join("dbs/db/colls/coll/docs/item1").unwrap(),
                Method::Post,
            );
            request.set_body(bytes);

            let response = emulator
                .execute_gateway_v2_request(&request)
                .await
                .unwrap()
                .try_into_raw_response()
                .await
                .unwrap();
            assert_eq!(
                response.status(),
                StatusCode::Ok,
                "each region's Gateway 2.0 endpoint ({gateway_v2_url}) must independently serve its own replicated data"
            );
            let framed = RntbdResponse::read(response.body().as_ref()).unwrap();
            assert_eq!(framed.status.status_code(), StatusCode::Ok);
            assert_eq!(framed.activity_id, activity_id);
        }
    }

    #[tokio::test]
    async fn throttled_response_uses_matching_outer_and_inner_status() {
        let mut headers = Headers::new();
        headers.insert("x-ms-substatus", "3200");
        headers.insert("x-ms-retry-after-ms", "25");
        let response = encode_response(
            AsyncRawResponse::from_bytes(StatusCode::TooManyRequests, headers, Vec::new()),
            Uuid::new_v4(),
        )
        .await
        .unwrap()
        .try_into_raw_response()
        .await
        .unwrap();

        assert_eq!(response.status(), StatusCode::TooManyRequests);
        let framed = RntbdResponse::read(response.body().as_ref()).unwrap();
        assert_eq!(framed.status.status_code(), StatusCode::TooManyRequests);
        assert_eq!(framed.status.sub_status().unwrap().value(), 3200);
        assert_eq!(framed.retry_after_ms, Some(25));
    }

    #[test]
    fn filtering_tokens_preserve_text_and_do_not_infer_key_type() {
        let outer = Request::new(
            "http://127.0.0.1:18444/dbs/db/colls/coll/docs"
                .parse()
                .unwrap(),
            Method::Post,
        );
        for key_type in [None, Some(3)] {
            let mut metadata = vec![
                Token::database_name("db".into()),
                Token::collection_name("coll".into()),
                Token::payload_present(false),
                Token::start_epk("10AB".into()),
                Token::end_epk("20CD".into()),
                Token::start_epk_hash(vec![0x10, 0xAB]),
                Token::end_epk_hash(vec![0x20, 0xCD]),
            ];
            if let Some(value) = key_type {
                metadata.push(Token::read_feed_key_type(value));
            }
            let frame = RntbdRequestFrame {
                resource_type: ResourceType::Document,
                operation_type: OperationType::ReadFeed,
                activity_id: Uuid::new_v4(),
                metadata,
                body: None,
            };
            let request = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap();
            assert_eq!(
                request
                    .headers()
                    .get_optional_str(&HeaderName::from_static(request_header_names::START_EPK)),
                Some("10AB")
            );
            assert_eq!(
                request
                    .headers()
                    .get_optional_str(&HeaderName::from_static(request_header_names::END_EPK)),
                Some("20CD")
            );
            assert_eq!(
                request.headers().get_optional_str(&HeaderName::from_static(
                    request_header_names::READ_FEED_KEY_TYPE
                )),
                key_type.map(|_| "EffectivePartitionKeyRange")
            );
            if key_type.is_none() {
                assert!(matches!(
                    super::super::dispatch::parse_request(&request).operation,
                    super::super::dispatch::OperationType::InvalidInput(_)
                ));
            }
        }
    }

    #[test]
    fn rejects_malformed_filtering_tokens() {
        for token in [
            Token::new(RntbdRequestToken::StartEpk, TokenValue::Bytes(vec![0xFF])),
            Token::start_epk("not-hex".into()),
            Token::end_epk("ABC".into()),
            Token::read_feed_key_type(0),
            Token::read_feed_key_type(4),
        ] {
            assert!(decode_metadata(vec![token]).is_err());
        }
    }

    #[tokio::test]
    async fn range_filters_apply_to_documents_but_hashes_only_route_partitions() {
        let thin_url: Url = "http://127.0.0.1:18444/".parse().unwrap();
        let gateway_url: Url = "http://127.0.0.1:18081/".parse().unwrap();
        let emulator = InMemoryEmulatorHttpClient::new(
            VirtualAccountConfig::new(vec![VirtualRegion::new("East US", gateway_url.clone())
                .with_gateway_v2_url(thin_url.clone())])
            .unwrap(),
        );
        let store = emulator.store();
        store.create_database("db");
        let definition: PartitionKeyDefinition = "/pk".into();
        store.create_container_with_config(
            "db",
            "coll",
            definition.clone(),
            ContainerConfig::new()
                .with_partition_count(2)
                .build()
                .unwrap(),
        );
        let first_max = store
            .region("East US")
            .unwrap()
            .with_container("db", "coll", |state| {
                state.physical_partitions[0].epk_max.clone()
            })
            .unwrap();
        let mut first_partition = Vec::new();
        for index in 0..32 {
            let pk = format!("range-pk-{index:02}");
            let id = format!("doc-{index:02}");
            let epk = FeedRange::for_partition(PartitionKey::from(pk.clone()), &definition)
                .min_inclusive()
                .clone();
            let mut seed = Request::new(
                gateway_url.join("dbs/db/colls/coll/docs").unwrap(),
                Method::Post,
            );
            seed.headers_mut().insert(
                request_header_names::PARTITION_KEY,
                serde_json::to_string(&[&pk]).unwrap(),
            );
            seed.set_body(serde_json::to_vec(&serde_json::json!({"id":id,"pk":pk})).unwrap());
            assert_eq!(
                emulator.execute_request(&seed).await.unwrap().status(),
                StatusCode::Created
            );
            if epk < first_max {
                first_partition.push((epk, id));
            }
        }
        first_partition.sort();
        assert!(first_partition.len() > 6 && first_partition.len() < 32);
        for (operation, change_feed) in [
            (OperationType::Query, false),
            (OperationType::ReadFeed, false),
            (OperationType::ReadFeed, true),
        ] {
            for open_min in [false, true] {
                for with_filters in [true, false] {
                    let start = if open_min {
                        EffectivePartitionKey::MIN.clone()
                    } else {
                        first_partition[2].0.clone()
                    };
                    let end = first_partition[6].0.clone();
                    let mut continuation: Option<String> = None;
                    let mut ids = Vec::new();
                    let mut page_count = 0;
                    loop {
                        page_count += 1;
                        assert!(page_count <= 32, "pagination must terminate");
                        let mut headers = Headers::new();
                        headers.insert("authorization", "auth-token");
                        headers.insert("x-ms-date", "Wed, 21 Oct 2015 07:28:00 GMT");
                        headers.insert("x-ms-activity-id", Uuid::new_v4().to_string());
                        headers.insert(request_header_names::START_EPK, start.to_hex());
                        headers.insert(request_header_names::END_EPK, end.to_hex());
                        headers.insert(
                            request_header_names::READ_FEED_KEY_TYPE,
                            "EffectivePartitionKeyRange",
                        );
                        headers.insert(request_header_names::MAX_ITEM_COUNT, "2");
                        let previous = continuation.take();
                        if change_feed {
                            headers.insert(request_header_names::A_IM, "Incremental Feed");
                        }
                        if let Some(value) = previous.as_ref() {
                            headers.insert(
                                if change_feed {
                                    request_header_names::IF_NONE_MATCH
                                } else {
                                    request_header_names::CONTINUATION
                                },
                                value.clone(),
                            );
                        }
                        let body = (operation == OperationType::Query)
                            .then(|| Bytes::from_static(br#"{"query":"SELECT * FROM c"}"#));
                        let request = HttpRequest {
                            url: thin_url.join("dbs/db/colls/coll/docs").unwrap(),
                            method: Method::Post,
                            headers,
                            body,
                            timeout: None,
                            #[cfg(feature = "fault_injection")]
                            evaluation_collector: None,
                        };
                        let auth_context = AuthorizationContext::new(
                            Method::Post,
                            ResourceType::Document,
                            "dbs/db/colls/coll/docs",
                        );
                        let wrapped = wrap_request_for_gateway_v2(
                            request,
                            &WrapInputs {
                                auth_context: &auth_context,
                                operation_type: operation,
                                resource_type: ResourceType::Document,
                                effective_partition_key: None,
                                effective_consistency: DefaultConsistencyLevel::Session,
                                read_consistency_strategy: ReadConsistencyStrategy::Default,
                                account_name: Some("account"),
                                collection_rid: None,
                            },
                        )
                        .unwrap();
                        let mut frame =
                            RntbdRequestFrame::read(wrapped.body.as_ref().unwrap()).unwrap();
                        if change_feed {
                            assert!(!frame
                                .metadata
                                .iter()
                                .any(|token| token.id.value() == 0x0006));
                            assert_eq!(
                                frame
                                    .metadata
                                    .iter()
                                    .find(|token| token.id.value() == 0x0008)
                                    .map(|token| &token.value),
                                previous
                                    .as_ref()
                                    .map(|value| TokenValue::String(value.clone()))
                                    .as_ref(),
                            );
                        }
                        if !with_filters {
                            frame
                                .metadata
                                .retain(|token| !matches!(token.id.value(), 0x0062..=0x0064));
                        }
                        let response = execute_frame(&emulator, &thin_url, frame).await;
                        if change_feed && response.status.status_code() == StatusCode::NotModified {
                            assert!(previous.is_some());
                            assert_eq!(response.etag, previous);
                            assert!(response.continuation_token.is_none());
                            assert!(response.body.is_empty());
                            break;
                        }
                        assert_eq!(response.status.status_code(), StatusCode::Ok);
                        let body: serde_json::Value =
                            serde_json::from_slice(&response.body).unwrap();
                        ids.extend(
                            body["Documents"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| item["id"].as_str().unwrap().to_owned()),
                        );
                        continuation = if change_feed {
                            assert!(response.continuation_token.is_none());
                            let etag = response.etag.expect("change-feed pages require ETag");
                            assert_ne!(previous.as_ref(), Some(&etag));
                            Some(etag)
                        } else {
                            response.continuation_token
                        };
                        if continuation.is_none() {
                            break;
                        }
                    }
                    let mut expected: Vec<_> = first_partition
                        .iter()
                        .filter(|(epk, _)| !with_filters || (*epk >= start && *epk < end))
                        .map(|(_, id)| id.clone())
                        .collect();
                    ids.sort();
                    expected.sort();
                    assert_eq!(
                        ids, expected,
                        "{operation:?}, change_feed={change_feed}, open_min={open_min}, filters={with_filters}"
                    );
                    assert!(page_count > 1);
                }
            }
        }
    }

    #[test]
    fn effective_partition_key_does_not_repair_invalid_filter_key_type() {
        for key_type in [None, Some(2)] {
            let mut metadata = vec![
                Token::database_name("db".into()),
                Token::collection_name("coll".into()),
                Token::payload_present(false),
                Token::effective_partition_key(vec![0x10]),
                Token::end_epk("FF".into()),
            ];
            if let Some(key_type) = key_type {
                metadata.push(Token::read_feed_key_type(key_type));
            }
            let frame = RntbdRequestFrame {
                resource_type: ResourceType::Document,
                operation_type: OperationType::ReadFeed,
                activity_id: Uuid::new_v4(),
                metadata,
                body: None,
            };
            let outer = Request::new(
                "http://127.0.0.1:18444/dbs/db/colls/coll/docs"
                    .parse()
                    .unwrap(),
                Method::Post,
            );
            let decoded = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap();
            assert!(matches!(
                super::super::dispatch::parse_request(&decoded).operation,
                super::super::dispatch::OperationType::InvalidInput(_)
            ));
        }
    }

    #[tokio::test]
    async fn effective_partition_scope_intersects_explicit_filters() {
        for hierarchical in [false, true] {
            let thin_url: Url = "http://127.0.0.1:18444/".parse().unwrap();
            let gateway_url: Url = "http://127.0.0.1:18081/".parse().unwrap();
            let emulator = InMemoryEmulatorHttpClient::new(
                VirtualAccountConfig::new(vec![VirtualRegion::new("East US", gateway_url.clone())
                    .with_gateway_v2_url(thin_url.clone())])
                .unwrap(),
            );
            let store = emulator.store();
            store.create_database("db");
            let definition: PartitionKeyDefinition = if hierarchical {
                serde_json::from_value(serde_json::json!({
                    "paths":["/tenant", "/pk"], "kind":"MultiHash", "version":2
                }))
                .unwrap()
            } else {
                "/pk".into()
            };
            store.create_container_with_config(
                "db",
                "coll",
                definition.clone(),
                ContainerConfig::new()
                    .with_partition_count(2)
                    .build()
                    .unwrap(),
            );
            let first_max = store
                .region("East US")
                .unwrap()
                .with_container("db", "coll", |state| {
                    state.physical_partitions[0].epk_max.clone()
                })
                .unwrap();
            let mut rows = Vec::new();
            for index in 0..32 {
                let pk = format!("pk-{index:02}");
                let tenant = format!("tenant-{:02}", index / 4);
                let components = if hierarchical {
                    vec![tenant.clone(), pk.clone()]
                } else {
                    vec![pk.clone()]
                };
                let key = PartitionKey::try_from(
                    components
                        .iter()
                        .cloned()
                        .map(PartitionKeyValue::from)
                        .collect::<Vec<_>>(),
                )
                .unwrap();
                let epk = FeedRange::for_partition(key, &definition)
                    .min_inclusive()
                    .clone();
                let id = format!("doc-{index:02}");
                let mut seed = Request::new(
                    gateway_url.join("dbs/db/colls/coll/docs").unwrap(),
                    Method::Post,
                );
                seed.headers_mut().insert(
                    request_header_names::PARTITION_KEY,
                    serde_json::to_string(&components).unwrap(),
                );
                seed.set_body(
                    serde_json::to_vec(&serde_json::json!({"id":id, "pk":pk, "tenant":tenant}))
                        .unwrap(),
                );
                assert_eq!(
                    emulator.execute_request(&seed).await.unwrap().status(),
                    StatusCode::Created
                );
                rows.push((epk, id, if hierarchical { tenant } else { pk }));
            }
            rows.sort();
            assert!(rows[0].0 < first_max);
            assert!(
                rows.last().unwrap().0 >= first_max,
                "seed both physical partitions"
            );
            let target = &rows[0].2;
            let scope_key = PartitionKey::from(target.clone());
            let scope = FeedRange::for_partition(scope_key, &definition);
            let scoped: Vec<_> = rows.iter().filter(|row| &row.2 == target).collect();
            assert_eq!(scoped.len(), if hierarchical { 4 } else { 1 });
            let upper = scoped.last().unwrap().0.clone();
            for operation in [OperationType::Query, OperationType::ReadFeed] {
                for with_partition_key in [false, true] {
                    for (start, end) in [
                        (
                            EffectivePartitionKey::MIN.clone(),
                            EffectivePartitionKey::MAX.clone(),
                        ),
                        (rows[0].0.clone(), upper.clone()),
                        (first_max.clone(), EffectivePartitionKey::MAX.clone()),
                    ] {
                        let mut metadata = vec![
                            Token::database_name("db".into()),
                            Token::collection_name("coll".into()),
                            Token::effective_partition_key(
                                scope.min_inclusive().as_bytes().to_vec(),
                            ),
                            Token::start_epk(start.to_hex()),
                            Token::end_epk(end.to_hex()),
                            Token::read_feed_key_type(3),
                            Token::payload_present(operation == OperationType::Query),
                        ];
                        if with_partition_key {
                            metadata.push(Token::partition_key(
                                serde_json::to_string(&[target]).unwrap(),
                            ));
                        }
                        let response = execute_frame(
                            &emulator,
                            &thin_url,
                            RntbdRequestFrame {
                                resource_type: ResourceType::Document,
                                operation_type: operation,
                                activity_id: Uuid::new_v4(),
                                metadata,
                                body: (operation == OperationType::Query)
                                    .then(|| br#"{"query":"SELECT * FROM c"}"#.to_vec()),
                            },
                        )
                        .await;
                        assert_eq!(response.status.status_code(), StatusCode::Ok);
                        let body: serde_json::Value =
                            serde_json::from_slice(&response.body).unwrap();
                        let mut actual: Vec<_> = body["Documents"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|doc| doc["id"].as_str().unwrap().to_owned())
                            .collect();
                        let mut expected: Vec<_> = scoped
                            .iter()
                            .filter(|row| row.0 >= start && row.0 < end)
                            .map(|row| row.1.clone())
                            .collect();
                        actual.sort();
                        expected.sort();
                        assert_eq!(actual, expected,
                            "{operation:?}, hierarchical={hierarchical}, with_partition_key={with_partition_key}");
                    }
                }
            }
        }
    }

    #[test]
    fn effective_partition_key_remains_independent_of_filter_headers() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Query,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::effective_partition_key(vec![0x10, 0x20]),
                Token::payload_present(true),
            ],
            body: Some(br#"{"query":"SELECT * FROM c"}"#.to_vec()),
        };
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );

        let request = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap();
        assert_eq!(
            request.headers().get_optional_str(&EFFECTIVE_PARTITION_KEY),
            Some("1020")
        );
        for name in [
            request_header_names::START_EPK,
            request_header_names::END_EPK,
            request_header_names::READ_FEED_KEY_TYPE,
        ] {
            assert!(request
                .headers()
                .get_optional_str(&HeaderName::from_static(name))
                .is_none());
        }
    }

    #[test]
    fn read_consistency_strategy_is_forwarded_to_the_emulator_core() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::document_name("item1".to_owned()),
                Token::partition_key(r#"["pk1"]"#.to_owned()),
                Token::read_consistency_strategy(ReadConsistencyStrategy::LatestCommitted),
                Token::payload_present(false),
            ],
            body: None,
        };
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs/item1").unwrap(),
            Method::Post,
        );

        let request = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap();
        assert_eq!(
            request.headers().get_optional_str(&HeaderName::from_static(
                "x-ms-cosmos-read-consistency-strategy"
            )),
            Some("LatestCommitted")
        );
    }

    #[test]
    fn binary_negotiation_re_emits_serialization_formats_header() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::document_name("item1".to_owned()),
                Token::partition_key(r#"["pk1"]"#.to_owned()),
                Token::new(
                    RntbdRequestToken::SupportedSerializationFormats,
                    TokenValue::Byte(0x03),
                ),
                Token::payload_present(false),
            ],
            body: None,
        };
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs/item1").unwrap(),
            Method::Post,
        );

        let request = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap();
        assert_eq!(
            request.headers().get_optional_str(&HeaderName::from_static(
                "x-ms-cosmos-supported-serialization-formats"
            )),
            Some("JsonText,CosmosBinary")
        );
    }

    #[test]
    fn ordinary_read_feed_does_not_gain_change_feed_header() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::ReadFeed,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::payload_present(false),
            ],
            body: None,
        };
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );

        let request = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap();
        assert_eq!(
            request
                .headers()
                .get_optional_str(&HeaderName::from_static("a-im")),
            None
        );
    }

    #[test]
    fn incremental_read_feed_forwards_a_im_token() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::ReadFeed,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::a_im("Incremental Feed".to_owned()),
                Token::change_feed_wire_format_version("2021-09-15".to_owned()),
                Token::payload_present(false),
            ],
            body: None,
        };
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );

        let request = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap();
        assert_eq!(
            request
                .headers()
                .get_optional_str(&HeaderName::from_static("a-im")),
            Some("Incremental Feed")
        );
        assert_eq!(
            request.headers().get_optional_str(&HeaderName::from_static(
                "x-ms-cosmos-changefeed-wire-format-version"
            )),
            Some("2021-09-15")
        );
    }

    #[test]
    fn unknown_read_consistency_strategy_is_rejected() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::document_name("item1".to_owned()),
                Token::partition_key(r#"["pk1"]"#.to_owned()),
                Token::new(
                    RntbdRequestToken::ReadConsistencyStrategy,
                    TokenValue::Byte(0xFF),
                ),
                Token::payload_present(false),
            ],
            body: None,
        };
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs/item1").unwrap(),
            Method::Post,
        );

        let error = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap_err();
        assert!(error.to_string().contains("ReadConsistencyStrategy value"));
    }

    #[test]
    fn tentative_writes_are_rejected_explicitly() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Create,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::allow_tentative_writes(true),
                Token::payload_present(true),
            ],
            body: Some(br#"{"id":"item"}"#.to_vec()),
        };
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );

        let error = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap_err();
        assert!(error.to_string().contains("AllowTentativeWrites"));
    }

    #[test]
    fn rejects_wrong_types_for_known_tokens() {
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        for token in [
            Token::new(RntbdRequestToken::Match, TokenValue::ULong(1)),
            Token::new(
                RntbdRequestToken::PageSize,
                TokenValue::String("1".to_owned()),
            ),
            Token::new(
                RntbdRequestToken::EffectivePartitionKey,
                TokenValue::String("01".to_owned()),
            ),
            Token::new(RntbdRequestToken::StartEpkHash, TokenValue::Byte(1)),
            Token::new(RntbdRequestToken::EndEpkHash, TokenValue::Byte(1)),
            Token::new(RntbdRequestToken::StartEpk, TokenValue::String("10".into())),
            Token::new(RntbdRequestToken::EndEpk, TokenValue::Byte(1)),
            Token::new(RntbdRequestToken::ReadFeedKeyType, TokenValue::ULong(3)),
        ] {
            let frame = RntbdRequestFrame {
                resource_type: ResourceType::Document,
                operation_type: OperationType::Query,
                activity_id: Uuid::new_v4(),
                metadata: vec![
                    Token::database_name("db".to_owned()),
                    Token::collection_name("coll".to_owned()),
                    Token::payload_present(true),
                    token,
                ],
                body: Some(br#"{"query":"SELECT * FROM c"}"#.to_vec()),
            };
            let error = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap_err();
            assert!(error.to_string().contains("must use"));
        }
    }

    #[test]
    fn rejects_payload_present_mismatches() {
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        for (payload_present, body) in [(true, None), (false, Some(Vec::new()))] {
            let frame = RntbdRequestFrame {
                resource_type: ResourceType::Document,
                operation_type: OperationType::ReadFeed,
                activity_id: Uuid::new_v4(),
                metadata: vec![
                    Token::database_name("db".to_owned()),
                    Token::collection_name("coll".to_owned()),
                    Token::payload_present(payload_present),
                ],
                body,
            };
            let error = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap_err();
            assert!(error.to_string().contains("PayloadPresent"));
        }
    }

    #[test]
    fn rejects_outer_path_that_disagrees_with_frame_target() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::ReadFeed,
            activity_id: Uuid::new_v4(),
            metadata: vec![
                Token::database_name("db".to_owned()),
                Token::collection_name("coll".to_owned()),
                Token::payload_present(false),
            ],
            body: None,
        };
        let outer = Request::new(
            Url::parse("http://127.0.0.1:18444/wrong").unwrap(),
            Method::Post,
        );

        let error = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap_err();
        assert!(error.to_string().contains("does not match RNTBD target"));
    }
}
