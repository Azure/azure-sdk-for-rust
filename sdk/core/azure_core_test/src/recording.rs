// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The [`Recording`] and other types used in recorded tests.

use crate::{
    proxy::{
        client::{Client, ClientAddSanitizerOptions, ClientSetMatcherOptions, RecordingId},
        policy::RecordingPolicy,
        Proxy,
    },
    Matcher, Sanitizer,
};
use azure_core::{
    error::ErrorKind,
    headers::{AsHeaders, HeaderName, HeaderValue},
    test::TestMode,
    ClientOptions, Header,
};
use std::{cell::OnceCell, sync::Arc};
use tracing::Span;

/// Represents a playback or recording session using the [`Proxy`].
#[derive(Debug)]
pub struct Recording {
    test_mode: TestMode,
    // Keep the span open for our lifetime.
    _span: Span,
    _proxy: Option<Arc<Proxy>>,
    client: Option<Client>,
    policy: OnceCell<Arc<RecordingPolicy>>,
    id: Option<RecordingId>,
}

impl Recording {
    /// Adds a [`Sanitizer`] to sanitize PII for the current test.
    pub async fn add_sanitizer<S>(&self, _sanitizer: S) -> azure_core::Result<()>
    where
        S: Sanitizer,
        azure_core::Error: From<<S as AsHeaders>::Error>,
    {
        let Some(client) = &self.client else {
            return Ok(());
        };

        let options = ClientAddSanitizerOptions {
            recording_id: self.id.as_ref(),
            ..Default::default()
        };
        client.add_sanitizer(_sanitizer, Some(options)).await
    }

    /// Instruments the [`ClientOptions`] to support recording and playing back of session records.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_core_test::{recorded, TestContext};
    ///
    /// # struct MyClient;
    /// # #[derive(Default)]
    /// # struct MyClientOptions { client_options: azure_core::ClientOptions };
    /// # impl MyClient {
    /// #   fn new(endpoint: impl AsRef<str>, options: Option<MyClientOptions>) -> Self { todo!() }
    /// #   async fn invoke(&self) -> azure_core::Result<()> { todo!() }
    /// # }
    /// #[recorded::test]
    /// async fn test_invoke(ctx: TestContext) -> azure_core::Result<()> {
    ///     let recording = ctx.recording();
    ///
    ///     let mut options = MyClientOptions::default();
    ///     ctx.instrument(&mut options.client_options);
    ///
    ///     let client = MyClient::new("https://azure.net", Some(options));
    ///     client.invoke().await
    /// }
    /// ```
    pub fn instrument(&self, options: &mut ClientOptions) {
        if self.client.is_none() {
            return;
        }

        let policy = self
            .policy
            .get_or_init(|| {
                Arc::new(RecordingPolicy {
                    test_mode: self.test_mode,
                    ..Default::default()
                })
            })
            .clone();

        options.per_try_policies.push(policy);
    }

    /// Sets a [`Matcher`] to compare requests and/or responses.
    pub async fn set_matcher(&self, _matcher: Matcher) -> azure_core::Result<()> {
        let Some(client) = &self.client else {
            return Ok(());
        };

        let options = ClientSetMatcherOptions {
            recording_id: self.id.as_ref(),
            ..Default::default()
        };
        client.set_matcher(_matcher, Some(options)).await
    }

    /// Skip recording the request body, or the entire request and response until the [`SkipGuard`] is dropped.
    ///
    /// This only affects [`TestMode::Record`] mode and is intended for cleanup.
    /// When [`Recording::test_mode()`] is [`TestMode::Playback`] you should avoid sending those requests.
    pub fn skip(&mut self, skip: Skip) -> azure_core::Result<SkipGuard<'_>> {
        self.set_skip(Some(skip))?;
        Ok(SkipGuard(self))
    }

    /// Gets the current [`TestMode`].
    pub fn test_mode(&self) -> TestMode {
        self.test_mode
    }

    /// Gets the named variable from the environment or recording.
    pub fn var(&self, name: impl AsRef<str>) -> Option<String> {
        if self.test_mode == TestMode::Live {
            return std::env::var(name.as_ref()).ok();
        }

        // TODO: attempt to get it from the recording or fallthrough to the environment; or, do we need separate calls like .NET to fallthrough?
        todo!()
    }
}

impl Recording {
    pub(crate) fn new(
        test_mode: TestMode,
        span: Span,
        proxy: Option<Arc<Proxy>>,
        client: Option<Client>,
    ) -> Self {
        Self {
            test_mode,
            _span: span,
            _proxy: proxy,
            client,
            policy: OnceCell::new(),
            id: None,
        }
    }

    fn set_skip(&mut self, skip: Option<Skip>) -> azure_core::Result<()> {
        let Some(policy) = self.policy.get_mut() else {
            return Ok(());
        };

        let mut options = policy
            .options
            .write()
            .map_err(|err| azure_core::Error::message(ErrorKind::Other, err.to_string()))?;
        options.skip = skip;

        Ok(())
    }
}

/// What to skip when recording to a file.
///
/// This only affects [`TestMode::Record`] mode and is intended for cleanup.
/// When [`Recording::test_mode()`] is [`TestMode::Playback`] you should avoid sending those requests.
#[derive(Debug)]
pub enum Skip {
    /// Skip recording only the request body.
    RequestBody,

    /// Skip recording both the request and response entirely.
    RequestResponse,
}

impl Header for Skip {
    fn name(&self) -> HeaderName {
        HeaderName::from_static("x-recording-skip")
    }

    fn value(&self) -> HeaderValue {
        match self {
            Self::RequestBody => HeaderValue::from_static("request-body"),
            Self::RequestResponse => HeaderValue::from_static("request-response"),
        }
    }
}

/// When the `SkipGuard` is dropped, recording requests and responses will begin again.
///
/// Returned from [`Recording::skip()`].
pub struct SkipGuard<'a>(&'a mut Recording);

impl Drop for SkipGuard<'_> {
    fn drop(&mut self) {
        if self.0.test_mode == TestMode::Record {
            let _ = self.0.set_skip(None);
        }
    }
}
