use std::convert::TryInto;

use azure_core::errors::AzureError;

use http::Method;
use serde::Serialize;

use crate::service::resources::{
    identity::IdentityOperation, AuthenticationMechanism, AuthenticationType, SymmetricKey,
    X509ThumbPrint,
};

use crate::service::responses::ModuleIdentityResponse;
use crate::service::{IoTHubError, ServiceClient, API_VERSION};

/// The CreateOrUpdateModuleIdentityBuilder is used to construct a new module identity
/// or the update an existing one.
pub struct CreateOrUpdateModuleIdentityBuilder<'a> {
    service_client: &'a ServiceClient,
    authentication: Option<AuthenticationMechanism>,
    device_id: Option<String>,
    module_id: Option<String>,
    managed_by: Option<String>,
    etag: Option<String>,
    operation: IdentityOperation,
}

impl<'a> CreateOrUpdateModuleIdentityBuilder<'a> {
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        operation: IdentityOperation,
        etag: Option<String>,
    ) -> Self {
        Self {
            service_client,
            authentication: None,
            device_id: None,
            module_id: None,
            managed_by: None,
            etag,
            operation,
        }
    }

    /// Set authentication to SAS on the module
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

    /// Set authentication to x509 on the module
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

    /// Set authentication to certificate authority on the module
    pub fn authentication_using_certificate_authority(mut self) -> Self {
        self.authentication = Some(AuthenticationMechanism {
            authentication_type: AuthenticationType::Authority,
            x509_thumbprint: X509ThumbPrint::default(),
            symmetric_key: SymmetricKey::default(),
        });

        self
    }

    /// Sets the device id of the module
    pub fn device_id<S>(mut self, device_id: S) -> Self
    where
        S: Into<String>,
    {
        self.device_id = Some(device_id.into());
        self
    }

    /// Sets the module id of the module
    pub fn module_id<S>(mut self, module_id: S) -> Self
    where
        S: Into<String>,
    {
        self.module_id = Some(module_id.into());
        self
    }

    /// Sets the managed by property of the module
    pub fn managed_by<S>(mut self, managed_by: S) -> Self
    where
        S: Into<String>,
    {
        self.managed_by = Some(managed_by.into());
        self
    }

    /// Performs the create or update request on the device identity
    pub async fn execute(self) -> Result<ModuleIdentityResponse, IoTHubError> {
        let device_id = self.device_id.ok_or_else(|| {
            AzureError::GenericErrorWithText("Field 'device_id' is not set".to_string())
        })?;

        let authentication = self.authentication.ok_or_else(|| {
            AzureError::GenericErrorWithText("Field 'authentication' is not set".to_string())
        })?;

        let managed_by = self.managed_by.ok_or_else(|| {
            AzureError::GenericErrorWithText("Field 'managed_by' is not set".to_string())
        })?;

        let module_id = self.module_id.ok_or_else(|| {
            AzureError::GenericErrorWithText("Field 'module_id' is not set".to_string())
        })?;

        let uri = format!(
            "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
            self.service_client.iothub_name, device_id, module_id, API_VERSION
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

        let body = CreateOrUpdateModuleIdentityBody {
            authentication,
            device_id,
            module_id,
            managed_by,
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
struct CreateOrUpdateModuleIdentityBody {
    authentication: AuthenticationMechanism,
    device_id: String,
    module_id: String,
    managed_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
}
