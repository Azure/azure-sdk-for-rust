// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{HttpClient, Request, Response, Result};
use futures::lock::Mutex;
use std::{fmt, future::Future, pin::Pin};

pub struct MockHttpClient<C>(Mutex<C>);

impl<C> MockHttpClient<C>
where
    C: FnMut(&Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send + Sync + '_>>
        + Send
        + Sync,
{
    pub fn new(client: C) -> Self {
        Self(Mutex::new(client))
    }
}

impl<C> fmt::Debug for MockHttpClient<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(stringify!("MockHttpClient"))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<C> HttpClient for MockHttpClient<C>
where
    C: FnMut(&Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send + Sync + '_>>
        + Send
        + Sync,
{
    async fn execute_request(&self, req: &Request) -> Result<Response> {
        let mut client = self.0.lock().await;
        (client)(req).await
    }
}

#[tokio::test]
async fn test_mock_http_client() {
    use azure_core::{
        headers::{HeaderName, Headers},
        Method, StatusCode,
    };
    use std::sync::{Arc, Mutex};

    const COUNT_HEADER: HeaderName = HeaderName::from_static("x-count");

    let count = Arc::new(Mutex::new(0));
    let mock_client = Arc::new(MockHttpClient::new(|req| {
        let count = count.clone();
        Box::pin(async move {
            assert_eq!(req.url().host_str(), Some("localhost"));

            if req.headers().get_optional_str(&COUNT_HEADER).is_some() {
                let mut count = count.lock().unwrap();
                *count += 1;
            }

            Ok(Response::from_bytes(StatusCode::Ok, Headers::new(), vec![]))
        })
    })) as Arc<dyn HttpClient>;

    let req = Request::new("https://localhost".parse().unwrap(), Method::Get);
    mock_client.execute_request(&req).await.unwrap();

    let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
    req.insert_header(COUNT_HEADER, "true");
    mock_client.execute_request(&req).await.unwrap();

    assert_eq!(*count.lock().unwrap(), 1);
}
