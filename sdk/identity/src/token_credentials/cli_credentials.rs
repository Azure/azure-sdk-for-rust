use azure_core::errors::AzureError;
use azure_core::{TokenCredential, TokenResponse};
use chrono::{DateTime, Utc};
use oauth2::AccessToken;
use serde::Deserialize;

use std::io::ErrorKind;
use std::process::Command;
use std::str;

mod az_cli_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S.%6f";

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

impl AzureCliCredential {
    /// Get an access token for an optional resource
    fn get_access_token(resource: Option<&str>) -> Result<CliTokenResponse, AzureError> {
        // on window az is a cmd and it should be called like this
        // see https://doc.rust-lang.org/nightly/std/process/struct.Command.html
        let program = if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "az"
        };
        let mut args = Vec::new();
        if cfg!(target_os = "windows") {
            args.push("/C");
            args.push("az");
        }
        args.push("account");
        args.push("get-access-token");
        args.push("--output");
        args.push("json");
        if let Some(resource) = resource {
            args.push("--resource");
            args.push(resource);
        }

        match Command::new(program).args(args).output() {
            Ok(az_output) if az_output.status.success() => {
                let output = str::from_utf8(&az_output.stdout).map_err(|_| {
                    AzureError::GenericErrorWithText(
                        "Token response was not utf8 encoded".to_string(),
                    )
                })?;

                let token_response =
                    serde_json::from_str::<CliTokenResponse>(output).map_err(|_| {
                        AzureError::GenericErrorWithText(
                            "Failed to deserialize token response".to_string(),
                        )
                    })?;
                Ok(token_response)
            }
            Ok(az_output) => {
                let output = String::from_utf8_lossy(&az_output.stderr);
                Err(AzureError::GenericErrorWithText(output.to_string()))
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Err(AzureError::GenericErrorWithText(
                    "Azure CLI not installed".to_owned(),
                )),
                _ => Err(AzureError::GenericErrorWithText(e.to_string())),
            },
        }
    }

    /// Returns the current subscription ID from the Azure CLI.
    pub fn get_subscription() -> Result<String, AzureError> {
        let tr = Self::get_access_token(None)?;
        Ok(tr.subscription)
    }

    /// Returns the current tenant ID from the Azure CLI.
    pub fn get_tenant() -> Result<String, AzureError> {
        let tr = Self::get_access_token(None)?;
        Ok(tr.tenant)
    }
}

#[async_trait::async_trait]
impl TokenCredential for AzureCliCredential {
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError> {
        let tr = Self::get_access_token(Some(resource))?;
        Ok(TokenResponse::new(tr.access_token, tr.expires_on))
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use serde_test::{assert_de_tokens, Token};

    use super::*;

    #[derive(Debug, Deserialize)]
    struct AzureDateTime {
        #[serde(with = "az_cli_date_format")]
        date: DateTime<Utc>,
    }
    #[test]
    fn can_parse_cli_datetime() {
        let s = "2020-11-16T04:25:03Z";
        let utc = Utc.ymd(2020, 11, 16).and_hms(4, 25, 03);
        let dt = AzureDateTime { date: utc };
        assert_de_tokens(&dt.date, &[Token::Str(s)]);

        let s = "2020-11-16 04:25:03Z";
        let utc = Utc.ymd(2020, 11, 16).and_hms(4, 25, 03);
        let dt = AzureDateTime { date: utc };
        assert_de_tokens(&dt.date, &[Token::Str(s)]);
    }
}
