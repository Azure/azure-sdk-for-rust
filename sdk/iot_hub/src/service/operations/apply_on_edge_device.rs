use crate::service::{ServiceClient, API_VERSION};

use azure_core::{operation, Method};
use serde::Serialize;
operation! {
    /// The ApplyOnEdgeDeviceBuilder is used to construct a new device identity
    /// or the update an existing one.
    ApplyOnEdgeDevice,
    client: ServiceClient,
    device_id: String,
    ?device_content: serde_json::Value,
    ?module_content: serde_json::Value,
    ?modules_content: serde_json::Value
}

impl ApplyOnEdgeDeviceBuilder {
    /// Performs the apply on edge device request
    pub fn into_future(mut self) -> ApplyOnEdgeDevice {
        Box::pin(async move {
            let uri = format!(
                "https://{}.azure-devices.net/devices/{}/applyConfigurationContent?api-version={}",
                self.client.iot_hub_name, self.device_id, API_VERSION
            );

            let mut request = self.client.finalize_request(&uri, Method::Post)?;
            let body = ApplyOnEdgeDeviceBody {
                device_content: self.device_content.unwrap_or_default(),
                module_content: self.module_content.unwrap_or_default(),
                modules_content: self.modules_content.unwrap_or_default(),
            };

            let body = azure_core::to_json(&body)?;
            request.set_body(body);

            self.client.send(&mut self.context, &mut request).await?;

            Ok(())
        })
    }
}

pub type ApplyOnEdgeDeviceResponse = ();

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApplyOnEdgeDeviceBody {
    device_content: serde_json::Value,
    module_content: serde_json::Value,
    modules_content: serde_json::Value,
}
