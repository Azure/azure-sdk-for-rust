use std::convert::{TryFrom, TryInto};

use bytes::Bytes;
use http::{Method, Response, StatusCode};

use crate::service::{ServiceClient, API_VERSION};

/// Execute the request to get the identity of a device or module.
pub(crate) async fn get_identity<T>(
    service_client: &ServiceClient,
    device_id: String,
    module_id: Option<String>,
) -> crate::Result<T>
where
    T: TryFrom<Response<Bytes>, Error = crate::Error>,
{
    let uri = match module_id {
        Some(module_id) => format!(
            "https://{}.azure-devices.net/devices/{}/modules/{}?api-version={}",
            service_client.iot_hub_name, device_id, module_id, API_VERSION
        ),
        None => format!(
            "https://{}.azure-devices.net/devices/{}?api-version={}",
            service_client.iot_hub_name, device_id, API_VERSION
        ),
    };

    let request = service_client.prepare_request(&uri, Method::GET);
    let request = request.body(azure_core::EMPTY_BODY)?;

    service_client
        .http_client()
        .execute_request_check_status(request, StatusCode::OK)
        .await?
        .try_into()
}
