use crate::bytes_response::BytesResponse;
use crate::mock_transaction::MockTransaction;
use crate::policies::{Policy, PolicyResult};
use crate::{MockFrameworkError, TransportOptions};
use crate::{PipelineContext, Request, Response};

use std::io::Write;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransportRecorderPolicy {
    pub(crate) transport_options: TransportOptions,
    transaction: MockTransaction,
}

impl MockTransportRecorderPolicy {
    pub fn new(name: impl Into<String>, transport_options: TransportOptions) -> Self {
        Self {
            transport_options,
            transaction: MockTransaction::new(name),
        }
    }
}

#[async_trait::async_trait]
impl<C> Policy<C> for MockTransportRecorderPolicy
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

        // serialize to file both the request and the response
        let mut request_path = self.transaction.file_path()?;
        let mut response_path = request_path.clone();

        let number = self.transaction.number();
        request_path.push(format!("{}_request.json", number));
        response_path.push(format!("{}_response.json", number));

        let request_contents = serde_json::to_string(&request).unwrap();
        {
            let mut request_contents_stream = std::fs::File::create(&request_path).unwrap();
            request_contents_stream
                .write_all(request_contents.as_str().as_bytes())
                .map_err(|e| MockFrameworkError::IOError("cannot write request file".into(), e))?;
        }

        let response = { self.transport_options.http_client.execute_request2(request) };
        let response: Response = response.await?.into();

        // we need to duplicate the response because we are about to consume the response stream.
        // We replace the HTTP stream with a memory-backed stream.
        let (response, bytes_response) = BytesResponse::duplicate(response).await?;
        let response_contents = serde_json::to_string(&bytes_response).unwrap();
        {
            let mut response_contents_stream = std::fs::File::create(&response_path).unwrap();
            response_contents_stream
                .write_all(response_contents.as_str().as_bytes())
                .map_err(|e| MockFrameworkError::IOError("cannot write response file".into(), e))?;
        }

        self.transaction.increment_number();
        Ok(response)
    }
}
