// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway 2.0 HTTP dispatch helpers.

use azure_core::{
    error::ErrorKind,
    http::{
        headers::{HeaderName, HeaderValue, Headers, AUTHORIZATION, USER_AGENT},
        Method,
    },
};
use uuid::Uuid;

use crate::{
    models::{
        cosmos_headers::{request_header_names, response_header_names},
        effective_partition_key::EffectivePartitionKey,
        resource_id::decode_rid,
        DefaultConsistencyLevel, OperationType, ResourceType,
    },
    options::ReadConsistencyStrategy,
};

use super::{
    cosmos_headers::SUPPORTED_CAPABILITIES_BITS,
    cosmos_transport_client::{HttpRequest, HttpResponse},
    rntbd::{RntbdRequestFrame, RntbdResponse, Token},
    AuthorizationContext,
};

// Thin `HeaderName` aliases over the canonical wire strings in
// `models::cosmos_headers`, so the dispatcher shares a single source of truth
// with the rest of the crate instead of re-declaring the literals here.
const GLOBAL_DATABASE_ACCOUNT_NAME: HeaderName =
    HeaderName::from_static(request_header_names::GLOBAL_DATABASE_ACCOUNT_NAME);
const X_MS_DOCUMENTDB_COLLECTION_RID: HeaderName =
    HeaderName::from_static(request_header_names::COLLECTION_RID);
const X_MS_ACTIVITY_ID: HeaderName = HeaderName::from_static(request_header_names::ACTIVITY_ID);
const X_MS_DATE: HeaderName = HeaderName::from_static(request_header_names::DATE);
const X_MS_LSN: HeaderName = HeaderName::from_static(response_header_names::MS_LSN);
const X_MS_GLOBAL_COMMITTED_LSN: HeaderName =
    HeaderName::from_static(response_header_names::GLOBAL_COMMITTED_LSN);
const X_MS_CONTINUATION: HeaderName = HeaderName::from_static(request_header_names::CONTINUATION);
const X_MS_SESSION_TOKEN: HeaderName = HeaderName::from_static(request_header_names::SESSION_TOKEN);
const X_MS_MAX_ITEM_COUNT: HeaderName =
    HeaderName::from_static(request_header_names::MAX_ITEM_COUNT);
const IF_MATCH: HeaderName = HeaderName::from_static(request_header_names::IF_MATCH);
const IF_NONE_MATCH: HeaderName = HeaderName::from_static(request_header_names::IF_NONE_MATCH);
const X_MS_VERSION: HeaderName = HeaderName::from_static(request_header_names::VERSION);
const CACHE_CONTROL: HeaderName = HeaderName::from_static(request_header_names::CACHE_CONTROL);

// Gateway 2.0 (thin-client) outer header aliases, sharing the canonical wire
// strings in `models::cosmos_headers` rather than re-declaring the literals.
const GATEWAY_V2_OPERATION_TYPE: HeaderName =
    HeaderName::from_static(request_header_names::THINCLIENT_PROXY_OPERATION_TYPE);
const GATEWAY_V2_RESOURCE_TYPE: HeaderName =
    HeaderName::from_static(request_header_names::THINCLIENT_PROXY_RESOURCE_TYPE);
const GATEWAY_V2_RANGE_MIN: HeaderName =
    HeaderName::from_static(request_header_names::THINCLIENT_RANGE_MIN);
const GATEWAY_V2_RANGE_MAX: HeaderName =
    HeaderName::from_static(request_header_names::THINCLIENT_RANGE_MAX);
const GATEWAY_V2_DISCOVERY_OPT_IN: HeaderName =
    HeaderName::from_static(request_header_names::USE_THINCLIENT);

/// Inputs resolved by the operation pipeline before a Gateway 2.0 dispatch.
pub(crate) struct WrapInputs<'a> {
    pub(crate) auth_context: &'a AuthorizationContext,
    pub(crate) operation_type: OperationType,
    pub(crate) resource_type: ResourceType,
    /// Effective partition key precomputed in the operation pipeline (point or
    /// prefix EPK). `None` for cross-partition or partition-key-less requests.
    pub(crate) effective_partition_key: Option<&'a EffectivePartitionKey>,
    pub(crate) effective_consistency: DefaultConsistencyLevel,
    /// When non-`Default`, emit the `ReadConsistencyStrategy` RNTBD token (`0x00FE`)
    /// and SUPPRESS the `ConsistencyLevel` token. Callers must already have gated
    /// this to read operations.
    pub(crate) read_consistency_strategy: ReadConsistencyStrategy,
    pub(crate) account_name: Option<&'a str>,
    pub(crate) collection_rid: Option<&'a str>,
}

