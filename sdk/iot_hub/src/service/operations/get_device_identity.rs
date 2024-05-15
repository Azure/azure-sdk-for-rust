use crate::service::{responses::DeviceIdentityResponse, ServiceClient, API_VERSION};
use azure_core::Method;

azure_core::operation! {
    /// The ModuleIdentityBuilder is used to construct a request to get identity of a device
    DeviceIdentity,
    client: ServiceClient,
    device_id: String,
}

impl DeviceIdentityBuilder {
    /// Execute the request to get the identity of a device or module.
    pub fn into_future(self) -> DeviceIdentity {
        Box::pin(async move {
            let url = format!(
                "https://{}.azure-devices.net/devices/{}?api-version={}",
                self.client.iot_hub_name, self.device_id, API_VERSION
            );

            let mut request = self.client.finalize_request(&url, Method::GET)?;
            request.set_body(azure_core::EMPTY_BODY);

            self.client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}
