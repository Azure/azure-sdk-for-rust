// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live recording and playing back of client library tests.
use crate::{
    proxy::{self, Proxy, ProxyOptions, Session},
    TestContext,
};
use azure_core::Result;
pub use azure_core_test_macros::test;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::debug_span;

static TEST_PROXY: OnceCell<Result<Arc<Proxy>>> = OnceCell::const_new();

/// Starts playback or recording of live sessions.
///
/// The [Test Proxy](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md) service will be started as needed.
/// Every `#[recorded::test]` will call this automatically, but it can also be called manually by any other test e.g., those attributed with `#[tokio::test]`.
pub async fn start(ctx: &TestContext, options: Option<ProxyOptions>) -> Result<Session> {
    let proxy = TEST_PROXY
        .get_or_init(|| async move {
            proxy::start(ctx.test_data_dir(), options)
                .await
                .map(Arc::new)
        })
        .await
        .as_ref()
        .map_err(|err| azure_core::Error::new(err.kind().clone(), err))?;

    let span = debug_span!(target: crate::SPAN_TARGET, "session", mode = ?ctx.test_mode(), test = ?ctx.test_name());
    Ok(Session {
        proxy: proxy.clone(),
        span,
    })
}
