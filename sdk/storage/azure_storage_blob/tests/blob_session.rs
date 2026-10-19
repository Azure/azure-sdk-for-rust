// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live-only integration tests for session token authentication.
//!
//! These tests require a storage account that supports the session feature.
//!
//! Create Session and the requests that use its session should share one
//! transport. Tests that inject a [`ContainerSessionProvider`] pass
//! `shared_transport()` to both the provider and the clients; a self-created
//! provider already inherits the client's transport.

mod common;

use async_trait::async_trait;
use azure_core::{
    http::{
        headers::{AUTHORIZATION, ERROR_CODE},
        new_http_client,
        policies::{Policy, PolicyResult},
        Context, HttpClientOptions, Method, Request, RequestContent, StatusCode, Transport, Url,
    },
    time::{Duration, OffsetDateTime},
};
use azure_core_test::{recorded, BodyRegexSanitizer, Recording, TestContext};
use azure_storage_blob::{
    models::{
        BlobClientAcquireLeaseResultHeaders, BlobClientCreateSnapshotResultHeaders,
        BlobClientDownloadOptions, BlobClientGetPropertiesResultHeaders,
        BlockBlobClientUploadOptions, BlockListType,
    },
    BlobClient, BlobClientOptions, BlobServiceClient, BlobServiceClientOptions,
    ContainerSessionProvider, SessionMode, SessionOptions, SessionProvider,
};
use common::{ClientOptionsExt, StorageAccount};
use serial_test::serial;
use std::{
    error::Error,
    num::NonZero,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

/// Counts how eligible and ineligible requests were authenticated. A single
/// instance is shared across the client and its session provider, so it also
/// observes Create Session requests.
#[derive(Debug, Default)]
struct SessionAuthCounts {
    create_session: AtomicUsize,
    session_get: AtomicUsize,
    session_unauthorized: AtomicUsize,
    session_error_codes: Mutex<Vec<String>>,
    bearer_get: AtomicUsize,
    non_get_session: AtomicUsize,
}

#[derive(Debug, PartialEq, Eq)]
struct SessionAuthCountsSnapshot {
    create_session: usize,
    session_get: usize,
    session_unauthorized: usize,
    session_error_codes: Vec<String>,
    bearer_get: usize,
    non_get_session: usize,
}

impl SessionAuthCounts {
    fn snapshot(&self) -> SessionAuthCountsSnapshot {
        SessionAuthCountsSnapshot {
            create_session: self.create_session.load(Ordering::SeqCst),
            session_get: self.session_get.load(Ordering::SeqCst),
            session_unauthorized: self.session_unauthorized.load(Ordering::SeqCst),
            session_error_codes: self.session_error_codes.lock().unwrap().clone(),
            bearer_get: self.bearer_get.load(Ordering::SeqCst),
            non_get_session: self.non_get_session.load(Ordering::SeqCst),
        }
    }
}

#[derive(Debug)]
struct SessionAuthCountingPolicy {
    counts: Arc<SessionAuthCounts>,
}

#[async_trait]
impl Policy for SessionAuthCountingPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let method = request.method();
        let is_create_session = method == Method::Post
            && request
                .url()
                .query_pairs()
                .any(|(k, v)| k == "comp" && v == "session");
        let auth = request
            .headers()
            .get_optional_str(&AUTHORIZATION)
            .map(str::to_string);

        if is_create_session {
            self.counts.create_session.fetch_add(1, Ordering::SeqCst);
        } else if method == Method::Get {
            match auth.as_deref() {
                Some(a) if a.starts_with("Session ") => {
                    self.counts.session_get.fetch_add(1, Ordering::SeqCst);
                }
                Some(a) if a.starts_with("Bearer ") => {
                    self.counts.bearer_get.fetch_add(1, Ordering::SeqCst);
                }
                _ => {}
            }
        } else if matches!(auth.as_deref(), Some(a) if a.starts_with("Session ")) {
            self.counts.non_get_session.fetch_add(1, Ordering::SeqCst);
        }

        let response = next[0].send(ctx, request, &next[1..]).await?;
        if matches!(auth.as_deref(), Some(a) if a.starts_with("Session "))
            && response.status() == StatusCode::Unauthorized
        {
            self.counts
                .session_unauthorized
                .fetch_add(1, Ordering::SeqCst);
            if let Some(error_code) = response.headers().get_optional_str(&ERROR_CODE) {
                self.counts
                    .session_error_codes
                    .lock()
                    .unwrap()
                    .push(error_code.to_string());
            }
        }
        Ok(response)
    }
}

