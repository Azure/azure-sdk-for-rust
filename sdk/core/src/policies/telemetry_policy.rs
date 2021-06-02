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
        let crate_name = env!("CARGO_PKG_NAME");
        let crate_version = env!("CARGO_PKG_VERSION");
        let platform_info = format!("({}; {}; {})", env!("AZSDK_RUSTC_VERSION"), OS, ARCH,);
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
