use crate::service::{ServiceClient, API_VERSION};

use azure_core::Method;
use serde::Serialize;
/// The ApplyOnEdgeDeviceBuilder is used to construct a new device identity
/// or the update an existing one.
pub struct ApplyOnEdgeDeviceBuilder<'a> {
    service_client: &'a ServiceClient,
    device_id: String,
    device_content: serde_json::Value,
    module_content: serde_json::Value,
    modules_content: serde_json::Value,
}

impl<'a> ApplyOnEdgeDeviceBuilder<'a> {
    pub(crate) fn new(service_client: &'a ServiceClient, device_id: String) -> Self {
        Self {
            service_client,
            device_id,
            device_content: serde_json::json!({}),
            module_content: serde_json::json!({}),
            modules_content: serde_json::json!({}),
        }
    }

    /// Sets the device content
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
    /// let edge_configuration_builder = iot_hub.apply_on_edge_device("some-device").device_content(serde_json::json!({}));
    /// ```
    pub fn device_content(mut self, device_content: serde_json::Value) -> Self {
        self.device_content = device_content;
        self
    }

    /// Sets the module content
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
    /// let edge_configuration_builder = iot_hub.apply_on_edge_device("some-device").module_content(serde_json::json!({}));
    /// ```
    pub fn module_content(mut self, module_content: serde_json::Value) -> Self {
        self.module_content = module_content;
        self
    }

    /// Sets the modules content
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
    /// let edge_configuration_builder = iot_hub.apply_on_edge_device("some-device").modules_content(serde_json::json!({}));
    /// ```
    pub fn modules_content(mut self, modules_content: serde_json::Value) -> Self {
        self.modules_content = modules_content;
        self
    }

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
    pub async fn execute(self) -> azure_core::Result<()> {
        let uri = format!(
            "https://{}.azure-devices.net/devices/{}/applyConfigurationContent?api-version={}",
            self.service_client.iot_hub_name, self.device_id, API_VERSION
        );

        let mut request = self.service_client.finalize_request(&uri, Method::Post)?;
        let body = ApplyOnEdgeDeviceBody {
            device_content: self.device_content,
            module_content: self.module_content,
            modules_content: self.modules_content,
        };

        let body = azure_core::to_json(&body)?;
        request.set_body(body);

        self.service_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApplyOnEdgeDeviceBody {
    device_content: serde_json::Value,
    module_content: serde_json::Value,
    modules_content: serde_json::Value,
}
