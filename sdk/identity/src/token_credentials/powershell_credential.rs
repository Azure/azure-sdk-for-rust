use azure_core::TokenResponse;
use chrono::{DateTime, Utc};
use oauth2::AccessToken;
use regex::Regex;
use serde::Deserialize;
use std::io::Error;
use std::str::Utf8Error;
use std::{io::ErrorKind, path::PathBuf, str::FromStr};
use tokio::process::Command;

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
    #[error("Failed to import the Az.Account module (is it installed?): {0}")]
    ImportFailed(String),
    #[error("Failed to authenticate using the Az.Account module (are you logged in using `Connect-AzAccount`?): {0}")]
    CommandFailed(String),
    #[error("PowerShell is not installed and available on the system PATH: {0}")]
    PowerShellNotInstalled(Error),
    #[error("Az.Account token response was not UTF-8 encoded")]
    ResponseNotUtf8(Utf8Error),
    #[error("Failed to deserialize Az.Account token response")]
    ResponseFailedToDeserialize(serde_json::Error),
    #[error("Unknown error of kind: {0:?}")]
    UnknownError(ErrorKind),
}

fn error_to_powershellcredential_error<T>(
    error: Error,
) -> Result<T, AzurePowerShellCredentialError> {
    match error.kind() {
        ErrorKind::NotFound => Err(AzurePowerShellCredentialError::PowerShellNotInstalled(
            error,
        )),
        error_kind => Err(AzurePowerShellCredentialError::UnknownError(error_kind)),
    }
}

impl AzurePowerShellCredential {
    pub fn new(powershell_path: Option<PathBuf>) -> AzurePowerShellCredential {
        AzurePowerShellCredential {
            powershell_path: powershell_path,
        }
    }

    async fn get_access_token(
        &self,
        resource: Option<&str>,
    ) -> Result<PowerShellTokenResponse, AzurePowerShellCredentialError> {
        let pwsh = self
            .powershell_path
            .as_ref()
            .unwrap_or(&*POWERSHELL_PATH)
            .as_os_str();

        match Command::new(pwsh)
            .args(vec![
                "-Command",
                "Import-Module Az.Accounts -MinimumVersion 2.2.0 -PassThru",
            ])
            .output()
            .await
        {
            Ok(response) if !response.status.success() => {
                let output = String::from_utf8_lossy(&response.stderr);
                return Err(AzurePowerShellCredentialError::ImportFailed(
                    output.to_string(),
                ));
            }
            Err(err) => {
                return error_to_powershellcredential_error(err);
            }
            _ => { /* Everything else is OK to proceed. */ }
        };

        let resource_fragment = match resource {
            Some(scope) => format!("-ResourceUrl \"{}\" ", get_scope_resource(scope)),
            None => String::from(""),
        };

        let result = Command::new(pwsh)
            .args(vec![
                "-Command",
                format!("Get-AzAccessToken {}| ConvertTo-Json", resource_fragment).as_str(),
            ])
            .output()
            .await;

        match result {
            Ok(get_token_output) if get_token_output.status.success() => {
                let response = std::str::from_utf8(&get_token_output.stdout)
                    .map_err(AzurePowerShellCredentialError::ResponseNotUtf8)?;

                let token_response = serde_json::from_str::<PowerShellTokenResponse>(response)
                    .map_err(AzurePowerShellCredentialError::ResponseFailedToDeserialize)?;
                Ok(token_response)
            }
            Ok(get_token_output) => {
                let output = String::from_utf8_lossy(&get_token_output.stderr);
                Err(AzurePowerShellCredentialError::CommandFailed(
                    output.to_string(),
                ))
            }
            Err(error) => error_to_powershellcredential_error(error),
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
