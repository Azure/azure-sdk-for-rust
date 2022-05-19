use http::{Method, StatusCode};
use serde::Serialize;
use std::convert::TryInto;

use crate::service::{ServiceClient, API_VERSION};

/// The ApplyOnEdgeDeviceBuilder is used to construct a new device identity
/// or the update an existing one.
pub struct ApplyOnEdgeDeviceBuilder<'a> {
    service_client: &'a ServiceClient,
    device_id: String,
    device_content: serde_json::Value,
    module_content: serde_json::Value,
    modules_content: serde_json::Value
}

impl<'a> ApplyOnEdgeDeviceBuilder<'a> {
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        device_id: String
    ) -> Self {
        Self {
            service_client,
            device_id,
            device_content: serde_json::json!({}),
            module_content: serde_json::json!({}),
            modules_content: serde_json::json!({})
        }
    }

    /// Sets the device content
    pub fn device_content(mut self, device_content: serde_json::Value) -> Self {
        self.device_content = device_content;
        self
    }

    /// Sets the module content
    pub fn module_content(mut self, module_content: serde_json::Value) -> Self {
        self.module_content = module_content;
        self
    }

    /// Sets the modules content
    pub fn modules_content(mut self, modules_content: serde_json::Value) -> Self {
        self.modules_content = modules_content;
        self
    }

    /// Performs the apply on edge device request
    pub async fn execute(
        self,
    ) -> crate::Result<()>
    {
        let uri = format!(
            "https://{}.azure-devices.net/devices/{}/applyConfigurationContent?api-version={}",
            self.service_client.iot_hub_name,
            self.device_id,
            API_VERSION
        );

        let mut request = self.service_client.prepare_request(&uri, Method::POST);
        let body = ApplyOnEdgeDeviceBody {
            device_content: self.device_content,
            module_content: self.module_content,
            modules_content: self.modules_content
        };

        let body = azure_core::to_json(&body)?;
        let request = request.body(body)?;

        self.service_client
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApplyOnEdgeDeviceBody<'a> {
    device_content: serde_json::Value,
    module_content: serde_json::Value,
    modules_content: serde_json::Value,
}
