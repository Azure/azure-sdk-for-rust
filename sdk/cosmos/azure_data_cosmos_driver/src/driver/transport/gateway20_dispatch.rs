// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway 2.0 HTTP dispatch helpers.

use std::sync::atomic::{AtomicU32, Ordering};

use azure_core::{
    error::ErrorKind,
    http::{
        headers::{HeaderName, HeaderValue, Headers, AUTHORIZATION, USER_AGENT},
        Method,
    },
};
use uuid::Uuid;

use crate::{
    constants::{GATEWAY20_RANGE_MAX, GATEWAY20_RANGE_MIN},
    models::{
        cosmos_headers::response_header_names, effective_partition_key::EffectivePartitionKey,
        DefaultConsistencyLevel, OperationType, PartitionKey, PartitionKeyDefinition, ResourceType,
    },
};

use super::{
    cosmos_headers::SUPPORTED_CAPABILITIES_BITS,
    cosmos_transport_client::{HttpRequest, HttpResponse},
    rntbd::{RntbdRequestFrame, RntbdResponse, Token},
    AuthorizationContext,
};

const X_MS_ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
const X_MS_DATE: HeaderName = HeaderName::from_static("x-ms-date");
const X_MS_LSN: HeaderName = HeaderName::from_static("x-ms-lsn");
const X_MS_GLOBAL_COMMITTED_LSN: HeaderName = HeaderName::from_static("x-ms-global-committed-lsn");
const X_MS_CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
static TRANSPORT_REQUEST_ID: AtomicU32 = AtomicU32::new(0);

/// Inputs resolved by the operation pipeline before a Gateway 2.0 dispatch.
pub(crate) struct WrapInputs<'a> {
    pub(crate) auth_context: &'a AuthorizationContext,
    pub(crate) operation_type: OperationType,
    pub(crate) resource_type: ResourceType,
    pub(crate) partition_key: Option<&'a PartitionKey>,
    pub(crate) partition_key_definition: Option<&'a PartitionKeyDefinition>,
    pub(crate) effective_consistency: DefaultConsistencyLevel,
    pub(crate) account_name: Option<&'a str>,
}

/// Wraps a signed Cosmos HTTP request into a Gateway 2.0 RNTBD request frame.
pub(crate) fn wrap_request_for_gateway20(
    request: &HttpRequest,
    inputs: &WrapInputs<'_>,
) -> azure_core::Result<HttpRequest> {
    let authorization = required_header(request, &AUTHORIZATION, "authorization")?;
    let date = required_header(request, &X_MS_DATE, "x-ms-date")?;
    let activity_id = required_header(request, &X_MS_ACTIVITY_ID, "x-ms-activity-id")?;
    let activity_id = Uuid::parse_str(&activity_id)
        .map_err(|e| data_conversion_error(format!("x-ms-activity-id is not a valid UUID: {e}")))?;
    let account_name = inputs
        .account_name
        .filter(|value| !value.is_empty())
        .ok_or_else(|| data_conversion_error("Gateway 2.0 dispatch requires an account name"))?;

    let resource_names = parse_resource_names(inputs.auth_context.resource_link.as_str())?;
    let has_payload = request.body.as_ref().is_some_and(|body| !body.is_empty());

    let epk_payload = effective_partition_key_payload(inputs)?;

    let mut metadata = Vec::with_capacity(11);
    if let Some(EpkPayload::Point(epk)) = epk_payload.as_ref() {
        metadata.push(Token::effective_partition_key(epk.clone()));
    }
    metadata.push(Token::global_database_account_name(account_name.to_owned()));
    metadata.push(Token::database_name(resource_names.database));
    metadata.push(Token::collection_name(resource_names.collection));
    metadata.push(Token::payload_present(has_payload));
    if inputs.resource_type == ResourceType::Document
        && inputs.operation_type != OperationType::Create
    {
        if let Some(document) = resource_names.document {
            metadata.push(Token::document_name(document));
        }
    }
    metadata.push(Token::authorization_token(authorization));
    metadata.push(Token::date(date));
    metadata.push(Token::consistency_level(inputs.effective_consistency));
    metadata.push(Token::transport_request_id(next_transport_request_id()));
    metadata.push(Token::sdk_supported_capabilities(
        SUPPORTED_CAPABILITIES_BITS,
    ));
    if let Some(continuation) = request.headers.get_optional_str(&X_MS_CONTINUATION) {
        metadata.push(Token::continuation_token(continuation.to_owned()));
    }

    let frame = RntbdRequestFrame {
        resource_type: inputs.resource_type,
        operation_type: inputs.operation_type,
        activity_id,
        metadata,
        body: if has_payload {
            request.body.as_ref().map(|body| body.to_vec())
        } else {
            None
        },
    }
    .serialize()?;

    let mut headers = Headers::new();
    if let Some(user_agent) = request.headers.get_optional_str(&USER_AGENT) {
        headers.insert(USER_AGENT, HeaderValue::from(user_agent.to_owned()));
    }
    headers.insert(X_MS_ACTIVITY_ID, HeaderValue::from(activity_id.to_string()));
    if let Some(EpkPayload::Range { min, max }) = epk_payload.as_ref() {
        headers.insert(GATEWAY20_RANGE_MIN, HeaderValue::from(min.clone()));
        headers.insert(GATEWAY20_RANGE_MAX, HeaderValue::from(max.clone()));
    }

    Ok(HttpRequest {
        url: request.url.clone(),
        method: Method::Post,
        headers,
        body: Some(bytes::Bytes::from(frame)),
        timeout: request.timeout,
        #[cfg(feature = "fault_injection")]
        evaluation_collector: request.evaluation_collector.clone(),
    })
}

