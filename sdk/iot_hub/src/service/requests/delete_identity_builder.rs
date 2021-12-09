use http::{Method, StatusCode};

use crate::service::{ServiceClient, API_VERSION};

/// The DeleteIdentityBuilder is used to construct a request to delete a module or device identity.
pub struct DeleteIdentityBuilder<'a> {
    service_client: &'a ServiceClient,
    if_match: String,
    device_id: String,
    module_id: Option<String>,
}

impl<'a> DeleteIdentityBuilder<'a> {
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        if_match: String,
        device_id: String,
        module_id: Option<String>,
    ) -> Self {
        Self {
            service_client,
            if_match,
            device_id,
            module_id,
        }
    }

    /// Execute the request to delete the module or device identity.
    pub async fn execute(&self) -> crate::Result<()> {
        let uri = match &self.module_id {
            Some(module_id) => format!(
                "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
                self.service_client.iot_hub_name, self.device_id, module_id, API_VERSION
            ),
            None => format!(
                "https://{}.azure-devices.net/devices/{}?api-version={}",
                self.service_client.iot_hub_name, self.device_id, API_VERSION
            ),
        };

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
