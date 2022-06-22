use crate::service::resources::{
    identity::DesiredCapability, identity::IdentityOperation, AuthenticationMechanism,
    DeviceCapabilities, Status,
};
use crate::service::responses::DeviceIdentityResponse;
use crate::service::{ServiceClient, API_VERSION};
use azure_core::error::{Error, ErrorKind};
use azure_core::headers;
use http::Method;
use serde::Serialize;
use std::convert::TryInto;

/// The CreateOrUpdateDeviceIdentityBuilder is used to construct a new device identity
/// or the update an existing one.
pub struct CreateOrUpdateDeviceIdentityBuilder<'a> {
    service_client: &'a ServiceClient,
    capabilities: DeviceCapabilities,
    etag: Option<String>,
    operation: IdentityOperation,
}

impl<'a> CreateOrUpdateDeviceIdentityBuilder<'a> {
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        operation: IdentityOperation,
        etag: Option<String>,
    ) -> Self {
        Self {
            service_client,
            capabilities: DeviceCapabilities::default(),
            etag,
            operation,
        }
    }

    /// Sets a device capability on the device
    pub fn device_capability(mut self, desired_capability: DesiredCapability) -> Self {
        match desired_capability {
            DesiredCapability::IotEdge => self.capabilities.iotedge = true,
        }
        self
    }

    /// Performs the create or update request on the device identity
    pub async fn execute<S>(
        self,
        device_id: S,
        status: Status,
        authentication: AuthenticationMechanism,
    ) -> azure_core::Result<DeviceIdentityResponse>
    where
        S: AsRef<str>,
    {
        let uri = format!(
            "https://{}.azure-devices.net/devices/{}?api-version={}",
            self.service_client.iot_hub_name,
            device_id.as_ref(),
            API_VERSION
        );

        let mut request = self.service_client.prepare_request(&uri, Method::PUT)?;

        if self.operation == IdentityOperation::Update {
            match &self.etag {
                Some(etag) => {
                    request.insert_header(headers::IF_MATCH, format!("\"{}\"", etag));
                }
                None => return Err(Error::message(ErrorKind::Other, "etag is not set")),
            }
        }

        let body = CreateOrUpdateDeviceIdentityBody {
            authentication,
            device_id: device_id.as_ref(),
            status,
            capabilities: self.capabilities,
            etag: self.etag,
        };

        let body = azure_core::to_json(&body)?;
        request.set_body(body);

        self.service_client
            .http_client()
            .execute_request_check_status(&request)
            .await?
            .try_into()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrUpdateDeviceIdentityBody<'a> {
    authentication: AuthenticationMechanism,
    device_id: &'a str,
    status: Status,
    capabilities: DeviceCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
}
