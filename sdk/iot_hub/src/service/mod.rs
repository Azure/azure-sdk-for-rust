use crate::authorization_policy::AuthorizationPolicy;
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::request_options::ContentType;
use azure_core::{
    auth::TokenCredential, prelude::Timeout, ClientOptions, CollectedResponse, Context, Method,
    Pipeline, Request, Response, TimeoutPolicy, Url,
};
use base64::{decode, encode_config};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;

/// Contains any operation that the IoT Hub service client can perform.
pub mod operations;
/// Contains various types that some of the requests or responses use.
pub mod resources;
/// Contains responses for the requests that the IoT Hub service client can perform.
pub mod responses;

use crate::service::operations::{
    ApplyOnEdgeDeviceBuilder, CreateOrUpdateConfigurationBuilder,
    CreateOrUpdateDeviceIdentityBuilder, CreateOrUpdateModuleIdentityBuilder,
    DeleteConfigurationBuilder, DeleteIdentityBuilder, GetIdentityBuilder, GetTwinBuilder,
    InvokeMethodBuilder, QueryBuilder, UpdateOrReplaceTwinBuilder,
};
use crate::service::resources::identity::IdentityOperation;

use self::operations::GetConfigurationBuilder;
use self::resources::{AuthenticationMechanism, Status};

/// The API version to use for any requests
pub const API_VERSION: &str = "2020-05-31-preview";

/// Credential for authorizing requests against IoT Hub
#[derive(Clone)]
pub enum IoTHubCredentials {
    /// Authorize via SAS token
    SASToken(String),
    /// Authorize via BearerToken token
    BearerToken(String),
    /// Authorize using a TokenCredential
    TokenCredential(Arc<dyn TokenCredential>),
}

impl std::fmt::Debug for IoTHubCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            IoTHubCredentials::SASToken(_) => f
                .debug_struct("IoTHubCredentials")
                .field("credential", &"SASToken")
                .finish(),
            IoTHubCredentials::BearerToken(_) => f
                .debug_struct("IoTHubCredentials")
                .field("credential", &"BearerToken")
                .finish(),
            IoTHubCredentials::TokenCredential(_) => f
                .debug_struct("IoTHubCredentials")
                .field("credential", &"TokenCredential")
                .finish(),
        }
    }
}

/// The ServiceClient is the main entry point for communicating with the IoT Hub.
///
/// There are several ways to construct the IoTHub Service object. Either by:
/// - providing the IoT Hub name and the private key.
/// - providing the connection string.
/// The IoTHubService then uses the provided information to create a SAS token that it will
/// use to communicate with the IoT Hub.
#[derive(Clone, Debug)]
pub struct ServiceClient {
    /// The name of the IoT Hub.
    pub iot_hub_name: String,
    pipeline: Pipeline,
}

impl ServiceClient {
    /// Return a new IoTHub struct
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// let iot_hub_name = "cool-iot-hub";
    /// let sas_token = "<a generated sas token>";
    ///
    /// let iot_hub = ServiceClient::new_sas_token(iot_hub_name, sas_token);
    /// ```
    pub fn new_sas_token<S, T>(iot_hub_name: S, sas_token: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let pipeline = new_pipeline_from_options(
            ServiceOptions::default(),
            IoTHubCredentials::SASToken(sas_token.into()),
        );
        Self {
            iot_hub_name: iot_hub_name.into(),
            pipeline,
        }
    }

