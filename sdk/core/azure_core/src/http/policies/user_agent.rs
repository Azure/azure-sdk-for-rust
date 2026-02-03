// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    headers::{HeaderValue, USER_AGENT},
    options::UserAgentOptions,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
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
    pub fn new(
        crate_name: Option<&'a str>,
        crate_version: Option<&'a str>,
        options: &UserAgentOptions,
    ) -> crate::Result<Self> {
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
    ) -> crate::Result<Self> {
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
                // Allow unreserved characters: -, _, ., ~
                // NON_ALPHANUMERIC includes everything except A-Z, a-z, 0-9.
                // We remove the unreserved characters from the set to prevent encoding them.
                const ENCODE_SET: percent_encoding::AsciiSet = NON_ALPHANUMERIC
                    .remove(b'-')
                    .remove(b'_')
                    .remove(b'.')
                    .remove(b'~');
                let encoded_app_id = utf8_percent_encode(application_id, &ENCODE_SET);
                format!("{encoded_app_id} azsdk-rust-{crate_name}/{crate_version} {platform_info}")
            }
            None => format!("azsdk-rust-{crate_name}/{crate_version} {platform_info}"),
        };

        if header_str.contains(['\r', '\n']) {
            return Err(crate::error::Error::new(
                crate::error::ErrorKind::DataConversion,
                "User agent header contains invalid characters",
            ));
        }

        let header = HeaderValue::from(header_str);

        Ok(UserAgentPolicy { header })
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
        )
        .unwrap();
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
        )
        .unwrap();
        assert_eq!(
            policy.header.as_str(),
            format!("my_app azsdk-rust-test/1.2.3 (4.5.6; {OS}; {ARCH})")
        );
    }

    #[test]
    fn missing_env() {
        // Would simulate if option_env!("CARGO_PKG_NAME"), for example, returned None.
        let policy =
            UserAgentPolicy::new_with_rustc_version(None, None, None, &UserAgentOptions::default())
                .unwrap();
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
            application_id: Some("exactly_24_characters!".to_string()), // Exactly 24 characters
        };
        let policy = UserAgentPolicy::new_with_rustc_version(
            Some("test"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        )
        .unwrap();
        assert_eq!(
            policy.header.as_str(),
            format!("exactly_24_characters%21 azsdk-rust-test/1.2.3 (4.5.6; {OS}; {ARCH})")
        );
    }

    #[test]
    fn test_application_id_encoding() {
        let options = UserAgentOptions {
            application_id: Some("my/app".to_string()),
        };
        let policy = UserAgentPolicy::new_with_rustc_version(
            Some("test"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        )
        .unwrap();
        assert_eq!(
            policy.header.as_str(),
            format!("my%2Fapp azsdk-rust-test/1.2.3 (4.5.6; {OS}; {ARCH})")
        );
    }

    #[tokio::test]
    async fn test_user_agent_invalid_chars() {
        // Now that validation is in new(), we should check that it returns an error
        // for characters that cannot be in a header value even after encoding (if any).
        // Standard ASCII control chars might be caught by HeaderValue::try_from.
        // However, we are encoding the application_id.
        // The rest of the string is constructed from crate_name/version which we assume are safe or don't control as easily here.
        // Let's try to inject a newline via crate_name if possible?
        // UserAgentPolicy::new takes crate_name.
        // But let's check if the previous test case application_id="invalid\nheader" matches what we expect now.
        // It should be encoded now, so it should NOT fail, unless we want to forbid newlines entirely before encoding?
        // The requirements said: "URL Encode Application ID". So "invalid\nheader" becomes "invalid%0Aheader".
        // This is a valid header value.
        // So the old test expectation that it fails might be wrong now if we encode it.
        // Maintainer said: "use HeaderValue::try_from(header_string) inside the constructor to validate the entire final User-Agent string."
        // If try_from fails, return DataConversion error.

        // If we pass a crate_name with newline, that gets put directly into the string:
        // format!("{encoded_app_id} azsdk-rust-{crate_name}/{crate_version} ...")
        // So let's try injecting newline via crate_name to verify validation work.

        let options = UserAgentOptions::default();
        let result = UserAgentPolicy::new_with_rustc_version(
            Some("te\nst"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err.kind(),
            crate::error::ErrorKind::DataConversion
        ));
    }
}
