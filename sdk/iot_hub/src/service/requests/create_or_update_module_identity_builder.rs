use crate::service::resources::{identity::IdentityOperation, AuthenticationMechanism};
use crate::service::responses::ModuleIdentityResponse;
use crate::service::{ServiceClient, API_VERSION};
use azure_core::error::{Error, ErrorKind};
use azure_core::headers;
use azure_core::Method;
use serde::Serialize;
use std::convert::TryInto;

/// The CreateOrUpdateModuleIdentityBuilder is used to construct a new module identity
/// or the update an existing one.
pub struct CreateOrUpdateModuleIdentityBuilder<'a> {
    service_client: &'a ServiceClient,
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
            etag,
            operation,
        }
    }

    /// Performs the create or update request on the device identity
    pub async fn execute<S, T, U>(
        self,
        device_id: S,
        module_id: T,
        managed_by: U,
        authentication: AuthenticationMechanism,
    ) -> azure_core::Result<ModuleIdentityResponse>
    where
        S: AsRef<str>,
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let uri = format!(
            "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
            self.service_client.iot_hub_name,
            device_id.as_ref(),
            module_id.as_ref(),
            API_VERSION
        );

        let mut request = self.service_client.prepare_request(&uri, Method::PUT)?;

        if self.operation == IdentityOperation::Update {
            match &self.etag {
                Some(etag) => {
                    request.insert_header(headers::IF_MATCH, format!("\"{}\"", etag));
                }
                None => return Err(Error::message(ErrorKind::Other, "etag is not set")),
            }
        }

        let body = CreateOrUpdateModuleIdentityBody {
            authentication,
            device_id: device_id.as_ref(),
            module_id: module_id.as_ref(),
            managed_by: managed_by.as_ref(),
            etag: self.etag,
        };

        let body = azure_core::to_json(&body)?;
        request.set_body(body);

        self.service_client
            .http_client()
            .execute_request_check_status(&request)
            .await?
            .try_into()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrUpdateModuleIdentityBody<'a, 'b, 'c> {
    authentication: AuthenticationMechanism,
    device_id: &'a str,
    module_id: &'b str,
    managed_by: &'c str,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
}
