// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recorded integration tests for session token authentication.
//!
//! These tests require a storage account that supports the session feature.
//!
//! Note: the account used to record must accept the client's default
//! `x-ms-version`. If it does not, pin an older api-version on the options.

mod common;

use async_trait::async_trait;
use azure_core::http::{
    headers::AUTHORIZATION,
    policies::{Policy, PolicyResult},
    Context, Method, Request, RequestContent, Url,
};
use azure_core_test::{recorded, BodyRegexSanitizer, Recording, TestContext};
use azure_storage_blob::{
    models::{AuthenticationType, BlockListType, CreateSessionConfiguration},
    BlobServiceClient, BlobServiceClientOptions, SessionMode, SessionOptions,
};
use common::{ClientOptionsExt, StorageAccount};
use std::{
    error::Error,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

/// Counts how eligible and ineligible requests were authenticated. A single
/// instance is shared across the client and its session provider, so it also
/// observes Create Session requests.
#[derive(Debug, Default)]
struct SessionAuthCounts {
    create_session: AtomicUsize,
    session_get: AtomicUsize,
    bearer_get: AtomicUsize,
    non_get_session: AtomicUsize,
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

        next[0].send(ctx, request, &next[1..]).await
    }
}

/// Redacts the session token and key from Create Session response bodies so
/// live credentials are never written to recordings. The replacement is valid
/// base64 so playback can still sign requests using the recorded body.
async fn redact_session_credentials(recording: &Recording) -> azure_core::Result<()> {
    // Base64 of "REDACTED": a valid key so playback signing still succeeds.
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
    let session_options = SessionOptions {
        mode,
        account_name: Some(account_name),
    };
    BlobServiceClient::new_with_session_options(
        Url::parse(&endpoint)?,
        Some(recording.credential()),
        session_options,
        Some(options),
    )
}

#[recorded::test]
async fn create_session_returns_credentials(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    redact_session_credentials(recording).await?;
    let container_client =
        common::get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let config = CreateSessionConfiguration {
        authentication_type: Some(AuthenticationType::Hmac),
    };
    let session = container_client
        .create_session(config.try_into()?, None)
        .await?
        .into_model()?;

    assert_eq!(session.authentication_type, Some(AuthenticationType::Hmac));
    let credentials = session.credentials.expect("session credentials");
    assert!(credentials.session_token.is_some());
    assert!(credentials.session_key.is_some());
    assert!(session.expiration.is_some());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
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

    assert!(
        counts.create_session.load(Ordering::SeqCst) >= 1,
        "expected a CreateSession call"
    );
    assert!(
        counts.session_get.load(Ordering::SeqCst) >= 1,
        "download should use session authentication"
    );
    assert_eq!(
        counts.non_get_session.load(Ordering::SeqCst),
        0,
        "non-GET requests must not use session authentication"
    );

    container.delete(None).await?;
    Ok(())
}

#[recorded::test]
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
        counts.create_session.load(Ordering::SeqCst),
        0,
        "comp operations should not acquire a session"
    );
    assert!(
        counts.bearer_get.load(Ordering::SeqCst) >= 1,
        "comp GET should use bearer authentication"
    );
    assert_eq!(
        counts.session_get.load(Ordering::SeqCst),
        0,
        "comp GET must not use session authentication"
    );

    container.delete(None).await?;
    Ok(())
}
