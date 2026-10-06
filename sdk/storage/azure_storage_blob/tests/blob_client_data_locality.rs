// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data-locality tests: the SDK's automatic layout-aware routing of managed downloads, the
//! caller-controlled `layout_endpoint` override, and both combined with session authentication.
//!
//! The file has three sections: routing against a mock transport, sessions with routing
//! against a mock transport, and live tests.

use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    http::{
        headers::Headers,
        policies::{Policy, PolicyResult},
        AsyncRawResponse, ClientOptions, Context, FixedRetryOptions, Request, RequestContent,
        RetryOptions, StatusCode, Transport, Url,
    },
    sleep::sleep,
    time::{Duration, OffsetDateTime},
    Bytes,
};
use azure_core_test::{credentials::MockCredential, http::MockHttpClient, recorded};
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{
    models::{BlobClientDownloadOptions, BlobLayout, EncryptionAlgorithmType, LayoutAwareRouting},
    BlobClient, BlobClientOptions, BlobContainerClient, BlobContainerClientOptions, SessionMode,
    SessionOptions,
};
use futures::{FutureExt as _, TryStreamExt};
use std::{
    collections::HashMap,
    error::Error,
    num::NonZero,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

const ACCOUNT_HOST: &str = "acct.blob.core.windows.net";
const ETAG: &str = "\"layout-test-etag\"";

fn parse_bytes_range(range: &str) -> (usize, usize) {
    let spec = range.strip_prefix("bytes=").expect("bytes= prefix");
    let (start, end) = spec.split_once('-').expect("start-end");
    (
        start.parse().expect("range start"),
        end.parse().expect("range end"),
    )
}

/// The operation a download-related request performs.
#[derive(Clone, Copy, Debug, PartialEq)]
enum RequestKind {
    CreateSession,
    Layout,
    Data,
}

impl RequestKind {
    fn of(url: &Url) -> Self {
        let query = url.query().unwrap_or_default();
        if query.contains("comp=session") {
            Self::CreateSession
        } else if query.contains("comp=layout") {
            Self::Layout
        } else {
            Self::Data
        }
    }
}

// Layout-aware routing, against a mock transport.

const BLOB_DATA: [u8; 12] = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21];

/// Three endpoints addressed by hostname, each serving a four-byte range of `BLOB_DATA`.
const LAYOUT: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<BlobLayout>
  <Endpoints>
    <Endpoint Index="0" Value="epa.blob.core.windows.net:443" />
    <Endpoint Index="1" Value="epb.blob.core.windows.net:443" />
    <Endpoint Index="2" Value="epc.blob.core.windows.net:443" />
  </Endpoints>
  <Ranges>
    <Range Start="0" End="3" EndpointIndex="0" />
    <Range Start="4" End="7" EndpointIndex="1" />
    <Range Start="8" End="11" EndpointIndex="2" />
  </Ranges>
</BlobLayout>"#;

/// The same shape as [`LAYOUT`], but with the non-default port an emulator would report.
const EMULATOR_LAYOUT: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<BlobLayout>
  <Endpoints>
    <Endpoint Index="0" Value="node-a.storage.local:20000" />
    <Endpoint Index="1" Value="node-b.storage.local:20000" />
    <Endpoint Index="2" Value="node-c.storage.local:20000" />
  </Endpoints>
  <Ranges>
    <Range Start="0" End="3" EndpointIndex="0" />
    <Range Start="4" End="7" EndpointIndex="1" />
    <Range Start="8" End="11" EndpointIndex="2" />
  </Ranges>
</BlobLayout>"#;

/// A request as the transport saw it, after any rewrite the routing policy applied.
#[derive(Clone)]
struct ObservedRequest {
    is_layout: bool,
    url: Url,
    original_host: Option<String>,
    range: Option<String>,
    headers: Headers,
}

/// Whether data responses carry `x-ms-download-hint`, which is what gates the SDK's
/// automatic layout lookup.
#[derive(Clone, Copy, PartialEq)]
enum LayoutHint {
    Advertised,
    Absent,
}

/// Builds a transport that serves `layout` for Get Blob Layout and slices `BLOB_DATA`
/// for range requests, recording where each request was sent.
fn layout_mock_transport(
    seen: Arc<Mutex<Vec<ObservedRequest>>>,
    layout: &'static [u8],
    hint: LayoutHint,
) -> Transport {
    Transport::new(Arc::new(MockHttpClient::new(move |request| {
        let is_layout = request
            .url()
            .query()
            .is_some_and(|query| query.contains("comp=layout"));
        let range = request
            .headers()
            .get_optional_str(&"range".into())
            .map(str::to_owned);
        seen.lock().unwrap().push(ObservedRequest {
            is_layout,
            url: request.url().clone(),
            original_host: request
                .headers()
                .get_optional_str(&"host".into())
                .map(str::to_owned),
            range: range.clone(),
            headers: request.headers().clone(),
        });

        let response = if is_layout {
            let mut headers = Headers::new();
            headers.insert("etag", ETAG);
            AsyncRawResponse::from_bytes(StatusCode::Ok, headers, Bytes::from_static(layout))
        } else {
            let range = range.expect("data request must carry a range header");
            let (start, end) = parse_bytes_range(&range);
            let slice = BLOB_DATA[start..=end].to_vec();
            let mut headers = Headers::new();
            headers.insert(
                "content-range",
                format!("bytes {start}-{end}/{}", BLOB_DATA.len()),
            );
            headers.insert("content-length", slice.len().to_string());
            headers.insert("etag", ETAG);
            if hint == LayoutHint::Advertised {
                headers.insert("x-ms-download-hint", "layout");
            }
            AsyncRawResponse::from_bytes(StatusCode::PartialContent, headers, Bytes::from(slice))
        };
        async move { Ok(response) }.boxed()
    })))
}

fn blob_client_with(transport: Transport, url: &str) -> Result<BlobClient, Box<dyn Error>> {
    Ok(BlobClient::new(
        Url::parse(url)?,
        None,
        Some(BlobClientOptions {
            client_options: ClientOptions {
                transport: Some(transport),
                ..Default::default()
            },
            ..Default::default()
        }),
    )?)
}

