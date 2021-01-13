use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};

use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::service::{ServiceClient, API_VERSION};


/// Representation of a desired device capability
pub enum DesiredCapability {
    /// The IoT Edge device capability
    IotEdge,
}

/// The connection state of a module or device
#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum ConnectionState {
    /// The device or module is connected
    Connected,
    /// The device or module is disconnected
    Disconnected,
}

/// Device or module status
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// The device or module is disabled
    Disabled,
    /// The device or module is enabled
    Enabled,
}

/// Representation of device capabilities.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct DeviceCapabilities {
    #[serde(rename = "iotEdge")]
    /// Whether the device has the IoT Edge capability or not.
    pub iotedge: bool,
}

/// Representation of a symmetric key for authentication.
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct SymmetricKey {
    primary_key: Option<String>,
    secondary_key: Option<String>,
}

/// Representation of a x509 thumbprint for authentication.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct X509ThumbPrint {
    /// The primary thumbprint.
    pub primary_thumbprint: Option<String>,
    /// The secondary thumbprint.
    pub secondary_thumbprint: Option<String>,
}

/// AuthenticationType of a module or device.
#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum AuthenticationType {
    /// Authentication using certificate
    #[serde(rename = "certificate")]
    Certificate,
    /// Authentication using a certificate authority.
    #[serde(rename = "Authority")]
    Authority,
    /// The device or module is not authenticated.
    #[serde(rename = "none")]
    None,
    /// Authentication using symmetric keys
    #[serde(rename = "sas")]
    SAS,
    /// Authentication using self signed certificates
    #[serde(rename = "selfSigned")]
    SelfSigned,
}

/// The authentication mechanism for a device or module identity.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationMechanism {
    /// The symmetric key pair used for authentication.
    pub symmetric_key: SymmetricKey,
    /// The type of authentication that is being used.
    #[serde(rename = "type")]
    pub authentication_type: AuthenticationType,
    /// The primary and secondary x509 thumbprints used for x509 based authentication.
    pub x509_thumbprint: X509ThumbPrint,
}

/// The representation of a device identity.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// The authentication mechanism of the device.
    pub authentication: AuthenticationMechanism,
    /// The capabilities of the device.
    pub capabilities: DeviceCapabilities,
    /// The amount of queued cloud to device messages.
    pub cloud_to_device_message_count: u64,
    /// The connection state of the device
    pub connection_state: ConnectionState,
    /// The date and time the connection state was last updated.
    pub connection_state_updated_time: String,
    /// The unique identifier of the device.
    pub device_id: String,
    /// The scope of the device.
    pub device_scope: Option<String>,
    /// The string representing a weak Etag for the device identity, as per RFC7232.
    pub etag: String,
    /// An IoT-Hub generated, case sensitive string which is used to distinguish devices
    /// with the same deviceId, when they have been deleted and re-created.
    pub generation_id: String,
    /// The date and time the device last connected, or sent or received a message.
    pub last_activity_time: String,
    /// The status of the device.
    pub status: Status,
    /// The reason for the device status.
    pub status_reason: Option<String>,
    /// The date and time the status was last updated.
    pub status_updated_time: String,
}

/// The representation of a module identity
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    /// The authentication mechanism of the module.
    pub authentication: AuthenticationMechanism,
    /// The amount of queued cloud to module messages.
    pub cloud_to_device_message_count: u64,
    /// The connection state of the module.
    pub connection_state: ConnectionState,
    /// The date and time the connection state was last updated.
    pub connection_state_updated_time: String,
    /// The unique identifier of the device.
    pub device_id: String,
    /// The string representing a weak Etag for the device identity, as per RFC7232.
    pub etag: String,
    /// An IoT-Hub generated, case sensitive string which is used to distinguish modules
    /// with the same deviceId, when they have been deleted and re-created.
    pub generation_id: String,
    /// The date and time the module last connected, or sent or received a message.
    pub last_activity_time: String,
    /// The entity that manages this module.
    pub managed_by: String,
    /// The unique identifier of the module
    pub module_id: String,
}

/// The operation to perform on an identity
pub(crate) enum IdentityOperation {
    Create,
    Update,
}

/// The DeviceIdentityBuilder is used to construct a new device identity
/// or the update an existing one
pub struct DeviceIdentityBuilder<'a> {
    service_client: &'a ServiceClient,
    authentication: Option<AuthenticationMechanism>,
    capabilities: DeviceCapabilities,
    device_id: Option<String>,
    status: Option<Status>,
    etag: Option<String>,
    operation: IdentityOperation,
}

