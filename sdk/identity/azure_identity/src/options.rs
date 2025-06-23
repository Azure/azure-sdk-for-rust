// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    env::Env,
    process::{new_executor, Executor},
};
use azure_core::{
    error::{ErrorKind, Result, ResultExt},
    http::{new_http_client, HttpClient, Url},
};
use std::sync::Arc;

const AZURE_AUTHORITY_HOST_ENV_KEY: &str = "AZURE_AUTHORITY_HOST";
const AZURE_PUBLIC_CLOUD: &str = "https://login.microsoftonline.com";

/// Provides options to configure how the Identity library makes authentication
/// requests to Azure Active Directory.
#[derive(Debug, Clone)]
pub struct TokenCredentialOptions {
    pub(crate) env: Env,
    pub(crate) http_client: Arc<dyn HttpClient>,
    pub(crate) authority_host: String,
    pub(crate) executor: Arc<dyn Executor>,
}

/// The default token credential options.
///
/// The authority host is taken from the `AZURE_AUTHORITY_HOST` environment variable if set and a valid URL.
/// If not, the default authority host is `https://login.microsoftonline.com` for the Azure public cloud.
impl Default for TokenCredentialOptions {
    fn default() -> Self {
        let env = Env::default();
        let authority_host = env
            .var(AZURE_AUTHORITY_HOST_ENV_KEY)
            .unwrap_or_else(|_| AZURE_PUBLIC_CLOUD.to_owned());
        Self {
            env: Env::default(),
            http_client: new_http_client(),
            authority_host,
            executor: new_executor(),
        }
    }
}

impl TokenCredentialOptions {
    /// Set the authority host for authentication requests.
    pub fn set_authority_host(&mut self, authority_host: String) {
        self.authority_host = authority_host;
    }

    /// The authority host to use for authentication requests.
    ///
    /// The default is `https://login.microsoftonline.com`.
    pub fn authority_host(&self) -> Result<Url> {
        Url::parse(&self.authority_host).with_context(ErrorKind::DataConversion, || {
            format!("invalid authority host URL {}", &self.authority_host)
        })
    }

    /// The [`HttpClient`] to make requests.
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    /// The [`Executor`] to run commands.
    pub fn executor(&self) -> Arc<dyn Executor> {
        self.executor.clone()
    }

    pub(crate) fn env(&self) -> &Env {
        &self.env
    }
}

impl From<Arc<dyn HttpClient>> for TokenCredentialOptions {
    fn from(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            ..Default::default()
        }
    }
}
