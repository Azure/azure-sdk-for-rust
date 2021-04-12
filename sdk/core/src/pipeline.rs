use crate::policies::{Policy, PolicyResult, RetryPolicy};
use crate::{Context, Request, Response};
use std::sync::Arc;

#[derive(Clone)]
pub struct Pipeline {
    per_call_policies: Vec<Arc<dyn Policy<Request>>>,
    retry: Arc<dyn RetryPolicy>,
    per_retry_policies: Vec<Arc<dyn Policy<Request>>>,
    transport: Arc<dyn Policy<Response>>,
}

impl std::fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pipeline{{per_call_policies.len() == {}, per_retry_policies.len() == {}}}",
            self.per_call_policies.len(),
            self.per_retry_policies.len(),
        )
    }
}

impl Pipeline {
    pub fn new(
        per_call_policies: Vec<Arc<dyn Policy<Request>>>,
        retry: Arc<dyn RetryPolicy>,
        per_retry_policies: Vec<Arc<dyn Policy<Request>>>,
        transport: Arc<dyn Policy<Response>>,
    ) -> Self {
        Self {
            per_call_policies,
            retry,
            per_retry_policies,
            transport,
        }
    }

    /// The send function performs these steps:
    /// 1. For each per_call_policy execute it and pass the result to the following policy. In case of errors just return it (no retry).
    /// 2. Instantiate a retry policy instance with retry count to zero using the retry policy as
    ///    template.
    /// 3. For each per_retry_policy execute it and pass the result to the following policy. In
    ///    case of errors, ask the retry policy if the pipeline should retry the policies. If yes,
    ///    reset the request at the end of per_call_policies pipeline and execute the
    ///    per_retry_policies again.
    /// 4. Send the request to transport. In case of errors, behave like per_retry_policy invoking
    ///    the retry_policy first then optionally retry.
    ///
    /// The context can be mutated by each policy and is **not** reset between retries. It can be
    /// used to store information with the lifecycle of the whole pipeline execution.
    pub async fn send(&self, ctx: &mut Context, request: Request) -> PolicyResult<Response> {
        // first, we execute the perCall policies. We do not retry these.
        let mut request = request;

        for policy in &self.per_call_policies {
            request = policy.send(ctx, request.clone()).await?;
        }

        // with create instance we make sure every pipeline execution has a new retry policy
        // instance (with for example the counters reset to zero) based off the template policy
        // specified in the pipeline.
        let mut retry_policy = self.retry.create_instance();

        loop {
            // each retry must start with a fresh copy of the request. This ensures idempotency
            // of the pipeline. This is necessary because as the pipeline progresses, the request
            // can be changed by each policy.
            let mut request = request.clone();

            for policy in &self.per_retry_policies {
                let request_result = policy.send(ctx, request.clone()).await;
                if request_result.is_err() {
                    if retry_policy.should_retry().await {
                        continue;
                    } else {
                        return Err(request_result.err().unwrap());
                    }
                } else {
                    request = request_result.unwrap();
                }
            }

            let response = self.transport.send(ctx, request.clone()).await;
            if response.is_err() {
                if retry_policy.should_retry().await {
                    continue;
                } else {
                    return Err(response.err().unwrap());
                }
            }

            return Ok(response.unwrap());
        }
    }
}
