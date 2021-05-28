use crate::policies::{Policy, PolicyResult};
use crate::{Context, Request, Response};
use std::sync::Arc;

/// Execution pipeline.
///
/// A pipeline follows a precise flow:
///
/// 1. Per call policies are executed. Per call policies can fail and bail out of the pipeline
///    immediately.
/// 2. Retry policy. It allows to reexecute the following policies.
/// 3. Per retry policies. Per retry polices are always executed at least once but are reexecuted
///    in case of retries.
/// 4. Transport policy. Transtport policy is always the last policy and is the policy that
///    actually constructs the `Response` to be passed up the pipeline.
///
/// A pipeline is immutable. In other words a policy can either succeed and call the following
/// policy of fail and return to the calling policy. Arbitrary policy "skip" must be avoided (but
/// cannot be enforced by code). All policies except Transport policy can assume there is another following policy (so
/// self.pipeline[0] is always valid).
#[derive(Debug, Clone)]
pub struct Pipeline {
    pipeline: Vec<Arc<dyn Policy>>,
}

impl Pipeline {
    pub fn new(
        per_call_policies: Vec<Arc<dyn Policy>>,
        retry: Arc<dyn Policy>,
        per_retry_policies: Vec<Arc<dyn Policy>>,
        transport_policy: Arc<dyn Policy>,
    ) -> Self {
        let mut pipeline =
            Vec::with_capacity(per_call_policies.len() + per_retry_policies.len() + 2);

        pipeline.extend_from_slice(&per_call_policies);
        pipeline.push(retry);
        pipeline.extend_from_slice(&per_retry_policies);
        pipeline.push(transport_policy);

        Self { pipeline }
    }

    pub async fn send(&self, ctx: &mut Context, request: &mut Request) -> PolicyResult<Response> {
        self.pipeline[0]
            .send(ctx, request, &self.pipeline[1..])
            .await
    }
}
