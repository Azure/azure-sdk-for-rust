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
        let frame =
            RntbdRequestFrame::read(request_body.as_ref()).map_err(gateway_v2_bad_request)?;
        let activity_id = frame.activity_id;
        let request = decode_request(request, frame, self.store().config().consistency())?;
        let response = self.execute_request(&request).await?;
        encode_response(response, activity_id).await
    }
}

#[derive(Default)]
struct RequestMetadata {
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
    let mut metadata = decode_metadata(frame.metadata);
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
            metadata.end_epk = Some(format!("{effective_partition_key}FF"));
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

fn decode_metadata(tokens: Vec<crate::driver::transport::rntbd::Token>) -> RequestMetadata {
    let mut metadata = RequestMetadata::default();
    for token in tokens {
        let Ok(kind) = RntbdRequestToken::try_from(token.id.value()) else {
            continue;
        };
        match kind {
            RntbdRequestToken::DatabaseName => metadata.database = token_string(token.value),
            RntbdRequestToken::CollectionName => metadata.collection = token_string(token.value),
            RntbdRequestToken::DocumentName => metadata.document = token_string(token.value),
            RntbdRequestToken::PartitionKey => metadata.partition_key = token_string(token.value),
            RntbdRequestToken::PartitionKeyRangeId => {
                metadata.partition_key_range_id = token_string(token.value)
            }
            RntbdRequestToken::ContinuationToken => {
                metadata.continuation = token_string(token.value)
            }
            RntbdRequestToken::SessionToken => metadata.session_token = token_string(token.value),
            RntbdRequestToken::Match => metadata.match_condition = token_string(token.value),
            RntbdRequestToken::StartEpkHash => metadata.start_epk = token_hex(token.value),
            RntbdRequestToken::EndEpkHash => metadata.end_epk = token_hex(token.value),
            RntbdRequestToken::EffectivePartitionKey => {
                metadata.effective_partition_key = token_hex(token.value)
            }
            RntbdRequestToken::PageSize => {
                if let TokenValue::ULong(value) = token.value {
                    metadata.page_size = Some(value);
                }
            }
            RntbdRequestToken::ReturnPreference => {
                metadata.return_minimal =
                    matches!(token.value, TokenValue::Byte(value) if value != 0)
            }
            RntbdRequestToken::SupportedQueryFeatures => {
                metadata.supported_query_features = token_string(token.value)
            }
            RntbdRequestToken::QueryVersion => metadata.query_version = token_string(token.value),
            RntbdRequestToken::AllowTentativeWrites => {
                metadata.allow_tentative_writes =
                    matches!(token.value, TokenValue::Byte(value) if value != 0)
            }
            RntbdRequestToken::ConsistencyLevel => {
                if let TokenValue::Byte(value) = token.value {
                    metadata.consistency_level = Some(value);
                }
            }
            RntbdRequestToken::ReadConsistencyStrategy => {
                if let TokenValue::Byte(value) = token.value {
                    metadata.read_consistency_strategy = Some(value);
                }
            }
            RntbdRequestToken::ResourceId
            | RntbdRequestToken::AuthorizationToken
            | RntbdRequestToken::PayloadPresent
            | RntbdRequestToken::Date
            | RntbdRequestToken::CollectionRid
            | RntbdRequestToken::TransportRequestId
            | RntbdRequestToken::SDKSupportedCapabilities
            | RntbdRequestToken::GlobalDatabaseAccountName => {
                // The hosted adapter already has the corresponding routing or
                // transport context; these tokens do not alter store semantics.
            }
        }
    }
    metadata
}

fn token_string(value: TokenValue) -> Option<String> {
    match value {
        TokenValue::SmallString(value)
        | TokenValue::String(value)
        | TokenValue::ULongString(value) => Some(value),
        _ => None,
    }
}

fn token_hex(value: TokenValue) -> Option<String> {
    let bytes = match value {
        TokenValue::SmallBytes(value)
        | TokenValue::Bytes(value)
        | TokenValue::ULongBytes(value) => value,
        _ => return None,
    };
    Some(bytes.iter().map(|byte| format!("{byte:02X}")).collect())
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
    };
    let mut body = Vec::new();
    rntbd.write(&mut body).map_err(gateway_v2_internal_error)?;
    let mut outer_headers = Headers::new();
    outer_headers.insert("content-type", "application/octet-stream");
    for name in [
        "x-ms-request-duration-ms",
        "lsn",
        "x-ms-item-lsn",
        "x-ms-global-committed-lsn",
        "x-ms-documentdb-query-metrics",
        "x-ms-index-utilization",
    ] {
        if let Some(value) = header_string(headers, name) {
            outer_headers.insert(name, value);
        }
    }
    Ok(AsyncRawResponse::from_bytes(
        StatusCode::Ok,
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
            .with_thin_client_url(thin_url.clone());
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
        let mut request = Request::new(thin_url, Method::Post);
        request.set_body(bytes);

        let response = emulator
            .execute_gateway_v2_request(&request)
            .await
            .unwrap()
            .try_into_raw_response()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);
        let response = RntbdResponse::read(response.body().as_ref()).unwrap();
        assert_eq!(response.status.status_code(), StatusCode::Created);
        assert_eq!(response.activity_id, activity_id);
        assert!(!response.body.is_empty());
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
            ],
            body: Some(br#"{"query":"SELECT * FROM c"}"#.to_vec()),
        };
        let outer = Request::new(Url::parse("http://127.0.0.1:18444/").unwrap(), Method::Post);

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
            ],
            body: Some(br#"{"id":"item"}"#.to_vec()),
        };
        let outer = Request::new(Url::parse("http://127.0.0.1:18444/").unwrap(), Method::Post);

        let error = decode_request(&outer, frame, ConsistencyLevel::Session).unwrap_err();
        assert!(error.to_string().contains("AllowTentativeWrites"));
    }
}
