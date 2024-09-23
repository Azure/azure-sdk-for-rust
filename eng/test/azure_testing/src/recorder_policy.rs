// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::mock_request::RequestSerializer;
use crate::TestContext;

use super::mock_response::MockResponse;
use super::MockTransaction;
use azure_core::error::{ErrorKind, ResultExt};
use azure_core::{Context, HttpClient, Policy, PolicyResult, Request};
use std::io::Write;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransportRecorderPolicy {
    transaction: MockTransaction,
    http_client: Arc<dyn HttpClient>,
}

impl MockTransportRecorderPolicy {
    pub fn new(tx_context: TestContext, http_client: Arc<dyn HttpClient>) -> Self {
        let transaction = MockTransaction::new(tx_context);
        Self {
            transaction,
            http_client,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for MockTransportRecorderPolicy {
    async fn send(
        &self,
        _ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // there must be no more policies
        assert_eq!(0, next.len());

        // serialize to file both the request and the response
        let mock_request = self.transaction.new_request(true)?;

        let request_contents =
            serde_json::to_string_pretty(&RequestSerializer::new(request)).unwrap();
        {
            let mut request_contents_stream =
                std::fs::File::create(&mock_request.request_path).unwrap();
            request_contents_stream
                .write_all(request_contents.as_bytes())
                .context(ErrorKind::MockFramework, "cannot write request file")?;
        }

        let response = self.http_client.execute_request(request).await?;

        // we need to duplicate the response because we are about to consume the response stream.
        // We replace the HTTP stream with a memory-backed stream.
        let (response, mock_response) = MockResponse::duplicate(response).await?;
        let response_contents = serde_json::to_string_pretty(&mock_response).unwrap();
        {
            let mut response_contents_stream =
                std::fs::File::create(&mock_request.response_path).unwrap();
            response_contents_stream
                .write_all(response_contents.as_bytes())
                .context(ErrorKind::MockFramework, "cannot write response file")?;
        }

        Ok(response)
    }
}
