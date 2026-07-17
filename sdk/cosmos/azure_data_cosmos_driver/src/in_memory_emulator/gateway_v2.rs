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

use crate::models::effective_partition_key::prefix_range_end_hex;
use crate::{
    driver::transport::rntbd::{
        tokens::{RntbdRequestToken, TokenValue},
        RntbdRequestFrame, RntbdResponse,
    },
    models::{CosmosStatus, OperationType, ResourceType},
};

use super::{ConsistencyLevel, InMemoryEmulatorHttpClient};

impl InMemoryEmulatorHttpClient {
    /// Executes a hosted Gateway V2 request and returns an RNTBD-framed response.
    #[doc(hidden)]
    pub async fn execute_gateway_v2_request(
        &self,
        request: &Request,
    ) -> crate::error::Result<AsyncRawResponse> {
        let request_body: Bytes = request.body().into();
        // A frame that fails to parse has no usable `activityId` field, so
        // there is nothing to echo back — but the response must still be a
        // well-formed RNTBD frame (not a bare JSON body) so that any client
        // speaking Gateway 2.0, not just the Rust driver, can decode the
        // failure the same way it decodes every other error on this path.
        let frame = match RntbdRequestFrame::read(request_body.as_ref()) {
            Ok(frame) => frame,
            Err(error) => {
                return encode_error_response(gateway_v2_bad_request(error), Uuid::new_v4()).await
            }
        };
        let activity_id = frame.activity_id;
        let request = match decode_request(request, frame, self.store().config().consistency()) {
            Ok(request) => request,
            Err(error) => return encode_error_response(error, activity_id).await,
        };
        let response = self.execute_request(&request).await?;
        encode_response(response, activity_id).await
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
    effective_partition_key: Option<String>,
    start_epk: Option<String>,
    end_epk: Option<String>,
    page_size: Option<u32>,
    return_minimal: bool,
    supported_query_features: Option<String>,
    query_version: Option<String>,
    allow_tentative_writes: bool,
    consistency_level: Option<u8>,
    read_consistency_strategy: Option<u8>,
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
    let mut metadata = decode_metadata(frame.metadata)?;
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
    if metadata.read_consistency_strategy.is_some() {
        return Err(gateway_v2_bad_request(
            "hosted Gateway V2 does not yet support non-default ReadConsistencyStrategy",
        ));
    }
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
        OperationType::Query | OperationType::SqlQuery | OperationType::ReadFeed
    ) && metadata.start_epk.is_none()
    {
        if let Some(effective_partition_key) = metadata.effective_partition_key.as_ref() {
            metadata.start_epk = Some(effective_partition_key.clone());
            let bytes = effective_partition_key
                .as_bytes()
                .chunks_exact(2)
                .map(|pair| {
                    let value = std::str::from_utf8(pair).map_err(gateway_v2_bad_request)?;
                    u8::from_str_radix(value, 16).map_err(gateway_v2_bad_request)
                })
                .collect::<crate::error::Result<Vec<_>>>()?;
            metadata.end_epk = Some(prefix_range_end_hex(&bytes));
        }
    }
    if matches!(
        frame.operation_type,
        OperationType::Read | OperationType::Replace | OperationType::Delete
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
        OperationType::Delete => (Method::Delete, true),
        OperationType::Query
        | OperationType::SqlQuery
        | OperationType::QueryPlan
        | OperationType::Batch => (Method::Post, false),
        OperationType::ReadFeed => (Method::Get, false),
        operation => {
            return Err(gateway_v2_bad_request(format!(
                "unsupported hosted Gateway V2 operation {operation:?}"
            )))
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
    if let Some(value) = metadata.continuation {
        request.headers_mut().insert("x-ms-continuation", value);
    }
    if let Some(value) = metadata.session_token {
        request.headers_mut().insert("x-ms-session-token", value);
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
    if let Some(value) = metadata.start_epk {
        request.headers_mut().insert("x-ms-start-epk", value);
        request
            .headers_mut()
            .insert("x-ms-read-key-type", "EffectivePartitionKeyRange");
    }
    if let Some(value) = metadata.end_epk {
        request.headers_mut().insert("x-ms-end-epk", value);
        request
            .headers_mut()
            .insert("x-ms-read-key-type", "EffectivePartitionKeyRange");
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
        OperationType::Batch => request
            .headers_mut()
            .insert("x-ms-cosmos-is-batch-request", "true"),
        _ => {}
    }
    if let Some(body) = frame.body {
        request.set_body(body);
    }
    Ok(request)
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
                metadata.database = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::CollectionName => {
                metadata.collection = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::DocumentName => {
                metadata.document = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::PartitionKey => {
                metadata.partition_key = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::PartitionKeyRangeId => {
                metadata.partition_key_range_id = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::ContinuationToken => {
                metadata.continuation = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::SessionToken => {
                metadata.session_token = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::Match => {
                metadata.match_condition = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::StartEpkHash => {
                metadata.start_epk = Some(expect_hex(kind, token.value)?)
            }
            RntbdRequestToken::EndEpkHash => {
                metadata.end_epk = Some(expect_hex(kind, token.value)?)
            }
            RntbdRequestToken::EffectivePartitionKey => {
                metadata.effective_partition_key = Some(expect_hex(kind, token.value)?)
            }
            RntbdRequestToken::PageSize => {
                metadata.page_size = Some(expect_ulong(kind, token.value)?);
            }
            RntbdRequestToken::ReturnPreference => {
                metadata.return_minimal = expect_byte(kind, token.value)? != 0
            }
            RntbdRequestToken::SupportedQueryFeatures => {
                metadata.supported_query_features = Some(expect_string(kind, token.value)?)
            }
            RntbdRequestToken::QueryVersion => {
                metadata.query_version = Some(expect_small_string(kind, token.value)?)
            }
            RntbdRequestToken::AllowTentativeWrites => {
                metadata.allow_tentative_writes = expect_byte(kind, token.value)? != 0
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
    let response = response
        .try_into_raw_response()
        .await
        .map_err(gateway_v2_internal_error)?;
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
        lsn: header_i64(headers, "lsn"),
        request_charge: header_f64(headers, "x-ms-request-charge"),
        owner_full_name: header_string(headers, "x-ms-alt-content-path"),
        partition_key_range_id: header_string(headers, "x-ms-documentdb-partitionkeyrangeid"),
        item_lsn: header_i64(headers, "x-ms-item-lsn"),
        global_committed_lsn: header_i64(headers, "x-ms-global-committed-lsn"),
        transport_request_id: header_u32(headers, "x-ms-transport-request-id"),
        session_token: header_string(headers, "x-ms-session-token"),
        item_count: header_u32(headers, "x-ms-item-count"),
        query_metrics: header_string(headers, "x-ms-documentdb-query-metrics"),
        index_utilization: header_string(headers, "x-ms-cosmos-index-utilization"),
        request_duration_ms: header_f64(headers, "x-ms-request-duration-ms"),
    };
    let mut body = Vec::new();
    rntbd.write(&mut body).map_err(gateway_v2_internal_error)?;
    let mut outer_headers = Headers::new();
    outer_headers.insert("content-type", "application/octet-stream");
    Ok(AsyncRawResponse::from_bytes(
        status.status_code(),
        outer_headers,
        body,
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
        driver::transport::rntbd::Token,
        in_memory_emulator::{ContainerConfig, VirtualAccountConfig, VirtualRegion},
        models::PartitionKeyDefinition,
    };
    use url::Url;

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
    fn effective_partition_key_scopes_query_range() {
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
            request
                .headers()
                .get_optional_str(&HeaderName::from_static("x-ms-start-epk")),
            Some("1020")
        );
        assert_eq!(
            request
                .headers()
                .get_optional_str(&HeaderName::from_static("x-ms-end-epk")),
            Some("1020FF")
        );
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
