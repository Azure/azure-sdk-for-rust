use crate::bytes_response::BytesResponse;
#[cfg(not(target_arch = "wasm32"))]
use crate::policies::{Policy, PolicyResult};
use crate::{MockFrameworkError, TransportOptions};
use crate::{PipelineContext, Request, Response};
use std::io::Read;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransportPlayerPolicy {
    pub(crate) transport_options: TransportOptions,
}

impl MockTransportPlayerPolicy {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(transport_options: TransportOptions) -> Self {
        Self { transport_options }
    }
}

#[async_trait::async_trait]
#[cfg(not(target_arch = "wasm32"))]
impl<C> Policy<C> for MockTransportPlayerPolicy
where
    C: Send + Sync,
{
    async fn send<'a, 'b, 'c>(
        &'a self,
        ctx: &'b mut PipelineContext<'a, C>,
        request: &'c mut Request,
        next: &'a [Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        // deserialize to file both the request and the response
        let (expected_request, expected_response) = {
            let mut request_path = ctx.get_inner_context().prepare_and_get_transaction_path()?;
            let mut response_path = request_path.clone();

            request_path.push(format!(
                "{}_request.json",
                ctx.get_inner_context().get_mock_transaction()?.number()
            ));
            response_path.push(format!(
                "{}_response.json",
                ctx.get_inner_context().get_mock_transaction()?.number()
            ));

            let mut request_contents_stream = std::fs::File::open(&request_path)?;

            let mut request = String::new();
            request_contents_stream.read_to_string(&mut request)?;

            let mut response_contents_stream = std::fs::File::open(&response_path)?;

            let mut response = String::new();
            response_contents_stream.read_to_string(&mut response)?;

            (request, response)
        };

        let expected_request: Request = serde_json::from_str(&expected_request)?;
        let expected_response: BytesResponse = serde_json::from_str(&expected_response)?;

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

        ctx.get_inner_context_mut().increment_mock_transaction()?;
        Ok(expected_response.into())
    }
}
