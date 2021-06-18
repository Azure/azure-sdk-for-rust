use async_channel::RecvError;
use azure_core::TokenResponse;
use chrono::{DateTime, Utc};
use oauth2::AccessToken;
use regex::Regex;
use serde::Deserialize;
use std::ffi::OsString;
use std::io::Error;
use std::process::{Command, Output};
use std::str::Utf8Error;
use std::thread;
use std::{io::ErrorKind, path::PathBuf, str::FromStr};

use super::TokenCredential;

lazy_static::lazy_static! {
    static ref POWERSHELL_PATH: PathBuf = get_default_powershell_path();
}

fn get_scope_resource(scope: &str) -> String {
    Regex::new(r"/.default$")
        .unwrap()
        .replace(scope, "")
        .to_string()
}

#[cfg(target_os = "windows")]
fn get_default_powershell_path() -> PathBuf {
    match Command::new("pwsh.exe").args("/?").output() {
        Ok(_) => PathBuf::from_str("pwsh.exe"),
        Err(_) => PathBuf::from_str("powershell.exe"),
    }
}

#[cfg(not(target_os = "windows"))]
fn get_default_powershell_path() -> PathBuf {
    PathBuf::from_str("pwsh").unwrap()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PowerShellTokenResponse {
    pub token: AccessToken,
    pub expires_on: DateTime<Utc>,
}

/// Enables authenticating with Azure Active Directory using Azure PowerShell.
///
/// To use this credential:
/// - Install the Azure PowerShell Module
///   `Install-Module -Name Az -Scope CurrentUser -Repository PSGallery -Force`
/// - Ensure that you are signed in using the `Connect-AzAccount` cmdlet.
pub struct AzurePowerShellCredential {
    powershell_path: Option<PathBuf>,
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum AzurePowerShellCredentialError {
    #[error("failed to import the Az.Account module (is it installed?): {0}")]
    ImportFailed(String),
    #[error("failed to authenticate using the Az.Account module (are you logged in using `Connect-AzAccount`?): {0}")]
    CommandFailed(String),
    #[error("unable to execute PowerShell: {0}")]
    PowerShellNotExecutable(Error),
    #[error("failed to receive PowerShell subprocess output: {0}")]
    ReceiveFailed(RecvError),
    #[error("the token response from Az.Account was not UTF-8 encoded")]
    ResponseNotUtf8(Utf8Error),
    #[error("failed to deserialize Az.Account token response")]
    ResponseFailedToDeserialize(serde_json::Error),
    #[error("unknown error of kind: {0:?}")]
    UnknownError(ErrorKind),
}

impl From<Error> for AzurePowerShellCredentialError {
    fn from(error: Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound => AzurePowerShellCredentialError::PowerShellNotExecutable(error),
            error_kind => AzurePowerShellCredentialError::UnknownError(error_kind),
        }
    }
}

impl AzurePowerShellCredential {
    pub fn new(powershell_path: Option<PathBuf>) -> Self {
        Self {
            powershell_path: powershell_path,
        }
    }

    /// A helper to run PowerShell commands asynchronously using threads and an async_channel
    async fn execute_powershell_command<I, S>(
        &self,
        args_iter: I,
    ) -> Result<Output, AzurePowerShellCredentialError>
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        let pwsh = self.powershell_path.as_ref().unwrap_or(&*POWERSHELL_PATH);

        let mut command = Command::new(pwsh.clone());

        let args: Vec<OsString> = args_iter.into_iter().map(|v| v.into()).collect();

        let (s, r) = async_channel::bounded(1);
        thread::spawn(move || {
            let output = command.args(args).output();
            futures::executor::block_on(s.send(output)).unwrap();
        });

        r.recv()
            .await
            .map_err(|recv_error| AzurePowerShellCredentialError::ReceiveFailed(recv_error))?
            .map_err(|error| error.into())
    }

    async fn get_access_token(
        &self,
        resource: Option<&str>,
    ) -> Result<PowerShellTokenResponse, AzurePowerShellCredentialError> {
        let import_response = self
            .execute_powershell_command(vec![
                "-Command",
                "Import-Module Az.Accounts -MinimumVersion 2.2.0 -PassThru",
            ])
            .await?;

        if !import_response.status.success() {
            let output = String::from_utf8_lossy(&import_response.stderr);
            return Err(AzurePowerShellCredentialError::ImportFailed(
                output.to_string(),
            ));
        }

        let resource_fragment = resource
            .map(|scope| format!("-ResourceUrl \"{}\"", get_scope_resource(scope)))
            .unwrap_or_default();

        let get_token_response = self
            .execute_powershell_command(vec![
                "-Command",
                format!("Get-AzAccessToken {}| ConvertTo-Json", resource_fragment).as_str(),
            ])
            .await?;

        if !get_token_response.status.success() {
            let output = String::from_utf8_lossy(&get_token_response.stderr);
            Err(AzurePowerShellCredentialError::CommandFailed(
                output.to_string(),
            ))
        } else {
            let result = std::str::from_utf8(&get_token_response.stdout)
                .map_err(AzurePowerShellCredentialError::ResponseNotUtf8)?;

            let token_response = serde_json::from_str::<PowerShellTokenResponse>(result)
                .map_err(AzurePowerShellCredentialError::ResponseFailedToDeserialize)?;
            Ok(token_response)
        }
    }
}

impl Into<TokenResponse> for PowerShellTokenResponse {
    fn into(self) -> TokenResponse {
        TokenResponse {
            token: self.token,
            expires_on: self.expires_on,
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for AzurePowerShellCredential {
    type Error = AzurePowerShellCredentialError;
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Self::Error> {
        Ok(self.get_access_token(Some(resource)).await?.into())
    }
}

#[async_trait::async_trait]
impl azure_core::TokenCredential for AzurePowerShellCredential {
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetTokenError(Box::new(error)))
    }
}