    /// Generate a new SAS token to use for authentication with IoT Hub
    fn generate_sas_token(
        iot_hub_name: &str,
        key_name: &str,
        private_key: &str,
        expires_in_seconds: u64,
    ) -> azure_core::Result<String> {
        type HmacSHA256 = Hmac<Sha256>;
        let expiry_date = OffsetDateTime::now_utc() + Duration::from_secs(expires_in_seconds);
        let expiry_date_seconds = expiry_date.unix_timestamp();
        let data = format!(
            "{}.azure-devices.net\n{}",
            iot_hub_name, &expiry_date_seconds
        );

        let key = decode(private_key).with_context(ErrorKind::Other, || {
            format!("failed to decode the given private key: {private_key}")
        })?;

        let mut hmac = HmacSHA256::new_from_slice(key.as_ref())
            .with_context(ErrorKind::Other, || {
                format!("failed to use the given private key for the hashing algorithm: {key:?}")
            })?;

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
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// let iot_hub_name = "iot-hub";
    /// let key_name = "iot_hubowner";
    /// let private_key = "YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    ///
    /// let result = ServiceClient::new_private_key(iot_hub_name, key_name, private_key, 3600);
    /// assert!(result.is_ok());
    /// ```
    pub fn new_private_key<S, T, U>(
        iot_hub_name: S,
        key_name: T,
        private_key: U,
        expires_in_seconds: u64,
    ) -> azure_core::Result<Self>
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

        let pipeline = new_pipeline_from_options(
            ServiceOptions::default(),
            IoTHubCredentials::SASToken(sas_token),
        );

        Ok(Self {
            iot_hub_name: iot_hub_name_str,
            pipeline,
        })
    }

    /// Create a new IoTHubService struct based on a given connection string
    ///
    /// The connection string should preferably be from a user / group that has the rights to make service requests.
    /// ```
    /// use std::sync::Arc;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    ///
    /// let result = ServiceClient::new_connection_string(connection_string, 3600);
    /// assert!(result.is_ok());
    /// ```
    pub fn new_connection_string<S>(
        connection_string: S,
        expires_in_seconds: u64,
    ) -> azure_core::Result<Self>
    where
        S: AsRef<str>,
    {
        let parts: Vec<&str> = connection_string.as_ref().split(';').collect();
        let mut iot_hub_name: Option<&str> = None;
        let mut key_name: Option<&str> = None;
        let mut primary_key: Option<&str> = None;

        if parts.len() != 3 {
            return Err(Error::message(
                ErrorKind::Other,
                "given connection string is invalid",
            ));
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

        let iot_hub_name = iot_hub_name.ok_or_else(|| {
            Error::message(
                ErrorKind::Other,
                "failed to get the hostname from the given connection string",
            )
        })?;

        let key_name = key_name.ok_or_else(|| {
            Error::message(
                ErrorKind::Other,
                "failed to get the shared access key name from the given connection string",
            )
        })?;

        let primary_key = primary_key.ok_or_else(|| {
            Error::message(
                ErrorKind::Other,
                "failed to get the primary key from the given connection string",
            )
        })?;

        let sas_token =
            Self::generate_sas_token(iot_hub_name, key_name, primary_key, expires_in_seconds)
                .context(ErrorKind::Other, "generate SAS token error")?;

        let pipeline = new_pipeline_from_options(
            ServiceOptions::default(),
            IoTHubCredentials::SASToken(sas_token),
        );

        Ok(Self {
            iot_hub_name: iot_hub_name.to_string(),
            pipeline,
        })
    }

    /// Create a new IoTHubService struct with a TokenCredential
    pub fn new_token_credential<A>(
        iot_hub_name: A,
        token_credential: Arc<dyn TokenCredential>,
    ) -> Self
    where
        A: Into<String>,
    {
        let credentials = IoTHubCredentials::TokenCredential(token_credential);
        let pipeline = new_pipeline_from_options(ServiceOptions::default(), credentials);

        Self {
            iot_hub_name: iot_hub_name.into(),
            pipeline,
        }
    }

    /// Create a new IoTHubService struct with a BearerToken
    pub fn new_bearer_token<A, BT>(iot_hub_name: A, bearer_token: BT) -> Self
    where
        A: Into<String>,
        BT: Into<String>,
    {
        let credentials = IoTHubCredentials::BearerToken(bearer_token.into());
        let pipeline = new_pipeline_from_options(ServiceOptions::default(), credentials);

        Self {
            iot_hub_name: iot_hub_name.into(),
            pipeline,
        }
    }

    /// Create a new device method
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the service client!");
    /// let device_method = iot_hub.create_device_method("some-device", "hello-world", serde_json::json!({}));
    /// ```
    pub fn create_device_method<S, T>(
        &self,
        device_id: S,
        method_name: T,
        payload: serde_json::Value,
    ) -> operations::InvokeMethodBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        InvokeMethodBuilder::new(self.clone(), device_id.into(), method_name.into(), payload)
    }

