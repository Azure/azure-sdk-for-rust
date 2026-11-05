// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Host-provided token credential support for the native ABI.

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex as StdMutex, OnceLock, Weak,
    },
};

use async_trait::async_trait;
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::ErrorKind,
    time::{Duration, OffsetDateTime},
    Error,
};
use tokio::{sync::oneshot, task::AbortHandle, time::sleep};

use crate::error::{CosmosErrorCode, CosmosStatusCode};

const TOKEN_REFRESH_RETRY_COUNT: usize = 2;
const MINIMUM_BACKGROUND_REFRESH_INTERVAL: Duration = Duration::minutes(1);

type TokenResultSender = oneshot::Sender<azure_core::Result<AccessToken>>;
type RefreshResult = Result<AccessToken, String>;
type RefreshWaiter = oneshot::Sender<RefreshResult>;

enum RefreshPolicy {
    UseValidCache,
    RefreshCachedToken { expected_expiry: OffsetDateTime },
}

static NEXT_TOKEN_REQUEST_ID: AtomicU64 = AtomicU64::new(1);
static PENDING_TOKEN_REQUESTS: OnceLock<StdMutex<HashMap<u64, TokenResultSender>>> =
    OnceLock::new();

fn pending_token_requests() -> &'static StdMutex<HashMap<u64, TokenResultSender>> {
    PENDING_TOKEN_REQUESTS.get_or_init(|| StdMutex::new(HashMap::new()))
}

/// Starts asynchronous token acquisition in the host.
///
/// Returning zero accepts the request; the host must eventually call
/// [`cosmos_token_request_complete`] with `request.request_id`. Returning
/// nonzero rejects synchronously; the host must not complete the request.
pub type CosmosTokenProviderCallback =
    unsafe extern "C" fn(user_data: isize, request: *const CosmosTokenRequest) -> i32;

/// Releases the host-owned state associated with a token provider.
pub type CosmosTokenProviderFree = unsafe extern "C" fn(user_data: isize);

/// One asynchronous access-token request passed to the host.
///
/// `scope` is borrowed and valid only until the token-provider callback
/// returns. The host must copy it before starting asynchronous work.
#[repr(C)]
pub struct CosmosTokenRequest {
    /// Opaque identifier passed to [`cosmos_token_request_complete`].
    pub request_id: u64,
    /// UTF-8 token scope bytes.
    pub scope: *const u8,
    /// Number of bytes addressable from `scope`.
    pub scope_len: usize,
}

/// Host callbacks used to acquire tokens and release host state.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosTokenProvider {
    /// Starts token acquisition. Must be non-NULL.
    pub get_token:
        Option<unsafe extern "C" fn(user_data: isize, request: *const CosmosTokenRequest) -> i32>,
    /// Releases `user_data` after the last Rust credential reference is gone.
    pub user_data_free: Option<unsafe extern "C" fn(user_data: isize)>,
}

struct HostTokenProvider {
    provider: CosmosTokenProvider,
    user_data: isize,
}

#[derive(Default)]
struct CredentialState {
    cached_token: Option<AccessToken>,
    refresh_waiters: Option<Vec<RefreshWaiter>>,
}

struct CallbackTokenCredential {
    provider: Arc<HostTokenProvider>,
    state: Arc<StdMutex<CredentialState>>,
    background_task: StdMutex<Option<AbortHandle>>,
}

impl CallbackTokenCredential {
    fn new(provider: CosmosTokenProvider, user_data: isize) -> Option<Arc<dyn TokenCredential>> {
        provider.get_token?;
        Some(Arc::new(Self {
            provider: Arc::new(HostTokenProvider {
                provider,
                user_data,
            }),
            state: Arc::new(StdMutex::new(CredentialState::default())),
            background_task: StdMutex::new(None),
        }))
    }

    fn start_background_refresh(&self, scope: String, token: &AccessToken) {
        let mut background_task = self.background_task.lock().unwrap();
        if background_task
            .as_ref()
            .is_some_and(|task| !task.is_finished())
        {
            return;
        }

        let provider = Arc::downgrade(&self.provider);
        let state = Arc::clone(&self.state);
        let expires_on = token.expires_on;
        let task = tokio::spawn(async move {
            background_refresh_loop(provider, state, scope, expires_on).await;
        });
        *background_task = Some(task.abort_handle());
    }
}

