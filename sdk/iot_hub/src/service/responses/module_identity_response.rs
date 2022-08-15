use crate::service::resources::{AuthenticationMechanism, ConnectionState};
use azure_core::error::Error;
use serde::Deserialize;

/// The representation of a module identity
#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModuleIdentityResponse {
    /// The authentication mechanism of the module.
    pub authentication: AuthenticationMechanism,
    /// The amount of queued cloud to module messages.
    pub cloud_to_device_message_count: u64,
    /// The connection state of the module.
    pub connection_state: ConnectionState,
    /// The date and time the connection state was last updated.
    pub connection_state_updated_time: String,
    /// The unique identifier of the device.
    pub device_id: String,
    /// The string representing a weak Etag for the device identity, as per RFC7232.
    pub etag: String,
    /// An IoT-Hub generated, case sensitive string which is used to distinguish modules
    /// with the same deviceId, when they have been deleted and re-created.
    pub generation_id: String,
    /// The date and time the module last connected, or sent or received a message.
    pub last_activity_time: String,
    /// The entity that manages this module.
    pub managed_by: String,
    /// The unique identifier of the module
    pub module_id: String,
}

impl std::convert::TryFrom<crate::service::CollectedResponse> for ModuleIdentityResponse {
    type Error = Error;

    fn try_from(response: crate::service::CollectedResponse) -> azure_core::Result<Self> {
        let body = response.body();

        let module_identity_response: ModuleIdentityResponse = serde_json::from_slice(body)?;

        Ok(module_identity_response)
    }
}

/// Response for CreateOrUpdateModuleIdentity
pub type CreateOrUpdateModuleIdentityResponse = ModuleIdentityResponse;

impl CreateOrUpdateModuleIdentityResponse {
    pub(crate) async fn try_from(response: azure_core::Response) -> azure_core::Result<Self> {
        let collected = azure_core::CollectedResponse::from_response(response).await?;
        let body = collected.body();
        Ok(serde_json::from_slice(body)?)
    }
}
