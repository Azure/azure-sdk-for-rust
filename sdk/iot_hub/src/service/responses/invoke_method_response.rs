use azure_core::error::{Error, Result};
use http::response::Response;
use serde::Deserialize;

/// The DirectMethodResponse struct contains the response
/// from the IoT Hub when a direct method was invoked.
#[derive(Deserialize)]
pub struct InvokeMethodResponse {
    /// The status of the direct method invocation.
    pub status: u64,
    /// The response payload of the direct method invocation.
    pub payload: Option<serde_json::Value>,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for InvokeMethodResponse {
    type Error = Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self> {
        let body = response.body();

        let invoke_method_response: InvokeMethodResponse = serde_json::from_slice(body)?;

        Ok(invoke_method_response)
    }
}
