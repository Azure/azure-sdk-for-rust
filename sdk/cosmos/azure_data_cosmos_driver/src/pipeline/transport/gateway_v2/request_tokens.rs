// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Maps request data to ordered RNTBD frame metadata.

use azure_core::http::headers::{HeaderName, AUTHORIZATION};
use uuid::Uuid;

use crate::{
    models::{cosmos_headers::request_header_names, resource_id, OperationType, ResourceType},
    pipeline::transport::gateway_v2,
};

use super::super::{
    cosmos_headers::SUPPORTED_CAPABILITIES_BITS, cosmos_transport_client::HttpRequest,
};
use crate::rntbd::{RntbdRequestFrame, Token};

use super::{
    WrapInputs, A_IM, CHANGE_FEED_WIRE_FORMAT_VERSION, IF_MATCH, IF_MODIFIED_SINCE, IF_NONE_MATCH,
    X_MS_ACTIVITY_ID, X_MS_CONTINUATION, X_MS_DATE, X_MS_MAX_ITEM_COUNT, X_MS_SESSION_TOKEN,
};
pub(super) struct GatewayV2FrameData<'a> {
    pub frame: RntbdRequestFrame,
    pub account_name: &'a str,
    pub emitted_point_epk: bool,
}

/// Maps signed request headers, payload, and routing inputs to a frame with ordered metadata tokens.
///
/// Validates required identity fields and returns facts also needed by the outer HTTP envelope.
pub(super) fn build_request_frame<'a>(
    request: &HttpRequest,
    inputs: &WrapInputs<'a>,
) -> azure_core::Result<GatewayV2FrameData<'a>> {
    let authorization = required_header(request, &AUTHORIZATION, "authorization")?;
    let date = required_header(request, &X_MS_DATE, "x-ms-date")?;
    let activity_id = required_header(request, &X_MS_ACTIVITY_ID, "x-ms-activity-id")?;
    let activity_id = Uuid::parse_str(&activity_id).map_err(|e| {
        gateway_v2::data_conversion_error(format!("x-ms-activity-id is not a valid UUID: {e}"))
    })?;
    let account_name = inputs
        .account_name
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            gateway_v2::data_conversion_error("Gateway 2.0 dispatch requires an account name")
        })?;

    let resource_names = parse_resource_names(inputs.auth_context.resource_link.as_str())?;
    let has_payload = request.body.as_ref().is_some_and(|body| !body.is_empty());

    let epk_payload = effective_partition_key_payload(inputs)?;

    let mut metadata = Vec::with_capacity(16);
    // Thin-client expects these routing tokens in the leading positions, in the
    // order defined by Java's `RntbdConstants.thinClientHeadersInOrderList`:
    //   EffectivePartitionKey, StartEpkHash, EndEpkHash, GlobalDatabaseAccountName,
    //   DatabaseName, CollectionName, CollectionRid, ResourceId, PayloadPresent,
    //   DocumentName, AuthorizationToken, Date.
    // We emit them first and in this sequence to match the reference client.
    let emitted_point_epk = matches!(epk_payload.as_ref(), Some(EpkPayload::Point(_)));
    if let Some(EpkPayload::Point(epk)) = epk_payload.as_ref() {
        metadata.push(Token::effective_partition_key(epk.clone()));
    }
    // Per-partition routing tokens for thin-client queries.
    // `StartEpkHash`/`EndEpkHash` (Bytes) are emitted when there is a resolved
    // partition key range and NO partition key — the two forms are mutually
    // exclusive. The Rust pipeline surfaces
    // pkrange boundaries as the `x-ms-start-epk`/`x-ms-end-epk` HTTP headers
    // (lower-hex strings), so decode and forward them here when we did NOT
    // already emit an `EffectivePartitionKey` point token. Empty string is a
    // valid value — it means the partition's min EPK boundary (`MinValue`
    // sentinel), which serializes to a zero-byte token.
    if !emitted_point_epk {
        // Prefer explicit narrowed bounds; otherwise use the physical partition's routing bounds.
        let start_epk_header = HeaderName::from_static(request_header_names::START_EPK);
        let start_fallback = HeaderName::from_static(request_header_names::THINCLIENT_PKRANGE_MIN);
        if let Some(epk_hex) = request
            .headers
            .get_optional_str(&start_epk_header)
            .or_else(|| request.headers.get_optional_str(&start_fallback))
        {
            if let Some(bytes) = decode_epk_hex(epk_hex) {
                metadata.push(Token::start_epk_hash(bytes));
            }
        }
        let end_epk_header = HeaderName::from_static(request_header_names::END_EPK);
        let end_fallback = HeaderName::from_static(request_header_names::THINCLIENT_PKRANGE_MAX);
        if let Some(epk_hex) = request
            .headers
            .get_optional_str(&end_epk_header)
            .or_else(|| request.headers.get_optional_str(&end_fallback))
        {
            if let Some(bytes) = decode_epk_hex(epk_hex) {
                metadata.push(Token::end_epk_hash(bytes));
            }
        }
    }
    metadata.push(Token::global_database_account_name(account_name.to_owned()));
    metadata.push(Token::database_name(resource_names.database));
    metadata.push(Token::collection_name(resource_names.collection));
    if let Some(rid) = inputs.collection_rid.filter(|s| !s.is_empty()) {
        metadata.push(Token::collection_rid(rid.to_owned()));
        let decoded = resource_id::decode_rid(rid).map_err(|e| {
            gateway_v2::data_conversion_error(format!("invalid collection RID: {e}"))
        })?;
        metadata.push(Token::resource_id(decoded));
    }
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
    append_feed_filter_tokens(request, &mut metadata)?;
    append_partition_tokens(request, &mut metadata);
    append_batch_tokens(request, &mut metadata, inputs.operation_type)?;
    append_operation_options(request, &mut metadata);
    // Rule 1+5: when RCS is non-Default on a read, emit the RNTBD
    // ReadConsistencyStrategy token (0x00FE) and DROP the ConsistencyLevel token.
    // Otherwise, emit ConsistencyLevel as before (Default => transparent on wire
    // by virtue of carrying the resolved effective consistency).
    if inputs.read_consistency_strategy.is_non_default() {
        metadata.push(Token::read_consistency_strategy(
            inputs.read_consistency_strategy,
        ));
    } else {
        metadata.push(Token::consistency_level(inputs.effective_consistency));
    }
    // TransportRequestId (0x004D) is in the thin-client exclusion list — the
    // proxy assigns its own request id, so emitting one here triggers a routing
    // error.
    metadata.push(Token::sdk_supported_capabilities(
        SUPPORTED_CAPABILITIES_BITS,
    ));
    append_pagination_and_condition_tokens(request, &mut metadata, inputs.operation_type);
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
    };
    Ok(GatewayV2FrameData {
        frame,
        account_name,
        emitted_point_epk,
    })
}

