use crate::service::resources::{identity::IdentityOperation, AuthenticationMechanism};
use crate::service::responses::ModuleIdentityResponse;
use crate::service::{ServiceClient, API_VERSION};
use http::Method;
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
    ) -> Result<ModuleIdentityResponse, crate::Error>
    where
        S: AsRef<str>,
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let uri = format!(
            "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
            self.service_client.iothub_name,
            device_id.as_ref(),
            module_id.as_ref(),
            API_VERSION
        );

        let mut request = self.service_client.prepare_request(&uri, Method::PUT);

        if self.operation == IdentityOperation::Update {
            match &self.etag {
                Some(etag) => {
                    request = request.header(http::header::IF_MATCH, format!("\"{}\"", etag));
                }
                None => return Err(crate::Error::EtagNotSet),
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
struct CreateOrUpdateModuleIdentityBody<'a, 'b, 'c> {
    authentication: AuthenticationMechanism,
    device_id: &'a str,
    module_id: &'b str,
    managed_by: &'c str,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
}
