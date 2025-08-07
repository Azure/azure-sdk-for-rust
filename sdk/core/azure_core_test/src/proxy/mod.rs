// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Wrappers for the [Test Proxy](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md) service.
#[cfg(not(target_arch = "wasm32"))]
mod bootstrap;
pub(crate) mod client;
pub(crate) mod matchers;
pub(crate) mod models;
pub(crate) mod policy;
pub(crate) mod sanitizers;

#[cfg(not(target_arch = "wasm32"))]
use self::{
    bootstrap::*, matchers::CustomDefaultMatcher, models::SanitizerList,
    sanitizers::DEFAULT_SANITIZERS_TO_REMOVE,
};
use azure_core::{
    error::ErrorKind,
    http::{
        headers::{Header, HeaderName, HeaderValue},
        Url,
    },
    Result,
};
use client::Client;
use serde::Serializer;
#[cfg(not(target_arch = "wasm32"))]
use std::process::ExitStatus;
use std::{fmt, str::FromStr, sync::Arc};
#[cfg(not(target_arch = "wasm32"))]
use tokio::process::Child;

const ABSTRACTION_IDENTIFIER: HeaderName = HeaderName::from_static("x-abstraction-identifier");
const RECORDING_ID: HeaderName = HeaderName::from_static("x-recording-id");
const RECORDING_MODE: HeaderName = HeaderName::from_static("x-recording-mode");
const RECORDING_UPSTREAM_BASE_URI: HeaderName =
    HeaderName::from_static("x-recording-upstream-base-uri");

#[cfg(not(target_arch = "wasm32"))]
pub use bootstrap::start;

/// Represents the running `test-proxy` service.
#[derive(Debug, Default)]
pub struct Proxy {
    #[cfg(not(target_arch = "wasm32"))]
    command: Option<Child>,
    endpoint: Option<Url>,
    client: Option<Client>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Proxy {
    async fn start<I: Iterator<Item = String>>(
        &mut self,
        git_dir: &Path,
        executable_file_path: &Path,
        args: I,
    ) -> Result<()> {
        let mut command = Command::new(executable_file_path)
            .args(args)
            .env(
                KESTREL_CERT_PATH_ENV,
                // cspell:ignore testproxy devcert
                git_dir.join("eng/common/testproxy/dotnet-devcert.pfx"),
            )
            .env(KESTREL_CERT_PASSWORD_ENV, KESTREL_CERT_PASSWORD)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                azure_core::Error::full(
                    ErrorKind::Io,
                    err,
                    format!("{} failed to start", executable_file_path.display()),
                )
            })?;

        let mut stdout = command
            .stdout
            .take()
            .ok_or_else(|| azure_core::Error::message(ErrorKind::Io, "no stdout pipe"))?;
        // Take stderr now but we won't listen until after start up, such that messages should buffer.
        let mut stderr = command
            .stderr
            .take()
            .ok_or_else(|| azure_core::Error::message(ErrorKind::Io, "no stderr pipe"))?;
        self.command = Some(command);

        // Wait until the service is listening on a port.
        self.wait_till_listening(&mut stdout).await?;

        // Then spawn a thread to keep pumping messages to stdout and stderr.
        // The pipe will be closed when the process is shut down, which will terminate the task.
        tokio::spawn(async move {
            let mut reader = BufReader::new(&mut stdout).lines();
            while let Some(line) = reader.next_line().await.unwrap_or(None) {
                // Trace useful lines that test-proxy writes to stdout.
                trace_line(Level::TRACE, &line);
            }
        });
        tokio::spawn(async move {
            let mut reader = BufReader::new(&mut stderr).lines();
            while let Some(line) = reader.next_line().await.unwrap_or(None) {
                // Trace useful lines that test-proxy writes to stdout.
                trace_line(Level::ERROR, &line);
            }
        });

        if let Some(endpoint) = &self.endpoint {
            self.client = Some(Client::new(endpoint.clone())?);
        }

        Ok(())
    }

    pub(crate) async fn initialize(&self) -> Result<()> {
        // Set default matcher and remove some sanitizers by default.
        if let Some(client) = &self.client {
            client
                .set_matcher(CustomDefaultMatcher::default().into(), None)
                .await?;

            let body = SanitizerList {
                sanitizers: Vec::from_iter(
                    DEFAULT_SANITIZERS_TO_REMOVE
                        .iter()
                        .map(|s| String::from(*s)),
                ),
            };
            client.remove_sanitizers(body.try_into()?, None).await?;
        }

        Ok(())
    }

    /// Attempts to stop the service.
    ///
    /// Waits until the process is killed.
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(command) = &mut self.command {
            tracing::debug!(pid = ?command.id(), "stopping");
            return Ok(command.kill().await?);
        }
        Ok(())
    }

    /// Waits for the Test Proxy service to exit, return the process exit code when completed.
    pub async fn wait(&mut self) -> Result<ExitStatus> {
        if let Some(command) = &mut self.command {
            return Ok(command.wait().await?);
        }
        Ok(ExitStatus::default())
    }

    async fn wait_till_listening(&mut self, stdout: &mut ChildStdout) -> Result<()> {
        let pid = self.command.as_ref().and_then(Child::id);
        let mut reader = BufReader::new(stdout).lines();
        while let Some(line) = reader.next_line().await? {
            const RUNNING_PATTERN: &str = "Running proxy version is Azure.Sdk.Tools.TestProxy ";
            const LISTENING_PATTERN: &str = "Now listening on: ";

            // Trace useful lines that test-proxy writes to stdout.
            trace_line(Level::TRACE, &line);

            if let Some(idx) = line.find(RUNNING_PATTERN) {
                let idx = idx + RUNNING_PATTERN.len();
                let version: Version = line[idx..].parse()?;
                tracing::info!(?pid, %version, "started test-proxy version {version}");

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
                let mut endpoint: Url = line[idx..].parse()?;
                endpoint.set_host(Some("localhost"))?;
                tracing::info!(?pid, %endpoint, "test-proxy listening on {endpoint}");

                self.endpoint = Some(endpoint);
                break;
            }
        }

        Ok(())
    }
}

impl Proxy {
    /// Gets a proxy representing an existing test-proxy process.
    pub fn existing() -> Result<Self> {
        let endpoint: Url = "http://localhost:5000".parse()?;
        Ok(Self {
            #[cfg(not(target_arch = "wasm32"))]
            command: None,
            endpoint: Some(endpoint.clone()),
            client: Some(Client::new(endpoint)?),
        })
    }

    /// Gets the [`Url`] to which the test-proxy is listening.
    pub fn endpoint(&self) -> &Url {
        self.endpoint
            .as_ref()
            // Okay to panic because this is a developer error.
            .unwrap_or_else(|| panic!("endpoint not set"))
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

pub(crate) trait ProxyExt<'a> {
    fn client(&'a self) -> Option<&'a Client>;
}

impl<'a> ProxyExt<'a> for Option<Arc<Proxy>> {
    fn client(&'a self) -> Option<&'a Client> {
        self.as_ref().and_then(|proxy| proxy.client.as_ref())
    }
}

/// Options for starting the [`Proxy`].
#[derive(Clone, Debug)]
pub struct ProxyOptions {
    /// `true` to bind to any available port; otherwise, bind to only the default port `:5000`.
    pub auto: bool,

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

        if self.auto {
            args.extend_from_slice(&["--", "--urls", "http://0.0.0.0:0"].map(Into::into));
        }
    }
}

impl Default for ProxyOptions {
    fn default() -> Self {
        Self {
            auto: true,
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

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
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