impl fmt::Debug for CallbackTokenCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallbackTokenCredential")
            .field("provider", &"<host callback>")
            .field("user_data", &"<opaque>")
            .finish_non_exhaustive()
    }
}

impl Drop for CallbackTokenCredential {
    fn drop(&mut self) {
        if let Some(task) = self.background_task.lock().unwrap().take() {
            task.abort();
        }
    }
}

impl Drop for HostTokenProvider {
    fn drop(&mut self) {
        if let Some(free) = self.provider.user_data_free {
            // SAFETY: `user_data` is the opaque value supplied with this
            // provider, and the callback is invoked exactly once on final drop.
            unsafe {
                free(self.user_data);
            }
        }
    }
}

#[async_trait]
impl TokenCredential for CallbackTokenCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let [scope] = scopes else {
            return Err(credential_error(
                "host token credentials require exactly one scope",
            ));
        };

        let token = get_or_start_refresh(
            Arc::clone(&self.provider),
            Arc::clone(&self.state),
            (*scope).to_owned(),
            RefreshPolicy::UseValidCache,
        )
        .await?;
        self.start_background_refresh((*scope).to_owned(), &token);
        Ok(token)
    }
}

async fn background_refresh_loop(
    provider: Weak<HostTokenProvider>,
    state: Arc<StdMutex<CredentialState>>,
    scope: String,
    mut tracked_expiry: OffsetDateTime,
) {
    let mut delay = successful_refresh_delay(tracked_expiry, OffsetDateTime::now_utc());

    loop {
        sleep(delay).await;

        let Some(provider) = provider.upgrade() else {
            return;
        };

        match get_or_start_refresh(
            provider,
            Arc::clone(&state),
            scope.clone(),
            RefreshPolicy::RefreshCachedToken {
                expected_expiry: tracked_expiry,
            },
        )
        .await
        {
            Ok(token) => {
                tracked_expiry = token.expires_on;
                delay = successful_refresh_delay(tracked_expiry, OffsetDateTime::now_utc());
            }
            Err(error) => {
                tracing::warn!(
                    error = %error,
                    "background AAD token refresh failed; retaining the cached token"
                );
                let Some(retry_delay) =
                    failed_refresh_retry_delay(tracked_expiry, OffsetDateTime::now_utc())
                else {
                    return;
                };
                delay = retry_delay;
            }
        }
    }
}

async fn get_or_start_refresh(
    provider: Arc<HostTokenProvider>,
    state: Arc<StdMutex<CredentialState>>,
    scope: String,
    policy: RefreshPolicy,
) -> azure_core::Result<AccessToken> {
    let (receiver, start_refresh) = {
        let mut state = state.lock().unwrap();
        match policy {
            RefreshPolicy::UseValidCache => {
                if let Some(token) = valid_cached_token(&state, OffsetDateTime::now_utc()) {
                    return Ok(token);
                }
            }
            RefreshPolicy::RefreshCachedToken { expected_expiry } => {
                let Some(token) = state.cached_token.as_ref() else {
                    return Err(credential_error(
                        "cached token unavailable for background refresh",
                    ));
                };
                if token.expires_on != expected_expiry {
                    return Ok(token.clone());
                }
            }
        }

        let (sender, receiver) = oneshot::channel();
        if let Some(waiters) = state.refresh_waiters.as_mut() {
            waiters.push(sender);
            (receiver, false)
        } else {
            state.refresh_waiters = Some(vec![sender]);
            (receiver, true)
        }
    };

    if start_refresh {
        let refresh_state = Arc::clone(&state);
        tokio::spawn(async move {
            let result = request_token_with_retry(&provider, &scope)
                .await
                .map_err(|error| error.to_string());
            let waiters = {
                let mut state = refresh_state.lock().unwrap();
                if let Ok(token) = &result {
                    state.cached_token = Some(token.clone());
                }
                let Some(waiters) = state.refresh_waiters.take() else {
                    tracing::error!("active token refresh lost its waiter list");
                    return;
                };
                waiters
            };

            for waiter in waiters {
                let waiter_result = match &result {
                    Ok(token) => Ok(token.clone()),
                    Err(message) => Err(message.clone()),
                };
                let _ = waiter.send(waiter_result);
            }
        });
    }

    receiver
        .await
        .map_err(|_| credential_error("host token refresh task ended without a result"))?
        .map_err(credential_error)
}