/// Redacts the session token and key from Create Session response bodies so
/// live credentials are never written to recordings. The replacement is valid
/// base64 so playback can still sign requests using the recorded body.
async fn redact_session_credentials(recording: &Recording) -> azure_core::Result<()> {
    // Base64 of "REDACTED": a valid key so playback signing still succeeds.
    // cspell:ignore VEQUNURUQ
    const REDACTED: &str = "UkVEQUNURUQ=";
    for element in ["SessionToken", "SessionKey"] {
        recording
            .add_sanitizer(BodyRegexSanitizer {
                value: Some(REDACTED.into()),
                regex: Some(format!("<{element}>([^<]+)</{element}>")),
                group_for_replace: Some("1".into()),
                ..Default::default()
            })
            .await?;
    }
    Ok(())
}

/// Builds a session-enabled `BlobServiceClient` with `counting` attached as a
/// per-try policy so it observes the final authorization scheme of each request.
async fn session_service_client(
    recording: &Recording,
    mode: SessionMode,
    counting: Arc<SessionAuthCountingPolicy>,
) -> azure_core::Result<BlobServiceClient> {
    redact_session_credentials(recording).await?;
    let mut options = BlobServiceClientOptions::default().with_per_try_policy(counting);
    let endpoint = common::recorded_test_setup(
        recording,
        StorageAccount::Standard,
        &mut options.client_options,
    );
    let account_name = recording
        .var("AZURE_STORAGE_ACCOUNT_NAME", None)
        .as_str()
        .to_string();
    options.session_options = Some(SessionOptions {
        mode,
        account_name: Some(account_name),
        ..Default::default()
    });
    BlobServiceClient::new(
        Url::parse(&endpoint)?,
        Some(recording.credential()),
        Some(options),
    )
}

/// A transport shared by the session provider and the clients that use its
/// sessions, so Create Session and the subsequent downloads reuse one pool.
/// Decompression stays off to match the storage client defaults.
fn shared_transport() -> Transport {
    Transport::new(new_http_client(Some(HttpClientOptions {
        automatic_decompression: false,
    })))
}

/// Builds a session-enabled `BlobServiceClient` that reuses the shared
/// `provider`, with `counting` attached so its downloads are observed.
fn shared_provider_client(
    recording: &Recording,
    account_name: &str,
    transport: Transport,
    provider: Arc<dyn SessionProvider>,
    counting: Arc<SessionAuthCountingPolicy>,
) -> azure_core::Result<BlobServiceClient> {
    let mut options = BlobServiceClientOptions::default().with_per_try_policy(counting);
    let endpoint = common::recorded_test_setup(
        recording,
        StorageAccount::Standard,
        &mut options.client_options,
    );
    options.client_options.transport = Some(transport);
    options.session_options = Some(SessionOptions {
        mode: SessionMode::Enabled,
        account_name: Some(account_name.to_string()),
        session_provider: Some(provider),
    });
    BlobServiceClient::new(
        Url::parse(&endpoint)?,
        Some(recording.credential()),
        Some(options),
    )
}

#[recorded::test(live)]
#[serial(blob_session)]
async fn session_download_uses_session_token(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let policy = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });

    let service = session_service_client(recording, SessionMode::Enabled, policy).await?;
    let container = service.blob_container_client(&common::get_container_name(recording));
    container.create(None).await?;

    let blob = container.blob_client(&common::get_blob_name(recording));
    let data = b"session round trip payload".to_vec();
    common::create_test_blob(&blob, Some(RequestContent::from(data.clone())), None).await?;

    let mut buffer = vec![0u8; data.len()];
    blob.download_into(&mut buffer, None).await?;
    assert_eq!(buffer, data);

    assert_eq!(
        counts.snapshot(),
        SessionAuthCountsSnapshot {
            create_session: 1,
            session_get: 1,
            session_unauthorized: 0,
            session_error_codes: Vec::new(),
            bearer_get: 0,
            non_get_session: 0,
        },
        "one download should mint and successfully use one session without bearer fallback"
    );

    container.delete(None).await?;
    Ok(())
}

