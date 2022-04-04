use std::sync::Arc;

use azure_core::HttpClient;
use base64::{decode, encode_config};
use hmac::{Hmac, Mac};
use http::request::Builder as RequestBuilder;
use http::{header, Method};
use sha2::Sha256;

/// The requests module contains any request that the IoT Hub service client can perform.
pub mod requests;
/// The resources module contains various types that some of the requests or responses use.
pub mod resources;
/// The response module contains responses for the requests that the IoT Hub service client can perform.
pub mod responses;

use crate::service::requests::{
    get_identity, get_twin, CreateOrUpdateDeviceIdentityBuilder,
    CreateOrUpdateModuleIdentityBuilder, DeleteIdentityBuilder, InvokeMethodBuilder, QueryBuilder,
    UpdateOrReplaceTwinBuilder,
};
use crate::service::resources::identity::IdentityOperation;
use crate::service::responses::{
    DeviceIdentityResponse, DeviceTwinResponse, ModuleIdentityResponse, ModuleTwinResponse,
};

/// The API version to use for any requests
pub const API_VERSION: &str = "2020-05-31-preview";

/// The ServiceClient is the main entry point for communicating with the IoT Hub.
///
/// There are several ways to construct the IoTHub Service object. Either by:
/// - providing the IoT Hub name and the private key.
/// - providing the connection string.
/// The IoTHubService then uses the provided information to create a SAS token that it will
/// use to communicate with the IoT Hub.
pub struct ServiceClient {
    http_client: Arc<dyn HttpClient>,
    /// The name of the IoT Hub.
    pub iot_hub_name: String,
    /// The SAS token that is used for authentication.
    pub(crate) sas_token: String,
}

#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum FromConnectionStringError {
    #[error("Given connection string is invalid")]
    InvalidError,
    #[error("Failed to get the hostname from the given connection string")]
    FailedToGetHostname,
    #[error("Failed to get the shared access key name from the given connection string")]
    FailedToGetSharedAccessKey,
    #[error("Failed to get the primary key from the given connection string")]
    FailedToGetPrimaryKey,
    #[error("Generate SAS token error: {0}")]
    GenerateSasTokenError(GenerateSasTokenError),
}

#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum GenerateSasTokenError {
    #[error("Failed to decode the given private key: {0}")]
    DecodePrivateKeyError(base64::DecodeError),
    #[error("Failed to use the given private key for the hashing algorithm: {0}")]
    HashingFailed(hmac::digest::InvalidLength),
}

