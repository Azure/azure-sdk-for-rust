use serde::{Deserialize, Serialize};

/// Representation of a desired device capability
pub enum DesiredCapability {
    /// The IoT Edge device capability
    IotEdge,
}

/// The connection state of a module or device
#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum ConnectionState {
    /// The device or module is connected
    Connected,
    /// The device or module is disconnected
    Disconnected,
}

/// Device or module status
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// The device or module is disabled
    Disabled,
    /// The device or module is enabled
    Enabled,
}

/// Representation of device capabilities.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct DeviceCapabilities {
    #[serde(rename = "iotEdge")]
    /// Whether the device has the IoT Edge capability or not.
    pub iotedge: bool,
}

/// Representation of a symmetric key for authentication.
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct SymmetricKey {
    /// The primary key.
    pub primary_key: Option<String>,
    /// The secondary key.
    pub secondary_key: Option<String>,
}

/// Representation of a x509 thumbprint for authentication.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct X509ThumbPrint {
    /// The primary thumbprint.
    pub primary_thumbprint: Option<String>,
    /// The secondary thumbprint.
    pub secondary_thumbprint: Option<String>,
}

/// AuthenticationType of a module or device.
#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum AuthenticationType {
    /// Authentication using certificate
    #[serde(rename = "certificate")]
    Certificate,
    /// Authentication using a certificate authority.
    #[serde(rename = "Authority")]
    Authority,
    /// The device or module is not authenticated.
    #[serde(rename = "none")]
    None,
    /// Authentication using symmetric keys
    #[serde(rename = "sas")]
    SAS,
    /// Authentication using self signed certificates
    #[serde(rename = "selfSigned")]
    SelfSigned,
}

/// The authentication mechanism for a device or module identity.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationMechanism {
    /// The symmetric key pair used for authentication.
    pub symmetric_key: SymmetricKey,
    /// The type of authentication that is being used.
    #[serde(rename = "type")]
    pub authentication_type: AuthenticationType,
    /// The primary and secondary x509 thumbprints used for x509 based authentication.
    pub x509_thumbprint: X509ThumbPrint,
}

/// The operation to perform on an identity
#[derive(PartialEq)]
pub(crate) enum IdentityOperation {
    Create,
    Update,
}
