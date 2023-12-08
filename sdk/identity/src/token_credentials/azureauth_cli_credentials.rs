use azure_core::{
    auth::{AccessToken, TokenCredential, TokenResponse},
    error::{Error, ErrorKind, ResultExt},
};
use oauth2::ClientId;
use serde::Deserialize;
use std::process::Command;
use std::str;
use time::OffsetDateTime;

mod unix_date_string {
    use azure_core::error::{Error, ErrorKind};
    use serde::{Deserialize, Deserializer};
    use time::OffsetDateTime;

    pub fn parse(s: &str) -> azure_core::Result<OffsetDateTime> {
        let as_i64 = s.parse().map_err(|_| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("unable to parse expiration_date '{s}")
            })
        })?;

        OffsetDateTime::from_unix_timestamp(as_i64).map_err(|_| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("unable to parse expiration_date '{s}")
            })
        })
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
struct CliTokenResponse {
    pub token: AccessToken,
    #[serde(with = "unix_date_string")]
    pub expiration_date: OffsetDateTime,
}

/// Authentication Mode
///
/// Note: While the azureauth CLI supports devicecode, users wishing to use
/// devicecode should use `azure_identity::device_code_flow`
#[derive(Debug, Clone, Copy)]
pub enum AzureauthCliMode {
    All,
    IntegratedWindowsAuth,
    Broker,
    Web,
}

/// Enables authentication to Azure Active Directory using Azure CLI to obtain an access token.
pub struct AzureauthCliCredential {
    tenant_id: String,
    client_id: ClientId,
    modes: Vec<AzureauthCliMode>,
    prompt_hint: Option<String>,
    cache: TokenCache,
}

impl AzureauthCliCredential {
    /// Create a new `AzureCliCredential`
    pub fn new<T, C>(tenant_id: T, client_id: C) -> Self
    where
        T: Into<String>,
        C: Into<ClientId>,
    {
        Self {
            tenant_id: tenant_id.into(),
            client_id: client_id.into(),
            modes: Vec::new(),
            prompt_hint: None,
            cache: TokenCache,
        }
    }

    pub fn add_mode(mut self, mode: AzureauthCliMode) -> Self {
        self.modes.push(mode);
        self
    }

    pub fn with_modes(mut self, modes: Vec<AzureauthCliMode>) -> Self {
        self.modes = modes;
        self
    }

    pub fn with_prompt_hint<S>(mut self, hint: S) -> Self
    where
        S: Into<String>,
    {
        self.prompt_hint = Some(hint.into());
        self
    }

