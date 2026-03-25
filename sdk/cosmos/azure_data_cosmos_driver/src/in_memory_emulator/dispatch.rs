// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request parsing, URL routing, and operation resolution.

use azure_core::http::headers::HeaderName;
use azure_core::http::Request;

/// The type of operation resolved from an HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OperationType {
    ReadAccount,
    CreateDatabase,
    ReadDatabase,
    DeleteDatabase,
    CreateContainer,
    ReadContainer,
    DeleteContainer,
    ReadPKRanges,
    Create,
    Read,
    Replace,
    Upsert,
    Delete,
    Query,
    Unsupported(String),
}

/// Parsed request data extracted from an HTTP request.
#[derive(Debug, Clone)]
pub(crate) struct ParsedRequest {
    pub operation: OperationType,
    pub db_id: Option<String>,
    pub coll_id: Option<String>,
    pub doc_id: Option<String>,
    pub partition_key_header: Option<String>,
    pub if_match: Option<String>,
    pub session_token: Option<String>,
    pub content_response_on_write: bool,
    #[allow(dead_code)]
    pub is_upsert: bool, // used during dispatch resolution
}

// Header name constants for request parsing
static IS_UPSERT: HeaderName = HeaderName::from_static("x-ms-documentdb-is-upsert");
static PARTITION_KEY: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkey");
static IF_MATCH: HeaderName = HeaderName::from_static("if-match");
static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
static CONTENT_RESPONSE: HeaderName =
    HeaderName::from_static("x-ms-cosmos-populate-content-response-on-write");
static IS_QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-query");

/// Parses an HTTP request into a `ParsedRequest`.
pub(crate) fn parse_request(request: &Request) -> ParsedRequest {
    let url = request.url();
    let method = request.method();
    let headers = request.headers();

    let partition_key_header = headers
        .get_optional_str(&PARTITION_KEY)
        .map(|s| s.to_string());
    let if_match = headers.get_optional_str(&IF_MATCH).map(|s| s.to_string());
    let session_token = headers
        .get_optional_str(&SESSION_TOKEN)
        .map(|s| s.to_string());
    let content_response_on_write = headers
        .get_optional_str(&CONTENT_RESPONSE)
        .map(|s| s.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let is_upsert = headers
        .get_optional_str(&IS_UPSERT)
        .map(|s| s.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let is_query = headers.get_optional_str(&IS_QUERY).is_some();

    let path = url.path();
    let segments = parse_path_segments(path);
    let operation = resolve_operation(method.as_ref(), &segments, is_upsert, is_query);

    let db_id = extract_segment(&segments, "dbs");
    let coll_id = extract_segment(&segments, "colls");
    let doc_id = extract_segment(&segments, "docs");

    ParsedRequest {
        operation,
        db_id,
        coll_id,
        doc_id,
        partition_key_header,
        if_match,
        session_token,
        content_response_on_write,
        is_upsert,
    }
}

/// Parses URL path into segments, skipping empty entries.
fn parse_path_segments(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Extracts the segment value following a resource type keyword (e.g., "dbs" → next segment).
fn extract_segment(segments: &[String], resource_type: &str) -> Option<String> {
    segments
        .iter()
        .position(|s| s == resource_type)
        .and_then(|i| segments.get(i + 1))
        .cloned()
}

/// Resolves the operation type from HTTP method + path segments + headers.
fn resolve_operation(
    method: &str,
    segments: &[String],
    is_upsert: bool,
    is_query: bool,
) -> OperationType {
    let depth = segments.len();

    match (method, depth) {
        // GET / → ReadAccount
        ("GET", 0) => OperationType::ReadAccount,

        // POST /dbs → CreateDatabase
        ("POST", 1) if segments[0] == "dbs" => OperationType::CreateDatabase,

        // GET /dbs/{db} → ReadDatabase
        ("GET", 2) if segments[0] == "dbs" => OperationType::ReadDatabase,

        // DELETE /dbs/{db} → DeleteDatabase
        ("DELETE", 2) if segments[0] == "dbs" => OperationType::DeleteDatabase,

        // POST /dbs/{db}/colls → CreateContainer
        ("POST", 3) if segments[0] == "dbs" && segments[2] == "colls" => {
            OperationType::CreateContainer
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

        // POST /dbs/{db}/colls/{coll}/docs → Create/Upsert/Query
        ("POST", 5) if segments[0] == "dbs" && segments[2] == "colls" && segments[4] == "docs" => {
            if is_query {
                OperationType::Query
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

    #[test]
    fn read_account() {
        let req = make_request("GET", "/");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::ReadAccount);
    }

    #[test]
    fn create_database() {
        let req = make_request("POST", "/dbs");
        let parsed = parse_request(&req);
        assert_eq!(parsed.operation, OperationType::CreateDatabase);
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
}
