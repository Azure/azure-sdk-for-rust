use std::convert::TryFrom;
use std::convert::TryInto;

use bytes::Bytes;
use http::{Method, Response, StatusCode};

use crate::service::{ServiceClient, API_VERSION};

/// Execute the request to get the twin of a module or device.
pub(crate) async fn get_twin<'a, T>(
    service_client: &'a ServiceClient,
    device_id: String,
    module_id: Option<String>,
) -> Result<T, crate::Error>
where
    T: TryFrom<Response<Bytes>, Error = crate::Error>,
{
    let uri = match module_id {
        Some(val) => format!(
            "https://{}.azure-devices.net/twins/{}/modules/{}?api-version={}",
            service_client.iothub_name, device_id, val, API_VERSION
        ),
        None => format!(
            "https://{}.azure-devices.net/twins/{}?api-version={}",
            service_client.iothub_name, device_id, API_VERSION
        ),
    };

    let request = service_client.prepare_request(&uri, Method::GET);
    let request = request.body(bytes::Bytes::from_static(azure_core::EMPTY_BODY))?;

    Ok(service_client
        .http_client()
        .execute_request_check_status(request, StatusCode::OK)
        .await?
        .try_into()?)
}
