// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    credentials::TokenCredentialOptions, validate_scope, validate_subscription, validate_tenant_id,
};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
    json::from_json,
    process::{new_executor, Executor},
};
use serde::Deserialize;
use std::{ffi::OsStr, fmt::Debug, str, sync::Arc};
use time::OffsetDateTime;
use tracing::trace;

#[cfg(feature = "old_azure_cli")]
mod az_cli_date_format {
    use azure_core::error::{ErrorKind, ResultExt};
    use serde::{Deserialize, Deserializer};
    use time::format_description::FormatItem;
    use time::macros::format_description;
    #[cfg(not(unix))]
    use time::UtcOffset;
    use time::{OffsetDateTime, PrimitiveDateTime};

    // cspell:ignore subsecond
    const FORMAT: &[FormatItem] =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]");

    pub fn parse(s: &str) -> azure_core::Result<OffsetDateTime> {
        // expiresOn from azure cli uses the local timezone and needs to be converted to UTC
        let dt = PrimitiveDateTime::parse(s, FORMAT)
            .with_context(ErrorKind::DataConversion, || {
                format!("unable to parse expiresOn '{s}")
            })?;
        Ok(assume_local(&dt))
    }

    #[cfg(unix)]
    /// attempt to convert `PrimitiveDateTime` to `OffsetDate` using
    /// `tz::TimeZone`.  If any part of the conversion fails, such as if no
    /// timezone can be found, then use use the value as UTC.
    pub(crate) fn assume_local(date: &PrimitiveDateTime) -> OffsetDateTime {
        let as_utc = date.assume_utc();

        // try parsing the timezone from `TZ` environment variable.  If that
        // fails, or the environment variable doesn't exist, try using
        // `TimeZone::local`.  If that fails, then just return the UTC date.
        let Some(tz) = std::env::var("TZ")
            .ok()
            .and_then(|x| tz::TimeZone::from_posix_tz(&x).ok())
            .or_else(|| tz::TimeZone::local().ok())
        else {
            return as_utc;
        };

        let as_unix = as_utc.unix_timestamp();

        // if we can't find the local time type, just return the UTC date
        let Ok(local_time_type) = tz.find_local_time_type(as_unix) else {
            return as_utc;
        };

        // if we can't convert the unix timestamp to a DateTime, just return the UTC date
        let date = as_utc.date();
        let time = as_utc.time();
        let Ok(date) = tz::DateTime::new(
            date.year(),
            u8::from(date.month()),
            date.day(),
            time.hour(),
            time.minute(),
            time.second(),
            time.nanosecond(),
            *local_time_type,
        ) else {
            return as_utc;
        };

        // if we can't then convert to unix time (with the timezone) and then
        // back into an OffsetDateTime, then return the UTC date
        let Ok(date) = OffsetDateTime::from_unix_timestamp(date.unix_time()) else {
            return as_utc;
        };

        date
    }

    /// Assumes the local offset. Default to UTC if unable to get local offset.
    #[cfg(not(unix))]
    pub(crate) fn assume_local(date: &PrimitiveDateTime) -> OffsetDateTime {
        date.assume_offset(UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse(&s).map_err(serde::de::Error::custom)
    }
}

/// The response from `az account get-access-token --output json`.
#[derive(Debug, Clone, Deserialize)]
struct CliTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: Secret,
    #[cfg(feature = "old_azure_cli")]
    #[serde(rename = "expiresOn", with = "az_cli_date_format")]
    /// The token's expiry time formatted in the local timezone.
    /// Unfortunately, this requires additional timezone dependencies.
    /// See https://github.com/Azure/azure-cli/issues/19700 for details.
    pub local_expires_on: OffsetDateTime,
    #[serde(rename = "expires_on")]
    /// The token's expiry time in seconds since the epoch, a unix timestamp.
    /// Available in Azure CLI 2.54.0 or newer.
    pub expires_on: Option<i64>,
    #[allow(unused)]
    #[serde(rename = "tokenType")]
    pub token_type: String,
}