/// Decodes a Gateway 2.0 RNTBD response body into a synthetic HTTP response.
pub(crate) fn unwrap_response_for_gateway20(
    response: HttpResponse,
) -> azure_core::Result<HttpResponse> {
    let response = RntbdResponse::deserialize(&response.body)?;
    let status = u16::from(response.status.status_code());
    if !(100..=599).contains(&status) {
        return Err(data_conversion_error(format!(
            "Gateway 2.0 RNTBD response contained invalid HTTP status {status}"
        )));
    }

    let mut headers = Headers::new();
    headers.insert(
        response_header_names::ACTIVITY_ID,
        response.activity_id.to_string(),
    );
    if let Some(charge) = response.request_charge {
        headers.insert(response_header_names::REQUEST_CHARGE, charge.to_string());
    }
    if let Some(token) = response.session_token {
        headers.insert(response_header_names::SESSION_TOKEN, token);
    }
    if let Some(etag) = response.etag {
        headers.insert(response_header_names::ETAG, etag);
    }
    if let Some(continuation) = response.continuation_token {
        headers.insert(response_header_names::CONTINUATION, continuation);
    }
    if let Some(substatus) = response.status.sub_status() {
        headers.insert(
            response_header_names::SUBSTATUS,
            substatus.value().to_string(),
        );
    }
    if let Some(retry_after_ms) = response.retry_after_ms {
        headers.insert("x-ms-retry-after-ms", retry_after_ms.to_string());
    }
    if let Some(lsn) = response.lsn.filter(|value| *value != 0) {
        let value = lsn.to_string();
        headers.insert(response_header_names::LSN, value.clone());
        headers.insert(X_MS_LSN, value);
    }
    if let Some(item_lsn) = response.item_lsn.filter(|value| *value != 0) {
        headers.insert(response_header_names::ITEM_LSN, item_lsn.to_string());
    }
    if let Some(global_committed_lsn) = response.global_committed_lsn.filter(|value| *value != 0) {
        headers.insert(X_MS_GLOBAL_COMMITTED_LSN, global_committed_lsn.to_string());
    }
    if let Some(owner_full_name) = response.owner_full_name {
        headers.insert(response_header_names::OWNER_FULL_NAME, owner_full_name);
    }

    Ok(HttpResponse {
        status,
        headers,
        body: response.body,
    })
}

fn required_header(
    request: &HttpRequest,
    header_name: &HeaderName,
    display_name: &'static str,
) -> azure_core::Result<String> {
    request
        .headers
        .get_optional_str(header_name)
        .map(str::to_owned)
        .ok_or_else(|| data_conversion_error(format!("missing required {display_name} header")))
}

