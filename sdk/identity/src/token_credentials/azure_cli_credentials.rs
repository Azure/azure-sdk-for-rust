use azure_core::auth::{AccessToken, TokenCredential, TokenResponse};
use azure_core::error::{Error, ErrorKind, ResultExt};
use serde::Deserialize;
use std::process::Command;
use std::str;
use time::OffsetDateTime;

mod az_cli_date_format {
    use azure_core::date;
    use azure_core::error::{ErrorKind, ResultExt};
    use serde::{self, Deserialize, Deserializer};
    use time::format_description::FormatItem;
    use time::macros::format_description;
    use time::{OffsetDateTime, PrimitiveDateTime};

    const FORMAT: &[FormatItem] =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]");

    pub fn parse(s: &str) -> azure_core::Result<OffsetDateTime> {
        // expiresOn from azure cli uses the local timezone and needs to be converted to UTC
        let dt = PrimitiveDateTime::parse(s, FORMAT)
            .with_context(ErrorKind::DataConversion, || {
                format!("unable to parse expiresOn '{s}")
            })?;
        Ok(date::assume_local(&dt))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CliTokenResponse {
    pub access_token: AccessToken,
    #[serde(with = "az_cli_date_format")]
    pub expires_on: OffsetDateTime,
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
                    format!("'az account get-access-token' command failed: {output}")
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

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AzureCliCredential {
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        let tr = Self::get_access_token(Some(resource))?;
        Ok(TokenResponse::new(tr.access_token, tr.expires_on))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::date;
    use time::macros::datetime;

    #[test]
    fn can_parse_expires_on() -> azure_core::Result<()> {
        let expires_on = "2022-07-30 12:12:53.919110";
        assert_eq!(
            az_cli_date_format::parse(expires_on)?,
            date::assume_local(&datetime!(2022-07-30 12:12:53.919110))
        );
        Ok(())
    }
}
