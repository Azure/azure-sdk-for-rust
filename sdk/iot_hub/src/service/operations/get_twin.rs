use crate::service::{ServiceClient, API_VERSION};
use azure_core::Method;

azure_core::operation! {
    /// The GetTwinBuilder is used to construct a request to get a twin module or device
    GetTwin,
    client: ServiceClient,
    device_id: String,
    ?module_id: String
}

impl GetTwinBuilder {
    /// Execute the request to get the twin of a module or device.
    pub fn into_future(mut self) -> GetTwin {
        Box::pin(async move {
            let uri = match self.module_id {
                Some(val) => format!(
                    "https://{}.azure-devices.net/twins/{}/modules/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, val, API_VERSION
                ),
                None => format!(
                    "https://{}.azure-devices.net/twins/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, API_VERSION
                ),
            };

            let mut request = self.client.finalize_request(&uri, Method::Get)?;
            request.set_body(azure_core::EMPTY_BODY);

            let response = self.client.send(&mut self.context, &mut request).await?;

            GetTwinResponse::from_response(response).await
        })
    }
}

pub type GetTwinResponse = crate::service::CollectedResponse;