/// Mirrors the work a caller does with the pages returned by `get_layout()`: find the
/// endpoint whose layout range covers `offset`.
fn endpoint_for_offset(layout: &BlobLayout, offset: i64) -> Option<String> {
    let endpoints = layout.endpoints.as_ref()?.endpoint.as_ref()?;
    let index = layout
        .ranges
        .range
        .iter()
        .find(|range| {
            range.start.unwrap_or_default() <= offset && offset <= range.end.unwrap_or_default()
        })?
        .endpoint_index?;
    endpoints
        .iter()
        .find(|endpoint| endpoint.index == Some(index))
        .and_then(|endpoint| endpoint.value.clone())
}

#[tokio::test]
async fn test_download_layout_aware_routing_routes_chunks() -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::<ObservedRequest>::new()));
    let transport = layout_mock_transport(seen.clone(), LAYOUT, LayoutHint::Advertised);
    let blob_client = blob_client_with(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_aware_routing: LayoutAwareRouting::Enabled,
            partition_size: Some(NonZero::new(4).unwrap()),
            parallel: Some(NonZero::new(2).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &BLOB_DATA[..]);
    let seen = seen.lock().unwrap();
    let layout_requests: Vec<&ObservedRequest> =
        seen.iter().filter(|request| request.is_layout).collect();
    let data_requests: Vec<&ObservedRequest> =
        seen.iter().filter(|request| !request.is_layout).collect();

    assert_eq!(layout_requests.len(), 1);
    assert_eq!(layout_requests[0].url.host_str(), Some(ACCOUNT_HOST));

    assert_eq!(data_requests.len(), 3);
    for request in &data_requests {
        let range = request.range.as_deref().expect("range header");
        match range {
            "bytes=0-3" => assert_eq!(
                request.url.host_str(),
                Some(ACCOUNT_HOST),
                "the initial chunk should not be routed"
            ),
            "bytes=4-7" | "bytes=8-11" => {
                let expected = if range == "bytes=4-7" {
                    "epb.blob.core.windows.net"
                } else {
                    "epc.blob.core.windows.net"
                };
                assert_eq!(request.url.host_str(), Some(expected));
                assert_eq!(request.original_host.as_deref(), Some(ACCOUNT_HOST));
            }
            other => panic!("unexpected data range: {other}"),
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_download_layout_aware_routing_routes_chunks_for_path_style_endpoint(
) -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::<ObservedRequest>::new()));
    let transport = layout_mock_transport(seen.clone(), EMULATOR_LAYOUT, LayoutHint::Advertised);
    let blob_client = blob_client_with(
        transport,
        "http://127.0.0.1:10000/devstoreaccount1/container/blob",
    )?;

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_aware_routing: LayoutAwareRouting::Enabled,
            partition_size: Some(NonZero::new(4).unwrap()),
            parallel: Some(NonZero::new(2).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &BLOB_DATA[..]);
    let seen = seen.lock().unwrap();
    let layout_requests: Vec<&ObservedRequest> =
        seen.iter().filter(|request| request.is_layout).collect();
    let data_requests: Vec<&ObservedRequest> =
        seen.iter().filter(|request| !request.is_layout).collect();

    assert_eq!(layout_requests.len(), 1);
    assert_eq!(layout_requests[0].url.host_str(), Some("127.0.0.1"));

    assert_eq!(data_requests.len(), 3);
    for request in &data_requests {
        let range = request.range.as_deref().expect("range header");
        // Rerouting must not disturb the emulator's scheme or its account path segment.
        assert_eq!(request.url.scheme(), "http");
        assert_eq!(request.url.path(), "/devstoreaccount1/container/blob");
        match range {
            "bytes=0-3" => {
                assert_eq!(
                    request.url.host_str(),
                    Some("127.0.0.1"),
                    "the initial chunk should not be routed"
                );
                assert_eq!(request.url.port(), Some(10000));
                assert_eq!(request.original_host, None);
            }
            "bytes=4-7" | "bytes=8-11" => {
                let expected = if range == "bytes=4-7" {
                    "node-b.storage.local"
                } else {
                    "node-c.storage.local"
                };
                assert_eq!(request.url.host_str(), Some(expected));
                assert_eq!(request.url.port(), Some(20000));
                assert_eq!(request.original_host.as_deref(), Some("127.0.0.1:10000"));
            }
            other => panic!("unexpected data range: {other}"),
        }
    }
    Ok(())
}

/// Without a layout hint the SDK must not look the layout up, and must leave every
/// request pointed at the client's configured endpoint.
#[tokio::test]
async fn test_download_layout_aware_routing_skips_without_hint() -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::<ObservedRequest>::new()));
    let transport = layout_mock_transport(seen.clone(), LAYOUT, LayoutHint::Absent);
    let blob_client = blob_client_with(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_aware_routing: LayoutAwareRouting::Enabled,
            partition_size: Some(NonZero::new(4).unwrap()),
            parallel: Some(NonZero::new(2).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &BLOB_DATA[..]);
    let seen = seen.lock().unwrap();
    assert!(
        !seen.iter().any(|request| request.is_layout),
        "no layout should be fetched without a hint"
    );
    for request in seen.iter() {
        assert_eq!(request.url.host_str(), Some(ACCOUNT_HOST));
    }
    Ok(())
}

/// Routing is opt-in: with default options the SDK ignores an advertised layout, fetches
/// nothing, and sends every request to the client's configured endpoint.
#[tokio::test]
async fn test_download_layout_aware_routing_is_disabled_by_default() -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::<ObservedRequest>::new()));
    let transport = layout_mock_transport(seen.clone(), LAYOUT, LayoutHint::Advertised);
    let blob_client = blob_client_with(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            partition_size: Some(NonZero::new(4).unwrap()),
            parallel: Some(NonZero::new(2).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &BLOB_DATA[..]);
    let seen = seen.lock().unwrap();
    assert!(
        !seen.iter().any(|request| request.is_layout),
        "no layout should be fetched unless routing is enabled"
    );
    for request in seen.iter() {
        assert_eq!(request.url.host_str(), Some(ACCOUNT_HOST));
        assert_eq!(request.original_host, None);
    }
    Ok(())
}

