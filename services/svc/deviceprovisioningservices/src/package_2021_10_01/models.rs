#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Attestation mechanism for individualEnrollment as well as enrollmentGroup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationMechanism {
    #[doc = "Attestation Type."]
    #[serde(rename = "type")]
    pub type_: attestation_mechanism::Type,
    #[doc = "Attestation via TPM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tpm: Option<TpmAttestation>,
    #[doc = "Attestation via X509."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<X509Attestation>,
    #[doc = "Attestation via SymmetricKey."]
    #[serde(rename = "symmetricKey", default, skip_serializing_if = "Option::is_none")]
    pub symmetric_key: Option<SymmetricKeyAttestation>,
}
impl AttestationMechanism {
    pub fn new(type_: attestation_mechanism::Type) -> Self {
        Self {
            type_,
            tpm: None,
            x509: None,
            symmetric_key: None,
        }
    }
}
pub mod attestation_mechanism {
    use super::*;
    #[doc = "Attestation Type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "tpm")]
        Tpm,
        #[serde(rename = "x509")]
        X509,
        #[serde(rename = "symmetricKey")]
        SymmetricKey,
    }
}
#[doc = "Bulk enrollment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEnrollmentGroupOperation {
    #[doc = "Enrollment items"]
    #[serde(rename = "enrollmentGroups")]
    pub enrollment_groups: Vec<EnrollmentGroup>,
    #[doc = "Operation mode."]
    pub mode: bulk_enrollment_group_operation::Mode,
}
impl BulkEnrollmentGroupOperation {
    pub fn new(enrollment_groups: Vec<EnrollmentGroup>, mode: bulk_enrollment_group_operation::Mode) -> Self {
        Self { enrollment_groups, mode }
    }
}
pub mod bulk_enrollment_group_operation {
    use super::*;
    #[doc = "Operation mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        #[serde(rename = "create")]
        Create,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "updateIfMatchETag")]
        UpdateIfMatchETag,
        #[serde(rename = "delete")]
        Delete,
    }
}
#[doc = "Bulk enrollment operation error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEnrollmentGroupOperationError {
    #[doc = "Enrollment group id."]
    #[serde(rename = "enrollmentGroupId")]
    pub enrollment_group_id: String,
    #[doc = "Error code"]
    #[serde(rename = "errorCode")]
    pub error_code: i32,
    #[doc = "Error status."]
    #[serde(rename = "errorStatus")]
    pub error_status: String,
}
impl BulkEnrollmentGroupOperationError {
    pub fn new(enrollment_group_id: String, error_code: i32, error_status: String) -> Self {
        Self {
            enrollment_group_id,
            error_code,
            error_status,
        }
    }
}
#[doc = "Results of a bulk enrollment group operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEnrollmentGroupOperationResult {
    #[doc = "Registration errors"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<BulkEnrollmentGroupOperationError>,
    #[doc = "Indicates if the operation was successful in its entirety."]
    #[serde(rename = "isSuccessful")]
    pub is_successful: bool,
}
impl BulkEnrollmentGroupOperationResult {
    pub fn new(is_successful: bool) -> Self {
        Self {
            errors: Vec::new(),
            is_successful,
        }
    }
}
#[doc = "Bulk enrollment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEnrollmentOperation {
    #[doc = "Enrollment items"]
    pub enrollments: Vec<IndividualEnrollment>,
    #[doc = "Operation mode."]
    pub mode: bulk_enrollment_operation::Mode,
}
impl BulkEnrollmentOperation {
    pub fn new(enrollments: Vec<IndividualEnrollment>, mode: bulk_enrollment_operation::Mode) -> Self {
        Self { enrollments, mode }
    }
}
pub mod bulk_enrollment_operation {
    use super::*;
    #[doc = "Operation mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        #[serde(rename = "create")]
        Create,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "updateIfMatchETag")]
        UpdateIfMatchETag,
        #[serde(rename = "delete")]
        Delete,
    }
}
#[doc = "Bulk enrollment operation error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEnrollmentOperationError {
    #[doc = "This id is used to uniquely identify a device registration of an enrollment.\r\nA case-insensitive string (up to 128 characters long) of alphanumeric characters plus certain special characters : . _ -. No special characters allowed at start or end."]
    #[serde(rename = "registrationId")]
    pub registration_id: String,
    #[doc = "Error code"]
    #[serde(rename = "errorCode")]
    pub error_code: i32,
    #[doc = "Error status."]
    #[serde(rename = "errorStatus")]
    pub error_status: String,
}
impl BulkEnrollmentOperationError {
    pub fn new(registration_id: String, error_code: i32, error_status: String) -> Self {
        Self {
            registration_id,
            error_code,
            error_status,
        }
    }
}
#[doc = "Results of a bulk enrollment operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEnrollmentOperationResult {
    #[doc = "Registration errors"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<BulkEnrollmentOperationError>,
    #[doc = "Indicates if the operation was successful in its entirety."]
    #[serde(rename = "isSuccessful")]
    pub is_successful: bool,
}
impl BulkEnrollmentOperationResult {
    pub fn new(is_successful: bool) -> Self {
        Self {
            errors: Vec::new(),
            is_successful,
        }
    }
}
#[doc = "This tells DPS which webhook to call when using custom allocation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomAllocationDefinition {
    #[doc = "The webhook URL used for allocation requests."]
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
    #[doc = "The API version of the provisioning service types (such as IndividualEnrollment) sent in the custom allocation request. Minimum supported version: \"2018-09-01-preview\"."]
    #[serde(rename = "apiVersion")]
    pub api_version: String,
}
impl CustomAllocationDefinition {
    pub fn new(webhook_url: String, api_version: String) -> Self {
        Self { webhook_url, api_version }
    }
}
#[doc = "Device capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    #[doc = "If set to true, this device is an IoTEdge device."]
    #[serde(rename = "iotEdge")]
    pub iot_edge: bool,
}
impl DeviceCapabilities {
    pub fn new(iot_edge: bool) -> Self {
        Self { iot_edge }
    }
}
#[doc = "Device registration state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceRegistrationState {
    #[doc = "This id is used to uniquely identify a device registration of an enrollment.\r\nA case-insensitive string (up to 128 characters long) of alphanumeric characters plus certain special characters : . _ -. No special characters allowed at start or end."]
    #[serde(rename = "registrationId", default, skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    #[doc = "Registration create date time (in UTC)."]
    #[serde(rename = "createdDateTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub created_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Assigned Azure IoT Hub."]
    #[serde(rename = "assignedHub", default, skip_serializing_if = "Option::is_none")]
    pub assigned_hub: Option<String>,
    #[doc = "Device ID."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Enrollment status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<device_registration_state::Status>,
    #[doc = "Substatus for 'Assigned' devices. Possible values include - 'initialAssignment': Device has been assigned to an IoT hub for the first time, 'deviceDataMigrated': Device has been assigned to a different IoT hub and its device data was migrated from the previously assigned IoT hub. Device data was removed from the previously assigned IoT hub, 'deviceDataReset':  Device has been assigned to a different IoT hub and its device data was populated from the initial state stored in the enrollment. Device data was removed from the previously assigned IoT hub, 'reprovisionedToInitialAssignment': Device has been re-provisioned to a previously assigned IoT hub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substatus: Option<device_registration_state::Substatus>,
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Last updated date time (in UTC)."]
    #[serde(rename = "lastUpdatedDateTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The entity tag associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Custom allocation payload returned from the webhook to the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}
