// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live recording and playing back of client library tests.
#[cfg(not(target_arch = "wasm32"))]
use crate::proxy::Proxy;
use crate::{proxy::ProxyOptions, recording::Recording, TestContext};
use azure_core::{test::TestMode, Result};
pub use azure_core_test_macros::test;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::OnceCell;
use tracing::debug_span;

#[cfg(not(target_arch = "wasm32"))]
static TEST_PROXY: OnceCell<Result<Arc<Proxy>>> = OnceCell::const_new();

/// Starts playback or recording of live recordings.
///
/// The [Test Proxy](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md) service will be started as needed.
/// Every `#[recorded::test]` will call this automatically, but it can also be called manually by any other test e.g., those attributed with `#[tokio::test]`.
pub async fn start(
    ctx: &mut TestContext,
    test_mode: TestMode,
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] options: Option<ProxyOptions>,
) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    let proxy = None;

    #[cfg(not(target_arch = "wasm32"))]
    let proxy = match test_mode {
        TestMode::Live => None,
        _ => Some(
            TEST_PROXY
                .get_or_init(|| async {
                    crate::proxy::start(ctx.test_data_dir(), options)
                        .await
                        .map(Arc::new)
                })
                .await
                .as_ref()
                .map(|proxy| proxy.clone())
                .map_err(|err| azure_core::Error::new(err.kind().clone(), err))?,
        ),
    };

    let span = debug_span!(target: crate::SPAN_TARGET, "recording", mode = ?test_mode, test = ?ctx.test_name());
    ctx.recording = Some(Recording::new(test_mode, span, proxy));

    Ok(())
}
