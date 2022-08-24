#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Contains information about the delivery package being shipped by the customer to the Microsoft data center."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryPackageInformation {
    #[doc = "The name of the carrier that is used to ship the import or export drives."]
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[doc = "The tracking number of the package."]
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    #[doc = "The number of drives included in the package."]
    #[serde(rename = "driveCount", default, skip_serializing_if = "Option::is_none")]
    pub drive_count: Option<i64>,
    #[doc = "The date when the package is shipped."]
    #[serde(rename = "shipDate", default, skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<String>,
}
impl DeliveryPackageInformation {
    pub fn new(carrier_name: String, tracking_number: String) -> Self {
        Self {
            carrier_name,
            tracking_number,
            drive_count: None,
            ship_date: None,
        }
    }
}
#[doc = "BitLocker recovery key or password to the specified drive"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DriveBitLockerKey {
    #[doc = "BitLocker recovery key or password"]
    #[serde(rename = "bitLockerKey", default, skip_serializing_if = "Option::is_none")]
    pub bit_locker_key: Option<String>,
    #[doc = "Drive ID"]
    #[serde(rename = "driveId", default, skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
}
impl DriveBitLockerKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides information about the drive's status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DriveStatus {
    #[doc = "The drive's hardware serial number, without spaces."]
    #[serde(rename = "driveId", default, skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
    #[doc = "The BitLocker key used to encrypt the drive."]
    #[serde(rename = "bitLockerKey", default, skip_serializing_if = "Option::is_none")]
    pub bit_locker_key: Option<String>,
    #[doc = "The relative path of the manifest file on the drive. "]
    #[serde(rename = "manifestFile", default, skip_serializing_if = "Option::is_none")]
    pub manifest_file: Option<String>,
    #[doc = "The Base16-encoded MD5 hash of the manifest file on the drive."]
    #[serde(rename = "manifestHash", default, skip_serializing_if = "Option::is_none")]
    pub manifest_hash: Option<String>,
    #[doc = "The drive header hash value."]
    #[serde(rename = "driveHeaderHash", default, skip_serializing_if = "Option::is_none")]
    pub drive_header_hash: Option<String>,
    #[doc = "The drive's current state. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<drive_status::State>,
    #[doc = "Detailed status about the data transfer process. This field is not returned in the response until the drive is in the Transferring state."]
    #[serde(rename = "copyStatus", default, skip_serializing_if = "Option::is_none")]
    pub copy_status: Option<String>,
    #[doc = "Percentage completed for the drive. "]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i64>,
    #[doc = "A URI that points to the blob containing the verbose log for the data transfer operation. "]
    #[serde(rename = "verboseLogUri", default, skip_serializing_if = "Option::is_none")]
    pub verbose_log_uri: Option<String>,
    #[doc = "A URI that points to the blob containing the error log for the data transfer operation."]
    #[serde(rename = "errorLogUri", default, skip_serializing_if = "Option::is_none")]
    pub error_log_uri: Option<String>,
    #[doc = "A URI that points to the blob containing the drive manifest file. "]
    #[serde(rename = "manifestUri", default, skip_serializing_if = "Option::is_none")]
    pub manifest_uri: Option<String>,
    #[doc = "Bytes successfully transferred for the drive."]
    #[serde(rename = "bytesSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub bytes_succeeded: Option<i64>,
}
impl DriveStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod drive_status {
    use super::*;
    #[doc = "The drive's current state. "]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Specified,
        Received,
        NeverReceived,
        Transferring,
        Completed,
        CompletedMoreInfo,
        ShippedBack,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Specified => serializer.serialize_unit_variant("State", 0u32, "Specified"),
                Self::Received => serializer.serialize_unit_variant("State", 1u32, "Received"),
                Self::NeverReceived => serializer.serialize_unit_variant("State", 2u32, "NeverReceived"),
                Self::Transferring => serializer.serialize_unit_variant("State", 3u32, "Transferring"),
                Self::Completed => serializer.serialize_unit_variant("State", 4u32, "Completed"),
                Self::CompletedMoreInfo => serializer.serialize_unit_variant("State", 5u32, "CompletedMoreInfo"),
                Self::ShippedBack => serializer.serialize_unit_variant("State", 6u32, "ShippedBack"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for State {
        fn default() -> Self {
            Self::Specified
        }
    }
}
#[doc = "Specifies the encryption key properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionKeyDetails {
    #[doc = "The type of kek encryption key"]
    #[serde(rename = "kekType", default, skip_serializing_if = "Option::is_none")]
    pub kek_type: Option<encryption_key_details::KekType>,
    #[doc = "Specifies the url for kek encryption key. "]
    #[serde(rename = "kekUrl", default, skip_serializing_if = "Option::is_none")]
    pub kek_url: Option<String>,
    #[doc = "Specifies the keyvault resource id for kek encryption key. "]
    #[serde(rename = "kekVaultResourceID", default, skip_serializing_if = "Option::is_none")]
    pub kek_vault_resource_id: Option<String>,
}
impl EncryptionKeyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_key_details {
    use super::*;
    #[doc = "The type of kek encryption key"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KekType")]
    pub enum KekType {
        MicrosoftManaged,
        CustomerManaged,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KekType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KekType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KekType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftManaged => serializer.serialize_unit_variant("KekType", 0u32, "MicrosoftManaged"),
                Self::CustomerManaged => serializer.serialize_unit_variant("KekType", 1u32, "CustomerManaged"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for KekType {
        fn default() -> Self {
            Self::MicrosoftManaged
        }
    }
}
#[doc = "Response when errors occurred"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Describes the error information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response {
    use super::*;
    #[doc = "Describes the error information."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Provides information about the error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Provides information about the error message."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "Provides information about the error target."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[doc = "Describes the error details if present."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<serde_json::Value>,
        #[doc = "Inner error object if present."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub innererror: Option<serde_json::Value>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A property containing information about the blobs to be exported for an export job. This property is required for export jobs, but must not be specified for import jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Export {
    #[doc = "A list of the blobs to be exported."]
    #[serde(rename = "blobList", default, skip_serializing_if = "Option::is_none")]
    pub blob_list: Option<export::BlobList>,
    #[doc = "The relative URI to the block blob that contains the list of blob paths or blob path prefixes as defined above, beginning with the container name. If the blob is in root container, the URI must begin with $root. "]
    #[serde(rename = "blobListBlobPath", default, skip_serializing_if = "Option::is_none")]
    pub blob_list_blob_path: Option<String>,
}
impl Export {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export {
    use super::*;
    #[doc = "A list of the blobs to be exported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BlobList {
        #[doc = "A collection of blob-path strings."]
        #[serde(rename = "blobPath", default, skip_serializing_if = "Vec::is_empty")]
        pub blob_path: Vec<String>,
        #[doc = "A collection of blob-prefix strings."]
        #[serde(rename = "blobPathPrefix", default, skip_serializing_if = "Vec::is_empty")]
        pub blob_path_prefix: Vec<String>,
    }
    impl BlobList {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "GetBitLockerKeys response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetBitLockerKeysResponse {
    #[doc = "drive status"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DriveBitLockerKey>,
}
impl azure_core::Continuable for GetBitLockerKeysResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GetBitLockerKeysResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the identity properties. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityDetails {
    #[doc = "The type of identity"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_details::Type>,
    #[doc = "Specifies the principal id for the identity for the job. "]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Specifies the tenant id for the identity for the job. "]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl IdentityDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_details {
    use super::*;
    #[doc = "The type of identity"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::None
        }
    }
}
#[doc = "Specifies the job properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDetails {
    #[doc = "The resource identifier of the storage account where data will be imported to or exported from."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The type of job"]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[doc = "Specifies the return address information for the job."]
    #[serde(rename = "returnAddress", default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<ReturnAddress>,
    #[doc = "Specifies the return carrier and customer's account with the carrier."]
    #[serde(rename = "returnShipping", default, skip_serializing_if = "Option::is_none")]
    pub return_shipping: Option<ReturnShipping>,
    #[doc = "Contains information about the Microsoft datacenter to which the drives should be shipped."]
    #[serde(rename = "shippingInformation", default, skip_serializing_if = "Option::is_none")]
    pub shipping_information: Option<ShippingInformation>,
    #[doc = "Contains information about the delivery package being shipped by the customer to the Microsoft data center."]
    #[serde(rename = "deliveryPackage", default, skip_serializing_if = "Option::is_none")]
    pub delivery_package: Option<DeliveryPackageInformation>,
    #[doc = "Contains information about the package being shipped by the customer to the Microsoft data center."]
    #[serde(rename = "returnPackage", default, skip_serializing_if = "Option::is_none")]
    pub return_package: Option<PackageInfomation>,
    #[doc = "The virtual blob directory to which the copy logs and backups of drive manifest files (if enabled) will be stored."]
    #[serde(rename = "diagnosticsPath", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_path: Option<String>,
    #[doc = "Default value is Error. Indicates whether error logging or verbose logging will be enabled."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "Default value is false. Indicates whether the manifest files on the drives should be copied to block blobs."]
    #[serde(rename = "backupDriveManifest", default, skip_serializing_if = "Option::is_none")]
    pub backup_drive_manifest: Option<bool>,
    #[doc = "Current state of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Indicates whether a request has been submitted to cancel the job."]
    #[serde(rename = "cancelRequested", default, skip_serializing_if = "Option::is_none")]
    pub cancel_requested: Option<bool>,
    #[doc = "Overall percentage completed for the job."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "A blob path that points to a block blob containing a list of blob names that were not exported due to insufficient drive space. If all blobs were exported successfully, then this element is not included in the response."]
    #[serde(rename = "incompleteBlobListUri", default, skip_serializing_if = "Option::is_none")]
    pub incomplete_blob_list_uri: Option<String>,
    #[doc = "List of up to ten drives that comprise the job. The drive list is a required element for an import job; it is not specified for export jobs."]
    #[serde(rename = "driveList", default, skip_serializing_if = "Vec::is_empty")]
    pub drive_list: Vec<DriveStatus>,
    #[doc = "A property containing information about the blobs to be exported for an export job. This property is required for export jobs, but must not be specified for import jobs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export: Option<Export>,
    #[doc = "Specifies the provisioning state of the job."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Specifies the encryption key properties"]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKeyDetails>,
}
impl JobDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the job information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResponse {
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Specifies the resource identifier of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the name of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the type of the job resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the Azure location where the job is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Specifies the tags that are assigned to the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Specifies the job properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobDetails>,
    #[doc = "Specifies the identity properties. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityDetails>,
}
impl JobResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List jobs response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListJobsResponse {
    #[doc = "link to next batch of jobs"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Job list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResponse>,
}
impl azure_core::Continuable for ListJobsResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListJobsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List operations response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListOperationsResponse {
    #[doc = "operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for ListOperationsResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListOperationsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides information about an Azure data center location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    #[doc = "Specifies the resource identifier of the location. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the name of the location. Use List Locations to get all supported locations. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the type of the location. "]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "location properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<location::Properties>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod location {
    use super::*;
    #[doc = "location properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The recipient name to use when shipping the drives to the Azure data center. "]
        #[serde(rename = "recipientName", default, skip_serializing_if = "Option::is_none")]
        pub recipient_name: Option<String>,
        #[doc = "The first line of the street address to use when shipping the drives to the Azure data center. "]
        #[serde(rename = "streetAddress1", default, skip_serializing_if = "Option::is_none")]
        pub street_address1: Option<String>,
        #[doc = "The second line of the street address to use when shipping the drives to the Azure data center. "]
        #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
        pub street_address2: Option<String>,
        #[doc = "The city name to use when shipping the drives to the Azure data center. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[doc = "The state or province to use when shipping the drives to the Azure data center. "]
        #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
        pub state_or_province: Option<String>,
        #[doc = "The postal code to use when shipping the drives to the Azure data center. "]
        #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
        pub postal_code: Option<String>,
        #[doc = "The country or region to use when shipping the drives to the Azure data center. "]
        #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
        pub country_or_region: Option<String>,
        #[doc = "The phone number for the Azure data center. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[doc = "Additional shipping information for customer, specific to datacenter to which customer should send their disks."]
        #[serde(rename = "additionalShippingInformation", default, skip_serializing_if = "Option::is_none")]
        pub additional_shipping_information: Option<String>,
        #[doc = "A list of carriers that are supported at this location. "]
        #[serde(rename = "supportedCarriers", default, skip_serializing_if = "Vec::is_empty")]
        pub supported_carriers: Vec<String>,
        #[doc = "A list of location IDs that should be used to ship shipping drives to for jobs created against the current location. If the current location is active, it will be part of the list. If it is temporarily closed due to maintenance, this list may contain other locations. "]
        #[serde(rename = "alternateLocations", default, skip_serializing_if = "Vec::is_empty")]
        pub alternate_locations: Vec<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Locations response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationsResponse {
    #[doc = "locations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Location>,
}
impl azure_core::Continuable for LocationsResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LocationsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a supported operation by the Storage Import/Export job API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "Name of the operation."]
    pub name: String,
    #[doc = "operation display properties"]
    pub display: operation::Display,
}
impl Operation {
    pub fn new(name: String, display: operation::Display) -> Self {
        Self { name, display }
    }
}
pub mod operation {
    use super::*;
    #[doc = "operation display properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The resource provider name to which the operation belongs."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The name of the resource to which the operation belongs."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The display name of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Short description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Contains information about the package being shipped by the customer to the Microsoft data center."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInfomation {
    #[doc = "The name of the carrier that is used to ship the import or export drives."]
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[doc = "The tracking number of the package."]
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    #[doc = "The number of drives included in the package."]
    #[serde(rename = "driveCount")]
    pub drive_count: i32,
    #[doc = "The date when the package is shipped."]
    #[serde(rename = "shipDate")]
    pub ship_date: String,
}
impl PackageInfomation {
    pub fn new(carrier_name: String, tracking_number: String, drive_count: i32, ship_date: String) -> Self {
        Self {
            carrier_name,
            tracking_number,
            drive_count,
            ship_date,
        }
    }
}
#[doc = "Put Job parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutJobParameters {
    #[doc = "Specifies the supported Azure location where the job should be created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Specifies the tags that will be assigned to the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Specifies the job properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobDetails>,
}
impl PutJobParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the return address information for the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnAddress {
    #[doc = "The name of the recipient who will receive the hard drives when they are returned. "]
    #[serde(rename = "recipientName")]
    pub recipient_name: String,
    #[doc = "The first line of the street address to use when returning the drives. "]
    #[serde(rename = "streetAddress1")]
    pub street_address1: String,
    #[doc = "The second line of the street address to use when returning the drives. "]
    #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    #[doc = "The city name to use when returning the drives."]
    pub city: String,
    #[doc = "The state or province to use when returning the drives."]
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[doc = "The postal code to use when returning the drives."]
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[doc = "The country or region to use when returning the drives. "]
    #[serde(rename = "countryOrRegion")]
    pub country_or_region: String,
    #[doc = "Phone number of the recipient of the returned drives."]
    pub phone: String,
    #[doc = "Email address of the recipient of the returned drives."]
    pub email: String,
}
impl ReturnAddress {
    pub fn new(
        recipient_name: String,
        street_address1: String,
        city: String,
        postal_code: String,
        country_or_region: String,
        phone: String,
        email: String,
    ) -> Self {
        Self {
            recipient_name,
            street_address1,
            street_address2: None,
            city,
            state_or_province: None,
            postal_code,
            country_or_region,
            phone,
            email,
        }
    }
}
#[doc = "Specifies the return carrier and customer's account with the carrier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnShipping {
    #[doc = "The carrier's name."]
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[doc = "The customer's account number with the carrier."]
    #[serde(rename = "carrierAccountNumber")]
    pub carrier_account_number: String,
}
impl ReturnShipping {
    pub fn new(carrier_name: String, carrier_account_number: String) -> Self {
        Self {
            carrier_name,
            carrier_account_number,
        }
    }
}
#[doc = "Contains information about the Microsoft datacenter to which the drives should be shipped."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShippingInformation {
    #[doc = "The name of the recipient who will receive the hard drives when they are returned. "]
    #[serde(rename = "recipientName", default, skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
    #[doc = "The first line of the street address to use when returning the drives. "]
    #[serde(rename = "streetAddress1", default, skip_serializing_if = "Option::is_none")]
    pub street_address1: Option<String>,
    #[doc = "The second line of the street address to use when returning the drives. "]
    #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    #[doc = "The city name to use when returning the drives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The state or province to use when returning the drives."]
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[doc = "The postal code to use when returning the drives."]
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "The country or region to use when returning the drives. "]
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
    #[doc = "Phone number of the recipient of the returned drives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Additional shipping information for customer, specific to datacenter to which customer should send their disks."]
    #[serde(rename = "additionalInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
}
impl ShippingInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update Job parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateJobParameters {
    #[doc = "Specifies the tags that will be assigned to the job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Specifies the properties of a UpdateJob."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<update_job_parameters::Properties>,
}
impl UpdateJobParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_job_parameters {
    use super::*;
    #[doc = "Specifies the properties of a UpdateJob."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "If specified, the value must be true. The service will attempt to cancel the job. "]
        #[serde(rename = "cancelRequested", default, skip_serializing_if = "Option::is_none")]
        pub cancel_requested: Option<bool>,
        #[doc = "If specified, the value must be Shipping, which tells the Import/Export service that the package for the job has been shipped. The ReturnAddress and DeliveryPackage properties must have been set either in this request or in a previous request, otherwise the request will fail. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[doc = "Specifies the return address information for the job."]
        #[serde(rename = "returnAddress", default, skip_serializing_if = "Option::is_none")]
        pub return_address: Option<ReturnAddress>,
        #[doc = "Specifies the return carrier and customer's account with the carrier."]
        #[serde(rename = "returnShipping", default, skip_serializing_if = "Option::is_none")]
        pub return_shipping: Option<ReturnShipping>,
        #[doc = "Contains information about the delivery package being shipped by the customer to the Microsoft data center."]
        #[serde(rename = "deliveryPackage", default, skip_serializing_if = "Option::is_none")]
        pub delivery_package: Option<DeliveryPackageInformation>,
        #[doc = "Indicates whether error logging or verbose logging is enabled."]
        #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
        pub log_level: Option<String>,
        #[doc = "Indicates whether the manifest files on the drives should be copied to block blobs."]
        #[serde(rename = "backupDriveManifest", default, skip_serializing_if = "Option::is_none")]
        pub backup_drive_manifest: Option<bool>,
        #[doc = "List of drives that comprise the job."]
        #[serde(rename = "driveList", default, skip_serializing_if = "Vec::is_empty")]
        pub drive_list: Vec<DriveStatus>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
