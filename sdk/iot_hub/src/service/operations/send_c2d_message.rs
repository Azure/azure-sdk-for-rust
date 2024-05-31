//! Send cloud to device messages.
use crate::service::responses::Cloud2DeviceMessageResponse;
use crate::service::{ServiceClient, API_VERSION};
use azure_core::Method;
use std::collections::HashMap;

azure_core::operation! {
    /// The Cloud2DeviceMessageBuilder is used for constructing the request to
    /// send a cloud to device message.
    Cloud2DeviceMessage,
    client: ServiceClient,
    device_id: String,
    ?message_body: serde_json::Value,
    ?properties: HashMap<String, String>
}

impl Cloud2DeviceMessageBuilder {
    /// Turn the builder into a `Future`.
    #[must_use]
    pub fn into_future(self) -> Cloud2DeviceMessage {
        Box::pin(async move {
            let uri = format!(
                "https://{}.azure-devices.net/devices/{}/messages/deviceBound?api-version={}",
                self.client.iot_hub_name, self.device_id, API_VERSION
            );
            let mut request = self.client.finalize_request(&uri, Method::Post)?;

            if let Some(properties) = self.properties {
                for (name, value) in properties {
                    // Application properties are prefixed with "iothub-app-"
                    request.insert_header(format!("iothub-app-{name}"), value);
                }
            }

            if let Some(message_body) = self.message_body {
                let body = azure_core::to_json(&message_body)?;
                request.set_body(body);
            }

            let response = self.client.send(&self.context, &mut request).await?;

            Cloud2DeviceMessageResponse::try_from(response)
        })
    }
}
