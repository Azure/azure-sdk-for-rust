// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    http::{request::Request, HttpClient, RawResponse},
    Result,
};
use futures::{future::BoxFuture, lock::Mutex};
use std::fmt;

/// An [`HttpClient`] from which you can assert [`Request`]s and return mock [`RawResponse`]s.
///
/// # Examples
///
/// ```
/// use azure_core::{
///     http::{headers::Headers, ClientOptions, RawResponse, StatusCode, TransportOptions},
///     Bytes,
/// };
/// use azure_core_test::http::MockHttpClient;
/// use azure_identity::DefaultAzureCredential;
/// use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
/// use futures::FutureExt as _;
/// use std::sync::Arc;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mock_client = Arc::new(MockHttpClient::new(|req| async {
///     assert_eq!(req.url().host_str(), Some("my-vault.vault.azure.net"));
///     Ok(RawResponse::from_bytes(
///         StatusCode::Ok,
///         Headers::new(),
///         Bytes::from_static(br#"{"value":"secret"}"#),
///     ))
/// }.boxed()));
/// let credential = DefaultAzureCredential::new()?;
/// let options = SecretClientOptions {
///     client_options: ClientOptions {
///         transport: Some(TransportOptions::new(mock_client.clone())),
///         ..Default::default()
///     },
///     ..Default::default()
/// };
/// let client = SecretClient::new(
///     "https://my-vault.vault.azure.net",
///     credential.clone(),
///     Some(options),
/// );
/// # Ok(())
/// # }
/// ```
pub struct MockHttpClient<C>(Mutex<C>);

impl<C> MockHttpClient<C>
where
    C: FnMut(&Request) -> BoxFuture<'_, Result<RawResponse>> + Send + Sync,
{
    /// Creates a new `MockHttpClient` using a capture.
    ///
    /// The capture takes a `&Request` and returns a `BoxedFuture<Output = azure_core::Result<Response>>`.
    /// See the example on [`MockHttpClient`].
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
    C: FnMut(&Request) -> BoxFuture<'_, Result<RawResponse>> + Send + Sync,
{
    async fn execute_request(&self, req: &Request) -> Result<RawResponse> {
        let mut client = self.0.lock().await;
        (client)(req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt as _;

    #[tokio::test]
    async fn mock_http_client() {
        use azure_core::http::{
            headers::{HeaderName, Headers},
            Method, StatusCode,
        };
        use std::sync::{Arc, Mutex};

        const COUNT_HEADER: HeaderName = HeaderName::from_static("x-count");

        let count = Arc::new(Mutex::new(0));
        let mock_client = Arc::new(MockHttpClient::new(|req| {
            let count = count.clone();
            async move {
                assert_eq!(req.url().host_str(), Some("localhost"));

                if req.headers().get_optional_str(&COUNT_HEADER).is_some() {
                    let mut count = count.lock().unwrap();
                    *count += 1;
                }

                Ok(RawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    vec![],
                ))
            }
            .boxed()
        })) as Arc<dyn HttpClient>;

        let req = Request::new("https://localhost".parse().unwrap(), Method::Get);
        mock_client.execute_request(&req).await.unwrap();

        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
        req.insert_header(COUNT_HEADER, "true");
        mock_client.execute_request(&req).await.unwrap();

        assert_eq!(*count.lock().unwrap(), 1);
    }
}
