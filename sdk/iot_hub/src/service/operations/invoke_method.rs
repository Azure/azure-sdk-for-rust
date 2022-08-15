use crate::service::responses::InvokeMethodResponse;
use crate::service::{ServiceClient, API_VERSION};
use azure_core::Method;
use serde::Serialize;

azure_core::operation! {
    /// The InvokeMethodBuilder is used for constructing the request to
    /// invoke a module or device method.
    InvokeMethod,
    client: ServiceClient,
    device_id: String,
    method_name: String,
    payload: serde_json::Value,
    ?module_id: String,
    ?response_time_out: u64,
    ?connect_time_out: u64
}

impl InvokeMethodBuilder {
    /// Turn the builder into a `Future`
    pub fn into_future(mut self) -> InvokeMethod {
        Box::pin(async move {
            let uri = match &self.module_id {
                Some(module_id_value) => format!(
                    "https://{}.azure-devices.net/twins/{}/modules/{}/methods?api-version={}",
                    self.client.iot_hub_name, self.device_id, module_id_value, API_VERSION
                ),
                None => format!(
                    "https://{}.azure-devices.net/twins/{}/methods?api-version={}",
                    self.client.iot_hub_name, self.device_id, API_VERSION
                ),
            };

            let mut request = self.client.finalize_request(&uri, Method::Post)?;
            let method = InvokeMethodBody {
                connect_timeout_in_seconds: self.connect_time_out.unwrap_or(15),
                method_name: &self.method_name,
                payload: self.payload,
                response_timeout_in_seconds: self.response_time_out.unwrap_or(15),
            };

            let body = azure_core::to_json(&method)?;

            request.set_body(body);

            let response = self.client.send(&mut self.context, &mut request).await?;

            InvokeMethodResponse::try_from(response).await
        })
    }
}

/// Body for the InvokeMethod request
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InvokeMethodBody<'a> {
    connect_timeout_in_seconds: u64,
    method_name: &'a str,
    payload: serde_json::Value,
    response_timeout_in_seconds: u64,
}
