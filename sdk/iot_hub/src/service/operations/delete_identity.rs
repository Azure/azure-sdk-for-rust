use crate::service::{ServiceClient, API_VERSION};
use azure_core::headers;
use azure_core::Method;

azure_core::operation! {
    /// The DeleteIdentityBuilder is used to construct a request to delete a module or device identity.
    DeleteIdentity,
    client: ServiceClient,
    if_match: String,
    device_id: String,
    module_id: Option<String>,
}

impl DeleteIdentityBuilder {
    /// Execute the request to delete the module or device identity.
    pub fn into_future(mut self) -> DeleteIdentity {
        Box::pin(async move {
            let uri = match &self.module_id {
                Some(module_id) => format!(
                    "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, module_id, API_VERSION
                ),
                None => format!(
                    "https://{}.azure-devices.net/devices/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, API_VERSION
                ),
            };

            let mut request = self.client.finalize_request(&uri, Method::Delete)?;
            request.insert_header(headers::IF_MATCH, format!("\"{}\"", &self.if_match));

            request.set_body(azure_core::EMPTY_BODY);

            self.client.send(&mut self.context, &mut request).await?;
            Ok(())
        })
    }
}

pub type DeleteIdentityResponse = ();
