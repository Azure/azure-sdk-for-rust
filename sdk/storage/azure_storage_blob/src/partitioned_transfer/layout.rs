// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Blob layout model and endpoint resolution for locality-aware downloads.
//!
//! The service's Get Blob Layout API describes which byte ranges of a blob are
//! served by which endpoints. This module turns those paginated responses into a
//! flat, ascending list of [`LayoutSegment`]s and resolves the serving endpoint
//! for a given byte offset via binary search.

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use azure_core::{
    error::ErrorKind,
    http::{
        policies::{Policy, PolicyResult},
        Context, Etag, Request, StatusCode, Url,
    },
    Error, Result,
};
use futures::StreamExt as _;

use crate::generated::{
    clients::BlobClient,
    models::{BlobClientListLayoutOptions, BlobLayout},
};

/// A contiguous byte range of a blob and the endpoint that serves it.
///
/// `end` is the inclusive last byte offset, matching the service's layout ranges.
// Internal-only helper; plain `Debug` is intentional so test assertions can print
// segment contents (endpoints are host:port, not secrets).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LayoutSegment {
    /// Inclusive start byte offset of the range.
    pub start: i64,
    /// Inclusive end byte offset of the range.
    pub end: i64,
    /// The `host:port` endpoint serving this range, or `None` when the service did
    /// not provide one; such ranges download from the client's configured endpoint.
    pub endpoint: Option<String>,
}

/// The resolved layout of a blob: non-overlapping, ascending byte-range segments.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Layout {
    segments: Vec<LayoutSegment>,
}

impl Layout {
    /// Returns `true` when the layout contains no segments (no routing applies).
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Appends the segments described by a single Get Blob Layout page.
    ///
    /// Endpoint indices are scoped to their page, so each page's `Endpoints` are
    /// mapped independently before its `Ranges` are resolved and appended.
    pub fn extend_from_page(&mut self, page: &BlobLayout) {
        let ranges = &page.ranges.range;
        if ranges.is_empty() {
            return;
        }

        let index_to_endpoint: HashMap<i32, &str> = page
            .endpoints
            .as_ref()
            .and_then(|endpoints| endpoints.endpoint.as_ref())
            .map(|endpoints| {
                endpoints
                    .iter()
                    .filter_map(|endpoint| Some((endpoint.index?, endpoint.value.as_deref()?)))
                    .collect()
            })
            .unwrap_or_default();

        self.segments.reserve(ranges.len());
        for range in ranges {
            let endpoint = range
                .endpoint_index
                .and_then(|index| index_to_endpoint.get(&index).copied())
                .filter(|value| !value.is_empty())
                .map(str::to_owned);
            self.segments.push(LayoutSegment {
                start: range.start.unwrap_or_default(),
                end: range.end.unwrap_or_default(),
                endpoint,
            });
        }
    }

    /// Resolves the serving endpoint for the segment covering `offset`.
    ///
    /// Uses binary search to find the first segment whose inclusive `end` is at or
    /// beyond `offset`. Returns `None` when no segment covers the offset or the
    /// covering segment has no endpoint; callers then fall back to the client's
    /// configured endpoint. The bytes returned are identical regardless.
    pub fn ideal_endpoint(&self, offset: i64) -> Option<&str> {
        if self.segments.is_empty() {
            return None;
        }

        let mut lo: isize = 0;
        let mut hi: isize = self.segments.len() as isize - 1;
        let mut overlap: Option<usize> = None;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if self.segments[mid as usize].end >= offset {
                overlap = Some(mid as usize);
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        overlap.and_then(|index| self.segments[index].endpoint.as_deref())
    }
}

/// Context value carrying the layout endpoint (`host:port` or an absolute URL) that
/// a single download range request should be routed to.
#[derive(Clone, Debug)]
pub(crate) struct LayoutEndpoint(pub String);

/// Per-call pipeline policy that reroutes a request to a layout endpoint while
/// preserving the original account authority as the `Host` header.
///
/// It is a no-op unless a [`LayoutEndpoint`] is present in the request context, so
/// it is safe to install on every request. A malformed endpoint fails the request
/// so that a bad layout response surfaces loudly rather than silently degrading.
#[derive(Debug)]
pub(crate) struct LayoutRoutingPolicy;

#[async_trait]
impl Policy for LayoutRoutingPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        if let Some(LayoutEndpoint(endpoint)) = ctx.value::<LayoutEndpoint>() {
            apply_layout_endpoint(request, endpoint)?;
        }
        next[0].send(ctx, request, &next[1..]).await
    }
}

