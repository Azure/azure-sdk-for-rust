use crate::service::responses::InvokeMethodResponse;
use crate::service::{ServiceClient, API_VERSION};
use azure_core::Method;
use serde::Serialize;
use std::convert::TryInto;

azure_core::operation! {
    /// The InvokeMethodBuilder is used for constructing the request to
    /// invoke a module or device method.
    InvokeMethod,
    client: ServiceClient,
    payload: serde_json::Value,
    device_id: String,
    module_id: Option<String>,
    method_name: String,
    response_time_out: u64,
    connect_time_out: u64,
}

impl InvokeMethodBuilder {
    /// Invoke the DirectMethod
    ///
    /// Either a module method, or device method is invoked based on the
    /// way the DirectMethod was created. On invocation a DirectMethodResponse
    /// is returned. This does not mean the invocation was successfull. The status
    /// code within the DirectMethodResponse should still be verified.
    ///
    /// # Examples
    /// ```
    /// # use std::sync::Arc;
    /// # use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    /// # let http_client = azure_core::new_http_client();
    ///
    /// let service = ServiceClient::from_sas_token(http_client, "some-iot-hub", "sas_token");
    /// let great_method = service.create_device_method(
    ///    "SomeDeviceId",
    ///    "GreatMethod",
    ///    100,
    ///    60
    /// );
    ///
    /// great_method.execute(serde_json::json!({"hello": "world"}));
    /// ```
    pub fn into_future(self) -> InvokeMethod {
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
                connect_timeout_in_seconds: self.connect_time_out,
                method_name: &self.method_name,
                payload: self.payload,
                response_timeout_in_seconds: self.response_time_out,
            };

            let body = azure_core::to_json(&method)?;

            request.set_body(body);

            self.client
                .http_client()
                .execute_request_check_status(&request)
                .await?
                .try_into()
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
