use base64::{decode, encode_config};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub mod directmethod;

use crate::service::directmethod::DirectMethod;

pub const API_VERSION: &str = "2020-03-13";

/// The ServiceClient is the main entry point for communicating with the IoT Hub.
///
/// There are several ways to construct the IoTHub Service object. Either by:
/// - providing the IoT Hub name and the private key.
/// - providing the connection string.
/// The IoTHubService then uses the provided information to create a SAS token that it will
/// use to communicate with the IoT Hub.
pub struct ServiceClient {
    pub iothub_name: String,
    pub sas_token: String,
}

impl ServiceClient {
    /// Return a new IoTHub struct
    ///
    /// # Example
    /// ```
    /// use azure_iothub_service::ServiceClient;
    ///
    /// let iothub_name = "cool-iot-hub";
    /// let sas_token = "<a generated sas token>";
    ///
    /// let iothub = ServiceClient::from_sas_token(iothub_name, sas_token);
    /// ```
    pub fn from_sas_token<S, T>(iothub_name: S, sas_token: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            iothub_name: iothub_name.into(),
            sas_token: sas_token.into(),
        }
    }

    /// Generate a new SAS token to use for authentication with IoT Hub
    fn generate_sas_token(
        iothub_name: &str,
        private_key: &str,
        expires_in_seconds: i64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        type HmacSHA256 = Hmac<Sha256>;
        let expiry_date = chrono::Utc::now() + chrono::Duration::seconds(expires_in_seconds);
        let expiry_date_seconds = expiry_date.timestamp();
        let data = format!(
            "{}.azure-devices.net\n{}",
            iothub_name, &expiry_date_seconds
        );

        let key = decode(private_key)?;
        let mut hmac = HmacSHA256::new_varkey(key.as_ref())?;
        hmac.update(data.as_bytes());
        let result = hmac.finalize();
        let sas_token: &str = &encode_config(&result.into_bytes(), base64::STANDARD);

        let encoded: String = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sr", &format!("{}.azure-devices.net", iothub_name))
            .append_pair("sig", sas_token)
            .append_pair("skn", "iothubowner")
            .append_pair("se", &expiry_date_seconds.to_string())
            .finish();

        Ok(format!("SharedAccessSignature {}", encoded))
    }

    /// Create a new IoTHubService struct based on a given IoT Hub name and a private key
    ///
    /// The private key should preferably be of a user / group that has the rights to make service requests.
    /// ```
    /// use azure_iothub_service::IoTHubService;
    ///
    /// let iothub_name = "cool-iot-hub";
    /// let private_key = "YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    ///
    /// let result = IoTHubService::from_private_key(iothub_name, private_key, 3600);
    /// assert!(result.is_ok(), true);
    /// ```
    pub fn from_private_key<S, T>(
        iothub_name: S,
        private_key: T,
        expires_in_seconds: i64,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        S: Into<String>,
        T: AsRef<str>,
    {
        let iothub_name_str = iothub_name.into();

        let sas_token = Self::generate_sas_token(
            iothub_name_str.as_str(),
            private_key.as_ref(),
            expires_in_seconds,
        )?;

        Ok(Self {
            iothub_name: iothub_name_str,
            sas_token,
        })
    }

    /// Create a new IoTHubService struct based on a given connection string
    ///
    /// The connection string should preferably be from a user / group that has the rights to make service requests.
    /// ```
    /// use azure_iothub_service::IoTHubService;
    ///
    /// let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    ///
    /// let result = IoTHubService::from_connection_string(connection_string, 3600);
    /// assert!(result.is_ok(), true);
    /// ```
    pub fn from_connection_string<S>(
        connection_string: S,
        expires_in_seconds: i64,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        S: AsRef<str>,
    {
        let parts: Vec<&str> = connection_string.as_ref().split(';').collect();

        let mut iothub_name: Option<&str> = None;
        let mut primary_key: Option<&str> = None;

        if parts.len() != 3 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Given connection string is invalid",
            )));
        }

        for val in parts.iter() {
            let start = match val.find('=') {
                Some(size) => size + 1,
                None => continue,
            };

            if val.contains("HostName=") {
                let end = match val.find(".azure-devices.net") {
                    Some(size) => size,
                    None => continue,
                };
                iothub_name = Some(&val[start..end])
            }

            if val.contains("SharedAccessKey=") {
                primary_key = Some(&val[start..val.len()])
            }
        }

        let matched_iothub_name = match iothub_name {
            Some(val) => val,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Failed to get the hostname from the given connection string!",
                )));
            }
        };

        let matched_primary_key = match primary_key {
            Some(val) => val,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Failed to get the primary key from the given connection string!",
                )));
            }
        };

        let sas_token =
            Self::generate_sas_token(matched_iothub_name, matched_primary_key, expires_in_seconds)?;

        Ok(Self {
            iothub_name: matched_iothub_name.to_string(),
            sas_token,
        })
    }

    /// Create a new device method
    ///
    /// ```
    /// use azure_iothub_service::IoTHubService;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iothub = IoTHubService::from_connection_string(connection_string, 3600).expect("Failed to create the IoTHubService!");
    /// let device_method = iothub.create_device_method("some-device", "hello-world", 30, 30);
    /// ```
    pub fn create_device_method<S, T>(
        &self,
        device_id: S,
        method_name: T,
        response_time_out: u64,
        connect_time_out: u64,
    ) -> DirectMethod
    where
        S: Into<String>,
        T: Into<String>,
    {
        DirectMethod::new(
            &self,
            device_id.into(),
            None,
            method_name.into(),
            connect_time_out,
            response_time_out,
        )
    }

    /// Create a new module method
    ///
    /// ```
    /// use azure_iothub_service::IoTHubService;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iothub = IoTHubService::from_connection_string(connection_string, 3600).expect("Failed to create the IoTHubService!");
    /// let device_method = iothub.create_module_method("some-device", "some-module", "hello-world", 30, 30);
    /// ```
    pub fn create_module_method<S, T, U>(
        &self,
        device_id: S,
        module_id: T,
        method_name: U,
        response_time_out: u64,
        connect_time_out: u64,
    ) -> DirectMethod
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        DirectMethod::new(
            &self,
            device_id.into(),
            Some(module_id.into()),
            method_name.into(),
            connect_time_out,
            response_time_out,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_connectionstring_success() -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ = ServiceClient::from_connection_string(connection_string, 3600)?;
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_incorrect_hostname(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let connection_string = "HostName==cool-iot-hub.azure-devices.net;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ = ServiceClient::from_connection_string(connection_string, 3600).is_err();

        let connection_string = "HostName=cool-iot-hub.azure-;SharedAccessKeyName=iothubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ = ServiceClient::from_connection_string(connection_string, 3600).is_err();
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_empty_connection_string(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let _ = ServiceClient::from_connection_string("", 3600).is_err();
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_incomplete_connection_string(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let _ = ServiceClient::from_connection_string("HostName=cool-iot-hub.azure-devices.net;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==", 3600).is_err();
        Ok(())
    }
}