/// A download that fits in one partition is served by the initial request, which is
/// never routed, so the layout is not worth fetching.
#[tokio::test]
async fn test_download_layout_aware_routing_skips_layout_for_complete_initial_range(
) -> Result<(), Box<dyn Error>> {
    const DATA: [u8; 8] = [10, 11, 12, 13, 14, 15, 16, 17];

    let request_count = Arc::new(AtomicUsize::new(0));
    let count_capture = request_count.clone();
    let mock_client = Arc::new(MockHttpClient::new(move |request| {
        assert_eq!(0, count_capture.fetch_add(1, Ordering::SeqCst));
        assert!(!request
            .url()
            .query()
            .is_some_and(|query| query.contains("comp=layout")));
        assert_eq!(
            Some("bytes=0-7"),
            request.headers().get_optional_str(&"range".into())
        );

        let mut headers = Headers::new();
        headers.insert("content-range", "bytes 0-7/8");
        headers.insert("content-length", DATA.len().to_string());
        headers.insert("etag", ETAG);
        headers.insert("x-ms-download-hint", "layout");
        async move {
            Ok(AsyncRawResponse::from_bytes(
                StatusCode::PartialContent,
                headers,
                Bytes::from_static(&DATA),
            ))
        }
        .boxed()
    }));

    let blob_client = blob_client_with(
        Transport::new(mock_client),
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_aware_routing: LayoutAwareRouting::Enabled,
            partition_size: Some(NonZero::new(DATA.len()).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &DATA[..]);
    assert_eq!(request_count.load(Ordering::SeqCst), 1);
    Ok(())
}

/// A `BlobClient` reached through a container client shares that client's pipeline, so
/// this covers the routing policy being installed for every client rather than only the
/// ones built by `BlobClient::new`.
#[tokio::test]
async fn test_download_layout_aware_routing_from_container_client() -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::<ObservedRequest>::new()));
    let transport = layout_mock_transport(seen.clone(), LAYOUT, LayoutHint::Advertised);

    let container_client = BlobContainerClient::new(
        Url::parse("https://acct.blob.core.windows.net/container")?,
        None,
        Some(BlobContainerClientOptions {
            client_options: ClientOptions {
                transport: Some(transport),
                ..Default::default()
            },
            ..Default::default()
        }),
    )?;
    let blob_client = container_client.blob_client("blob");

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_aware_routing: LayoutAwareRouting::Enabled,
            partition_size: Some(NonZero::new(4).unwrap()),
            parallel: Some(NonZero::new(2).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &BLOB_DATA[..]);
    let seen = seen.lock().unwrap();
    let routed: HashMap<String, String> = seen
        .iter()
        .filter(|request| !request.is_layout)
        .map(|request| {
            (
                request.range.clone().expect("range header"),
                request.url.host_str().unwrap_or_default().to_owned(),
            )
        })
        .collect();
    assert_eq!(
        routed.get("bytes=4-7").map(String::as_str),
        Some("epb.blob.core.windows.net"),
        "a blob client from a container client should still route"
    );
    assert_eq!(
        routed.get("bytes=8-11").map(String::as_str),
        Some("epc.blob.core.windows.net")
    );
    Ok(())
}

/// The layout request carries the download's snapshot, version, range, encryption key,
/// lease, conditions, and timeout, so the layout describes the same blob the chunks read.
#[tokio::test]
async fn test_download_layout_request_carries_download_options() -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::<ObservedRequest>::new()));
    let transport = layout_mock_transport(seen.clone(), LAYOUT, LayoutHint::Advertised);
    let blob_client = blob_client_with(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;
    let now = OffsetDateTime::now_utc();

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_aware_routing: LayoutAwareRouting::Enabled,
            partition_size: Some(NonZero::new(4).unwrap()),
            parallel: Some(NonZero::new(2).unwrap()),
            range: Some((2u64..10).into()),
            snapshot: Some("2026-01-01T00:00:00.0000000Z".to_owned()),
            version_id: Some("2026-01-02T00:00:00.0000000Z".to_owned()),
            encryption_algorithm: Some(EncryptionAlgorithmType::Aes256),
            encryption_key: Some("a2V5".to_owned()),
            encryption_key_sha256: Some("aGFzaA==".to_owned()),
            lease_id: Some("lease-1".to_owned()),
            if_modified_since: Some(now - Duration::days(1)),
            if_unmodified_since: Some(now + Duration::days(1)),
            if_tags: Some("\"k\" = 'v'".to_owned()),
            timeout: Some(30),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &BLOB_DATA[2..10]);
    let seen = seen.lock().unwrap();
    let layout = seen
        .iter()
        .find(|request| request.is_layout)
        .expect("the layout should be fetched");
    let query: HashMap<String, String> = layout.url.query_pairs().into_owned().collect();
    assert_eq!(
        query.get("snapshot").map(String::as_str),
        Some("2026-01-01T00:00:00.0000000Z")
    );
    assert_eq!(
        query.get("versionid").map(String::as_str),
        Some("2026-01-02T00:00:00.0000000Z")
    );
    assert_eq!(query.get("timeout").map(String::as_str), Some("30"));
    let header = |name: &'static str| layout.headers.get_optional_str(&name.into());
    assert_eq!(layout.range.as_deref(), Some("bytes=2-9"));
    // Pinned to the version the first chunk read.
    assert_eq!(header("if-match"), Some(ETAG));
    assert_eq!(header("x-ms-encryption-algorithm"), Some("AES256"));
    assert_eq!(header("x-ms-encryption-key"), Some("a2V5"));
    assert_eq!(header("x-ms-encryption-key-sha256"), Some("aGFzaA=="));
    assert_eq!(header("x-ms-lease-id"), Some("lease-1"));
    assert_eq!(header("x-ms-if-tags"), Some("\"k\" = 'v'"));
    assert!(header("if-modified-since").is_some());
    assert!(header("if-unmodified-since").is_some());
    Ok(())
}