    /// Create a new module method
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device_method = iot_hub.create_module_method("some-device", "some-module", "hello-world", serde_json::json!({}));
    /// ```
    pub fn create_module_method<S, T, U>(
        &self,
        device_id: S,
        module_id: T,
        method_name: U,
        payload: serde_json::Value,
    ) -> operations::InvokeMethodBuilder
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        InvokeMethodBuilder::new(self.clone(), device_id.into(), method_name.into(), payload)
            .module_id(module_id.into())
    }

    /// Get the module twin of a given device and module
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.get_module_twin("some-device", "some-module");
    /// ```
    pub fn get_module_twin<S, T>(&self, device_id: S, module_id: T) -> GetTwinBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        GetTwinBuilder::new(self.clone(), device_id.into()).module_id(module_id)
    }

    /// Get the device twin of a given device
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.get_device_twin("some-device");
    /// ```
    pub fn get_device_twin<S>(&self, device_id: S) -> GetTwinBuilder
    where
        S: Into<String>,
    {
        GetTwinBuilder::new(self.clone(), device_id.into())
    }

    /// Update the module twin of a given device or module
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_module_twin("some-device", "some-module")
    ///              .tag("TagName", "TagValue")
    ///              .desired_properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .into_future();
    /// ```
    pub fn update_module_twin<S, T>(&self, device_id: S, module_id: T) -> UpdateOrReplaceTwinBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        UpdateOrReplaceTwinBuilder::new(self.clone(), device_id.into(), Method::Patch)
            .module_id(module_id.into())
    }

    /// Replace the module twin of a given device and module
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.replace_module_twin("some-device", "some-module")
    ///              .tag("TagName", "TagValue")
    ///              .desired_properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .into_future();
    /// ```
    pub fn replace_module_twin<S, T>(
        &self,
        device_id: S,
        module_id: T,
    ) -> UpdateOrReplaceTwinBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        UpdateOrReplaceTwinBuilder::new(self.clone(), device_id.into(), Method::Put)
            .module_id(module_id.into())
    }

    /// Update the device twin of a given device
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.update_device_twin("some-device")
    ///              .tag("TagName", "TagValue")
    ///              .desired_properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .into_future();
    /// ```
    pub fn update_device_twin<S>(&self, device_id: S) -> UpdateOrReplaceTwinBuilder
    where
        S: Into<String>,
    {
        UpdateOrReplaceTwinBuilder::new(self.clone(), device_id.into(), Method::Patch)
    }

    /// Replace the device twin of a given device
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let twin = iot_hub.replace_device_twin("some-device")
    ///              .tag("TagName", "TagValue")
    ///              .desired_properties(serde_json::json!({"PropertyName": "PropertyValue"}))
    ///              .into_future();
    /// ```
    pub fn replace_device_twin<S>(&self, device_id: S) -> UpdateOrReplaceTwinBuilder
    where
        S: Into<String>,
    {
        UpdateOrReplaceTwinBuilder::new(self.clone(), device_id.into(), Method::Put)
    }

    /// Get the identity of a given device
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.get_device_identity("some-device");
    /// ```
    pub fn get_device_identity<S>(&self, device_id: S) -> GetIdentityBuilder
    where
        S: Into<String>,
    {
        GetIdentityBuilder::new(self.clone(), device_id.into())
    }

    /// Create a new device identity
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.create_device_identity("some-existing-device", Status::Enabled, AuthenticationMechanism::new_using_symmetric_key("first-key", "second-key"))
    ///     .into_future();
    /// ```
    pub fn create_device_identity<S>(
        &self,
        device_id: S,
        status: Status,
        authentication: AuthenticationMechanism,
    ) -> CreateOrUpdateDeviceIdentityBuilder
    where
        S: Into<String>,
    {
        CreateOrUpdateDeviceIdentityBuilder::new(
            self.clone(),
            IdentityOperation::Create,
            device_id.into(),
            status,
            authentication,
            None,
        )
    }

    /// Update an existing device identity
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.update_device_identity("some-existing-device", Status::Enabled, AuthenticationMechanism::new_using_symmetric_key("first-key", "second-key"), "etag-of-device-to-update");
    /// ```
    pub fn update_device_identity<S1, S2>(
        &self,
        device_id: S1,
        status: Status,
        authentication: AuthenticationMechanism,
        etag: S2,
    ) -> CreateOrUpdateDeviceIdentityBuilder
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        CreateOrUpdateDeviceIdentityBuilder::new(
            self.clone(),
            IdentityOperation::Update,
            device_id.into(),
            status,
            authentication,
            Some(etag.into()),
        )
    }

    /// Delete a device identity
    ///
    /// The if-match value can either be Some(String) or None. When if-match is None,
    /// an unconditional delete will be performed.
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.delete_device_identity("some-device-id", "some-etag");
    /// ```
    pub fn delete_device_identity<S, T>(&self, device_id: S, if_match: T) -> DeleteIdentityBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        DeleteIdentityBuilder::new(self.clone(), if_match.into(), device_id.into(), None)
    }

    /// Get the identity of a given module
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.get_module_identity("some-device", "some-module");
    /// ```
    pub fn get_module_identity<S, T>(&self, device_id: S, module_id: T) -> GetIdentityBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        GetIdentityBuilder::new(self.clone(), device_id.into()).module_id(module_id)
    }

    /// Create a new module identity
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.create_module_identity("some-existing-device", "some-existing-module", "IoTEdge", AuthenticationMechanism::new_using_symmetric_key("first-key", "second-key")).into_future();
    /// ```
    pub fn create_module_identity<S1, S2, S3>(
        &self,
        device_id: S1,
        module_id: S2,
        managed_by: S3,
        authentication: AuthenticationMechanism,
    ) -> CreateOrUpdateModuleIdentityBuilder
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        CreateOrUpdateModuleIdentityBuilder::new(
            self.clone(),
            IdentityOperation::Create,
            device_id.into(),
            module_id.into(),
            managed_by.into(),
            authentication,
            None,
        )
    }

    /// Update an existing module identity
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.update_module_identity("some-existing-device", "some-existing-module", "IoTEdge", AuthenticationMechanism::new_using_symmetric_key("first-key", "second-key"), "etag-of-device-to-update");
    /// ```
    pub fn update_module_identity<S1, S2, S3, S4>(
        &self,
        device_id: S1,
        module_id: S2,
        managed_by: S3,
        authentication: AuthenticationMechanism,
        etag: S4,
    ) -> CreateOrUpdateModuleIdentityBuilder
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        CreateOrUpdateModuleIdentityBuilder::new(
            self.clone(),
            IdentityOperation::Update,
            device_id.into(),
            module_id.into(),
            managed_by.into(),
            authentication,
            Some(etag.into()),
        )
    }

    /// Create a new device identity
    ///
    /// The if-match value can either be Some(String) or None. When if-match is None,
    /// an unconditional delete will be performed.
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.delete_module_identity("some-device-id", "some-module-id", "some-etag");
    /// ```
    pub fn delete_module_identity<S, T, U>(
        &self,
        device_id: S,
        module_id: T,
        if_match: U,
    ) -> DeleteIdentityBuilder
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        DeleteIdentityBuilder::new(
            self.clone(),
            if_match.into(),
            device_id.into(),
            Some(module_id.into()),
        )
    }

    /// Invoke a query
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let query_builder = iot_hub.query("$SOME_QUERY");
    /// ```
    pub fn query<Q>(&self, query: Q) -> QueryBuilder
    where
        Q: Into<String>,
    {
        QueryBuilder::new(self.clone(), query.into())
    }

    /// Apply configuration on an Edge device
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let edge_configuration_builder = iot_hub.apply_on_edge_device("some-device");
    /// ```
    pub fn apply_on_edge_device<S>(&self, device_id: S) -> ApplyOnEdgeDeviceBuilder
    where
        S: Into<String>,
    {
        ApplyOnEdgeDeviceBuilder::new(self.clone(), device_id.into())
    }

    /// Get a configuration
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.get_configuration("some-configuration");
    /// ```
    pub fn get_configuration<S>(&self, configuration_id: S) -> GetConfigurationBuilder
    where
        S: Into<String>,
    {
        GetConfigurationBuilder::new(self.clone()).configuration_id(configuration_id.into())
    }

    /// Get all configurations
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.get_configurations();
    /// ```
    pub fn get_configurations(&self) -> GetConfigurationBuilder {
        GetConfigurationBuilder::new(self.clone())
    }

    /// Create a new configuration.
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let configuration = iot_hub.create_configuration("some-configuration-id", 10, "tags.environment='test'")
    ///     .into_future();
    /// ```
    pub fn create_configuration<S, T>(
        &self,
        configuration_id: S,
        priority: u64,
        target_condition: T,
    ) -> CreateOrUpdateConfigurationBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        CreateOrUpdateConfigurationBuilder::new(
            self.clone(),
            configuration_id.into(),
            priority,
            target_condition.into(),
            None,
        )
    }

    /// Update a configuration.
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    /// use azure_iot_hub::service::resources::{Status, AuthenticationMechanism};
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let configuration = iot_hub.update_configuration("some-configuration-id", 10, "tags.environment='test'", "some-etag-value")
    ///     .into_future();
    /// ```
    pub fn update_configuration<S, T, U>(
        &self,
        configuration_id: S,
        priority: u64,
        target_condition: T,
        etag: U,
    ) -> CreateOrUpdateConfigurationBuilder
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        CreateOrUpdateConfigurationBuilder::new(
            self.clone(),
            configuration_id.into(),
            priority,
            target_condition.into(),
            Some(etag.into()),
        )
    }

    /// Delete a configuration
    ///
    /// The if-match value can either be Some(String) or None. When if-match is None,
    /// an unconditional delete will be performed.
    ///
    /// ```
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::new_connection_string(connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let device = iot_hub.delete_configuration("some-configuration-id", "some-etag");
    /// ```
    pub fn delete_configuration<S, T>(
        &self,
        configuration_id: S,
        if_match: T,
    ) -> DeleteConfigurationBuilder
    where
        S: Into<String>,
        T: Into<String>,
    {
        DeleteConfigurationBuilder::new(self.clone(), if_match.into(), configuration_id.into())
    }

    /// Prepares a request that can be used by any request builders.
    pub(crate) fn finalize_request(
        &self,
        uri: &str,
        method: Method,
    ) -> azure_core::Result<Request> {
        let mut request = Request::new(Url::parse(uri)?, method);
        request.insert_headers(&ContentType::APPLICATION_JSON);
        Ok(request)
    }

    /// send the request via the request pipeline
    pub async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.pipeline.send(context, request).await
    }
}

