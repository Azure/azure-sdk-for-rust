use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;

use bytes::Bytes;
use http::{Method, Response, StatusCode};

use crate::service::{IoTHubError, ServiceClient, API_VERSION};

/// The GetIdentityBuilder is used to construct a request for creating
/// or updating a module or device identity.
pub struct GetIdentityBuilder<'a, T>
where
    T: TryFrom<Response<Bytes>, Error = IoTHubError>,
{
    service_client: &'a ServiceClient,
    device_id: String,
    module_id: Option<String>,
    return_type: PhantomData<T>,
}

impl<'a, T> GetIdentityBuilder<'a, T>
where
    T: TryFrom<Response<Bytes>, Error = IoTHubError>,
{
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        device_id: String,
        module_id: Option<String>,
    ) -> Self {
        Self {
            service_client,
            device_id,
            module_id,
            return_type: PhantomData,
        }
    }

    /// Execute the request to create or update the module or device identity.
    pub async fn execute(&self) -> Result<T, IoTHubError> {
        let uri = match &self.module_id {
            Some(module_id) => format!(
                "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
                self.service_client.iothub_name, self.device_id, module_id, API_VERSION
            ),
            None => format!(
                "https://{}.azure-devices.net/devices/{}?api-version={}",
                self.service_client.iothub_name, self.device_id, API_VERSION
            ),
        };

        let request = self.service_client.prepare_request(&uri, Method::GET);
        let request = request.body(bytes::Bytes::from_static(azure_core::EMPTY_BODY))?;

        Ok(self
            .service_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