/// Rewrites `request`'s URL authority to `endpoint`, preserving the original
/// authority as an explicit `Host` header.
///
/// The rewrite is computed on a scratch copy and only committed once fully
/// successful, so a malformed endpoint returns an error without leaving the
/// request in a half-rewritten state.
fn apply_layout_endpoint(request: &mut Request, endpoint: &str) -> azure_core::Result<()> {
    let (host, port) = parse_endpoint_authority(endpoint).ok_or_else(|| {
        Error::with_message(
            ErrorKind::Other,
            format!("invalid layout endpoint {endpoint:?}"),
        )
    })?;
    let original_host = original_host_header(request.url());
    let mut rewritten = request.url().clone();
    rewritten
        .set_host(Some(&host))
        .map_err(|e| Error::with_error(ErrorKind::Other, e, "invalid layout endpoint host"))?;
    rewritten
        .set_port(port)
        .map_err(|()| Error::with_message(ErrorKind::Other, "invalid layout endpoint port"))?;
    request.insert_header("host", original_host);
    *request.url_mut() = rewritten;
    Ok(())
}

/// Derives the `Host` header value the client would send for `url`, i.e. the host
/// plus the port when a non-default port is explicitly present.
fn original_host_header(url: &Url) -> String {
    match (url.host_str(), url.port()) {
        (Some(host), Some(port)) => format!("{host}:{port}"),
        (Some(host), None) => host.to_owned(),
        (None, _) => String::new(),
    }
}

/// Parses a layout endpoint value into a host and optional port.
///
/// Accepts both the documented `host:port` form and an absolute URL form
/// (`scheme://host[:port]`). Returns `None` for values that cannot be interpreted
/// as an authority; the caller turns that into a failed request.
fn parse_endpoint_authority(endpoint: &str) -> Option<(String, Option<u16>)> {
    let endpoint = endpoint.trim();
    if endpoint.is_empty() {
        return None;
    }

    if endpoint.contains("://") {
        let url = Url::parse(endpoint).ok()?;
        let host = url.host_str()?.to_owned();
        return Some((host, url.port()));
    }

    match endpoint.rsplit_once(':') {
        Some((host, port)) if !host.is_empty() => Some((host.to_owned(), Some(port.parse().ok()?))),
        Some(_) => None,
        None => Some((endpoint.to_owned(), None)),
    }
}

/// The outcome of prefetching a blob's layout for a locality-aware download.
pub(crate) struct LayoutPrefetch {
    /// The resolved, non-empty layout used to route range requests.
    pub layout: Layout,
    /// The ETag pinning the download to a single blob version: the caller-supplied
    /// condition when present, otherwise the ETag from the first layout page.
    pub etag: Option<Etag>,
}

/// Fetches a blob's layout for locality-aware routing, following pagination.
///
/// Returns `Ok(Some(_))` when a non-empty layout is available; `Ok(None)` when the
/// download should proceed normally without routing (no layout, HTTP 400, or 5xx);
/// and `Err(_)` when the download must fail (HTTP 403/404/409/412, a transport
/// error, or a response deserialization error).
///
/// The generated pager owns request construction, response handling, and
/// continuation. The first page's ETag pins the subsequent blob download when the
/// caller did not already supply an `If-Match` condition.
pub(crate) async fn fetch_layout(
    client: &BlobClient,
    context: &Context<'_>,
    options: &BlobClientListLayoutOptions<'_>,
) -> Result<Option<LayoutPrefetch>> {
    let mut layout = Layout::default();
    let mut locked_etag = options.if_match.clone();
    let mut options = options.clone();
    options.method_options.context = context.clone().into_owned();
    let mut pages = client.list_layout(Some(options))?;

    while let Some(response) = pages.next().await {
        let response = match response {
            Ok(response) => response,
            Err(err) => return classify_layout_error(err),
        };
        if locked_etag.is_none() {
            locked_etag = response
                .headers()
                .get_optional_str(&"etag".into())
                .map(Etag::from);
        }
        let page = response.into_model()?;
        layout.extend_from_page(&page);
    }

    if layout.is_empty() {
        return Ok(None);
    }
    Ok(Some(LayoutPrefetch {
        layout,
        etag: locked_etag,
    }))
}

