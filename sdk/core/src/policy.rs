use crate::{Request, Response};

use async_trait::async_trait;

use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub type PolicyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Copy, Clone, Debug)]
pub struct RetryOptions {
    num_retries: usize,
}

impl RetryOptions {
    pub fn new(num_retries: usize) -> Self {
        Self { num_retries }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RetryPolicy {
    options: RetryOptions,
}

impl RetryPolicy {
    pub fn new(options: RetryOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl Policy for RetryPolicy {
    async fn send(
        &self,
        ctx: Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        let retries = self.options.num_retries;
        let mut last_result = next[0].send(ctx.clone(), request, &next[1..]).await;
        loop {
            if last_result.is_ok() || retries == 0 {
                return last_result;
            }

            last_result = next[0].send(ctx.clone(), request, &next[1..]).await;
        }
    }
}

type BoxedFuture<T> = Box<dyn Future<Output = PolicyResult<T>> + Send>;
type Transport = dyn Fn(Context, &mut Request) -> Pin<BoxedFuture<Response>> + Send;

pub struct TransportOptions {
    send: Box<Mutex<Transport>>,
}

impl TransportOptions {
    pub fn new<F>(send: F) -> Self
    where
        F: Fn(Context, &mut Request) -> Pin<BoxedFuture<Response>> + Send + 'static,
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
impl Policy for TransportPolicy {
    async fn send(
        &self,
        ctx: Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        if !next.is_empty() {
            panic!("Transport policy was not last policy")
        }
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
pub trait Policy: Send + Sync + std::fmt::Debug {
    async fn send(
        &self,
        ctx: Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response>;
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    policies: Vec<Arc<dyn Policy>>,
}

impl Pipeline {
    // TODO: how can we ensure that the transport policy is the last policy?
    // Make this more idiot proof
    pub fn new(policies: Vec<Arc<dyn Policy>>) -> Self {
        Self { policies }
    }

    pub async fn send(&self, ctx: Context, mut request: Request) -> PolicyResult<Response> {
        self.policies[0]
            .send(ctx, &mut request, &self.policies[1..])
            .await
    }
}
