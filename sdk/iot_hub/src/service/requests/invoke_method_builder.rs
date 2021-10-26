use std::convert::TryInto;

use http::{Method, StatusCode};
use serde::Serialize;

use crate::service::responses::InvokeMethodResponse;
use crate::service::{ServiceClient, API_VERSION};

/// The InvokeMethodBuilder is used for constructing the request to
/// invoke a module or device method.
pub struct InvokeMethodBuilder<'a> {
    iot_hub_service: &'a ServiceClient,
    device_id: String,
    module_id: Option<String>,
    method_name: String,
    connect_time_out: u64,
    response_time_out: u64,
}

impl<'a> InvokeMethodBuilder<'a> {
    /// Create a new DirectMethod
    pub(crate) fn new(
        iot_hub_service: &'a ServiceClient,
        device_id: String,
        module_id: Option<String>,
        method_name: String,
        response_time_out: u64,
        connect_time_out: u64,
    ) -> Self {
        Self {
            iot_hub_service,
            device_id,
            module_id,
            method_name,
            connect_time_out,
            response_time_out,
        }
    }

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
    /// use iot_hub::service::ServiceClient;
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
    pub async fn execute(
        &self,
        payload: serde_json::Value,
    ) -> Result<InvokeMethodResponse, crate::Error> {
        let uri = match &self.module_id {
            Some(module_id_value) => format!(
                "https://{}.azure-devices.net/twins/{}/modules/{}/methods?api-version={}",
                self.iot_hub_service.iot_hub_name, self.device_id, module_id_value, API_VERSION
            ),
            None => format!(
                "https://{}.azure-devices.net/twins/{}/methods?api-version={}",
                self.iot_hub_service.iot_hub_name, self.device_id, API_VERSION
            ),
        };

        let request = self.iot_hub_service.prepare_request(&uri, Method::POST);
        let method = InvokeMethodBody {
            connect_timeout_in_seconds: self.connect_time_out,
            method_name: &self.method_name,
            payload,
            response_timeout_in_seconds: self.response_time_out,
        };

        let body = azure_core::to_json(&method)?;

        let request = request.body(body)?;

        Ok(self
            .iot_hub_service
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
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

#[cfg(test)]
mod tests {
    use crate::service::ServiceClient;

    #[test]
    fn directmethod_new_should_succeed() {
        use crate::service::InvokeMethodBuilder;

        let http_client = azure_core::new_http_client();
        let service: ServiceClient = ServiceClient::from_sas_token(http_client, "test", "test");
        let direct_method = InvokeMethodBuilder::new(
            &service,
            "SomeDevice".to_string(),
            None,
            "GreatMethod".to_string(),
            20,
            10,
        );
        assert_eq!(direct_method.device_id, "SomeDevice");
        assert_eq!(direct_method.module_id, None);
        assert_eq!(direct_method.method_name, "GreatMethod");
        assert_eq!(direct_method.connect_time_out, 10);
        assert_eq!(direct_method.response_time_out, 20);
    }
}
