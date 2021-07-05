#[cfg(not(target_arch = "wasm32"))]
use crate::policies::TransportPolicy;
use crate::policies::{Policy, TelemetryPolicy};
use crate::{ClientOptions, Error, HttpClient, PipelineContext, Request, Response};
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
/// 7. Transport policy. Transport policy is always the last policy and is the policy that
///    actually constructs the `Response` to be passed up the pipeline.
///
/// A pipeline is immutable. In other words a policy can either succeed and call the following
/// policy of fail and return to the calling policy. Arbitrary policy "skip" must be avoided (but
/// cannot be enforced by code). All policies except Transport policy can assume there is another following policy (so
/// self.pipeline[0] is always valid).
#[derive(Debug, Clone)]
pub struct Pipeline<R>
where
    R: Send + Sync,
{
    http_client: Arc<dyn HttpClient>,
    pipeline: Vec<Arc<dyn Policy<R>>>,
}

impl<R> Pipeline<R>
where
    R: Send + Sync,
{
    /// Creates a new pipeline given the client library crate name and version,
    /// alone with user-specified and client library-specified policies.
    ///
    /// Crates can simply pass `option_env!("CARGO_PKG_NAME")` and `option_env!("CARGO_PKG_VERSION")` for the
    /// `crate_name` and `crate_version` arguments respectively.
    pub fn new(
        crate_name: Option<&'static str>,
        crate_version: Option<&'static str>,
        options: &ClientOptions<R>,
        per_call_policies: Vec<Arc<dyn Policy<R>>>,
        per_retry_policies: Vec<Arc<dyn Policy<R>>>,
    ) -> Self {
        let mut pipeline: Vec<Arc<dyn Policy<R>>> = Vec::with_capacity(
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

        let retry_policy = options.retry.to_policy();
        pipeline.push(retry_policy);

        pipeline.extend_from_slice(&per_retry_policies);
        pipeline.extend_from_slice(&options.per_retry_policies);

        // TODO: Add transport policy for WASM once https://github.com/Azure/azure-sdk-for-rust/issues/293 is resolved.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let transport_policy = TransportPolicy::new(&options.transport);
            pipeline.push(Arc::new(transport_policy));
        }

        Self {
            http_client: options.transport.http_client.clone(),
            pipeline,
        }
    }

    /// Gets the `HttpClient` used by the pipeline.
    pub fn http_client(&self) -> &dyn HttpClient {
        // TODO: Request methods should be defined directly on the pipeline instead of exposing the HttpClient.
        self.http_client.as_ref()
    }

    pub async fn send(
        &self,
        ctx: &mut PipelineContext<R>,
        request: &mut Request,
    ) -> Result<Response, Error> {
        self.pipeline[0]
            .send(ctx, request, &self.pipeline[1..])
            .await
            .map_err(Error::PolicyError)
    }
}
