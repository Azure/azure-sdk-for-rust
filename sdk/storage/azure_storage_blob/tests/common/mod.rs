// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Shared helpers for `azure_storage_blob` integration tests. Each test binary that
// declares `mod common;` compiles this module and uses only a subset of its helpers,
// so unused items are expected.
#![allow(dead_code)]

use std::{
    future::Future,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::Sender,
        Arc,
    },
    time::Duration,
};

use async_trait::async_trait;
use azure_core::{
    error::ErrorKind,
    http::{
        policies::{Policy, PolicyResult},
        AsyncRawResponse, ClientOptions, Context, NoFormat, Request, RequestContent, StatusCode,
        Url,
    },
    Bytes, Result,
};
use azure_core_test::{Recording, TestMode};
use azure_storage_blob::{
    models::{
        BlockBlobClientUploadOptions, BlockBlobClientUploadResult, BlockLookupList,
        EncryptionAlgorithmType,
    },
    BlobClient, BlobClientOptions, BlobContainerClient, BlobContainerClientOptions,
    BlobServiceClient, BlobServiceClientOptions,
};

pub const KB: usize = 1024;
pub const MB: usize = KB * 1024;
pub const GB: usize = MB * 1024;

/// Returns a valid customer-provided key tuple used by blob encryption tests.
pub fn get_cpk() -> (EncryptionAlgorithmType, String, String) {
    (
        EncryptionAlgorithmType::Aes256,
        "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=".to_string(),
        "Yw3NKWbEM2aRElRIu7JbT/QSpJxzLbLIq8G4WBvXEN0=".to_string(),
    )
}

/// Returns a second valid customer-provided key tuple for mismatch testing.
pub fn get_cpk_2() -> (EncryptionAlgorithmType, String, String) {
    (
        EncryptionAlgorithmType::Aes256,
        "AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA=".to_string(),
        "riFsLvUkejeCwTXvonmj5M3GEJQnD10r5YxiBLemEsk=".to_string(),
    )
}

/// Returns the encryption scope name provisioned in test-resources.bicep.
pub fn get_valid_encryption_scope() -> String {
    "testscope".to_string()
}

/// Returns an encryption scope name that should not exist in test accounts.
pub fn get_invalid_encryption_scope() -> String {
    "invalid-encryption-scope-for-tests".to_string()
}

/// Returns a base64-encoded value that is valid but intentionally not the SHA-256 hash of
/// any test key.
///
/// Used to verify that the service rejects mismatched key hashes.
pub fn invalid_key_sha256() -> String {
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string()
}

/// Returns a [`BlockLookupList`] that stages the given block ID from the latest block list.
///
/// Used by block blob tests to finalize a staged block into a committed blob.
pub fn block_lookup(block_id: Vec<u8>) -> BlockLookupList {
    BlockLookupList {
        committed: Some(Vec::new()),
        latest: Some(vec![block_id]),
        uncommitted: Some(Vec::new()),
    }
}

/// Asserts the error status for invalid encryption configuration requests.
pub fn assert_bad_request_or_conflict(status: Option<StatusCode>) {
    assert!(matches!(
        status,
        Some(StatusCode::BadRequest | StatusCode::Conflict)
    ));
}

/// Specifies which storage account to use for testing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageAccount {
    /// The standard storage account (AZURE_STORAGE_ACCOUNT_NAME)
    Standard,
    /// The versioned storage account (VERSIONED_AZURE_STORAGE_ACCOUNT_NAME)
    Versioned,
}

/// Takes in a Recording instance and returns an instrumented options bag and endpoint.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
/// * `account_type` - The storage account type to use.
pub fn recorded_test_setup(
    recording: &Recording,
    account_type: StorageAccount,
    client_options: &mut ClientOptions,
) -> String {
    recording.instrument(client_options);

    let account_name_var = match account_type {
        StorageAccount::Standard => "AZURE_STORAGE_ACCOUNT_NAME",
        StorageAccount::Versioned => "VERSIONED_AZURE_STORAGE_ACCOUNT_NAME",
    };

    format!(
        "https://{}.blob.core.windows.net/",
        recording.var(account_name_var, None).as_str()
    )
}

/// Takes in a Recording instance and returns a randomized blob name with prefix "blob" of length 16.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub fn get_blob_name(recording: &Recording) -> String {
    recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase()
}

/// Takes in a Recording instance and returns a randomized container name with prefix "container" of length 16.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub fn get_container_name(recording: &Recording) -> String {
    recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase()
}

/// Returns an instance of a BlobServiceClient.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
/// * `account_type` - The storage account type to use.
pub fn get_blob_service_client(
    recording: &Recording,
    account_type: StorageAccount,
    service_client_options: Option<BlobServiceClientOptions>,
) -> Result<BlobServiceClient> {
    let mut service_client_options = service_client_options.unwrap_or_default();
    let endpoint = recorded_test_setup(
        recording,
        account_type,
        &mut service_client_options.client_options,
    );
    let service_url = Url::parse(&endpoint)?;
    BlobServiceClient::new(
        service_url,
        Some(recording.credential()),
        Some(service_client_options),
    )
}

