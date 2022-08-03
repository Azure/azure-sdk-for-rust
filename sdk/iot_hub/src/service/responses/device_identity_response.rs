use crate::service::resources::{
    AuthenticationMechanism, ConnectionState, DeviceCapabilities, Status,
};
use azure_core::error::Error;
use serde::{Deserialize, Serialize};

/// The representation of a device identity.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceIdentityResponse {
    /// The authentication mechanism of the device.
    pub authentication: AuthenticationMechanism,
    /// The capabilities of the device.
    pub capabilities: DeviceCapabilities,
    /// The amount of queued cloud to device messages.
    pub cloud_to_device_message_count: u64,
    /// The connection state of the device
    pub connection_state: ConnectionState,
    /// The date and time the connection state was last updated.
    pub connection_state_updated_time: String,
    /// The unique identifier of the device.
    pub device_id: String,
    /// The scope of the device.
    pub device_scope: Option<String>,
    /// The string representing a weak Etag for the device identity, as per RFC7232.
    pub etag: String,
    /// An IoT-Hub generated, case sensitive string which is used to distinguish devices
    /// with the same deviceId, when they have been deleted and re-created.
    pub generation_id: String,
    /// The date and time the device last connected, or sent or received a message.
    pub last_activity_time: String,
    /// The status of the device.
    pub status: Status,
    /// The reason for the device status.
    pub status_reason: Option<String>,
    /// The date and time the status was last updated.
    pub status_updated_time: String,
}

impl std::convert::TryFrom<crate::service::CollectedResponse> for DeviceIdentityResponse {
    type Error = Error;

    fn try_from(response: crate::service::CollectedResponse) -> azure_core::Result<Self> {
        let body = response.body();

        let device_identity_response: DeviceIdentityResponse = serde_json::from_slice(body)?;

        Ok(device_identity_response)
    }
}

/// Response of CreateOrUpdateDeviceIdentity
pub type CreateOrUpdateDeviceIdentityResponse = DeviceIdentityResponse;

impl CreateOrUpdateDeviceIdentityResponse {
    pub(crate) async fn try_from(response: azure_core::Response) -> azure_core::Result<Self> {
        let collected = azure_core::CollectedResponse::from_response(response).await?;
        let body = collected.body();
        Ok(serde_json::from_slice(body)?)
    }
}