/// Appends explicit EPK filter bounds and the feed-key type, rejecting an invalid nonempty key type.
fn append_feed_filter_tokens(
    request: &HttpRequest,
    metadata: &mut Vec<Token>,
) -> azure_core::Result<()> {
    // Filtering uses UTF-8 hex text, unlike the binary routing hashes above.
    for (name, constructor) in [
        (
            request_header_names::START_EPK,
            Token::start_epk as fn(String) -> Token,
        ),
        (request_header_names::END_EPK, Token::end_epk),
    ] {
        if let Some(value) = request
            .headers
            .get_optional_str(&HeaderName::from_static(name))
            .filter(|value| !value.is_empty())
        {
            metadata.push(constructor(value.to_owned()));
        }
    }
    if let Some(value) = request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::READ_FEED_KEY_TYPE,
        ))
        .filter(|value| !value.is_empty())
    {
        metadata.push(Token::read_feed_key_type(value.parse()?));
    }
    Ok(())
}

/// Appends nonempty logical partition-key and physical partition-range headers as metadata tokens.
fn append_partition_tokens(request: &HttpRequest, metadata: &mut Vec<Token>) {
    // String-form partition key (e.g. `["pk1"]`) emitted alongside
    // EffectivePartitionKey. The V1 request already carries it as the
    // x-ms-documentdb-partitionkey header, so forward that JSON unchanged.
    let pk_header = HeaderName::from_static(request_header_names::PARTITION_KEY);
    if let Some(pk_json) = request.headers.get_optional_str(&pk_header) {
        if !pk_json.is_empty() {
            metadata.push(Token::partition_key(pk_json.to_owned()));
        }
    }
    // PartitionKeyRangeId (0x002C) — filled from the
    // `x-ms-documentdb-partitionkeyrangeid` HTTP header. The thin-client proxy
    // uses it alongside
    // StartEpkHash/EndEpkHash to disambiguate the target physical partition
    // when the EPK range straddles boundaries. Without it the proxy/backend
    // interprets the empty StartEpkHash as a `[]` partition-key value and
    // rejects with "PartitionKey supplied in x-ms-partitionkey header has
    // fewer components than defined in the the collection."
    let pkr_id_header = HeaderName::from_static(request_header_names::PARTITION_KEY_RANGE_ID);
    if let Some(pkr_id) = request
        .headers
        .get_optional_str(&pkr_id_header)
        .filter(|v| !v.is_empty())
    {
        metadata.push(Token::partition_key_range_id(pkr_id.to_owned()));
    }
}