    fn get_access_token(&self, resource: &str) -> azure_core::Result<CliTokenResponse> {
        // try using azureauth.exe first, such that azureauth through WSL is
        // used first if possible.
        let (cmd_name, use_windows_features) = if Command::new("azureauth.exe")
            .arg("--version")
            .output()
            .map(|x| x.status.success())
            .unwrap_or(false)
        {
            ("azureauth.exe", true)
        } else {
            ("azureauth", false)
        };

        let mut resource = resource.to_owned();
        if !resource.ends_with("/.default") {
            if resource.ends_with('/') {
                resource.push_str(".default");
            } else {
                resource.push_str("/.default");
            }
        }

        let mut cmd = Command::new(cmd_name);
        cmd.args([
            "aad",
            "--scope",
            &resource,
            resource,
            "--client",
            self.client_id.as_str(),
            "--tenant",
            self.tenant_id.as_str(),
            "--output",
            "json",
        ]);

        if let Some(prompt_hint) = &self.prompt_hint {
            cmd.args(["--prompt-hint", prompt_hint]);
        }

        for mode in &self.modes {
            match mode {
                AzureauthCliMode::All => {
                    cmd.args(["--mode", "all"]);
                }
                AzureauthCliMode::IntegratedWindowsAuth => {
                    if use_windows_features {
                        cmd.args(["--mode", "iwa"]);
                    }
                }
                AzureauthCliMode::Broker => {
                    if use_windows_features {
                        cmd.args(["--mode", "broker"]);
                    }
                }
                AzureauthCliMode::Web => {
                    cmd.args(["--mode", "web"]);
                }
            };
        }

        let result = cmd.output();

        let output = result.map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => {
                Error::message(ErrorKind::Other, "azureauth CLI not installed")
            }
            error_kind => Error::with_message(ErrorKind::Other, || {
                format!("Unknown error of kind: {error_kind:?}")
            }),
        })?;

        if !output.status.success() {
            let output = String::from_utf8_lossy(&output.stderr);
            return Err(Error::with_message(ErrorKind::Credential, || {
                format!("'azureauth' command failed: {output}")
            }));
        }

        let token_response: CliTokenResponse = from_json(output.stdout)?;

        Ok(TokenResponse::new(
            token_response.token,
            token_response.expiration_date,
        ))
    }

    /// Clear the azureauth cache as well as the internal cache
    fn clear_cache(&self) -> azure_core::Result<CliTokenResponse> {
        let resources = { self.cache.read().await.keys().cloned().collect::<Vec<_>>() };

        // try using azureauth.exe first, such that azureauth through WSL is
        // used first if possible.
        let (cmd_name) = if Command::new("azureauth.exe")
            .arg("--version")
            .output()
            .map(|x| x.status.success())
            .unwrap_or(false)
        {
            "azureauth.exe"
        } else {
            "azureauth"
        };

        for resource in resources {
            let mut resource = resource.to_owned();
            if !resource.ends_with("/.default") {
                if resource.ends_with('/') {
                    resource.push_str(".default");
                } else {
                    resource.push_str("/.default");
                }
            }

            let mut cmd = Command::new(cmd_name);
            cmd.args([
                "aad",
                "--scope",
                &resource,
                "--client",
                self.client_id.as_str(),
                "--tenant",
                self.tenant_id.as_str(),
                "--clear",
            ]);

            if let Some(prompt_hint) = &self.prompt_hint {
                cmd.args(["--prompt-hint", prompt_hint]);
            }

            for mode in &self.modes {
                match mode {
                    AzureauthCliMode::All => {
                        cmd.args(["--mode", "all"]);
                    }
                    AzureauthCliMode::IntegratedWindowsAuth => {
                        if use_windows_features {
                            cmd.args(["--mode", "iwa"]);
                        }
                    }
                    AzureauthCliMode::Broker => {
                        if use_windows_features {
                            cmd.args(["--mode", "broker"]);
                        }
                    }
                    AzureauthCliMode::Web => {
                        cmd.args(["--mode", "web"]);
                    }
                };
            }

            let result = cmd.output();

            let output = result.map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    Error::message(ErrorKind::Other, "azureauth CLI not installed")
                }
                error_kind => Error::with_message(ErrorKind::Other, || {
                    format!("Unknown error of kind: {error_kind:?}")
                }),
            })?;

            if !output.status.success() {
                let output = String::from_utf8_lossy(&output.stderr);
                return Err(Error::with_message(ErrorKind::Credential, || {
                    format!("'azureauth' command failed: {output}")
                }));
            }
        }

        self.cache.clear().await?;

        Ok(())
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AzureauthCliCredential {
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        self.cache
            .get_token(resource, self.get_access_token(resource))
            .await
    }
    async fn clear_cache(&self) -> azure_core::Result<TokenResponse> {
        self.cache.clear().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() -> azure_core::Result<()> {
        let src = r#"{
            "user": "example@contoso.com",
            "display_name": "Example User",
            "token": "security token here",
            "expiration_date": "1700166595"
        }"#;

        let response: CliTokenResponse = from_json(src)?;
        assert_eq!(response.token.secret(), "security token here");
        assert_eq!(
            response.expiration_date,
            OffsetDateTime::from_unix_timestamp(1700166595).expect("known valid date")
        );

        Ok(())
    }
}
