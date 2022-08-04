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

impl InvokeMethodResponse {
    pub(crate) async fn try_from(response: azure_core::Response) -> azure_core::Result<Self> {
        let collected = azure_core::CollectedResponse::from_response(response).await?;
        let body = collected.body();
        Ok(serde_json::from_slice(body)?)
    }
}
