use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json::json;

use crate::service::{ServiceClient, API_VERSION};

/// The DirectMethodResponse struct contains the response
/// from the IoT Hub when a direct method was invoked.
#[derive(Deserialize)]
pub struct DirectMethodResponse {
    pub status: u64,
    pub payload: Option<String>,
}

/// The DirectMethod struct contains all neccessary properties
/// to be able to invoke the method.
pub struct DirectMethod<'a> {
    iothub_service: &'a ServiceClient,
    device_id: String,
    module_id: Option<String>,
    method_name: String,
    connect_time_out: u64,
    response_time_out: u64,
}

impl<'a> DirectMethod<'a> {
    /// Create a new DirectMethod
    pub(crate) fn new(
        iothub_service: &'a ServiceClient,
        device_id: String,
        module_id: Option<String>,
        method_name: String,
        response_time_out: u64,
        connect_time_out: u64,
    ) -> Self {
        Self {
            iothub_service,
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
    /// # use serde_json::json;
    /// use iothub::service::ServiceClient;
    ///
    /// let service = ServiceClient::from_sas_token("some-iot-hub", "sas_token");
    /// let great_method = service.create_device_method(
    ///    "SomeDeviceId",
    ///    "GreatMethod",
    ///    100,
    ///    60
    /// );
    ///
    /// great_method.execute(json!({"hello": "world"}));
    /// ```
    pub async fn execute(
        &self,
        payload: serde_json::Value,
    ) -> Result<DirectMethodResponse, AzureError> {
        let uri = match &self.module_id {
            Some(module_id_value) => format!(
                "https://{}.azure-devices.net/twins/{}/modules/{}/methods?api-version={}",
                self.iothub_service.iothub_name, self.device_id, module_id_value, API_VERSION
            ),
            None => format!(
                "https://{}.azure-devices.net/twins/{}/methods?api-version={}",
                self.iothub_service.iothub_name, self.device_id, API_VERSION
            ),
        };

        Ok(self.invoke_method(&uri, payload).await?)
    }

    /// Helper method for invoking the method
    async fn invoke_method(
        &self,
        uri: &str,
        payload: serde_json::Value,
    ) -> Result<DirectMethodResponse, AzureError> {
        let json_payload = json!({
            "connectTimeoutInSeconds": self.connect_time_out,
            "methodName": self.method_name,
            "payload": payload,
            "responseTimeoutInSeconds": self.response_time_out,
        });

        let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
        let request = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header("Authorization", &self.iothub_service.sas_token)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&json_payload)?))?;

        let (status_code, _, whole_body) =
            extract_status_headers_and_body(client.request(request)).await?;
        if !status_code.is_success() {
            return Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                StatusCode::OK,
                status_code,
                std::str::from_utf8(&whole_body)?,
            )));
        }

        Ok(serde_json::from_slice::<DirectMethodResponse>(&whole_body)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::service::ServiceClient;

    #[test]
    fn directmethod_new_should_succeed() {
        use crate::service::DirectMethod;

        let service: ServiceClient = ServiceClient::from_sas_token("test", "test");
        let direct_method = DirectMethod::new(
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