impl ServiceClient {
    /// Return a new IoTHub struct
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// let http_client = azure_core::new_http_client();
    /// let iot_hub_name = "cool-iot-hub";
    /// let sas_token = "<a generated sas token>";
    ///
    /// let iot_hub = ServiceClient::from_sas_token(http_client, iot_hub_name, sas_token);
    /// ```
    pub fn from_sas_token<S, T>(
        http_client: Arc<dyn HttpClient>,
        iot_hub_name: S,
        sas_token: T,
    ) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            http_client,
            iot_hub_name: iot_hub_name.into(),
            sas_token: sas_token.into(),
        }
    }

    /// Generate a new SAS token to use for authentication with IoT Hub
    fn generate_sas_token(
        iot_hub_name: &str,
        key_name: &str,
        private_key: &str,
        expires_in_seconds: i64,
    ) -> Result<String, GenerateSasTokenError> {
        type HmacSHA256 = Hmac<Sha256>;
        let expiry_date = chrono::Utc::now() + chrono::Duration::seconds(expires_in_seconds);
        let expiry_date_seconds = expiry_date.timestamp();
        let data = format!(
            "{}.azure-devices.net\n{}",
            iot_hub_name, &expiry_date_seconds
        );

        let key = decode(private_key).map_err(GenerateSasTokenError::DecodePrivateKeyError)?;

        let mut hmac = HmacSHA256::new_from_slice(key.as_ref())
            .map_err(GenerateSasTokenError::HashingFailed)?;

        hmac.update(data.as_bytes());
        let result = hmac.finalize();
        let sas_token: &str = &encode_config(&result.into_bytes(), base64::STANDARD);

        let encoded: String = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sr", &format!("{}.azure-devices.net", iot_hub_name))
            .append_pair("sig", sas_token)
            .append_pair("skn", key_name)
            .append_pair("se", &expiry_date_seconds.to_string())
            .finish();

        Ok(format!("SharedAccessSignature {}", encoded))
    }

    /// Create a new IoTHubService struct based on a given IoT Hub name and a private key
    ///
    /// The private key should preferably be of a user / group that has the rights to make service requests.
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// let http_client = azure_core::new_http_client();
    ///
    /// let iot_hub_name = "iot-hub";
    /// let key_name = "iot_hubowner";
    /// let private_key = "YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    ///
    /// let result = ServiceClient::from_private_key(http_client, iot_hub_name, key_name, private_key, 3600);
    /// assert!(result.is_ok());
    /// ```
    pub fn from_private_key<S, T, U>(
        http_client: Arc<dyn HttpClient>,
        iot_hub_name: S,
        key_name: T,
        private_key: U,
        expires_in_seconds: i64,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        S: Into<String>,
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let iot_hub_name_str = iot_hub_name.into();

        let sas_token = Self::generate_sas_token(
            iot_hub_name_str.as_str(),
            key_name.as_ref(),
            private_key.as_ref(),
            expires_in_seconds,
        )?;

        Ok(Self {
            http_client,
            iot_hub_name: iot_hub_name_str,
            sas_token,
        })
    }

    /// Create a new IoTHubService struct based on a given connection string
    ///
    /// The connection string should preferably be from a user / group that has the rights to make service requests.
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// let http_client = azure_core::new_http_client();
    /// let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    ///
    /// let result = ServiceClient::from_connection_string(http_client, connection_string, 3600);
    /// assert!(result.is_ok());
    /// ```
    pub fn from_connection_string<S>(
        http_client: Arc<dyn HttpClient>,
        connection_string: S,
        expires_in_seconds: i64,
    ) -> Result<Self, FromConnectionStringError>
    where
        S: AsRef<str>,
    {
        let parts: Vec<&str> = connection_string.as_ref().split(';').collect();
        let mut iot_hub_name: Option<&str> = None;
        let mut key_name: Option<&str> = None;
        let mut primary_key: Option<&str> = None;

        if parts.len() != 3 {
            return Err(FromConnectionStringError::InvalidError);
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
                iot_hub_name = Some(&val[start..end]);
            }

            if val.contains("SharedAccessKeyName=") {
                key_name = Some(&val[start..]);
            }

            if val.contains("SharedAccessKey=") {
                primary_key = Some(&val[start..]);
            }
        }

        let iot_hub_name = iot_hub_name.ok_or(FromConnectionStringError::FailedToGetHostname)?;

        let key_name = key_name.ok_or(FromConnectionStringError::FailedToGetSharedAccessKey)?;

        let primary_key = primary_key.ok_or(FromConnectionStringError::FailedToGetPrimaryKey)?;

        let sas_token =
            Self::generate_sas_token(iot_hub_name, key_name, primary_key, expires_in_seconds)
                .map_err(FromConnectionStringError::GenerateSasTokenError)?;

        Ok(Self {
            http_client,
            iot_hub_name: iot_hub_name.to_string(),
            sas_token,
        })
    }

    /// Create a new device method
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the service client!");
    /// let device_method = iot_hub.create_device_method("some-device", "hello-world", 30, 30);
    /// ```
    pub fn create_device_method<S, T>(
        &self,
        device_id: S,
        method_name: T,
        response_time_out: u64,
        connect_time_out: u64,
    ) -> requests::InvokeMethodBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        InvokeMethodBuilder::new(
            self,
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
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device_method = iot_hub.create_module_method("some-device", "some-module", "hello-world", 30, 30);
    /// ```
    pub fn create_module_method<S, T, U>(
        &self,
        device_id: S,
        module_id: T,
        method_name: U,
        response_time_out: u64,
        connect_time_out: u64,
    ) -> requests::InvokeMethodBuilder
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        InvokeMethodBuilder::new(
            self,
            device_id.into(),
            Some(module_id.into()),
            method_name.into(),
            connect_time_out,
            response_time_out,
        )
    }

    /// Get the module twin of a given device and module
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.get_module_twin("some-device", "some-module");
    /// ```
    pub async fn get_module_twin<S, T>(
        &self,
        device_id: S,
        module_id: T,
    ) -> crate::Result<ModuleTwinResponse>
    where
        S: Into<String>,
        T: Into<String>,
    {
        get_twin(self, device_id.into(), Some(module_id.into())).await
    }

    /// Get the HttpClient of the IoTHub service
    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.http_client.as_ref()
    }

    /// Get the device twin of a given device
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.get_device_twin("some-device");
    /// ```
    pub async fn get_device_twin<S>(&self, device_id: S) -> crate::Result<DeviceTwinResponse>
    where
        S: Into<String>,
    {
        get_twin(self, device_id.into(), None).await
    }

    /// Update the module twin of a given device or module
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_module_twin("some-device", "some-module")
    ///              .tag("TagName", "TagValue")
    ///              .properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .execute();
    /// ```
    pub fn update_module_twin<S, T>(
        &self,
        device_id: S,
        module_id: T,
    ) -> UpdateOrReplaceTwinBuilder<'_, ModuleTwinResponse>
    where
        S: Into<String>,
        T: Into<String>,
    {
        UpdateOrReplaceTwinBuilder::new(
            self,
            device_id.into(),
            Some(module_id.into()),
            Method::PATCH,
        )
    }

    /// Replace the module twin of a given device and module
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.replace_module_twin("some-device", "some-module")
    ///              .tag("TagName", "TagValue")
    ///              .properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .execute();
    /// ```
    pub fn replace_module_twin<S, T>(
        &self,
        device_id: S,
        module_id: T,
    ) -> UpdateOrReplaceTwinBuilder<'_, ModuleTwinResponse>
    where
        S: Into<String>,
        T: Into<String>,
    {
        UpdateOrReplaceTwinBuilder::new(self, device_id.into(), Some(module_id.into()), Method::PUT)
    }

    /// Update the device twin of a given device
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_device_twin("some-device")
    ///              .tag("TagName", "TagValue")
    ///              .properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .execute();
    /// ```
    pub fn update_device_twin<S>(
        &self,
        device_id: S,
    ) -> UpdateOrReplaceTwinBuilder<'_, DeviceTwinResponse>
    where
        S: Into<String>,
    {
        UpdateOrReplaceTwinBuilder::new(self, device_id.into(), None, Method::PATCH)
    }

    /// Replace the device twin of a given device
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.replace_device_twin("some-device")
    ///              .tag("TagName", "TagValue")
    ///              .properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .execute();
    /// ```
    pub fn replace_device_twin<S>(
        &self,
        device_id: S,
    ) -> UpdateOrReplaceTwinBuilder<'_, DeviceTwinResponse>
    where
        S: Into<String>,
    {
        UpdateOrReplaceTwinBuilder::new(self, device_id.into(), None, Method::PUT)
    }

    /// Get the identity of a given device
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.get_device_identity("some-device");
    /// ```
    pub async fn get_device_identity<S>(
        &self,
        device_id: S,
    ) -> crate::Result<DeviceIdentityResponse>
    where
        S: Into<String>,
    {
        get_identity(self, device_id.into(), None).await
    }

    /// Create a new device identity
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.create_device_identity()
    ///     .execute("some-existing-device", Status::Enabled, AuthenticationMechanism::new_using_symmetric_key("first-key", "second-key"));
    /// ```
    pub fn create_device_identity(&self) -> CreateOrUpdateDeviceIdentityBuilder {
        CreateOrUpdateDeviceIdentityBuilder::new(self, IdentityOperation::Create, None)
    }

    /// Update an existing device identity
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.update_device_identity("etag-of-device-to-update")
    ///     .execute("some-existing-device", Status::Enabled, AuthenticationMechanism::new_using_symmetric_key("first-key", "second-key"));
    /// ```
    pub fn update_device_identity<S>(&self, etag: S) -> CreateOrUpdateDeviceIdentityBuilder
    where
        S: Into<String>,
    {
        CreateOrUpdateDeviceIdentityBuilder::new(self, IdentityOperation::Update, Some(etag.into()))
    }

    /// Create a new device identity
    ///
    /// The if-match value can either be Some(String) or None. When if-match is None,
    /// an unconditional delete will be performed.
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.delete_device_identity("some-device-id", "some-etag");
    /// ```
    pub fn delete_device_identity<S, T>(
        &self,
        device_id: S,
        if_match: T,
    ) -> DeleteIdentityBuilder<'_>
    where
        S: Into<String>,
        T: Into<String>,
    {
        DeleteIdentityBuilder::new(self, if_match.into(), device_id.into(), None)
    }

    /// Get the identity of a given module
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.get_module_identity("some-device", "some-module");
    /// ```
    pub async fn get_module_identity<S, T>(
        &self,
        device_id: S,
        module_id: T,
    ) -> crate::Result<ModuleIdentityResponse>
    where
        S: Into<String>,
        T: Into<String>,
    {
        get_identity(self, device_id.into(), Some(module_id.into())).await
    }

    /// Create a new module identity
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.create_module_identity()
    ///     .execute("some-existing-device", "some-existing-module", "IoTEdge", AuthenticationMechanism::new_using_symmetric_key("first-key", "second-key"));
    /// ```
    pub fn create_module_identity(&self) -> CreateOrUpdateModuleIdentityBuilder {
        CreateOrUpdateModuleIdentityBuilder::new(self, IdentityOperation::Create, None)
    }

    /// Update an existing module identity
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.update_module_identity("etag-of-device-to-update")
    ///     .execute("some-existing-device", "some-existing-module", "IoTEdge", AuthenticationMechanism::new_using_symmetric_key("first-key", "second-key"));
    /// ```
    pub fn update_module_identity<S>(&self, etag: S) -> CreateOrUpdateModuleIdentityBuilder
    where
        S: Into<String>,
    {
        CreateOrUpdateModuleIdentityBuilder::new(self, IdentityOperation::Update, Some(etag.into()))
    }

    /// Create a new device identity
    ///
    /// The if-match value can either be Some(String) or None. When if-match is None,
    /// an unconditional delete will be performed.
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.delete_module_identity("some-device-id", "some-module-id", "some-etag");
    /// ```
    pub fn delete_module_identity<S, T, U>(
        &self,
        device_id: S,
        module_id: T,
        if_match: U,
    ) -> DeleteIdentityBuilder<'_>
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        DeleteIdentityBuilder::new(
            self,
            if_match.into(),
            device_id.into(),
            Some(module_id.into()),
        )
    }

    /// Invoke a query
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let query_builder = iot_hub.query();
    /// ```
    pub fn query(&self) -> QueryBuilder<'_> {
        QueryBuilder::new(self)
    }

    /// Prepares a request that can be used by any request builders.
    pub(crate) fn prepare_request(&self, uri: &str, method: Method) -> RequestBuilder {
        RequestBuilder::new()
            .uri(uri)
            .method(method)
            .header(header::AUTHORIZATION, &self.sas_token)
            .header(header::CONTENT_TYPE, "application/json")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_connectionstring_success() -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;

        let http_client = azure_core::new_http_client();
        let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ = ServiceClient::from_connection_string(http_client, connection_string, 3600)?;
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_incorrect_hostname(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;

        let http_client = azure_core::new_http_client();
        let connection_string = "HostName==cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ = ServiceClient::from_connection_string(http_client.clone(), connection_string, 3600)
            .is_err();

        let connection_string = "HostName=cool-iot-hub.azure-;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ =
            ServiceClient::from_connection_string(http_client, connection_string, 3600).is_err();
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_empty_connection_string(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let http_client = azure_core::new_http_client();

        let _ = ServiceClient::from_connection_string(http_client, "", 3600).is_err();
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_incomplete_connection_string(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let http_client = azure_core::new_http_client();

        let _ = ServiceClient::from_connection_string(http_client, "HostName=cool-iot-hub.azure-devices.net;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==", 3600).is_err();
        Ok(())
    }

    #[test]
    fn update_module_twin_should_create_builder() -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let http_client = azure_core::new_http_client();

        let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let service_client =
            ServiceClient::from_connection_string(http_client, connection_string, 3600)?;

        let builder = service_client.update_module_twin("deviceid", "moduleid");
        assert_eq!(builder.device_id, "deviceid".to_string());
        assert_eq!(builder.module_id, Some("moduleid".to_string()));
        assert_eq!(builder.method, http::Method::PATCH);

        Ok(())
    }

    #[test]
    fn replace_module_twin_should_create_builder() -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let http_client = azure_core::new_http_client();

        let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let service_client =
            ServiceClient::from_connection_string(http_client, connection_string, 3600)?;

        let builder = service_client.replace_module_twin("deviceid", "moduleid");
        assert_eq!(builder.device_id, "deviceid".to_string());
        assert_eq!(builder.module_id, Some("moduleid".to_string()));
        assert_eq!(builder.method, http::Method::PUT);

        Ok(())
    }

    #[test]
    fn update_device_twin_should_create_builder() -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let http_client = azure_core::new_http_client();

        let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let service_client =
            ServiceClient::from_connection_string(http_client, connection_string, 3600)?;

        let builder = service_client.update_device_twin("deviceid");
        assert_eq!(builder.device_id, "deviceid".to_string());
        assert_eq!(builder.module_id, None);
        assert_eq!(builder.method, http::Method::PATCH);

        Ok(())
    }

    #[test]
    fn replace_device_twin_should_create_builder() -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let http_client = azure_core::new_http_client();

        let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let service_client =
            ServiceClient::from_connection_string(http_client, connection_string, 3600)?;

        let builder = service_client.replace_device_twin("deviceid");
        assert_eq!(builder.device_id, "deviceid".to_string());
        assert_eq!(builder.module_id, None);
        assert_eq!(builder.method, http::Method::PUT);

        Ok(())
    }
}
