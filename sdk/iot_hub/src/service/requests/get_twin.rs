use crate::service::{ServiceClient, API_VERSION};
use azure_core::error::{Error, ErrorKind, Result, ResultExt};
use bytes::Bytes;
use http::{Method, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;

/// Execute the request to get the twin of a module or device.
pub(crate) async fn get_twin<T>(
    service_client: &ServiceClient,
    device_id: String,
    module_id: Option<String>,
) -> Result<T>
where
    T: TryFrom<Response<Bytes>, Error = Error>,
{
    let uri = match module_id {
        Some(val) => format!(
            "https://{}.azure-devices.net/twins/{}/modules/{}?api-version={}",
            service_client.iot_hub_name, device_id, val, API_VERSION
        ),
        None => format!(
            "https://{}.azure-devices.net/twins/{}?api-version={}",
            service_client.iot_hub_name, device_id, API_VERSION
        ),
    };

    let request = service_client.prepare_request(&uri, Method::GET);
    let request = request
        .body(azure_core::EMPTY_BODY)
        .map_kind(ErrorKind::Io)?;

    service_client
        .http_client()
        .execute_request_check_status(request, StatusCode::OK)
        .await?
        .try_into()
}
