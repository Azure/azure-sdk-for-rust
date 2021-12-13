use super::mock_response::MockResponse;
use super::MockTransaction;
use crate::policies::{Policy, PolicyResult};
use crate::{Context, Request, Response, TransportOptions};
use std::io::Write;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransportRecorderPolicy {
    pub(crate) transport_options: TransportOptions,
    transaction: MockTransaction,
}

impl MockTransportRecorderPolicy {
    pub fn new(transport_options: TransportOptions) -> Self {
        let transaction = MockTransaction::new(transport_options.transaction_name.clone());
        Self {
            transport_options,
            transaction,
        }
    }
}

#[async_trait::async_trait]
impl Policy for MockTransportRecorderPolicy {
    async fn send(
        &self,
        _ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        // serialize to file both the request and the response
        let mut request_path = self.transaction.file_path(true)?;
        let mut response_path = request_path.clone();

        let number = self.transaction.number();
        request_path.push(format!("{}_request.json", number));
        response_path.push(format!("{}_response.json", number));

        let request_contents = serde_json::to_string(&request).unwrap();
        {
            let mut request_contents_stream = std::fs::File::create(&request_path).unwrap();
            request_contents_stream
                .write_all(request_contents.as_str().as_bytes())
                .map_err(|e| {
                    super::MockFrameworkError::IOError("cannot write request file".into(), e)
                })?;
        }

        let response = self
            .transport_options
            .http_client
            .execute_request2(request)
            .await?;

        // we need to duplicate the response because we are about to consume the response stream.
        // We replace the HTTP stream with a memory-backed stream.
        let (response, mock_response) = MockResponse::duplicate(response).await?;
        let response_contents = serde_json::to_string(&mock_response).unwrap();
        {
            let mut response_contents_stream = std::fs::File::create(&response_path).unwrap();
            response_contents_stream
                .write_all(response_contents.as_bytes())
                .map_err(|e| {
                    super::MockFrameworkError::IOError("cannot write response file".into(), e)
                })?;
        }

        self.transaction.increment_number();
        Ok(response)
    }
}
