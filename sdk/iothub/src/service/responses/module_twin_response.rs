use http::Response;
use serde::Deserialize;

use crate::service::resources::{
    AuthenticationType, ConnectionState, Status, TwinProperties, X509ThumbPrint,
};

/// The representation of a response for a module twin request.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleTwinResponse {
    /// The authentication mechanism of the module.
    pub authentication_type: AuthenticationType,
    /// The amount of queued cloud to module messages.
    pub cloud_to_device_message_count: i64,
    /// The connection state of the module.
    pub connection_state: ConnectionState,
    /// The etag of the device.
    pub device_etag: String,
    /// The unique identifier of the device.
    pub device_id: String,
    /// The string representing a weak Etag for the module twin, as per RFC7232.
    pub etag: String,
    /// The date and time the module last connected, or sent or received a message.
    pub last_activity_time: String,
    /// The unique identifier of the module.
    pub module_id: String,
    /// The twin properties of the module.
    pub properties: TwinProperties,
    /// The status of the module.
    pub status: Status,
    /// The date and time the status was last updated.
    pub status_update_time: String,
    /// The version of the module twin including tags and desired properties.
    pub version: i64,
    /// The X509 thumbprint of the module.
    pub x509_thumbprint: X509ThumbPrint,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for ModuleTwinResponse {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let body = response.body();

        debug!("body == {:#?}", std::str::from_utf8(body));

        let module_twin_response: ModuleTwinResponse = serde_json::from_slice(body)?;

        Ok(module_twin_response)
    }
}