/// A caller can fetch the layout themselves and pin a download to the endpoint serving
/// the range they want, without the SDK looking up the layout on their behalf.
#[tokio::test]
async fn test_download_layout_endpoint_selected_by_caller() -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::<ObservedRequest>::new()));
    // No layout hint, so nothing but the caller's endpoint can cause a rewrite.
    let transport = layout_mock_transport(seen.clone(), LAYOUT, LayoutHint::Absent);
    let blob_client = blob_client_with(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let mut pages = blob_client.get_layout(None)?;
    let layout = pages
        .try_next()
        .await?
        .expect("a layout page")
        .into_model()?;
    let endpoint = endpoint_for_offset(&layout, 4).expect("an endpoint covering offset 4");
    assert_eq!(endpoint, "epb.blob.core.windows.net:443");

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_endpoint: Some(endpoint),
            range: Some((4u64..8).into()),
            partition_size: Some(NonZero::new(4).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &BLOB_DATA[4..8]);
    let seen = seen.lock().unwrap();
    let layout_requests: Vec<&ObservedRequest> =
        seen.iter().filter(|request| request.is_layout).collect();
    let data_requests: Vec<&ObservedRequest> =
        seen.iter().filter(|request| !request.is_layout).collect();

    assert_eq!(
        layout_requests.len(),
        1,
        "only the caller's own get_layout call should be issued"
    );
    assert_eq!(layout_requests[0].url.host_str(), Some(ACCOUNT_HOST));

    assert_eq!(data_requests.len(), 1);
    assert_eq!(data_requests[0].range.as_deref(), Some("bytes=4-7"));
    assert_eq!(
        data_requests[0].url.host_str(),
        Some("epb.blob.core.windows.net"),
        "the download should be sent to the caller's endpoint"
    );
    assert_eq!(
        data_requests[0].original_host.as_deref(),
        Some(ACCOUNT_HOST),
        "the account authority should be preserved as the Host header"
    );
    Ok(())
}

