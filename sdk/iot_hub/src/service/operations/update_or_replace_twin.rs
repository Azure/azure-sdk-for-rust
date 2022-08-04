use azure_core::headers;
use azure_core::Method;
use serde::Serialize;
use std::collections::HashMap;

use crate::service::{ServiceClient, API_VERSION};

azure_core::operation! {
    /// The UpdateOrReplaceTwinBuilder is used to construct a request for
    /// updating or replacing a device or module twin.
    UpdateOrReplaceTwin,
    client: ServiceClient,
    device_id: String,
    method: Method,
    ?module_id: String,
    ?if_match: String,
    ?desired_properties: serde_json::Value,
    ?desired_tags: HashMap<String, String>
}

impl UpdateOrReplaceTwinBuilder {
    /// Add a new tag to the desired twin.
    ///
    /// This function can be invoked multiple times to add multiple tags to the desired twin.
    /// When adding a tag which is already in the desired twin, its value will be updated.
    ///
    /// # Example
    /// ```
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_device_twin("some-device")
    ///                  .tag("TagName", "TagValue")
    ///                  .tag("AnotherTag", "WithAnotherValue")
    ///                  .tag("LastTag", "LastValue");
    /// ```
    pub fn tag<T>(mut self, tag_name: T, tag_value: T) -> Self
    where
        T: Into<String>,
    {
        let tags = self.desired_tags.get_or_insert(Default::default());
        tags.insert(tag_name.into(), tag_value.into());
        self
    }

    /// Updates the twin with the desired settings
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_device_twin("some-device")
    ///              .tag("TagName", "TagValue")
    ///              .desired_properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .into_future();
    /// ```
    pub fn into_future(mut self) -> UpdateOrReplaceTwin {
        Box::pin(async move {
            let body = DesiredTwinBody {
                tags: self.desired_tags.unwrap_or_default(),
                properties: DesiredTwinProperties {
                    desired: self.desired_properties.unwrap_or_default(),
                },
            };

            let uri = match self.module_id {
                Some(val) => format!(
                    "https://{}.azure-devices.net/twins/{}/modules/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, val, API_VERSION
                ),
                None => format!(
                    "https://{}.azure-devices.net/twins/{}?api-version={}",
                    self.client.iot_hub_name, self.device_id, API_VERSION
                ),
            };

            let mut request = self.client.finalize_request(&uri, self.method)?;
            if let Some(if_match) = self.if_match {
                request.insert_header(headers::IF_MATCH, format!("\"{}\"", if_match));
            }
            let body = azure_core::to_json(&body)?;

            request.set_body(body);

            let response = self.client.send(&mut self.context, &mut request).await?;

            UpdateOrReplaceTwinResponse::from_response(response).await
        })
    }
}

pub type UpdateOrReplaceTwinResponse = crate::service::CollectedResponse;

#[derive(Serialize)]
struct DesiredTwinProperties {
    desired: serde_json::Value,
}

#[derive(Serialize)]
struct DesiredTwinBody {
    tags: HashMap<String, String>,
    properties: DesiredTwinProperties,
}
