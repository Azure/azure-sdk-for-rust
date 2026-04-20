// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Chaos tests for Azure Storage Blob.
//!
//! These tests are designed to be run in tandem with the Azure SDK HTTP Fault Injector
//! (<https://github.com/Azure/azure-sdk-tools/tree/main/tools/http-fault-injector>).
//!
//! Run with:
//! `cargo test -p azure_storage_blob --test chaos -- --ignored --nocapture --test-threads=1`
//!
//!
//! Tests are grouped into modules by fault type. Each module's doc comment tells the
//! operator exactly which fault code to use.

use azure_core::{
    http::{
        policies::{Policy, PolicyResult},
        Body, Context, ExponentialRetryOptions, NoFormat, Request, RequestContent, RetryOptions,
        Transport, Url,
    },
    Bytes,
};
use azure_storage_blob::{
    models::{BlobClientDownloadOptions, BlockBlobClientUploadOptions},
    stream::tokio::FileStream,
    BlobContainerClient, BlobContainerClientOptions,
};
use azure_storage_blob_test::MB;
use futures::TryStreamExt;
use std::{
    num::NonZero,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

// ---------------------------------------------------------------------------
// Fault Injector Redirect Policy
// ---------------------------------------------------------------------------

const FAULT_INJECTOR_HTTPS_URI: &str = "https://127.0.0.1:7778";

/// A pipeline policy that redirects requests to the HTTP Fault Injector.
///
/// Per the fault injector protocol:
/// 1. Saves the original URL into the `x-upstream-base-uri` header.
/// 2. Rewrites the request URL to the fault injector address.
#[derive(Debug)]
struct FaultInjectorRedirectPolicy {
    request_count: Arc<AtomicUsize>,
}

impl FaultInjectorRedirectPolicy {
    fn new(request_count: Arc<AtomicUsize>) -> Self {
        Self { request_count }
    }
}

#[async_trait::async_trait]
impl Policy for FaultInjectorRedirectPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        self.request_count.fetch_add(1, Ordering::Relaxed);

        let original_url = request.url().clone();

        // Set the full original URL so the fault injector knows the upstream
        // scheme, host, and port to proxy to.
        request.insert_header("x-upstream-base-uri", original_url.to_string());

        // Always use HTTPS so the fault injector connects to the upstream
        // storage service over HTTPS (which Azure Storage requires).
        let mut new_url: Url = FAULT_INJECTOR_HTTPS_URI
            .parse()
            .expect("valid fault injector URL");
        new_url.set_path(original_url.path());
        new_url.set_query(original_url.query());
        *request.url_mut() = new_url;

        // // Debug: show exactly what we're sending to the fault injector.
        // eprintln!("[DEBUG] original: {original_url}");
        // eprintln!("[DEBUG] rewritten: {}", request.url());
        // for (name, value) in request.headers().iter() {
        //     let name_str = name.as_str();
        //     if name_str.starts_with("x-upstream") || name_str == "host" {
        //         eprintln!("[DEBUG] header: {name_str}: {}", value.as_str());
        //     }
        // }

        next[0].send(ctx, request, &next[1..]).await
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn generate_random_data(size: usize) -> Vec<u8> {
    let mut data = vec![0u8; size];
    rand::fill(&mut data[..]);
    data
}

fn account_name() -> String {
    std::env::var("AZURE_STORAGE_ACCOUNT_NAME").expect("AZURE_STORAGE_ACCOUNT_NAME must be set")
}

fn unique_container_name(prefix: &str) -> String {
    format!("{prefix}-{}", rand::random::<u32>())
}

/// Creates a [`BlobContainerClient`] wired through the fault injector.
fn create_injector_client(
    container_name: &str,
    request_count: Arc<AtomicUsize>,
    retry: Option<RetryOptions>,
    read_timeout: Option<Duration>,
) -> azure_core::Result<BlobContainerClient> {
    let redirect_policy = Arc::new(FaultInjectorRedirectPolicy::new(request_count));

    // Build a custom reqwest client that accepts the fault injector's
    // self-signed TLS certificate and disables automatic decompression
    // (required for partitioned downloads to work correctly).
    let http_client = Arc::new(
        ::reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .read_timeout(read_timeout.unwrap_or(Duration::from_secs(10)))
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .build()
            .expect("build reqwest client"),
    );

    let mut options = BlobContainerClientOptions::default();
    options.client_options.transport = Some(Transport::new(http_client));
    options
        .client_options
        .per_call_policies
        .push(redirect_policy);
    if let Some(retry) = retry {
        options.client_options.retry = retry;
    }

    let endpoint = format!("https://{}.blob.core.windows.net/", account_name());
    let credential = azure_identity::DeveloperToolsCredential::new(None)?;

    BlobContainerClient::new(&endpoint, container_name, Some(credential), Some(options))
}

/// Global test counter for sequential numbering in output.
static TEST_NUMBER: AtomicUsize = AtomicUsize::new(0);
const TOTAL_TESTS: usize = 23;

fn next_test_number() -> usize {
    TEST_NUMBER.fetch_add(1, Ordering::Relaxed) + 1
}

fn low_retry_options() -> RetryOptions {
    RetryOptions::exponential(ExponentialRetryOptions {
        max_retries: 2,
        initial_delay: azure_core::time::Duration::milliseconds(100),
        max_delay: azure_core::time::Duration::seconds(1),
        max_total_elapsed: azure_core::time::Duration::seconds(10),
    })
}

/// Uploads a blob and returns the original data.
async fn setup_blob(
    client: &BlobContainerClient,
    blob_name: &str,
    blob_size: usize,
    partition_size: usize,
) -> Vec<u8> {
    let setup_requests = 1 + blob_size.div_ceil(partition_size) + 1;
    eprintln!("=== SETUP: press 'f' for all requests (x{setup_requests}) ===");
    client.create(None).await.expect("create container");
    let blob_client = client.blob_client(blob_name);
    let data = generate_random_data(blob_size);
    blob_client
        .upload(
            RequestContent::<Bytes, NoFormat>::from(data.clone()),
            Some(BlockBlobClientUploadOptions {
                partition_size: Some(NonZero::new(partition_size as u64).unwrap()),
                ..Default::default()
            }),
        )
        .await
        .expect("setup upload");
    data
}

fn assert_data_integrity(downloaded: &[u8], original: &[u8]) {
    assert_eq!(
        downloaded.len(),
        original.len(),
        "Size mismatch: expected {} got {}",
        original.len(),
        downloaded.len(),
    );
    assert_eq!(
        downloaded, original,
        "Data corruption detected! Bytes do not match.",
    );
}

fn print_done(request_count: &AtomicUsize) {
    eprintln!(
        "=== DONE. Total HTTP requests: {} ===",
        request_count.load(Ordering::Relaxed),
    );
}

/// Writes data to a temp file and returns the path.
async fn write_temp_file(data: &[u8]) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("chaos-{}.bin", rand::random::<u32>()));
    tokio::fs::write(&path, data)
        .await
        .expect("write temp file");
    path
}

