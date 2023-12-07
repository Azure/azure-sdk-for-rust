use crate::service::{responses::ModuleIdentityResponse, ServiceClient, API_VERSION};
use azure_core::Method;

azure_core::operation! {
    /// The ModuleIdentityBuilder is used to construct a request to get identity of a module.
    ModuleIdentity,
    client: ServiceClient,
    device_id: String,
    module_id: String,
}

impl ModuleIdentityBuilder {
    /// Execute the request to get the identity of a module.
    pub fn into_future(self) -> ModuleIdentity {
        Box::pin(async move {
            let url = format!(
                "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
                self.client.iot_hub_name, self.device_id, self.module_id, API_VERSION
            );

            let mut request = self.client.finalize_request(&url, Method::Get)?;
            request.set_body(azure_core::EMPTY_BODY);

            self.client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}
