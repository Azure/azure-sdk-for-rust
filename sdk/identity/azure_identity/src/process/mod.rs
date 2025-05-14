// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore workdir

use azure_core::{
    credentials::AccessToken,
    error::{Error, ErrorKind, Result},
    process::Executor,
};
use std::{ffi::OsStr, sync::Arc};

use crate::env::Env;

/// Runs a command in the appropriate platform shell and processes the output
/// using the specified `OutputProcessor`.
///
/// - Windows: Runs `cmd /C {command}` in %SYSTEMROOT%
/// - Everywhere else: Runs `/bin/sh -c {command}` in /bin
pub(crate) async fn shell_exec<T: OutputProcessor>(
    executor: Arc<dyn Executor>,
    env: &Env,
    command: &str,
) -> Result<AccessToken> {
    let (workdir, program, c_switch) = if cfg!(target_os = "windows") {
        let system_root = env.var("SYSTEMROOT").map_err(|_| {
            Error::message(
                ErrorKind::Credential,
                "SYSTEMROOT environment variable not set",
            )
        })?;
        (system_root, "cmd", "/C")
    } else {
        ("/bin".to_string(), "/bin/sh", "-c")
    };

    let command_string = format!("cd {workdir} && {command}");
    let args = vec![OsStr::new(c_switch), OsStr::new(&command_string)];

    let status = executor.run(OsStr::new(program), &args).await;

    match status {
        Ok(output) if output.status.success() => {
            T::deserialize_token(&String::from_utf8_lossy(&output.stdout))
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let message = if let Some(error_message) = T::get_error_message(&stderr) {
                error_message.to_string()
            } else if output.status.code() == Some(127) || stderr.contains("' is not recognized") {
                format!("{} not found on path", T::tool_name())
            } else {
                stderr.to_string()
            };
            Err(Error::with_message(ErrorKind::Credential, || {
                format!("{} authentication failed: {message}", T::credential_name())
            }))
        }
        Err(e) => {
            let message = format!(
                "{} failed due to {} error: {e}",
                T::credential_name(),
                e.kind()
            );
            Err(Error::with_message(ErrorKind::Credential, || message))
        }
    }
}

pub trait OutputProcessor: Send + Sized + Sync + 'static {
    /// The credential name to include in error messages
    fn credential_name() -> &'static str;

    /// Deserialize an AccessToken from stdout
    fn deserialize_token(stdout: &str) -> Result<AccessToken>;

    /// Optionally convert stderr to a user-friendly error message.
    /// When this method returns None, the error message will include stderr verbatim.
    fn get_error_message(stderr: &str) -> Option<&str>;

    /// Friendly name of the tool used to get the token e.g. "Azure CLI"
    fn tool_name() -> &'static str;
}
