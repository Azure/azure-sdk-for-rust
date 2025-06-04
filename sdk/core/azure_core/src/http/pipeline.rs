// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::policies::ClientRequestIdPolicy;
use crate::http::{options::TelemetryOptions, policies::TelemetryPolicy};
use std::{
    any::{Any, TypeId},
    ops::Deref,
    sync::Arc,
};
use typespec_client_core::http::{self, policies::Policy};

/// Execution pipeline.
///
/// A pipeline follows a precise flow:
///
/// 1. Client library-specified per-call policies are executed. Per-call policies can fail and bail out of the pipeline
///    immediately.
/// 2. User-specified per-call policies are executed.
/// 3. Built-in per-call policies are executed. If a [`ClientRequestIdPolicy`] was not added to `per_call_policies`,
///    the default will be added automatically.
/// 4. The retry policy is executed. It allows to re-execute the following policies.
/// 5. Client library-specified per-retry policies. Per-retry polices are always executed at least once but are re-executed
///    in case of retries.
/// 6. User-specified per-retry policies are executed.
/// 7. The authorization policy is executed. Authorization can depend on the HTTP headers and/or the request body so it
///    must be executed right before sending the request to the transport. Also, the authorization
///    can depend on the current time so it must be executed at every retry.
/// 8. The transport policy is executed. Transport policy is always the last policy and is the policy that
///    actually constructs the `Response` to be passed up the pipeline.
///
/// A pipeline is immutable. In other words a policy can either succeed and call the following
/// policy of fail and return to the calling policy. Arbitrary policy "skip" must be avoided (but
/// cannot be enforced by code). All policies except Transport policy can assume there is another following policy (so
/// `self.pipeline[0]` is always valid).
#[derive(Debug, Clone)]
pub struct Pipeline(http::Pipeline);

impl Pipeline {
    /// Creates a new pipeline given the client library crate name and version,
    /// alone with user-specified and client library-specified policies.
    ///
    /// Crates can simply pass `option_env!("CARGO_PKG_NAME")` and `option_env!("CARGO_PKG_VERSION")` for the
    /// `crate_name` and `crate_version` arguments respectively.
    pub fn new(
        crate_name: Option<&'static str>,
        crate_version: Option<&'static str>,
        options: http::ClientOptions,
        per_call_policies: Vec<Arc<dyn Policy>>,
        per_retry_policies: Vec<Arc<dyn Policy>>,
    ) -> Self {
        let mut per_call_policies = per_call_policies.clone();

        if per_call_policies
            .iter()
            .all(|policy| TypeId::of::<ClientRequestIdPolicy>() != policy.type_id())
        {
            per_call_policies.push(Arc::new(ClientRequestIdPolicy::default()));
        }

        let telemetry_policy = TelemetryPolicy::new(
            crate_name,
            crate_version,
            // TODO: &options.telemetry.unwrap_or_default(),
            &TelemetryOptions::default(),
        );
        per_call_policies.push(Arc::new(telemetry_policy));

        Self(http::Pipeline::new(
            options,
            per_call_policies,
            per_retry_policies,
        ))
    }
}

// TODO: Should we instead use the newtype pattern?
impl Deref for Pipeline {
    type Target = http::Pipeline;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        http::{
            headers::{self, HeaderName, Headers},
            policies::Policy,
            request::options::ClientRequestId,
            Context, Method, Request, StatusCode, TransportOptions,
        },
        Bytes,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt as _;
    use std::sync::Arc;
    use typespec_client_core::http::{ClientOptions, RawResponse};

    #[tokio::test]
    async fn pipeline_with_custom_client_request_id_policy() {
        // Arrange
        const CUSTOM_HEADER_NAME: &str = "x-custom-request-id";
        const CUSTOM_HEADER: HeaderName = HeaderName::from_static(CUSTOM_HEADER_NAME);
        const CLIENT_REQUEST_ID: &str = "custom-request-id";

        let mut ctx = Context::new();
        ctx.insert(ClientRequestId::new(CLIENT_REQUEST_ID.to_string()));

        let transport = TransportOptions::new(Arc::new(MockHttpClient::new(|req| {
            async {
                // Assert
                let header_value = req
                    .headers()
                    .get_optional_str(&CUSTOM_HEADER)
                    .expect("Custom header should be present");
                assert_eq!(
                    header_value, CLIENT_REQUEST_ID,
                    "Custom header value should match the client request ID"
                );

                Ok(RawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        })));
        let options = ClientOptions {
            transport: Some(transport),
            ..Default::default()
        };

        let per_call_policies: Vec<Arc<dyn Policy>> =
            vec![
                Arc::new(ClientRequestIdPolicy::with_header_name(CUSTOM_HEADER_NAME))
                    as Arc<dyn Policy>,
            ];
        let per_retry_policies = vec![];

        let pipeline = Pipeline::new(
            Some("test-crate"),
            Some("1.0.0"),
            options,
            per_call_policies,
            per_retry_policies,
        );

        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);

        // Act
        pipeline
            .send(&ctx, &mut request)
            .await
            .expect("Pipeline execution failed");
    }

    #[tokio::test]
    async fn pipeline_without_client_request_id_policy() {
        // Arrange
        const CLIENT_REQUEST_ID: &str = "default-request-id";

        let mut ctx = Context::new();
        ctx.insert(ClientRequestId::new(CLIENT_REQUEST_ID.to_string()));

        let transport = TransportOptions::new(Arc::new(MockHttpClient::new(|req| {
            async {
                // Assert
                let header_value = req
                    .headers()
                    .get_optional_str(&headers::CLIENT_REQUEST_ID)
                    .expect("Default header should be present");
                assert_eq!(
                    header_value, CLIENT_REQUEST_ID,
                    "Default header value should match the client request ID"
                );

                Ok(RawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        })));
        let options = ClientOptions {
            transport: Some(transport),
            ..Default::default()
        };

        let per_call_policies = vec![]; // No ClientRequestIdPolicy added
        let per_retry_policies = vec![];

        let pipeline = Pipeline::new(
            Some("test-crate"),
            Some("1.0.0"),
            options,
            per_call_policies,
            per_retry_policies,
        );

        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);

        // Act
        pipeline
            .send(&ctx, &mut request)
            .await
            .expect("Pipeline execution failed");
    }
}