/// Returns an instance of a BlobContainerClient.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
/// * `create` - An optional flag to determine whether the container should also be created.
/// * `account_type` - The storage account type to use.
pub async fn get_container_client(
    recording: &Recording,
    create: bool,
    account_type: StorageAccount,
    container_client_options: Option<BlobContainerClientOptions>,
) -> Result<BlobContainerClient> {
    let container_name = get_container_name(recording);
    let mut container_client_options = container_client_options.unwrap_or_default();
    let endpoint = recorded_test_setup(
        recording,
        account_type,
        &mut container_client_options.client_options,
    );
    let mut container_url = Url::parse(&endpoint)?;
    container_url
        .path_segments_mut()
        .map_err(|_| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "Invalid endpoint URL: cannot append container name.",
            )
        })?
        .push(&container_name);
    let container_client = BlobContainerClient::new(
        container_url,
        Some(recording.credential()),
        Some(container_client_options),
    )?;
    if create {
        container_client.create(None).await?;
    }
    Ok(container_client)
}

/// Creates a test blob with no options, containing the data "b'hello rusty world'" with content length 17 if no data specified.
///
/// # Arguments
///
/// * `blob_client` - A reference to a BlobClient instance.
/// * `data` - Blob content to be uploaded.
/// * `options` - Optional configuration for the upload request.
pub async fn create_test_blob(
    blob_client: &BlobClient,
    data: Option<RequestContent<Bytes, NoFormat>>,
    options: Option<BlockBlobClientUploadOptions<'_>>,
) -> Result<BlockBlobClientUploadResult> {
    match data {
        Some(content) => blob_client.upload(content, options).await,
        None => {
            blob_client
                .upload(RequestContent::from(b"hello rusty world".to_vec()), options)
                .await
        }
    }
}

pub trait ClientOptionsExt {
    fn with_per_call_policy(self, policy: Arc<dyn Policy + 'static>) -> Self;
    fn with_per_try_policy(self, policy: Arc<dyn Policy + 'static>) -> Self;
}
impl ClientOptionsExt for BlobServiceClientOptions {
    fn with_per_call_policy(mut self, policy: Arc<dyn Policy + 'static>) -> Self {
        self.client_options.per_call_policies.push(policy);
        self
    }

    fn with_per_try_policy(mut self, policy: Arc<dyn Policy + 'static>) -> Self {
        self.client_options.per_try_policies.push(policy);
        self
    }
}
impl ClientOptionsExt for BlobContainerClientOptions {
    fn with_per_call_policy(mut self, policy: Arc<dyn Policy + 'static>) -> Self {
        self.client_options.per_call_policies.push(policy);
        self
    }

    fn with_per_try_policy(mut self, policy: Arc<dyn Policy + 'static>) -> Self {
        self.client_options.per_try_policies.push(policy);
        self
    }
}
impl ClientOptionsExt for BlobClientOptions {
    fn with_per_call_policy(mut self, policy: Arc<dyn Policy + 'static>) -> Self {
        self.client_options.per_call_policies.push(policy);
        self
    }

    fn with_per_try_policy(mut self, policy: Arc<dyn Policy + 'static>) -> Self {
        self.client_options.per_try_policies.push(policy);
        self
    }
}

pub struct AssertionScope {
    counter: Arc<AtomicUsize>,
}

impl Drop for AssertionScope {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::Relaxed);
    }
}

type Check<T> = Arc<dyn Fn(&T) -> Result<()> + Send + Sync>;
type Predicate<T> = Arc<dyn Fn(&T) -> bool + Send + Sync>;

pub mod predicates {
    use azure_core::http::Request;

    pub fn is_stage_block_request(request: &Request) -> bool {
        if let Some(url_query) = request.url().query() {
            url_query.contains("comp=block") && !url_query.contains("blocklist")
        } else {
            false
        }
    }
}

pub struct TestPolicy {
    request_scope_counter: Arc<AtomicUsize>,
    response_scope_counter: Arc<AtomicUsize>,
    on_request: Check<Request>,
    on_response: Check<AsyncRawResponse>,
}

impl Default for TestPolicy {
    fn default() -> Self {
        Self {
            request_scope_counter: Default::default(),
            response_scope_counter: Default::default(),
            on_request: Arc::new(|_| Ok(())),
            on_response: Arc::new(|_| Ok(())),
        }
    }
}

impl TestPolicy {
    pub fn new(
        on_request: Option<Check<Request>>,
        on_response: Option<Check<AsyncRawResponse>>,
    ) -> Self {
        TestPolicy {
            on_request: on_request.unwrap_or(Arc::new(|_| Ok(()))),
            on_response: on_response.unwrap_or(Arc::new(|_| Ok(()))),
            ..Self::default()
        }
    }

