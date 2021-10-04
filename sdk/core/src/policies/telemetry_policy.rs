use crate::options::TelemetryOptions;
use crate::policies::{Policy, PolicyResult};
use crate::{PipelineContext, Request, Response};

use http::{header::USER_AGENT, HeaderValue};
use std::env::consts::{ARCH, OS};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TelemetryPolicy {
    header: String,
}

/// Sets the User-Agent header with useful information in a typical format for Azure SDKs.
impl<'a> TelemetryPolicy {
    pub fn new(
        crate_name: Option<&'a str>,
        crate_version: Option<&'a str>,
        options: &TelemetryOptions,
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
        options: &TelemetryOptions,
    ) -> Self {
        const UNKNOWN: &'static str = "unknown";
        let mut crate_name = crate_name.unwrap_or(UNKNOWN);
        let crate_version = crate_version.unwrap_or(UNKNOWN);
        let rustc_version = rustc_version.unwrap_or(UNKNOWN);
        let platform_info = format!("({}; {}; {})", rustc_version, OS, ARCH,);

        if let Some(name) = crate_name.strip_prefix("azure_") {
            crate_name = name;
        }

        let header = match &options.application_id {
            Some(application_id) => format!(
                "{} azsdk-rust-{}/{} {}",
                application_id, crate_name, crate_version, platform_info
            ),
            None => format!(
                "azsdk-rust-{}/{} {}",
                crate_name, crate_version, platform_info
            ),
        };

        TelemetryPolicy { header }
    }
}

#[async_trait::async_trait]
impl<C> Policy<C> for TelemetryPolicy
where
    C: Send + Sync,
{
    async fn send(
        &self,
        ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        request
            .headers_mut()
            .insert(USER_AGENT, HeaderValue::from_str(&self.header)?);

        next[0].send(ctx, request, &next[1..]).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_without_application_id() {
        let policy = TelemetryPolicy::new_with_rustc_version(
            Some("azure_test"), // Tests that "azure_" is removed.
            Some("1.2.3"),
            Some("4.5.6"),
            &TelemetryOptions::default(),
        );
        assert_eq!(
            policy.header,
            format!("azsdk-rust-test/1.2.3 (4.5.6; {}; {})", OS, ARCH)
        );
    }

    #[test]
    fn test_with_application_id() {
        let options = TelemetryOptions {
            application_id: Some("my_app".to_string()),
        };
        let policy = TelemetryPolicy::new_with_rustc_version(
            Some("test"),
            Some("1.2.3"),
            Some("4.5.6"),
            &options,
        );
        assert_eq!(
            policy.header,
            format!("my_app azsdk-rust-test/1.2.3 (4.5.6; {}; {})", OS, ARCH)
        );
    }

    #[test]
    fn test_missing_env() {
        // Would simulate if option_env!("CARGO_PKG_NAME"), for example, returned None.
        let policy =
            TelemetryPolicy::new_with_rustc_version(None, None, None, &TelemetryOptions::default());
        assert_eq!(
            policy.header,
            format!("azsdk-rust-unknown/unknown (unknown; {}; {})", OS, ARCH)
        )
    }
}