/// A caller-supplied endpoint takes precedence over automatic routing: the layout is
/// never fetched and every request the download issues is pinned, including the first.
#[tokio::test]
async fn test_download_layout_endpoint_overrides_layout_aware_routing() -> Result<(), Box<dyn Error>>
{
    let seen = Arc::new(Mutex::new(Vec::<ObservedRequest>::new()));
    let transport = layout_mock_transport(seen.clone(), LAYOUT, LayoutHint::Advertised);
    let blob_client = blob_client_with(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_endpoint: Some("epa.blob.core.windows.net:443".into()),
            // Left at its default of `Enabled` to show the explicit endpoint wins.
            partition_size: Some(NonZero::new(4).unwrap()),
            parallel: Some(NonZero::new(2).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;

    assert_eq!(&body[..], &BLOB_DATA[..]);
    let seen = seen.lock().unwrap();
    assert!(
        !seen.iter().any(|request| request.is_layout),
        "an explicit endpoint should suppress the automatic layout lookup"
    );
    assert_eq!(seen.len(), 3);
    for request in seen.iter() {
        assert_eq!(
            request.url.host_str(),
            Some("epa.blob.core.windows.net"),
            "every request, including the first, should be pinned"
        );
        assert_eq!(request.original_host.as_deref(), Some(ACCOUNT_HOST));
    }
    Ok(())
}

// Session authentication with layout-aware routing, against a mock transport.

/// A 32-byte blob split across two endpoints, for downloads with enough chunks to overlap.
const SESSION_MOCK_LAYOUT: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<BlobLayout>
  <Endpoints>
    <Endpoint Index="0" Value="epa.blob.core.windows.net:443" />
    <Endpoint Index="1" Value="epb.blob.core.windows.net:443" />
  </Endpoints>
  <Ranges>
    <Range Start="0" End="15" EndpointIndex="0" />
    <Range Start="16" End="31" EndpointIndex="1" />
  </Ranges>
</BlobLayout>"#;

const SESSION_MOCK_LEN: usize = 32;
const SESSION_MOCK_CONTAINER: &str = "container";

fn session_mock_data() -> Vec<u8> {
    (0..SESSION_MOCK_LEN)
        .map(|index| index as u8 + 100)
        .collect()
}

/// A request as the session-and-routing mock saw it, with the status it answered.
#[derive(Clone, Debug)]
struct SessionMockRequest {
    kind: RequestKind,
    host: String,
    path: String,
    host_header: Option<String>,
    auth: Option<String>,
    range: Option<String>,
    status: StatusCode,
    /// The mock failed the request at the transport, so `status` is meaningless.
    connection_failed: bool,
}

impl SessionMockRequest {
    fn session_token(&self) -> Option<&str> {
        self.auth
            .as_deref()?
            .strip_prefix("Session ")?
            .split(':')
            .next()
    }

    fn is_bearer(&self) -> bool {
        self.auth
            .as_deref()
            .is_some_and(|auth| auth.starts_with("Bearer "))
    }
}

/// How the mock fails a data request instead of serving it.
#[derive(Clone, Copy, Debug, PartialEq)]
enum MockFailure {
    Status(StatusCode),
    /// The connection fails, as it would for an unreachable node.
    Connection,
}

/// Returns a failure to inject for a data request instead of serving it.
type MockFault = Box<dyn Fn(&SessionMockRequest) -> Option<MockFailure> + Send + Sync>;

/// A fault that fires once, for the first data request `matches` accepts.
fn fault_once(
    status: StatusCode,
    matches: impl Fn(&SessionMockRequest) -> bool + Send + Sync + 'static,
) -> MockFault {
    let fired = AtomicBool::new(false);
    Box::new(move |seen: &SessionMockRequest| {
        (matches(seen) && !fired.swap(true, Ordering::SeqCst))
            .then_some(MockFailure::Status(status))
    })
}

fn no_fault() -> MockFault {
    Box::new(|_: &SessionMockRequest| None)
}

/// Serves Create Session (minting `token-1`, `token-2`, ... for the blob's container), Get
/// Blob Layout (`500` when `layout` is empty), and ranged reads of [`session_mock_data`],
/// injecting `fault` into data requests.
fn session_mock_transport(
    seen: Arc<Mutex<Vec<SessionMockRequest>>>,
    layout: &'static [u8],
    create_session_status: StatusCode,
    fault: MockFault,
) -> Transport {
    let sessions = AtomicUsize::new(0);
    let data = session_mock_data();
    Transport::new(Arc::new(MockHttpClient::new(move |request| {
        let kind = RequestKind::of(request.url());
        let header = |name: &'static str| {
            request
                .headers()
                .get_optional_str(&name.into())
                .map(str::to_owned)
        };
        let mut observed = SessionMockRequest {
            kind,
            host: request.url().host_str().unwrap_or_default().to_owned(),
            path: request.url().path().to_owned(),
            host_header: header("host"),
            auth: header("authorization"),
            range: header("range"),
            status: StatusCode::Ok,
            connection_failed: false,
        };

        let mut headers = Headers::new();
        let mut delay = false;
        let (status, body) = match kind {
            RequestKind::CreateSession => {
                let container = observed.path.rsplit('/').next().unwrap_or_default();
                if container != SESSION_MOCK_CONTAINER {
                    headers.insert("x-ms-error-code", "ContainerNotFound");
                    (StatusCode::NotFound, Bytes::new())
                } else if create_session_status == StatusCode::Created {
                    let token = sessions.fetch_add(1, Ordering::SeqCst) + 1;
                    let xml = format!(
                        r#"<?xml version="1.0" encoding="utf-8"?>
<CreateSessionResult>
  <AuthenticationType>HMAC</AuthenticationType>
  <Credentials>
    <SessionKey>c2Vzc2lvbi1rZXk=</SessionKey>
    <SessionToken>token-{token}</SessionToken>
  </Credentials>
  <Expiration>Wed, 01 Jan 2031 00:00:00 GMT</Expiration>
  <Id>session-{token}</Id>
</CreateSessionResult>"#
                    );
                    (StatusCode::Created, Bytes::from(xml))
                } else {
                    (create_session_status, Bytes::new())
                }
            }
            RequestKind::Layout if layout.is_empty() => {
                (StatusCode::InternalServerError, Bytes::new())
            }
            RequestKind::Layout => {
                headers.insert("etag", ETAG);
                (StatusCode::Ok, Bytes::from_static(layout))
            }
            RequestKind::Data => match fault(&observed) {
                Some(MockFailure::Connection) => {
                    observed.connection_failed = true;
                    seen.lock().unwrap().push(observed);
                    return async {
                        Err(azure_core::Error::with_message(
                            azure_core::error::ErrorKind::Io,
                            "mock connection reset",
                        ))
                    }
                    .boxed();
                }
                Some(MockFailure::Status(status)) => {
                    // Hold injected failures briefly so concurrent chunks are in flight together.
                    delay = true;
                    if status == StatusCode::Unauthorized {
                        headers.insert(
                            "www-authenticate",
                            "Session error=\"session_token_invalid\"",
                        );
                        headers.insert("x-ms-error-code", "InvalidAuthenticationInfo");
                    }
                    (status, Bytes::new())
                }
                None => {
                    let range = observed
                        .range
                        .as_deref()
                        .expect("data request must carry a range header");
                    let (start, end) = parse_bytes_range(range);
                    let end = end.min(data.len() - 1);
                    headers.insert(
                        "content-range",
                        format!("bytes {start}-{end}/{}", data.len()),
                    );
                    headers.insert("content-length", (end + 1 - start).to_string());
                    headers.insert("etag", ETAG);
                    headers.insert("x-ms-download-hint", "layout");
                    (
                        StatusCode::PartialContent,
                        Bytes::copy_from_slice(&data[start..=end]),
                    )
                }
            },
        };

        observed.status = status;
        seen.lock().unwrap().push(observed);
        let response = AsyncRawResponse::from_bytes(status, headers, body);
        async move {
            if delay {
                sleep(Duration::milliseconds(50)).await;
            }
            Ok(response)
        }
        .boxed()
    })))
}

fn session_mock_client(transport: Transport, url: &str) -> Result<BlobClient, Box<dyn Error>> {
    let credential: Arc<dyn TokenCredential> = MockCredential::new()?;
    Ok(BlobClient::new(
        Url::parse(url)?,
        Some(credential),
        Some(BlobClientOptions {
            client_options: ClientOptions {
                transport: Some(transport),
                retry: RetryOptions::fixed(FixedRetryOptions {
                    delay: Duration::milliseconds(1),
                    ..Default::default()
                }),
                ..Default::default()
            },
            session_options: Some(SessionOptions {
                mode: SessionMode::Enabled,
                ..Default::default()
            }),
            ..Default::default()
        }),
    )?)
}

async fn session_mock_download(
    client: &BlobClient,
    partition_size: usize,
    parallel: usize,
    layout_endpoint: Option<&str>,
) -> azure_core::Result<Bytes> {
    client
        .download(Some(BlobClientDownloadOptions {
            layout_aware_routing: LayoutAwareRouting::Enabled,
            layout_endpoint: layout_endpoint.map(str::to_owned),
            partition_size: Some(NonZero::new(partition_size).unwrap()),
            parallel: Some(NonZero::new(parallel).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await
}

/// Create Session and Get Blob Layout always go to the account endpoint, never a node.
fn assert_control_requests_not_routed(seen: &[SessionMockRequest], account_host: &str) {
    for request in seen.iter().filter(|r| r.kind != RequestKind::Data) {
        assert_eq!(request.host, account_host, "{request:?} was routed");
        assert_eq!(
            request.host_header, None,
            "{request:?} carried a Host header"
        );
    }
}

fn count_kind(seen: &[SessionMockRequest], kind: RequestKind) -> usize {
    seen.iter().filter(|r| r.kind == kind).count()
}

/// A caller-chosen endpoint routes every chunk, the first included, but the Create Session
/// it triggers still goes to the account endpoint.
#[tokio::test]
async fn test_session_routing_caller_endpoint_keeps_create_session_on_account(
) -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let transport = session_mock_transport(
        seen.clone(),
        SESSION_MOCK_LAYOUT,
        StatusCode::Created,
        no_fault(),
    );
    let client = session_mock_client(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = session_mock_download(&client, 4, 2, Some("epb.blob.core.windows.net:443")).await?;

    assert_eq!(&body[..], &session_mock_data()[..]);
    let seen = seen.lock().unwrap();
    assert_control_requests_not_routed(&seen, ACCOUNT_HOST);
    assert_eq!(count_kind(&seen, RequestKind::CreateSession), 1);
    assert_eq!(count_kind(&seen, RequestKind::Layout), 0);
    for request in seen.iter().filter(|r| r.kind == RequestKind::Data) {
        assert_eq!(request.host, "epb.blob.core.windows.net");
        assert_eq!(request.host_header.as_deref(), Some(ACCOUNT_HOST));
        assert_eq!(request.session_token(), Some("token-1"));
    }
    Ok(())
}

/// A 401 on a routed chunk drops the session and sends that chunk again with bearer to the same
/// node, keeping the account `Host` header; the next chunk then mints a new session.
#[tokio::test]
async fn test_session_routing_unauthorized_routed_chunk_is_resent_with_bearer_to_node(
) -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let fault = fault_once(StatusCode::Unauthorized, |r| {
        r.host != ACCOUNT_HOST && r.session_token().is_some()
    });
    let transport = session_mock_transport(
        seen.clone(),
        SESSION_MOCK_LAYOUT,
        StatusCode::Created,
        fault,
    );
    let client = session_mock_client(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = session_mock_download(&client, 4, 1, None).await?;

    assert_eq!(&body[..], &session_mock_data()[..]);
    let seen = seen.lock().unwrap();
    assert_control_requests_not_routed(&seen, ACCOUNT_HOST);
    let rejected = seen
        .iter()
        .position(|r| r.status == StatusCode::Unauthorized)
        .expect("one routed chunk was rejected");
    let resend = &seen[rejected + 1];
    assert_eq!(resend.range, seen[rejected].range);
    assert!(
        resend.is_bearer(),
        "the resend should use bearer: {resend:?}"
    );
    assert_eq!(resend.host, seen[rejected].host);
    assert_eq!(resend.host_header.as_deref(), Some(ACCOUNT_HOST));
    assert_eq!(resend.status, StatusCode::PartialContent);
    assert_eq!(count_kind(&seen, RequestKind::CreateSession), 2);
    assert!(seen[rejected + 2..]
        .iter()
        .filter(|r| r.kind == RequestKind::Data)
        .all(|r| r.session_token() == Some("token-2")));
    Ok(())
}

/// When every routed chunk in flight is rejected with the same session, each falls back
/// to bearer on its node and only one replacement session is minted between them.
#[tokio::test]
async fn test_session_routing_concurrent_unauthorized_mints_one_new_session(
) -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let fault: MockFault = Box::new(|r: &SessionMockRequest| {
        (r.host != ACCOUNT_HOST && r.session_token() == Some("token-1"))
            .then_some(MockFailure::Status(StatusCode::Unauthorized))
    });
    let transport = session_mock_transport(
        seen.clone(),
        SESSION_MOCK_LAYOUT,
        StatusCode::Created,
        fault,
    );
    let client = session_mock_client(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = session_mock_download(&client, 2, 8, None).await?;

    assert_eq!(&body[..], &session_mock_data()[..]);
    let seen = seen.lock().unwrap();
    assert_control_requests_not_routed(&seen, ACCOUNT_HOST);
    let rejected: Vec<&SessionMockRequest> = seen
        .iter()
        .filter(|r| r.status == StatusCode::Unauthorized)
        .collect();
    assert!(
        rejected.len() > 1,
        "expected several chunks rejected together, saw {}",
        rejected.len()
    );
    for request in &rejected {
        assert!(
            seen.iter().any(|r| r.is_bearer()
                && r.range == request.range
                && r.host == request.host
                && r.host_header.as_deref() == Some(ACCOUNT_HOST)
                && r.status == StatusCode::PartialContent),
            "{request:?} was not resent with bearer to its node"
        );
    }
    assert_eq!(count_kind(&seen, RequestKind::CreateSession), 2);
    assert!(seen
        .iter()
        .filter(|r| r.session_token() == Some("token-2"))
        .all(|r| r.status == StatusCode::PartialContent));
    Ok(())
}

/// A transient failure on a routed session chunk is retried on the same node, re-signed
/// with the same session.
#[tokio::test]
async fn test_session_routing_retry_stays_on_node_with_session() -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let fault = fault_once(StatusCode::ServiceUnavailable, |r| {
        r.host != ACCOUNT_HOST && r.session_token().is_some()
    });
    let transport = session_mock_transport(
        seen.clone(),
        SESSION_MOCK_LAYOUT,
        StatusCode::Created,
        fault,
    );
    let client = session_mock_client(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = session_mock_download(&client, 4, 1, None).await?;

    assert_eq!(&body[..], &session_mock_data()[..]);
    let seen = seen.lock().unwrap();
    assert_control_requests_not_routed(&seen, ACCOUNT_HOST);
    let failed = seen
        .iter()
        .position(|r| r.status == StatusCode::ServiceUnavailable)
        .expect("one routed chunk failed transiently");
    let retry = &seen[failed + 1];
    assert_eq!(retry.range, seen[failed].range);
    assert_eq!(retry.session_token(), Some("token-1"));
    assert_eq!(retry.host, seen[failed].host);
    assert_eq!(retry.host_header.as_deref(), Some(ACCOUNT_HOST));
    assert_eq!(retry.status, StatusCode::PartialContent);
    assert_eq!(count_kind(&seen, RequestKind::CreateSession), 1);
    Ok(())
}

/// Records current behavior: a routed chunk whose node cannot be reached is retried on that
/// node, still signed with the session, and the download fails once retries run out; it is
/// not redirected to the account endpoint.
#[tokio::test]
async fn test_session_routing_unreachable_node_retries_on_node_then_fails(
) -> Result<(), Box<dyn Error>> {
    const DOWN_NODE: &str = "epb.blob.core.windows.net";

    let seen = Arc::new(Mutex::new(Vec::new()));
    let fault: MockFault =
        Box::new(|r: &SessionMockRequest| (r.host == DOWN_NODE).then_some(MockFailure::Connection));
    let transport = session_mock_transport(
        seen.clone(),
        SESSION_MOCK_LAYOUT,
        StatusCode::Created,
        fault,
    );
    let client = session_mock_client(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let result = session_mock_download(&client, 4, 1, None).await;

    assert!(
        result.is_err(),
        "the download should fail when a node is unreachable"
    );
    let seen = seen.lock().unwrap();
    assert_control_requests_not_routed(&seen, ACCOUNT_HOST);
    let failed: Vec<&SessionMockRequest> = seen.iter().filter(|r| r.connection_failed).collect();
    assert!(
        failed.len() > 1,
        "the unreachable node should be retried: {failed:#?}"
    );
    let range = failed[0].range.clone();
    for request in seen
        .iter()
        .filter(|r| r.kind == RequestKind::Data && r.range == range)
    {
        assert_eq!(request.host, DOWN_NODE, "{request:?} left the node");
        assert_eq!(request.host_header.as_deref(), Some(ACCOUNT_HOST));
        assert_eq!(request.session_token(), Some("token-1"));
    }
    assert_eq!(count_kind(&seen, RequestKind::CreateSession), 1);
    Ok(())
}

/// When Create Session fails, routed chunks still go to their nodes, authenticated with bearer.
#[tokio::test]
async fn test_session_routing_create_session_failure_routes_with_bearer(
) -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let transport = session_mock_transport(
        seen.clone(),
        SESSION_MOCK_LAYOUT,
        StatusCode::Forbidden,
        no_fault(),
    );
    let client = session_mock_client(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = session_mock_download(&client, 4, 2, None).await?;

    assert_eq!(&body[..], &session_mock_data()[..]);
    let seen = seen.lock().unwrap();
    assert_control_requests_not_routed(&seen, ACCOUNT_HOST);
    assert_eq!(count_kind(&seen, RequestKind::CreateSession), 1);
    let data: Vec<&SessionMockRequest> = seen
        .iter()
        .filter(|r| r.kind == RequestKind::Data)
        .collect();
    assert!(data.iter().all(|r| r.is_bearer()), "{data:#?}");
    let routed: Vec<&&SessionMockRequest> =
        data.iter().filter(|r| r.host != ACCOUNT_HOST).collect();
    assert_eq!(
        routed.len(),
        data.len() - 1,
        "every chunk after the first is routed"
    );
    assert!(routed
        .iter()
        .all(|r| r.host_header.as_deref() == Some(ACCOUNT_HOST)));
    Ok(())
}

/// A Get Blob Layout server error is a soft failure: the download continues on the account
/// endpoint, still authenticated with the session.
#[tokio::test]
async fn test_session_routing_layout_server_error_continues_with_session(
) -> Result<(), Box<dyn Error>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    // An empty layout makes the mock answer Get Blob Layout with 500.
    let transport = session_mock_transport(seen.clone(), b"", StatusCode::Created, no_fault());
    let client = session_mock_client(
        transport,
        "https://acct.blob.core.windows.net/container/blob",
    )?;

    let body = session_mock_download(&client, 4, 2, None).await?;

    assert_eq!(&body[..], &session_mock_data()[..]);
    let seen = seen.lock().unwrap();
    assert_control_requests_not_routed(&seen, ACCOUNT_HOST);
    assert!(count_kind(&seen, RequestKind::Layout) >= 1);
    assert_eq!(count_kind(&seen, RequestKind::CreateSession), 1);
    for request in seen.iter().filter(|r| r.kind == RequestKind::Data) {
        assert_eq!(request.host, ACCOUNT_HOST, "{request:?} was routed");
        assert_eq!(request.session_token(), Some("token-1"));
    }
    Ok(())
}

// Live tests against the account named by `AZURE_STORAGE_ACCOUNT_NAME`.

/// The URL of `blob_name` in the live test container. Each live test uses its own blob, so
/// tests running in parallel never upload to the same blob at once.
fn live_layout_blob_url(blob_name: &str) -> Option<Url> {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT_NAME").ok()?;
    let account = account.trim().trim_matches('"').trim();
    if account.is_empty() {
        return None;
    }
    Url::parse(&format!(
        "https://{account}.blob.core.windows.net/layout-routing-test/{blob_name}"
    ))
    .ok()
}

/// Creates the container holding `blob_url`, tolerating one that already exists.
async fn create_live_container(
    blob_url: &Url,
    credential: Arc<dyn TokenCredential>,
) -> Result<(), Box<dyn Error>> {
    let mut container_url = blob_url.clone();
    container_url
        .path_segments_mut()
        .expect("blob URL must be a base")
        .pop();
    let container_client = BlobContainerClient::new(container_url, Some(credential), None)?;
    if let Err(error) = container_client.create(None).await {
        if error.http_status() != Some(StatusCode::Conflict) {
            return Err(error.into());
        }
    }
    Ok(())
}

/// Records where each attempt was sent and how it was authenticated. Clients copy their
/// policies into their session provider, so this also sees Create Session.
#[derive(Debug)]
struct RoutingObserver {
    seen: Arc<Mutex<Vec<RoutedRequest>>>,
}

#[derive(Debug)]
struct RoutedRequest {
    kind: RequestKind,
    range: Option<String>,
    sent_to: Option<String>,
    original_host: Option<String>,
    session: bool,
    download_hint: Option<String>,
    status: StatusCode,
}

#[async_trait]
impl Policy for RoutingObserver {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let kind = RequestKind::of(request.url());
        let range = request
            .headers()
            .get_optional_str(&"range".into())
            .map(str::to_owned);
        let sent_to = request.url().host_str().map(str::to_owned);
        let original_host = request
            .headers()
            .get_optional_str(&"host".into())
            .map(str::to_owned);
        let session = request
            .headers()
            .get_optional_str(&"authorization".into())
            .is_some_and(|auth| auth.starts_with("Session "));
        let response = next[0].send(ctx, request, &next[1..]).await?;
        let download_hint = response
            .headers()
            .get_optional_str(&"x-ms-download-hint".into())
            .map(str::to_owned);
        self.seen.lock().unwrap().push(RoutedRequest {
            kind,
            range,
            sent_to,
            original_host,
            session,
            download_hint,
            status: response.status(),
        });
        Ok(response)
    }
}

#[recorded::test(live)]
async fn test_download_layout_aware_routing() -> Result<(), Box<dyn Error>> {
    let Some(url) = live_layout_blob_url("routing-test-blob") else {
        eprintln!(
            "skipping test_download_layout_aware_routing: set AZURE_STORAGE_ACCOUNT_NAME to run it"
        );
        return Ok(());
    };
    let credential: Arc<dyn TokenCredential> = DeveloperToolsCredential::new(None)?;
    let account_host = url.host_str().unwrap_or_default().to_owned();
    let observed = Arc::new(Mutex::new(Vec::<RoutedRequest>::new()));
    let observer: Arc<dyn Policy> = Arc::new(RoutingObserver {
        seen: observed.clone(),
    });

    create_live_container(&url, credential.clone()).await?;

    let blob_client = BlobClient::new(
        url,
        Some(credential),
        Some(BlobClientOptions {
            client_options: ClientOptions {
                per_try_policies: vec![observer],
                ..Default::default()
            },
            ..Default::default()
        }),
    )?;

    const BLOB_SIZE: usize = 64 * 1024 * 1024;
    let data: Vec<u8> = (0..BLOB_SIZE).map(|index| (index % 251) as u8).collect();
    blob_client
        .upload(RequestContent::from(data.clone()), None)
        .await?;
    observed.lock().unwrap().clear();

    let body = blob_client
        .download(Some(BlobClientDownloadOptions {
            layout_aware_routing: LayoutAwareRouting::Enabled,
            partition_size: Some(NonZero::new(16 * 1024 * 1024).unwrap()),
            parallel: Some(NonZero::new(4).unwrap()),
            ..Default::default()
        }))
        .await?
        .body
        .collect()
        .await?;
    assert_eq!(&body[..], &data[..]);

    let mut buffer = vec![0u8; data.len()];
    let result = blob_client
        .download_into(
            &mut buffer,
            Some(BlobClientDownloadOptions {
                layout_aware_routing: LayoutAwareRouting::Enabled,
                partition_size: Some(NonZero::new(16 * 1024 * 1024).unwrap()),
                parallel: Some(NonZero::new(4).unwrap()),
                ..Default::default()
            }),
        )
        .await?;
    assert_eq!(result.len, data.len());
    assert_eq!(&buffer[..], &data[..]);

    let observed = observed.lock().unwrap();
    let layout_calls = observed
        .iter()
        .filter(|request| request.kind == RequestKind::Layout)
        .count();
    let data_chunks = observed
        .iter()
        .filter(|request| request.kind == RequestKind::Data)
        .count();
    let routed: Vec<&RoutedRequest> = observed
        .iter()
        .filter(|request| request.kind == RequestKind::Data && request.original_host.is_some())
        .collect();

    eprintln!();
    eprintln!("=== layout-aware routing ======================================");
    eprintln!("  account host          : {account_host}");
    eprintln!("  Get Blob Layout calls : {layout_calls}");
    eprintln!(
        "  data chunks           : {data_chunks} ({} routed)",
        routed.len()
    );
    for request in observed.iter() {
        let kind = match request.kind {
            RequestKind::CreateSession => "session",
            RequestKind::Layout => "layout",
            RequestKind::Data => "data",
        };
        let range = request.range.as_deref().unwrap_or("(whole blob)");
        let sent_to = request.sent_to.as_deref().unwrap_or("?");
        let hint = request.download_hint.as_deref().unwrap_or("-");
        let routed = if request.original_host.is_some() {
            "yes"
        } else {
            "no"
        };
        let status = request.status;
        eprintln!("  {kind:<6}  {status:<3}  {range:<24}  {routed:<7}  {hint:<7}  {sent_to}");
    }

    if routed.is_empty() {
        eprintln!(
            "NOTE: no chunk was rewritten - the account returned no layout hint for this blob"
        );
    } else {
        for request in &routed {
            assert_eq!(
                request.original_host.as_deref(),
                Some(account_host.as_str()),
                "a rewritten chunk did not preserve the account Host header"
            );
        }
    }

    Ok(())
}

/// Sessions and layout-aware routing together against the real service: the download
/// completes with the right bytes, the session is used, and chunks are routed when the
/// account offers a layout.
///
/// How the client handles rejected or retried chunks is covered by the mock tests above.
/// Sessions are bound to the caller's network, so where outbound connections leave from
/// different addresses the service may reject chunks that open new connections; those fall
/// back to bearer and still succeed.
#[recorded::test(live)]
async fn test_download_session_with_layout_aware_routing() -> Result<(), Box<dyn Error>> {
    const BLOB_SIZE: usize = 64 * 1024 * 1024;
    const PARTITION_SIZE: usize = 8 * 1024 * 1024;

    let Some(url) = live_layout_blob_url("session-routing-test-blob") else {
        eprintln!(
            "skipping test_download_session_with_layout_aware_routing: set AZURE_STORAGE_ACCOUNT_NAME to run it"
        );
        return Ok(());
    };
    let credential: Arc<dyn TokenCredential> = DeveloperToolsCredential::new(None)?;

    create_live_container(&url, credential.clone()).await?;
    let data: Vec<u8> = (0..BLOB_SIZE).map(|index| (index % 251) as u8).collect();
    BlobClient::new(url.clone(), Some(credential.clone()), None)?
        .upload(RequestContent::from(data.clone()), None)
        .await?;

    let observed = Arc::new(Mutex::new(Vec::<RoutedRequest>::new()));
    let observer: Arc<dyn Policy> = Arc::new(RoutingObserver {
        seen: observed.clone(),
    });
    let blob_client = BlobClient::new(
        url,
        Some(credential),
        Some(BlobClientOptions {
            client_options: ClientOptions {
                per_try_policies: vec![observer],
                ..Default::default()
            },
            session_options: Some(SessionOptions {
                mode: SessionMode::Enabled,
                ..Default::default()
            }),
            ..Default::default()
        }),
    )?;

    for parallel in [1usize, 8] {
        observed.lock().unwrap().clear();
        let mut buffer = vec![0u8; BLOB_SIZE];
        blob_client
            .download_into(
                &mut buffer,
                Some(BlobClientDownloadOptions {
                    layout_aware_routing: LayoutAwareRouting::Enabled,
                    partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                    parallel: Some(NonZero::new(parallel).unwrap()),
                    ..Default::default()
                }),
            )
            .await?;
        assert_eq!(buffer, data, "parallel={parallel}: bytes differ");

        let observed = observed.lock().unwrap();
        let data_requests = || {
            observed
                .iter()
                .filter(|request| request.kind == RequestKind::Data)
        };
        let session_ok = data_requests()
            .filter(|request| request.session && request.status.is_success())
            .count();
        let routed = data_requests()
            .filter(|request| request.original_host.is_some())
            .count();
        let rejected = data_requests()
            .filter(|request| request.status == StatusCode::Unauthorized)
            .count();
        let layout_fetched = observed
            .iter()
            .any(|request| request.kind == RequestKind::Layout && request.status.is_success());
        eprintln!(
            "parallel={parallel}: {} data requests ({session_ok} session ok, {routed} routed, {rejected} rejected), layout fetched: {layout_fetched}",
            data_requests().count(),
        );

        assert!(
            session_ok > 0,
            "parallel={parallel}: no chunk was authenticated with the session"
        );
        if layout_fetched {
            assert!(
                routed > 0,
                "parallel={parallel}: a layout was fetched but no chunk was routed"
            );
        } else {
            eprintln!("NOTE: the account returned no layout, so no chunk was routed");
        }
    }

    Ok(())
}
