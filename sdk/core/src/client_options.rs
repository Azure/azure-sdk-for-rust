use crate::policies::{Policy, TelemetryOptions};
use std::sync::Arc;

/// Options passed clients to customer policies, telemetry, etc.
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    // TODO: Expose retry options and transport overrides.
    pub per_call_policies: Vec<Arc<dyn Policy>>,
    pub per_retry_policies: Vec<Arc<dyn Policy>>,

    pub telemetry: TelemetryOptions,
}
