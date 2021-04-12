use crate::policies::{Policy, PolicyResult};
use crate::{BoxedFuture, Context, Request, Response};
use std::pin::Pin;
use std::sync::Mutex;

type Transport = dyn Fn(&mut Context, Request) -> Pin<BoxedFuture<Response>> + Send;

pub struct TransportOptions {
    send: Box<Mutex<Transport>>,
}

impl TransportOptions {
    pub fn new<F>(send: F) -> Self
    where
        F: Fn(&mut Context, Request) -> Pin<BoxedFuture<Response>> + Send + 'static,
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
    async fn send(&self, ctx: &mut Context, request: Request) -> PolicyResult<Response> {
        let response = {
            let transport = self.options.send.lock().unwrap();
            (transport)(ctx, request)
        };
        Ok(response.await?)
    }
}
