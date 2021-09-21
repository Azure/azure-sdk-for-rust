use crate::bytes_response::BytesResponse;
use crate::bytes_response::SerializedBytesResponse;
#[cfg(not(target_arch = "wasm32"))]
use crate::policies::{Policy, PolicyResult};
use crate::{MockFrameworkError, TransportOptions};
use crate::{PipelineContext, Request, Response};
use std::io::Write;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransportRecorderPolicy {
    pub(crate) transport_options: TransportOptions,
}

impl MockTransportRecorderPolicy {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(transport_options: TransportOptions) -> Self {
        Self { transport_options }
    }
}

#[async_trait::async_trait]
#[cfg(not(target_arch = "wasm32"))]
impl<C> Policy<C> for MockTransportRecorderPolicy
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

        // serialize to file both the request and the response
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

        let request_contents = serde_json::to_string(&request).unwrap();
        {
            let mut request_contents_stream = std::fs::File::create(&request_path).unwrap();
            request_contents_stream
                .write_all(request_contents.as_str().as_bytes())
                .map_err(|e| MockFrameworkError::IOError("cannot write request file", e))?;
        }

        let response = { self.transport_options.http_client.execute_request2(request) };
        let response: Response = response.await?.into();

        // we need to duplicate the response because we are about to consume the response stream.
        // We replace the HTTP stream with a memory-backed stream.
        let (response, bytes_response) = BytesResponse::duplicate(response).await?;
        let bytes_response: SerializedBytesResponse<'_> = (&bytes_response).into();
        let response_contents = serde_json::to_string(&bytes_response).unwrap();
        {
            let mut response_contents_stream = std::fs::File::create(&response_path).unwrap();
            response_contents_stream
                .write_all(response_contents.as_str().as_bytes())
                .map_err(|e| MockFrameworkError::IOError("cannot write response file", e))?;
        }

        ctx.get_inner_context_mut().increment_mock_transaction()?;
        Ok(response)
    }
}