#[recorded::test(live)]
#[serial(blob_session)]
async fn comp_operation_falls_back_to_bearer(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let policy = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });

    let service = session_service_client(recording, SessionMode::Enabled, policy).await?;
    let container = service.blob_container_client(&common::get_container_name(recording));
    container.create(None).await?;

    let blob = container.blob_client(&common::get_blob_name(recording));
    common::create_test_blob(
        &blob,
        Some(RequestContent::from(b"blocklist".to_vec())),
        None,
    )
    .await?;

    // GetBlockList carries `comp=blocklist`, so it is ineligible for session auth.
    blob.block_blob_client()
        .get_block_list(BlockListType::All, None)
        .await?;

    assert_eq!(
        counts.snapshot(),
        SessionAuthCountsSnapshot {
            create_session: 0,
            session_get: 0,
            session_unauthorized: 0,
            session_error_codes: Vec::new(),
            bearer_get: 1,
            non_get_session: 0,
        },
        "one comp GET should use bearer directly without acquiring or trying a session"
    );

    container.delete(None).await?;
    Ok(())
}

#[recorded::test(live)]
#[serial(blob_session)]
async fn sessions_are_cached_per_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let policy = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });

    let service = session_service_client(recording, SessionMode::Enabled, policy).await?;
    let first_container = service.blob_container_client(&common::get_container_name(recording));
    let second_container = service.blob_container_client(&common::get_container_name(recording));
    first_container.create(None).await?;
    second_container.create(None).await?;

    let first_blob = first_container.blob_client(&common::get_blob_name(recording));
    let second_blob = second_container.blob_client(&common::get_blob_name(recording));
    let first_data = b"first container session payload".to_vec();
    let second_data = b"second container session payload".to_vec();
    common::create_test_blob(
        &first_blob,
        Some(RequestContent::from(first_data.clone())),
        None,
    )
    .await?;
    common::create_test_blob(
        &second_blob,
        Some(RequestContent::from(second_data.clone())),
        None,
    )
    .await?;

    for (blob, expected) in [
        (&first_blob, first_data.as_slice()),
        (&second_blob, second_data.as_slice()),
        (&first_blob, first_data.as_slice()),
        (&second_blob, second_data.as_slice()),
    ] {
        let mut buffer = vec![0u8; expected.len()];
        blob.download_into(&mut buffer, None).await?;
        assert_eq!(buffer, expected);
    }

    assert_eq!(
        counts.snapshot(),
        SessionAuthCountsSnapshot {
            create_session: 2,
            session_get: 4,
            session_unauthorized: 0,
            session_error_codes: Vec::new(),
            bearer_get: 0,
            non_get_session: 0,
        },
        "four downloads across two containers should mint one session per container and never fall back to bearer"
    );

    first_container.delete(None).await?;
    second_container.delete(None).await?;
    Ok(())
}

#[recorded::test(live)]
#[serial(blob_session)]
async fn shared_provider_reuses_session_across_clients(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let counting = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });
    redact_session_credentials(recording).await?;
    let account_name = recording
        .var("AZURE_STORAGE_ACCOUNT_NAME", None)
        .as_str()
        .to_string();

    // One provider owns the single session cache; its own service client is
    // observed by the shared counter so its Create Session call is counted.
    let transport = shared_transport();
    let mut provider_options =
        BlobServiceClientOptions::default().with_per_try_policy(counting.clone());
    let endpoint = common::recorded_test_setup(
        recording,
        StorageAccount::Standard,
        &mut provider_options.client_options,
    );
    provider_options.client_options.transport = Some(transport.clone());
    let provider: Arc<dyn SessionProvider> = Arc::new(ContainerSessionProvider::new(
        &Url::parse(&endpoint)?,
        recording.credential(),
        Some(provider_options),
    )?);

    // Two independent clients share the one provider (and its cache).
    let client1 = shared_provider_client(
        recording,
        &account_name,
        transport.clone(),
        provider.clone(),
        counting.clone(),
    )?;
    let client2 = shared_provider_client(
        recording,
        &account_name,
        transport.clone(),
        provider.clone(),
        counting.clone(),
    )?;

    let container_name = common::get_container_name(recording);
    let blob_name = common::get_blob_name(recording);
    let container = client1.blob_container_client(&container_name);
    container.create(None).await?;
    let blob = container.blob_client(&blob_name);
    let data = b"shared session payload".to_vec();
    common::create_test_blob(&blob, Some(RequestContent::from(data.clone())), None).await?;

    // Download the same blob through each client; the second reuses the session.
    let mut buffer = vec![0u8; data.len()];
    client1
        .blob_container_client(&container_name)
        .blob_client(&blob_name)
        .download_into(&mut buffer, None)
        .await?;
    assert_eq!(buffer, data);
    let mut buffer = vec![0u8; data.len()];
    client2
        .blob_container_client(&container_name)
        .blob_client(&blob_name)
        .download_into(&mut buffer, None)
        .await?;
    assert_eq!(buffer, data);

    assert_eq!(
        counts.snapshot(),
        SessionAuthCountsSnapshot {
            create_session: 1,
            session_get: 2,
            session_unauthorized: 0,
            session_error_codes: Vec::new(),
            bearer_get: 0,
            non_get_session: 0,
        },
        "two clients sharing a provider should reuse one session without bearer fallback"
    );

    container.delete(None).await?;
    Ok(())
}

