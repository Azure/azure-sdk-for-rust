// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    slice,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use async_trait::async_trait;
use azure_core::{
    http::StatusCode,
    http::{
        policies::{Policy, PolicyResult},
        AsyncRawResponse, Body, ClientOptions, Context, NoFormat, Request, RequestContent,
        Response,
    },
    Bytes, Result,
};
use azure_core_test::Recording;
use azure_storage_blob::{
    models::{BlockBlobClientUploadOptions, BlockBlobClientUploadResult, EncryptionAlgorithmType},
    BlobClient, BlobClientOptions, BlobContainerClient, BlobContainerClientOptions,
    BlobServiceClient, BlobServiceClientOptions,
};
use bytes::BytesMut;
use futures::{AsyncRead, AsyncReadExt};

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

/// Returns an encryption scope name that should not exist in test accounts.
pub fn get_invalid_encryption_scope() -> String {
    "invalid-encryption-scope-for-tests".to_string()
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
    BlobServiceClient::new(
        &endpoint,
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
    let container_client = BlobContainerClient::new(
        &endpoint,
        &container_name,
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
) -> Result<Response<BlockBlobClientUploadResult, NoFormat>> {
    match data {
        Some(content) => {
            blob_client
                .upload(content.clone(), true, content.body().len() as u64, options)
                .await
        }
        None => {
            blob_client
                .upload(
                    RequestContent::from(b"hello rusty world".to_vec()),
                    true,
                    17,
                    options,
                )
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

#[async_trait]
pub trait AsyncReadTestExt {
    async fn read_into_spare_capacity(
        &mut self,
        buffer: &mut BytesMut,
    ) -> futures::io::Result<usize>;
}

#[async_trait]
impl<Stream: AsyncRead + Unpin + Send> AsyncReadTestExt for Stream {
    async fn read_into_spare_capacity(
        &mut self,
        buffer: &mut BytesMut,
    ) -> futures::io::Result<usize> {
        let spare_capacity = buffer.spare_capacity_mut();
        let spare_capacity = unsafe {
            // spare_capacity_mut() gives us the known remaining capacity of BytesMut.
            // Those bytes are valid reserved memory but have had no values written
            // to them. Those are the exact bytes we want to write into.
            // MaybeUninit<u8> can be safely cast into u8, and so this pointer cast
            // is safe. Since the spare capacity length is safely known, we can
            // provide those to from_raw_parts without worry.
            slice::from_raw_parts_mut(spare_capacity.as_mut_ptr() as *mut u8, spare_capacity.len())
        };
        let bytes_read = self.read(spare_capacity).await?;
        // read() wrote bytes_read-many bytes into the spare capacity.
        // those values are therefore initialized and we can add them to
        // the existing buffer length
        unsafe { buffer.set_len(buffer.len() + bytes_read) };
        Ok(bytes_read)
    }
}

#[async_trait]
pub trait BodyTestExt {
    async fn collect_bytes(&mut self) -> azure_core::Result<Bytes>;
}

#[async_trait]
impl BodyTestExt for Body {
    async fn collect_bytes(&mut self) -> azure_core::Result<Bytes> {
        match self {
            Body::Bytes(bytes) => Ok(bytes.clone()),
            Body::SeekableStream(seekable_stream) => {
                seekable_stream.reset().await?;
                let mut bytes = BytesMut::with_capacity(seekable_stream.len());
                while seekable_stream.read_into_spare_capacity(&mut bytes).await? != 0 {}
                seekable_stream.reset().await?;
                Ok(bytes.freeze())
            }
        }
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

impl TestPolicy {
    pub fn new(
        on_request: Option<Check<Request>>,
        on_response: Option<Check<AsyncRawResponse>>,
    ) -> Self {
        TestPolicy {
            request_scope_counter: Arc::new(AtomicUsize::new(0)),
            response_scope_counter: Arc::new(AtomicUsize::new(0)),
            on_request: on_request.unwrap_or(Arc::new(|_| Ok(()))),
            on_response: on_response.unwrap_or(Arc::new(|_| Ok(()))),
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