fn next_transport_request_id() -> u32 {
    // AcqRel ensures the increment is globally visible. Relaxed would also produce
    // unique values across threads (fetch_add is atomic regardless of ordering),
    // but AcqRel is preferred here for diagnostic clarity in concurrent traces.
    TRANSPORT_REQUEST_ID.fetch_add(1, Ordering::AcqRel)
}

/// Wire-form payload derived from the partition key + definition for a
/// Gateway 2.0 dispatch.
///
/// `Point` represents a single-logical-partition operation and is emitted as
/// the `EffectivePartitionKey` RNTBD metadata token (binary EPK bytes).
/// `Range` represents an EPK range — either a hierarchical-PK prefix that
/// fans out across multiple physical partitions, or a feed/cross-partition
/// operation scoped to a sub-range — and is emitted as the
/// `x-ms-thinclient-range-min` / `-max` outer HTTP headers carrying the
/// canonical, un-padded hex EPK string per `GATEWAY_20_SPEC §"Range header
/// wire format"`.
///
/// The two arms are mutually exclusive; the proxy must never see both an
/// EPK token and EPK range headers on the same request.
enum EpkPayload {
    Point(Vec<u8>),
    Range { min: String, max: String },
}

fn effective_partition_key_payload(
    inputs: &WrapInputs<'_>,
) -> azure_core::Result<Option<EpkPayload>> {
    let (Some(partition_key), Some(partition_key_definition)) =
        (inputs.partition_key, inputs.partition_key_definition)
    else {
        return Ok(None);
    };

    if partition_key.is_empty() {
        return Ok(None);
    }

    let range =
        EffectivePartitionKey::compute_range(partition_key.values(), partition_key_definition)
            .map_err(|err| {
                data_conversion_error(format!("Gateway 2.0 EPK range computation failed: {err}"))
            })?;

    if range.start == range.end {
        let bytes = hex_to_bytes(range.start.as_str())?;
        Ok(Some(EpkPayload::Point(bytes)))
    } else {
        Ok(Some(EpkPayload::Range {
            min: range.start.as_str().to_owned(),
            max: range.end.as_str().to_owned(),
        }))
    }
}

fn hex_to_bytes(value: &str) -> azure_core::Result<Vec<u8>> {
    if value.len() & 1 != 0 {
        return Err(data_conversion_error(format!(
            "effective partition key hex length {} is not even",
            value.len()
        )));
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    for chunk in value.as_bytes().chunks_exact(2) {
        let hi = hex_digit(chunk[0])?;
        let lo = hex_digit(chunk[1])?;
        bytes.push((hi << 4) | lo);
    }
    Ok(bytes)
}

fn hex_digit(value: u8) -> azure_core::Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(data_conversion_error(format!(
            "invalid effective partition key hex digit 0x{value:02X}"
        ))),
    }
}

struct ResourceNames {
    database: String,
    collection: String,
    document: Option<String>,
}

fn parse_resource_names(resource_link: &str) -> azure_core::Result<ResourceNames> {
    let mut database = None;
    let mut collection = None;
    let mut document = None;
    let mut segments = resource_link
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty());

    while let Some(kind) = segments.next() {
        let Some(name) = segments.next() else {
            break;
        };
        match kind {
            "dbs" => database = Some(name.to_owned()),
            "colls" => collection = Some(name.to_owned()),
            "docs" => document = Some(name.to_owned()),
            _ => {}
        }
    }

    let database = database.filter(|value| !value.is_empty()).ok_or_else(|| {
        data_conversion_error("Gateway 2.0 resource link is missing database name")
    })?;
    let collection = collection
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            data_conversion_error("Gateway 2.0 resource link is missing collection name")
        })?;

    Ok(ResourceNames {
        database,
        collection,
        document,
    })
}

