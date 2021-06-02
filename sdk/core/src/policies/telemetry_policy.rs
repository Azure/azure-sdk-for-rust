use crate::policies::{Policy, PolicyResult};
use crate::{Context, Request, Response};

use http::{header::USER_AGENT, HeaderValue};
use std::env::consts::{ARCH, OS};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct TelemetryOptions {
    application_id: Option<String>,
}

impl TelemetryOptions {
    pub fn new(application_id: Option<String>) -> Self {
        Self { application_id }
    }
}

#[derive(Clone, Debug)]
pub struct TelemetryPolicy {
    header: String,
}

impl TelemetryPolicy {
    pub fn new(options: TelemetryOptions) -> Self {
        Self::with_environment::<Env>(options)
    }

    fn with_environment<T: Environment>(options: TelemetryOptions) -> Self {
        const UNKNOWN: &'static str = "unknown";
        let crate_name = T::crate_name().unwrap_or(UNKNOWN);
        let crate_version = T::crate_version().unwrap_or(UNKNOWN);
        let rustc_version = T::rustc_version().unwrap_or(UNKNOWN);
        let platform_info = format!("({}; {}; {})", rustc_version, OS, ARCH,);
        let header = match options.application_id {
            Some(application_id) => format!(
                "{} azsdk-rust-{}/{} {}",
                application_id, crate_name, crate_version, platform_info
            ),
            None => format!(
                "azsdk-rust-{}/{} {}",
                crate_name, crate_version, platform_info
            ),
        };

        TelemetryPolicy { header: header }
    }
}

impl Default for TelemetryPolicy {
    fn default() -> Self {
        TelemetryPolicy::new(TelemetryOptions::default())
    }
}

trait Environment {
    fn crate_name() -> Option<&'static str> {
        option_env!("CARGO_PKG_NAME")
    }

    fn crate_version() -> Option<&'static str> {
        option_env!("CARGO_PKG_VERSION")
    }

    fn rustc_version() -> Option<&'static str> {
        option_env!("AZSDK_RUSTC_VERSION")
    }
}

struct Env;
impl Environment for Env {}

#[async_trait::async_trait]
impl Policy for TelemetryPolicy {
    async fn send(
        &self,
        ctx: &mut Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        request
            .headers_mut()
            .insert(USER_AGENT, HeaderValue::from_str(&self.header).unwrap());

        next[0].send(ctx, request, &next[1..]).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // tests assume cargo + rustc
    const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
    const CRATE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const RUSTC_VERSION: &'static str = env!("AZSDK_RUSTC_VERSION");

    struct EmptyEnv;
    impl Environment for EmptyEnv {
        fn crate_name() -> Option<&'static str> {
            None
        }

        fn crate_version() -> Option<&'static str> {
            None
        }

        fn rustc_version() -> Option<&'static str> {
            None
        }
    }

    #[test]
    fn test_default() {
        let policy = TelemetryPolicy::default();
        assert_eq!(
            policy.header,
            format!(
                "azsdk-rust-{}/{} ({}; {}; {})",
                CRATE_NAME, CRATE_VERSION, RUSTC_VERSION, OS, ARCH
            )
        );
    }

    #[test]
    fn test_with_application_id() {
        let options = TelemetryOptions::new(Some("test".to_string()));
        let policy = TelemetryPolicy::new(options);
        assert_eq!(
            policy.header,
            format!(
                "test azsdk-rust-{}/{} ({}; {}; {})",
                CRATE_NAME, CRATE_VERSION, RUSTC_VERSION, OS, ARCH
            )
        );
    }

    #[test]
    fn test_missing_env() {
        let policy = TelemetryPolicy::with_environment::<EmptyEnv>(TelemetryOptions::default());
        assert_eq!(
            policy.header,
            format!("azsdk-rust-unknown/unknown (unknown; {}; {})", OS, ARCH)
        )
    }
}