    pub fn capture(request_sender: Option<Sender<Request>>) -> Self {
        TestPolicy {
            on_request: match request_sender {
                Some(sender) => Arc::new(move |req| {
                    sender.send(req.clone()).map_err(|e| {
                        azure_core::Error::with_error(ErrorKind::Other, e, "Capture failure.")
                    })
                }),
                None => Arc::new(|_| Ok(())),
            },
            ..Self::default()
        }
    }

    pub fn count_requests(count: Arc<AtomicUsize>, predicate: Option<Predicate<Request>>) -> Self {
        Self::new(
            match predicate {
                Some(pred) => Some(Arc::new(move |request| {
                    if pred(request) {
                        count.fetch_add(1, Ordering::Relaxed);
                    }
                    Ok(())
                })),
                None => Some(Arc::new(move |_| {
                    count.fetch_add(1, Ordering::Relaxed);
                    Ok(())
                })),
            },
            None,
        )
    }

    /// DO NOT assign this to `_`. It will be dropped immediately instead of the intended scope.
    pub fn check_request_scope(&self) -> AssertionScope {
        self.request_scope_counter.fetch_add(1, Ordering::Relaxed);
        AssertionScope {
            counter: self.request_scope_counter.clone(),
        }
    }

    /// DO NOT assign this to `_`. It will be dropped immediately instead of the intended scope.
    pub fn check_response_scope(&self) -> AssertionScope {
        self.response_scope_counter.fetch_add(1, Ordering::Relaxed);
        AssertionScope {
            counter: self.response_scope_counter.clone(),
        }
    }
}

#[async_trait]
impl Policy for TestPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        if self.request_scope_counter.load(Ordering::Relaxed) > 0 {
            (self.on_request)(request)?;
        }
        let response = next[0].send(ctx, request, &next[1..]).await?;
        if self.response_scope_counter.load(Ordering::Relaxed) > 0 {
            (self.on_response)(&response)?;
        }
        Ok(response)
    }
}

impl std::fmt::Debug for TestPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<TestPolicy>())
            .field("check_request_counter", &self.request_scope_counter)
            .field("check_response_counter", &self.response_scope_counter)
            .finish()
    }
}

/// A [`Policy`] that fails (returns an `Io` error) for the first `fail_count` invocations
/// without forwarding the request downstream, then passes through normally.
///
/// This is designed to sit inside the retry loop via `per_try_policies` so that the SDK's
/// retry infrastructure can be exercised without any real network calls.
pub struct FailFirstPolicy {
    fail_count: usize,
    call_count: Arc<AtomicUsize>,
}

impl FailFirstPolicy {
    /// Creates a new `FailFirstPolicy`.
    ///
    /// * `fail_count` - number of initial invocations that will return an error.
    /// * `call_count` - shared counter incremented on every invocation (total, including failures).
    pub fn new(fail_count: usize, call_count: Arc<AtomicUsize>) -> Self {
        Self {
            fail_count,
            call_count,
        }
    }
}

impl std::fmt::Debug for FailFirstPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FailFirstPolicy")
            .field("fail_count", &self.fail_count)
            .field("call_count", &self.call_count)
            .finish()
    }
}

#[async_trait]
impl Policy for FailFirstPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let n = self.call_count.fetch_add(1, Ordering::SeqCst);
        if n < self.fail_count {
            return Err(azure_core::Error::new(
                ErrorKind::Io,
                std::io::Error::new(
                    std::io::ErrorKind::ConnectionRefused,
                    "simulated transient error",
                ),
            ));
        }
        next[0].send(ctx, request, &next[1..]).await
    }
}

/// Polls an async condition until it returns `true`, with behavior adapted to the test mode.
///
/// - **Live**: polls every 5 seconds up to 60 seconds total, panicking on timeout.
/// - **Record**: sleeps 15 seconds, then returns (caller asserts after).
/// - **Playback**: returns immediately.
///
/// # Arguments
///
/// * `recording` - The current test recording context.
/// * `check` - An async closure that returns `Ok(true)` when the condition is met.
pub async fn poll_until<F, Fut>(recording: &Recording, mut check: F) -> Result<()>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<bool>>,
{
    match recording.test_mode() {
        TestMode::Live => {
            let deadline = tokio::time::Instant::now() + Duration::from_secs(60);
            loop {
                if check().await? {
                    return Ok(());
                }
                assert!(
                    tokio::time::Instant::now() < deadline,
                    "Timed out after 60s waiting for eventual consistency"
                );
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
        TestMode::Record => {
            tokio::time::sleep(Duration::from_secs(15)).await;
            Ok(())
        }
        TestMode::Playback => Ok(()),
    }
}
