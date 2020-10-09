use crate::token_credentials::{TokenCredential, TokenResponse};
use azure_sdk_core::errors::AzureError;
use chrono::{DateTime, Utc};
use oauth2::AccessToken;
use std::io::ErrorKind;
use std::process::Command;
use std::str;

mod az_cli_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%6f";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CliTokenResponse {
    pub access_token: AccessToken,
    #[serde(with = "az_cli_date_format")]
    pub expires_on: DateTime<Utc>,
    pub subscription: String,
    pub tenant: String,
    pub token_type: String,
}

/// Enables authentication to Azure Active Directory using Azure CLI to obtain an access token.
pub struct AzureCliCredential;

#[async_trait::async_trait]
impl TokenCredential for AzureCliCredential {
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError> {
        let az_command = if cfg!(target_os = "windows") {
            // on window az is a cmd and it should be called like this
            // see https://doc.rust-lang.org/nightly/std/process/struct.Command.html
            Command::new("cmd")
            .args(&[
                "/C",
                "az",
                "account",
                "get-access-token",
                "--output",
                "json",
                "--resource",
                resource,
            ]).output()
        } else {
            Command::new("az")
            .args(&[
                "account",
                "get-access-token",
                "--output",
                "json",
                "--resource",
                resource,
            ]).output()
        };

        let res = match az_command {
            Ok(az_output) => {
                if az_output.status.success() {
                    let output = str::from_utf8(&az_output.stdout).unwrap();
                    let tr = serde_json::from_str::<CliTokenResponse>(output)
                        .map(|tr| TokenResponse::new(tr.access_token, tr.expires_on))
                        .map_err(|_| {
                            AzureError::GenericErrorWithText(
                                "Failed to serialize response".to_string(),
                            )
                        })?;
                    Ok(tr)
                } else {
                    let output = str::from_utf8(&az_output.stderr).unwrap();
                    Err(AzureError::GenericErrorWithText(output.to_string()))
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Err(AzureError::GenericErrorWithText(
                    "Azure CLI not installed".to_owned(),
                )),
                _ => Err(AzureError::GenericErrorWithText(e.to_string())),
            },
        };

        res
    }
}
