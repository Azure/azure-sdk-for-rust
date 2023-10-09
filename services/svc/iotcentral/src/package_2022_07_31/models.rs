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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiTokenCollection {
    pub fn new(value: Vec<ApiToken>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Type of the attestation."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AttestationUnion {
    #[serde(rename = "symmetricKey")]
    SymmetricKey(SymmetricKeyAttestation),
    #[serde(rename = "tpm")]
    Tpm(TpmAttestation),
    #[serde(rename = "x509")]
    X509(X509Attestation),
}
#[doc = "The capability job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityJobData {
    #[doc = "The device template which defines the target capability for the job."]
    pub target: String,
    #[doc = "The path to the target capability within the device template."]
    pub path: String,
    #[doc = "Can be anything: string, number, array, object, etc. (except `null`)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<AnyValue>,
}
impl CapabilityJobData {
    pub fn new(target: String, path: String) -> Self {
        Self { target, path, value: None }
    }
}
#[doc = "The cloud property job data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudPropertyJobData {
    #[serde(flatten)]
    pub capability_job_data: CapabilityJobData,
}
impl CloudPropertyJobData {
    pub fn new(capability_job_data: CapabilityJobData) -> Self {
        Self { capability_job_data }
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
#[doc = "The command job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandJobData {
    #[serde(flatten)]
    pub capability_job_data: CapabilityJobData,
}
impl CommandJobData {
    pub fn new(capability_job_data: CapabilityJobData) -> Self {
        Self { capability_job_data }
    }
}
#[doc = "The date based end definition of job schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateJobScheduleEnd {
    #[doc = "The date when to end the scheduled job."]
    pub date: String,
}
impl DateJobScheduleEnd {
    pub fn new(date: String) -> Self {
        Self { date }
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
    #[doc = "List of organization IDs that the device is a part of, only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
        self.next_link.clone().filter(|value| !value.is_empty())
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
    pub connection_timeout: Option<i32>,
    #[doc = "Response timeout in seconds to wait for a command completion on a device. Defaults to 30 seconds."]
    #[serde(rename = "responseTimeout", default, skip_serializing_if = "Option::is_none")]
    pub response_timeout: Option<i32>,
    #[doc = "The payload for the device command, support any primitive types or object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
    #[doc = "The payload of the device command response, support any primitive types or object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,
    #[doc = "The status code of the device command response."]
    #[serde(rename = "responseCode", default, skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,
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
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[doc = "List of organization IDs of the device group, only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceGroupCollection {
    pub fn new(value: Vec<DeviceGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The paged results of devices belonging to the device group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGroupDeviceCollection {
    #[doc = "The collection of devices belonging to the device group."]
    pub value: Vec<Device>,
    #[doc = "URL to get the next page of devices in the group."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceGroupDeviceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceGroupDeviceCollection {
    pub fn new(value: Vec<Device>) -> Self {
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
#[doc = "An object representing the relationship between an upstream and a downstream device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceRelationship {
    #[doc = "The unique identifier of this relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The device ID of the source (parent) device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The device ID of the target (child) device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl DeviceRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of device relationships."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceRelationshipCollection {
    #[doc = "The collection of device relationships."]
    pub value: Vec<DeviceRelationship>,
    #[doc = "URL to get the next page of device relationships."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceRelationshipCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceRelationshipCollection {
    pub fn new(value: Vec<DeviceRelationship>) -> Self {
        Self { value, next_link: None }
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceTemplateCollection {
    pub fn new(value: Vec<DeviceTemplate>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The device template migration job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTemplateMigrationJobData {
    #[doc = "The target device template to which devices will be migrated."]
    pub template: String,
}
impl DeviceTemplateMigrationJobData {
    pub fn new(template: String) -> Self {
        Self { template }
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
#[doc = "The enrollment group definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentGroup {
    #[doc = "Unique ID of the enrollment group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the enrollment group."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Whether the devices using the group are allowed to connect to IoT Central."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Type of devices that connect through the group."]
    #[serde(rename = "type")]
    pub type_: enrollment_group::Type,
    #[doc = "The attestation definition for an enrollment group."]
    pub attestation: GroupAttestationUnion,
    #[doc = "ETag used to prevent conflict in enrollment group updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl EnrollmentGroup {
    pub fn new(display_name: String, type_: enrollment_group::Type, attestation: GroupAttestationUnion) -> Self {
        Self {
            id: None,
            display_name,
            enabled: None,
            type_,
            attestation,
            etag: None,
        }
    }
}
pub mod enrollment_group {
    use super::*;
    #[doc = "Type of devices that connect through the group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "iot")]
        Iot,
        #[serde(rename = "iotEdge")]
        IotEdge,
    }
}
#[doc = "The paged results of enrollment groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentGroupCollection {
    #[doc = "The collection of enrollment groups."]
    pub value: Vec<EnrollmentGroup>,
    #[doc = "URL to get the next page of enrollment groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EnrollmentGroupCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnrollmentGroupCollection {
    pub fn new(value: Vec<EnrollmentGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The response error definition."]
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
#[doc = "Type of the attestation."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GroupAttestationUnion {
    #[serde(rename = "symmetricKey")]
    SymmetricKey(GroupSymmetricKeyAttestation),
    #[serde(rename = "x509")]
    X509(GroupX509Attestation),
}
#[doc = "The symmetric key attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupSymmetricKeyAttestation {
    #[doc = "The symmetric key definition."]
    #[serde(rename = "symmetricKey", default, skip_serializing_if = "Option::is_none")]
    pub symmetric_key: Option<SymmetricKey>,
}
impl GroupSymmetricKeyAttestation {
    pub fn new() -> Self {
        Self { symmetric_key: None }
    }
}
#[doc = "The X509 attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupX509Attestation {
    #[doc = "The X509 definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<SigningX509>,
}
impl GroupX509Attestation {
    pub fn new() -> Self {
        Self { x509: None }
    }
}
#[doc = "The job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[doc = "Unique ID of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Id of the scheduled job definition that created this job."]
    #[serde(rename = "scheduledJobId", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_job_id: Option<String>,
    #[doc = "Display name of the job."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Detailed description of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ID of the device group on which to execute the job."]
    pub group: String,
    #[doc = "The job batch definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch: Option<JobBatch>,
    #[doc = "The job cancellation threshold definition."]
    #[serde(rename = "cancellationThreshold", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_threshold: Option<JobCancellationThreshold>,
    #[doc = "The capabilities being updated by the job and the values with which they are being updated."]
    pub data: Vec<JobDataUnion>,
    #[doc = "The start time of the job"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub start: Option<time::OffsetDateTime>,
    #[doc = "The end time of the job"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub end: Option<time::OffsetDateTime>,
    #[doc = "progress summary for a scheduled job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<JobProgress>,
    #[doc = "Indicates whether the job is starting, running, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "List of organizations of the job, only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub organizations: Vec<String>,
}
impl Job {
    pub fn new(group: String, data: Vec<JobDataUnion>) -> Self {
        Self {
            id: None,
            scheduled_job_id: None,
            display_name: None,
            description: None,
            group,
            batch: None,
            cancellation_threshold: None,
            data,
            start: None,
            end: None,
            progress: None,
            status: None,
            organizations: Vec::new(),
        }
    }
}
#[doc = "The job batch definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobBatch {
    #[doc = "Whether batching is done on a specified number of devices or a percentage of the total devices."]
    #[serde(rename = "type")]
    pub type_: job_batch::Type,
    #[doc = "The number or percentage of devices on which batching is done."]
    pub value: f64,
}
impl JobBatch {
    pub fn new(type_: job_batch::Type, value: f64) -> Self {
        Self { type_, value }
    }
}
pub mod job_batch {
    use super::*;
    #[doc = "Whether batching is done on a specified number of devices or a percentage of the total devices."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "number")]
        Number,
        #[serde(rename = "percentage")]
        Percentage,
    }
}
#[doc = "The job cancellation threshold definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCancellationThreshold {
    #[doc = "Whether the cancellation threshold is per a specified number of devices or a percentage of the total devices."]
    #[serde(rename = "type")]
    pub type_: job_cancellation_threshold::Type,
    #[doc = "The number or percentage of devices on which the cancellation threshold is applied."]
    pub value: f64,
    #[doc = "Whether the cancellation threshold applies per-batch or to the overall job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch: Option<bool>,
}
impl JobCancellationThreshold {
    pub fn new(type_: job_cancellation_threshold::Type, value: f64) -> Self {
        Self { type_, value, batch: None }
    }
}
pub mod job_cancellation_threshold {
    use super::*;
    #[doc = "Whether the cancellation threshold is per a specified number of devices or a percentage of the total devices."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "number")]
        Number,
        #[serde(rename = "percentage")]
        Percentage,
    }
}
#[doc = "The paged results of jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCollection {
    #[doc = "The collection of jobs."]
    pub value: Vec<Job>,
    #[doc = "URL to get the next page of jobs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobCollection {
    pub fn new(value: Vec<Job>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Type of the job data."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JobDataUnion {
    #[serde(rename = "cloudProperty")]
    CloudProperty(CloudPropertyJobData),
    #[serde(rename = "command")]
    Command(CommandJobData),
    #[serde(rename = "deviceTemplateMigration")]
    DeviceTemplateMigration(DeviceTemplateMigrationJobData),
    #[serde(rename = "property")]
    Property(PropertyJobData),
}
#[doc = "The job device status definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDeviceStatus {
    #[doc = "ID of the device whose job status is being provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates whether the job is starting, running, etc. for the given device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl JobDeviceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of job device statuses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDeviceStatusCollection {
    #[doc = "The collection of job device statuses."]
    pub value: Vec<JobDeviceStatus>,
    #[doc = "URL to get the next page of job device statuses."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobDeviceStatusCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobDeviceStatusCollection {
    pub fn new(value: Vec<JobDeviceStatus>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "progress summary for a scheduled job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProgress {
    #[doc = "The total number of entities targeted by the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[doc = "The number of entities for which the job is not yet running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<i32>,
    #[doc = "The number of entities for which the job has completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<i32>,
    #[doc = "The number of entities for which the job has failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
}
impl JobProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The schedule definition of job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobSchedule {
    #[doc = "The recurrence of the scheduled job. If not provided, the job will run once at the specified start time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<job_schedule::Recurrence>,
    #[doc = "The start time for the scheduled job"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub start: time::OffsetDateTime,
    #[doc = "The end definition of job schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<JobScheduleEndUnion>,
}
impl JobSchedule {
    pub fn new(start: time::OffsetDateTime) -> Self {
        Self {
            recurrence: None,
            start,
            end: None,
        }
    }
}
pub mod job_schedule {
    use super::*;
    #[doc = "The recurrence of the scheduled job. If not provided, the job will run once at the specified start time."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Recurrence {
        #[serde(rename = "daily")]
        Daily,
        #[serde(rename = "weekly")]
        Weekly,
        #[serde(rename = "monthly")]
        Monthly,
    }
}
#[doc = "Type of the job schedule end."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JobScheduleEndUnion {
    #[serde(rename = "date")]
    Date(DateJobScheduleEnd),
    #[serde(rename = "occurrences")]
    Occurrences(OccurrencesJobScheduleEnd),
}
#[doc = "The occurences based end definition of job schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OccurrencesJobScheduleEnd {
    #[doc = "The number of occurrences after which to end the scheduled job."]
    pub occurrences: i32,
}
impl OccurrencesJobScheduleEnd {
    pub fn new(occurrences: i32) -> Self {
        Self { occurrences }
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
#[doc = "The paged results of organizations."]
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
        self.next_link.clone().filter(|value| !value.is_empty())
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
#[doc = "The property job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyJobData {
    #[serde(flatten)]
    pub capability_job_data: CapabilityJobData,
}
impl PropertyJobData {
    pub fn new(capability_job_data: CapabilityJobData) -> Self {
        Self { capability_job_data }
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleCollection {
    pub fn new(value: Vec<Role>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The scheduled job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledJob {
    #[doc = "ETag used to prevent conflict in scheduled job updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Unique ID of the scheduled job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the scheduled job."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Detailed description of the scheduled job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ID of the device group on which to execute the scheduled job."]
    pub group: String,
    #[doc = "The job batch definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch: Option<JobBatch>,
    #[doc = "The job cancellation threshold definition."]
    #[serde(rename = "cancellationThreshold", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_threshold: Option<JobCancellationThreshold>,
    #[doc = "Data related to the operation being performed by this job. All entries must be of the same type."]
    pub data: Vec<JobDataUnion>,
    #[doc = "List of organizations of the job, only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub organizations: Vec<String>,
    #[doc = "The schedule definition of job."]
    pub schedule: JobSchedule,
    #[doc = "Whether the scheduled job is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Whether the scheduled job has completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}
impl ScheduledJob {
    pub fn new(group: String, data: Vec<JobDataUnion>, schedule: JobSchedule) -> Self {
        Self {
            etag: None,
            id: None,
            display_name: None,
            description: None,
            group,
            batch: None,
            cancellation_threshold: None,
            data,
            organizations: Vec::new(),
            schedule,
            enabled: None,
            completed: None,
        }
    }
}
#[doc = "The paged results of scheduled job definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledJobCollection {
    #[doc = "The collection of scheduled jobs."]
    pub value: Vec<ScheduledJob>,
    #[doc = "URL to get the next page of scheduled jobs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduledJobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScheduledJobCollection {
    pub fn new(value: Vec<ScheduledJob>) -> Self {
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
#[doc = "The X509 definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SigningX509 {
    #[doc = "The X509 certificates definition."]
    #[serde(rename = "signingCertificates", default, skip_serializing_if = "Option::is_none")]
    pub signing_certificates: Option<SigningX509Certificates>,
}
impl SigningX509 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 certificate definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SigningX509Certificate {
    #[doc = "Whether the certificate has been verified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[doc = "The string representation of this certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "The X509 certificate info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<X509CertificateInfo>,
    #[doc = "ETag used to prevent conflict across multiple updates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl SigningX509Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 certificates definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SigningX509Certificates {
    #[doc = "The X509 certificate definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<SigningX509Certificate>,
    #[doc = "The X509 certificate definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<SigningX509Certificate>,
}
impl SigningX509Certificates {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "The symmetric key definition."]
    #[serde(rename = "symmetricKey")]
    pub symmetric_key: SymmetricKey,
}
impl SymmetricKeyAttestation {
    pub fn new(symmetric_key: SymmetricKey) -> Self {
        Self { symmetric_key }
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
#[doc = "The TPM attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TpmAttestation {
    #[doc = "The trusted platform module definition."]
    pub tpm: Tpm,
}
impl TpmAttestation {
    pub fn new(tpm: Tpm) -> Self {
        Self { tpm }
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
}
impl User {
    pub fn new(permission: Permission) -> Self {
        Self { permission, id: None }
    }
}
#[doc = "Type of the user."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UserUnion {
    #[serde(rename = "adGroup")]
    AdGroup(AdGroupUser),
    #[serde(rename = "email")]
    Email(EmailUser),
    #[serde(rename = "servicePrincipal")]
    ServicePrincipal(ServicePrincipalUser),
}
#[doc = "The paged results of users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCollection {
    #[doc = "The collection of users."]
    pub value: Vec<UserUnion>,
    #[doc = "URL to get the next page of users."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UserCollection {
    pub fn new(value: Vec<UserUnion>) -> Self {
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
    #[doc = "The X509 definition."]
    pub x509: X509,
}
impl X509Attestation {
    pub fn new(x509: X509) -> Self {
        Self { x509 }
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
#[doc = "The X509 verification certificate definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509VerificationCertificate {
    #[doc = "The string representation of this certificate."]
    pub certificate: String,
}
impl X509VerificationCertificate {
    pub fn new(certificate: String) -> Self {
        Self { certificate }
    }
}
#[doc = "The X509 certificate verification code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509VerificationCode {
    #[doc = "The X509 certificate verification code."]
    #[serde(rename = "verificationCode", default, skip_serializing_if = "Option::is_none")]
    pub verification_code: Option<String>,
}
impl X509VerificationCode {
    pub fn new() -> Self {
        Self::default()
    }
}
