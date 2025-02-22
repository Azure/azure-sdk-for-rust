// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Wrappers for the [Test Proxy](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md) service.
pub(crate) mod client;
pub(crate) mod matchers;
pub(crate) mod models;
pub(crate) mod policy;
pub(crate) mod sanitizers;

#[cfg(not(target_arch = "wasm32"))]
use azure_core::Result;
use azure_core::{
    error::ErrorKind,
    headers::{HeaderName, HeaderValue},
    Header, Url,
};
use serde::Serializer;
#[cfg(not(target_arch = "wasm32"))]
use std::process::ExitStatus;
use std::{fmt, str::FromStr};
#[cfg(not(target_arch = "wasm32"))]
use tokio::process::Child;
#[cfg(not(target_arch = "wasm32"))]
use tracing::Level;

const ABSTRACTION_IDENTIFIER: HeaderName = HeaderName::from_static("x-abstraction-identifier");
const RECORDING_ID: HeaderName = HeaderName::from_static("x-recording-id");
const RECORDING_MODE: HeaderName = HeaderName::from_static("x-recording-mode");
const RECORDING_UPSTREAM_BASE_URI: HeaderName =
    HeaderName::from_static("x-recording-upstream-base-uri");

#[cfg(not(target_arch = "wasm32"))]
pub use bootstrap::start;

#[cfg(not(target_arch = "wasm32"))]
mod bootstrap {
    use super::*;
    use azure_core::{test::TestMode, Result};
    use serde_json::json;
    use std::{env, io, path::Path, process::Stdio, time::Duration};
    use tokio::{
        fs,
        io::{AsyncBufReadExt, BufReader},
        process::{ChildStdout, Command},
    };

    // cspell:ignore aspnetcore devcert teamprojectid testproxy
    const KESTREL_CERT_PATH_ENV: &str = "ASPNETCORE_Kestrel__Certificates__Default__Path";
    const KESTREL_CERT_PASSWORD_ENV: &str = "ASPNETCORE_Kestrel__Certificates__Default__Password";
    const KESTREL_CERT_PASSWORD: &str = "password";
    const PROXY_MANUAL_START: &str = "PROXY_MANUAL_START";
    const SYSTEM_TEAMPROJECTID: &str = "SYSTEM_TEAMPROJECTID";
    const MIN_VERSION: Version = Version {
        major: 20241213,
        minor: 1,
        metadata: None,
    };

