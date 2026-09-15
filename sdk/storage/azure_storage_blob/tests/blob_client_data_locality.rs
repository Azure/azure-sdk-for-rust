// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data-locality tests: the SDK's automatic layout-aware routing of managed downloads,
//! and the caller-controlled `layout_endpoint` override.

use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    http::{
        headers::Headers,
        policies::{Policy, PolicyResult},
        AsyncRawResponse, ClientOptions, Context, Request, RequestContent, StatusCode, Transport,
        Url,
    },
    Bytes,
};
use azure_core_test::{http::MockHttpClient, recorded};
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{
    models::{BlobClientDownloadOptions, BlobLayout, LayoutAwareRouting},
    BlobClient, BlobClientOptions, BlobContainerClient, BlobContainerClientOptions,
};
use futures::{FutureExt as _, TryStreamExt};
use std::{
    collections::HashMap,
    error::Error,
    num::NonZero,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

const ACCOUNT_HOST: &str = "acct.blob.core.windows.net";
const ETAG: &str = "\"layout-test-etag\"";

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
}

/// Whether data responses carry `x-ms-download-hint`, which is what gates the SDK's
/// automatic layout lookup.
#[derive(Clone, Copy, PartialEq)]
enum LayoutHint {
    Advertised,
    Absent,
}

fn parse_bytes_range(range: &str) -> (usize, usize) {
    let spec = range.strip_prefix("bytes=").expect("bytes= prefix");
    let (start, end) = spec.split_once('-').expect("start-end");
    (
        start.parse().expect("range start"),
        end.parse().expect("range end"),
    )
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

fn live_layout_blob_url() -> Option<Url> {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT_NAME").ok()?;
    let account = account.trim().trim_matches('"').trim();
    if account.is_empty() {
        return None;
    }
    Url::parse(&format!(
        "https://{account}.blob.core.windows.net/layout-routing-test/routing-test-blob"
    ))
    .ok()
}

#[derive(Debug)]
struct RoutingObserver {
    seen: Arc<Mutex<Vec<RoutedRequest>>>,
}

#[derive(Debug)]
struct RoutedRequest {
    is_layout: bool,
    range: Option<String>,
    sent_to: Option<String>,
    original_host: Option<String>,
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
        let is_layout = request
            .url()
            .query()
            .is_some_and(|query| query.contains("comp=layout"));
        let range = request
            .headers()
            .get_optional_str(&"range".into())
            .map(str::to_owned);
        let sent_to = request.url().host_str().map(str::to_owned);
        let original_host = request
            .headers()
            .get_optional_str(&"host".into())
            .map(str::to_owned);
        let response = next[0].send(ctx, request, &next[1..]).await?;
        let download_hint = response
            .headers()
            .get_optional_str(&"x-ms-download-hint".into())
            .map(str::to_owned);
        self.seen.lock().unwrap().push(RoutedRequest {
            is_layout,
            range,
            sent_to,
            original_host,
            download_hint,
            status: response.status(),
        });
        Ok(response)
    }
}

#[recorded::test(live)]
async fn test_download_layout_aware_routing() -> Result<(), Box<dyn Error>> {
    let Some(url) = live_layout_blob_url() else {
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

    let mut container_url = url.clone();
    container_url
        .path_segments_mut()
        .expect("blob URL must be a base")
        .pop();
    let container_client = BlobContainerClient::new(container_url, Some(credential.clone()), None)?;
    if let Err(error) = container_client.create(None).await {
        if error.http_status() != Some(StatusCode::Conflict) {
            return Err(error.into());
        }
    }

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
    let layout_calls = observed.iter().filter(|request| request.is_layout).count();
    let data_chunks = observed.iter().filter(|request| !request.is_layout).count();
    let routed: Vec<&RoutedRequest> = observed
        .iter()
        .filter(|request| !request.is_layout && request.original_host.is_some())
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
        let kind = if request.is_layout { "layout" } else { "data" };
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