/// Appends batch control flags for batch operations, rejecting header values other than true or false.
fn append_batch_tokens(
    request: &HttpRequest,
    metadata: &mut Vec<Token>,
    operation_type: OperationType,
) -> azure_core::Result<()> {
    if operation_type == OperationType::Batch {
        for (name, constructor) in [
            (
                request_header_names::BATCH_ATOMIC,
                Token::is_batch_atomic as fn(bool) -> Token,
            ),
            (
                request_header_names::BATCH_CONTINUE_ON_ERROR,
                Token::should_batch_continue_on_error,
            ),
        ] {
            if let Some(value) = request
                .headers
                .get_optional_str(&HeaderName::from_static(name))
            {
                let value = value.trim();
                let enabled = if value.eq_ignore_ascii_case("true") {
                    true
                } else if value.eq_ignore_ascii_case("false") {
                    false
                } else {
                    return Err(gateway_v2::data_conversion_error(format!(
                        "{name} must be true or false"
                    )));
                };
                metadata.push(constructor(enabled));
            }
        }
    }
    Ok(())
}

/// Appends write preferences, query-plan options, and recognized serialization formats from headers.
fn append_operation_options(request: &HttpRequest, metadata: &mut Vec<Token>) {
    // AllowTentativeWrites (0x0066) and ReturnPreference (0x0082) must be CONDITIONAL,
    // not unconditional — emit them only when the caller passes the corresponding
    // HTTP headers (`x-ms-cosmos-allow-tentative-writes` and `Prefer: return=minimal`).
    // Emitting `ReturnPreference=true` unconditionally tells the server "return minimal"
    // (no body) -- which on a Create silently discards the document body server-side
    // and returns an empty response, leading to phantom 0-byte documents and EOF
    // parsing errors on subsequent reads.
    let allow_tentative_header =
        HeaderName::from_static(request_header_names::ALLOW_TENTATIVE_WRITES);
    if request
        .headers
        .get_optional_str(&allow_tentative_header)
        .is_some_and(|v| v.eq_ignore_ascii_case("true"))
    {
        metadata.push(Token::allow_tentative_writes(true));
    }
    let prefer_header = HeaderName::from_static(request_header_names::PREFER);
    if request
        .headers
        .get_optional_str(&prefer_header)
        .is_some_and(|v| v.to_ascii_lowercase().contains("return=minimal"))
    {
        metadata.push(Token::return_preference(true));
    }
    // QueryPlan token forwarding — when the SDK negotiates query features via
    // the standard HTTP headers, mirror them into the RNTBD body so the proxy
    // can resolve a compatible plan.
    let supported_query_features_header =
        HeaderName::from_static(request_header_names::SUPPORTED_QUERY_FEATURES);
    if let Some(features) = request
        .headers
        .get_optional_str(&supported_query_features_header)
        .filter(|v| !v.is_empty())
    {
        metadata.push(Token::supported_query_features(features.to_owned()));
    }
    let query_version_header = HeaderName::from_static(request_header_names::QUERY_VERSION);
    if let Some(version) = request
        .headers
        .get_optional_str(&query_version_header)
        .filter(|v| !v.is_empty())
    {
        metadata.push(Token::query_version(version.to_owned()));
    }
    // SupportedSerializationFormats (0x00C4, Byte): forward the binary-response
    // negotiation from the `x-ms-cosmos-supported-serialization-formats` header
    // (a string accept-list) as its equivalent RNTBD flags byte. Skip empty or
    // unrecognized (byte 0) values so nothing changes when binary is off.
    let supported_serialization_formats_header =
        HeaderName::from_static(request_header_names::SUPPORTED_SERIALIZATION_FORMATS);
    if let Some(value) = request
        .headers
        .get_optional_str(&supported_serialization_formats_header)
        .filter(|v| !v.is_empty())
    {
        let flags = parse_supported_serialization_formats(value);
        if flags != 0 {
            metadata.push(Token::supported_serialization_formats(flags));
        }
    }
}

