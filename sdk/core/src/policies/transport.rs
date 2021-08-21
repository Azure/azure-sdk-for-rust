#[cfg(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
))]
use crate::bytes_response::BytesResponse;
#[cfg(not(target_arch = "wasm32"))]
use crate::policies::{Policy, PolicyResult};
#[allow(unused_imports)]
use crate::TransportOptions;
#[allow(unused_imports)]
use crate::{Context, HttpClient, PipelineContext, Request, Response};
#[cfg(feature = "mock_transport_consume")]
use std::io::Read;
#[cfg(feature = "mock_transport_generate")]
use std::io::Write;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransportPolicy {
    pub(crate) http_client: Arc<dyn HttpClient>,
}

impl TransportPolicy {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(options: &TransportOptions) -> Self {
        Self {
            http_client: options.http_client.clone(),
        }
    }
}

#[async_trait::async_trait]
#[cfg(not(target_arch = "wasm32"))]
impl<C> Policy<C> for TransportPolicy
where
    C: Send + Sync,
{
    #[cfg(not(any(
        feature = "mock_transport_generate",
        feature = "mock_transport_consume"
    )))]
    async fn send(
        &self,
        _ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        let response = { self.http_client.execute_request2(request) };

        Ok(response.await?)
    }

    #[cfg(feature = "mock_transport_consume")]
    async fn send(
        &self,
        _ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        use crate::mock_transport::*;
        // there must be no more policies
        assert_eq!(0, next.len());

        // deserialize to file both the request and the response
        let (read_request, read_response) = {
            let mut request_path = prepare_and_get_transaction_path();
            let mut response_path = request_path.clone();

            request_path.push(format!("{}_request.json", get_transaction_num()));
            response_path.push(format!("{}_response.json", get_transaction_num()));

            let mut request_contents_stream = std::fs::File::open(&request_path)
                .expect(&format!("Cannot open request file {:?}", request_path));

            let mut request = String::new();
            request_contents_stream
                .read_to_string(&mut request)
                .expect("cannot read request file");

            let mut response_contents_stream = std::fs::File::open(&response_path)
                .expect(&format!("Cannot open response file {:?}", response_path));

            let mut response = String::new();
            response_contents_stream
                .read_to_string(&mut response)
                .expect("cannot read response file");

            (request, response)
        };

        let read_request: Request =
            serde_json::from_str(&read_request).expect("error deserializing the request");

        println!("read_request == {:#?}", read_request);

        let read_response: BytesResponse =
            serde_json::from_str(&read_response).expect("error deserializing the response");

        println!("read_response == {:#?}", read_response);

        // check if the passed request matches the one read from disk
        // We will ignore some headers that are bound to change every time
        // We'll probabily want to make the exclusion list dynamic at some point.
        const SKIPPED_HEADERS: &[&'static str] = &["Date", "x-ms-date", "authorization"];
        let received_headers_to_match = request
            .headers()
            .iter()
            .filter(|h| {
                SKIPPED_HEADERS
                    .iter()
                    .find(|to_skip| to_skip == &&h.0.as_str())
                    .is_none()
            })
            .collect::<Vec<_>>();

        let read_headers_to_match = read_request
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
        if received_headers_to_match.len() != read_headers_to_match.len() {
            panic!(
                "different number of headers in request. Recevied: {}, Read: {}",
                received_headers_to_match.len(),
                read_headers_to_match.len()
            );
        }

        for received_header_to_match in received_headers_to_match.iter() {
            let read_header_to_match = read_headers_to_match
                .iter()
                .find(|h| received_header_to_match.0.as_str() == h.0.as_str())
                .expect(&format!(
                    "received request have header {} but it was not present in the read request",
                    received_header_to_match.0.as_str()
                ));

            if received_header_to_match.1 != read_header_to_match.1 {
                panic!(
                    "header {} value is different. Received: {}, Read: {}",
                    received_header_to_match.0.as_str(),
                    received_header_to_match.1.to_str().unwrap(),
                    read_header_to_match.1.to_str().unwrap()
                );
            }
        }

        if read_request.method() != request.method() {
            panic!(
                "HTTP method is different. Received: {}, Read: {}",
                read_request.method(),
                request.method()
            );
        }

        let received_body = match request.body() {
            crate::Body::Bytes(bytes) => &bytes as &[u8],
            crate::Body::SeekableStream(_) => unimplemented!(),
        };

        let read_body = match read_request.body() {
            crate::Body::Bytes(bytes) => &bytes as &[u8],
            crate::Body::SeekableStream(_) => unimplemented!(),
        };

        if received_body != read_body {
            panic!(
                "request body is different. Received: {}, Read: {}",
                read_request.method(),
                request.method()
            );
        }

        increment_transaction();
        Ok(read_response.into())
    }

    #[cfg(feature = "mock_transport_generate")]
    async fn send(
        &self,
        _ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        use crate::mock_transport::*;
        // there must be no more policies
        assert_eq!(0, next.len());

        // serialize to file both the request and the response
        let mut request_path = prepare_and_get_transaction_path();
        let mut response_path = request_path.clone();

        request_path.push(format!("{}_request.json", get_transaction_num()));
        response_path.push(format!("{}_response.json", get_transaction_num()));

        let request_contents = serde_json::to_string(&request).unwrap();
        {
            let mut request_contents_stream = std::fs::File::create(&request_path).unwrap();
            request_contents_stream
                .write_all(request_contents.as_str().as_bytes())
                .expect("cannot write request file");
        }

        let response = { self.http_client.execute_request2(request) };
        let response: Response = response.await?.into();

        // we need to duplicate the response because we are about to consume the response stream.
        // We replace the HTTP stream with a memory-backed stream.
        let (response, bytes_response) = BytesResponse::duplicate(response).await?;
        let response_contents = serde_json::to_string(&bytes_response).unwrap();
        {
            let mut response_contents_stream = std::fs::File::create(&response_path).unwrap();
            response_contents_stream
                .write_all(response_contents.as_str().as_bytes())
                .expect("cannot write response file");
        }

        increment_transaction();
        Ok(response)
    }
}
