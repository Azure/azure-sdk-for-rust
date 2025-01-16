// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The [`Recording`] and other types used in recorded tests.

use crate::proxy::Proxy;
use azure_core::test::TestMode;
use std::sync::Arc;
use tracing::Span;

/// Represents a playback or recording session using the [`Proxy`].
pub struct Recording {
    #[allow(dead_code)]
    pub(crate) proxy: Arc<Proxy>,
    #[allow(dead_code)]
    pub(crate) span: Span,
    pub(crate) mode: TestMode,
}

impl Recording {
    pub fn skip(&self, _skip: Skip) -> SkipGuard<'_> {
        // Tell transport to start sending `x-recording-skip` header with `skip.value()`.
        let _header_value = _skip.value();
        SkipGuard(self)
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
pub struct SkipGuard<'a>(&'a Recording);

impl Drop for SkipGuard<'_> {
    fn drop(&mut self) {
        if self.0.mode == TestMode::Record {
            // TODO: Tell transport to stop sending `x-recording-skip` header.
            todo!()
        }
    }
}
