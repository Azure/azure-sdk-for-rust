use async_trait::async_trait;
use bytes::Bytes;

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
        request: http::Request<Bytes>,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<http::Response<Bytes>> {
        let retries = self.options.num_retries;
        // TODO loop
        if retries == 0 {
            // TODO: return last error
            return Err(":-(".into());
        }
        next[0].send(ctx.clone(), request, &next[1..]).await
    }
}

type BoxedFuture<T> = Box<dyn Future<Output = PolicyResult<T>> + Send>;
type Transport =
    dyn Fn(Context, http::Request<Bytes>) -> Pin<BoxedFuture<http::Response<Bytes>>> + Send;

pub struct TransportOptions {
    send: Box<Mutex<Transport>>,
}

impl TransportOptions {
    pub fn new<F>(send: F) -> Self
    where
        F: Fn(Context, http::Request<Bytes>) -> Pin<BoxedFuture<http::Response<Bytes>>>
            + Send
            + 'static,
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
        request: http::Request<bytes::Bytes>,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<http::Response<bytes::Bytes>> {
        if !next.is_empty() {
            panic!("Transport policy was not last policy")
        }
        let future = {
            let transport = self.options.send.lock().unwrap();
            (transport)(ctx, request)
        };
        Ok(future.await?)
    }
}

#[derive(Clone)]
pub struct Context;

#[async_trait::async_trait]
pub trait Policy: Send + Sync + std::fmt::Debug {
    async fn send(
        &self,
        ctx: Context,
        request: http::Request<Bytes>,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<http::Response<Bytes>>;
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    policies: Vec<Arc<dyn Policy>>,
}

impl Pipeline {
    // TODO: how can we ensure that the transport policy is the last policy?
    pub fn new(policies: Vec<Arc<dyn Policy>>) -> Self {
        Self { policies }
    }

    pub async fn send(
        &self,
        ctx: Context,
        request: http::Request<Bytes>,
    ) -> PolicyResult<http::Response<Bytes>> {
        self.policies[0]
            .send(ctx, request, &self.policies[1..])
            .await
    }
}

struct CosmosClient {
    pipeline: Pipeline,
    url: url::Url,
}

// struct Credentials;
// struct CosmosOptions {
//     retry: RetryOptions,
//     // For example: TransportOptions {
//     //     send: Box::new(|ctx: Context, request: http::Request<bytes::Bytes>| todo!()),
//     // },
//     transport: TransportOptions,
// }

// impl CosmosClient {
//     fn new(url: url::Url, credential: Credentials, options: CosmosOptions) -> Self {
//         let mut policies = Vec::new();
//         let retry_policy = RetryPolicy {
//             options: options.retry,
//         };
//         policies.push(Arc::new(retry_policy) as Arc<dyn Policy>);
//         let transport_policy = TransportPolicy {
//             options: options.transport,
//         };
//         policies.push(Arc::new(transport_policy) as Arc<dyn Policy>);
//         let pipeline = Pipeline { policies };

//         Self { pipeline, url }
//     }

//     pub fn create_collection(
//         &self,
//         ctx: Context, /* Argument */
//     ) -> PolicyResult<CreateCollectionResult> {
//         // Create the http request option
//         // Url, headers, query params, payload body, etc.
//         let request = todo!();
//         let response = self.pipeline.send(ctx, request);
//         todo!("Deserialize headers and body into CreateCollectionResult")
//     }
// }

// fn main() {
//     let credentials = todo!();
//     let url = todo!();
//     let client = CosmosClient::new(url, credentials, CosmosOptions::default());
//     let ctx = todo!();
//     client.create_collection(ctx)
// }

// struct CreateCollectionResult;
