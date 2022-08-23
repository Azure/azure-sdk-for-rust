#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The active directory group user definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdGroupUser {
    #[serde(flatten)]
    pub user: User,
    #[doc = "The AAD tenant ID of the AD Group."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "The AAD object ID of the AD Group."]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl AdGroupUser {
    pub fn new(user: User, tenant_id: String, object_id: String) -> Self {
        Self {
            user,
            tenant_id,
            object_id,
        }
    }
}
#[doc = "Can be anything: string, number, array, object, etc. (except `null`)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnyValue {}
impl AnyValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API access token definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiToken {
    #[serde(flatten)]
    pub permission: Permission,
    #[doc = "Unique ID of the API token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Value of the API token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "String-formatted date representing the time when the token expires."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
}
impl ApiToken {
    pub fn new(permission: Permission) -> Self {
        Self {
            permission,
            id: None,
            token: None,
            expiry: None,
        }
    }
}
#[doc = "The paged results of API tokens."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiTokenCollection {
    #[doc = "The collection of API tokens."]
    pub value: Vec<ApiToken>,
    #[doc = "URL to get the next page of API tokens."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiTokenCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiTokenCollection {
    pub fn new(value: Vec<ApiToken>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attestation {
    #[doc = "Type of the attestation."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl Attestation {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The paged results of entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    #[doc = "The collection of entities."]
    pub value: Vec<serde_json::Value>,
    #[doc = "URL to get the next page of entities."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl Collection {
    pub fn new(value: Vec<serde_json::Value>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The device definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Device {
    #[doc = "Unique ID of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "ETag used to prevent conflict in device updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Display name of the device."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The device template definition for the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "Whether the device connection to IoT Central has been enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Whether resources have been allocated for the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<bool>,
    #[doc = "Whether the device is simulated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulated: Option<bool>,
    #[doc = "List of organization IDs that the device is a part of, only one organization support today, multiple organizations will be supported soon."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<String>,
}
impl Device {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCollection {
    #[doc = "The collection of devices."]
    pub value: Vec<Device>,
    #[doc = "URL to get the next page of devices."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceCollection {
    pub fn new(value: Vec<Device>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The device command definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceCommand {
    #[doc = "The request ID of the device command execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Connection timeout in seconds to wait for a disconnected device to come online. Defaults to 0 seconds."]
    #[serde(rename = "connectionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i64>,
    #[doc = "Response timeout in seconds to wait for a command completion on a device. Defaults to 30 seconds."]
    #[serde(rename = "responseTimeout", default, skip_serializing_if = "Option::is_none")]
    pub response_timeout: Option<i64>,
    #[doc = "The payload for the device command, support any primitive types or object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
    #[doc = "The payload of the device command response, support any primitive types or object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,
    #[doc = "The status code of the device command response."]
    #[serde(rename = "responseCode", default, skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
}
impl DeviceCommand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of device command executions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCommandCollection {
    #[doc = "The collection of device command executions."]
    pub value: Vec<DeviceCommand>,
    #[doc = "URL to get the next page of device command executions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceCommandCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceCommandCollection {
    pub fn new(value: Vec<DeviceCommand>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The device credentials definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCredentials {
    #[doc = "ID scope for connecting to the IoT Central application."]
    #[serde(rename = "idScope")]
    pub id_scope: String,
    #[doc = "The symmetric key definition."]
    #[serde(rename = "symmetricKey", default, skip_serializing_if = "Option::is_none")]
    pub symmetric_key: Option<SymmetricKey>,
    #[doc = "The X509 definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<X509>,
    #[doc = "The trusted platform module definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tpm: Option<Tpm>,
}
impl DeviceCredentials {
    pub fn new(id_scope: String) -> Self {
        Self {
            id_scope,
            symmetric_key: None,
            x509: None,
            tpm: None,
        }
    }
}
#[doc = "The device group definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGroup {
    #[doc = "Unique ID of the device group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the device group."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Query defining which devices should be in this group, [Query Language Reference](https://aka.ms/iotcquery)."]
    pub filter: String,
    #[doc = "Short summary of device group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "ETag used to prevent conflict in device group updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "List of organization IDs of the device group, only one organization support today, multiple organizations will be supported soon."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<String>,
}
impl DeviceGroup {
    pub fn new(display_name: String, filter: String) -> Self {
        Self {
            id: None,
            display_name,
            filter,
            description: None,
            etag: None,
            organizations: Vec::new(),
        }
    }
}
#[doc = "The paged results of device groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGroupCollection {
    #[doc = "The collection of device groups."]
    pub value: Vec<DeviceGroup>,
    #[doc = "URL to get the next page of device groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceGroupCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceGroupCollection {
    pub fn new(value: Vec<DeviceGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Property values associated with the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceProperties {}
impl DeviceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The device telemetry definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceTelemetry {
    #[doc = "The last known value of this device telemetry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "String-formatted date representing the time when the telemetry value was sent."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl DeviceTelemetry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The device template definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTemplate {
    #[doc = "Unique ID of the device template."]
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The JSON-LD types of this device template."]
    #[serde(rename = "@type")]
    pub type_: Vec<String>,
    #[doc = "ETag used to prevent conflict in device template updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Display name of the device template."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Detailed description of the device template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The capability model utilized by this device template."]
    #[serde(rename = "capabilityModel")]
    pub capability_model: serde_json::Value,
}
impl DeviceTemplate {
    pub fn new(type_: Vec<String>, capability_model: serde_json::Value) -> Self {
        Self {
            id: None,
            type_,
            etag: None,
            display_name: None,
            description: None,
            capability_model,
        }
    }
}
#[doc = "The paged results of device templates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTemplateCollection {
    #[doc = "The collection of device templates."]
    pub value: Vec<DeviceTemplate>,
    #[doc = "URL to get the next page of device templates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceTemplateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceTemplateCollection {
    pub fn new(value: Vec<DeviceTemplate>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The email user definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailUser {
    #[serde(flatten)]
    pub user: User,
    #[doc = "Email address of the user."]
    pub email: String,
}
impl EmailUser {
    pub fn new(user: User, email: String) -> Self {
        Self { user, email }
    }
}
#[doc = "The response error definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "The detail information of the error."]
    pub error: ErrorDetails,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new(error: ErrorDetails) -> Self {
        Self { error }
    }
}
#[doc = "The detail information of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    pub code: String,
    #[doc = "Error message details."]
    pub message: String,
    #[doc = "Correlation Id for current request."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The time that error request failed."]
    #[serde(default, with = "azure_core::date::rfc1123::option")]
    pub time: Option<time::OffsetDateTime>,
}
impl ErrorDetails {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            request_id: None,
            time: None,
        }
    }
}
#[doc = "The file upload configuration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileUpload {
    #[doc = "The storage account name where to upload the file to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[doc = "The connection string used to configure the storage account"]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    #[doc = "The name of the container inside the storage account"]
    pub container: String,
    #[doc = "ISO 8601 duration standard, The amount of time the device’s request to upload a file is valid before it expires."]
    #[serde(rename = "sasTtl", default, skip_serializing_if = "Option::is_none")]
    pub sas_ttl: Option<String>,
    #[doc = "The state of the file upload configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<file_upload::State>,
    #[doc = "ETag used to prevent conflict with multiple uploads"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl FileUpload {
    pub fn new(connection_string: String, container: String) -> Self {
        Self {
            account: None,
            connection_string,
            container,
            sas_ttl: None,
            state: None,
            etag: None,
        }
    }
}
pub mod file_upload {
    use super::*;
    #[doc = "The state of the file upload configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
    }
}
#[doc = "The organization definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Organization {
    #[doc = "Unique ID of the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the organization."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "ID of the parent of the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}
impl Organization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCollection {
    #[doc = "The collection of organizations."]
    pub value: Vec<Organization>,
    #[doc = "URL to get the next page of organizations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OrganizationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OrganizationCollection {
    pub fn new(value: Vec<Organization>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The permission definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    #[doc = "List of role assignments that specify the permissions to access the application."]
    pub roles: Vec<RoleAssignment>,
}
impl Permission {
    pub fn new(roles: Vec<RoleAssignment>) -> Self {
        Self { roles }
    }
}
#[doc = "The role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Role {
    #[doc = "Unique ID of the role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the role."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Role {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role assignment definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[doc = "ID of the role for this role assignment."]
    pub role: String,
    #[doc = "ID of the organization for this role assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}
impl RoleAssignment {
    pub fn new(role: String) -> Self {
        Self { role, organization: None }
    }
}
#[doc = "The paged results of roles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleCollection {
    #[doc = "The collection of roles."]
    pub value: Vec<Role>,
    #[doc = "URL to get the next page of roles."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleCollection {
    pub fn new(value: Vec<Role>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The service principal user definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalUser {
    #[serde(flatten)]
    pub user: User,
    #[doc = "The AAD tenant ID of the service principal."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "The AAD object ID of the service principal."]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl ServicePrincipalUser {
    pub fn new(user: User, tenant_id: String, object_id: String) -> Self {
        Self {
            user,
            tenant_id,
            object_id,
        }
    }
}
#[doc = "The symmetric key definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymmetricKey {
    #[doc = "The primary key for this credential."]
    #[serde(rename = "primaryKey")]
    pub primary_key: String,
    #[doc = "The secondary key for this credential."]
    #[serde(rename = "secondaryKey")]
    pub secondary_key: String,
}
impl SymmetricKey {
    pub fn new(primary_key: String, secondary_key: String) -> Self {
        Self {
            primary_key,
            secondary_key,
        }
    }
}
#[doc = "The symmetric key attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymmetricKeyAttestation {
    #[serde(flatten)]
    pub attestation: Attestation,
    #[doc = "The symmetric key definition."]
    #[serde(rename = "symmetricKey")]
    pub symmetric_key: SymmetricKey,
}
impl SymmetricKeyAttestation {
    pub fn new(attestation: Attestation, symmetric_key: SymmetricKey) -> Self {
        Self {
            attestation,
            symmetric_key,
        }
    }
}
#[doc = "The trusted platform module definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tpm {
    #[doc = "The TPM endorsement key for this credential."]
    #[serde(rename = "endorsementKey")]
    pub endorsement_key: String,
}
impl Tpm {
    pub fn new(endorsement_key: String) -> Self {
        Self { endorsement_key }
    }
}
#[doc = "The attestation of TPM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TpmAttestation {
    #[serde(flatten)]
    pub attestation: Attestation,
    #[doc = "The trusted platform module definition."]
    pub tpm: Tpm,
}
impl TpmAttestation {
    pub fn new(attestation: Attestation, tpm: Tpm) -> Self {
        Self { attestation, tpm }
    }
}
#[doc = "The user definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub permission: Permission,
    #[doc = "Unique ID of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the user."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl User {
    pub fn new(permission: Permission, type_: String) -> Self {
        Self {
            permission,
            id: None,
            type_,
        }
    }
}
#[doc = "The paged results of users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCollection {
    #[doc = "The collection of users."]
    pub value: Vec<User>,
    #[doc = "URL to get the next page of users."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UserCollection {
    pub fn new(value: Vec<User>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The X509 definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509 {
    #[doc = "The X509 certificates definition."]
    #[serde(rename = "clientCertificates", default, skip_serializing_if = "Option::is_none")]
    pub client_certificates: Option<X509Certificates>,
}
impl X509 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509Attestation {
    #[serde(flatten)]
    pub attestation: Attestation,
    #[doc = "The X509 definition."]
    pub x509: X509,
}
impl X509Attestation {
    pub fn new(attestation: Attestation, x509: X509) -> Self {
        Self { attestation, x509 }
    }
}
#[doc = "The X509 certificate definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509Certificate {
    #[doc = "The string representation of this certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "The X509 certificate info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<X509CertificateInfo>,
}
impl X509Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 certificate info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509CertificateInfo {
    #[doc = "The SHA-1 hash value of the certificate."]
    #[serde(rename = "sha1Thumbprint")]
    pub sha1_thumbprint: String,
}
impl X509CertificateInfo {
    pub fn new(sha1_thumbprint: String) -> Self {
        Self { sha1_thumbprint }
    }
}
#[doc = "The X509 certificates definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509Certificates {
    #[doc = "The X509 certificate definition."]
    pub primary: X509Certificate,
    #[doc = "The X509 certificate definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<X509Certificate>,
}
impl X509Certificates {
    pub fn new(primary: X509Certificate) -> Self {
        Self { primary, secondary: None }
    }
}
