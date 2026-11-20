// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{borrow::Cow, collections::HashMap};

use azure_core::http::headers::{ACCEPT, CONTENT_TYPE};

use azure_core::{
    error::ErrorKind,
    http::{
        headers::{HeaderName, HeaderValue, Headers, AUTHORIZATION, USER_AGENT},
        Method,
    },
};
use uuid::Uuid;

use crate::models::{
    cosmos_headers::{request_header_names, response_header_names},
    effective_partition_key::EffectivePartitionKey,
    DefaultConsistencyLevel, OperationType, ResourceType,
};

use super::super::{
    cosmos_headers::{CLIENT_ID, SUPPORTED_CAPABILITIES_BITS},
    cosmos_transport_client::{HttpRequest, HttpResponse},
    AuthorizationContext,
};
use crate::rntbd_encoding::RntbdRequestFrame;

use super::request_tokens::parse_supported_serialization_formats;
use super::{unwrap_response_for_gateway_v2, wrap_request_for_gateway_v2};
use super::{
    GatewayV2Headers, GATEWAY_V2_RANGE_MAX, GATEWAY_V2_RANGE_MIN, X_MS_ACTIVITY_ID,
    X_MS_CONTINUATION, X_MS_DATE, X_MS_GLOBAL_COMMITTED_LSN, X_MS_LSN,
};
use crate::models::effective_partition_key::{
    effective_partition_key_multi_hash_v2_binary, effective_partition_key_v1_binary,
    effective_partition_key_v2_binary,
};
use crate::models::PartitionKeyValue;
use crate::models::PartitionKeyVersion;
use crate::models::{PartitionKey, PartitionKeyDefinition};

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
    effective_partition_key: Option<&'a EffectivePartitionKey>,
) -> GatewayV2Headers<'a> {
    GatewayV2Headers {
        auth_context,
        operation_type,
        resource_type: ResourceType::Document,
        effective_partition_key,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        account_name: Some("account"),
        collection_rid: None,
    }
}

/// Computes the effective partition key the way the operation pipeline does,
/// so dispatch tests can supply a precomputed EPK to `wrap_inputs`.
fn epk(
    partition_key: &PartitionKey,
    partition_key_definition: &PartitionKeyDefinition,
) -> EffectivePartitionKey {
    EffectivePartitionKey::compute(
        partition_key.values(),
        partition_key_definition.kind(),
        partition_key_definition.version(),
    )
}

fn parse_wrapped_request(request: &HttpRequest, _token_count: usize) -> ParsedRequest {
    let body_bytes = request.body.as_ref().unwrap();
    let mut src = body_bytes.as_ref();
    // RNTBD `LengthInBytes` is the request header section length only
    // (length field + resource/operation type + activity id + metadata),
    // excluding the body length prefix and body bytes.
    let header_len = take_u32(&mut src) as usize;
    let resource_type = take_u16(&mut src);
    let operation_type = take_u16(&mut src);
    let activity_id = take_uuid(&mut src);

    let mut tokens = HashMap::new();
    // Read tokens until the cursor reaches the end of the header section.
    while body_bytes.len() - src.len() < header_len {
        let id = take_u16(&mut src);
        let token_type = take_u8(&mut src);
        let value = parse_token_value(token_type, &mut src);
        tokens.insert(id, value);
    }
    assert_eq!(body_bytes.len() - src.len(), header_len);

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
    if !body.is_empty() {
        // PayloadPresent = true (id 0x0000, type Byte).
        bytes.extend_from_slice(&0x0000_u16.to_le_bytes());
        bytes.push(0x00);
        bytes.push(1);
    }
    let total_len = u32::try_from(bytes.len()).unwrap();
    bytes[0..4].copy_from_slice(&total_len.to_le_bytes());
    if !body.is_empty() {
        bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
        bytes.extend_from_slice(body);
    }
    bytes
}

fn write_string_token(bytes: &mut Vec<u8>, id: u16, value: &str) {
    bytes.extend_from_slice(&id.to_le_bytes());
    bytes.push(0x08);
    bytes.extend_from_slice(&(value.len() as u16).to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

fn write_small_string_token(bytes: &mut Vec<u8>, id: u16, value: &str) {
    bytes.extend_from_slice(&id.to_le_bytes());
    bytes.push(0x07);
    bytes.push(u8::try_from(value.len()).unwrap());
    bytes.extend_from_slice(value.as_bytes());
}

fn write_byte_token(bytes: &mut Vec<u8>, id: u16, value: u8) {
    bytes.extend_from_slice(&id.to_le_bytes());
    bytes.push(0x00);
    bytes.push(value);
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

fn write_u64_token(bytes: &mut Vec<u8>, id: u16, value: u64) {
    bytes.extend_from_slice(&id.to_le_bytes());
    bytes.push(0x04);
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

mod request;
mod request_tokens;
mod response;
