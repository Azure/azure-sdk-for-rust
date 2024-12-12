// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

#[cfg(not(target_arch = "wasm32"))]
pub mod proxy;
#[cfg(not(target_arch = "wasm32"))]
pub mod recorded;
mod sanitizers;
mod transport;

pub use azure_core::test::TestMode;
use azure_core::{ClientOptions, TransportOptions};
pub use sanitizers::*;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

const SPAN_TARGET: &str = "test-proxy";

/// Context information required by recorded client library tests.
///
/// This context is required for any recorded tests not attributed as `#[recorded::test(live)]`
/// to setup up the HTTP client to record or play back session records.
#[derive(Clone, Debug)]
pub struct TestContext {
    test_mode: TestMode,
    crate_dir: &'static Path,
    test_name: &'static str,
}

impl TestContext {
    /// Not intended for use outside the `azure_core` crates.
    #[doc(hidden)]
    pub fn new(test_mode: TestMode, crate_dir: &'static str, test_name: &'static str) -> Self {
        Self {
            test_mode,
            crate_dir: Path::new(crate_dir),
            test_name,
        }
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
    ///     let mut options = MyClientOptions::default();
    ///     ctx.instrument(&mut options.client_options);
    ///
    ///     let client = MyClient::new("https://azure.net", Some(options));
    ///     client.invoke().await
    /// }
    /// ```
    pub fn instrument(&self, options: &mut ClientOptions) {
        let transport = options.transport.clone().unwrap_or_default();
        options.transport = Some(TransportOptions::new_custom_policy(Arc::new(
            transport::ProxyTransportPolicy {
                inner: transport,
                mode: self.test_mode,
            },
        )));
    }

    /// Gets the root directory of the crate under test.
    pub fn crate_dir(&self) -> &'static Path {
        self.crate_dir
    }

    /// Gets the test data directory under [`Self::crate_dir`].
    pub fn test_data_dir(&self) -> PathBuf {
        self.crate_dir.join("tests/data")
    }

    /// Gets the current [`TestMode`].
    pub fn test_mode(&self) -> TestMode {
        self.test_mode
    }

    /// Gets the current test function name.
    pub fn test_name(&self) -> &'static str {
        self.test_name
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn find_ancestor(dir: impl AsRef<Path>, name: &str) -> azure_core::Result<PathBuf> {
    for dir in dir.as_ref().ancestors() {
        let path = dir.join(name);
        if path.exists() {
            return Ok(path);
        }
    }
    Err(azure_core::Error::new::<std::io::Error>(
        azure_core::error::ErrorKind::Io,
        std::io::ErrorKind::NotFound.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_new() {
        let ctx = TestContext::new(
            TestMode::default(),
            env!("CARGO_MANIFEST_DIR"),
            "test_content_new",
        );
        assert_eq!(ctx.test_mode(), TestMode::Playback);
        assert!(ctx
            .crate_dir()
            .to_str()
            .unwrap()
            .replace("\\", "/")
            .ends_with("sdk/core/azure_core_test"));
        assert_eq!(ctx.test_name(), "test_content_new");
    }
}
