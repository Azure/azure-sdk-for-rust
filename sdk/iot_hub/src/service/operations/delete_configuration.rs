use crate::service::{ServiceClient, API_VERSION};

use azure_core::headers;
use azure_core::Method;

azure_core::operation! {
    /// The DeleteConfigurationBuilder is used to construct a request to delete a configuration.
    DeleteConfiguration,
    client: ServiceClient,
    if_match: String,
    configuration_id: String,
}

impl DeleteConfigurationBuilder {
    /// Execute the request to delete the configuration.
    pub fn into_future(mut self) -> DeleteConfiguration {
        Box::pin(async move {
            let uri = format!(
                "https://{}.azure-devices.net/configurations/{}?api-version={}",
                self.client.iot_hub_name, self.configuration_id, API_VERSION
            );

            let mut request = self.client.finalize_request(&uri, Method::Delete)?;
            request.insert_header(headers::IF_MATCH, format!("\"{}\"", &self.if_match));

            request.set_body(azure_core::EMPTY_BODY);

            self.client.send(&mut self.context, &mut request).await?;

            Ok(())
        })
    }
}

pub type DeleteConfigurationResponse = ();