fn data_conversion_error(message: impl Into<String>) -> azure_core::Error {
    azure_core::Error::with_message(ErrorKind::DataConversion, message.into())
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, collections::HashMap};

    use azure_core::http::headers::{ACCEPT, CONTENT_TYPE};

    use super::*;
    use crate::models::{PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion};

    const ACTIVITY_ID: &str = "00112233-4455-6677-8899-aabbccddeeff";

    #[derive(Clone, Debug, PartialEq)]
    enum ParsedTokenValue {
        Byte(u8),
        ULong(u32),
        LongLong(i64),
        Double(f64),
        SmallString(String),
        String(String),
        Bytes(Vec<u8>),
    }

    #[derive(Debug)]
    struct ParsedRequest {
        resource_type: u16,
        operation_type: u16,
        activity_id: Uuid,
        tokens: HashMap<u16, ParsedTokenValue>,
        body: Option<Vec<u8>>,
    }

    fn signed_request(body: Option<&[u8]>) -> HttpRequest {
        let mut headers = Headers::new();
        headers.insert(AUTHORIZATION, "auth-token");
        headers.insert(X_MS_DATE, "Wed, 21 Oct 2015 07:28:00 GMT");
        headers.insert(X_MS_ACTIVITY_ID, ACTIVITY_ID);
        headers.insert(USER_AGENT, "test-agent");
        headers.insert(CONTENT_TYPE, "application/json");
        headers.insert(ACCEPT, "application/json");

        HttpRequest {
            url: "https://account-thin.documents.azure.com:444/dbs/db1/colls/coll1/docs/doc1"
                .parse()
                .unwrap(),
            method: Method::Get,
            headers,
            body: body.map(bytes::Bytes::copy_from_slice),
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        }
    }

    fn wrap_inputs<'a>(
        auth_context: &'a AuthorizationContext,
        operation_type: OperationType,
        partition_key: Option<&'a PartitionKey>,
        partition_key_definition: Option<&'a PartitionKeyDefinition>,
    ) -> WrapInputs<'a> {
        WrapInputs {
            auth_context,
            operation_type,
            resource_type: ResourceType::Document,
            partition_key,
            partition_key_definition,
            effective_consistency: DefaultConsistencyLevel::Session,
            account_name: Some("account"),
        }
    }

    fn parse_wrapped_request(request: &HttpRequest, token_count: usize) -> ParsedRequest {
        let mut src = request.body.as_ref().unwrap().as_ref();
        let total_len = take_u32(&mut src) as usize;
        assert_eq!(total_len, request.body.as_ref().unwrap().len());
        let resource_type = take_u16(&mut src);
        let operation_type = take_u16(&mut src);
        let activity_id = take_uuid(&mut src);

        let mut tokens = HashMap::new();
        for _ in 0..token_count {
            let id = take_u16(&mut src);
            let token_type = take_u8(&mut src);
            let value = parse_token_value(token_type, &mut src);
            tokens.insert(id, value);
        }

        let body = if src.is_empty() {
            None
        } else {
            let body_len = take_u32(&mut src) as usize;
            assert_eq!(src.len(), body_len);
            Some(src.to_vec())
        };

        ParsedRequest {
            resource_type,
            operation_type,
            activity_id,
            tokens,
            body,
        }
    }

    fn parse_token_value(token_type: u8, src: &mut &[u8]) -> ParsedTokenValue {
        match token_type {
            0x00 => ParsedTokenValue::Byte(take_u8(src)),
            0x02 => ParsedTokenValue::ULong(take_u32(src)),
            0x05 => ParsedTokenValue::LongLong(take_i64(src)),
            0x07 => {
                let len = take_u8(src) as usize;
                ParsedTokenValue::SmallString(take_string(src, len))
            }
            0x08 => {
                let len = take_u16(src) as usize;
                ParsedTokenValue::String(take_string(src, len))
            }
            0x0B => {
                let len = take_u16(src) as usize;
                ParsedTokenValue::Bytes(take_bytes(src, len).to_vec())
            }
            0x0E => ParsedTokenValue::Double(f64::from_le_bytes(take_array(src))),
            other => panic!("unexpected token type 0x{other:02X}"),
        }
    }

    #[test]
    fn wrap_builds_required_request_tokens_for_read() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let wrapped = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 10);

        assert_eq!(wrapped.method, Method::Post);
        assert_eq!(parsed.resource_type, 0x0003);
        assert_eq!(parsed.operation_type, 0x0003);
        assert_eq!(parsed.activity_id, Uuid::parse_str(ACTIVITY_ID).unwrap());
        assert_eq!(
            parsed.tokens[&0x0001],
            ParsedTokenValue::String("auth-token".into())
        );
        assert_eq!(parsed.tokens[&0x0002], ParsedTokenValue::Byte(0));
        assert_eq!(
            parsed.tokens[&0x0003],
            ParsedTokenValue::SmallString("Wed, 21 Oct 2015 07:28:00 GMT".into())
        );
        assert_eq!(parsed.tokens[&0x0010], ParsedTokenValue::Byte(0x02));
        assert_eq!(
            parsed.tokens[&0x0015],
            ParsedTokenValue::String("db1".into())
        );
        assert_eq!(
            parsed.tokens[&0x0016],
            ParsedTokenValue::String("coll1".into())
        );
        assert_eq!(
            parsed.tokens[&0x0017],
            ParsedTokenValue::String("doc1".into())
        );
        assert_eq!(
            parsed.tokens[&0x004D],
            ParsedTokenValue::ULong(parsed_transport_id(&parsed))
        );
        assert_eq!(
            parsed.tokens[&0x00A2],
            ParsedTokenValue::ULong(SUPPORTED_CAPABILITIES_BITS)
        );
        assert_eq!(
            parsed.tokens[&0x00CE],
            ParsedTokenValue::String("account".into())
        );
    }

    #[test]
    fn wrap_preserves_payload_and_sets_payload_present() {
        let request = signed_request(Some(br#"{"id":"doc1"}"#));
        let auth_context =
            AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(&auth_context, OperationType::Create, None, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 9);

        assert_eq!(parsed.tokens[&0x0002], ParsedTokenValue::Byte(1));
        assert_eq!(parsed.body, Some(br#"{"id":"doc1"}"#.to_vec()));
    }

    #[test]
    fn wrap_omits_document_name_for_create() {
        let request = signed_request(Some(b"{}"));
        let auth_context =
            AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(&auth_context, OperationType::Create, None, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 9);

        assert!(!parsed.tokens.contains_key(&0x0017));
    }

    #[test]
    fn wrap_uses_resolved_consistency_token() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None, None);
        inputs.effective_consistency = DefaultConsistencyLevel::Eventual;

        let wrapped = wrap_request_for_gateway20(&request, &inputs).unwrap();
        let parsed = parse_wrapped_request(&wrapped, 10);

        assert_eq!(parsed.tokens[&0x0010], ParsedTokenValue::Byte(0x03));
    }

    #[test]
    fn wrap_computes_effective_partition_key_bytes() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let partition_key = PartitionKey::from("tenant1");
        let partition_key_definition = PartitionKeyDefinition::new(vec![Cow::from("/tenantId")]);
        let expected = hex_to_bytes(
            EffectivePartitionKey::compute(
                partition_key.values(),
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V2,
            )
            .as_str(),
        )
        .unwrap();

        let wrapped = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Read,
                Some(&partition_key),
                Some(&partition_key_definition),
            ),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 11);

        assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
    }

    /// HPK partial-PK (prefix on a MultiHash container) is dispatched as an
    /// EPK *range* via the outer `x-ms-thinclient-range-min`/`-max` HTTP
    /// headers, not as an `EffectivePartitionKey` RNTBD token. The two
    /// emission paths must be mutually exclusive.
    #[test]
    fn wrap_emits_range_headers_for_hpk_prefix_partition_key() {
        let request = signed_request(None);
        let auth_context =
            AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");
        let partition_key =
            PartitionKey::from(vec![PartitionKeyValue::from("tenant1".to_string())]);
        let partition_key_definition =
            PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
        let expected_range =
            EffectivePartitionKey::compute_range(partition_key.values(), &partition_key_definition)
                .unwrap();
        assert_ne!(
            expected_range.start, expected_range.end,
            "HPK prefix must produce a non-point range — sanity check"
        );

        let wrapped = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Query,
                Some(&partition_key),
                Some(&partition_key_definition),
            ),
        )
        .unwrap();

        // Range headers on the outer HTTP request, carrying canonical un-padded hex.
        assert_eq!(
            wrapped.headers.get_optional_str(&GATEWAY20_RANGE_MIN),
            Some(expected_range.start.as_str())
        );
        assert_eq!(
            wrapped.headers.get_optional_str(&GATEWAY20_RANGE_MAX),
            Some(expected_range.end.as_str())
        );

        // No EPK token in the inner RNTBD frame for the range path.
        // Token layout: 9 base tokens (account, db, coll, payload_present,
        // auth, date, consistency, transport_request_id, capabilities) — no
        // document_name (resource link omits /docs/...) and no EPK token.
        let parsed = parse_wrapped_request(&wrapped, 9);
        assert!(
            !parsed.tokens.contains_key(&0x005A),
            "EffectivePartitionKey token must not be emitted alongside range headers"
        );
    }

    /// Full HPK key (component count == definition path count) collapses to a
    /// point op: emit the EPK token, no range headers.
    #[test]
    fn wrap_emits_token_only_for_full_hpk_partition_key() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let partition_key = PartitionKey::from(vec![
            PartitionKeyValue::from("tenant1".to_string()),
            PartitionKeyValue::from("user1".to_string()),
            PartitionKeyValue::from("session1".to_string()),
        ]);
        let partition_key_definition =
            PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));

        let wrapped = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Read,
                Some(&partition_key),
                Some(&partition_key_definition),
            ),
        )
        .unwrap();

        // Range headers must NOT be present on the point path.
        assert!(wrapped
            .headers
            .get_optional_str(&GATEWAY20_RANGE_MIN)
            .is_none());
        assert!(wrapped
            .headers
            .get_optional_str(&GATEWAY20_RANGE_MAX)
            .is_none());

        // EPK token present in the inner RNTBD frame.
        let parsed = parse_wrapped_request(&wrapped, 11);
        assert!(
            parsed.tokens.contains_key(&0x005A),
            "EffectivePartitionKey token must be emitted for full HPK partition key"
        );
    }

    /// `compute_range` error cases (e.g., more PK components supplied than the
    /// container's definition declares) must surface as a wrap error, mapped
    /// to `BadRequest` upstream — never silently emit broken EPK metadata.
    #[test]
    fn wrap_rejects_partition_key_with_too_many_components() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let partition_key = PartitionKey::from(vec![
            PartitionKeyValue::from("tenant1".to_string()),
            PartitionKeyValue::from("extra".to_string()),
        ]);
        let partition_key_definition = PartitionKeyDefinition::from("/tenantId");

        let error = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Read,
                Some(&partition_key),
                Some(&partition_key_definition),
            ),
        )
        .unwrap_err();

        assert_eq!(error.kind(), &ErrorKind::DataConversion);
    }

    #[test]
    fn wrap_propagates_continuation_token_into_rntbd_metadata() {
        let mut request = signed_request(None);
        request.headers.insert(X_MS_CONTINUATION, "page-token-1");
        let auth_context =
            AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway20(
            &request,
            &WrapInputs {
                auth_context: &auth_context,
                operation_type: OperationType::Query,
                resource_type: ResourceType::Document,
                partition_key: None,
                partition_key_definition: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                account_name: Some("account"),
            },
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 10);

        assert_eq!(
            parsed.tokens[&0x0006],
            ParsedTokenValue::String("page-token-1".into()),
            "continuation token should be encoded as string token 0x0006",
        );
        assert!(
            wrapped
                .headers
                .get_optional_str(&X_MS_CONTINUATION)
                .is_none(),
            "x-ms-continuation header should not be forwarded on the outer HTTP request",
        );
    }

    #[test]
    fn wrap_omits_continuation_token_when_header_absent() {
        let request = signed_request(None);
        let auth_context =
            AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway20(
            &request,
            &WrapInputs {
                auth_context: &auth_context,
                operation_type: OperationType::Query,
                resource_type: ResourceType::Document,
                partition_key: None,
                partition_key_definition: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                account_name: Some("account"),
            },
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 9);

        assert!(
            !parsed.tokens.contains_key(&0x0006),
            "continuation token should be absent when no x-ms-continuation header is present",
        );
    }

    #[test]
    fn wrap_emits_empty_continuation_token_when_header_value_empty() {
        // Symmetry with .NET (`ThinClientStoreClient.PrepareRequestForProxyAsync`),
        // Java (`RntbdRequestHeader.ContinuationToken` is *not* in
        // `thinClientProxyExcludedSet`), and the unwrap side which forwards
        // empty continuation strings verbatim. Continuation is opaque on the
        // wire — the wrap path does not infer intent from emptiness.
        let mut request = signed_request(None);
        request.headers.insert(X_MS_CONTINUATION, "");
        let auth_context =
            AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway20(
            &request,
            &WrapInputs {
                auth_context: &auth_context,
                operation_type: OperationType::Query,
                resource_type: ResourceType::Document,
                partition_key: None,
                partition_key_definition: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                account_name: Some("account"),
            },
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 10);

        assert_eq!(
            parsed.tokens[&0x0006],
            ParsedTokenValue::String(String::new()),
            "empty continuation header should be emitted as a zero-length string token",
        );
    }

    #[test]
    fn wrap_only_keeps_user_agent_and_activity_id_headers() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let wrapped = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None, None),
        )
        .unwrap();

        assert_eq!(
            wrapped.headers.get_optional_str(&USER_AGENT),
            Some("test-agent")
        );
        assert_eq!(
            wrapped.headers.get_optional_str(&X_MS_ACTIVITY_ID),
            Some(ACTIVITY_ID)
        );
        assert!(wrapped.headers.get_optional_str(&AUTHORIZATION).is_none());
        assert!(wrapped.headers.get_optional_str(&X_MS_DATE).is_none());
        assert!(wrapped.headers.get_optional_str(&CONTENT_TYPE).is_none());
        assert!(wrapped.headers.get_optional_str(&ACCEPT).is_none());
    }

    #[test]
    fn wrap_rejects_missing_authorization_header() {
        let mut request = signed_request(None);
        request.headers.remove(AUTHORIZATION);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let error = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None, None),
        )
        .unwrap_err();

        assert_eq!(error.kind(), &ErrorKind::DataConversion);
    }

    #[test]
    fn wrap_rejects_missing_date_header() {
        let mut request = signed_request(None);
        request.headers.remove(X_MS_DATE);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let error = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None, None),
        )
        .unwrap_err();

        assert_eq!(error.kind(), &ErrorKind::DataConversion);
    }

    #[test]
    fn wrap_rejects_invalid_activity_id() {
        let mut request = signed_request(None);
        request.headers.insert(X_MS_ACTIVITY_ID, "not-a-guid");
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let error = wrap_request_for_gateway20(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None, None),
        )
        .unwrap_err();

        assert_eq!(error.kind(), &ErrorKind::DataConversion);
    }

    #[test]
    fn unwrap_maps_response_status_headers_and_body() {
        let activity_id = Uuid::parse_str(ACTIVITY_ID).unwrap();
        let response = HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: response_frame(
                404,
                activity_id,
                |tokens| {
                    write_u32_token(tokens, 0x001C, 1002);
                    write_double_token(tokens, 0x0015, 3.5);
                    write_string_token(tokens, 0x003E, "1:2#3");
                    write_string_token(tokens, 0x0004, "\"etag\"");
                    write_string_token(tokens, 0x0003, "continuation");
                    write_i64_token(tokens, 0x0013, 42);
                    write_i64_token(tokens, 0x0032, 43);
                    write_i64_token(tokens, 0x0029, 44);
                    write_string_token(tokens, 0x0017, "dbs/db1/colls/coll1/docs/doc1");
                },
                b"{}",
            ),
        };

        let unwrapped = unwrap_response_for_gateway20(response).unwrap();

        assert_eq!(unwrapped.status, 404);
        assert_eq!(unwrapped.body, b"{}".to_vec());
        assert_eq!(
            unwrapped.headers.get_optional_str(&X_MS_ACTIVITY_ID),
            Some(ACTIVITY_ID)
        );
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-substatus")),
            Some("1002")
        );
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-request-charge")),
            Some("3.5")
        );
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-session-token")),
            Some("1:2#3")
        );
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("etag")),
            Some("\"etag\"")
        );
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-continuation")),
            Some("continuation")
        );
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("lsn")),
            Some("42")
        );
        assert_eq!(unwrapped.headers.get_optional_str(&X_MS_LSN), Some("42"));
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-item-lsn")),
            Some("43")
        );
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&X_MS_GLOBAL_COMMITTED_LSN),
            Some("44")
        );
    }

    #[test]
    fn unwrap_preserves_retry_after_for_throttle() {
        let response = HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: response_frame(
                429,
                Uuid::parse_str(ACTIVITY_ID).unwrap(),
                |tokens| write_u32_token(tokens, 0x000C, 125),
                b"",
            ),
        };

        let unwrapped = unwrap_response_for_gateway20(response).unwrap();

        assert_eq!(unwrapped.status, 429);
        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-retry-after-ms")),
            Some("125")
        );
    }

    #[test]
    fn unwrap_rejects_malformed_rntbd_body() {
        let response = HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: vec![1, 2, 3],
        };

        let error = unwrap_response_for_gateway20(response).unwrap_err();

        assert_eq!(error.kind(), &ErrorKind::DataConversion);
    }

    #[test]
    fn unwrap_rejects_out_of_range_inner_status() {
        let response = HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: response_frame(70_000, Uuid::parse_str(ACTIVITY_ID).unwrap(), |_| {}, b""),
        };

        let error = unwrap_response_for_gateway20(response).unwrap_err();

        assert_eq!(error.kind(), &ErrorKind::DataConversion);
    }

    fn parsed_transport_id(parsed: &ParsedRequest) -> u32 {
        match parsed.tokens[&0x004D] {
            ParsedTokenValue::ULong(value) => value,
            _ => unreachable!(),
        }
    }

    fn response_frame(
        status: u32,
        activity_id: Uuid,
        write_tokens: impl FnOnce(&mut Vec<u8>),
        body: &[u8],
    ) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&status.to_le_bytes());
        write_uuid(&mut bytes, activity_id);
        write_tokens(&mut bytes);
        bytes.extend_from_slice(body);
        let total_len = u32::try_from(bytes.len()).unwrap();
        bytes[0..4].copy_from_slice(&total_len.to_le_bytes());
        bytes
    }

    fn write_string_token(bytes: &mut Vec<u8>, id: u16, value: &str) {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.push(0x08);
        bytes.extend_from_slice(&(value.len() as u16).to_le_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }

    fn write_u32_token(bytes: &mut Vec<u8>, id: u16, value: u32) {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.push(0x02);
        bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn write_i64_token(bytes: &mut Vec<u8>, id: u16, value: i64) {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.push(0x05);
        bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn write_double_token(bytes: &mut Vec<u8>, id: u16, value: f64) {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.push(0x0E);
        bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn write_uuid(bytes: &mut Vec<u8>, value: Uuid) {
        let (data1, data2, data3, data4) = value.as_fields();
        bytes.extend_from_slice(&data1.to_le_bytes());
        bytes.extend_from_slice(&data2.to_le_bytes());
        bytes.extend_from_slice(&data3.to_le_bytes());
        bytes.extend_from_slice(data4);
    }

    fn take_u8(src: &mut &[u8]) -> u8 {
        let value = src[0];
        *src = &src[1..];
        value
    }

    fn take_u16(src: &mut &[u8]) -> u16 {
        u16::from_le_bytes(take_array(src))
    }

    fn take_u32(src: &mut &[u8]) -> u32 {
        u32::from_le_bytes(take_array(src))
    }

    fn take_i64(src: &mut &[u8]) -> i64 {
        i64::from_le_bytes(take_array(src))
    }

    fn take_uuid(src: &mut &[u8]) -> Uuid {
        let data1 = u32::from_le_bytes(take_array(src));
        let data2 = u16::from_le_bytes(take_array(src));
        let data3 = u16::from_le_bytes(take_array(src));
        let data4: [u8; 8] = take_array(src);
        Uuid::from_fields(data1, data2, data3, &data4)
    }

    fn take_string(src: &mut &[u8], len: usize) -> String {
        String::from_utf8(take_bytes(src, len).to_vec()).unwrap()
    }

    fn take_bytes<'a>(src: &mut &'a [u8], len: usize) -> &'a [u8] {
        let (head, tail) = src.split_at(len);
        *src = tail;
        head
    }

    fn take_array<const N: usize>(src: &mut &[u8]) -> [u8; N] {
        let bytes = take_bytes(src, N);
        let mut out = [0; N];
        out.copy_from_slice(bytes);
        out
    }
}
