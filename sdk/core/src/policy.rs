use crate::retry_policy::RetryPolicy;
use crate::Request;
use crate::Response;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub type PolicyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

type BoxedFuture<T> = Box<dyn Future<Output = PolicyResult<T>> + Send>;
type Transport = dyn Fn(Context, Request) -> Pin<BoxedFuture<Response>> + Send;

pub struct TransportOptions {
    send: Box<Mutex<Transport>>,
}

impl TransportOptions {
    pub fn new<F>(send: F) -> Self
    where
        F: Fn(Context, Request) -> Pin<BoxedFuture<Response>> + Send + 'static,
    {
        Self {
            send: Box::new(Mutex::new(send)),
        }
    }
}

impl std::fmt::Debug for TransportOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TransportOptions")
    }
}

#[derive(Debug)]
pub struct TransportPolicy {
    options: TransportOptions,
}

impl TransportPolicy {
    pub fn new(options: TransportOptions) -> Self {
        Self { options }
    }
}

#[async_trait::async_trait]
impl Policy<Response> for TransportPolicy {
    async fn send(&self, ctx: Context, request: Request) -> PolicyResult<Response> {
        let response = {
            let transport = self.options.send.lock().unwrap();
            (transport)(ctx, request)
        };
        Ok(response.await?)
    }
}

#[derive(Clone)]
pub struct Context {
    // Temporary hack to make sure that Context is not initializeable
    // Soon Context will have proper data fields
    _priv: (),
}

impl Context {
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

#[async_trait::async_trait]
pub trait Policy<R>: Send + Sync + std::fmt::Debug
where
    R: Send + Sync,
{
    async fn send(&self, ctx: Context, request: Request) -> PolicyResult<R>;
}

#[derive(Clone)]
pub struct Pipeline {
    policies: Vec<Arc<dyn Policy<Request>>>,
    retry: Arc<dyn RetryPolicy>,
    transport: Arc<dyn Policy<Response>>,
}

impl Pipeline {
    pub fn new(
        policies: Vec<Arc<dyn Policy<Request>>>,
        retry: Arc<dyn RetryPolicy>,
        transport: Arc<dyn Policy<Response>>,
    ) -> Self {
        Self {
            policies,
            retry,
            transport,
        }
    }

    pub async fn send(&self, ctx: Context, request: Request) -> PolicyResult<Response> {
        // with create instance we make sure every pipeline execution has a new retry policy
        // instance (with for example the counters reset to zero) based off the template policy
        // specified in the pipeline.
        let mut retry_policy = self.retry.create_instance().await;

        loop {
            // each retry must start with a fresh copy of the request. This ensures idempotency
            // of the pipeline. This is necessary because as the pipeline progresses, the request
            // can be changed by each policy.
            let mut request = request.clone();

            for policy in &self.policies {
                let request_result = policy.send(ctx.clone(), request.clone()).await;
                if request_result.is_err() {
                    if retry_policy.retry().await {
                        continue;
                    } else {
                        return Err(request_result.err().unwrap());
                    }
                } else {
                    request = request_result.unwrap();
                }
            }

            let response = self.transport.send(ctx.clone(), request.clone()).await;
            if response.is_err() {
                if retry_policy.retry().await {
                    continue;
                } else {
                    return Err(response.err().unwrap());
                }
            }

            return Ok(response.unwrap());
        }
    }
}
