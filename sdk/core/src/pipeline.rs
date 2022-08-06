use crate::policies::TransportPolicy;
use crate::policies::{CustomHeadersPolicy, Policy, TelemetryPolicy};
use crate::{ClientOptions, Context, Request, Response};
use std::sync::Arc;

/// Execution pipeline.
///
/// A pipeline follows a precise flow:
///
/// 1. Client library-specified per-call policies are executed. Per-call policies can fail and bail out of the pipeline
///    immediately.
/// 2. User-specified per-call policies are executed.
/// 3. Telemetry policy.
/// 4. Retry policy. It allows to re-execute the following policies.
/// 5. Client library-specified per-retry policies. Per-retry polices are always executed at least once but are re-executed
///    in case of retries.
/// 6. User-specified per-retry policies are executed.
/// 7. Authorization policy. Authorization can depend on the HTTP headers and/or the request body so it
///    must be executed right before sending the request to the transport. Also, the authorization
///    can depend on the current time so it must be executed at every retry.
/// 8. Transport policy. Transport policy is always the last policy and is the policy that
///    actually constructs the `Response` to be passed up the pipeline.
///
/// A pipeline is immutable. In other words a policy can either succeed and call the following
/// policy of fail and return to the calling policy. Arbitrary policy "skip" must be avoided (but
/// cannot be enforced by code). All policies except Transport policy can assume there is another following policy (so
/// `self.pipeline[0]` is always valid).
///
/// The `C` generic contains the pipeline-specific context. Different crates can pass
/// different contexts using this generic. This way each crate can have its own specific pipeline
/// context. For example, in `CosmosDB`, the generic carries the operation-specific information used by
/// the authorization policy.
#[derive(Debug, Clone)]
pub struct Pipeline {
    pipeline: Vec<Arc<dyn Policy>>,
}

impl Pipeline {
    /// Creates a new pipeline given the client library crate name and version,
    /// alone with user-specified and client library-specified policies.
    ///
    /// Crates can simply pass `option_env!("CARGO_PKG_NAME")` and `option_env!("CARGO_PKG_VERSION")` for the
    /// `crate_name` and `crate_version` arguments respectively.
    pub fn new(
        crate_name: Option<&'static str>,
        crate_version: Option<&'static str>,
        options: ClientOptions,
        per_call_policies: Vec<Arc<dyn Policy>>,
        per_retry_policies: Vec<Arc<dyn Policy>>,
    ) -> Self {
        let mut pipeline: Vec<Arc<dyn Policy>> = Vec::with_capacity(
            options.per_call_policies.len()
                + per_call_policies.len()
                + options.per_retry_policies.len()
                + per_retry_policies.len()
                + 3,
        );

        pipeline.extend_from_slice(&per_call_policies);
        pipeline.extend_from_slice(&options.per_call_policies);

        let telemetry_policy = TelemetryPolicy::new(crate_name, crate_version, &options.telemetry);
        pipeline.push(Arc::new(telemetry_policy));

        pipeline.push(Arc::new(CustomHeadersPolicy::default()));

        let retry_policy = options.retry.to_policy();
        pipeline.push(retry_policy);

        pipeline.extend_from_slice(&per_retry_policies);
        pipeline.extend_from_slice(&options.per_retry_policies);

        let transport: Arc<dyn Policy> = Arc::new(TransportPolicy::new(options.transport.clone()));

        pipeline.push(transport);

        Self { pipeline }
    }

    pub fn replace_policy(&mut self, policy: Arc<dyn Policy>, position: usize) -> Arc<dyn Policy> {
        std::mem::replace(&mut self.pipeline[position], policy)
    }

    pub fn policies(&self) -> &[Arc<dyn Policy>] {
        &self.pipeline
    }

    pub async fn send(&self, ctx: &mut Context, request: &mut Request) -> crate::Result<Response> {
        self.pipeline[0]
            .send(ctx, request, &self.pipeline[1..])
            .await
    }
}
