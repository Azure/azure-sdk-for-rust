#[cfg(not(target_arch = "wasm32"))]
use crate::policies::TransportPolicy;
use crate::policies::{CustomHeadersInjectorPolicy, Policy, TelemetryPolicy};
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
/// 7. Authorization policy. Authorization can depend on the HTTP headers and/or the request body so it
///    must be executed right before sending the request to the transport. Also, the authorization
///    can depend on the current time so it must be executed at every retry.
/// 8. Transport policy. Transport policy is always the last policy and is the policy that
///    actually constructs the `Response` to be passed up the pipeline.
///
/// A pipeline is immutable. In other words a policy can either succeed and call the following
/// policy of fail and return to the calling policy. Arbitrary policy "skip" must be avoided (but
/// cannot be enforced by code). All policies except Transport policy can assume there is another following policy (so
/// self.pipeline[0] is always valid).
///
/// The `C` generic contains the pipeline-specific context. Different crates can pass
/// different contexts using this generic. This way each crate can have its own specific pipeline
/// context. For example, in CosmosDB, the generic carries the operation-specific information used by
/// the authorization policy.
#[derive(Debug, Clone)]
pub struct Pipeline<C>
where
    C: Send + Sync,
{
    http_client: Arc<dyn HttpClient>,
    pipeline: Vec<Arc<dyn Policy<C>>>,
}

impl<C> Pipeline<C>
where
    C: Send + Sync,
{
    /// Creates a new pipeline given the client library crate name and version,
    /// alone with user-specified and client library-specified policies.
    ///
    /// Crates can simply pass `option_env!("CARGO_PKG_NAME")` and `option_env!("CARGO_PKG_VERSION")` for the
    /// `crate_name` and `crate_version` arguments respectively.
    pub fn new(
        crate_name: Option<&'static str>,
        crate_version: Option<&'static str>,
        options: ClientOptions<C>,
        per_call_policies: Vec<Arc<dyn Policy<C>>>,
        per_retry_policies: Vec<Arc<dyn Policy<C>>>,
    ) -> Self {
        let mut pipeline: Vec<Arc<dyn Policy<C>>> = Vec::with_capacity(
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

        pipeline.push(Arc::new(CustomHeadersInjectorPolicy::default()));

        let retry_policy = options.retry.to_policy();
        pipeline.push(retry_policy);

        pipeline.extend_from_slice(&per_retry_policies);
        pipeline.extend_from_slice(&options.per_retry_policies);
        let http_client = options.transport.http_client.clone();

        // TODO: Add transport policy for WASM once https://github.com/Azure/azure-sdk-for-rust/issues/293 is resolved.
        #[cfg(not(target_arch = "wasm32"))]
        {
            #[allow(unused_mut)]
            let mut policy: Arc<dyn Policy<_>> =
                Arc::new(TransportPolicy::new(options.transport.clone()));

            // This code replaces the default transport policy at runtime if these two conditions
            // are met:
            // 1. The mock_transport_framework is enabled
            // 2. The environmental variable TESTING_MODE is either RECORD or PLAY
            #[cfg(feature = "mock_transport_framework")]
            match std::env::var(crate::TESTING_MODE_KEY)
                .as_deref()
                .unwrap_or(crate::TESTING_MODE_REPLAY)
            {
                crate::TESTING_MODE_RECORD => {
                    log::warn!("mock testing framework record mode enabled");
                    policy = Arc::new(crate::policies::MockTransportRecorderPolicy::new(
                        options.transport,
                    ))
                }
                crate::TESTING_MODE_REPLAY => {
                    log::info!("mock testing framework replay mode enabled");
                    policy = Arc::new(crate::policies::MockTransportPlayerPolicy::new(
                        options.transport,
                    ))
                }
                m => {
                    log::error!(
                        "invalid TESTING_MODE '{}' selected. Supported options are '{}' and '{}'",
                        m,
                        crate::TESTING_MODE_RECORD,
                        crate::TESTING_MODE_REPLAY
                    );
                }
            };

            pipeline.push(policy);
        }

        Self {
            http_client,
            pipeline,
        }
    }

    /// Gets the `HttpClient` used by the pipeline.
    pub fn http_client(&self) -> &dyn HttpClient {
        // TODO: Request methods should be defined directly on the pipeline instead of exposing the HttpClient.
        self.http_client.as_ref()
    }

    pub fn replace_policy(
        &mut self,
        policy: Arc<dyn Policy<C>>,
        position: usize,
    ) -> Arc<dyn Policy<C>> {
        std::mem::replace(&mut self.pipeline[position], policy)
    }

    pub fn policies(&self) -> &[Arc<dyn Policy<C>>] {
        &self.pipeline
    }

    pub async fn send(
        &self,
        ctx: &mut PipelineContext<C>,
        request: &mut Request,
    ) -> Result<Response, Error> {
        self.pipeline[0]
            .send(ctx, request, &self.pipeline[1..])
            .await
            .map_err(Error::PolicyError)
    }
}