/// Collects all blob names from a list_blobs pager.
async fn collect_blob_names(client: &BlobContainerClient) -> azure_core::Result<Vec<String>> {
    let pager = client.list_blobs(None)?;
    let pages: Vec<_> = pager.into_pages().try_collect().await?;
    let mut names: Vec<String> = pages
        .into_iter()
        .filter_map(|p| p.into_model().ok())
        .flat_map(|m| m.segment.blob_items.into_iter().filter_map(|b| b.name))
        .collect();
    names.sort();
    Ok(names)
}

/// Creates N small blobs in the container and returns their sorted names.
async fn setup_list_blobs(client: &BlobContainerClient, count: usize) -> Vec<String> {
    let setup_requests = 1 + count;
    eprintln!(
        "=== SETUP: Creating {count} blobs - press 'f' for all requests (x{setup_requests}) ==="
    );
    client.create(None).await.expect("create container");
    let names: Vec<String> = (0..count).map(|i| format!("blob-{i:04}")).collect();
    for name in &names {
        client
            .blob_client(name)
            .upload(RequestContent::from(b"chaos-test".to_vec()), None)
            .await
            .expect("upload blob");
    }
    let mut sorted = names;
    sorted.sort();
    sorted
}

fn assert_blob_names(found: &[String], expected: &[String]) {
    assert_eq!(found, expected, "Blob names mismatch");
}

// ===========================================================================
// Module: connection_close - use fault code `pc` or `nc`
// ===========================================================================

