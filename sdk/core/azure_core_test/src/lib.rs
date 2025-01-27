// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

pub mod proxy;
pub mod recorded;
mod recording;

pub use azure_core::test::TestMode;
pub use proxy::{matchers::*, sanitizers::*};
pub use recording::*;
use std::path::{Path, PathBuf};

const SPAN_TARGET: &str = "test-proxy";

/// Context information required by recorded client library tests.
///
/// This context is required for any recorded tests not attributed as `#[recorded::test(live)]`
/// to setup up the HTTP client to record or play back session records.
#[derive(Debug)]
pub struct TestContext {
    crate_dir: &'static Path,
    test_name: &'static str,
    recording: Option<Recording>,
}

impl TestContext {
    /// Not intended for use outside the `azure_core` crates.
    #[doc(hidden)]
    pub fn new(crate_dir: &'static str, test_name: &'static str) -> Self {
        Self {
            crate_dir: Path::new(crate_dir),
            test_name,
            recording: None,
        }
    }

    /// Gets the root directory of the crate under test.
    pub fn crate_dir(&self) -> &'static Path {
        self.crate_dir
    }

    /// Gets the [`Recording`].
    ///
    /// # Panics
    ///
    /// Panics if a recording or playback has not been started.
    pub fn recording(&self) -> &Recording {
        self.recording
            .as_ref()
            .expect("not recording or playback started")
    }

    /// Gets the test data directory under [`Self::crate_dir`].
    pub fn test_data_dir(&self) -> PathBuf {
        self.crate_dir.join("tests/data")
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
        let ctx = TestContext::new(env!("CARGO_MANIFEST_DIR"), "test_content_new");
        assert!(ctx.recording.is_none());
        assert!(ctx
            .crate_dir()
            .to_str()
            .unwrap()
            .replace("\\", "/")
            .ends_with("sdk/core/azure_core_test"));
        assert_eq!(ctx.test_name(), "test_content_new");
    }
}