/// Session auth should be applied to every partition of a fan-out download, not
/// just the initial range.
///
/// The outcome of those requests is deliberately not asserted. Ranged GETs run
/// concurrently and each in-flight request needs its own connection, so the
/// download spans more connections than Create Session used. Some network
/// environments do not present those connections to the service identically, in
/// which case the affected partitions are rejected and fall back to bearer; the
/// download still returns the correct bytes.
///
/// How many fall back is therefore a property of the environment, not the SDK,
/// and varies per run. [`session_download_partitioned_serially_reuses_one_session`]
/// covers the same workload without concurrency and does assert the outcome.
#[recorded::test(live)]
#[serial(blob_session)]
async fn session_download_partitioned_signs_every_chunk(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const PARTITION_SIZE: usize = 1024 * 1024;
    const PARTITIONS: usize = 8;

    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let policy = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });

    let service = session_service_client(recording, SessionMode::Enabled, policy).await?;
    let container = service.blob_container_client(&common::get_container_name(recording));
    container.create(None).await?;

    let blob = container.blob_client(&common::get_blob_name(recording));
    let data = vec![0x5au8; PARTITION_SIZE * PARTITIONS];
    common::create_test_blob(&blob, Some(RequestContent::from(data.clone())), None).await?;

    let mut buffer = vec![0u8; data.len()];
    blob.download_into(
        &mut buffer,
        Some(BlobClientDownloadOptions {
            parallel: NonZero::new(PARTITIONS),
            partition_size: NonZero::new(PARTITION_SIZE),
            ..Default::default()
        }),
    )
    .await?;
    assert_eq!(buffer, data);

    let snapshot = counts.snapshot();
    assert_eq!(
        snapshot.session_get, PARTITIONS,
        "every partition should be attempted with session authorization"
    );
    assert_eq!(
        snapshot.non_get_session, 0,
        "no non-GET request should use session authorization"
    );

    container.delete(None).await?;
    Ok(())
}

/// The controlled counterpart to [`session_download_partitioned_signs_every_chunk`]:
/// identical workload, but `parallel: 1` runs the ranged GETs one at a time
/// so they reuse the pooled connection the session was minted on.
/// Concurrency is therefore the only difference between the two, and this one can strongly assert.
#[recorded::test(live)]
#[serial(blob_session)]
async fn session_download_partitioned_serially_reuses_one_session(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const PARTITION_SIZE: usize = 1024 * 1024;
    const PARTITIONS: usize = 8;

    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let policy = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });

    let service = session_service_client(recording, SessionMode::Enabled, policy).await?;
    let container = service.blob_container_client(&common::get_container_name(recording));
    container.create(None).await?;

    let blob = container.blob_client(&common::get_blob_name(recording));
    let data = vec![0x5au8; PARTITION_SIZE * PARTITIONS];
    common::create_test_blob(&blob, Some(RequestContent::from(data.clone())), None).await?;

    let mut buffer = vec![0u8; data.len()];
    blob.download_into(
        &mut buffer,
        Some(BlobClientDownloadOptions {
            parallel: NonZero::new(1),
            partition_size: NonZero::new(PARTITION_SIZE),
            ..Default::default()
        }),
    )
    .await?;
    assert_eq!(buffer, data);

    assert_eq!(
        counts.snapshot(),
        SessionAuthCountsSnapshot {
            create_session: 1,
            session_get: PARTITIONS,
            session_unauthorized: 0,
            session_error_codes: Vec::new(),
            bearer_get: 0,
            non_get_session: 0,
        },
        "sequential partitions should all reuse the one session without bearer fallback"
    );

    container.delete(None).await?;
    Ok(())
}