    /// Starts the test-proxy.
    ///
    /// This is intended for internal use only and should not be called directly in tests.
    pub async fn start(
        test_mode: Option<TestMode>,
        crate_dir: impl AsRef<Path>,
        options: Option<ProxyOptions>,
    ) -> Result<Proxy> {
        if env::var(PROXY_MANUAL_START).is_ok_and(|v| v.eq_ignore_ascii_case("true")) {
            tracing::event!(target: crate::SPAN_TARGET, Level::WARN, "environment variable {PROXY_MANUAL_START} is 'true'; not starting test proxy");
            return Ok(Proxy::default());
        }

        // Find root of git repo or work tree: a ".git" directory or file will exist either way.
        let git_dir = crate::find_ancestor_file(crate_dir.as_ref(), ".git")?;
        let git_dir = git_dir.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "parent git repository not found")
        })?;

        // Create an assets.json file in the crate_dir if a parent doesn't already exist.
        #[cfg(not(target_arch = "wasm32"))]
        if test_mode == Some(TestMode::Record)
            && matches!(crate::find_ancestor_file(crate_dir.as_ref(), "assets.json"), Err(err) if err.kind() == &ErrorKind::Io)
        {
            let assets_file = crate_dir.as_ref().join("assets.json");
            tracing::trace!("creating {path}", path = assets_file.display());

            let assets_dir = assets_file
                .parent()
                .and_then(Path::file_name)
                .map(|dir| dir.to_ascii_lowercase())
                .ok_or_else(|| {
                    azure_core::Error::message(
                        ErrorKind::Io,
                        "failed to get assets.json parent directory name",
                    )
                })?;
            let assets_dir = assets_dir.to_string_lossy();
            let assets_content = json!({
                "AssetsRepo": "Azure/azure-sdk-assets",
                "AssetsRepoPrefixPath": "rust",
                "TagPrefix": format!("rust/{assets_dir}"),
                "Tag": "",
            });
            let file = fs::File::create_new(assets_file).await?;
            serde_json::to_writer_pretty(file.into_std().await, &assets_content)?;
        }

        // Construct the command line arguments and start the test-proxy service.
        let mut args: Vec<String> = Vec::new();
        args.extend_from_slice(&[
            "start".into(),
            "--storage-location".into(),
            git_dir
                .to_str()
                .ok_or_else(|| ErrorKind::Other.into_error())?
                .into(),
        ]);
        options.unwrap_or_default().copy_to(&mut args);
        args.extend_from_slice(&["--", "--urls", "http://0.0.0.0:0"].map(Into::into));

        let mut command = Command::new("test-proxy")
            .args(args.iter())
            .env(
                KESTREL_CERT_PATH_ENV,
                git_dir.join("eng/common/testproxy/dotnet-devcert.pfx"),
            )
            .env(KESTREL_CERT_PASSWORD_ENV, KESTREL_CERT_PASSWORD)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| azure_core::Error::full(ErrorKind::Io, e, "test-proxy not found"))?;

        // Wait until the service is listening on a port.
        let mut stdout = command.stdout.take();
        let max_seconds = Duration::from_secs(env::var(SYSTEM_TEAMPROJECTID).map_or(5, |_| 20));
        let url = tokio::select! {
            v = wait_till_listening(&mut stdout) => { v? },
            _ = tokio::time::sleep(max_seconds) => {
                command.kill().await?;
                return Err(azure_core::Error::message(ErrorKind::Other, "timed out waiting for test-proxy to start"));
            },
        };

        Ok(Proxy {
            command: Some(command),
            url,
        })
    }

    async fn wait_till_listening(stdout: &mut Option<ChildStdout>) -> Result<Url> {
        let Some(stdout) = stdout else {
            return Err(azure_core::Error::message(
                ErrorKind::Io,
                "test-proxy stdout not captured",
            ));
        };

        // Wait for the process to respond to requests and check output until pattern: "Now listening on: http://0.0.0.0:60583" (random port).
        let mut reader = BufReader::new(stdout).lines();
        while let Some(line) = reader.next_line().await? {
            const RUNNING_PATTERN: &str = "Running proxy version is Azure.Sdk.Tools.TestProxy ";
            const LISTENING_PATTERN: &str = "Now listening on: ";

            if let Some(idx) = line.find(RUNNING_PATTERN) {
                let idx = idx + RUNNING_PATTERN.len();
                let version: Version = line[idx..].parse()?;
                tracing::event!(target: crate::SPAN_TARGET, Level::INFO, "starting test-proxy version {version}");

                // Need to check version since `test-proxy start` does not fail with unknown parameters.
                if version < MIN_VERSION {
                    return Err(azure_core::Error::message(
                        ErrorKind::Io,
                        format!("test-proxy older than required version {MIN_VERSION}"),
                    ));
                }

                continue;
            }

            if let Some(idx) = line.find(LISTENING_PATTERN) {
                let idx = idx + LISTENING_PATTERN.len();
                let mut url: Url = line[idx..].parse()?;
                url.set_host(Some("localhost"))?;
                tracing::event!(target: crate::SPAN_TARGET, Level::INFO, "listening on {url}");

                return Ok(url);
            }
        }

        Err(azure_core::Error::message(
            ErrorKind::Io,
            "timed out waiting for test-proxy to start",
        ))
    }
}

/// Represents the running `test-proxy` service.
#[derive(Debug)]
pub struct Proxy {
    #[cfg(not(target_arch = "wasm32"))]
    command: Option<Child>,
    url: Url,
}

#[cfg(not(target_arch = "wasm32"))]
impl Proxy {
    /// Waits for the Test Proxy service to exit, return the process exit code when completed.
    pub async fn wait(&mut self) -> Result<ExitStatus> {
        if let Some(command) = &mut self.command {
            return Ok(command.wait().await?);
        }
        Ok(ExitStatus::default())
    }

    /// Attempts to stop the service.
    ///
    /// Waits until the process is killed.
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(command) = &mut self.command {
            tracing::event!(target: crate::SPAN_TARGET, Level::DEBUG, "stopping");
            return Ok(command.kill().await?);
        }
        Ok(())
    }
}

impl Proxy {
    /// Gets the [`Url`] to which the test proxy is listening.
    pub fn endpoint(&self) -> &Url {
        &self.url
    }
}

