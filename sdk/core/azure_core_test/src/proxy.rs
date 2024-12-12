// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Wrappers for the [Test Proxy](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md) service.

use azure_core::{error::ErrorKind, Result, Url};
use std::{
    env, io,
    path::Path,
    process::{ExitStatus, Stdio},
    sync::Arc,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, ChildStdout, Command},
};
use tracing::{Level, Span};

// cspell:ignore aspnetcore devcert testproxy
const KESTREL_CERT_PATH_ENV: &str = "ASPNETCORE_Kestrel__Certificates__Default__Path";
const KESTREL_CERT_PASSWORD_ENV: &str = "ASPNETCORE_Kestrel__Certificates__Default__Password";
const KESTREL_CERT_PASSWORD: &str = "password";
const PROXY_MANUAL_START: &str = "PROXY_MANUAL_START";

pub async fn start(
    test_data_dir: impl AsRef<Path>,
    options: Option<ProxyOptions>,
) -> Result<Proxy> {
    if env::var(PROXY_MANUAL_START).is_ok_and(|v| v.to_ascii_lowercase() == "true") {
        tracing::event!(target: crate::SPAN_TARGET, Level::WARN, "environment variable {PROXY_MANUAL_START} is 'true'; not starting test proxy");
        return Ok(Proxy::default());
    }

    // Find root of git repo or work tree: a ".git" directory or file will exist either way.
    let git_dir = crate::find_ancestor(test_data_dir, ".git")?;
    let git_dir = git_dir.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "parent git repository not found")
    })?;

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
    let url = wait_till_listening(&mut stdout).await?;

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

    // cspell:ignore teamprojectid
    let _max_seconds = env::var("SYSTEM_TEAMPROJECTID").map_or(5, |_| 20);

    // Wait for the process to respond to requests and check output until pattern: "Now listening on: http://0.0.0.0:60583" (random port).
    let mut reader = BufReader::new(stdout).lines();
    while let Some(line) = reader.next_line().await? {
        const PATTERN: &str = "Now listening on: ";

        if let Some(idx) = line.find(PATTERN) {
            let idx = idx + PATTERN.len();
            let url = line[idx..].parse()?;
            tracing::event!(target: crate::SPAN_TARGET, Level::INFO, "listening on {}", url);

            return Ok(url);
        }
    }

    Err(azure_core::Error::message(
        ErrorKind::Io,
        "timed out waiting for test-proxy to start",
    ))
}

/// Represents the running `test-proxy` service.
#[derive(Debug)]
pub struct Proxy {
    command: Option<Child>,
    url: Url,
}

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

    /// Gets the [`Url`] to which the test proxy is listening.
    pub fn url(&self) -> &Url {
        &self.url
    }
}

impl Default for Proxy {
    fn default() -> Self {
        Self {
            command: None,
            url: "http://localhost:5000".parse().unwrap(),
        }
    }
}

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
#[derive(Clone, Debug, Default)]
pub struct ProxyOptions {
    /// Allow insecure upstream SSL certs.
    pub insecure: bool,
}

impl ProxyOptions {
    fn copy_to(&self, args: &mut Vec<String>) {
        if self.insecure {
            args.push("--insecure".into());
        }
    }
}

/// Represents a playback or recording session using the [`Proxy`].
pub struct Session {
    #[allow(dead_code)]
    pub(crate) proxy: Arc<Proxy>,

    #[allow(dead_code)]
    pub(crate) span: Span,
}