/// Tests for TCP close (FIN) faults.
///
/// The fault injector either sends partial body then closes (`pc`) or sends
/// nothing then closes (`nc`). The SDK should see a transport-level error
/// and retry.
///
/// **Operator: press `pc` or `nc` on requests you want to fault, `f` for the rest.**
mod connection_close {
    use super::*;

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::partitioned_download ===");
        eprintln!("=== Chunked download (5 x 2MB). Simulates network dropping mid-transfer (e.g., Wi-Fi blip, LB idle timeout). ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-dl");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!(
            "=== PHASE 2: Download ({} chunks) - press 'pc' or 'nc' to fault ===",
            BLOB_SIZE / PARTITION_SIZE,
        );
        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                parallel: Some(NonZero::new(1).unwrap()),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => match response.body.collect().await {
                Ok(downloaded) => {
                    assert_data_integrity(&downloaded, &original);
                    eprintln!("=== [SUCCESS] Download succeeded with full data integrity. ===");
                }
                Err(e) => {
                    eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
                }
            },
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== PHASE 3: Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn ranged_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::ranged_download ===");
        eprintln!("=== Ranged download (2MB..8MB, 3 chunks). Simulates network drop during a partial range fetch. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;
        const RANGE_START: usize = 2 * MB;
        const RANGE_END: usize = 8 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-rdl");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!(
            "=== PHASE 2: Ranged download (bytes {}..{}, {} chunks) - press 'pc' or 'nc' ===",
            RANGE_START,
            RANGE_END,
            (RANGE_END - RANGE_START) / PARTITION_SIZE,
        );
        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                parallel: Some(NonZero::new(1).unwrap()),
                range: Some(RANGE_START..RANGE_END),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => match response.body.collect().await {
                Ok(downloaded) => {
                    assert_data_integrity(&downloaded, &original[RANGE_START..RANGE_END]);
                    eprintln!(
                        "=== [SUCCESS] Ranged download succeeded with full data integrity. ==="
                    );
                }
                Err(e) => {
                    eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
                }
            },
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== PHASE 3: Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_upload_bytes() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::partitioned_upload_bytes ===");
        eprintln!("=== Chunked upload from bytes (5 blocks + commit). Simulates connection lost while uploading blocks. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-ul");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let blob_client = client.blob_client("blob");