impl DeviceRegistrationState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod device_registration_state {
    use super::*;
    #[doc = "Enrollment status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unassigned")]
        Unassigned,
        #[serde(rename = "assigning")]
        Assigning,
        #[serde(rename = "assigned")]
        Assigned,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "disabled")]
        Disabled,
    }
    #[doc = "Substatus for 'Assigned' devices. Possible values include - 'initialAssignment': Device has been assigned to an IoT hub for the first time, 'deviceDataMigrated': Device has been assigned to a different IoT hub and its device data was migrated from the previously assigned IoT hub. Device data was removed from the previously assigned IoT hub, 'deviceDataReset':  Device has been assigned to a different IoT hub and its device data was populated from the initial state stored in the enrollment. Device data was removed from the previously assigned IoT hub, 'reprovisionedToInitialAssignment': Device has been re-provisioned to a previously assigned IoT hub."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Substatus {
        #[serde(rename = "initialAssignment")]
        InitialAssignment,
        #[serde(rename = "deviceDataMigrated")]
        DeviceDataMigrated,
        #[serde(rename = "deviceDataReset")]
        DeviceDataReset,
        #[serde(rename = "reprovisionedToInitialAssignment")]
        ReprovisionedToInitialAssignment,
    }
}
#[doc = "Enrollment group record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentGroup {
    #[doc = "Enrollment Group ID."]
    #[serde(rename = "enrollmentGroupId")]
    pub enrollment_group_id: String,
    #[doc = "Attestation mechanism for individualEnrollment as well as enrollmentGroup."]
    pub attestation: AttestationMechanism,
    #[doc = "Device capabilities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<DeviceCapabilities>,
    #[doc = "The Iot Hub host name."]
    #[serde(rename = "iotHubHostName", default, skip_serializing_if = "Option::is_none")]
    pub iot_hub_host_name: Option<String>,
    #[doc = "Initial device twin. Contains a subset of the properties of Twin."]
    #[serde(rename = "initialTwin", default, skip_serializing_if = "Option::is_none")]
    pub initial_twin: Option<InitialTwin>,
    #[doc = "The entity tag associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The provisioning status."]
    #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<enrollment_group::ProvisioningStatus>,
    #[doc = "The behavior of the service when a device is re-provisioned to an IoT hub."]
    #[serde(rename = "reprovisionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub reprovision_policy: Option<ReprovisionPolicy>,
    #[doc = "The DateTime this resource was created."]
    #[serde(rename = "createdDateTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub created_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The DateTime this resource was last updated."]
    #[serde(rename = "lastUpdatedDateTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The allocation policy of this resource. This policy overrides the tenant level allocation policy for this individual enrollment or enrollment group. Possible values include 'hashed': Linked IoT hubs are equally likely to have devices provisioned to them, 'geoLatency':  Devices are provisioned to an IoT hub with the lowest latency to the device.If multiple linked IoT hubs would provide the same lowest latency, the provisioning service hashes devices across those hubs, 'static' : Specification of the desired IoT hub in the enrollment list takes priority over the service-level allocation policy, 'custom': Devices are provisioned to an IoT hub based on your own custom logic. The provisioning service passes information about the device to the logic, and the logic returns the desired IoT hub as well as the desired initial configuration. We recommend using Azure Functions to host your logic."]
    #[serde(rename = "allocationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub allocation_policy: Option<enrollment_group::AllocationPolicy>,
    #[doc = "The list of IoT Hub hostnames the device(s) in this resource can be allocated to. Must be a subset of tenant level list of IoT hubs."]
    #[serde(rename = "iotHubs", default, skip_serializing_if = "Vec::is_empty")]
    pub iot_hubs: Vec<String>,
    #[doc = "This tells DPS which webhook to call when using custom allocation."]
    #[serde(rename = "customAllocationDefinition", default, skip_serializing_if = "Option::is_none")]
    pub custom_allocation_definition: Option<CustomAllocationDefinition>,
}
impl EnrollmentGroup {
    pub fn new(enrollment_group_id: String, attestation: AttestationMechanism) -> Self {
        Self {
            enrollment_group_id,
            attestation,
            capabilities: None,
            iot_hub_host_name: None,
            initial_twin: None,
            etag: None,
            provisioning_status: None,
            reprovision_policy: None,
            created_date_time_utc: None,
            last_updated_date_time_utc: None,
            allocation_policy: None,
            iot_hubs: Vec::new(),
            custom_allocation_definition: None,
        }
    }
}
pub mod enrollment_group {
    use super::*;
    #[doc = "The provisioning status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningStatus {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
    impl Default for ProvisioningStatus {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "The allocation policy of this resource. This policy overrides the tenant level allocation policy for this individual enrollment or enrollment group. Possible values include 'hashed': Linked IoT hubs are equally likely to have devices provisioned to them, 'geoLatency':  Devices are provisioned to an IoT hub with the lowest latency to the device.If multiple linked IoT hubs would provide the same lowest latency, the provisioning service hashes devices across those hubs, 'static' : Specification of the desired IoT hub in the enrollment list takes priority over the service-level allocation policy, 'custom': Devices are provisioned to an IoT hub based on your own custom logic. The provisioning service passes information about the device to the logic, and the logic returns the desired IoT hub as well as the desired initial configuration. We recommend using Azure Functions to host your logic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AllocationPolicy {
        #[serde(rename = "hashed")]
        Hashed,
        #[serde(rename = "geoLatency")]
        GeoLatency,
        #[serde(rename = "static")]
        Static,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[doc = "The device enrollment record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndividualEnrollment {
    #[doc = "This id is used to uniquely identify a device registration of an enrollment.\r\nA case-insensitive string (up to 128 characters long) of alphanumeric characters plus certain special characters : . _ -. No special characters allowed at start or end."]
    #[serde(rename = "registrationId")]
    pub registration_id: String,
    #[doc = "Desired IoT Hub device ID (optional)."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Device registration state."]
    #[serde(rename = "registrationState", default, skip_serializing_if = "Option::is_none")]
    pub registration_state: Option<DeviceRegistrationState>,
    #[doc = "Represents a collection of properties within a Twin"]
    #[serde(rename = "optionalDeviceInformation", default, skip_serializing_if = "Option::is_none")]
    pub optional_device_information: Option<TwinCollection>,
    #[doc = "Attestation mechanism for individualEnrollment as well as enrollmentGroup."]
    pub attestation: AttestationMechanism,
    #[doc = "Device capabilities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<DeviceCapabilities>,
    #[doc = "The Iot Hub host name."]
    #[serde(rename = "iotHubHostName", default, skip_serializing_if = "Option::is_none")]
    pub iot_hub_host_name: Option<String>,
    #[doc = "Initial device twin. Contains a subset of the properties of Twin."]
    #[serde(rename = "initialTwin", default, skip_serializing_if = "Option::is_none")]
    pub initial_twin: Option<InitialTwin>,
    #[doc = "The entity tag associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The provisioning status."]
    #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<individual_enrollment::ProvisioningStatus>,
    #[doc = "The behavior of the service when a device is re-provisioned to an IoT hub."]
    #[serde(rename = "reprovisionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub reprovision_policy: Option<ReprovisionPolicy>,
    #[doc = "The DateTime this resource was created."]
    #[serde(rename = "createdDateTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub created_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The DateTime this resource was last updated."]
    #[serde(rename = "lastUpdatedDateTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The allocation policy of this resource. This policy overrides the tenant level allocation policy for this individual enrollment or enrollment group. Possible values include 'hashed': Linked IoT hubs are equally likely to have devices provisioned to them, 'geoLatency':  Devices are provisioned to an IoT hub with the lowest latency to the device.If multiple linked IoT hubs would provide the same lowest latency, the provisioning service hashes devices across those hubs, 'static' : Specification of the desired IoT hub in the enrollment list takes priority over the service-level allocation policy, 'custom': Devices are provisioned to an IoT hub based on your own custom logic. The provisioning service passes information about the device to the logic, and the logic returns the desired IoT hub as well as the desired initial configuration. We recommend using Azure Functions to host your logic."]
    #[serde(rename = "allocationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub allocation_policy: Option<individual_enrollment::AllocationPolicy>,
    #[doc = "The list of IoT Hub hostnames the device(s) in this resource can be allocated to. Must be a subset of tenant level list of IoT hubs."]
    #[serde(rename = "iotHubs", default, skip_serializing_if = "Vec::is_empty")]
    pub iot_hubs: Vec<String>,
    #[doc = "This tells DPS which webhook to call when using custom allocation."]
    #[serde(rename = "customAllocationDefinition", default, skip_serializing_if = "Option::is_none")]
    pub custom_allocation_definition: Option<CustomAllocationDefinition>,
}
impl IndividualEnrollment {
    pub fn new(registration_id: String, attestation: AttestationMechanism) -> Self {
        Self {
            registration_id,
            device_id: None,
            registration_state: None,
            optional_device_information: None,
            attestation,
            capabilities: None,
            iot_hub_host_name: None,
            initial_twin: None,
            etag: None,
            provisioning_status: None,
            reprovision_policy: None,
            created_date_time_utc: None,
            last_updated_date_time_utc: None,
            allocation_policy: None,
            iot_hubs: Vec::new(),
            custom_allocation_definition: None,
        }
    }
}
pub mod individual_enrollment {
    use super::*;
    #[doc = "The provisioning status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningStatus {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
    impl Default for ProvisioningStatus {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "The allocation policy of this resource. This policy overrides the tenant level allocation policy for this individual enrollment or enrollment group. Possible values include 'hashed': Linked IoT hubs are equally likely to have devices provisioned to them, 'geoLatency':  Devices are provisioned to an IoT hub with the lowest latency to the device.If multiple linked IoT hubs would provide the same lowest latency, the provisioning service hashes devices across those hubs, 'static' : Specification of the desired IoT hub in the enrollment list takes priority over the service-level allocation policy, 'custom': Devices are provisioned to an IoT hub based on your own custom logic. The provisioning service passes information about the device to the logic, and the logic returns the desired IoT hub as well as the desired initial configuration. We recommend using Azure Functions to host your logic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AllocationPolicy {
        #[serde(rename = "hashed")]
        Hashed,
        #[serde(rename = "geoLatency")]
        GeoLatency,
        #[serde(rename = "static")]
        Static,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[doc = "Initial device twin. Contains a subset of the properties of Twin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitialTwin {
    #[doc = "Represents a collection of properties within a Twin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TwinCollection>,
    #[doc = "Represents the initial properties that will be set on the device twin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InitialTwinProperties>,
}
impl InitialTwin {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the initial properties that will be set on the device twin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitialTwinProperties {
    #[doc = "Represents a collection of properties within a Twin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<TwinCollection>,
}
impl InitialTwinProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for the TwinCollection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metadata {
    #[doc = "Last time the TwinCollection was updated"]
    #[serde(rename = "lastUpdated", with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "This is null for reported properties metadata and is not null for desired properties metadata."]
    #[serde(rename = "lastUpdatedVersion", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_version: Option<i64>,
}
impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the properties of an error returned by the Azure IoT Hub Provisioning Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisioningServiceErrorDetails {
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
    #[serde(rename = "timestampUtc", with = "azure_core::date::rfc3339::option")]
    pub timestamp_utc: Option<time::OffsetDateTime>,
}
impl ProvisioningServiceErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuerySpecification {
    pub query: String,
}
impl QuerySpecification {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}
#[doc = "The behavior of the service when a device is re-provisioned to an IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReprovisionPolicy {
    #[doc = "When set to true (default), the Device Provisioning Service will evaluate the device's IoT Hub assignment and update it if necessary for any provisioning requests beyond the first from a given device. If set to false, the device will stay assigned to its current IoT hub."]
    #[serde(rename = "updateHubAssignment")]
    pub update_hub_assignment: bool,
    #[doc = "When set to true (default), the Device Provisioning Service will migrate the device's data (twin, device capabilities, and device ID) from one IoT hub to another during an IoT hub assignment update. If set to false, the Device Provisioning Service will reset the device's data to the initial desired configuration stored in the corresponding enrollment list."]
    #[serde(rename = "migrateDeviceData")]
    pub migrate_device_data: bool,
}
impl ReprovisionPolicy {
    pub fn new(update_hub_assignment: bool, migrate_device_data: bool) -> Self {
        Self {
            update_hub_assignment,
            migrate_device_data,
        }
    }
}
#[doc = "Attestation via SymmetricKey."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SymmetricKeyAttestation {
    #[doc = "Primary symmetric key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Secondary symmetric key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl SymmetricKeyAttestation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Attestation via TPM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TpmAttestation {
    #[serde(rename = "endorsementKey")]
    pub endorsement_key: String,
    #[serde(rename = "storageRootKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_root_key: Option<String>,
}
impl TpmAttestation {
    pub fn new(endorsement_key: String) -> Self {
        Self {
            endorsement_key,
            storage_root_key: None,
        }
    }
}
#[doc = "Represents a collection of properties within a Twin"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TwinCollection {
    #[doc = "Version of the TwinCollection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "Number of properties in the TwinCollection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Metadata for the TwinCollection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
impl TwinCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Attestation via X509."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509Attestation {
    #[doc = "Primary and secondary certificates"]
    #[serde(rename = "clientCertificates", default, skip_serializing_if = "Option::is_none")]
    pub client_certificates: Option<X509Certificates>,
    #[doc = "Primary and secondary certificates"]
    #[serde(rename = "signingCertificates", default, skip_serializing_if = "Option::is_none")]
    pub signing_certificates: Option<X509Certificates>,
    #[doc = "Primary and secondary CA references."]
    #[serde(rename = "caReferences", default, skip_serializing_if = "Option::is_none")]
    pub ca_references: Option<X509caReferences>,
}
impl X509Attestation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Primary and secondary CA references."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509caReferences {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}
impl X509caReferences {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "X509 certificate info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509CertificateInfo {
    #[serde(rename = "subjectName")]
    pub subject_name: String,
    #[serde(rename = "sha1Thumbprint")]
    pub sha1_thumbprint: String,
    #[serde(rename = "sha256Thumbprint")]
    pub sha256_thumbprint: String,
    #[serde(rename = "issuerName")]
    pub issuer_name: String,
    #[serde(rename = "notBeforeUtc", with = "azure_core::date::rfc3339")]
    pub not_before_utc: time::OffsetDateTime,
    #[serde(rename = "notAfterUtc", with = "azure_core::date::rfc3339")]
    pub not_after_utc: time::OffsetDateTime,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    pub version: i32,
}
impl X509CertificateInfo {
    pub fn new(
        subject_name: String,
        sha1_thumbprint: String,
        sha256_thumbprint: String,
        issuer_name: String,
        not_before_utc: time::OffsetDateTime,
        not_after_utc: time::OffsetDateTime,
        serial_number: String,
        version: i32,
    ) -> Self {
        Self {
            subject_name,
            sha1_thumbprint,
            sha256_thumbprint,
            issuer_name,
            not_before_utc,
            not_after_utc,
            serial_number,
            version,
        }
    }
}
#[doc = "Certificate and Certificate info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509CertificateWithInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "X509 certificate info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<X509CertificateInfo>,
}
impl X509CertificateWithInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Primary and secondary certificates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509Certificates {
    #[doc = "Certificate and Certificate info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<X509CertificateWithInfo>,
    #[doc = "Certificate and Certificate info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<X509CertificateWithInfo>,
}
impl X509Certificates {
    pub fn new() -> Self {
        Self::default()
    }
}
