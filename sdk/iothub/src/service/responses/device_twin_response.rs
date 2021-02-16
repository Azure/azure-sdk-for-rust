use std::collections::HashMap;

use http::Response;
use serde::Deserialize;

use crate::service::resources::{
    AuthenticationType, ConnectionState, DeviceCapabilities, Status, TwinProperties, X509ThumbPrint,
};
use crate::service::IoTHubError;

/// The representation of a response for a device twin request.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTwinResponse {
    /// The authentication type of the device.
    pub authentication_type: AuthenticationType,
    /// The capabilities of the device.
    pub capabilities: DeviceCapabilities,
    /// The amount of queued cloud to device messages.
    pub cloud_to_device_message_count: i64,
    /// The connection state of the device.
    pub connection_state: ConnectionState,
    /// The etag of the device.
    pub device_etag: String,
    /// The unique identifier of the device.
    pub device_id: String,
    /// The scope of the device.
    pub device_scope: Option<String>,
    /// The string representing a weak Etag for the twin, as per RFC7232.
    pub etag: String,
    /// The date and time the device last connected, or sent or received a message.
    pub last_activity_time: String,
    /// The scopes of the parent.
    pub parent_scopes: Option<Vec<String>>,
    /// The twin properties of the device.
    pub properties: TwinProperties,
    /// The status of the device
    pub status: Status,
    /// The reason for the device status.
    pub status_reason: Option<String>,
    /// The date and time the status was last updated.
    pub status_update_time: String,
    /// The tags for of the device
    pub tags: Option<HashMap<String, String>>,
    /// The version of the device twin including tags and desired properties.
    pub version: i64,
    /// The x509 thumbprint of the device.
    pub x509_thumbprint: X509ThumbPrint,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for DeviceTwinResponse {
    type Error = IoTHubError;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let body = response.body();

        let device_twin_response: DeviceTwinResponse = serde_json::from_slice(body)?;

        Ok(device_twin_response)
    }
}
