use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};

use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::service::{ServiceClient, API_VERSION};

enum DesiredCapability {
    IotEdge,
}

/// The connection state of a module or device
#[derive(Debug, Deserialize, PartialEq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

/// Device or module status
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Disabled,
    Enabled,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct DeviceCapabilities {
    #[serde(rename = "iotEdge")]
    pub iotedge: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SymmetricKey {
    primary_key: Option<String>,
    secondary_key: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct X509ThumbPrint {
    pub primary_thumbprint: Option<String>,
    pub secondary_thumbprint: Option<String>,
}

/// AuthenticationType of a module or device
#[derive(Debug, Deserialize, PartialEq)]
pub enum AuthenticationType {
    #[serde(rename = "certificate")]
    Certificate,
    Authority,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "sas")]
    SAS,
    #[serde(rename = "selfSigned")]
    SelfSigned,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationMechanism {
    pub symmetric_key: SymmetricKey,
    #[serde(rename = "type")]
    pub authentication_type: AuthenticationType,
    pub x509_thumbprint: X509ThumbPrint,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub authentication: AuthenticationMechanism,
    pub capabilities: DeviceCapabilities,
    pub cloud_to_device_message_count: u64,
    pub connection_state: ConnectionState,
    pub connection_state_updated_time: String,
    pub device_id: String,
    pub device_scope: String,
    pub etag: String,
    pub generation_id: String,
    pub last_activity_time: String,
    pub status: Status,
    pub status_reason: Option<String>,
    pub status_updated_time: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub authentication: AuthenticationMechanism,
    pub cloud_to_device_message_count: u64,
    pub connection_state: ConnectionState,
    pub connection_state_updated_time: String,
    pub device_id: String,
    pub etag: String,
    pub generation_id: String,
    pub last_activity_time: String,
    pub managed_by: String,
    pub module_id: String,
}

struct IdentityBuilder<'a> {
    service_client: &'a ServiceClient,
    authentication: Option<AuthenticationType>,
    capabilities: DeviceCapabilities,
    cloud_to_device_message_count: Option<u64>,
    connection_state: Option<ConnectionState>,
    device_id: Option<String>,
    status: Option<Status>,
}

impl<'a> IdentityBuilder<'a> {
    pub(crate) fn new(service_client: &'a ServiceClient) -> Self {
        Self {
            service_client,
            authentication: None,
            capabilities: DeviceCapabilities { iotedge: false },
            cloud_to_device_message_count: None,
            connection_state: None,
            device_id: None,
            status: None,
        }
    }

    pub fn authentication(mut self, authentication: AuthenticationType) -> Self {
        self.authentication = Some(authentication);
        self
    }

    pub fn device_capability(mut self, desired_capability: DesiredCapability) -> Self {
        match desired_capability {
            DesiredCapability::IotEdge => self.capabilities.iotedge = true,
        }
        self
    }

    pub fn connection_state(mut self, connection_state: ConnectionState) -> Self {
        self.connection_state = Some(connection_state);
        self
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }
}

pub(crate) async fn get_identity<T>(
    service_client: &ServiceClient,
    device_id: String,
    module_id: Option<String>,
) -> Result<T, AzureError>
where
    T: DeserializeOwned,
{
    let uri = match module_id {
        Some(module_id) => format!(
            "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
            service_client.iothub_name, device_id, module_id, API_VERSION
        ),
        None => format!(
            "https://{}.azure-devices.net/devices/{}?api-version={}",
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