impl Default for Proxy {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            command: None,
            url: "http://localhost:5000".parse().unwrap(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for Proxy {
    /// Attempts to stop the service.
    ///
    /// Does not wait until the process is killed.
    fn drop(&mut self) {
        if let Some(command) = &mut self.command {
            let _ = command.start_kill();
        }
    }
}

/// Options for starting the [`Proxy`].
#[derive(Clone, Debug)]
pub struct ProxyOptions {
    /// Allow insecure upstream SSL certs.
    pub insecure: bool,

    /// Number of seconds to automatically shut down when no activity.
    pub auto_shutdown_in_seconds: u32,
}

#[cfg(not(target_arch = "wasm32"))]
impl ProxyOptions {
    fn copy_to(&self, args: &mut Vec<String>) {
        if self.insecure {
            args.push("--insecure".into());
        }

        args.extend_from_slice(&[
            "--auto-shutdown-in-seconds".into(),
            self.auto_shutdown_in_seconds.to_string(),
        ]);
    }
}

impl Default for ProxyOptions {
    fn default() -> Self {
        Self {
            insecure: false,
            auto_shutdown_in_seconds: 300,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RecordingId(String);

impl Header for RecordingId {
    fn name(&self) -> HeaderName {
        RECORDING_ID
    }

    fn value(&self) -> HeaderValue {
        self.0.clone().into()
    }
}

impl Header for &RecordingId {
    fn name(&self) -> HeaderName {
        RECORDING_ID
    }

    fn value(&self) -> HeaderValue {
        self.0.clone().into()
    }
}

impl AsRef<str> for RecordingId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for RecordingId {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        Ok(RecordingId(value.to_string()))
    }
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct Version {
    major: i32,
    minor: i32,
    metadata: Option<String>,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(metadata) = &self.metadata {
            return write!(f, "{}.{}-{metadata}", self.major, self.minor);
        }
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl FromStr for Version {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut v = Version::default();

        // cspell:ignore splitn
        let mut cur = s.splitn(2, '.');
        if let Some(major) = cur.next() {
            v.major = major.parse()?;
        } else {
            return Err(azure_core::Error::message(
                ErrorKind::DataConversion,
                "major version required",
            ));
        }
        if let Some(minor) = cur.next() {
            let mut cur = minor.splitn(2, '-');
            if let Some(minor) = cur.next() {
                v.minor = minor.parse()?;
            }
            v.metadata = cur.next().map(String::from);
        }

        Ok(v)
    }
}

#[test]
fn version_eq() {
    let a = Version {
        major: 1,
        ..Default::default()
    };
    let b = Version {
        major: 1,
        ..Default::default()
    };
    assert_eq!(a, b);

    let a = Version {
        major: 1,
        minor: 2,
        metadata: Some("preview".into()),
    };
    let b = Version {
        major: 1,
        minor: 2,
        metadata: Some("preview".into()),
    };
    assert_eq!(a, b);
}

#[test]
fn version_cmp() {
    let a = Version {
        major: 20240107,
        minor: 1,
        ..Default::default()
    };
    let b = Version {
        major: 20240107,
        minor: 2,
        ..Default::default()
    };
    let c = Version {
        major: 20240109,
        minor: 1,
        metadata: Some("1".into()),
    };
    let d = Version {
        major: 20240109,
        minor: 1,
        metadata: Some("2".into()),
    };
    assert!(a == a);
    assert!(a < b);
    assert!(b > a);
    assert!(b < c);
    assert!(c != d);
    assert!(c < d);
}

#[test]
fn version_fmt() {
    let mut v = Version {
        major: 1,
        ..Default::default()
    };
    assert_eq!(v.to_string(), "1.0");

    v.minor = 2;
    v.metadata = Some("preview".into());
    assert_eq!(v.to_string(), "1.2-preview");
}

#[test]
fn version_parse() {
    let mut v = Version {
        major: 1,
        ..Default::default()
    };
    assert!(matches!("1".parse::<Version>(), Ok(ver) if ver == v));
    assert!(matches!("1.0".parse::<Version>(), Ok(ver) if ver == v));

    v.minor = 2;
    assert!(matches!("1.2".parse::<Version>(), Ok(ver) if ver == v));

    v.metadata = Some("preview".into());
    assert!(matches!("1.2-preview".parse::<Version>(), Ok(ver) if ver == v));
}

fn join<S>(value: &[&str], serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = value.join(",");
    serializer.serialize_str(s.as_ref())
}