/// Blob names the session signature's percent-encoded canonical path must agree with the
/// service on: reserved and unreserved punctuation, characters the `url` crate leaves
/// unencoded, non-ASCII, and path-like names.
const ENCODED_BLOB_NAMES: &[&str] = &[
    "a b",
    "a+b",
    "a%b",
    "a'b(c)!*~",
    "a#b?c",
    "a&b=c;d",
    "a[b]^c|d",
    "a{b}\"c<d>`e",
    "a@b$c,d:e",
    "a\\b",
    "üñï-文字",
    "dir/sub/b",
    ".hidden",
    "a.",
];

/// Each blob name is downloaded with a single request on the connection the session was
/// minted on, so a mismatched signature is the only reason a download would not use it.
#[recorded::test(live)]
#[serial(blob_session)]
async fn session_download_signs_encoded_blob_names(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let policy = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });

    let service = session_service_client(recording, SessionMode::Enabled, policy).await?;
    let container = service.blob_container_client(&common::get_container_name(recording));
    container.create(None).await?;

    let mut names: Vec<String> = ENCODED_BLOB_NAMES
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
    // The longest name the service accepts.
    names.push(format!("long-{}", "x".repeat(1024 - 5)));

    let data = b"encoded blob name".to_vec();
    for name in &names {
        let blob = container.blob_client(name);
        common::create_test_blob(&blob, Some(RequestContent::from(data.clone())), None).await?;
        let mut buffer = vec![0u8; data.len()];
        blob.download_into(&mut buffer, None).await?;
        assert_eq!(buffer, data, "{name:?}");
    }

    assert_eq!(
        counts.snapshot(),
        SessionAuthCountsSnapshot {
            create_session: 1,
            session_get: names.len(),
            session_unauthorized: 0,
            session_error_codes: Vec::new(),
            bearer_get: 0,
            non_get_session: 0,
        },
        "every blob name should be signed so the service accepts the session"
    );

    container.delete(None).await?;
    Ok(())
}

/// A blob URL that keeps `/` in the blob name raw, as `BlobClient::new` callers commonly
/// pass, should sign the same way as the `%2F` form `blob_client` produces.
#[recorded::test(live)]
#[serial(blob_session)]
async fn session_download_signs_raw_slash_blob_url(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let policy = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });

    let service = session_service_client(recording, SessionMode::Enabled, policy.clone()).await?;
    let container = service.blob_container_client(&common::get_container_name(recording));
    container.create(None).await?;

    let data = b"raw slash blob name".to_vec();
    common::create_test_blob(
        &container.blob_client("dir/sub/b"),
        Some(RequestContent::from(data.clone())),
        None,
    )
    .await?;

    let mut url = container.url().clone();
    url.path_segments_mut()
        .expect("container URL must be a base")
        .extend(["dir", "sub", "b"]);
    let mut options = BlobClientOptions::default().with_per_try_policy(policy);
    common::recorded_test_setup(
        recording,
        StorageAccount::Standard,
        &mut options.client_options,
    );
    options.session_options = Some(SessionOptions {
        mode: SessionMode::Enabled,
        account_name: Some(
            recording
                .var("AZURE_STORAGE_ACCOUNT_NAME", None)
                .as_str()
                .to_string(),
        ),
        ..Default::default()
    });
    let blob = BlobClient::new(url, Some(recording.credential()), Some(options))?;

    let mut buffer = vec![0u8; data.len()];
    blob.download_into(&mut buffer, None).await?;
    assert_eq!(buffer, data);

    assert_eq!(
        counts.snapshot(),
        SessionAuthCountsSnapshot {
            create_session: 1,
            session_get: 1,
            session_unauthorized: 0,
            session_error_codes: Vec::new(),
            bearer_get: 0,
            non_get_session: 0,
        },
        "a raw-slash blob URL should be signed so the service accepts the session"
    );

    container.delete(None).await?;
    Ok(())
}