/// Create a Pipeline from ServiceOptions
fn new_pipeline_from_options(options: ServiceOptions, credentials: IoTHubCredentials) -> Pipeline {
    let auth_policy: Arc<dyn azure_core::Policy> = Arc::new(AuthorizationPolicy::new(credentials));

    // The `AuthorizationPolicy` must be the **last** retry policy.
    // Policies can change the url and/or the headers, and the `AuthorizationPolicy`
    // must be able to inspect them or the resulting token will be invalid.
    let per_retry_policies = vec![
        Arc::new(options.timeout_policy) as Arc<dyn azure_core::Policy>,
        auth_policy,
    ];

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        options.options,
        Vec::new(),
        per_retry_policies,
    )
}

/// Options to cufigure the ServiceClient
#[derive(Debug, Clone, Default)]
pub struct ServiceOptions {
    options: ClientOptions,
    timeout_policy: TimeoutPolicy,
}

impl ServiceOptions {
    /// set timeout duration for requests
    pub fn set_timeout(&mut self, default_timeout: Timeout) {
        self.timeout_policy = TimeoutPolicy::new(Some(default_timeout))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_connectionstring_success() -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;

        let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ = ServiceClient::new_connection_string(connection_string, 3600)?;
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_incorrect_hostname(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;

        let connection_string = "HostName==cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ = ServiceClient::new_connection_string(connection_string, 3600).is_err();

        let connection_string = "HostName=cool-iot-hub.azure-;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
        let _ = ServiceClient::new_connection_string(connection_string, 3600).is_err();
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_empty_connection_string(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;

        let _ = ServiceClient::new_connection_string("", 3600).is_err();
        Ok(())
    }

    #[test]
    fn from_connectionstring_should_fail_on_incomplete_connection_string(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::service::ServiceClient;
        let _ = ServiceClient::new_connection_string( "HostName=cool-iot-hub.azure-devices.net;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==", 3600).is_err();
        Ok(())
    }
}