/// Wraps a signed Cosmos HTTP request into a Gateway 2.0 RNTBD request frame.
pub(crate) fn wrap_request_for_gateway_v2(
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
        // Try the public `x-ms-start-epk`/`x-ms-end-epk` headers first
        // (narrowed-range XPK / scoped reads via `FeedRange`). If absent,
        // fall back to the internal `x-ms-thinclient-pkrange-min`/`-max`
        // headers that carry the full physical pkrange bounds (set by
        // `OperationOverrides::apply_headers` for the full-pkrange XPK
        // fan-out path, where the public EPK headers must NOT be emitted —
        // the standard gateway rejects public EPK headers paired with
        // `partitionkeyrangeid` with HTTP 400, regardless of the min bound).
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
        let decoded = decode_rid(rid)
            .map_err(|e| data_conversion_error(format!("invalid collection RID: {e}")))?;
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
    let match_header = match inputs.operation_type {
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
    let mut frame_bytes = Vec::new();
    frame.write(&mut frame_bytes)?;

    let mut headers = Headers::new();
    if let Some(user_agent) = request.headers.get_optional_str(&USER_AGENT) {
        headers.insert(USER_AGENT, HeaderValue::from(user_agent.to_owned()));
    }
    headers.insert(X_MS_ACTIVITY_ID, HeaderValue::from(activity_id.to_string()));
    // Forward x-ms-version (defaults to CURRENT_VERSION = 2020-07-15)
    // and Cache-Control. The proxy requires x-ms-version to dispatch the request.
    if let Some(version) = request.headers.get_optional_str(&X_MS_VERSION) {
        headers.insert(X_MS_VERSION, HeaderValue::from(version.to_owned()));
    } else {
        headers.insert(X_MS_VERSION, HeaderValue::from("2020-07-15"));
    }
    headers.insert(CACHE_CONTROL, HeaderValue::from("no-cache"));
    headers.insert(GATEWAY_V2_DISCOVERY_OPT_IN, HeaderValue::from("true"));
    headers.insert(
        GATEWAY_V2_OPERATION_TYPE,
        HeaderValue::from(proxy_operation_type_name(inputs.operation_type)),
    );
    headers.insert(
        GATEWAY_V2_RESOURCE_TYPE,
        HeaderValue::from(proxy_resource_type_name(inputs.resource_type)),
    );
    headers.insert(
        GLOBAL_DATABASE_ACCOUNT_NAME,
        HeaderValue::from(account_name.to_owned()),
    );
    if let Some(rid) = inputs.collection_rid.filter(|s| !s.is_empty()) {
        headers.insert(
            X_MS_DOCUMENTDB_COLLECTION_RID,
            HeaderValue::from(rid.to_owned()),
        );
    }
    if inputs.operation_type == OperationType::Query
        && !matches!(epk_payload.as_ref(), Some(EpkPayload::Point(_)))
    {
        // For cross-partition Query (no partition-key-scoped EPK), these
        // headers are set to the literal `"true"`
        // to signal the proxy that the EPK boundaries are carried in the
        // RNTBD body as `StartEpkHash`/`EndEpkHash` tokens. For partition-
        // scoped queries (point or prefix EPK on the body), the proxy uses
        // the `EffectivePartitionKey` token directly and these flag headers
        // must NOT be sent — otherwise the proxy looks for absent
        // Start/End EPK hash tokens and rejects the request with a bare 400.
        headers.insert(GATEWAY_V2_RANGE_MIN, HeaderValue::from("true"));
        headers.insert(GATEWAY_V2_RANGE_MAX, HeaderValue::from("true"));
    }
    // The G2 wrap synthesizes a fresh HTTP header set carrying only what the
    // proxy needs (the RNTBD frame is in the body). The fault-injection
    // operation tag must be forwarded so `FaultInjectingHttpClient` can match
    // operation-typed rules against the G2 path the same way it matches V1.
    #[cfg(feature = "fault_injection")]
    {
        use crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_OPERATION;
        let fault_op_header = HeaderName::from_static(FAULT_INJECTION_OPERATION);
        if let Some(op) = request.headers.get_optional_str(&fault_op_header) {
            headers.insert(fault_op_header, HeaderValue::from(op.to_owned()));
        }
    }

    let url = request.url.clone();
    Ok(HttpRequest {
        url,
        method: Method::Post,
        headers,
        body: Some(bytes::Bytes::from(frame_bytes)),
        timeout: request.timeout,
        #[cfg(feature = "fault_injection")]
        evaluation_collector: request.evaluation_collector.clone(),
    })
}

/// Decodes a Gateway 2.0 RNTBD response body into a synthetic HTTP response.
pub(crate) fn unwrap_response_for_gateway_v2(
    response: HttpResponse,
) -> azure_core::Result<HttpResponse> {
    let response = RntbdResponse::read(&response.body)?;
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
        // GW2 surfaces only the vector portion of the session token, with the
        // partition key range id carried separately. Classic gateway emits
        // `<pkRangeId>:<vector>`, and the backend rejects a bare vector with a
        // terminal 400 on re-supply, so re-compose the composite form here.
        let composite = match &response.partition_key_range_id {
            Some(pk_range_id) if !token.contains(':') => format!("{pk_range_id}:{token}"),
            _ => token,
        };
        headers.insert(response_header_names::SESSION_TOKEN, composite);
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

// PascalCase names the Gateway 2.0 proxy expects for the
// `x-ms-thinclient-proxy-operation-type` routing header. Match
// `com.azure.cosmos.implementation.OperationType` so the proxy can
// dispatch the request.
fn proxy_operation_type_name(op: OperationType) -> &'static str {
    match op {
        OperationType::Create => "Create",
        OperationType::Read => "Read",
        OperationType::ReadFeed => "ReadFeed",
        OperationType::Replace => "Replace",
        OperationType::Delete => "Delete",
        OperationType::Upsert => "Upsert",
        OperationType::Query => "Query",
        OperationType::SqlQuery => "SqlQuery",
        OperationType::QueryPlan => "QueryPlan",
        OperationType::Batch => "Batch",
        OperationType::Head => "Head",
        OperationType::HeadFeed => "HeadFeed",
        OperationType::Execute => "ExecuteJavaScript",
        OperationType::Patch => "Patch",
        // Distributed transactions never reach Gateway 2.0 dispatch (rejected
        // by `is_operation_supported_by_gateway_v2`); they route through the
        // standard gateway coordinator.
        #[cfg(feature = "preview_dtx")]
        OperationType::CommitDistributedTransaction | OperationType::ReadDistributedTransaction => {
            unreachable!("distributed transaction operations must not reach Gateway 2.0 dispatch")
        }
    }
}

// PascalCase names the Gateway 2.0 proxy expects for the
// `x-ms-thinclient-proxy-resource-type` routing header. Match
// `com.azure.cosmos.implementation.ResourceType`.
fn proxy_resource_type_name(rt: ResourceType) -> &'static str {
    match rt {
        ResourceType::DatabaseAccount => "DatabaseAccount",
        ResourceType::Database => "Database",
        ResourceType::DocumentCollection => "DocumentCollection",
        ResourceType::Document => "Document",
        ResourceType::StoredProcedure => "StoredProcedure",
        ResourceType::Trigger => "Trigger",
        ResourceType::UserDefinedFunction => "UserDefinedFunction",
        ResourceType::PartitionKeyRange => "PartitionKeyRange",
        ResourceType::Offer => "Offer",
        #[cfg(feature = "preview_dtx")]
        ResourceType::DistributedTransactionBatch => {
            unreachable!("distributed transaction batch must not reach Gateway 2.0 dispatch")
        }
    }
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
/// canonical, un-padded hex EPK string.
/// Encapsulates the EPK information for a Gateway 2.0 request.
///
/// The proxy routes by `EffectivePartitionKey` (binary EPK token in the RNTBD
/// body) when a partition key is present (point EPK for full keys, prefix EPK
/// for partial-HPK keys). For cross-partition requests with no partition key,
/// the dispatcher instead emits `StartEpkHash`/`EndEpkHash` body tokens
/// (sourced from the request's `x-ms-start-epk`/`x-ms-end-epk` HTTP headers)
/// — that path doesn't use this enum.
enum EpkPayload {
    Point(Vec<u8>),
}

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

    // The EPK was computed in the operation pipeline via the shared routing-core
    // routine, which selects the v1/v2/multi-hash binary encoder by the
    // collection's partition-key kind/version. For partial-HPK keys this is a
    // prefix EPK (16 * N bytes for N supplied components) which the proxy treats
    // as a partition-key prefix scope. Emitted whenever a partition key is
    // present, point or prefix.
    Ok(Some(EpkPayload::Point(
        effective_partition_key.as_bytes().to_vec(),
    )))
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

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, collections::HashMap};

    use azure_core::http::headers::{ACCEPT, CONTENT_TYPE};

    use super::*;
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
    ) -> WrapInputs<'a> {
        WrapInputs {
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

    #[test]
    fn wrap_emits_tokens_in_thin_client_required_order() {
        // The thin-client contract mandates the relative ordering
        // EffectivePartitionKey -> GlobalDatabaseAccountName -> PayloadPresent
        // on every thin-client request (CorrelatedActivityId is not emitted by
        // the Rust driver). The emission order is hard-coded in
        // `wrap_request_for_gateway_v2`; pin the contract here so a
        // future refactor that shuffles the token order is a compile-time
        // failure rather than a silent wire-compat break.
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let partition_key = PartitionKey::from("pk-value");
        let partition_key_definition = PartitionKeyDefinition::new(vec![Cow::from("/pk")]);

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Read,
                Some(&epk(&partition_key, &partition_key_definition)),
            ),
        )
        .unwrap();

        // Read the metadata token IDs in stream order (don't go through
        // `parse_wrapped_request` which puts tokens into a HashMap and
        // loses ordering).
        let mut src = wrapped.body.as_ref().unwrap().as_ref();
        let _total_len = take_u32(&mut src);
        let _resource_type = take_u16(&mut src);
        let _operation_type = take_u16(&mut src);
        let _activity_id = take_uuid(&mut src);

        let mut emitted_ids = Vec::new();
        // Stop when the remaining bytes can't form a token header; the
        // body (if any) comes last and is too short to be confused with a
        // token frame here (a Read has no body).
        while src.len() >= 3 {
            let id = take_u16(&mut src);
            emitted_ids.push(id);
            let token_type = take_u8(&mut src);
            // Consume the value bytes so the next iteration starts at the
            // next token header. Mirrors `parse_token_value` for the
            // token types this test fixture actually produces.
            let _ = parse_token_value(token_type, &mut src);
        }
        assert!(
            src.is_empty(),
            "Read should have no inner body; leftover bytes {:?}",
            src
        );

        let pos = |id: u16| -> usize {
            emitted_ids
                .iter()
                .position(|&x| x == id)
                .unwrap_or_else(|| panic!("token 0x{id:04X} not emitted; got {emitted_ids:?}"))
        };
        let epk = pos(0x005A); // EffectivePartitionKey
        let global_account = pos(0x00CE); // GlobalDatabaseAccountName
        let payload_present = pos(0x0002); // PayloadPresent

        assert!(
            epk < global_account,
            "EffectivePartitionKey (0x005A) must precede GlobalDatabaseAccountName (0x00CE) per thin-client contract; got {emitted_ids:?}"
        );
        assert!(
            global_account < payload_present,
            "GlobalDatabaseAccountName (0x00CE) must precede PayloadPresent (0x0002) per thin-client contract; got {emitted_ids:?}"
        );
    }

    #[cfg(feature = "fault_injection")]
    #[test]
    fn wrap_forwards_fault_injection_operation_header() {
        // Regression: the G2 wrap builds a fresh `Headers` map and used to
        // drop `x-ms-fault-injection-operation`, causing
        // `FaultInjectingHttpClient` to report `OperationMismatch` for any
        // operation-typed rule on G2 traffic even when the V1 path matched.
        use crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_OPERATION;
        let mut request = signed_request(None);
        request.headers.insert(
            HeaderName::from_static(FAULT_INJECTION_OPERATION),
            "ReadItem",
        );
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        assert_eq!(
            wrapped
                .headers
                .get_optional_str(&HeaderName::from_static(FAULT_INJECTION_OPERATION))
                .map(|s| s.to_owned()),
            Some("ReadItem".to_owned()),
            "G2 wrap must forward the fault-injection operation header so FaultInjectingHttpClient can match operation-typed rules"
        );
    }

    #[test]
    fn wrap_builds_required_request_tokens_for_read() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
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
        // TransportRequestId (0x004D) is intentionally NOT emitted on the G2
        // path — it is in the thin-client exclusion list and the proxy assigns
        // its own request id.
        assert!(!parsed.tokens.contains_key(&0x004D));
        assert_eq!(
            parsed.tokens[&0x00A2],
            ParsedTokenValue::ULong(SUPPORTED_CAPABILITIES_BITS)
        );
        assert_eq!(
            parsed.tokens[&0x00CE],
            ParsedTokenValue::String("account".into())
        );
    }

    /// End-to-end regression: the full G2 wrap pipeline produces a frame
    /// whose first 4 bytes (`LengthInBytes`) cover only the header section
    /// and exclude the body length prefix + body bytes. This is the exact
    /// property the Gateway 2.0 proxy reads — getting it wrong is what
    /// caused HTTP 400 sub-status 13007 ("error routing the request") on
    /// every G2 Create request in CI. Pin it at the wrap-function boundary
    /// in addition to the lower-level `RntbdRequestFrame::serialize` tests
    /// so a future change to the wrap function that adds bytes between the
    /// header and the body cannot regress this.
    #[test]
    fn wrap_writes_length_in_bytes_covering_header_only_for_create_with_payload() {
        let payload = br#"{"id":"doc1","pk":"abc","data":"hello"}"#;
        let request = signed_request(Some(payload));
        let auth_context =
            AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Create, None),
        )
        .unwrap();

        let bytes = wrapped.body.as_ref().unwrap().as_ref();
        let length_in_bytes = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        // The `LengthInBytes` field must be strictly less than the frame size
        // (body prefix + body bytes follow). If it ever equals `bytes.len()`,
        // the proxy will treat the body as additional metadata tokens and
        // fail with sub-status 13007.
        assert!(
            length_in_bytes < bytes.len(),
            "LengthInBytes ({length_in_bytes}) must exclude body section but equals total frame size ({})",
            bytes.len()
        );
        // And the body-length prefix immediately at offset = header_len must
        // match the actual payload length the proxy receives.
        let body_len = u32::from_le_bytes([
            bytes[length_in_bytes],
            bytes[length_in_bytes + 1],
            bytes[length_in_bytes + 2],
            bytes[length_in_bytes + 3],
        ]) as usize;
        assert_eq!(body_len, payload.len());
        assert_eq!(bytes.len(), length_in_bytes + 4 + payload.len());
    }

    #[test]
    fn wrap_omits_allow_tentative_writes_and_return_preference_by_default() {
        // Regression: `ReturnPreference=1` is only set
        // when the caller passes `Prefer: return=minimal`; otherwise the token
        // is absent and the server returns the full body. Likewise
        // `AllowTentativeWrites` is only set when the caller passes the
        // `x-ms-cosmos-allow-tentative-writes` header.
        //
        // Pinning the conditional path here: emitting these tokens
        // unconditionally tells the server to drop response bodies (and on
        // Create silently discards the request body server-side, persisting
        // a phantom 0-byte document).
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);

        assert!(
            !parsed.tokens.contains_key(&0x0066),
            "AllowTentativeWrites (0x0066) must NOT be emitted by default"
        );
        assert!(
            !parsed.tokens.contains_key(&0x0082),
            "ReturnPreference (0x0082) must NOT be emitted by default"
        );
    }

    #[test]
    fn wrap_emits_return_preference_when_prefer_return_minimal_header_is_set() {
        // Emit `ReturnPreference=1`
        // when the request carries `Prefer: return=minimal` and only then.
        let mut request = signed_request(None);
        request
            .headers
            .insert(HeaderName::from_static("prefer"), "return=minimal");
        let auth_context = AuthorizationContext::new(
            Method::Post,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs",
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Create, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);

        assert_eq!(parsed.tokens[&0x0082], ParsedTokenValue::Byte(1));
        assert!(
            !parsed.tokens.contains_key(&0x0066),
            "AllowTentativeWrites must remain absent when only Prefer is set"
        );
    }

    #[test]
    fn wrap_emits_allow_tentative_writes_when_header_is_set_to_true() {
        // Emit `AllowTentativeWrites=1` only when the caller forwards the
        // `x-ms-cosmos-allow-tentative-writes` header.
        let mut request = signed_request(None);
        request.headers.insert(
            HeaderName::from_static("x-ms-cosmos-allow-tentative-writes"),
            "true",
        );
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);

        assert_eq!(parsed.tokens[&0x0066], ParsedTokenValue::Byte(1));
        assert!(
            !parsed.tokens.contains_key(&0x0082),
            "ReturnPreference must remain absent when only allow-tentative-writes is set"
        );
    }

    #[test]
    fn wrap_emits_collection_rid_and_decoded_resource_id_when_supplied() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        for (rid, expected_resource_id) in [
            (
                "-NY+AJoR+cc=",
                [0xfc, 0xd6, 0x3e, 0x00, 0x9a, 0x11, 0xf9, 0xc7],
            ),
            (
                "YpNuAIVuY-0=",
                [0x62, 0x93, 0x6e, 0x00, 0x85, 0x6e, 0x63, 0xfd],
            ),
        ] {
            let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
            inputs.collection_rid = Some(rid);

            let wrapped = wrap_request_for_gateway_v2(&request, &inputs).unwrap();
            let parsed = parse_wrapped_request(&wrapped, 0);

            assert_eq!(
                parsed.tokens[&0x0035],
                ParsedTokenValue::String(rid.into()),
                "CollectionRid (0x0035) must preserve the wire value"
            );
            match &parsed.tokens[&0x0000] {
                ParsedTokenValue::Bytes(bytes) => assert_eq!(bytes, &expected_resource_id),
                other => panic!("expected ResourceId bytes, got {other:?}"),
            }
        }
    }

    #[test]
    fn wrap_rejects_invalid_collection_rid() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
        inputs.collection_rid = Some("invalid!");

        let err = wrap_request_for_gateway_v2(&request, &inputs).unwrap_err();
        assert!(err.to_string().contains("invalid collection RID"));
    }

    #[test]
    fn wrap_omits_collection_rid_and_resource_id_when_not_supplied() {
        // The collection rid is optional from the operation pipeline; pin
        // that absence means neither token is emitted (rather than e.g. an
        // empty string token which the proxy would reject).
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);

        assert!(!parsed.tokens.contains_key(&0x0035));
        assert!(!parsed.tokens.contains_key(&0x0000));
    }

    #[test]
    fn wrap_emits_partition_key_range_id_token_when_http_header_present() {
        // Regression: cross-partition Query routes via StartEpkHash/EndEpkHash
        // bytes, but the thin-client proxy ALSO requires the
        // PartitionKeyRangeId (0x002C, String) RNTBD token to disambiguate
        // the target physical partition. This is filled from the
        // `x-ms-documentdb-partitionkeyrangeid` HTTP header. Without it, the proxy
        // misinterprets an empty StartEpkHash as a `[]` partition-key value
        // and rejects with "PartitionKey supplied in x-ms-partitionkey header
        // has fewer components than defined in the the collection."
        let mut request = signed_request(None);
        request.headers.insert(
            HeaderName::from_static(request_header_names::PARTITION_KEY_RANGE_ID),
            "0",
        );
        let auth_context = AuthorizationContext::new(
            Method::Post,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs",
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Query, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);

        assert_eq!(
            parsed.tokens[&0x002C],
            ParsedTokenValue::String("0".into()),
            "PartitionKeyRangeId (0x002C) must be emitted from the HTTP header"
        );
    }

    #[test]
    fn wrap_omits_partition_key_range_id_token_when_http_header_absent() {
        // Pin the conditional emission path: no header => no token (rather
        // than an empty-string token, which the proxy would treat as a real
        // routing key for partition id "").
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);

        assert!(!parsed.tokens.contains_key(&0x002C));
    }

    #[test]
    fn wrap_emits_session_token_when_http_header_present() {
        // Session-consistent reads require the client's per-partition LSN
        // progress so the backend can pick a caught-up replica. Java's
        // thin-client encoder forwards it as the SessionToken (0x0005, String)
        // RNTBD token, filled from the `x-ms-session-token` header.
        let mut request = signed_request(None);
        request.headers.insert(
            HeaderName::from_static(request_header_names::SESSION_TOKEN),
            "0:1#42",
        );
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);

        assert_eq!(
            parsed.tokens[&0x0005],
            ParsedTokenValue::String("0:1#42".into()),
            "SessionToken (0x0005) must be emitted from the HTTP header"
        );
    }

    #[test]
    fn wrap_omits_session_token_when_http_header_absent_or_empty() {
        // No header => no token. An empty value is also skipped so the proxy
        // never receives a zero-length session token it would have to reject.
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        assert!(!parse_wrapped_request(&wrapped, 0)
            .tokens
            .contains_key(&0x0005));

        let mut empty = signed_request(None);
        empty.headers.insert(
            HeaderName::from_static(request_header_names::SESSION_TOKEN),
            "",
        );
        let wrapped_empty = wrap_request_for_gateway_v2(
            &empty,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        assert!(!parse_wrapped_request(&wrapped_empty, 0)
            .tokens
            .contains_key(&0x0005));
    }

    #[test]
    fn wrap_emits_page_size_from_max_item_count_header() {
        // Query/read-feed page size must reach the proxy as PageSize (0x0004,
        // ULong), filled from `x-ms-max-item-count`. A negative (unbounded)
        // request is encoded as 0xFFFFFFFF.
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );

        let mut request = signed_request(None);
        request.headers.insert(
            HeaderName::from_static(request_header_names::MAX_ITEM_COUNT),
            "100",
        );
        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
        )
        .unwrap();
        assert_eq!(
            parse_wrapped_request(&wrapped, 0).tokens[&0x0004],
            ParsedTokenValue::ULong(100),
            "PageSize (0x0004) must be emitted from x-ms-max-item-count"
        );

        let mut unbounded = signed_request(None);
        unbounded.headers.insert(
            HeaderName::from_static(request_header_names::MAX_ITEM_COUNT),
            "-1",
        );
        let wrapped_unbounded = wrap_request_for_gateway_v2(
            &unbounded,
            &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
        )
        .unwrap();
        assert_eq!(
            parse_wrapped_request(&wrapped_unbounded, 0).tokens[&0x0004],
            ParsedTokenValue::ULong(0xFFFF_FFFF),
            "a negative max item count must encode as 0xFFFFFFFF"
        );
    }

    #[test]
    fn wrap_omits_page_size_when_header_absent() {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
        )
        .unwrap();
        assert!(!parse_wrapped_request(&wrapped, 0)
            .tokens
            .contains_key(&0x0004));
    }

    #[test]
    fn wrap_emits_match_from_if_match_on_writes() {
        // Writes carry the ETag precondition as Match (0x0008, String) filled
        // from `If-Match`; `If-None-Match` is ignored on a write.
        let auth_context = AuthorizationContext::new(
            Method::Put,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let mut request = signed_request(Some(b"{}"));
        request.headers.insert(
            HeaderName::from_static(request_header_names::IF_MATCH),
            "\"etag-42\"",
        );
        request.headers.insert(
            HeaderName::from_static(request_header_names::IF_NONE_MATCH),
            "\"ignored\"",
        );
        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Replace, None),
        )
        .unwrap();
        assert_eq!(
            parse_wrapped_request(&wrapped, 0).tokens[&0x0008],
            ParsedTokenValue::String("\"etag-42\"".into()),
            "Match (0x0008) on a write must come from If-Match"
        );
    }

    #[test]
    fn wrap_emits_match_from_if_none_match_on_reads() {
        // Reads/read-feeds carry the conditional-GET precondition as Match
        // (0x0008) filled from `If-None-Match`; `If-Match` is ignored on reads.
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let mut request = signed_request(None);
        request.headers.insert(
            HeaderName::from_static(request_header_names::IF_NONE_MATCH),
            "\"etag-7\"",
        );
        request.headers.insert(
            HeaderName::from_static(request_header_names::IF_MATCH),
            "\"ignored\"",
        );
        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
        )
        .unwrap();
        assert_eq!(
            parse_wrapped_request(&wrapped, 0).tokens[&0x0008],
            ParsedTokenValue::String("\"etag-7\"".into()),
            "Match (0x0008) on a read must come from If-None-Match"
        );
    }

    #[test]
    fn wrap_preserves_payload_and_sets_payload_present() {
        let request = signed_request(Some(br#"{"id":"doc1"}"#));
        let auth_context =
            AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Create, None),
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

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Create, None),
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
        let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
        inputs.effective_consistency = DefaultConsistencyLevel::Eventual;

        let wrapped = wrap_request_for_gateway_v2(&request, &inputs).unwrap();
        let parsed = parse_wrapped_request(&wrapped, 10);

        assert_eq!(parsed.tokens[&0x0010], ParsedTokenValue::Byte(0x03));
    }

    /// Rule 1+5 (RCS resolution):
    /// non-`Default` RCS on the wrap path emits token `0x00FE` and SUPPRESSES
    /// the legacy `ConsistencyLevel` token `0x0010`.
    #[test]
    fn wrap_emits_read_consistency_strategy_token_and_drops_consistency_level() {
        use crate::options::ReadConsistencyStrategy;

        let cases = [
            (ReadConsistencyStrategy::Eventual, 0x01u8),
            (ReadConsistencyStrategy::Session, 0x02u8),
            (ReadConsistencyStrategy::LatestCommitted, 0x03u8),
            (ReadConsistencyStrategy::GlobalStrong, 0x04u8),
        ];

        for (strategy, expected_byte) in cases {
            let request = signed_request(None);
            let auth_context = AuthorizationContext::new(
                Method::Get,
                ResourceType::Document,
                "dbs/db1/colls/coll1/docs/doc1",
            );
            let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
            inputs.read_consistency_strategy = strategy;

            let wrapped = wrap_request_for_gateway_v2(&request, &inputs).unwrap();
            let parsed = parse_wrapped_request(&wrapped, 10);

            assert_eq!(
                parsed.tokens[&0x00FE],
                ParsedTokenValue::Byte(expected_byte),
                "ReadConsistencyStrategy {strategy:?} should serialize to byte {expected_byte:#x}"
            );
            assert!(
                !parsed.tokens.contains_key(&0x0010),
                "ConsistencyLevel token (0x0010) must be omitted when RCS={strategy:?} is non-Default"
            );
        }
    }

    /// Rule 3: `Default` RCS is transparent — wrap behaves identically to the
    /// pre-RCS world, emitting only `ConsistencyLevel`.
    #[test]
    fn wrap_with_default_rcs_emits_only_consistency_level_token() {
        use crate::options::ReadConsistencyStrategy;

        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
        inputs.read_consistency_strategy = ReadConsistencyStrategy::Default;
        inputs.effective_consistency = DefaultConsistencyLevel::Session;

        let wrapped = wrap_request_for_gateway_v2(&request, &inputs).unwrap();
        let parsed = parse_wrapped_request(&wrapped, 10);

        assert_eq!(parsed.tokens[&0x0010], ParsedTokenValue::Byte(0x02));
        assert!(!parsed.tokens.contains_key(&0x00FE));
    }

    #[test]
    fn wrap_emits_v2_binary_effective_partition_key_for_v2_collections() {
        // For collections with PartitionKey version=V2 (the default), the RNTBD
        // EffectivePartitionKey token (0x005A) must carry the V2 binary form:
        // the raw 16-byte Murmur3_128 hash with the top 2 bits of byte 0
        // cleared. Sending V1 binary against a V2 collection causes the proxy
        // to mis-route (or silently drop the body), persisting a phantom 0-byte
        // document.
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let partition_key = PartitionKey::from("tenant1");
        let partition_key_definition = PartitionKeyDefinition::new(vec![Cow::from("/tenantId")]);
        assert_eq!(
            partition_key_definition.version(),
            PartitionKeyVersion::V2,
            "sanity check: PartitionKeyDefinition::new defaults to V2"
        );
        let expected = effective_partition_key_v2_binary(partition_key.values());

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Read,
                Some(&epk(&partition_key, &partition_key_definition)),
            ),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 11);

        assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
    }

    #[test]
    fn wrap_emits_v1_binary_effective_partition_key_for_v1_collections() {
        // For legacy V1 collections, EPK must remain the V1 binary tuple
        // (`Number(hash) + truncated_components`). Pin the version-aware
        // dispatch so legacy accounts still route correctly.
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let partition_key = PartitionKey::from("tenant1");
        let partition_key_definition = PartitionKeyDefinition::new(vec![Cow::from("/tenantId")])
            .with_version(PartitionKeyVersion::V1);
        let expected = effective_partition_key_v1_binary(partition_key.values());

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Read,
                Some(&epk(&partition_key, &partition_key_definition)),
            ),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 11);

        assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
    }

    #[test]
    fn wrap_emits_v1_binary_effective_partition_key_for_version_less_collections() {
        // A container definition read back from the service with no `version`
        // field is a legacy V1 container (the service omits `version` only for
        // V1). Gateway 2.0 routes point ops on the client-computed EPK, so a
        // version-less container MUST emit the V1 binary EPK token (0x005A),
        // NOT V2 — mis-detecting it as V2 was the root-cause routing bug this
        // guards against.
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let partition_key = PartitionKey::from("tenant1");
        let partition_key_definition: PartitionKeyDefinition =
            serde_json::from_str(r#"{"paths":["/tenantId"]}"#).unwrap();
        assert_eq!(
            partition_key_definition.version(),
            PartitionKeyVersion::V1,
            "sanity check: a version-less definition deserializes to V1"
        );
        let expected = effective_partition_key_v1_binary(partition_key.values());
        assert_ne!(
            expected,
            effective_partition_key_v2_binary(partition_key.values()),
            "sanity check: V1 and V2 EPK encodings differ for this pk value"
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Read,
                Some(&epk(&partition_key, &partition_key_definition)),
            ),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 11);

        assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
    }

    /// HPK partial-PK (prefix on a MultiHash container) emits an
    /// `EffectivePartitionKey` RNTBD token containing the per-component
    /// MultiHash V2 binary for the supplied prefix components (16 * N bytes
    /// for N supplied components). The same token is emitted for point AND
    /// prefix keys whenever a partition key is present on the request.
    #[test]
    fn wrap_emits_prefix_epk_token_for_hpk_prefix_partition_key() {
        let request = signed_request(None);
        let auth_context =
            AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");
        let partition_key =
            PartitionKey::from(vec![PartitionKeyValue::from("tenant1".to_string())]);
        let partition_key_definition =
            PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
        let expected = effective_partition_key_multi_hash_v2_binary(partition_key.values());
        assert_eq!(
            expected.len(),
            16,
            "1-of-3 component prefix must produce a single 16-byte MultiHash V2 chunk"
        );

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Query,
                Some(&epk(&partition_key, &partition_key_definition)),
            ),
        )
        .unwrap();

        // Token layout for a partial-HPK Query: 9 base tokens + EPK = 10.
        let parsed = parse_wrapped_request(&wrapped, 10);
        assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
        // Range headers must NOT be emitted in the EPK-token path — the
        // emission is mutually exclusive: partition key present → EPK token,
        // otherwise → StartEpkHash/EndEpkHash range tokens.
        assert!(
            wrapped
                .headers
                .get_optional_str(&GATEWAY_V2_RANGE_MIN)
                .is_none(),
            "range-min header must not be emitted alongside EPK token"
        );
        assert!(
            wrapped
                .headers
                .get_optional_str(&GATEWAY_V2_RANGE_MAX)
                .is_none(),
            "range-max header must not be emitted alongside EPK token"
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

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(
                &auth_context,
                OperationType::Read,
                Some(&epk(&partition_key, &partition_key_definition)),
            ),
        )
        .unwrap();

        // Range headers must NOT be present on the point path.
        assert!(wrapped
            .headers
            .get_optional_str(&GATEWAY_V2_RANGE_MIN)
            .is_none());
        assert!(wrapped
            .headers
            .get_optional_str(&GATEWAY_V2_RANGE_MAX)
            .is_none());

        // EPK token present in the inner RNTBD frame, and bytes must be the
        // per-component MultiHash V2 concatenation (16 * N bytes for N PK
        // paths). Sending a single 16-byte V2 hash for a MultiHash container
        // causes the proxy to silently drop the body.
        let parsed = parse_wrapped_request(&wrapped, 11);
        let expected = effective_partition_key_multi_hash_v2_binary(partition_key.values());
        assert_eq!(expected.len(), 16 * 3, "sanity check: 3 PK paths * 16B");
        assert_eq!(
            parsed.tokens[&0x005A],
            ParsedTokenValue::Bytes(expected),
            "EffectivePartitionKey token must be per-component MultiHash V2 bytes"
        );
    }

    #[test]
    fn wrap_propagates_continuation_token_into_rntbd_metadata() {
        let mut request = signed_request(None);
        request.headers.insert(X_MS_CONTINUATION, "page-token-1");
        let auth_context =
            AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &WrapInputs {
                auth_context: &auth_context,
                operation_type: OperationType::Query,
                resource_type: ResourceType::Document,
                effective_partition_key: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
                account_name: Some("account"),
                collection_rid: None,
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

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &WrapInputs {
                auth_context: &auth_context,
                operation_type: OperationType::Query,
                resource_type: ResourceType::Document,
                effective_partition_key: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
                account_name: Some("account"),
                collection_rid: None,
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
        // The wrap side forwards empty continuation strings verbatim, matching
        // the unwrap side. Continuation is opaque on the wire — the wrap path
        // does not infer intent from emptiness.
        let mut request = signed_request(None);
        request.headers.insert(X_MS_CONTINUATION, "");
        let auth_context =
            AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &WrapInputs {
                auth_context: &auth_context,
                operation_type: OperationType::Query,
                resource_type: ResourceType::Document,
                effective_partition_key: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
                account_name: Some("account"),
                collection_rid: None,
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

        let wrapped = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
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

        let error = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
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

        let error = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
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

        let error = wrap_request_for_gateway_v2(
            &request,
            &wrap_inputs(&auth_context, OperationType::Read, None),
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

        let unwrapped = unwrap_response_for_gateway_v2(response).unwrap();

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
    fn unwrap_prefixes_session_token_with_partition_key_range_id() {
        let response = HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: response_frame(
                200,
                Uuid::parse_str(ACTIVITY_ID).unwrap(),
                |tokens| {
                    // GW2 surfaces the bare vector token and the pk range id separately.
                    write_string_token(tokens, 0x0021, "0");
                    write_string_token(tokens, 0x003E, "0#2#2=-1");
                },
                b"",
            ),
        };

        let unwrapped = unwrap_response_for_gateway_v2(response).unwrap();

        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-session-token")),
            Some("0:0#2#2=-1")
        );
    }

    #[test]
    fn unwrap_keeps_session_token_when_already_prefixed() {
        let response = HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: response_frame(
                200,
                Uuid::parse_str(ACTIVITY_ID).unwrap(),
                |tokens| {
                    write_string_token(tokens, 0x0021, "0");
                    write_string_token(tokens, 0x003E, "0:0#2#2=-1");
                },
                b"",
            ),
        };

        let unwrapped = unwrap_response_for_gateway_v2(response).unwrap();

        assert_eq!(
            unwrapped
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-session-token")),
            Some("0:0#2#2=-1")
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

        let unwrapped = unwrap_response_for_gateway_v2(response).unwrap();

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

        let error = unwrap_response_for_gateway_v2(response).unwrap_err();

        assert_eq!(error.kind(), &ErrorKind::DataConversion);
    }

    #[test]
    fn unwrap_rejects_out_of_range_inner_status() {
        let response = HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: response_frame(70_000, Uuid::parse_str(ACTIVITY_ID).unwrap(), |_| {}, b""),
        };

        let error = unwrap_response_for_gateway_v2(response).unwrap_err();

        assert_eq!(error.kind(), &ErrorKind::DataConversion);
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
