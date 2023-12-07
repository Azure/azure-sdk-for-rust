use crate::service::resources::{
    AuthenticationMechanism, ConnectionState, DeviceCapabilities, Status,
};
use serde::{Deserialize, Serialize};

/// The representation of a device identity.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

/// Response of `CreateOrUpdateDeviceIdentity`
pub type CreateOrUpdateDeviceIdentityResponse = DeviceIdentityResponse;
