// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    http::{
        headers::Headers,
        policies::{Policy, PolicyResult},
        Context, Request, Response, StatusCode,
    },
    Bytes,
};
use std::{fmt, sync::Arc};

/// Mock [`Policy`] useful for testing other policies.
///
/// Use as a terminal [`Policy`] to return a successful empty response by default,
/// or invoke a function for which you can return your [`PolicyResult`].
#[derive(Default)]
pub struct MockPolicy(Option<Box<dyn Fn() -> PolicyResult + Send + Sync>>);

impl fmt::Debug for MockPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_some() {
            return f.write_str("TestPolicy({function})");
        }

        f.write_str("TestPolicy")
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for MockPolicy {
    async fn send(
        &self,
        _ctx: &Context,
        _request: &mut Request,
        _next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let Some(func) = &self.0 else {
            return Ok(Response::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                Bytes::new(),
            ));
        };

        func()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{headers::Headers, Method, Response, StatusCode};
    use crate::Bytes;

    #[tokio::test]
    async fn returns_empty_response_by_default() {
        // Arrange
        let policy = MockPolicy::default();
        let ctx = Context::new();
        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);

        // Act
        let result = policy
            .send(&ctx, &mut request, &[])
            .await
            .expect("Policy execution failed");

        // Assert
        let (status, headers, body) = result.deconstruct();
        assert_eq!(StatusCode::Ok, status);
        assert_eq!(headers.iter().count(), 0);
        assert!(body.collect().await.is_ok_and(|b| b.is_empty()));
    }

    #[tokio::test]
    async fn returns_custom_response() {
        // Arrange
        let headers = Headers::new();
        let body = Bytes::from("test data");
        let status = StatusCode::Created;

        let expected_body = body.clone();
        let policy = MockPolicy(Some(Box::new(move || {
            Ok(Response::from_bytes(status, headers.clone(), body.clone()))
        })));

        let ctx = Context::new();
        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);

        // Act
        let result = policy
            .send(&ctx, &mut request, &[])
            .await
            .expect("Policy execution failed");

        // Assert
        let (actual_status, _, actual_body) = result.deconstruct();
        assert_eq!(actual_status, status, "Custom status should match");
        assert!(
            expected_body.eq(&actual_body.collect().await.unwrap()),
            "Custom body should match"
        );
    }
}
