// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    headers::{HeaderValue, USER_AGENT},
    options::UserAgentOptions,
};
use std::env::consts::{ARCH, OS};
use std::sync::Arc;
use typespec_client_core::http::policies::{Policy, PolicyResult};
use typespec_client_core::http::{Context, Request};

/// Sets the User-Agent header with useful information in a typical format for Azure SDKs.
#[derive(Clone, Debug)]
pub struct UserAgentPolicy {
    header: String,
}

impl<'a> UserAgentPolicy {
    pub fn new(
        crate_name: Option<&'a str>,
        crate_version: Option<&'a str>,
        options: &UserAgentOptions,
    ) -> Self {
        Self::new_with_rustc_version(
            crate_name,
            crate_version,
            option_env!("AZSDK_RUSTC_VERSION"),
            options,
        )
    }

    fn new_with_rustc_version(
        crate_name: Option<&'a str>,
        crate_version: Option<&'a str>,
        rustc_version: Option<&'a str>,
        options: &UserAgentOptions,
    ) -> Self {
        const UNKNOWN: &str = "unknown";
        let mut crate_name = crate_name.unwrap_or(UNKNOWN);
        let crate_version = crate_version.unwrap_or(UNKNOWN);
        let rustc_version = rustc_version.unwrap_or(UNKNOWN);
        let platform_info = format!("({rustc_version}; {OS}; {ARCH})",);

        if let Some(name) = crate_name.strip_prefix("azure_") {
            crate_name = name;
        }

        let header = match &options.application_id {
            Some(application_id) => {
                format!("{application_id} azsdk-rust-{crate_name}/{crate_version} {platform_info}")
            }
            None => format!("azsdk-rust-{crate_name}/{crate_version} {platform_info}"),
        };

        UserAgentPolicy { header }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for UserAgentPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        request.insert_header(USER_AGENT, HeaderValue::from(self.header.to_string()));

        next[0].send(ctx, request, &next[1..]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_application_id() {
        let policy = UserAgentPolicy::new_with_rustc_version(
            Some("azure_test"), // Tests that "azure_" is removed.
            Some("1.2.3"),
            Some("4.5.6"),
            &UserAgentOptions::default(),
        );
        assert_eq!(
            policy.header,
            format!("azsdk-rust-test/1.2.3 (4.5.6; {OS}; {ARCH})")
        );
    }

    #[test]
    fn with_application_id() {
        let options = UserAgentOptions {
            application_id: Some("my_app".to_string()),
            ..Default::default()
        };
        let policy = UserAgentPolicy::new_with_rustc_version(
            Some("test"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        );
        assert_eq!(
            policy.header,
            format!("my_app azsdk-rust-test/1.2.3 (4.5.6; {OS}; {ARCH})")
        );
    }

    #[test]
    fn missing_env() {
        // Would simulate if option_env!("CARGO_PKG_NAME"), for example, returned None.
        let policy =
            UserAgentPolicy::new_with_rustc_version(None, None, None, &UserAgentOptions::default());
        assert_eq!(
            policy.header,
            format!("azsdk-rust-unknown/unknown (unknown; {OS}; {ARCH})")
        );
    }
}
