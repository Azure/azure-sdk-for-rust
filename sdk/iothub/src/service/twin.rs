use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::service::identity::{
    AuthenticationType, ConnectionState, DeviceCapabilities, Status, X509ThumbPrint,
};
use crate::service::{ServiceClient, API_VERSION};

#[derive(Deserialize, Debug)]
pub struct TwinProperties {
    pub desired: serde_json::Value,
    pub reported: serde_json::Value,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTwin {
    pub authentication_type: AuthenticationType,
    pub capabilities: DeviceCapabilities,
    pub cloud_to_device_message_count: i64,
    pub connection_state: ConnectionState,
    pub device_etag: String,
    pub device_id: String,
    pub device_scope: Option<String>,
    pub etag: String,
    pub last_activity_time: String,
    pub parent_scopes: Option<Vec<String>>,
    pub properties: TwinProperties,
    pub status: Status,
    pub status_reason: Option<String>,
    pub status_update_time: String,
    pub tags: HashMap<String, String>,
    pub version: i64,
    pub x509_thumbprint: X509ThumbPrint,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleTwin {
    pub authentication_type: AuthenticationType,
    pub cloud_to_device_message_count: i64,
    pub connection_state: ConnectionState,
    pub device_etag: String,
    pub device_id: String,
    pub etag: String,
    pub last_activity_time: String,
    pub module_id: String,
    pub properties: TwinProperties,
    pub status: Status,
    pub status_update_time: String,
    pub version: i64,
    pub x509_thumbprint: X509ThumbPrint,
}

struct DesiredTwin {
    contents: serde_json::Value,
}

pub struct DesiredTwinBuilder<'a, R>
where
    R: DeserializeOwned,
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

impl<'a, R> DesiredTwinBuilder<'a, R>
where
    R: DeserializeOwned,
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
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// use iothub::service::ServiceClient;
    ///
    /// let iothub = ServiceClient::from_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iothub.update_device_twin("some-device")
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
    /// use iothub::service::ServiceClient;
    /// use serde_json;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iothub = ServiceClient::from_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iothub.update_device_twin("some-device")
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
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// use iothub::service::ServiceClient;
    ///
    /// let iothub = ServiceClient::from_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iothub.update_device_twin("some-device")
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
    /// use iothub::service::ServiceClient;
    /// use serde_json;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iothub = ServiceClient::from_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iothub.update_device_twin("some-device")
    ///              .tag("TagName", "TagValue")
    ///              .properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .execute();
    /// ```
    pub async fn execute(self) -> Result<R, AzureError> {
        let desired_twin = DesiredTwin {
            contents: serde_json::json!({
                "properties": {
                    "desired": self.desired_properties.unwrap_or_else(|| serde_json::json!({}))
                },
                "tags": self.desired_tags
            }),
        };

        update_twin(
            self.service_client,
            self.method,
            self.device_id,
            self.module_id,
            self.if_match,
            desired_twin,
        )
        .await
    }
}

/// Helper function for getting a device/module twin
pub(crate) async fn get_twin<T>(
    service_client: &ServiceClient,
    device_id: String,
    module_id: Option<String>,
) -> Result<T, AzureError>
where
    T: DeserializeOwned,
{
    let uri = match module_id {
        Some(val) => format!(
            "https://{}.azure-devices.net/twins/{}/modules/{}?api-version={}",
            service_client.iothub_name, device_id, val, API_VERSION
        ),
        None => format!(
            "https://{}.azure-devices.net/twins/{}?api-version={}",
            service_client.iothub_name, device_id, API_VERSION
        ),
    };

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::builder()
        .uri(uri)
        .method(Method::GET)
        .header("Authorization", &service_client.sas_token)
        .header("Content-Type", "application/json")
        .body(Body::empty())?;

    let (status_code, _, whole_body) =
        extract_status_headers_and_body(client.request(request)).await?;
    if !status_code.is_success() {
        return Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            StatusCode::OK,
            status_code,
            std::str::from_utf8(&whole_body)?,
        )));
    }

    Ok(serde_json::from_slice(&whole_body)?)
}

/// Helper function for updating the device/module twin
async fn update_twin<S, T, U, V>(
    service_client: &ServiceClient,
    method: Method,
    device_id: S,
    module_id: Option<T>,
    if_match: Option<U>,
    desired_twin: DesiredTwin,
) -> Result<V, AzureError>
where
    S: Into<String>,
    T: Into<String>,
    U: Into<String>,
    for<'de> V: Deserialize<'de>,
{
    let uri = match module_id {
        Some(val) => format!(
            "https://{}.azure-devices.net/twins/{}/modules/{}?api-version={}",
            service_client.iothub_name,
            device_id.into(),
            val.into(),
            API_VERSION
        ),
        None => format!(
            "https://{}.azure-devices.net/twins/{}?api-version={}",
            service_client.iothub_name,
            device_id.into(),
            API_VERSION
        ),
    };

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut request_builder = Request::builder()
        .uri(uri)
        .method(method)
        .header("Authorization", &service_client.sas_token)
        .header("Content-Type", "application/json");

    if let Some(if_match) = if_match {
        request_builder = request_builder.header("If-Match", if_match.into());
    }

    let request =
        request_builder.body(Body::from(serde_json::to_string(&desired_twin.contents)?))?;
    let (status_code, _, whole_body) =
        extract_status_headers_and_body(client.request(request)).await?;

    if !status_code.is_success() {
        return Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            StatusCode::OK,
            status_code,
            std::str::from_utf8(&whole_body)?,
        )));
    }

    Ok(serde_json::from_slice(&whole_body)?)
}