fn valid_cached_token(state: &CredentialState, now: OffsetDateTime) -> Option<AccessToken> {
    state
        .cached_token
        .as_ref()
        .filter(|token| token.expires_on > now)
        .cloned()
}

async fn request_token_with_retry(
    provider: &HostTokenProvider,
    scope: &str,
) -> azure_core::Result<AccessToken> {
    let mut last_error = None;
    for _ in 0..TOKEN_REFRESH_RETRY_COUNT {
        match provider.request_token(scope).await {
            Ok(token) => return Ok(token),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.expect("token refresh retry count is nonzero"))
}

impl HostTokenProvider {
    async fn request_token(&self, scope: &str) -> azure_core::Result<AccessToken> {
        let (request_id, receiver, _pending_request) = register_pending_token_request();
        let status = {
            let request = CosmosTokenRequest {
                request_id,
                scope: scope.as_ptr(),
                scope_len: scope.len(),
            };
            // SAFETY: the provider callback is supplied by the host under the C ABI
            // contract. The request and scope remain valid for the duration of the call.
            unsafe {
                (self
                    .provider
                    .get_token
                    .expect("validated when the credential was constructed"))(
                    self.user_data,
                    &request,
                )
            }
        };
        if status != 0 {
            return Err(credential_error(format!(
                "host token provider rejected the request with status {status}"
            )));
        }

        receiver.await.map_err(|_| {
            credential_error("host token provider dropped the token request without completing it")
        })?
    }
}

struct PendingTokenRequestGuard {
    request_id: u64,
}

impl Drop for PendingTokenRequestGuard {
    fn drop(&mut self) {
        pending_token_requests()
            .lock()
            .unwrap()
            .remove(&self.request_id);
    }
}

fn register_pending_token_request() -> (
    u64,
    oneshot::Receiver<azure_core::Result<AccessToken>>,
    PendingTokenRequestGuard,
) {
    let (sender, receiver) = oneshot::channel();
    let mut sender = Some(sender);
    let request_id = loop {
        let candidate = NEXT_TOKEN_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
        if candidate == 0 {
            continue;
        }
        let mut pending = pending_token_requests().lock().unwrap();
        if let std::collections::hash_map::Entry::Vacant(entry) = pending.entry(candidate) {
            entry.insert(sender.take().expect("sender is inserted exactly once"));
            break candidate;
        }
    };
    (
        request_id,
        receiver,
        PendingTokenRequestGuard { request_id },
    )
}

fn successful_refresh_delay(
    expires_on: OffsetDateTime,
    now: OffsetDateTime,
) -> std::time::Duration {
    let half_remaining: Duration = (expires_on - now) / 2_i32;
    // Match .NET Cosmos: very short-lived tokens use the one-minute floor and
    // may expire first, in which case foreground acquisition handles refresh.
    duration_to_std(half_remaining.max(MINIMUM_BACKGROUND_REFRESH_INTERVAL))
}

fn failed_refresh_retry_delay(
    expires_on: OffsetDateTime,
    now: OffsetDateTime,
) -> Option<std::time::Duration> {
    let half_remaining: Duration = (expires_on - now) / 2_i32;
    (half_remaining >= MINIMUM_BACKGROUND_REFRESH_INTERVAL).then(|| duration_to_std(half_remaining))
}

fn duration_to_std(duration: Duration) -> std::time::Duration {
    duration
        .try_into()
        .expect("refresh intervals are always nonnegative")
}

/// Completes a pending host token request.
///
/// The host calls this function exactly once after accepting `request_id` in
/// its token-provider callback. Token and error buffers are borrowed only for
/// this call; Rust copies them before returning. Unknown, cancelled, late, or
/// duplicate request IDs return `400 / CLIENT_FFI_NULL_ARGUMENT`.
///
/// # Safety
///
/// Non-NULL buffers must remain readable for their corresponding lengths for
/// the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn cosmos_token_request_complete(
    request_id: u64,
    status: i32,
    token: *const u8,
    token_len: usize,
    expires_on_unix_seconds: i64,
    error_message: *const u8,
    error_message_len: usize,
) -> CosmosStatusCode {
    let Some(sender) = pending_token_requests().lock().unwrap().remove(&request_id) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };

    let result = build_access_token(
        status,
        token,
        token_len,
        expires_on_unix_seconds,
        error_message,
        error_message_len,
    );
    let _ = sender.send(result);
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}

