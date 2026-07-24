// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore misroute misrouted
//! Request parsing, URL routing, and operation resolution.

use azure_core::http::headers::HeaderName;
use azure_core::http::Request;
use percent_encoding::percent_decode_str;

/// The type of operation resolved from an HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OperationType {
    ReadAccount,
    CreateDatabase,
    ReadFeedDatabases,
    QueryDatabases,
    ReadDatabase,
    DeleteDatabase,
    CreateContainer,
    ReadFeedContainers,
    QueryContainers,
    ReadContainer,
    DeleteContainer,
    ReadPKRanges,
    ReadFeedItems,
    Create,
    Read,
    Replace,
    Upsert,
    Delete,
    QueryItems,
    QueryPlan,
    Batch,
    ReadFeedOffers,
    QueryOffers,
    ReadOffer,
    ReplaceOffer,
    #[cfg(feature = "preview_dtx")]
    DistributedTransaction,
    Unsupported(String),
    /// Trailing-slash on a resource URL. Real Cosmos returns 400 BadRequest
    /// for these (not 501) — keep the variant separate so the handler can
    /// emit the right status and substatus.
    BadRequestPath(String),
    /// A request whose headers are internally inconsistent — for example an
    /// EPK-range-scoped read (`x-ms-start-epk`/`x-ms-end-epk`) that declares
    /// the point key type instead of `EffectivePartitionKeyRange`. Real Cosmos
    /// rejects these with `400 "One of the input values is invalid"`.
    InvalidInput(String),
}

/// Parsed request data extracted from an HTTP request.
#[derive(Debug, Clone)]
pub(crate) struct ParsedRequest {
    pub operation: OperationType,
    pub db_id: Option<String>,
    pub coll_id: Option<String>,
    pub doc_id: Option<String>,
    pub offer_id: Option<String>,
    pub partition_key_header: Option<String>,
    pub if_match: Option<String>,
    pub if_none_match: Option<String>,
    pub session_token: Option<String>,
    pub activity_id: Option<String>,
    pub content_response_on_write: bool,
    /// Provisioned RU/s parsed from the `x-ms-offer-throughput` request header.
    /// Forwarded to container creation so the emulator honors caller-specified
    /// throughput instead of silently falling back to `ContainerConfig::default()`
    /// (which has no provisioned RU/s and disables throttling for the container).
    pub offer_throughput: Option<u32>,
    #[allow(dead_code)]
    pub offer_autopilot_settings: Option<String>,
    #[allow(dead_code)]
    pub max_item_count: Option<i32>,
    #[allow(dead_code)]
    pub continuation: Option<String>,
    #[allow(dead_code)]
    pub partition_key_range_id: Option<String>,
    #[allow(dead_code)]
    pub start_epk: Option<String>,
    #[allow(dead_code)]
    pub end_epk: Option<String>,
    #[allow(dead_code)]
    pub is_query_plan: bool,
    #[allow(dead_code)]
    pub is_batch: bool,
    #[allow(dead_code)]
    pub is_upsert: bool, // used during dispatch resolution
}

// Header name constants for request parsing
static IS_UPSERT: HeaderName = HeaderName::from_static("x-ms-documentdb-is-upsert");
static PARTITION_KEY: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkey");
static IF_MATCH: HeaderName = HeaderName::from_static("if-match");
static IF_NONE_MATCH: HeaderName = HeaderName::from_static("if-none-match");
static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
static CONTENT_RESPONSE: HeaderName =
    HeaderName::from_static("x-ms-cosmos-populate-content-response-on-write");
static PREFER: HeaderName = HeaderName::from_static("prefer");
static IS_QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-isquery");
static IS_QUERY_LEGACY: HeaderName = HeaderName::from_static("x-ms-documentdb-query");
static IS_QUERY_PLAN_REQUEST: HeaderName =
    HeaderName::from_static("x-ms-cosmos-is-query-plan-request");
static MAX_ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-max-item-count");
static CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
static PARTITION_KEY_RANGE_ID: HeaderName =
    HeaderName::from_static("x-ms-documentdb-partitionkeyrangeid");