impl CliTokenResponse {
    pub fn expires_on(&self) -> azure_core::Result<OffsetDateTime> {
        match self.expires_on {
            Some(timestamp) => Ok(OffsetDateTime::from_unix_timestamp(timestamp)
                .with_context(ErrorKind::DataConversion, || {
                    format!("unable to parse expires_on '{timestamp}'")
                })?),
            None => {
                #[cfg(feature = "old_azure_cli")]
                {
                    Ok(self.local_expires_on)
                }
                #[cfg(not(feature = "old_azure_cli"))]
                {
                    Err(Error::message(
                        ErrorKind::DataConversion,
                        "expires_on field not found. Please use Azure CLI 2.54.0 or newer.",
                    ))
                }
            }
        }
    }
}

/// Enables authentication to Azure Active Directory using Azure CLI to obtain an access token.
#[derive(Debug)]
pub struct AzureCliCredential {
    options: AzureCliCredentialOptions,
}

/// Options for constructing an [`AzureCliCredential`].
#[derive(Clone, Debug, Default)]
pub struct AzureCliCredentialOptions {
    /// Specifies tenants to which the credential may authenticate, in addition to [`Self::tenant_id`].
    ///
    /// When `tenant_id` is `None` this option has no effect and the credential will authenticate to any requested tenant.
    /// Add the wildcard value "*" to allow the credential to authenticate to any tenant.
    pub additionally_allowed_tenants: Vec<String>,

    /// The name or ID of a subscription
    ///
    /// Set this to acquire tokens for an account other than the Azure CLI's current account.
    pub subscription: Option<String>,

    /// Identifies the tenant the credential should authenticate in.
    ///
    /// Defaults to the CLI's default tenant, which is typically the home tenant of the logged in user.
    pub tenant_id: Option<String>,

    /// An implementation of [`Executor`] to run commands asynchronously.
    ///
    /// If `None`, one is created using [`new_executor`]; alternatively,
    /// you can supply your own implementation using a different asynchronous runtime.
    pub executor: Option<Arc<dyn Executor>>,
}

impl AzureCliCredential {
    /// Create a new `AzureCliCredential`.
    pub fn new(options: Option<AzureCliCredentialOptions>) -> azure_core::Result<Arc<Self>> {
        let mut options = options.unwrap_or_default();
        if let Some(ref tenant_id) = options.tenant_id {
            validate_tenant_id(tenant_id)?;
        }
        if let Some(ref subscription) = options.subscription {
            validate_subscription(subscription)?;
        }
        if options.executor.is_none() {
            options.executor = Some(new_executor());
        }

        Ok(Arc::new(Self { options }))
    }