pub(crate) fn create_token_credential(
    provider: CosmosTokenProvider,
    user_data: isize,
) -> Option<Arc<dyn TokenCredential>> {
    CallbackTokenCredential::new(provider, user_data)
}

fn credential_error(message: impl Into<String>) -> Error {
    Error::with_message(ErrorKind::Credential, message.into())
}

fn build_access_token(
    status: i32,
    token: *const u8,
    token_len: usize,
    expires_on_unix_seconds: i64,
    error_message: *const u8,
    error_message_len: usize,
) -> azure_core::Result<AccessToken> {
    if status != 0 {
        // Mirror the sync-reject path (which includes the numeric status) so
        // callers can classify AAD failures without string-matching against the
        // host-supplied message.
        let message = copy_host_bytes(error_message, error_message_len)
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .unwrap_or_else(|_| "no error message".to_string());
        return Err(credential_error(format!(
            "host token provider failed with status {status}: {message}"
        )));
    }

    let token = copy_host_bytes(token, token_len)?;
    let token = String::from_utf8(token)
        .map_err(|_| credential_error("host token provider returned a non-UTF-8 access token"))?;
    if token.is_empty() {
        return Err(credential_error(
            "host token provider returned an empty access token",
        ));
    }
    let expires_on =
        OffsetDateTime::from_unix_timestamp(expires_on_unix_seconds).map_err(|_| {
            credential_error("host token provider returned an invalid expiry timestamp")
        })?;
    if expires_on <= OffsetDateTime::now_utc() {
        return Err(credential_error(
            "host token provider returned an expired access token",
        ));
    }
    Ok(AccessToken::new(Secret::new(token), expires_on))
}

