use http::{Method, StatusCode};

use crate::service::{ServiceClient, API_VERSION};

/// The DeleteConfigurationBuilder is used to construct a request to delete a configuration.
pub struct DeleteConfigurationBuilder<'a> {
    service_client: &'a ServiceClient,
    if_match: String,
    configuration_id: String,
}

impl<'a> DeleteConfigurationBuilder<'a> {
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        if_match: String,
        configuration_id: String,
    ) -> Self {
        Self {
            service_client,
            if_match,
            configuration_id,
        }
    }

    /// Execute the request to delete the configuration.
    pub async fn execute(&self) -> Result<()> {
        let uri = format!(
            "https://{}.azure-devices.net/configurations/{}?api-version={}",
            self.service_client.iot_hub_name, self.configuration_id, API_VERSION
        );

        let request = self
            .service_client
            .prepare_request(&uri, Method::DELETE)
            .header(http::header::IF_MATCH, format!("\"{}\"", &self.if_match));

        let request = request.body(azure_core::EMPTY_BODY)?;

        self.service_client
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?;
        Ok(())
    }
}
