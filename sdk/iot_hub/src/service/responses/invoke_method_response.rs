use serde::Deserialize;

/// The `DirectMethodResponse` struct contains the response
/// from the `IoT` Hub when a direct method was invoked.
#[derive(Deserialize)]
pub struct InvokeMethodResponse {
    /// The status of the direct method invocation.
    pub status: u64,
    /// The response payload of the direct method invocation.
    pub payload: Option<serde_json::Value>,
}
