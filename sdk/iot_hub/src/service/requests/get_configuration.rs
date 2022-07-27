use crate::service::{ServiceClient, API_VERSION};
use azure_core::Method;

azure_core::operation! {
    GetConfiguration,
    client: ServiceClient,
    ?configuration_id: String

}

impl GetConfigurationBuilder {
    /// Execute the request to get the configuration of a given identifier.
    pub fn into_future(self) -> GetConfiguration {
        Box::pin(async move {
            let uri = match self.configuration_id {
                Some(val) => format!(
                    "https://{}.azure-devices.net/configurations/{}?api-version={}",
                    self.client.iot_hub_name, val, API_VERSION
                ),
                None => format!(
                    "https://{}.azure-devices.net/configurations?api-version={}",
                    self.client.iot_hub_name, API_VERSION
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

pub type GetConfigurationResponse = crate::service::CollectedResponse;
