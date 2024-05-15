use crate::service::responses::ConfigurationResponse;
use crate::service::{ServiceClient, API_VERSION};
use azure_core::Method;

azure_core::operation! {
    /// The GetConfigurationBuilder is used to get configuration
    GetConfiguration,
    client: ServiceClient,
    ?configuration_id: String

}

impl GetConfigurationBuilder {
    /// Execute the request to get the configuration of a given identifier.
    pub fn into_future(self) -> GetConfiguration {
        Box::pin(async move {
            let uri = if let Some(val) = self.configuration_id {
                format!(
                    "https://{}.azure-devices.net/configurations/{}?api-version={}",
                    self.client.iot_hub_name, val, API_VERSION
                )
            } else {
                format!(
                    "https://{}.azure-devices.net/configurations?api-version={}",
                    self.client.iot_hub_name, API_VERSION
                )
            };

            let mut request = self.client.finalize_request(&uri, Method::GET)?;
            request.set_body(azure_core::EMPTY_BODY);

            self.client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}

pub type GetConfigurationResponse = ConfigurationResponse;
