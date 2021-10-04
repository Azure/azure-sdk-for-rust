use crate::bytes_response::BytesResponse;
use crate::policies::{Policy, PolicyResult};
use crate::{MockFrameworkError, TransportOptions};
use crate::{PipelineContext, Request, Response};

use crate::mock_transaction::MockTransaction;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransportPlayerPolicy {
    pub(crate) transport_options: TransportOptions,
    transaction: MockTransaction,
}

impl MockTransportPlayerPolicy {
    pub fn new(transport_options: TransportOptions) -> Self {
        let transaction = MockTransaction::new(transport_options.transaction_name.clone());
        Self {
            transport_options,
            transaction,
        }
    }
}

#[async_trait::async_trait]
impl<C> Policy<C> for MockTransportPlayerPolicy
where
    C: Send + Sync,
{
    async fn send(
        &self,
        _ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        // deserialize to file both the request and the response
        let (expected_request, expected_response) = {
            let mut request_path = self.transaction.file_path()?;
            let mut response_path = request_path.clone();

            let number = self.transaction.number();
            request_path.push(format!("{}_request.json", number));
            response_path.push(format!("{}_response.json", number));

            let request = std::fs::read_to_string(&request_path)?;
            let response = std::fs::read_to_string(&response_path)?;

            (request, response)
        };

        let expected_request: Request = serde_json::from_str(&expected_request)?;
        let expected_response = serde_json::from_str::<BytesResponse>(&expected_response)?;

        // check if the passed request matches the one read from disk
        // We will ignore some headers that are bound to change every time
        // We'll probabily want to make the exclusion list dynamic at some point.
        const SKIPPED_HEADERS: &[&'static str] = &["Date", "x-ms-date", "authorization"];
        let actual_headers = request
            .headers()
            .iter()
            .filter(|h| {
                SKIPPED_HEADERS
                    .iter()
                    .find(|to_skip| to_skip == &&h.0.as_str())
                    .is_none()
            })
            .collect::<Vec<_>>();

        let expected_headers = expected_request
            .headers()
            .iter()
            .filter(|h| {
                SKIPPED_HEADERS
                    .iter()
                    .find(|to_skip| to_skip == &&h.0.as_str())
                    .is_none()
            })
            .collect::<Vec<_>>();

        // In order to accept a request, we make sure that:
        // 1. There are no extra headers (in both the received and read request).
        // 2. Each header has the same value.
        if actual_headers.len() != expected_headers.len() {
            return Err(Box::new(MockFrameworkError::MismatchedRequestHeadersCount(
                actual_headers.len(),
                expected_headers.len(),
            )));
        }

        for actual_header_to_match in actual_headers.iter() {
            let read_header_to_match = expected_headers
                .iter()
                .find(|h| actual_header_to_match.0.as_str() == h.0.as_str())
                .ok_or(MockFrameworkError::MissingRequestHeader(
                    actual_header_to_match.0.as_str().to_owned(),
                ))?;

            if actual_header_to_match.1 != read_header_to_match.1 {
                return Err(Box::new(MockFrameworkError::MismatchedRequestHeader(
                    actual_header_to_match.0.as_str().to_owned(),
                    actual_header_to_match.1.to_str().unwrap().to_owned(),
                    read_header_to_match.1.to_str().unwrap().to_owned(),
                )));
            }
        }

        if expected_request.method() != request.method() {
            return Err(Box::new(MockFrameworkError::MismatchedRequestHTTPMethod(
                expected_request.method(),
                request.method(),
            )));
        }

        let received_body = match request.body() {
            crate::Body::Bytes(bytes) => &bytes as &[u8],
            crate::Body::SeekableStream(_) => unimplemented!(),
        };

        let read_body = match expected_request.body() {
            crate::Body::Bytes(bytes) => &bytes as &[u8],
            crate::Body::SeekableStream(_) => unimplemented!(),
        };

        if received_body != read_body {
            return Err(Box::new(MockFrameworkError::MismatchedRequestBody(
                received_body.to_vec(),
                read_body.to_vec(),
            )));
        }

        self.transaction.increment_number();
        Ok(expected_response.into())
    }
}
