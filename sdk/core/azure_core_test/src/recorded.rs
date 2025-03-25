// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live recording and playing back of client library tests.
use crate::{
    proxy::{client::Client, Proxy, ProxyOptions},
    recording::Recording,
    TestContext,
};
use azure_core::{test::TestMode, Result};
pub use azure_core_test_macros::test;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::OnceCell;

#[cfg(not(target_arch = "wasm32"))]
static ONLY_TRACE: std::sync::OnceLock<()> = std::sync::OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
static TEST_PROXY: OnceCell<Result<Arc<Proxy>>> = OnceCell::const_new();

/// Starts playback or recording of live recordings.
///
/// The [Test Proxy](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md) service will be started as needed.
/// Every `#[recorded::test]` will call this automatically, but it can also be called manually by any other test e.g., those attributed with `#[tokio::test]`.
#[tracing::instrument(level = "debug", err)]
pub async fn start(
    mode: TestMode,
    crate_dir: &'static str,
    module_dir: &'static str,
    name: &'static str,
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] options: Option<ProxyOptions>,
) -> Result<TestContext> {
    let mut ctx = TestContext::new(crate_dir, module_dir, name)?;

    #[cfg(target_arch = "wasm32")]
    let proxy: Option<Arc<Proxy>> = None;

    #[cfg(not(target_arch = "wasm32"))]
    let proxy = {
        match mode {
            TestMode::Live => {
                ONLY_TRACE.get_or_init(init_tracing);
                None
            }
            _ => Some(
                TEST_PROXY
                    .get_or_init(|| async move {
                        init_tracing();
                        crate::proxy::start(Some(mode), crate_dir, options)
                            .await
                            .map(Arc::new)
                    })
                    .await
                    .as_ref()
                    .map(Clone::clone)
                    .map_err(|err| azure_core::Error::new(err.kind().clone(), err))?,
            ),
        }
    };

    // TODO: Could we cache the client? Hypothetically, this function should only run once per `tests/*` file so it should be practical.
    let mut client = None;
    if let Some(proxy) = proxy.as_ref() {
        client = Some(Client::new(proxy.endpoint().clone())?);
    }

    let span = tracing::debug_span!("recording", ?mode, name);
    let mut recording = Recording::new(
        mode,
        span.entered(),
        proxy.clone(),
        client,
        ctx.service_dir(),
        ctx.test_recording_file(),
        ctx.test_recording_assets_file(mode),
    );
    recording.start().await?;

    ctx.recording = Some(recording);
    Ok(ctx)
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn init_tracing() {
    #[cfg(feature = "tracing")]
    {
        use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_ansi(std::env::var("NO_COLOR").map_or(true, |v| v.is_empty()))
            .with_writer(std::io::stderr)
            .init();
    }
}
