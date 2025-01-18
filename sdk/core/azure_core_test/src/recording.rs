// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The [`Recording`] and other types used in recorded tests.

use crate::{
    proxy::{policy::RecordingPolicy, Proxy},
    Matcher, Sanitizer,
};
use azure_core::{test::TestMode, ClientOptions};
use std::sync::Arc;
use tracing::Span;

/// Represents a playback or recording session using the [`Proxy`].
#[derive(Debug)]
pub struct Recording {
    test_mode: TestMode,
    // Keep the span open for our lifetime.
    _span: Span,
    _proxy: Option<Arc<Proxy>>,
}

impl Recording {
    pub(crate) fn new(test_mode: TestMode, span: Span, proxy: Option<Arc<Proxy>>) -> Self {
        Self {
            test_mode,
            _span: span,
            _proxy: proxy,
        }
    }

    /// Adds a [`Sanitizer`] to sanitize PII for the current test.
    pub fn add_sanitizer<S: Sanitizer>(&self, _sanitizer: S) -> azure_core::Result<()> {
        todo!()
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
        options.per_try_policies.push(Arc::new(RecordingPolicy {
            test_mode: self.test_mode,
        }));
    }

    /// Sets a [`Matcher`] to compare requests and/or responses.
    pub fn set_matcher(&self, _matcher: Matcher) -> azure_core::Result<()> {
        todo!()
    }

    /// Skip recording the request body, or the entire request and response until the [`SkipGuard`] is dropped.
    pub fn skip(&self, _skip: Skip) -> SkipGuard<'_> {
        // TODO: Tell transport to start sending `x-recording-skip` header with `skip.value()`.
        let _header_value = _skip.value();
        SkipGuard(self)
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

/// What to skip when recording to a file.
#[derive(Debug)]
pub enum Skip {
    /// Skip recording only the request body.
    RequestBody,

    /// Skip recording both the request and response entirely.
    RequestResponse,
}

impl Skip {
    #[allow(dead_code)]
    fn value(&self) -> &'static str {
        match self {
            Self::RequestBody => "request-body",
            Self::RequestResponse => "request-response",
        }
    }
}

/// When the `SkipGuard` is dropped, recording requests and responses will begin again.
///
/// Returned from [`Recording::skip()`].
pub struct SkipGuard<'a>(&'a Recording);

impl Drop for SkipGuard<'_> {
    fn drop(&mut self) {
        if self.0.test_mode == TestMode::Record {
            // TODO: Tell transport to stop sending `x-recording-skip` header.
            todo!()
        }
    }
}