impl<'a> DeviceIdentityBuilder<'a> {
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        operation: IdentityOperation,
        etag: Option<String>,
    ) -> Self {
        Self {
            service_client,
            authentication: None,
            capabilities: DeviceCapabilities::default(),
            device_id: None,
            status: None,
            etag,
            operation,
        }
    }

    /// Set authentication to SAS on the device
    pub fn authentication_using_sas<S, T>(mut self, primary_key: S, secondary_key: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.authentication = Some(AuthenticationMechanism {
            authentication_type: AuthenticationType::SAS,
            x509_thumbprint: X509ThumbPrint::default(),
            symmetric_key: SymmetricKey {
                primary_key: Some(primary_key.into()),
                secondary_key: Some(secondary_key.into()),
            },
        });

        self
    }

    /// Set authentication to x509 on the device
    pub fn authentication_using_x509<S, T>(
        mut self,
        primary_thumbprint: S,
        secondary_thumbprint: T,
    ) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.authentication = Some(AuthenticationMechanism {
            authentication_type: AuthenticationType::SelfSigned,
            x509_thumbprint: X509ThumbPrint {
                primary_thumbprint: Some(primary_thumbprint.into()),
                secondary_thumbprint: Some(secondary_thumbprint.into()),
            },
            symmetric_key: SymmetricKey::default(),
        });

        self
    }

    /// Set authentication to certificate authority on the device
    pub fn authentication_using_certificate_authority(mut self) -> Self {
        self.authentication = Some(AuthenticationMechanism {
            authentication_type: AuthenticationType::Authority,
            x509_thumbprint: X509ThumbPrint::default(),
            symmetric_key: SymmetricKey::default(),
        });

        self
    }

    /// Sets a device capability on the device
    pub fn device_capability(mut self, desired_capability: DesiredCapability) -> Self {
        match desired_capability {
            DesiredCapability::IotEdge => self.capabilities.iotedge = true,
        }
        self
    }

    /// Sets the device id of the device
    pub fn device_id<S>(mut self, device_id: S) -> Self
    where
        S: Into<String>,
    {
        self.device_id = Some(device_id.into());
        self
    }

    /// Sets the desired status of the device
    pub fn status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    /// Performs the create or update request on the device identity
    pub async fn execute(self) -> Result<Device, AzureError> {
        let device_id = self.device_id.ok_or_else(|| {
            AzureError::GenericErrorWithText("Field 'device_id' is not set".to_string())
        })?;

        let status = self.status.ok_or_else(|| {
            AzureError::GenericErrorWithText("Field 'status' is not set".to_string())
        })?;

        let authentication = self.authentication.ok_or_else(|| {
            AzureError::GenericErrorWithText("Field 'authentication' is not set".to_string())
        })?;

        let uri = format!(
            "https://{}.azure-devices.net/devices/{}?api-version={}",
            self.service_client.iothub_name, device_id, API_VERSION
        );

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let mut request_builder = Request::builder()
            .uri(uri)
            .method(Method::PUT)
            .header("Authorization", &self.service_client.sas_token)
            .header("Content-Type", "application/json");

        let mut body = serde_json::json!({
            "authentication": authentication,
            "deviceId": device_id,
            "status": status,
            "capabilities": self.capabilities
        });

        match self.operation {
            IdentityOperation::Update => {
                let etag = self.etag.ok_or_else(|| {
                    AzureError::GenericErrorWithText("etag is not set".to_string())
                })?;
                request_builder = request_builder.header("If-Match", "*");
                body["etag"] = serde_json::json!(etag);
            }
            IdentityOperation::Create => {}
        }

        let request = request_builder.body(Body::from(serde_json::to_string(&body)?))?;
        let (status_code, _, whole_body) =
            extract_status_headers_and_body(client.request(request)).await?;
        if !status_code.is_success() {
            return Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                StatusCode::OK,
                status_code,
                std::str::from_utf8(&whole_body)?,
            )));
        }

        Ok(serde_json::from_slice::<Device>(&whole_body)?)
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

pub(crate) async fn delete_identity(
    service_client: &ServiceClient,
    if_match: Option<String>,
    device_id: String,
    module_id: Option<String>,
) -> Result<(), AzureError> {
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
        .method(Method::DELETE)
        .header("Authorization", &service_client.sas_token)
        .header("Content-Type", "application/json")
        .header("If-Match", if_match.unwrap_or("*".to_string()))
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

    Ok(())
}