/// Appends paging, session, conditional-request, and change-feed metadata from the request headers.
fn append_pagination_and_condition_tokens(
    request: &HttpRequest,
    metadata: &mut Vec<Token>,
    operation_type: OperationType,
) {
    if let Some(continuation) = request.headers.get_optional_str(&X_MS_CONTINUATION) {
        metadata.push(Token::continuation_token(continuation.to_owned()));
    }
    // Session token (0x0005) carries the client's per-partition LSN progress so
    // the backend can serve session-consistent reads. Java's thin-client encoder
    // forwards it from the `x-ms-session-token` header; empty values are skipped.
    if let Some(session_token) = request
        .headers
        .get_optional_str(&X_MS_SESSION_TOKEN)
        .filter(|s| !s.is_empty())
    {
        metadata.push(Token::session_token(session_token.to_owned()));
    }

    // Page size (0x0004) carries the requested max item count for query and
    // read-feed pages, parsed from the `x-ms-max-item-count` header. A negative
    // (unbounded) request is encoded as 0xFFFFFFFF. Empty or unparseable values
    // are skipped.
    if let Some(value) = request
        .headers
        .get_optional_str(&X_MS_MAX_ITEM_COUNT)
        .filter(|s| !s.is_empty())
    {
        if let Ok(parsed) = value.parse::<i64>() {
            let page_size = if parsed < 0 {
                0xFFFF_FFFF
            } else {
                parsed.min(0xFFFF_FFFF) as u32
            };
            metadata.push(Token::page_size(page_size));
        }
    }

    // Match condition (0x0008): reads and read-feeds carry `If-None-Match`
    // (conditional GET), all other operations carry `If-Match` (the ETag write
    // precondition). Empty values are skipped.
    let match_header = match operation_type {
        OperationType::Read | OperationType::ReadFeed => &IF_NONE_MATCH,
        _ => &IF_MATCH,
    };
    if let Some(value) = request
        .headers
        .get_optional_str(match_header)
        .filter(|s| !s.is_empty())
    {
        metadata.push(Token::match_condition(value.to_owned()));
    }
    if let Some(value) = request
        .headers
        .get_optional_str(&IF_MODIFIED_SINCE)
        .filter(|s| !s.is_empty())
    {
        metadata.push(Token::if_modified_since(value.to_owned()));
    }
    if let Some(value) = request
        .headers
        .get_optional_str(&A_IM)
        .filter(|s| !s.is_empty())
    {
        metadata.push(Token::a_im(value.to_owned()));
    }
    if let Some(value) = request
        .headers
        .get_optional_str(&CHANGE_FEED_WIRE_FORMAT_VERSION)
        .filter(|s| !s.is_empty())
    {
        metadata.push(Token::change_feed_wire_format_version(value.to_owned()));
    }
}