static START_EPK: HeaderName = HeaderName::from_static("x-ms-start-epk");
static END_EPK: HeaderName = HeaderName::from_static("x-ms-end-epk");
static READ_FEED_KEY_TYPE: HeaderName = HeaderName::from_static("x-ms-read-key-type");
static IS_BATCH_REQUEST: HeaderName = HeaderName::from_static("x-ms-cosmos-is-batch-request");
static OFFER_THROUGHPUT: HeaderName = HeaderName::from_static("x-ms-offer-throughput");
static OFFER_AUTOPILOT_SETTINGS: HeaderName =
    HeaderName::from_static("x-ms-cosmos-offer-autopilot-settings");

/// Parses an HTTP request into a `ParsedRequest`.
pub(crate) fn parse_request(request: &Request) -> ParsedRequest {
    let url = request.url();
    let method = request.method();
    let headers = request.headers();

    let partition_key_header = headers
        .get_optional_str(&PARTITION_KEY)
        .map(|s| s.to_string());
    let if_match = headers.get_optional_str(&IF_MATCH).map(|s| s.to_string());
    let if_none_match = headers
        .get_optional_str(&IF_NONE_MATCH)
        .map(|s| s.to_string());
    let session_token = headers
        .get_optional_str(&SESSION_TOKEN)
        .map(|s| s.to_string());
    let activity_id = headers
        .get_optional_str(&ACTIVITY_ID)
        .map(|s| s.to_string());
    // Determine whether write responses should include the document body.
    // Check the explicit header first; if absent, check the `Prefer` header
    // (the driver pipeline sends `Prefer: return=minimal` to suppress bodies).
    // Default to true (service returns body when neither header is present).
    let content_response_on_write = if let Some(val) = headers.get_optional_str(&CONTENT_RESPONSE) {
        val.eq_ignore_ascii_case("true")
    } else if let Some(prefer) = headers.get_optional_str(&PREFER) {
        !prefer.contains("return=minimal")
    } else {
        true
    };
    let is_upsert = headers
        .get_optional_str(&IS_UPSERT)
        .map(|s| s.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let is_query = header_true(headers.get_optional_str(&IS_QUERY))
        || header_true(headers.get_optional_str(&IS_QUERY_LEGACY));
    let is_query_plan = header_true(headers.get_optional_str(&IS_QUERY_PLAN_REQUEST));
    let is_batch = header_true(headers.get_optional_str(&IS_BATCH_REQUEST));
    let max_item_count = headers
        .get_optional_str(&MAX_ITEM_COUNT)
        .and_then(|s| s.trim().parse::<i32>().ok());
    let continuation = headers
        .get_optional_str(&CONTINUATION)
        .map(|s| s.to_string());
    let partition_key_range_id = headers
        .get_optional_str(&PARTITION_KEY_RANGE_ID)
        .map(|s| s.to_string());
    let start_epk = headers.get_optional_str(&START_EPK).map(|s| s.to_string());
    let end_epk = headers.get_optional_str(&END_EPK).map(|s| s.to_string());
    let read_key_type = headers
        .get_optional_str(&READ_FEED_KEY_TYPE)
        .map(|s| s.to_string());
    // Parse `x-ms-offer-throughput` (RU/s) from the request headers. Invalid /
    // non-numeric values are treated as absent; the container creation handler
    // then uses `ContainerConfig::default()`. A failing parse is intentionally
    // not surfaced as 400 so requests from older clients that send empty or
    // legacy values still succeed (matching real-service tolerance).
    let offer_throughput = headers
        .get_optional_str(&OFFER_THROUGHPUT)
        .and_then(|s| s.trim().parse::<u32>().ok());
    let offer_autopilot_settings = headers
        .get_optional_str(&OFFER_AUTOPILOT_SETTINGS)
        .map(|s| s.to_string());

    let path = url.path();
    // Reject trailing slashes after the leading `/`. `/dbs/mydb/colls/mycoll/docs/`
    // would otherwise parse to depth=5 and misroute to Create. Only the root
    // path "/" is allowed to be a single slash.
    let has_trailing_slash = path.len() > 1 && path.ends_with('/');
    let segments = parse_path_segments(path);
    let operation = if has_trailing_slash {
        OperationType::BadRequestPath(format!(
            "{} {} (trailing slash rejected)",
            method.as_ref(),
            path
        ))
    } else {
        resolve_operation(
            method.as_ref(),
            &segments,
            is_upsert,
            is_query,
            is_query_plan,
            is_batch,
        )
    };

    // A read scoped to an effective-partition-key *range*
    // (`x-ms-start-epk`/`x-ms-end-epk`) must declare the range key type
    // `EffectivePartitionKeyRange`. The point key type `EffectivePartitionKey`
    // is rejected by the real gateway with `400 "One of the input values is
    // invalid"`; mirror that here so a regression that reverts the driver's
    // key-type value is caught by the emulator-backed tests (issues #4680 and
    // #4681).
    let operation = if (start_epk.is_some() || end_epk.is_some())
        && read_key_type.as_deref()
            != Some(
                crate::models::cosmos_headers::request_header_names::READ_FEED_KEY_TYPE_EPK_RANGE,
            ) {
        OperationType::InvalidInput(format!(
            "x-ms-read-key-type must be 'EffectivePartitionKeyRange' when \
             x-ms-start-epk/x-ms-end-epk are present, got {read_key_type:?}"
        ))
    } else {
        operation
    };

    // Index by *position*, not by keyword search. Cosmos URLs are
    // `/dbs/{db}/colls/{coll}/docs/{doc}/...`, so the keyword always
    // appears at an even index and the value follows it. Searching by
    // keyword string is wrong: a path like `/dbs/colls/colls/mycoll/docs`
    // (where the database happens to be named `colls`) returns the
    // database name when looking up the container.
    let db_id = segment_after_keyword(&segments, 0, "dbs");
    let coll_id = segment_after_keyword(&segments, 2, "colls");
    let doc_id = segment_after_keyword(&segments, 4, "docs");
    let offer_id = segment_after_keyword(&segments, 0, "offers");

    ParsedRequest {
        operation,
        db_id,
        coll_id,
        doc_id,
        offer_id,
        partition_key_header,
        if_match,
        if_none_match,
        session_token,
        activity_id,
        content_response_on_write,
        offer_throughput,
        offer_autopilot_settings,
        max_item_count,
        continuation,
        partition_key_range_id,
        start_epk,
        end_epk,
        is_query_plan,
        is_batch,
        is_upsert,
    }
}

fn header_true(value: Option<&str>) -> bool {
    value
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Parses URL path into segments, skipping empty entries.
///
/// Each segment is percent-decoded so document, container, and database IDs
/// that contain characters the SDK percent-encodes (spaces, '+', '%',
/// non-ASCII) match the stored IDs the way they would against a real Cosmos
/// DB gateway. Invalid UTF-8 in a percent-decoded segment falls back to the
/// raw segment so we never panic on malformed input.
fn parse_path_segments(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| !s.is_empty())
        .map(|s| {
            percent_decode_str(s)
                .decode_utf8()
                .map(|cow| cow.into_owned())
                .unwrap_or_else(|_| s.to_string())
        })
        .collect()
}

/// Extracts the segment value at the position immediately after a fixed
/// keyword position in a Cosmos resource path.
///
/// Cosmos URLs follow the rigid shape `/dbs/{db}/colls/{coll}/docs/{doc}`:
/// the keyword `dbs` is always at index 0, `colls` at index 2, `docs` at
/// index 4. A previous implementation searched for the keyword by string
/// match, which broke when a user's resource happened to be named after a
/// keyword (e.g. a database named `colls`). Anchoring by position makes
/// such collisions impossible while still returning `None` for paths that
/// don't carry the segment in the expected slot.
///
/// Returns `None` if the keyword is not at `keyword_index`, or if no
/// value follows it.
fn segment_after_keyword(
    segments: &[String],
    keyword_index: usize,
    keyword: &str,
) -> Option<String> {
    if segments.get(keyword_index).map(String::as_str) != Some(keyword) {
        return None;
    }
    segments.get(keyword_index + 1).cloned()
}

/// Resolves the operation type from HTTP method + path segments + headers.
fn resolve_operation(
    method: &str,
    segments: &[String],
    is_upsert: bool,
    is_query: bool,
    is_query_plan: bool,
    is_batch: bool,
) -> OperationType {
    let depth = segments.len();

    match (method, depth) {
        // GET / → ReadAccount
        ("GET", 0) => OperationType::ReadAccount,

        #[cfg(feature = "preview_dtx")]
        ("POST", 2) if segments[0] == "operations" && segments[1] == "dtc" => {
            OperationType::DistributedTransaction
        }

        // GET /dbs → ReadFeedDatabases
        ("GET", 1) if segments[0] == "dbs" => OperationType::ReadFeedDatabases,

        // POST /dbs → CreateDatabase/QueryDatabases
        ("POST", 1) if segments[0] == "dbs" => {
            if is_query {
                OperationType::QueryDatabases
            } else {
                OperationType::CreateDatabase
            }
        }

        // GET /dbs/{db} → ReadDatabase
        ("GET", 2) if segments[0] == "dbs" => OperationType::ReadDatabase,

        // DELETE /dbs/{db} → DeleteDatabase
        ("DELETE", 2) if segments[0] == "dbs" => OperationType::DeleteDatabase,

        // GET /dbs/{db}/colls → ReadFeedContainers
        ("GET", 3) if segments[0] == "dbs" && segments[2] == "colls" => {
            OperationType::ReadFeedContainers
        }

        // POST /dbs/{db}/colls → CreateContainer/QueryContainers
        ("POST", 3) if segments[0] == "dbs" && segments[2] == "colls" => {
            if is_query {
                OperationType::QueryContainers
            } else {
                OperationType::CreateContainer
            }
        }

        // GET /dbs/{db}/colls/{coll} → ReadContainer
        ("GET", 4) if segments[0] == "dbs" && segments[2] == "colls" => {
            OperationType::ReadContainer
        }

        // DELETE /dbs/{db}/colls/{coll} → DeleteContainer
        ("DELETE", 4) if segments[0] == "dbs" && segments[2] == "colls" => {
            OperationType::DeleteContainer
        }

        // GET /dbs/{db}/colls/{coll}/pkranges → ReadPKRanges
        ("GET", 5)
            if segments[0] == "dbs" && segments[2] == "colls" && segments[4] == "pkranges" =>
        {
            OperationType::ReadPKRanges
        }

        // GET /dbs/{db}/colls/{coll}/docs → ReadFeedItems
        ("GET", 5) if segments[0] == "dbs" && segments[2] == "colls" && segments[4] == "docs" => {
            OperationType::ReadFeedItems
        }

        // POST /dbs/{db}/colls/{coll}/docs → Create/Upsert/Query/QueryPlan/Batch
        ("POST", 5) if segments[0] == "dbs" && segments[2] == "colls" && segments[4] == "docs" => {
            if is_query_plan {
                OperationType::QueryPlan
            } else if is_query {
                OperationType::QueryItems
            } else if is_batch {
                OperationType::Batch
            } else if is_upsert {
                OperationType::Upsert
            } else {
                OperationType::Create
            }
        }

        // GET /dbs/{db}/colls/{coll}/docs/{doc} → Read
        ("GET", 6) if segments[0] == "dbs" && segments[2] == "colls" && segments[4] == "docs" => {
            OperationType::Read
        }

        // PUT /dbs/{db}/colls/{coll}/docs/{doc} → Replace
        ("PUT", 6) if segments[0] == "dbs" && segments[2] == "colls" && segments[4] == "docs" => {
            OperationType::Replace
        }

        // DELETE /dbs/{db}/colls/{coll}/docs/{doc} → Delete
        ("DELETE", 6)
            if segments[0] == "dbs" && segments[2] == "colls" && segments[4] == "docs" =>
        {
            OperationType::Delete
        }

        // GET /offers → ReadFeedOffers
        ("GET", 1) if segments[0] == "offers" => OperationType::ReadFeedOffers,

        // POST /offers → QueryOffers
        ("POST", 1) if segments[0] == "offers" && is_query => OperationType::QueryOffers,

        // GET /offers/{rid} → ReadOffer
        ("GET", 2) if segments[0] == "offers" => OperationType::ReadOffer,

        // PUT /offers/{rid} → ReplaceOffer
        ("PUT", 2) if segments[0] == "offers" => OperationType::ReplaceOffer,

        _ => OperationType::Unsupported(format!("{} {}", method, segments.join("/"))),
    }
}

/// Resolves the region name from the request URL by matching against configured regions.
pub(crate) fn resolve_region<'a>(
    url: &azure_core::http::Url,
    config: &'a super::config::VirtualAccountConfig,
) -> Option<&'a str> {
    config.region_for_url(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::headers::HeaderValue;
    use azure_core::http::{Method, Request, Url};

    fn make_request(method: &str, path: &str) -> Request {
        let url = format!("https://test.emulator.local{}", path);
        let method = match method {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => Method::Get,
        };
        Request::new(url.parse().unwrap(), method)
    }

    fn insert_header(req: &mut Request, name: HeaderName, value: &str) {
        req.headers_mut()
            .insert(name, HeaderValue::from(value.to_string()));
    }

    #[test]
    fn read_account() {
        let req = make_request("GET", "/");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadAccount);
    }

    #[test]
    fn segment_after_keyword_anchors_by_position() {
        // Path: /dbs/colls/colls/mycoll/docs/d1
        // Database is named "colls" — searching for the keyword "colls" by
        // string match would return the database name when looking for the
        // container. Position-anchoring returns the correct value.
        let segments = vec![
            "dbs".to_string(),
            "colls".to_string(),
            "colls".to_string(),
            "mycoll".to_string(),
            "docs".to_string(),
            "d1".to_string(),
        ];
        assert_eq!(
            segment_after_keyword(&segments, 0, "dbs"),
            Some("colls".to_string())
        );
        assert_eq!(
            segment_after_keyword(&segments, 2, "colls"),
            Some("mycoll".to_string())
        );
        assert_eq!(
            segment_after_keyword(&segments, 4, "docs"),
            Some("d1".to_string())
        );
    }

    #[test]
    fn segment_after_keyword_returns_none_when_keyword_absent_at_position() {
        let segments = vec!["dbs".to_string(), "mydb".to_string()];
        // No `colls` segment → coll lookup returns None.
        assert_eq!(segment_after_keyword(&segments, 2, "colls"), None);
    }

    #[test]
    fn parse_request_resolves_container_named_after_keyword() {
        // Database name is `colls` — used to be ambiguous under the old
        // string-search extractor.
        let req = make_request("GET", "/dbs/colls/colls/mycoll");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadContainer);
        assert_eq!(parsed.db_id.as_deref(), Some("colls"));
        assert_eq!(parsed.coll_id.as_deref(), Some("mycoll"));
    }

    #[test]
    fn parse_request_resolves_document_when_container_named_docs() {
        // Container name is `docs` — same bug class as above.
        let req = make_request("GET", "/dbs/mydb/colls/docs/docs/d1");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::Read);
        assert_eq!(parsed.db_id.as_deref(), Some("mydb"));
        assert_eq!(parsed.coll_id.as_deref(), Some("docs"));
        assert_eq!(parsed.doc_id.as_deref(), Some("d1"));
    }

    #[test]
    fn create_database() {
        let req = make_request("POST", "/dbs");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::CreateDatabase);
    }

    #[test]
    fn read_database_feed() {
        let req = make_request("GET", "/dbs");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadFeedDatabases);
    }

    #[test]
    fn query_databases() {
        let mut req = make_request("POST", "/dbs");
        insert_header(&mut req, IS_QUERY.clone(), "True");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::QueryDatabases);
    }

    #[test]
    fn read_database() {
        let req = make_request("GET", "/dbs/mydb");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadDatabase);
        assert_eq!(parsed.db_id.as_deref(), Some("mydb"));
    }

    #[test]
    fn create_container() {
        let req = make_request("POST", "/dbs/mydb/colls");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::CreateContainer);
        assert_eq!(parsed.db_id.as_deref(), Some("mydb"));
    }

    #[test]
    fn read_container_feed() {
        let req = make_request("GET", "/dbs/mydb/colls");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadFeedContainers);
        assert_eq!(parsed.db_id.as_deref(), Some("mydb"));
    }

    #[test]
    fn query_containers() {
        let mut req = make_request("POST", "/dbs/mydb/colls");
        insert_header(&mut req, IS_QUERY.clone(), "True");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::QueryContainers);
        assert_eq!(parsed.db_id.as_deref(), Some("mydb"));
    }

    #[test]
    fn read_document() {
        let req = make_request("GET", "/dbs/mydb/colls/mycoll/docs/doc1");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::Read);
        assert_eq!(parsed.db_id.as_deref(), Some("mydb"));
        assert_eq!(parsed.coll_id.as_deref(), Some("mycoll"));
        assert_eq!(parsed.doc_id.as_deref(), Some("doc1"));
    }

    #[test]
    fn create_document() {
        let req = make_request("POST", "/dbs/mydb/colls/mycoll/docs");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::Create);
    }

    #[test]
    fn read_document_feed() {
        let req = make_request("GET", "/dbs/mydb/colls/mycoll/docs");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadFeedItems);
    }

    #[test]
    fn query_items_uses_current_driver_header() {
        let mut req = make_request("POST", "/dbs/mydb/colls/mycoll/docs");
        insert_header(&mut req, IS_QUERY.clone(), "True");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::QueryItems);
    }

    #[test]
    fn query_items_accepts_legacy_query_header() {
        let mut req = make_request("POST", "/dbs/mydb/colls/mycoll/docs");
        insert_header(&mut req, IS_QUERY_LEGACY.clone(), "True");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::QueryItems);
    }

    #[test]
    fn query_plan_takes_precedence_over_item_query() {
        let mut req = make_request("POST", "/dbs/mydb/colls/mycoll/docs");
        insert_header(&mut req, IS_QUERY.clone(), "True");
        insert_header(&mut req, IS_QUERY_PLAN_REQUEST.clone(), "True");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::QueryPlan);
    }

    #[test]
    fn batch_document_feed() {
        let mut req = make_request("POST", "/dbs/mydb/colls/mycoll/docs");
        insert_header(&mut req, IS_BATCH_REQUEST.clone(), "True");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::Batch);
    }

    #[test]
    fn upsert_document() {
        let url: Url = "https://test.emulator.local/dbs/mydb/colls/mycoll/docs"
            .parse()
            .unwrap();
        let mut req = Request::new(url, Method::Post);
        req.headers_mut().insert(
            IS_UPSERT.clone(),
            azure_core::http::headers::HeaderValue::from("True".to_string()),
        );
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::Upsert);
    }

    #[test]
    fn pkranges() {
        let req = make_request("GET", "/dbs/mydb/colls/mycoll/pkranges");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadPKRanges);
    }

    #[test]
    fn offer_routes() {
        let read_feed = parse_request(&make_request("GET", "/offers"));
        assert_eq!(read_feed.operation, OperationType::ReadFeedOffers);

        let mut query = make_request("POST", "/offers");
        insert_header(&mut query, IS_QUERY.clone(), "True");
        assert_eq!(parse_request(&query).operation, OperationType::QueryOffers);

        assert_eq!(
            parse_request(&make_request("GET", "/offers/offer1")).operation,
            OperationType::ReadOffer
        );
        assert_eq!(
            parse_request(&make_request("PUT", "/offers/offer1")).operation,
            OperationType::ReplaceOffer
        );
    }

    #[test]
    fn trailing_slash_on_docs_collection_is_rejected() {
        // Without explicit rejection this would resolve to Create (POST) /
        // a misrouted GET; both are wrong because the gateway does not
        // accept trailing slashes on resource paths. Surfacing as
        // BadRequestPath (mapped to 400 by the handler) matches the real
        // gateway's response and makes the misuse loud instead of silent.
        let req = make_request("POST", "/dbs/mydb/colls/mycoll/docs/");
        let parsed = parse_request(&req);
        assert!(matches!(parsed.operation, OperationType::BadRequestPath(_)));
    }

    #[test]
    fn trailing_slash_on_document_is_rejected() {
        let req = make_request("GET", "/dbs/mydb/colls/mycoll/docs/d1/");
        let parsed = parse_request(&req);
        assert!(matches!(parsed.operation, OperationType::BadRequestPath(_)));
    }

    #[test]
    fn root_path_is_still_read_account() {
        // The single-slash root path must continue to resolve normally.
        let req = make_request("GET", "/");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadAccount);
    }
}
