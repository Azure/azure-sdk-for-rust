// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    headers::{HeaderValue, USER_AGENT},
    options::UserAgentOptions,
};
use std::{
    env::consts::{ARCH, OS},
    sync::Arc,
};
use typespec_client_core::http::{
    policies::{Policy, PolicyResult},
    Context, Request,
};

/// Sets the `User-Agent` header with useful information in a typical format for Azure SDKs.
#[derive(Clone, Debug)]
pub struct UserAgentPolicy {
    header: HeaderValue,
}

impl<'a> UserAgentPolicy {
    /// Create a new `UserAgentPolicy`.
    ///
    /// # Panics
    ///
    /// Panics if [`UserAgentOptions::application_id`] is greater than 24 characters.
    /// See [guidelines](https://azure.github.io/azure-sdk/general_azurecore.html#azurecore-http-telemetry-appid-length) for details.
    ///
    /// Panics if [`UserAgentOptions::application_id`] contains invalid characters.
    /// Only RFC 9110 "tchar" tokens are allowed.
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

        const MAX_APPLICATION_ID_LEN: usize = 24;
        let header_str = match &options.application_id {
            Some(application_id) => {
                if application_id.len() > MAX_APPLICATION_ID_LEN {
                    panic!(
                        "application_id must be shorter than {} characters",
                        MAX_APPLICATION_ID_LEN + 1
                    );
                }

                // RFC 9110 tchar validation
                // tchar = "!" / "#" / "$" / "%" / "&" / "'" / "*"
                //       / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
                //       / DIGIT / ALPHA
                if !application_id.chars().all(|c| {
                    c.is_ascii_alphanumeric()
                        || matches!(
                            c,
                            '!' | '#'
                                | '$'
                                | '%'
                                | '&'
                                | '\''
                                | '*'
                                | '+'
                                | '-'
                                | '.'
                                | '^'
                                | '_'
                                | '`'
                                | '|'
                                | '~'
                        )
                }) {
                    panic!("application_id contains invalid characters. Only RFC 9110 tokens are allowed.");
                }

                format!("{application_id} azsdk-rust-{crate_name}/{crate_version} {platform_info}")
            }
            None => format!("azsdk-rust-{crate_name}/{crate_version} {platform_info}"),
        };

        let header = HeaderValue::from(header_str);

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
        request.insert_header(USER_AGENT, self.header.clone());
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
            policy.header.as_str(),
            format!("azsdk-rust-test/1.2.3 (4.5.6; {OS}; {ARCH})")
        );
    }

    #[test]
    fn with_application_id() {
        let options = UserAgentOptions {
            application_id: Some("my_app".to_string()),
        };
        let policy = UserAgentPolicy::new_with_rustc_version(
            Some("test"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        );
        assert_eq!(
            policy.header.as_str(),
            format!("my_app azsdk-rust-test/1.2.3 (4.5.6; {OS}; {ARCH})")
        );
    }

    #[test]
    fn missing_env() {
        // Would simulate if option_env!("CARGO_PKG_NAME"), for example, returned None.
        let policy =
            UserAgentPolicy::new_with_rustc_version(None, None, None, &UserAgentOptions::default());
        assert_eq!(
            policy.header.as_str(),
            format!("azsdk-rust-unknown/unknown (unknown; {OS}; {ARCH})")
        );
    }

    #[test]
    #[should_panic(expected = "application_id must be shorter than 25 characters")]
    fn panics_when_application_id_too_long() {
        let options = UserAgentOptions {
            application_id: Some(
                "this_application_id_is_way_too_long_and_exceeds_limit".to_string(),
            ), // 53 characters
        };
        let _policy = UserAgentPolicy::new_with_rustc_version(
            Some("test"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        );
    }

    #[test]
    fn works_with_application_id_at_limit() {
        let options = UserAgentOptions {
            application_id: Some("exactly_24_characters!!!".to_string()), // Exactly 24 characters
        };
        let policy = UserAgentPolicy::new_with_rustc_version(
            Some("test"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        );
        assert_eq!(
            policy.header.as_str(),
            format!("exactly_24_characters!!! azsdk-rust-test/1.2.3 (4.5.6; {OS}; {ARCH})")
        );
    }

    #[test]
    #[should_panic(
        expected = "application_id contains invalid characters. Only RFC 9110 tokens are allowed."
    )]
    fn test_user_agent_invalid_chars() {
        // "Space" is not allowed in tchar.
        let options = UserAgentOptions {
            application_id: Some("invalid application id".to_string()),
        };
        let _policy = UserAgentPolicy::new_with_rustc_version(
            Some("test"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        );
    }
}
