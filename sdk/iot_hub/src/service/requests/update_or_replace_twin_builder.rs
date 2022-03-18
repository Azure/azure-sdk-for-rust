use std::convert::TryInto;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::marker::PhantomData;

use bytes::Bytes;
use http::{Method, Response, StatusCode};
use serde::Serialize;

use crate::service::{ServiceClient, API_VERSION};

/// The UpdateOrReplaceTwinBuilder is used to construct a request for
/// updating or replacing a device or module twin.
pub struct UpdateOrReplaceTwinBuilder<'a, R>
where
    R: TryFrom<Response<Bytes>, Error = crate::Error>,
{
    service_client: &'a ServiceClient,
    pub(crate) device_id: String,
    pub(crate) module_id: Option<String>,
    if_match: Option<String>,
    desired_properties: Option<serde_json::Value>,
    desired_tags: HashMap<String, String>,
    pub(crate) method: Method,
    desired_twin_return_type: PhantomData<R>,
}

impl<'a, R> UpdateOrReplaceTwinBuilder<'a, R>
where
    R: TryFrom<Response<Bytes>, Error = crate::Error>,
{
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        device_id: String,
        module_id: Option<String>,
        method: Method,
    ) -> Self {
        Self {
            service_client,
            device_id,
            module_id,
            if_match: None,
            desired_properties: None,
            desired_tags: HashMap::new(),
            method,
            desired_twin_return_type: PhantomData,
        }
    }

    /// Add a new tag to the desired twin.
    ///
    /// This function can be invoked multiple times to add multiple tags to the desired twin.
    /// When adding a tag which is already in the desired twin, its value will be updated.
    ///
    /// # Example
    /// ```
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// use azure_iot_hub::service::ServiceClient;
    /// # let http_client = azure_core::new_http_client();
    ///
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_device_twin("some-device")
    ///                  .tag("TagName", "TagValue")
    ///                  .tag("AnotherTag", "WithAnotherValue")
    ///                  .tag("LastTag", "LastValue");
    /// ```
    pub fn tag<T>(mut self, tag_name: T, tag_value: T) -> Self
    where
        T: Into<String>,
    {
        self.desired_tags.insert(tag_name.into(), tag_value.into());
        self
    }

    /// Add new properties to the desired twin
    ///
    /// # Example
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    /// # let http_client = azure_core::new_http_client();
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_device_twin("some-device")
    ///              .properties(serde_json::json!({
    ///                "PropertyName": "PropertyValue",
    ///                "ParentProperty": {
    ///                  "ChildProperty": "ChildValue"
    ///                }
    ///              }));
    pub fn properties(mut self, desired_properties: serde_json::Value) -> Self {
        self.desired_properties = Some(desired_properties);
        self
    }

    /// Set the ETag for the twin
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// # let http_client = azure_core::new_http_client();
    ///
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_device_twin("some-device")
    ///                  .if_match("AAAAAAAAAAA=");
    /// ```
    pub fn if_match<T>(mut self, if_match: T) -> Self
    where
        T: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    /// Updates the twin with the desired settings
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// # let http_client = azure_core::new_http_client();
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_device_twin("some-device")
    ///              .tag("TagName", "TagValue")
    ///              .properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .execute();
    /// ```
    pub async fn execute(self) -> crate::Result<R> {
        let body = DesiredTwinBody {
            tags: self.desired_tags,
            properties: DesiredTwinProperties {
                desired: self
                    .desired_properties
                    .unwrap_or_else(|| serde_json::json!({})),
            },
        };

        let uri = match self.module_id {
            Some(val) => format!(
                "https://{}.azure-devices.net/twins/{}/modules/{}?api-version={}",
                self.service_client.iot_hub_name, self.device_id, val, API_VERSION
            ),
            None => format!(
                "https://{}.azure-devices.net/twins/{}?api-version={}",
                self.service_client.iot_hub_name, self.device_id, API_VERSION
            ),
        };

        let mut request = self.service_client.prepare_request(&uri, self.method);
        if let Some(if_match) = self.if_match {
            request = request.header(http::header::IF_MATCH, format!("\"{}\"", if_match));
        }
        let body = azure_core::to_json(&body)?;

        let request = request.body(body)?;

        self
            .service_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()
    }
}

#[derive(Serialize)]
struct DesiredTwinProperties {
    desired: serde_json::Value,
}

#[derive(Serialize)]
struct DesiredTwinBody {
    tags: HashMap<String, String>,
    properties: DesiredTwinProperties,
}
