// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live recording and playing back of client library tests.
use crate::{
    proxy::{self, Proxy, ProxyOptions},
    recording::Recording,
    TestContext,
};
use azure_core::Result;
pub use azure_core_test_macros::test;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::debug_span;

static TEST_PROXY: OnceCell<Result<Arc<Proxy>>> = OnceCell::const_new();

/// Starts playback or recording of live recordings.
///
/// The [Test Proxy](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md) service will be started as needed.
/// Every `#[recorded::test]` will call this automatically, but it can also be called manually by any other test e.g., those attributed with `#[tokio::test]`.
///
/// This function will return `Ok(None)` if the test is running in live mode and should not use the test proxy at all.
pub async fn start(ctx: &TestContext, options: Option<ProxyOptions>) -> Result<Option<Recording>> {
    // Live tests don't use test-proxy.
    if ctx.test_mode() == azure_core::test::TestMode::Live {
        return Ok(None);
    }

    let proxy = TEST_PROXY
        .get_or_init(|| async move {
            proxy::start(ctx.test_data_dir(), options)
                .await
                .map(Arc::new)
        })
        .await
        .as_ref()
        .map_err(|err| azure_core::Error::new(err.kind().clone(), err))?;

    let span = debug_span!(target: crate::SPAN_TARGET, "recording", mode = ?ctx.test_mode(), test = ?ctx.test_name());
    Ok(Some(Recording {
        proxy: proxy.clone(),
        span,
        mode: ctx.test_mode(),
    }))
}