    /// Get an access token for an optional resource
    async fn get_access_token(&self, scopes: &[&str]) -> azure_core::Result<CliTokenResponse> {
        if scopes.is_empty() {
            return Err(Error::new(
                ErrorKind::Credential,
                "exactly one scope required",
            ));
        }
        // Pass the CLI a Microsoft Entra ID v1 resource because we don't know which CLI version is installed and older ones don't support v2 scopes.
        let resource = scopes[0].trim_end_matches("/.default");
        validate_scope(resource)?;

        // On Windows az is a cmd and it should be called like this.
        // See https://doc.rust-lang.org/nightly/std/process/struct.Command.html
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
        args.push("--resource");
        args.push(resource);

        if let Some(ref tenant_id) = self.options.tenant_id {
            args.push("--tenant");
            args.push(tenant_id);
        }
        if let Some(ref subscription) = self.options.subscription {
            args.push("--subscription");
            args.push(subscription);
        }

        trace!(
            "fetching credential via Azure CLI: {program} {}",
            args.join(" "),
        );

        let args = args.iter().map(|arg| arg.as_ref()).collect::<Vec<&OsStr>>();

        let status = self
            .options
            .executor
            .as_ref()
            // It's okay to call unwrap() here because new() ensures it's initialized.
            .unwrap()
            .run(OsStr::new(program), &args)
            .await;
        match status {
            Ok(az_output) if az_output.status.success() => {
                let output = str::from_utf8(&az_output.stdout)?;

                let access_token = from_json(output)?;
                Ok(access_token)
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

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let tr = self.get_access_token(scopes).await?;
        let expires_on = tr.expires_on()?;
        Ok(AccessToken::new(tr.access_token, expires_on))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AzureCliCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.get_token(scopes).await
    }
}

impl From<TokenCredentialOptions> for AzureCliCredentialOptions {
    fn from(options: TokenCredentialOptions) -> Self {
        Self {
            executor: Some(options.executor.clone()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "old_azure_cli")]
    use serial_test::serial;
    #[cfg(feature = "old_azure_cli")]
    use time::macros::datetime;

    #[cfg(feature = "old_azure_cli")]
    #[test]
    #[serial]
    fn can_parse_expires_on() -> azure_core::Result<()> {
        let expires_on = "2022-07-30 12:12:53.919110";
        assert_eq!(
            az_cli_date_format::parse(expires_on)?,
            az_cli_date_format::assume_local(&datetime!(2022-07-30 12:12:53.919110))
        );

        Ok(())
    }

    #[cfg(all(feature = "old_azure_cli", unix))]
    #[test]
    #[serial]
    /// test the timezone conversion works as expected on unix platforms
    ///
    /// To validate the timezone conversion works as expected, this test
    /// temporarily sets the timezone to PST, performs the check, then resets
    /// the TZ environment variable.
    fn check_timezone() -> azure_core::Result<()> {
        let before = std::env::var("TZ").ok();
        std::env::set_var("TZ", "US/Pacific");
        let expires_on = "2022-11-30 12:12:53.919110";
        let result = az_cli_date_format::parse(expires_on);

        if let Some(before) = before {
            std::env::set_var("TZ", before);
        } else {
            std::env::remove_var("TZ");
        }

        let expected = datetime!(2022-11-30 20:12:53.0).assume_utc();
        assert_eq!(expected, result?);

        Ok(())
    }

    /// Test `from_json` for `CliTokenResponse` for old Azure CLI
    #[test]
    fn read_old_cli_token_response() -> azure_core::Result<()> {
        let json = br#"
        {
            "accessToken": "MuchLonger_NotTheRealOne_Sv8Orn0Wq0OaXuQEg",
            "expiresOn": "2024-01-01 19:23:16.000000",
            "subscription": "33b83be5-faf7-42ea-a712-320a5f9dd111",
            "tenant": "065e9f5e-870d-4ed1-af2b-1b58092353f3",
            "tokenType": "Bearer"
          }
        "#;
        let token_response: CliTokenResponse = from_json(json)?;
        assert_eq!(token_response.expires_on, None);
        Ok(())
    }

    /// Test `from_json` for `CliTokenResponse` for current Azure CLI
    #[test]
    fn read_cli_token_response() -> azure_core::Result<()> {
        let json = br#"
        {
            "accessToken": "MuchLonger_NotTheRealOne_Sv8Orn0Wq0OaXuQEg",
            "expiresOn": "2024-01-01 19:23:16.000000",
            "expires_on": 1704158596,
            "subscription": "33b83be5-faf7-42ea-a712-320a5f9dd111",
            "tenant": "065e9f5e-870d-4ed1-af2b-1b58092353f3",
            "tokenType": "Bearer"
        }
        "#;
        let token_response: CliTokenResponse = from_json(json)?;
        assert_eq!(token_response.expires_on, Some(1704158596));
        assert_eq!(token_response.expires_on()?.unix_timestamp(), 1704158596);
        Ok(())
    }
}
