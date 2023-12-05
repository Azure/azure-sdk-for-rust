use azure_core::error::{Error, ErrorKind};

/// The `Cloud2DeviceResponse` struct contains the response
/// from the `IoTHub` when a message is sent to a device.
pub struct Cloud2DeviceMessageResponse;

impl Cloud2DeviceMessageResponse {
    pub(crate) fn try_from(response: azure_core::Response) -> azure_core::Result<Self> {
        let (status, _headers, _body) = response.deconstruct();
        if status != 204 {
            let message = format!("Invalid status code, expected 204 No Content, found: {status}");
            let error_code = Some(message.clone());
            return Err(Error::message(
                ErrorKind::HttpResponse { status, error_code },
                message,
            ));
        }

        Ok(Self)
    }
}
