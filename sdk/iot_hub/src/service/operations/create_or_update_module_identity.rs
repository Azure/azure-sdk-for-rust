use crate::service::resources::{identity::IdentityOperation, AuthenticationMechanism};
use crate::service::responses::CreateOrUpdateModuleIdentityResponse;
use crate::service::{ServiceClient, API_VERSION};
use azure_core::error::{Error, ErrorKind};
use azure_core::headers;
use azure_core::Method;
use serde::Serialize;

azure_core::operation! {
    /// The CreateOrUpdateModuleIdentityBuilder is used to construct a new module identity
    /// or the update an existing one.
    CreateOrUpdateModuleIdentity,
    client: ServiceClient,
    operation: IdentityOperation,
    device_id: String,
    module_id: String,
    managed_by: String,
    authentication: AuthenticationMechanism,
    etag: Option<String>,
}

impl CreateOrUpdateModuleIdentityBuilder {
    /// Performs the create or update request on the device identity
    pub fn into_future(mut self) -> CreateOrUpdateModuleIdentity {
        Box::pin(async move {
            let uri = format!(
                "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
                self.client.iot_hub_name, self.device_id, self.module_id, API_VERSION
            );

            let mut request = self.client.finalize_request(&uri, Method::Put)?;

            if self.operation == IdentityOperation::Update {
                match &self.etag {
                    Some(etag) => {
                        request.insert_header(headers::IF_MATCH, format!("\"{}\"", etag));
                    }
                    None => return Err(Error::message(ErrorKind::Other, "etag is not set")),
                }
            }

            let body = CreateOrUpdateModuleIdentityBody {
                authentication: self.authentication,
                device_id: self.device_id.as_ref(),
                module_id: self.module_id.as_ref(),
                managed_by: self.managed_by.as_ref(),
                etag: self.etag,
            };

            let body = azure_core::to_json(&body)?;
            request.set_body(body);

            let response = self.client.send(&mut self.context, &mut request).await?;

            CreateOrUpdateModuleIdentityResponse::try_from(response).await
        })
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
