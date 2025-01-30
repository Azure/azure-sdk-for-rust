// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

pub mod proxy;
pub mod recorded;
mod recording;

use azure_core::Error;
pub use azure_core::{error::ErrorKind, test::TestMode};
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
    service_directory: &'static str,
    test_module: &'static str,
    test_name: &'static str,
    recording: Option<Recording>,
}

impl TestContext {
    pub(crate) fn new(
        crate_dir: &'static str,
        test_module: &'static str,
        test_name: &'static str,
    ) -> azure_core::Result<Self> {
        let service_directory = parent_of(crate_dir, "sdk")
            .ok_or_else(|| Error::message(ErrorKind::Other, "not under 'sdk' folder in repo"))?;
        let test_module = Path::new(test_module)
            .file_stem()
            .ok_or_else(|| Error::message(ErrorKind::Other, "invalid test module"))?
            .to_str()
            .ok_or_else(|| Error::message(ErrorKind::Other, "invalid test module"))?;
        Ok(Self {
            crate_dir: Path::new(crate_dir),
            service_directory,
            test_module,
            test_name,
            recording: None,
        })
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

    /// Gets the service directory containing the current test.
    ///
    /// This is the directory under `sdk/` within the repository e.g., "core" in `sdk/core`.
    pub fn service_directory(&self) -> &'static str {
        self.service_directory
    }

    /// Gets the test data directory under [`Self::crate_dir`].
    pub fn test_data_dir(&self) -> PathBuf {
        self.crate_dir.join("tests/data")
    }

    /// Gets the module name containing the current test.
    pub fn test_module(&self) -> &'static str {
        self.test_module
    }

    /// Gets the current test function name.
    pub fn test_name(&self) -> &'static str {
        self.test_name
    }

    /// Gets the recording assets file under the crate directory.
    pub(crate) fn test_recording_assets_file(&self) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            None
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let path =
                find_ancestor(self.crate_dir, "assets.json").unwrap_or_else(|err| panic!("{err}"));
            path.as_path().to_str().map(String::from)
        }
    }

    /// Gets the recording file of the current test.
    pub(crate) fn test_recording_file(&self) -> String {
        let path = self
            .test_data_dir()
            .join(self.test_module)
            .join(self.test_name)
            .as_path()
            .with_extension("json");
        path.to_str()
            .map(String::from)
            .unwrap_or_else(|| panic!("{path:?} is invalid"))
    }
}

fn parent_of<'a>(dir: &'a str, name: &'static str) -> Option<&'a str> {
    let mut child = None;

    let dir = Path::new(dir);
    let components = dir.components().rev();
    for dir in components {
        if dir.as_os_str() == name {
            return child;
        }
        child = dir.as_os_str().to_str();
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn find_ancestor(dir: impl AsRef<Path>, name: &str) -> azure_core::Result<PathBuf> {
    for dir in dir.as_ref().ancestors() {
        let path = dir.join(name);
        if path.exists() {
            return Ok(path);
        }

        // Keep looking until we get to the repo root where `.git` is either a directory (primary repo) or file (worktree).
        let path = dir.join(".git");
        if path.exists() {
            return Err(azure_core::Error::message(
                ErrorKind::Other,
                format!("{name} not found under repo {}", dir.display()),
            ));
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
        let ctx =
            TestContext::new(env!("CARGO_MANIFEST_DIR"), file!(), "test_content_new").unwrap();
        assert!(ctx.recording.is_none());
        assert!(ctx
            .crate_dir()
            .to_str()
            .unwrap()
            .replace("\\", "/")
            .ends_with("sdk/core/azure_core_test"));
        assert_eq!(ctx.test_module(), "lib");
        assert_eq!(ctx.test_name(), "test_content_new");
        assert!(ctx
            .test_recording_file()
            .as_str()
            .replace("\\", "/")
            .ends_with("sdk/core/azure_core_test/tests/data/lib/test_content_new.json"));
    }

    #[test]
    fn test_parent_of() {
        assert_eq!(
            parent_of("~/src/azure-sdk-for-rust/sdk/core", "sdk"),
            Some("core"),
        );
        assert!(parent_of("~/src/azure-sdk-for-rust/sdk/", "sdk").is_none());
        assert!(parent_of("~/src/azure-sdk-for-rust/sdk/core", "should_not_exist").is_none());
    }
}
