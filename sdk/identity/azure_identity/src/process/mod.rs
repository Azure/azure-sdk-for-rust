// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused))]

// cspell:ignore workdir

use crate::env::Env;
use async_trait::async_trait;
use azure_core::{
    credentials::AccessToken,
    error::{Error, ErrorKind, Result},
};
use std::{
    ffi::{OsStr, OsString},
    fmt, io,
    process::Output,
    sync::Arc,
};

mod standard;
#[cfg(feature = "tokio")]
mod tokio;

#[allow(unused)]
pub use standard::StdExecutor;
#[cfg(feature = "tokio")]
pub use tokio::TokioExecutor;

/// Creates a new [`Executor`].
///
/// The returned Executor spawns a [`std::process::Command`] in a separate thread unless `tokio` is enabled,
/// in which case it spawns a `tokio::process::Command`.
pub fn new_executor() -> Arc<dyn Executor> {
    #[cfg(not(feature = "tokio"))]
    {
        Arc::new(StdExecutor)
    }
    #[cfg(feature = "tokio")]
    {
        Arc::new(TokioExecutor)
    }
}

/// An async command runner.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Executor: Send + Sync + fmt::Debug {
    /// Run a program with the given arguments until it terminates, returning the output.
    async fn run(&self, program: &OsStr, args: &[&OsStr]) -> io::Result<Output>;
}

/// Runs a command in the appropriate platform shell and processes the output
/// using the specified `OutputProcessor`.
///
/// - Windows: Runs `cmd /C {command}` in %SYSTEMROOT%
/// - Everywhere else: Runs `/bin/sh -c {command}` in /bin
pub(crate) async fn shell_exec<T: OutputProcessor>(
    executor: Arc<dyn Executor>,
    #[cfg_attr(not(windows), allow(unused_variables))] env: &Env,
    command: &OsStr,
) -> Result<AccessToken> {
    let (workdir, program, c_switch) = {
        #[cfg(windows)]
        {
            let system_root = env.var_os("SYSTEMROOT").map_err(|_| {
                Error::message(
                    ErrorKind::Credential,
                    "SYSTEMROOT environment variable not set",
                )
            })?;
            (system_root, OsStr::new("cmd"), OsStr::new("/C"))
        }
        #[cfg(not(windows))]
        {
            (
                OsString::from("/bin"),
                OsStr::new("/bin/sh"),
                OsStr::new("-c"),
            )
        }
    };

    let mut command_string = OsString::from("cd ");
    command_string.push(workdir);
    command_string.push(" && ");
    command_string.push(command);
    let args = &[c_switch, &command_string];

    let status = executor.run(program, args).await;

    match status {
        Ok(output) if output.status.success() => {
            T::deserialize_token(&String::from_utf8_lossy(&output.stdout))
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let message = if let Some(error_message) = T::get_error_message(&stderr) {
                error_message.to_string()
            } else if output.status.code() == Some(127) || stderr.contains("' is not recognized") {
                format!("{} not found on PATH", T::tool_name())
            } else {
                stderr.to_string()
            };
            Err(Error::with_message(ErrorKind::Credential, || {
                format!("{} authentication failed: {message}", T::credential_name())
            }))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let message = format!(
                "{} authentication failed: {program:?} wasn't found on PATH",
                T::credential_name(),
            );
            Err(Error::full(ErrorKind::Credential, e, message))
        }
        Err(e) => {
            let message = format!(
                "{} failed due to {} error: {e}",
                T::credential_name(),
                e.kind()
            );
            Err(Error::full(ErrorKind::Credential, e, message))
        }
    }
}

pub(crate) trait OutputProcessor: Send + Sized + Sync + 'static {
    /// The credential name to include in error messages
    fn credential_name() -> &'static str;

    /// Deserialize an AccessToken from stdout
    fn deserialize_token(stdout: &str) -> Result<AccessToken>;

    /// Optionally convert stderr to a user-friendly error message.
    /// When this method returns None, the error message will include stderr verbatim.
    fn get_error_message(stderr: &str) -> Option<&str>;

    /// Name of the tool used to get the token e.g. "azd"
    fn tool_name() -> &'static str;
}