/// Copies a required header value or returns a conversion error naming the missing header.
fn required_header(
    request: &HttpRequest,
    header_name: &HeaderName,
    display_name: &'static str,
) -> azure_core::Result<String> {
    request
        .headers
        .get_optional_str(header_name)
        .map(str::to_owned)
        .ok_or_else(|| {
            gateway_v2::data_conversion_error(format!("missing required {display_name} header"))
        })
}

/// Binary point-key metadata for a logical partition request.
///
/// Range targets omit this payload and use separate EPK routing-hash tokens.
enum EpkPayload {
    Point(Vec<u8>),
}

/// Copies a precomputed, nonempty logical partition key into its wire payload, or returns `None`.
fn effective_partition_key_payload(
    inputs: &WrapInputs<'_>,
) -> azure_core::Result<Option<EpkPayload>> {
    let Some(effective_partition_key) = inputs.effective_partition_key else {
        return Ok(None);
    };

    // An empty EPK (no partition key supplied) is not a routable point key.
    if effective_partition_key.as_bytes().is_empty() {
        return Ok(None);
    }

    // The pipeline supplies a computed EPK only for logical targets, never range leaves.
    Ok(Some(EpkPayload::Point(
        effective_partition_key.as_bytes().to_vec(),
    )))
}

struct ResourceNames {
    database: String,
    collection: String,
    document: Option<String>,
}

/// Extracts database, collection, and optional document names, requiring the first two to be present.
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
        gateway_v2::data_conversion_error("Gateway 2.0 resource link is missing database name")
    })?;
    let collection = collection
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            gateway_v2::data_conversion_error(
                "Gateway 2.0 resource link is missing collection name",
            )
        })?;

    Ok(ResourceNames {
        database,
        collection,
        document,
    })
}

/// Decodes a lower- or upper-hex EPK string from `x-ms-start-epk` /
/// `x-ms-end-epk` HTTP headers into the raw bytes passed to
/// `StartEpkHash` / `EndEpkHash`.
/// Returns `None` on invalid input — the dispatcher then omits the token
/// rather than failing the request.
fn decode_epk_hex(hex: &str) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    let mut out = Vec::with_capacity(hex.len() / 2);
    for pair in hex.as_bytes().chunks_exact(2) {
        let hi = hex_nibble(pair[0])?;
        let lo = hex_nibble(pair[1])?;
        out.push((hi << 4) | lo);
    }
    Some(out)
}

/// Converts one ASCII hexadecimal digit to its numeric value, rejecting other bytes.
fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// `RntbdSupportedSerializationFormats` flag bits (OR-combined), matching the
/// server-side enum.
mod supported_serialization_format_flags {
    pub(super) const JSON_TEXT: u8 = 0x01;
    pub(super) const COSMOS_BINARY: u8 = 0x02;
    pub(super) const HYBRID_ROW: u8 = 0x04;
}

/// Parses the `x-ms-cosmos-supported-serialization-formats` accept-list into the
/// RNTBD flags byte.
///
/// The header is a comma-separated, case-insensitive list of format names. Any
/// `+`-suffixed feature (e.g. `CosmosBinary+UInt64`) rides a separate token, so
/// only the base name is matched; unknown entries are skipped (result `0`).
pub(super) fn parse_supported_serialization_formats(value: &str) -> u8 {
    use supported_serialization_format_flags::{COSMOS_BINARY, HYBRID_ROW, JSON_TEXT};
    let mut flags = 0u8;
    for entry in value.split(',') {
        let name = entry.trim().split('+').next().unwrap_or("").trim();
        if name.eq_ignore_ascii_case("JsonText") {
            flags |= JSON_TEXT;
        } else if name.eq_ignore_ascii_case("CosmosBinary") {
            flags |= COSMOS_BINARY;
        } else if name.eq_ignore_ascii_case("HybridRow") {
            flags |= HYBRID_ROW;
        }
    }
    flags
}
