use crate::service::{CollectedResponse, ServiceClient, API_VERSION};
use azure_core::Method;

azure_core::operation! {
    /// The GetIdentityBuilder is used to construct a request to get identity
    GetIdentity,
    client: ServiceClient,
    device_id: String,
    ?module_id: String
}

impl GetIdentityBuilder {
    /// Execute the request to get the identity of a device or module.
    pub fn into_future(self) -> GetIdentity {
        Box::pin(async move {
            let url = if let Some(module_id) = &self.module_id {
                format!(
                    "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, module_id, API_VERSION
                )
            } else {
                format!(
                    "https://{}.azure-devices.net/devices/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, API_VERSION
                )
            };

            let mut request = self.client.finalize_request(&url, Method::Get)?;
            request.set_body(azure_core::EMPTY_BODY);

            let response = self.client.send(&self.context, &mut request).await?;

            GetIdentityResponse::from_response(response).await
        })
    }
}

pub type GetIdentityResponse = CollectedResponse;
