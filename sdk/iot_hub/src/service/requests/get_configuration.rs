use crate::service::{ServiceClient, API_VERSION};
use azure_core::error::Error;
use http::Method;
use std::convert::{TryFrom, TryInto};

/// Execute the request to get the configuration of a given identifier.
pub(crate) async fn get_configuration<T>(
    service_client: &ServiceClient,
    configuration_id: Option<String>,
) -> azure_core::Result<T>
where
    T: TryFrom<crate::service::CollectedResponse, Error = Error>,
{
    let uri = match configuration_id {
        Some(val) => format!(
            "https://{}.azure-devices.net/configurations/{}?api-version={}",
            service_client.iot_hub_name, val, API_VERSION
        ),
        None => format!(
            "https://{}.azure-devices.net/configurations?api-version={}",
            service_client.iot_hub_name, API_VERSION
        ),
    };

    let mut request = service_client.prepare_request(&uri, Method::GET)?;
    request.set_body(azure_core::EMPTY_BODY);

    service_client
        .http_client()
        .execute_request_check_status(&request)
        .await?
        .try_into()
}