/// Download options that add signed query values or headers (snapshot, customer-provided key,
/// range, lease, conditions, `timeout`, per-range CRC64) should all authenticate with the
/// session. Each download is a single request on the session's connection.
#[recorded::test(live)]
#[serial(blob_session)]
async fn session_download_signs_request_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let counts = Arc::new(SessionAuthCounts::default());
    let policy = Arc::new(SessionAuthCountingPolicy {
        counts: counts.clone(),
    });

    let service = session_service_client(recording, SessionMode::Enabled, policy).await?;
    let container = service.blob_container_client(&common::get_container_name(recording));
    container.create(None).await?;

    let data: Vec<u8> = (0..1024).map(|index| (index % 251) as u8).collect();
    let blob = container.blob_client(&common::get_blob_name(recording));
    common::create_test_blob(&blob, Some(RequestContent::from(data.clone())), None).await?;
    let mut downloads = 0;

    // Snapshot: a `snapshot` query value.
    let snapshot = blob
        .create_snapshot(None)
        .await?
        .snapshot()?
        .expect("Create Snapshot should return a snapshot id");
    assert_eq!(
        blob.with_snapshot(&snapshot)?
            .download(None)
            .await?
            .body
            .collect()
            .await?,
        data
    );
    downloads += 1;

    // Range, `timeout`, and per-range CRC64.
    let response = blob
        .download(Some(BlobClientDownloadOptions {
            range: Some((100u64..900).into()),
            timeout: Some(30),
            range_get_content_crc64: Some(true),
            ..Default::default()
        }))
        .await?;
    assert_eq!(response.body.collect().await?, &data[100..900]);
    downloads += 1;

    // Conditions signed as standard headers.
    let etag = blob
        .get_properties(None)
        .await?
        .etag()?
        .expect("Get Properties should return an ETag");
    let now = OffsetDateTime::now_utc();
    let response = blob
        .download(Some(BlobClientDownloadOptions {
            if_match: Some(etag),
            if_modified_since: Some(now - Duration::days(3650)),
            if_unmodified_since: Some(now + Duration::days(1)),
            ..Default::default()
        }))
        .await?;
    assert_eq!(response.body.collect().await?, data);
    downloads += 1;

    // Lease: an `x-ms-lease-id` header.
    let lease_id = blob
        .acquire_lease(15, None)
        .await?
        .lease_id()?
        .expect("Acquire Lease should return a lease id");
    let response = blob
        .download(Some(BlobClientDownloadOptions {
            lease_id: Some(lease_id.clone()),
            ..Default::default()
        }))
        .await?;
    assert_eq!(response.body.collect().await?, data);
    blob.release_lease(lease_id, None).await?;
    downloads += 1;

    // Customer-provided key: `x-ms-encryption-*` headers.
    let (algorithm, key, key_sha256) = common::get_cpk();
    let cpk_blob = container.blob_client(&format!("{}-cpk", common::get_blob_name(recording)));
    common::create_test_blob(
        &cpk_blob,
        Some(RequestContent::from(data.clone())),
        Some(BlockBlobClientUploadOptions {
            encryption_algorithm: Some(algorithm),
            encryption_key: Some(key.clone()),
            encryption_key_sha256: Some(key_sha256.clone()),
            ..Default::default()
        }),
    )
    .await?;
    let response = cpk_blob
        .download(Some(BlobClientDownloadOptions {
            encryption_algorithm: Some(algorithm),
            encryption_key: Some(key),
            encryption_key_sha256: Some(key_sha256),
            ..Default::default()
        }))
        .await?;
    assert_eq!(response.body.collect().await?, data);
    downloads += 1;

    assert_eq!(
        counts.snapshot(),
        SessionAuthCountsSnapshot {
            create_session: 1,
            session_get: downloads,
            session_unauthorized: 0,
            session_error_codes: Vec::new(),
            bearer_get: 0,
            non_get_session: 0,
        },
        "every download option should be signed so the service accepts the session"
    );

    container.delete(None).await?;
    Ok(())
}
