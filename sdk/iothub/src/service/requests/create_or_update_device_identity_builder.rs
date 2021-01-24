use std::convert::TryInto;

use azure_core::errors::AzureError;
use http::Method;
use serde::Serialize;

use crate::service::resources::{
    identity::DesiredCapability, identity::IdentityOperation, AuthenticationMechanism,
    AuthenticationType, DeviceCapabilities, Status, SymmetricKey, X509ThumbPrint,
};
use crate::service::responses::DeviceIdentityResponse;
use crate::service::{IoTHubError, ServiceClient, API_VERSION};

/// The CreateOrUpdateDeviceIdentityBuilder is used to construct a new device identity
/// or the update an existing one.
pub struct CreateOrUpdateDeviceIdentityBuilder<'a> {
    service_client: &'a ServiceClient,
    authentication: Option<AuthenticationMechanism>,
    capabilities: DeviceCapabilities,
    device_id: Option<String>,
    status: Option<Status>,
    etag: Option<String>,
    operation: IdentityOperation,
}

impl<'a> CreateOrUpdateDeviceIdentityBuilder<'a> {
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
    pub async fn execute(self) -> Result<DeviceIdentityResponse, IoTHubError> {
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

        let mut request = self.service_client.prepare_request(&uri, Method::PUT);

        if self.operation == IdentityOperation::Update {
            match &self.etag {
                Some(etag) => {
                    request = request.header(http::header::IF_MATCH, format!("\"{}\"", etag));
                }
                None => {
                    return Err(Box::new(AzureError::GenericErrorWithText(
                        "etag is not set".to_string(),
                    )))
                }
            }
        }

        let body = CreateOrUpdateDeviceIdentityBody {
            authentication,
            device_id,
            status,
            capabilities: self.capabilities,
            etag: self.etag,
        };

        let body = azure_core::to_json(&body)?;
        let request = request.body(body)?;

        Ok(self
            .service_client
            .http_client()
            .execute_request_check_statuses(
                request,
                &[http::StatusCode::OK, http::StatusCode::CREATED],
            )
            .await?
            .try_into()?)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrUpdateDeviceIdentityBody {
    authentication: AuthenticationMechanism,
    device_id: String,
    status: Status,
    capabilities: DeviceCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
}
