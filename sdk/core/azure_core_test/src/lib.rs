// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

pub mod credentials;
#[cfg(doctest)]
mod docs;
pub mod http;
pub mod perf;
pub mod proxy;
pub mod recorded;
mod recording;
#[cfg(doctest)]
mod root_readme;
pub mod stream;
pub mod tracing;
use azure_core::Error;
pub use azure_core::{error::ErrorKind, test::TestMode};
pub use proxy::policy::RecordingOptions;
pub use proxy::{matchers::*, sanitizers::*};
pub use recording::*;
use std::path::{Path, PathBuf};

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
const ASSETS_FILE: &str = "assets.json";

/// Context information required by recorded client library tests.
///
/// This context is required for any recorded tests not attributed as `#[recorded::test(live)]`
/// to setup up the HTTP client to record or play back session records.
#[derive(Debug)]
pub struct TestContext {
    repo_dir: &'static Path,
    crate_dir: &'static Path,
    service_dir: &'static str,
    module_name: &'static str,
    name: &'static str,
    recording: Option<Recording>,
}

impl TestContext {
    pub(crate) fn new(
        crate_dir: &'static str,
        module_dir: &'static str,
        name: &'static str,
    ) -> azure_core::Result<Self> {
        let service_dir = parent_of(crate_dir, "sdk").ok_or_else(|| {
            Error::with_message(ErrorKind::Other, "not under 'sdk' folder in repo")
        })?;
        let test_module = Path::new(module_dir)
            .file_stem()
            .ok_or_else(|| Error::with_message(ErrorKind::Other, "invalid test module"))?
            .to_str()
            .ok_or_else(|| Error::with_message(ErrorKind::Other, "invalid test module"))?;
        Ok(Self {
            repo_dir: find_ancestor_of(crate_dir, ".git")?,
            crate_dir: Path::new(crate_dir),
            service_dir,
            module_name: test_module,
            name,
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

    /// Gets the repository root.
    pub fn repo_dir(&self) -> &'static Path {
        self.repo_dir
    }

    /// Gets the service directory containing the current test.
    ///
    /// This is the directory under `sdk/` within the repository e.g., "core" in `sdk/core`.
    pub fn service_dir(&self) -> &'static str {
        self.service_dir
    }

    /// Gets the test data directory under [`Self::crate_dir`].
    ///
    /// The path is relative to the repository root e.g., `sdk/core/azure_core/tests/data`.
    ///
    /// # Panics
    ///
    /// Panics if the [`TestContext::crate_dir()`] is not rooted within a Git repository.
    pub fn test_data_dir(&self) -> PathBuf {
        self.crate_dir
            .join("tests/data")
            .strip_prefix(self.repo_dir)
            .expect("not rooted within repo")
            .to_path_buf()
    }

    /// Gets the module name containing the current test.
    pub fn module_name(&self) -> &'static str {
        self.module_name
    }

    /// Gets the current test function name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Gets the recording assets file under the crate directory.
    ///
    /// The path is relative to the repository root e.g., `sdk/core/assets.json`.
    ///
    /// # Panics
    ///
    /// Panics if the [`TestContext::crate_dir()`] is not rooted within a Git repository.
    pub(crate) fn test_recording_assets_file(
        &self,
        #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] mode: TestMode,
    ) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            None
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if mode == TestMode::Live {
                return None;
            }
            let path = match find_ancestor_file(self.crate_dir, ASSETS_FILE) {
                Ok(path) => path,
                Err(_) if mode == TestMode::Record => {
                    return Path::new("sdk")
                        .join(self.service_dir)
                        .join(ASSETS_FILE)
                        .as_path()
                        .to_str()
                        .map(String::from);
                }
                Err(err) => panic!("{err}"),
            };
            path.strip_prefix(self.repo_dir)
                .expect("not rooted within repo")
                .to_str()
                .map(String::from)
        }
    }

    /// Gets the recording file of the current test.
    pub(crate) fn test_recording_file(&self) -> String {
        let path = self
            .test_data_dir()
            .join(self.module_name)
            .join(self.name)
            .as_path()
            .with_extension("json");
        path.to_str()
            .map(String::from)
            .unwrap_or_else(|| panic!("{path:?} is invalid"))
    }
}

/// Imports the contents of a `.env` file if it exists.
///
/// This function searches for a `.env` file starting from the current crate directory and going up to the repository root.
/// It loads the environment variables defined in that file, which can be useful for tests that require specific configurations or secrets.
///
/// Note that if no `.env` file is found, this function does nothing and returns `Ok(())`.
///
/// # Arguments
///
/// * `cargo_dir` - The directory of the Cargo package, typically the value of the `CARGO_MANIFEST_DIR` environment variable.
pub fn load_dotenv_file(cargo_dir: impl AsRef<Path>) -> azure_core::Result<()> {
    if let Ok(path) = find_ancestor_file(cargo_dir, ".env") {
        ::tracing::debug!("loading environment variables from {}", path.display());

        use azure_core::error::ResultExt as _;
        dotenvy::from_filename(&path).with_context_fn(azure_core::error::ErrorKind::Io, || {
            format!(
                "failed to load environment variables from {}",
                path.display()
            )
        })?;
    }
    Ok(())
}

/// Finds `name` under `dir` and returns the path to the parent `dir`.
///
/// This function does *not* check the file system.
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

/// Finds `name` under `dir` and returns the path to the named entry.
///
/// This function does check the file system.
fn find_ancestor_file(dir: impl AsRef<Path>, name: &str) -> azure_core::Result<PathBuf> {
    for dir in dir.as_ref().ancestors() {
        let path = dir.join(name);
        if path.exists() {
            return Ok(path);
        }

        // Keep looking until we get to the repo root where `.git` is either a directory (primary repo) or file (worktree).
        let path = dir.join(".git");
        if path.exists() {
            return Err(azure_core::Error::with_message(
                ErrorKind::Io,
                format!("{name} not found under repo {}", dir.display()),
            ));
        }
    }
    Err(azure_core::Error::new::<std::io::Error>(
        azure_core::error::ErrorKind::Io,
        std::io::ErrorKind::NotFound.into(),
    ))
}

/// Finds `name` under `dir` and returns the path to the parent `dir`.
///
/// This function does check the file system.
fn find_ancestor_of(dir: &'static str, name: &'static str) -> azure_core::Result<&'static Path> {
    let dir = Path::new(dir);
    for dir in dir.ancestors() {
        let path = dir.join(name);
        if path.exists() {
            return Ok(dir);
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
    fn test_context_new() {
        let ctx =
            TestContext::new(env!("CARGO_MANIFEST_DIR"), file!(), "test_context_new").unwrap();
        assert!(ctx.recording.is_none());
        assert!(ctx
            .crate_dir()
            .to_str()
            .unwrap()
            .replace("\\", "/")
            .ends_with("sdk/core/azure_core_test"));
        assert_eq!(ctx.module_name(), "lib");
        assert_eq!(ctx.name(), "test_context_new");
        assert_eq!(
            ctx.test_recording_file().replace("\\", "/"),
            "sdk/core/azure_core_test/tests/data/lib/test_context_new.json"
        );
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