fn copy_host_bytes(ptr: *const u8, len: usize) -> azure_core::Result<Vec<u8>> {
    if len == 0 {
        return Ok(Vec::new());
    }
    if ptr.is_null() {
        return Err(credential_error(
            "host token provider returned a NULL buffer with a nonzero length",
        ));
    }
    // SAFETY: the host guarantees the buffer is readable for `len` bytes for
    // the duration of the completion callback. Copy before returning.
    Ok(unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex as StdMutex,
    };

    struct TestProvider {
        calls: AtomicUsize,
        frees: AtomicUsize,
        tokens: StdMutex<Vec<(String, i64)>>,
        scopes: StdMutex<Vec<String>>,
        pending_request_ids: StdMutex<Vec<u64>>,
    }

    unsafe extern "C" fn get_token(user_data: isize, request: *const CosmosTokenRequest) -> i32 {
        // SAFETY: tests pass a live `Arc<TestProvider>` raw pointer as user data.
        let state = unsafe { &*(user_data as *const TestProvider) };
        // SAFETY: the native adapter passes a valid request for this call.
        let request = unsafe { &*request };
        // SAFETY: scope bytes are valid for the duration of this callback.
        let scope = unsafe { std::slice::from_raw_parts(request.scope, request.scope_len) };
        state
            .scopes
            .lock()
            .unwrap()
            .push(String::from_utf8(scope.to_vec()).unwrap());
        state.calls.fetch_add(1, Ordering::SeqCst);
        let (token, expires_on) = state.tokens.lock().unwrap().remove(0);
        // SAFETY: token bytes remain alive until this synchronous completion
        // call returns, and the native function copies them.
        unsafe {
            cosmos_token_request_complete(
                request.request_id,
                0,
                token.as_ptr(),
                token.len(),
                expires_on,
                std::ptr::null(),
                0,
            );
        }
        0
    }

    unsafe extern "C" fn get_token_async(
        user_data: isize,
        request: *const CosmosTokenRequest,
    ) -> i32 {
        // SAFETY: tests pass a live `Arc<TestProvider>` raw pointer as user data.
        let state = unsafe { &*(user_data as *const TestProvider) };
        // SAFETY: the native adapter passes a valid request for this call.
        let request = unsafe { &*request };
        // SAFETY: scope bytes are valid for the duration of this callback.
        let scope = unsafe { std::slice::from_raw_parts(request.scope, request.scope_len) };
        state
            .scopes
            .lock()
            .unwrap()
            .push(String::from_utf8(scope.to_vec()).unwrap());
        state.calls.fetch_add(1, Ordering::SeqCst);
        let (token, expires_on) = state.tokens.lock().unwrap().remove(0);
        let request_id = request.request_id;
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(20));
            // SAFETY: token bytes remain live until completion returns.
            unsafe {
                cosmos_token_request_complete(
                    request_id,
                    0,
                    token.as_ptr(),
                    token.len(),
                    expires_on,
                    std::ptr::null(),
                    0,
                );
            }
        });
        0
    }

    unsafe extern "C" fn capture_token_request(
        user_data: isize,
        request: *const CosmosTokenRequest,
    ) -> i32 {
        // SAFETY: tests pass a live `Arc<TestProvider>` raw pointer as user data.
        let state = unsafe { &*(user_data as *const TestProvider) };
        // SAFETY: the native adapter passes a valid request for this call.
        let request = unsafe { &*request };
        // SAFETY: scope bytes are valid for the duration of this callback.
        let scope = unsafe { std::slice::from_raw_parts(request.scope, request.scope_len) };
        state
            .scopes
            .lock()
            .unwrap()
            .push(String::from_utf8(scope.to_vec()).unwrap());
        state
            .pending_request_ids
            .lock()
            .unwrap()
            .push(request.request_id);
        state.calls.fetch_add(1, Ordering::SeqCst);
        0
    }

    unsafe extern "C" fn reject_token(
        _user_data: isize,
        _request: *const CosmosTokenRequest,
    ) -> i32 {
        17
    }

    unsafe extern "C" fn fail_token(user_data: isize, request: *const CosmosTokenRequest) -> i32 {
        const MESSAGE: &[u8] = b"credential unavailable";
        // SAFETY: tests pass a live `Arc<TestProvider>` raw pointer as user data.
        let state = unsafe { &*(user_data as *const TestProvider) };
        state.calls.fetch_add(1, Ordering::SeqCst);
        // SAFETY: the native adapter passes a valid request for this call.
        let request = unsafe { &*request };
        // SAFETY: the message remains live for the duration of completion.
        unsafe {
            cosmos_token_request_complete(
                request.request_id,
                1,
                std::ptr::null(),
                0,
                0,
                MESSAGE.as_ptr(),
                MESSAGE.len(),
            );
        }
        0
    }

    unsafe extern "C" fn get_token_then_fail(
        user_data: isize,
        request: *const CosmosTokenRequest,
    ) -> i32 {
        // SAFETY: tests pass a live `Arc<TestProvider>` raw pointer as user data.
        let state = unsafe { &*(user_data as *const TestProvider) };
        let call = state.calls.fetch_add(1, Ordering::SeqCst);
        // SAFETY: the native adapter passes a valid request for this call.
        let request = unsafe { &*request };
        if call == 0 {
            let (token, expires_on) = state.tokens.lock().unwrap().remove(0);
            // SAFETY: token bytes remain alive for this synchronous call.
            unsafe {
                cosmos_token_request_complete(
                    request.request_id,
                    0,
                    token.as_ptr(),
                    token.len(),
                    expires_on,
                    std::ptr::null(),
                    0,
                );
            }
        } else {
            const MESSAGE: &[u8] = b"credential unavailable";
            // SAFETY: message bytes remain alive for this synchronous call.
            unsafe {
                cosmos_token_request_complete(
                    request.request_id,
                    1,
                    std::ptr::null(),
                    0,
                    0,
                    MESSAGE.as_ptr(),
                    MESSAGE.len(),
                );
            }
        }
        0
    }

    unsafe extern "C" fn fail_once_then_get_token(
        user_data: isize,
        request: *const CosmosTokenRequest,
    ) -> i32 {
        // SAFETY: tests pass a live `Arc<TestProvider>` raw pointer as user data.
        let state = unsafe { &*(user_data as *const TestProvider) };
        let call = state.calls.fetch_add(1, Ordering::SeqCst);
        // SAFETY: the native adapter passes a valid request for this call.
        let request = unsafe { &*request };
        if call == 0 {
            const MESSAGE: &[u8] = b"transient failure";
            // SAFETY: message bytes remain alive for this synchronous call.
            unsafe {
                cosmos_token_request_complete(
                    request.request_id,
                    1,
                    std::ptr::null(),
                    0,
                    0,
                    MESSAGE.as_ptr(),
                    MESSAGE.len(),
                );
            }
        } else {
            let (token, expires_on) = state.tokens.lock().unwrap().remove(0);
            // SAFETY: token bytes remain alive for this synchronous call.
            unsafe {
                cosmos_token_request_complete(
                    request.request_id,
                    0,
                    token.as_ptr(),
                    token.len(),
                    expires_on,
                    std::ptr::null(),
                    0,
                );
            }
        }
        0
    }

    unsafe extern "C" fn free_provider(user_data: isize) {
        // SAFETY: ownership of this strong reference was transferred to the
        // credential at construction.
        let state = unsafe { Arc::from_raw(user_data as *const TestProvider) };
        state.frees.fetch_add(1, Ordering::SeqCst);
    }

    fn credential(tokens: Vec<(String, i64)>) -> (Arc<TestProvider>, Arc<dyn TokenCredential>) {
        credential_with_callback(tokens, get_token)
    }

    fn credential_with_callback(
        tokens: Vec<(String, i64)>,
        get_token: CosmosTokenProviderCallback,
    ) -> (Arc<TestProvider>, Arc<dyn TokenCredential>) {
        let state = Arc::new(TestProvider {
            calls: AtomicUsize::new(0),
            frees: AtomicUsize::new(0),
            tokens: StdMutex::new(tokens),
            scopes: StdMutex::new(Vec::new()),
            pending_request_ids: StdMutex::new(Vec::new()),
        });
        let user_data = Arc::into_raw(Arc::clone(&state)) as isize;
        let credential = create_token_credential(
            CosmosTokenProvider {
                get_token: Some(get_token),
                user_data_free: Some(free_provider),
            },
            user_data,
        )
        .unwrap();
        (state, credential)
    }

    async fn wait_for_call_count(state: &TestProvider, expected: usize) {
        for _ in 0..20 {
            if state.calls.load(Ordering::SeqCst) == expected {
                return;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(state.calls.load(Ordering::SeqCst), expected);
    }

    #[tokio::test]
    async fn uses_valid_cached_token() {
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(10))
            .unix_timestamp();
        let (state, credential) = credential(vec![("token-a".to_string(), expires)]);

        let first = credential.get_token(&["scope"], None).await.unwrap();
        let second = credential.get_token(&["scope"], None).await.unwrap();

        assert_eq!(first.token.secret(), "token-a");
        assert_eq!(second.token.secret(), "token-a");
        assert_eq!(state.calls.load(Ordering::SeqCst), 1);
        assert_eq!(state.scopes.lock().unwrap().as_slice(), ["scope"]);
    }

    #[tokio::test(start_paused = true)]
    async fn background_refreshes_token_at_half_lifetime() {
        let now = OffsetDateTime::now_utc();
        let (state, credential) = credential(vec![
            (
                "token-a".to_string(),
                now.saturating_add(Duration::minutes(2)).unix_timestamp(),
            ),
            (
                "token-b".to_string(),
                now.saturating_add(Duration::minutes(10)).unix_timestamp(),
            ),
        ]);

        let first = credential.get_token(&["scope"], None).await.unwrap();
        tokio::task::yield_now().await;
        tokio::time::advance(std::time::Duration::from_secs(60)).await;
        for _ in 0..10 {
            tokio::task::yield_now().await;
            if state.calls.load(Ordering::SeqCst) == 2 {
                break;
            }
        }
        let second = credential.get_token(&["scope"], None).await.unwrap();

        assert_eq!(first.token.secret(), "token-a");
        assert_eq!(second.token.secret(), "token-b");
        assert_eq!(state.calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn awaits_asynchronous_host_completion() {
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(10))
            .unix_timestamp();
        let (state, credential) =
            credential_with_callback(vec![("token-a".to_string(), expires)], get_token_async);

        let token = credential.get_token(&["scope"], None).await.unwrap();

        assert_eq!(token.token.secret(), "token-a");
        assert_eq!(state.calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn concurrent_requests_single_flight_refresh() {
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(10))
            .unix_timestamp();
        let (state, credential) =
            credential_with_callback(vec![("token-a".to_string(), expires)], get_token_async);

        let (a, b, c, d) = tokio::join!(
            credential.get_token(&["scope"], None),
            credential.get_token(&["scope"], None),
            credential.get_token(&["scope"], None),
            credential.get_token(&["scope"], None),
        );

        for token in [a, b, c, d] {
            assert_eq!(token.unwrap().token.secret(), "token-a");
        }
        assert_eq!(state.calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn cancelling_initiator_keeps_refresh_single_flight() {
        let (state, credential) = credential_with_callback(Vec::new(), capture_token_request);
        let first_credential = Arc::clone(&credential);
        let first = tokio::spawn(async move { first_credential.get_token(&["scope"], None).await });
        wait_for_call_count(&state, 1).await;

        first.abort();
        assert!(first.await.unwrap_err().is_cancelled());

        let second_credential = Arc::clone(&credential);
        let second =
            tokio::spawn(async move { second_credential.get_token(&["scope"], None).await });
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }
        assert_eq!(state.calls.load(Ordering::SeqCst), 1);

        let request_id = state.pending_request_ids.lock().unwrap()[0];
        let token = b"token-after-cancellation";
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(10))
            .unix_timestamp();
        // SAFETY: token bytes remain valid for this synchronous call.
        let status = unsafe {
            cosmos_token_request_complete(
                request_id,
                0,
                token.as_ptr(),
                token.len(),
                expires,
                std::ptr::null(),
                0,
            )
        };

        assert_eq!(
            status,
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
        );
        assert_eq!(
            second.await.unwrap().unwrap().token.secret(),
            "token-after-cancellation"
        );
        assert_eq!(state.calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn accepted_refresh_retains_provider_after_waiter_cancellation() {
        let (state, credential) = credential_with_callback(Vec::new(), capture_token_request);
        let task_credential = Arc::clone(&credential);
        let waiter = tokio::spawn(async move { task_credential.get_token(&["scope"], None).await });
        wait_for_call_count(&state, 1).await;

        waiter.abort();
        assert!(waiter.await.unwrap_err().is_cancelled());
        drop(credential);
        assert_eq!(state.frees.load(Ordering::SeqCst), 0);

        let request_id = state.pending_request_ids.lock().unwrap()[0];
        let token = b"token-after-drop";
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(10))
            .unix_timestamp();
        // SAFETY: token bytes remain valid for this synchronous call.
        let status = unsafe {
            cosmos_token_request_complete(
                request_id,
                0,
                token.as_ptr(),
                token.len(),
                expires,
                std::ptr::null(),
                0,
            )
        };
        assert_eq!(
            status,
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
        );

        for _ in 0..20 {
            if state.frees.load(Ordering::SeqCst) == 1 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(state.frees.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn concurrent_waiters_receive_same_refresh_error() {
        let (state, credential) = credential_with_callback(Vec::new(), fail_token);

        let (a, b, c) = tokio::join!(
            credential.get_token(&["scope"], None),
            credential.get_token(&["scope"], None),
            credential.get_token(&["scope"], None),
        );

        for result in [a, b, c] {
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("credential unavailable"));
        }
        assert_eq!(
            state.calls.load(Ordering::SeqCst),
            TOKEN_REFRESH_RETRY_COUNT
        );
    }

    #[tokio::test]
    async fn propagates_synchronous_provider_rejection() {
        let (_state, credential) = credential_with_callback(Vec::new(), reject_token);

        let error = credential.get_token(&["scope"], None).await.unwrap_err();

        assert!(error.to_string().contains("status 17"));
    }

    #[tokio::test]
    async fn propagates_host_completion_error() {
        let (state, credential) = credential_with_callback(Vec::new(), fail_token);

        let error = credential.get_token(&["scope"], None).await.unwrap_err();

        let message = error.to_string();
        assert!(
            message.contains("status 1"),
            "missing status code: {message}"
        );
        assert!(
            message.contains("credential unavailable"),
            "missing host message: {message}"
        );
        assert_eq!(
            state.calls.load(Ordering::SeqCst),
            TOKEN_REFRESH_RETRY_COUNT
        );
    }

    #[tokio::test]
    async fn retries_token_acquisition_once() {
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(10))
            .unix_timestamp();
        let (state, credential) = credential_with_callback(
            vec![("token-after-retry".to_string(), expires)],
            fail_once_then_get_token,
        );

        let token = credential.get_token(&["scope"], None).await.unwrap();

        assert_eq!(token.token.secret(), "token-after-retry");
        assert_eq!(
            state.calls.load(Ordering::SeqCst),
            TOKEN_REFRESH_RETRY_COUNT
        );
    }

    #[tokio::test]
    async fn rejects_expired_host_token() {
        let expires = OffsetDateTime::now_utc()
            .saturating_sub(Duration::seconds(1))
            .unix_timestamp();
        let (state, credential) = credential(vec![
            ("expired-a".to_string(), expires),
            ("expired-b".to_string(), expires),
        ]);

        let error = credential.get_token(&["scope"], None).await.unwrap_err();

        assert!(error.to_string().contains("expired access token"));
        assert_eq!(
            state.calls.load(Ordering::SeqCst),
            TOKEN_REFRESH_RETRY_COUNT
        );
    }

    #[tokio::test(start_paused = true)]
    async fn background_failure_keeps_valid_cached_token() {
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(2))
            .unix_timestamp();
        let (state, credential) =
            credential_with_callback(vec![("token-a".to_string(), expires)], get_token_then_fail);

        let first = credential.get_token(&["scope"], None).await.unwrap();
        tokio::task::yield_now().await;
        tokio::time::advance(std::time::Duration::from_secs(60)).await;
        for _ in 0..10 {
            tokio::task::yield_now().await;
            if state.calls.load(Ordering::SeqCst) == 3 {
                break;
            }
        }
        let second = credential.get_token(&["scope"], None).await.unwrap();

        assert_eq!(first.token.secret(), "token-a");
        assert_eq!(second.token.secret(), "token-a");
        assert_eq!(state.calls.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn refresh_delays_follow_half_lifetime_policy() {
        let now = OffsetDateTime::UNIX_EPOCH;

        assert_eq!(
            successful_refresh_delay(now + Duration::minutes(60), now),
            std::time::Duration::from_secs(30 * 60)
        );
        assert_eq!(
            successful_refresh_delay(now + Duration::seconds(30), now),
            std::time::Duration::from_secs(60)
        );
        assert_eq!(
            failed_refresh_retry_delay(now + Duration::minutes(8), now),
            Some(std::time::Duration::from_secs(4 * 60))
        );
        assert_eq!(
            failed_refresh_retry_delay(now + Duration::seconds(90), now),
            None
        );
    }

    #[tokio::test]
    async fn completion_rejects_duplicate_and_cancelled_request_ids() {
        let (request_id, receiver, guard) = register_pending_token_request();
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(10))
            .unix_timestamp();
        let token = b"token-a";

        // SAFETY: token bytes remain valid for each synchronous call.
        let first = unsafe {
            cosmos_token_request_complete(
                request_id,
                0,
                token.as_ptr(),
                token.len(),
                expires,
                std::ptr::null(),
                0,
            )
        };
        assert_eq!(
            first,
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
        );
        assert_eq!(receiver.await.unwrap().unwrap().token.secret(), "token-a");

        // SAFETY: all buffers are empty, so NULL is valid.
        let duplicate = unsafe {
            cosmos_token_request_complete(
                request_id,
                0,
                std::ptr::null(),
                0,
                expires,
                std::ptr::null(),
                0,
            )
        };
        assert_eq!(
            duplicate,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
        drop(guard);

        let (cancelled_id, _receiver, cancelled_guard) = register_pending_token_request();
        drop(cancelled_guard);
        // SAFETY: all buffers are empty, so NULL is valid.
        let cancelled = unsafe {
            cosmos_token_request_complete(
                cancelled_id,
                0,
                std::ptr::null(),
                0,
                expires,
                std::ptr::null(),
                0,
            )
        };
        assert_eq!(
            cancelled,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
    }

    #[tokio::test]
    async fn rejects_multiple_scopes_without_calling_host() {
        let (state, credential) = credential(Vec::new());

        let error = credential
            .get_token(&["scope-a", "scope-b"], None)
            .await
            .unwrap_err();

        assert!(error.to_string().contains("exactly one scope"));
        assert_eq!(state.calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn frees_host_state_after_final_credential_drop() {
        let expires = OffsetDateTime::now_utc()
            .saturating_add(Duration::minutes(10))
            .unix_timestamp();
        let (state, credential) = credential(vec![("token-a".to_string(), expires)]);

        assert_eq!(state.frees.load(Ordering::SeqCst), 0);
        drop(credential);
        assert_eq!(state.frees.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn rejects_missing_provider_callback() {
        let credential = create_token_credential(
            CosmosTokenProvider {
                get_token: None,
                user_data_free: None,
            },
            0,
        );
        assert!(credential.is_none());
    }
}