        eprintln!(
            "=== PHASE 1: Upload bytes ({} blocks + commit) - press 'nc' to fault (pc won't work: empty response body) ===",
            BLOB_SIZE / PARTITION_SIZE,
        );
        let result = blob_client
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                Some(BlockBlobClientUploadOptions {
                    partition_size: Some(NonZero::new(PARTITION_SIZE as u64).unwrap()),
                    parallel: Some(NonZero::new(1).unwrap()),
                    ..Default::default()
                }),
            )
            .await;

        match result {
            Ok(_) => {
                eprintln!(
                    "=== Upload succeeded. Verifying round-trip integrity - press 'f' (x1) ==="
                );
                match blob_client.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_upload_stream() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::partitioned_upload_stream ===");
        eprintln!(
            "=== Chunked upload from file stream (5 blocks + commit). Simulates connection lost during streamed upload. ==="
        );
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-uls");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let temp_path = write_temp_file(&original).await;
        let blob_client = client.blob_client("blob");

        eprintln!(
            "=== PHASE 1: Upload from file ({} blocks + commit) - press 'nc' to fault (pc won't work: empty response body) ===",
            BLOB_SIZE / PARTITION_SIZE,
        );

        let file = tokio::fs::File::open(&temp_path).await.expect("open file");
        let stream = FileStream::builder(file)
            .build()
            .await
            .expect("build stream");
        let body: Body = stream.into();

        let result = blob_client
            .upload(
                body.into(),
                Some(BlockBlobClientUploadOptions {
                    partition_size: Some(NonZero::new(PARTITION_SIZE as u64).unwrap()),
                    parallel: Some(NonZero::new(1).unwrap()),
                    ..Default::default()
                }),
            )
            .await;

        match result {
            Ok(_) => {
                eprintln!(
                    "=== Upload succeeded. Verifying round-trip integrity - press 'f' (x1) ==="
                );
                match blob_client.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        let _ = tokio::fs::remove_file(&temp_path).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn upload_blob_from_url() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::upload_blob_from_url ===");
        eprintln!("=== Server-side blob copy via URL. Simulates network drop during the copy request. ===");
        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-url");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' for all requests (x2) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(4 * MB);

        let source_blob = client.blob_client("source");
        source_blob
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                None,
            )
            .await
            .expect("upload source");
        let source_url: String = source_blob.url().to_string();

        eprintln!("=== TARGET: Upload from URL (1 request) - press 'nc' to fault (pc won't work: empty response body) ===");
        let dest_blob = client.blob_client("dest");
        let result = dest_blob
            .block_blob_client()
            .upload_blob_from_url(source_url, None)
            .await;

        match result {
            Ok(_) => {
                eprintln!("=== Copy succeeded. Verifying integrity - press 'f' (x1) ===");
                match dest_blob.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn list_blobs_paginated() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::list_blobs_paginated ===");
        eprintln!("=== Paginated blob listing. Simulates network drop while iterating pages. ===");
        const BLOB_COUNT: usize = 5;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-ls");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let expected = setup_list_blobs(&client, BLOB_COUNT).await;

        eprintln!("=== PHASE 2: Listing blobs - press 'pc' or 'nc' to fault ===");
        match collect_blob_names(&client).await {
            Ok(found) => {
                assert_blob_names(&found, &expected);
                eprintln!("=== [SUCCESS] All {BLOB_COUNT} blobs listed correctly. ===");
            }
            Err(e) => eprintln!("=== [SUCCESS] List failed gracefully: {e} ==="),
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    /// Faults **every** request during the download phase to exhaust retries.
    ///
    /// This test asserts that the SDK returns a well-typed error, not a panic.
    /// **Operator: press `pc` or `nc` on EVERY request during phase 2.**
    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn retry_exhaustion_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::retry_exhaustion_download ===");
        eprintln!("=== Downloads with every request faulted. Verifies SDK returns a clean error after retries are exhausted. ===");
        const BLOB_SIZE: usize = 4 * MB;
        const PARTITION_SIZE: usize = 4 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-exh");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let _original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!("=== PHASE 2: Download - press 'pc' or 'nc' on EVERY request ===");
        eprintln!("=== With max_retries=2, fault 3 times to exhaust retries. ===");

        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                parallel: Some(NonZero::new(1).unwrap()),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => {
                // Download headers may succeed but body collection should fail.
                let body_result = response.body.collect().await;
                match body_result {
                    Ok(data) => {
                        // If we somehow got data, it must still be correct.
                        assert_eq!(data.len(), BLOB_SIZE, "Unexpected success with wrong size");
                        eprintln!(
                            "=== [WARN] Unexpected success (operator may not have faulted enough). ==="
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "=== [SUCCESS] Body collection failed as expected (graceful): {e} ==="
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] Download failed as expected (graceful): {e} ===");
                // Verify it's a transport/IO error, not an assertion or logic error.
                let msg = format!("{e:?}");
                assert!(
                    !msg.contains("panic") && !msg.contains("unwrap"),
                    "Error looks like a panic, not a graceful failure: {msg}",
                );
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    /// Uploads and downloads a blob smaller than the partition size.
    ///
    /// This exercises the single-request `put_blob` + single-GET code path,
    /// which is completely different from the chunked/partitioned path.
    /// **Operator: press `pc` or `nc` to fault the single download request.**
    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn single_request_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::single_request_download ===");
        eprintln!("=== Single-GET download (blob < partition size). Simulates network drop on a non-chunked download. ===");
        const BLOB_SIZE: usize = MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-srdl");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' for all requests (x2) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let blob_client = client.blob_client("small-blob");
        blob_client
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                None,
            )
            .await
            .expect("upload small blob");

        eprintln!("=== PHASE 2: Download (1 request) - press 'pc' or 'nc' to fault ===");
        let result = blob_client.download(None).await;

        match result {
            Ok(response) => match response.body.collect().await {
                Ok(downloaded) => {
                    assert_data_integrity(&downloaded, &original);
                    eprintln!(
                        "=== [SUCCESS] Single-request download succeeded with full data integrity. ==="
                    );
                }
                Err(e) => {
                    eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
                }
            },
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    /// Uploads a blob smaller than the partition size.
    ///
    /// This exercises the single-request `put_blob` code path (no stage_block
    /// + commit_block_list).
    /// **Operator: press `pc` or `nc` to fault the single upload request.**
    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn single_request_upload() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::single_request_upload ===");
        eprintln!("=== Single-PUT upload (blob < partition size). Simulates connection lost on a non-chunked upload. ===");
        const BLOB_SIZE: usize = MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-srul");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let blob_client = client.blob_client("small-blob");

        eprintln!("=== TARGET: Upload small blob (1 request) - press 'nc' to fault (pc won't work: empty response body) ===");
        let result = blob_client
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                None,
            )
            .await;

        match result {
            Ok(_) => {
                eprintln!(
                    "=== Upload succeeded. Verifying round-trip integrity - press 'f' (x1) ==="
                );
                match blob_client.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    /// Faults **every** request during the upload phase to exhaust retries.
    ///
    /// Upload retries differ from download: each block retries independently,
    /// and `commit_block_list` can also fail separately. This asserts the SDK
    /// returns a well-typed error.
    /// **Operator: press `pc` or `nc` on EVERY request during phase 1.**
    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn retry_exhaustion_upload() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_close::retry_exhaustion_upload ===");
        eprintln!("=== Uploads with every request faulted. Verifies SDK returns a clean error after retries are exhausted. ===");
        const BLOB_SIZE: usize = 4 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-cc-exhu");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let blob_client = client.blob_client("blob");

        eprintln!("=== TARGET: Upload - press 'nc' on EVERY request (pc won't work: empty response body) ===");
        eprintln!("=== With max_retries=2, fault 3 times to exhaust retries. ===");

        let result = blob_client
            .upload(RequestContent::<Bytes, NoFormat>::from(original), None)
            .await;

        match result {
            Ok(_) => {
                eprintln!(
                    "=== [WARN] Unexpected success (operator may not have faulted enough). ==="
                );
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] Upload failed as expected (graceful): {e} ===");
                let msg = format!("{e:?}");
                assert!(
                    !msg.contains("panic") && !msg.contains("unwrap"),
                    "Error looks like a panic, not a graceful failure: {msg}",
                );
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }
}

// ===========================================================================
// Module: connection_abort - use fault code `pa` or `na`
// ===========================================================================

/// Tests for TCP abort (RST) faults.
///
/// Similar to connection_close but with RST instead of FIN. Some transports
/// handle these differently (RST may produce different error types).
///
/// **Operator: press `pa` or `na` on requests you want to fault, `f` for the rest.**
mod connection_abort {
    use super::*;

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_abort::partitioned_download ===");
        eprintln!("=== Chunked download (5 x 2MB). Simulates a server crash or load balancer forcibly dropping the connection. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-ca-dl");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!(
            "=== PHASE 2: Download ({} chunks) - press 'pa' or 'na' to fault ===",
            BLOB_SIZE / PARTITION_SIZE,
        );
        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                parallel: Some(NonZero::new(1).unwrap()),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => match response.body.collect().await {
                Ok(downloaded) => {
                    assert_data_integrity(&downloaded, &original);
                    eprintln!("=== [SUCCESS] Download succeeded with full data integrity. ===");
                }
                Err(e) => {
                    eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
                }
            },
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== PHASE 3: Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn ranged_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_abort::ranged_download ===");
        eprintln!("=== Ranged download (2MB..8MB, 3 chunks). Simulates server crash or forced disconnect during a partial range fetch. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;
        const RANGE_START: usize = 2 * MB;
        const RANGE_END: usize = 8 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-ca-rdl");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!(
            "=== PHASE 2: Ranged download (bytes {}..{}, {} chunks) - press 'pa' or 'na' ===",
            RANGE_START,
            RANGE_END,
            (RANGE_END - RANGE_START) / PARTITION_SIZE,
        );
        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                parallel: Some(NonZero::new(1).unwrap()),
                range: Some(RANGE_START..RANGE_END),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => match response.body.collect().await {
                Ok(downloaded) => {
                    assert_data_integrity(&downloaded, &original[RANGE_START..RANGE_END]);
                    eprintln!(
                        "=== [SUCCESS] Ranged download succeeded with full data integrity. ==="
                    );
                }
                Err(e) => {
                    eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
                }
            },
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== PHASE 3: Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_upload_bytes() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_abort::partitioned_upload_bytes ===");
        eprintln!("=== Chunked upload (5 blocks + commit). Simulates server crash or forced disconnect while uploading blocks. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-ca-ul");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let blob_client = client.blob_client("blob");

        eprintln!(
            "=== PHASE 1: Upload bytes ({} blocks + commit) - press 'na' to fault (pa won't work: empty response body) ===",
            BLOB_SIZE / PARTITION_SIZE,
        );
        let result = blob_client
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                Some(BlockBlobClientUploadOptions {
                    partition_size: Some(NonZero::new(PARTITION_SIZE as u64).unwrap()),
                    parallel: Some(NonZero::new(1).unwrap()),
                    ..Default::default()
                }),
            )
            .await;

        match result {
            Ok(_) => {
                eprintln!(
                    "=== Upload succeeded. Verifying round-trip integrity - press 'f' (x1) ==="
                );
                match blob_client.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK retried and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn list_blobs_paginated() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] connection_abort::list_blobs_paginated ===");
        eprintln!("=== Paginated blob listing. Simulates server crash or forced disconnect while iterating pages. ===");
        const BLOB_COUNT: usize = 5;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-ca-ls");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let expected = setup_list_blobs(&client, BLOB_COUNT).await;

        eprintln!("=== PHASE 2: Listing blobs - press 'pa' or 'na' to fault ===");
        match collect_blob_names(&client).await {
            Ok(found) => {
                assert_blob_names(&found, &expected);
                eprintln!("=== [SUCCESS] All {BLOB_COUNT} blobs listed correctly. ===");
            }
            Err(e) => eprintln!("=== [SUCCESS] List failed gracefully: {e} ==="),
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }
}

// ===========================================================================
// Module: partial_finish - use fault code `pn`
// ===========================================================================

/// Tests for silent truncation faults.
///
/// The fault injector sends 50% of the response body, then finishes the HTTP
/// response normally (`pn`). The client receives what looks like a successful
/// response but with truncated data.
///
/// **This is the most dangerous fault type** - it can cause silent data
/// corruption if the SDK doesn't validate response completeness.
///
/// **Operator: press `pn` on requests you want to fault, `f` for the rest.**
mod partial_finish {
    use super::*;

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] partial_finish::partitioned_download ===");
        eprintln!("=== Chunked download with silently truncated responses. Simulates a proxy or CDN serving incomplete data as if valid. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-pn-dl");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!(
            "=== PHASE 2: Download ({} chunks) - press 'pn' to fault ===",
            BLOB_SIZE / PARTITION_SIZE,
        );
        eprintln!("=== WARNING: 'pn' = 50% body as 'successful' response. ===");
        eprintln!("=== The SDK MUST detect truncation. Silent corruption = bug. ===");

        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                parallel: Some(NonZero::new(1).unwrap()),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => {
                match response.body.collect().await {
                    Ok(downloaded) => {
                        // If the SDK thinks this succeeded, the data MUST match.
                        assert_data_integrity(&downloaded, &original);
                        eprintln!("=== [SUCCESS] Download succeeded with full data integrity. ===");
                    }
                    Err(e) => {
                        eprintln!("=== [SUCCESS] SDK detected truncation: {e} ===");
                    }
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK detected truncation: {e} ===");
            }
        }

        eprintln!("=== PHASE 3: Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn ranged_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] partial_finish::ranged_download ===");
        eprintln!("=== Ranged download with silently truncated responses. Simulates proxy serving partial content as complete. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;
        const RANGE_START: usize = 2 * MB;
        const RANGE_END: usize = 8 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-pn-rdl");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!(
            "=== PHASE 2: Ranged download (bytes {}..{}) - press 'pn' to fault ===",
            RANGE_START, RANGE_END,
        );

        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                parallel: Some(NonZero::new(1).unwrap()),
                range: Some(RANGE_START..RANGE_END),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => match response.body.collect().await {
                Ok(downloaded) => {
                    assert_data_integrity(&downloaded, &original[RANGE_START..RANGE_END]);
                    eprintln!(
                        "=== [SUCCESS] Ranged download succeeded with full data integrity. ==="
                    );
                }
                Err(e) => {
                    eprintln!("=== [SUCCESS] SDK detected truncation: {e} ===");
                }
            },
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK detected truncation: {e} ===");
            }
        }

        eprintln!("=== PHASE 3: Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_upload_bytes() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] partial_finish::partitioned_upload_bytes ===");
        eprintln!("=== Chunked upload with truncated responses. Simulates proxy corruption on upload acknowledgments. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-pn-ul");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let blob_client = client.blob_client("blob");

        eprintln!(
            "=== PHASE 1: Upload ({} blocks + commit) - press 'pn' to fault ===",
            BLOB_SIZE / PARTITION_SIZE,
        );
        eprintln!("=== 'pn' on stage_block: server may accept partial block data. ===");

        let result = blob_client
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                Some(BlockBlobClientUploadOptions {
                    partition_size: Some(NonZero::new(PARTITION_SIZE as u64).unwrap()),
                    parallel: Some(NonZero::new(1).unwrap()),
                    ..Default::default()
                }),
            )
            .await;

        match result {
            Ok(_) => {
                eprintln!(
                    "=== Upload reports success. Verifying round-trip integrity - press 'f' (x1) ==="
                );
                match blob_client.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK detected truncation: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_upload_stream() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] partial_finish::partitioned_upload_stream ===");
        eprintln!("=== File stream upload with truncated responses. Simulates proxy corruption on upload acknowledgments. ===");
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-pn-uls");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let temp_path = write_temp_file(&original).await;
        let blob_client = client.blob_client("blob");

        eprintln!(
            "=== PHASE 1: Upload from file ({} blocks + commit) - press 'pn' to fault ===",
            BLOB_SIZE / PARTITION_SIZE,
        );

        let file = tokio::fs::File::open(&temp_path).await.expect("open file");
        let stream = FileStream::builder(file)
            .build()
            .await
            .expect("build stream");
        let body: Body = stream.into();

        let result = blob_client
            .upload(
                body.into(),
                Some(BlockBlobClientUploadOptions {
                    partition_size: Some(NonZero::new(PARTITION_SIZE as u64).unwrap()),
                    parallel: Some(NonZero::new(1).unwrap()),
                    ..Default::default()
                }),
            )
            .await;

        match result {
            Ok(_) => {
                eprintln!(
                    "=== Upload reports success. Verifying round-trip integrity - press 'f' (x1) ==="
                );
                match blob_client.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK detected truncation: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        let _ = tokio::fs::remove_file(&temp_path).await;
        print_done(&request_count);
    }

    /// Server-side copy under silent truncation.
    ///
    /// `upload_blob_from_url` is a single request. With `pn`, the service
    /// returns a truncated success response - the SDK must either detect the
    /// truncated response or the copy must still succeed server-side.
    /// **Operator: press `pn` on the copy request.**
    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn upload_blob_from_url() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] partial_finish::upload_blob_from_url ===");
        eprintln!("=== Server-side blob copy with truncated response. Simulates proxy corruption on the copy acknowledgment. ===");
        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-pn-url");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' for all requests (x2) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(4 * MB);

        let source_blob = client.blob_client("source");
        source_blob
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                None,
            )
            .await
            .expect("upload source");
        let source_url: String = source_blob.url().to_string();

        eprintln!("=== TARGET: Upload from URL (1 request) - press 'pn' to fault ===");
        eprintln!("=== 'pn' truncates the copy response. SDK must handle gracefully. ===");
        let dest_blob = client.blob_client("dest");
        let result = dest_blob
            .block_blob_client()
            .upload_blob_from_url(source_url, None)
            .await;

        match result {
            Ok(_) => {
                eprintln!("=== Copy reports success. Verifying integrity - press 'f' (x1) ===");
                match dest_blob.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK detected truncation: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn list_blobs_paginated() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] partial_finish::list_blobs_paginated ===");
        eprintln!("=== Paginated blob listing with truncated XML. Simulates proxy serving incomplete list responses as valid. ===");
        const BLOB_COUNT: usize = 5;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-pn-ls");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            None,
        )
        .expect("client");

        let expected = setup_list_blobs(&client, BLOB_COUNT).await;

        eprintln!("=== PHASE 2: Listing blobs - press 'pn' to fault ===");
        eprintln!("=== 'pn' may truncate XML response body. SDK must detect it. ===");

        match collect_blob_names(&client).await {
            Ok(found) => {
                assert_blob_names(&found, &expected);
                eprintln!("=== [SUCCESS] All {BLOB_COUNT} blobs listed correctly. ===");
            }
            Err(e) => eprintln!("=== [SUCCESS] SDK detected truncation: {e} ==="),
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }
}

// ===========================================================================
// Module: hang - use fault code `p` or `n`
// ===========================================================================

/// Tests for hang/timeout faults.
///
/// The fault injector either sends partial body then hangs (`p`) or sends
/// nothing and hangs (`n`). The reqwest transport has a 3s read timeout,
/// so each faulted request will take ~3s before the SDK sees an error.
///
/// Tests use `parallel=1` and `max_retries=2` to keep run time manageable.
///
/// **Operator: press `p` or `n` on requests you want to fault, `f` for the rest.**
mod hang {
    use super::*;

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_download() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] hang::partitioned_download ===");
        eprintln!(
            "=== Chunked download against a hung server. Simulates server freeze or network black hole (~3s per fault). ==="
        );
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-hang-dl");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            Some(Duration::from_secs(3)),
        )
        .expect("client");

        let original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!(
            "=== PHASE 2: Download ({} chunks) - press 'p' or 'n' to fault ===",
            BLOB_SIZE / PARTITION_SIZE,
        );
        eprintln!("=== NOTE: Each faulted request hangs ~3s (transport read timeout). ===");

        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                parallel: Some(NonZero::new(1).unwrap()),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => match response.body.collect().await {
                Ok(downloaded) => {
                    assert_data_integrity(&downloaded, &original);
                    eprintln!("=== [SUCCESS] Download succeeded with full data integrity. ===");
                }
                Err(e) => {
                    eprintln!("=== [SUCCESS] SDK timed out and failed gracefully: {e} ===");
                }
            },
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK timed out and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== PHASE 3: Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_upload_bytes() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] hang::partitioned_upload_bytes ===");
        eprintln!(
            "=== Chunked upload against a hung server. Simulates server freeze or network black hole (~3s per fault). ==="
        );
        const BLOB_SIZE: usize = 10 * MB;
        const PARTITION_SIZE: usize = 2 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-hang-ul");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            Some(Duration::from_secs(3)),
        )
        .expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let blob_client = client.blob_client("blob");

        eprintln!(
            "=== PHASE 1: Upload ({} blocks + commit) - press 'p' or 'n' to fault ===",
            BLOB_SIZE / PARTITION_SIZE,
        );
        eprintln!("=== NOTE: Each faulted request hangs ~3s (transport read timeout). ===");

        let result = blob_client
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                Some(BlockBlobClientUploadOptions {
                    partition_size: Some(NonZero::new(PARTITION_SIZE as u64).unwrap()),
                    parallel: Some(NonZero::new(1).unwrap()),
                    ..Default::default()
                }),
            )
            .await;

        match result {
            Ok(_) => {
                eprintln!(
                    "=== Upload succeeded. Verifying round-trip integrity - press 'f' (x1) ==="
                );
                match blob_client.download(None).await {
                    Ok(response) => match response.body.collect().await {
                        Ok(downloaded) => {
                            assert_data_integrity(&downloaded, &original);
                            eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
                        }
                        Err(e) => eprintln!("=== [FAIL] Verification body read failed: {e} ==="),
                    },
                    Err(e) => eprintln!("=== [FAIL] Verification download failed: {e} ==="),
                }
            }
            Err(e) => {
                eprintln!("=== [SUCCESS] SDK timed out and failed gracefully: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn list_blobs_paginated() {
        let n = next_test_number();
        eprintln!("\n=== [{n}/{TOTAL_TESTS}] hang::list_blobs_paginated ===");
        eprintln!("=== Paginated blob listing against a hung server. Simulates server freeze or network black hole (~3s per fault). ===");
        const BLOB_COUNT: usize = 5;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-hang-ls");
        let client = create_injector_client(
            &container,
            request_count.clone(),
            Some(low_retry_options()),
            Some(Duration::from_secs(3)),
        )
        .expect("client");

        let expected = setup_list_blobs(&client, BLOB_COUNT).await;

        eprintln!("=== PHASE 2: Listing blobs - press 'p' or 'n' to fault ===");
        eprintln!("=== NOTE: Each faulted request hangs ~3s (transport read timeout). ===");

        match collect_blob_names(&client).await {
            Ok(found) => {
                assert_blob_names(&found, &expected);
                eprintln!("=== [SUCCESS] All {BLOB_COUNT} blobs listed correctly. ===");
            }
            Err(e) => eprintln!("=== [SUCCESS] List timed out gracefully: {e} ==="),
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }
}

// ===========================================================================
// Module: default_settings - use any fault code
// ===========================================================================
// Commented out: these use default retries (8) and parallelism (4), making
// interactive runs very slow.

/*
/// Tests with default SDK retry/parallelism settings.
///
/// These represent the real customer experience: default parallelism (4),
/// default retries (8), default timeouts. The operator can use any fault code.
///
/// Because retries are high, single recoverable faults (`pc`, `pa`, `nc`, `na`)
/// will likely be retried away. Use `pn` or fault many requests to test limits.
///
/// **Operator: press any fault code on requests you want to fault, `f` for the rest.**
mod default_settings {
    use super::*;

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_download_40mb() {
        const BLOB_SIZE: usize = 40 * MB;
        const PARTITION_SIZE: usize = 4 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-def-dl");
        let client =
            create_injector_client(&container, request_count.clone(), None, None).expect("client");

        let original = setup_blob(&client, "blob", BLOB_SIZE, PARTITION_SIZE).await;

        eprintln!(
            "=== PHASE 2: Download ({} chunks, parallel=4, retries=8) - fault at will ===",
            BLOB_SIZE / PARTITION_SIZE,
        );

        let result = client
            .blob_client("blob")
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(PARTITION_SIZE).unwrap()),
                ..Default::default()
            }))
            .await;

        match result {
            Ok(response) => {
                let downloaded = response.body.collect().await.expect("collect body");
                assert_data_integrity(&downloaded, &original);
                eprintln!("=== [SUCCESS] Download succeeded with full data integrity. ===");
            }
            Err(e) => {
                eprintln!("=== Download failed: {e} ===");
            }
        }

        eprintln!("=== PHASE 3: Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }

    #[tokio::test]
    #[ignore = "requires http-fault-injector + AZURE_STORAGE_ACCOUNT_NAME"]
    async fn partitioned_upload_bytes_40mb() {
        const BLOB_SIZE: usize = 40 * MB;
        const PARTITION_SIZE: usize = 4 * MB;

        let request_count = Arc::new(AtomicUsize::new(0));
        let container = unique_container_name("chaos-def-ul");
        let client =
            create_injector_client(&container, request_count.clone(), None, None).expect("client");

        eprintln!("=== SETUP: press 'f' (x1) ===");
        client.create(None).await.expect("create container");
        let original = generate_random_data(BLOB_SIZE);
        let blob_client = client.blob_client("blob");

        eprintln!(
            "=== PHASE 1: Upload ({} blocks + commit, parallel=4, retries=8) - fault at will ===",
            BLOB_SIZE / PARTITION_SIZE,
        );

        let result = blob_client
            .upload(
                RequestContent::<Bytes, NoFormat>::from(original.clone()),
                Some(BlockBlobClientUploadOptions {
                    partition_size: Some(NonZero::new(PARTITION_SIZE as u64).unwrap()),
                    ..Default::default()
                }),
            )
            .await;

        match result {
            Ok(_) => {
                eprintln!("=== Upload succeeded. Verifying round-trip integrity. ===");
                let response = blob_client.download(None).await.expect("download");
                let downloaded = response.body.collect().await.expect("collect body");
                assert_data_integrity(&downloaded, &original);
                eprintln!("=== [SUCCESS] Round-trip integrity verified. ===");
            }
            Err(e) => {
                eprintln!("=== Upload failed: {e} ===");
            }
        }

        eprintln!("=== Cleanup - press 'f' (x1) ===");
        let _ = client.delete(None).await;
        print_done(&request_count);
    }
}
*/
