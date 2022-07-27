use crate::service::{ServiceClient, API_VERSION};
use azure_core::Method;

azure_core::operation! {
    GetIdentity,
    client: ServiceClient,
    device_id: String,
    ?module_id: String
}

impl GetIdentityBuilder {
    /// Execute the request to get the identity of a device or module.
    pub fn into_future<T>(self) -> GetIdentity {
        Box::pin(async move {
            let uri = match self.module_id {
                Some(module_id) => format!(
                    "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, module_id, API_VERSION
                ),
                None => format!(
                    "https://{}.azure-devices.net/devices/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, API_VERSION
                ),
            };

            let mut request = self.client.finalize_request(&uri, Method::Get)?;
            request.set_body(azure_core::EMPTY_BODY);

            self.client
                .http_client()
                .execute_request_check_status(&request)
                .await
        })
    }
}
