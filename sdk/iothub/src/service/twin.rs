use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use serde::de::{self};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use crate::service::{ServiceClient, API_VERSION};

/// AuthenticationType of a module or device
#[derive(Debug)]
pub enum AuthenticationType {
    Certificate,
    Authority,
    None,
    SAS,
    SelfSigned,
}

impl<'de> Deserialize<'de> for AuthenticationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "certificate" => Ok(AuthenticationType::Certificate),
            "sas" => Ok(AuthenticationType::SAS),
            "Authority" => Ok(AuthenticationType::Authority),
            "selfSigned" => Ok(AuthenticationType::SelfSigned),
            "none" => Ok(AuthenticationType::None),
            _ => Err(de::Error::custom(format!("Expected status to be 'certificate','sas','Authority','selfSigned' or 'none' but received: {}", s))),
        }
    }
}

/// The connection state of a module or device
#[derive(Debug)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionState::Connected => write!(f, "connected"),
            ConnectionState::Disconnected => write!(f, "disconnected"),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Connected" => Ok(ConnectionState::Connected),
            "Disconnected" => Ok(ConnectionState::Disconnected),
            _ => Err(de::Error::custom(format!(
                "Expected status to be 'Connected' or 'Disconnected' but received: {}",
                s
            ))),
        }
    }
}

/// Device or module status
#[derive(Debug)]
pub enum Status {
    Disabled,
    Enabled,
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "disabled" => Ok(Status::Disabled),
            "enabled" => Ok(Status::Enabled),
            _ => Err(de::Error::custom(format!(
                "Expected status to be enabled or disabled but received: {}",
                s
            ))),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DeviceCapabilities {
    #[serde(rename = "iotEdge")]
    pub iotedge: bool,
}

#[derive(Deserialize, Debug)]
pub struct X509ThumbPrint {
    pub primary_thumbprint: Option<String>,
    pub secondary_thumbprint: Option<String>,
}

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

#[derive(Deserialize)]
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

async fn get_twin<T>(service_client: &ServiceClient, uri: String) -> Result<T, AzureError>
where
    for<'de> T: Deserialize<'de>,
{
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

pub(crate) async fn get_module_twin<S, T>(
    service_client: &ServiceClient,
    device_id: S,
    module_id: T,
) -> Result<ModuleTwin, AzureError>
where
    S: Into<String>,
    T: Into<String>,
{
    let uri = format!(
        "https://{}.azure-devices.net/twins/{}/modules/{}?api-version={}",
        service_client.iothub_name,
        device_id.into(),
        module_id.into(),
        API_VERSION
    );

    get_twin(service_client, uri).await
}

pub(crate) async fn get_device_twin<T>(
    service_client: &ServiceClient,
    device_id: T,
) -> Result<DeviceTwin, AzureError>
where
    T: Into<String>,
{
    let uri = format!(
        "https://{}.azure-devices.net/twins/{}?api-version={}",
        service_client.iothub_name,
        device_id.into(),
        API_VERSION
    );

    get_twin(service_client, uri).await
}
