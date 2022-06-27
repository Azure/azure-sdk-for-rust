use super::mock_response::MockResponse;
use super::mock_transaction::MockTransaction;
use crate::error::{Error, ErrorKind};
use crate::policies::{Policy, PolicyResult};
use crate::{Context, Request, TransportOptions};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransportPlayerPolicy {
    #[allow(unused)]
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
impl Policy for MockTransportPlayerPolicy {
    async fn send(
        &self,
        _ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // there must be no more policies
        assert_eq!(0, next.len());

        // deserialize to file both the request and the response
        let (expected_request, expected_response) = {
            let mut request_path = self.transaction.file_path(false)?;
            let mut response_path = request_path.clone();

            let number = self.transaction.number();
            request_path.push(format!("{}_request.json", number));
            response_path.push(format!("{}_response.json", number));

            let request = std::fs::read_to_string(&request_path)?;
            let response = std::fs::read_to_string(&response_path)?;

            (request, response)
        };

        let expected_request: Request = serde_json::from_str(&expected_request)?;
        let expected_response = serde_json::from_str::<MockResponse>(&expected_response)?;

        let expected_uri = expected_request.path_and_query();
        let actual_uri = request.path_and_query();
        if expected_uri != actual_uri {
            return Err(Error::with_message(ErrorKind::MockFramework, || {
                format!(
                    "mismatched request uri. Actual '{0}', Expected: '{1}'",
                    actual_uri, expected_uri
                )
            }));
        }

        // check if the passed request matches the one read from disk
        // We will ignore some headers that are bound to change every time
        // We'll probabily want to make the exclusion list dynamic at some point.
        const SKIPPED_HEADERS: &[&str] = &["Date", "x-ms-date", "authorization", "user-agent"];
        let actual_headers = request
            .headers()
            .iter()
            .filter(|(h, _)| !SKIPPED_HEADERS.contains(&h.as_str()))
            .collect::<Vec<_>>();

        let expected_headers = expected_request
            .headers()
            .iter()
            .filter(|(h, _)| !SKIPPED_HEADERS.contains(&h.as_str()))
            .collect::<Vec<_>>();

        // In order to accept a request, we make sure that:
        // 1. There are no extra headers (in both the received and read request).
        // 2. Each header has the same value.
        if actual_headers.len() != expected_headers.len() {
            return Err(Error::with_message(ErrorKind::MockFramework, || {
                format!(
                    "different number of headers in request. Actual: {0}, Expected: {1}",
                    actual_headers.len(),
                    expected_headers.len(),
                )
            }));
        }

        for (actual_header_key, actual_header_value) in actual_headers.iter() {
            let (_, expected_header_value) = expected_headers
                .iter()
                .find(|(h, _)| actual_header_key.as_str() == h.as_str())
                .ok_or_else(|| Error::with_message(ErrorKind::MockFramework, ||
                    format!("received request have header '{0}' but it was not present in the read request",
                    actual_header_key.as_str(),
                )))?;

            if actual_header_value != expected_header_value {
                return Err(Error::with_message(ErrorKind::MockFramework, || {
                    format!(
                        "request header '{0}' value is different. Actual: {1}, Expected: {2}",
                        actual_header_key.as_str().to_owned(),
                        actual_header_value.as_str().to_owned(),
                        expected_header_value.as_str().to_owned(),
                    )
                }));
            }
        }

        if expected_request.method() != request.method() {
            return Err(Error::with_message(ErrorKind::MockFramework, || {
                format!(
                    "mismatched HTTP request method. Actual: {0}, Expected: {1}",
                    expected_request.method(),
                    request.method(),
                )
            }));
        }

        let actual_body = match request.body() {
            crate::Body::Bytes(bytes) => bytes as &[u8],
            crate::Body::SeekableStream(_) => unimplemented!(),
        };

        let expected_body = match expected_request.body() {
            crate::Body::Bytes(bytes) => bytes as &[u8],
            crate::Body::SeekableStream(_) => unimplemented!(),
        };

        if actual_body != expected_body {
            return Err(Error::with_message(ErrorKind::MockFramework, || {
                format!(
                    "mismatched request body. Actual: {0:?}, Expected: {1:?}",
                    actual_body.to_vec(),
                    expected_body.to_vec(),
                )
            }));
        }

        self.transaction.increment_number();
        Ok(expected_response.into())
    }
}
