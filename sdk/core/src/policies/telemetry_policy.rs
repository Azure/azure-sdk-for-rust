use crate::policies::{Policy, PolicyResult};
use crate::{Context, Request, Response};

use http::{header::USER_AGENT, HeaderValue};
use rustc_version::{version, Version};
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

const EMPTY_VERSION: Version = Version {
    major: 0,
    minor: 0,
    patch: 0,
    pre: Vec::new(),
    build: Vec::new(),
};

impl TelemetryPolicy {
    pub fn new(options: TelemetryOptions) -> Self {
        let platform_info = format!("({}; {}; {})", version().unwrap_or(EMPTY_VERSION), OS, ARCH);
        if let Some(application_id) = options.application_id {
            TelemetryPolicy {
                header: format!(
                    "{} azsdk-rust-{}/{} {}",
                    application_id,
                    clap::crate_name!(),
                    clap::crate_version!(),
                    platform_info
                ),
            }
        } else {
            TelemetryPolicy {
                header: format!(
                    "azsdk-rust-{}/{} {}",
                    clap::crate_name!(),
                    clap::crate_version!(),
                    platform_info
                ),
            }
        }
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
