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
    ///
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    /// use serde_json;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// # let http_client = azure_core::new_http_client();
    ///
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// iot_hub.apply_on_edge_device("some-device").execute();
    /// ```
    pub fn into_future(self) -> ApplyOnEdgeDevice {
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

            self.client
                .http_client()
                .execute_request_check_status(&request)
                .await?;
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
