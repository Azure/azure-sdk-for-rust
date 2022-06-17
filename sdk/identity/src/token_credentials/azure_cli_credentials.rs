use azure_core::auth::{TokenCredential, TokenResponse};
use azure_core::error::{Error, ErrorKind, ResultExt};
use chrono::{DateTime, Utc};
use oauth2::AccessToken;
use serde::Deserialize;
use std::process::Command;
use std::str;

mod az_cli_date_format {
    use chrono::{DateTime, Local, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S.%6f";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // expiresOn from azure cli uses the local timezone and needs to be converted to UTC
        let local_datetime = Local
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)?;
        Ok(local_datetime.with_timezone(&Utc))
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
    #[allow(unused)]
    pub token_type: String,
}

/// Enables authentication to Azure Active Directory using Azure CLI to obtain an access token.
pub struct AzureCliCredential;

impl AzureCliCredential {
    /// Get an access token for an optional resource
    fn get_access_token(resource: Option<&str>) -> azure_core::Result<CliTokenResponse> {
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
                let output = str::from_utf8(&az_output.stdout)?;

                let token_response = serde_json::from_str::<CliTokenResponse>(output)
                    .map_kind(ErrorKind::DataConversion)?;
                Ok(token_response)
            }
            Ok(az_output) => {
                let output = String::from_utf8_lossy(&az_output.stderr);
                Err(Error::with_message(ErrorKind::Credential, || {
                    format!("az account get-access-token command failed: {output}")
                }))
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    Err(Error::message(ErrorKind::Other, "Azure CLI not installed"))
                }
                error_kind => Err(Error::with_message(ErrorKind::Other, || {
                    format!("Unknown error of kind: {error_kind:?}")
                })),
            },
        }
    }

    /// Returns the current subscription ID from the Azure CLI.
    pub fn get_subscription() -> azure_core::Result<String> {
        let tr = Self::get_access_token(None)?;
        Ok(tr.subscription)
    }

    /// Returns the current tenant ID from the Azure CLI.
    pub fn get_tenant() -> azure_core::Result<String> {
        let tr = Self::get_access_token(None)?;
        Ok(tr.tenant)
    }
}

#[async_trait::async_trait]
impl TokenCredential for AzureCliCredential {
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
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
        let utc = Utc.ymd(2020, 11, 16).and_hms(4, 25, 3);
        let dt = AzureDateTime { date: utc };
        assert_de_tokens(&dt.date, &[Token::Str(s)]);

        let s = "2020-11-16 04:25:03Z";
        let utc = Utc.ymd(2020, 11, 16).and_hms(4, 25, 3);
        let dt = AzureDateTime { date: utc };
        assert_de_tokens(&dt.date, &[Token::Str(s)]);
    }
}
