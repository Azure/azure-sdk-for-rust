// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Host-provided token credential support for the native ABI.

use std::{ffi::c_void, fmt, sync::Arc};

use async_trait::async_trait;
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::ErrorKind,
    time::{Duration, OffsetDateTime},
    Error,
};
use tokio::sync::{oneshot, Mutex};

const TOKEN_REFRESH_SKEW: Duration = Duration::minutes(2);

/// Completes an asynchronous host token request.
///
/// The host must invoke this callback exactly once after its token-provider
/// callback returns success. Token and error buffers are borrowed only for the
/// duration of this call; Rust copies them before returning.
pub type CosmosTokenCompletion = unsafe extern "C" fn(
    completion_context: *mut c_void,
    status: i32,
    token: *const u8,
    token_len: usize,
    expires_on_unix_seconds: i64,
    error_message: *const u8,
    error_message_len: usize,
);

/// Starts asynchronous token acquisition in the host.
///
/// Returning zero transfers ownership of `request.completion_context` to the
/// host, which must eventually invoke `request.completion`. Returning nonzero
/// means the request was rejected synchronously and the completion must not be
/// invoked.
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
    /// UTF-8 token scope bytes.
    pub scope: *const u8,
    /// Number of bytes addressable from `scope`.
    pub scope_len: usize,
    /// Rust completion callback the host invokes exactly once.
    pub completion: CosmosTokenCompletion,
    /// Opaque Rust-owned context passed unchanged to `completion`.
    pub completion_context: *mut c_void,
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

struct PendingTokenRequest {
    sender: oneshot::Sender<azure_core::Result<AccessToken>>,
}

struct CallbackTokenCredential {
    provider: CosmosTokenProvider,
    user_data: isize,
    cached_token: Mutex<Option<AccessToken>>,
}

impl CallbackTokenCredential {
    fn new(provider: CosmosTokenProvider, user_data: isize) -> Option<Arc<dyn TokenCredential>> {
        provider.get_token?;
        Some(Arc::new(Self {
            provider,
            user_data,
            cached_token: Mutex::new(None),
        }))
    }

    async fn request_token(&self, scope: &str) -> azure_core::Result<AccessToken> {
        let (sender, receiver) = oneshot::channel();
        let completion_context =
            Box::into_raw(Box::new(PendingTokenRequest { sender })).cast::<c_void>();
        let status = {
            let request = CosmosTokenRequest {
                scope: scope.as_ptr(),
                scope_len: scope.len(),
                completion: complete_token_request,
                completion_context,
            };
            // SAFETY: the provider callback is supplied by the host under the C
            // ABI contract. The request remains valid for the duration of the call.
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
            // The callback rejected the request synchronously, so ownership of
            // the completion context did not transfer to the host.
            // SAFETY: this pointer was allocated immediately above and the
            // nonzero-return contract forbids the host from completing it.
            unsafe {
                drop(Box::from_raw(
                    completion_context.cast::<PendingTokenRequest>(),
                ));
            }
            return Err(credential_error(format!(
                "host token provider rejected the request with status {status}"
            )));
        }

        receiver.await.map_err(|_| {
            credential_error("host token provider dropped the token request without completing it")
        })?
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

        // Hold the lock through refresh so concurrent requests single-flight
        // token acquisition instead of invoking the host callback in parallel.
        let mut cached = self.cached_token.lock().await;
        let now = OffsetDateTime::now_utc();
        if let Some(token) = cached.as_ref() {
            if token.expires_on > now.saturating_add(TOKEN_REFRESH_SKEW) {
                return Ok(token.clone());
            }
        }

        let token = self.request_token(scope).await?;
        if token.expires_on <= OffsetDateTime::now_utc() {
            return Err(credential_error(
                "host token provider returned an expired access token",
            ));
        }
        *cached = Some(token.clone());
        Ok(token)
    }
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

unsafe extern "C" fn complete_token_request(
    completion_context: *mut c_void,
    status: i32,
    token: *const u8,
    token_len: usize,
    expires_on_unix_seconds: i64,
    error_message: *const u8,
    error_message_len: usize,
) {
    if completion_context.is_null() {
        return;
    }

    // SAFETY: ownership was transferred to the host only after the provider
    // callback returned success. The host contract requires exactly one call.
    let pending = unsafe { Box::from_raw(completion_context.cast::<PendingTokenRequest>()) };
    let result = build_access_token(
        status,
        token,
        token_len,
        expires_on_unix_seconds,
        error_message,
        error_message_len,
    );
    let _ = pending.sender.send(result);
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
        let message = copy_host_bytes(error_message, error_message_len)
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .unwrap_or_else(|_| "host token provider failed without an error message".to_string());
        return Err(credential_error(message));
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
        // call returns, and the completion copies them.
        unsafe {
            (request.completion)(
                request.completion_context,
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
        let completion = request.completion;
        let completion_context = request.completion_context as usize;
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(20));
            // SAFETY: the provider returned success, transferring this context
            // to the host. Token bytes remain live until completion returns.
            unsafe {
                completion(
                    completion_context as *mut c_void,
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

    unsafe extern "C" fn reject_token(
        _user_data: isize,
        _request: *const CosmosTokenRequest,
    ) -> i32 {
        17
    }

    unsafe extern "C" fn fail_token(_user_data: isize, request: *const CosmosTokenRequest) -> i32 {
        const MESSAGE: &[u8] = b"credential unavailable";
        // SAFETY: the native adapter passes a valid request for this call.
        let request = unsafe { &*request };
        // SAFETY: the message remains live for the duration of completion.
        unsafe {
            (request.completion)(
                request.completion_context,
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

    #[tokio::test]
    async fn caches_token_until_refresh_window() {
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

    #[tokio::test]
    async fn refreshes_token_near_expiry() {
        let now = OffsetDateTime::now_utc();
        let (state, credential) = credential(vec![
            (
                "token-a".to_string(),
                now.saturating_add(Duration::seconds(30)).unix_timestamp(),
            ),
            (
                "token-b".to_string(),
                now.saturating_add(Duration::minutes(10)).unix_timestamp(),
            ),
        ]);

        let first = credential.get_token(&["scope"], None).await.unwrap();
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
    async fn propagates_synchronous_provider_rejection() {
        let (_state, credential) = credential_with_callback(Vec::new(), reject_token);

        let error = credential.get_token(&["scope"], None).await.unwrap_err();

        assert!(error.to_string().contains("status 17"));
    }

    #[tokio::test]
    async fn propagates_host_completion_error() {
        let (_state, credential) = credential_with_callback(Vec::new(), fail_token);

        let error = credential.get_token(&["scope"], None).await.unwrap_err();

        assert!(error.to_string().contains("credential unavailable"));
    }

    #[tokio::test]
    async fn rejects_expired_host_token() {
        let expires = OffsetDateTime::now_utc()
            .saturating_sub(Duration::seconds(1))
            .unix_timestamp();
        let (_state, credential) = credential(vec![("expired".to_string(), expires)]);

        let error = credential.get_token(&["scope"], None).await.unwrap_err();

        assert!(error.to_string().contains("expired access token"));
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