/// Maps a Get Blob Layout failure to a graceful fall-back (`Ok(None)`) or a hard
/// failure (`Err`).
///
/// HTTP 400 (layout unsupported) and 5xx (transient) fall back to a normal
/// download; every other failure (403/404/409/412 and transport errors) fails.
fn classify_layout_error(err: Error) -> Result<Option<LayoutPrefetch>> {
    match err.http_status() {
        Some(status) if status == StatusCode::BadRequest || status.is_server_error() => Ok(None),
        _ => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::{
        clients::BlobClientOptions,
        models::{BlobLayoutEndpoint, BlobLayoutEndpoints, BlobLayoutRange, BlobLayoutRanges},
    };
    use azure_core::{
        http::{
            headers::Headers, AsyncRawResponse, ClientOptions, FixedRetryOptions, HttpClient,
            Method, RetryOptions, Transport,
        },
        Bytes,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt as _;

    fn segment(start: i64, end: i64, endpoint: Option<&str>) -> LayoutSegment {
        LayoutSegment {
            start,
            end,
            endpoint: endpoint.map(str::to_owned),
        }
    }

    fn layout(segments: Vec<LayoutSegment>) -> Layout {
        Layout { segments }
    }

    fn endpoint(index: i32, value: &str) -> BlobLayoutEndpoint {
        BlobLayoutEndpoint {
            index: Some(index),
            value: Some(value.to_owned()),
        }
    }

    fn range(start: i64, end: i64, endpoint_index: i32) -> BlobLayoutRange {
        BlobLayoutRange {
            start: Some(start),
            end: Some(end),
            endpoint_index: Some(endpoint_index),
        }
    }

    fn page(endpoints: Vec<BlobLayoutEndpoint>, ranges: Vec<BlobLayoutRange>) -> BlobLayout {
        BlobLayout {
            endpoints: Some(BlobLayoutEndpoints {
                endpoint: Some(endpoints),
            }),
            ranges: BlobLayoutRanges { range: ranges },
            ..Default::default()
        }
    }

    #[test]
    fn empty_layout_resolves_to_none() {
        let layout = Layout::default();
        assert!(layout.is_empty());
        assert_eq!(layout.ideal_endpoint(0), None);
        assert_eq!(layout.ideal_endpoint(1_000), None);
    }

    #[test]
    fn single_segment_covers_all_contained_offsets() {
        let layout = layout(vec![segment(0, 99, Some("a"))]);
        assert_eq!(layout.ideal_endpoint(0), Some("a"));
        assert_eq!(layout.ideal_endpoint(50), Some("a"));
        assert_eq!(layout.ideal_endpoint(99), Some("a"));
        // Beyond the final segment's end: no segment covers it.
        assert_eq!(layout.ideal_endpoint(100), None);
    }

    #[test]
    fn binary_search_selects_covering_segment() {
        let layout = layout(vec![
            segment(0, 9, Some("a")),
            segment(10, 19, Some("b")),
            segment(20, 29, Some("c")),
        ]);
        assert_eq!(layout.ideal_endpoint(0), Some("a"));
        assert_eq!(layout.ideal_endpoint(9), Some("a"));
        assert_eq!(layout.ideal_endpoint(10), Some("b"));
        assert_eq!(layout.ideal_endpoint(15), Some("b"));
        assert_eq!(layout.ideal_endpoint(19), Some("b"));
        assert_eq!(layout.ideal_endpoint(20), Some("c"));
        assert_eq!(layout.ideal_endpoint(29), Some("c"));
        assert_eq!(layout.ideal_endpoint(30), None);
    }

    #[test]
    fn inclusive_end_boundaries_route_correctly() {
        let layout = layout(vec![segment(0, 9, Some("a")), segment(10, 19, Some("b"))]);
        // end is inclusive: offset == first segment's end stays on the first segment.
        assert_eq!(layout.ideal_endpoint(9), Some("a"));
        // one past it moves to the next segment.
        assert_eq!(layout.ideal_endpoint(10), Some("b"));
    }

    #[test]
    fn segment_without_endpoint_resolves_to_none() {
        let layout = layout(vec![segment(0, 9, None), segment(10, 19, Some("b"))]);
        assert_eq!(layout.ideal_endpoint(5), None);
        assert_eq!(layout.ideal_endpoint(15), Some("b"));
    }

    #[test]
    fn extend_from_page_maps_endpoint_indices() {
        let mut layout = Layout::default();
        layout.extend_from_page(&page(
            vec![endpoint(0, "h0:443"), endpoint(1, "h1:443")],
            vec![range(0, 9, 1), range(10, 19, 0)],
        ));
        assert_eq!(
            layout.segments,
            vec![
                segment(0, 9, Some("h1:443")),
                segment(10, 19, Some("h0:443")),
            ]
        );
    }

    #[test]
    fn extend_from_page_handles_unordered_endpoint_indices() {
        let mut layout = Layout::default();
        layout.extend_from_page(&page(
            vec![endpoint(2, "h2:443"), endpoint(0, "h0:443")],
            vec![range(0, 9, 2), range(10, 19, 0)],
        ));
        assert_eq!(
            layout.segments,
            vec![
                segment(0, 9, Some("h2:443")),
                segment(10, 19, Some("h0:443")),
            ]
        );
    }

    #[test]
    fn extend_from_page_missing_endpoint_index_yields_none() {
        let mut layout = Layout::default();
        layout.extend_from_page(&page(vec![endpoint(0, "h0:443")], vec![range(0, 9, 7)]));
        assert_eq!(layout.segments, vec![segment(0, 9, None)]);
    }

    #[test]
    fn extend_from_page_accumulates_across_pages_with_independent_indices() {
        let mut layout = Layout::default();
        // Page 1: index 0 -> "p1:443".
        layout.extend_from_page(&page(vec![endpoint(0, "p1:443")], vec![range(0, 9, 0)]));
        // Page 2: index 0 -> "p2:443" (different endpoint space).
        layout.extend_from_page(&page(vec![endpoint(0, "p2:443")], vec![range(10, 19, 0)]));
        assert_eq!(
            layout.segments,
            vec![
                segment(0, 9, Some("p1:443")),
                segment(10, 19, Some("p2:443")),
            ]
        );
    }

    #[test]
    fn extend_from_page_with_empty_ranges_adds_nothing() {
        let mut layout = Layout::default();
        layout.extend_from_page(&page(vec![endpoint(0, "h0:443")], vec![]));
        assert!(layout.is_empty());
    }

    fn request_to(url: &str) -> Request {
        Request::new(url.parse().unwrap(), Method::Get)
    }

    fn host_header(request: &Request) -> Option<String> {
        request
            .headers()
            .get_optional_str(&"host".into())
            .map(str::to_owned)
    }

    #[test]
    fn parse_endpoint_authority_forms() {
        assert_eq!(
            parse_endpoint_authority("host.example.net:443"),
            Some(("host.example.net".to_owned(), Some(443)))
        );
        assert_eq!(
            parse_endpoint_authority("host.example.net:8443"),
            Some(("host.example.net".to_owned(), Some(8443)))
        );
        assert_eq!(
            parse_endpoint_authority("https://host.example.net:8443"),
            Some(("host.example.net".to_owned(), Some(8443)))
        );
        // Absolute URL with default port normalizes the port away.
        assert_eq!(
            parse_endpoint_authority("https://host.example.net"),
            Some(("host.example.net".to_owned(), None))
        );
        // Bare host, no port.
        assert_eq!(
            parse_endpoint_authority("host.example.net"),
            Some(("host.example.net".to_owned(), None))
        );
        // Surrounding whitespace is tolerated.
        assert_eq!(
            parse_endpoint_authority("  host.example.net:443  "),
            Some(("host.example.net".to_owned(), Some(443)))
        );
        // Malformed inputs yield None so the caller skips routing.
        assert_eq!(parse_endpoint_authority(""), None);
        assert_eq!(parse_endpoint_authority(":443"), None);
        assert_eq!(parse_endpoint_authority("host.example.net:port"), None);
    }

    #[test]
    fn apply_layout_endpoint_rewrites_url_and_preserves_host() {
        let mut request = request_to("https://acct.blob.core.windows.net/container/blob");
        apply_layout_endpoint(&mut request, "target.blob.storage.azure.net:443").unwrap();

        assert_eq!(
            request.url().host_str(),
            Some("target.blob.storage.azure.net")
        );
        assert_eq!(request.url().path(), "/container/blob");
        assert_eq!(
            host_header(&request).as_deref(),
            Some("acct.blob.core.windows.net")
        );
    }

    #[test]
    fn apply_layout_endpoint_preserves_original_non_default_port_in_host() {
        let mut request = request_to("https://acct.blob.core.windows.net:10000/container/blob");
        apply_layout_endpoint(&mut request, "target.blob.storage.azure.net:443").unwrap();

        assert_eq!(
            request.url().host_str(),
            Some("target.blob.storage.azure.net")
        );
        assert_eq!(
            host_header(&request).as_deref(),
            Some("acct.blob.core.windows.net:10000")
        );
    }

    #[test]
    fn apply_layout_endpoint_absolute_url_form() {
        let mut request = request_to("https://acct.blob.core.windows.net/container/blob");
        apply_layout_endpoint(&mut request, "https://target.blob.storage.azure.net:8443").unwrap();

        assert_eq!(
            request.url().host_str(),
            Some("target.blob.storage.azure.net")
        );
        assert_eq!(request.url().port(), Some(8443));
        assert_eq!(
            host_header(&request).as_deref(),
            Some("acct.blob.core.windows.net")
        );
    }

    #[test]
    fn apply_layout_endpoint_malformed_fails_and_leaves_request_untouched() {
        let mut request = request_to("https://acct.blob.core.windows.net/container/blob");
        let result = apply_layout_endpoint(&mut request, "");

        // Malformed endpoint fails the request and does not mutate it.
        assert!(result.is_err());
        assert_eq!(request.url().host_str(), Some("acct.blob.core.windows.net"));
        assert_eq!(host_header(&request), None);
    }

    const LAYOUT_SINGLE_PAGE: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<BlobLayout>
  <Endpoints>
    <Endpoint Index="0" Value="ep0.blob.storage.azure.net:443" />
    <Endpoint Index="1" Value="ep1.blob.storage.azure.net:443" />
  </Endpoints>
  <Ranges>
    <Range Start="0" End="4194303" EndpointIndex="0" />
    <Range Start="4194304" End="8388607" EndpointIndex="1" />
  </Ranges>
</BlobLayout>"#;

    const LAYOUT_PAGE_1: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<BlobLayout>
  <Endpoints>
    <Endpoint Index="0" Value="ep0.blob.storage.azure.net:443" />
  </Endpoints>
  <Ranges>
    <Range Start="0" End="4194303" EndpointIndex="0" />
  </Ranges>
  <NextMarker>m2</NextMarker>
</BlobLayout>"#;

    const LAYOUT_PAGE_2: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<BlobLayout>
  <Endpoints>
    <Endpoint Index="0" Value="ep1.blob.storage.azure.net:443" />
  </Endpoints>
  <Ranges>
    <Range Start="4194304" End="8388607" EndpointIndex="0" />
  </Ranges>
</BlobLayout>"#;

    const LAYOUT_EMPTY: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<BlobLayout><Ranges /></BlobLayout>"#;

    fn layout_client(transport: Arc<dyn HttpClient>) -> BlobClient {
        BlobClient::new(
            "https://acct.blob.core.windows.net/container/blob"
                .parse()
                .unwrap(),
            None,
            Some(BlobClientOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(transport)),
                    // Keep error-path tests fast: don't retry mocked 5xx responses.
                    retry: RetryOptions::fixed(FixedRetryOptions {
                        max_retries: 0,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .unwrap()
    }

    fn headers_with_etag(etag: &str) -> Headers {
        let mut headers = Headers::new();
        headers.insert("etag", etag.to_owned());
        headers
    }

    #[tokio::test]
    async fn fetch_layout_single_page_builds_layout() {
        let mock: Arc<dyn HttpClient> = Arc::new(MockHttpClient::new(|req| {
            assert!(req
                .url()
                .query()
                .is_some_and(|query| query.contains("comp=layout")));
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    headers_with_etag("etag-1"),
                    Bytes::from_static(LAYOUT_SINGLE_PAGE),
                ))
            }
            .boxed()
        }));

        let client = layout_client(mock);
        let prefetch = fetch_layout(
            &client,
            &Context::new(),
            &BlobClientListLayoutOptions::default(),
        )
        .await
        .unwrap()
        .expect("routing should be available");

        assert_eq!(prefetch.etag, Some(Etag::from("etag-1")));
        assert_eq!(
            prefetch.layout.ideal_endpoint(0),
            Some("ep0.blob.storage.azure.net:443")
        );
        assert_eq!(
            prefetch.layout.ideal_endpoint(4_194_304),
            Some("ep1.blob.storage.azure.net:443")
        );
    }

    #[tokio::test]
    async fn fetch_layout_paginates_with_caller_etag() {
        let mock: Arc<dyn HttpClient> = Arc::new(MockHttpClient::new(|req| {
            let query = req.url().query().unwrap_or_default().to_owned();
            let if_match = req
                .headers()
                .get_optional_str(&"if-match".into())
                .map(str::to_owned);
            let body: &[u8] = if query.contains("marker=m2") {
                assert_eq!(if_match.as_deref(), Some("etag-1"));
                LAYOUT_PAGE_2
            } else {
                assert_eq!(if_match.as_deref(), Some("etag-1"));
                LAYOUT_PAGE_1
            };
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    headers_with_etag("etag-1"),
                    Bytes::copy_from_slice(body),
                ))
            }
            .boxed()
        }));

        let client = layout_client(mock);
        let prefetch = fetch_layout(
            &client,
            &Context::new(),
            &BlobClientListLayoutOptions {
                if_match: Some(Etag::from("etag-1")),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .expect("routing should be available");

        assert_eq!(prefetch.etag, Some(Etag::from("etag-1")));
        assert_eq!(
            prefetch.layout.ideal_endpoint(0),
            Some("ep0.blob.storage.azure.net:443")
        );
        assert_eq!(
            prefetch.layout.ideal_endpoint(4_194_304),
            Some("ep1.blob.storage.azure.net:443")
        );
    }

    // This is a characterization test for the current emitter bug: 204 is an accepted
    // success response, but the generated pager attempts to deserialize its empty body as XML.
    // Once the emitter is fixed, this test should be changed to assert the corrected empty or
    // optional result generated for a successful 204 response.
    #[tokio::test]
    async fn generated_list_layout_fails_to_deserialize_valid_no_content_response() {
        let mock: Arc<dyn HttpClient> = Arc::new(MockHttpClient::new(|_req| {
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::NoContent,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));

        let client = layout_client(mock);
        let mut pages = client.list_layout(None).unwrap();

        let error = pages
            .next()
            .await
            .expect("the service returned one response")
            .expect_err("HTTP 204 should expose the generated XML deserialization bug");

        assert_eq!(*error.kind(), ErrorKind::DataConversion);
        assert!(error
            .to_string()
            .contains("failed to deserialize the following xml"));
    }

    #[tokio::test]
    async fn fetch_layout_empty_ranges_falls_back() {
        let mock: Arc<dyn HttpClient> = Arc::new(MockHttpClient::new(|_req| {
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    headers_with_etag("etag-1"),
                    Bytes::from_static(LAYOUT_EMPTY),
                ))
            }
            .boxed()
        }));

        let client = layout_client(mock);
        let result = fetch_layout(
            &client,
            &Context::new(),
            &BlobClientListLayoutOptions::default(),
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_layout_bad_request_falls_back() {
        let mock: Arc<dyn HttpClient> = Arc::new(MockHttpClient::new(|_req| {
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::BadRequest,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));

        let client = layout_client(mock);
        let result = fetch_layout(
            &client,
            &Context::new(),
            &BlobClientListLayoutOptions::default(),
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_layout_server_error_falls_back() {
        let mock: Arc<dyn HttpClient> = Arc::new(MockHttpClient::new(|_req| {
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::InternalServerError,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));

        let client = layout_client(mock);
        let result = fetch_layout(
            &client,
            &Context::new(),
            &BlobClientListLayoutOptions::default(),
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_layout_forbidden_fails() {
        let mock: Arc<dyn HttpClient> = Arc::new(MockHttpClient::new(|_req| {
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Forbidden,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));

        let client = layout_client(mock);
        let result = fetch_layout(
            &client,
            &Context::new(),
            &BlobClientListLayoutOptions::default(),
        )
        .await;
        assert!(result.is_err());
    }
}
